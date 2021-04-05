#![allow(non_snake_case, unused_imports)]
use clap::{App, Arg};
use image::{Luma, Primitive, Rgb};
use noise::{NoiseFn, OpenSimplex, Seedable, RangeFunction, Turbulence};
use rand::prelude::StdRng;
use rand::seq::index::sample;
use rand::{Rng, RngCore, SeedableRng};
use std::iter::Map;
use std::str::FromStr;
use std::thread;
use std::thread::JoinHandle;
use std::f64::consts::PI;
use std::cmp::min;
use std::ops::Rem;

fn main() {
    //! Allows for different arguments to be passed to the MapGenerator.
    let matches = App::new("MapGeneratorBackend")
        .version("0.0.2")
        .author("Shane Mecham")
        .about("Generates a World map given a seed, width, and height")
        .arg(Arg::with_name("seed")
            .short("s")
            .long("seed")
            .takes_value(true)
            .help("Sets the seed of the map generated."))
        .arg(Arg::with_name("map_width")
            .short("w")
            .long("map_width")
            .takes_value(true)
            .help("Sets the width of the map generated."))
        .arg(Arg::with_name("map_height")
            .short("h")
            .long("map_height")
            .takes_value(true)
            .help("Sets the height of the map generated."))
        .arg(Arg::with_name("persistence")
            .short("p")
            .long("persistence")
            .takes_value(true)
            .help("Sets the persistence which augments the amplitude of the octaves of OpenSimplex fucntion."))
        .arg(Arg::with_name("lacunarity")
            .short("l")
            .long("lacunarity")
            .takes_value(true)
            .help("Sets the lacunarity which augments the frequency of the octaves of OpenSimplex function."))
        .get_matches();
    // Use matches to get the values passed into the commandline, get default values if they weren't passed in.
    let seed = matches.value_of("seed");
    let s = match seed {
        None => u32::MIN,
        Some(s2) => match s2.parse::<u32>() {
            Ok(n) => n,
            Err(_) => u32::MIN,
        },
    };
    let map_width = matches.value_of("map_width");
    let width = match map_width {
        None => 512,
        Some(s) => match u32::from_str(s) {
            Ok(n) => n,
            Err(_) => 512,
        },
    };
    let map_height = matches.value_of("map_height");
    let height = match map_height {
        None => 512,
        Some(s) => match u32::from_str(s) {
            Ok(n) => n,
            Err(_) => 512,
        },
    };
    let persistence = matches.value_of("persistence");
    let pers = match persistence {
        None => 0.5,
        Some(s) => match f64::from_str(s) {
            Ok(n) => n,
            Err(_) => 0.5,
        },
    };
    let lacunarity = matches.value_of("lacunarity");
    let lac = match lacunarity {
        None => 2.0,
        Some(s) => match f64::from_str(s) {
            Ok(n) => n,
            Err(_) => 2.0,
        },
    };
    let mut mapGenerator = MapGenerator::new(s, width, height);
    mapGenerator.generateMap(pers, lac, 10);
}

#[derive(Default)]
/// The data structure used to store the basic map information.
struct MapGenerator {
    seed: u32,
    map_width: u32,
    map_height: u32,
    height_map: Vec<Vec<f64>>,
    heat_map: Vec<Vec<f64>>,
    noise: Vec<OpenSimplex>,
}

#[derive(Copy, Clone, Debug)]
/// Used to store the pixels and specify the
/// Biome of each pixel
enum Biome {
    SubtropicalDesert(image::Rgb<u8>),
    TemperateDesert(image::Rgb<u8>),
    ScorchedLand(image::Rgb<u8>),
    Grassland(image::Rgb<u8>),
    Shrubland(image::Rgb<u8>),
    BareLand(image::Rgb<u8>),
    TropicalSeasonalForest(image::Rgb<u8>),
    TemperateDecidousForest(image::Rgb<u8>),
    Taiga(image::Rgb<u8>),
    Tundra(image::Rgb<u8>),
    TropicalRainForest(image::Rgb<u8>),
    TemperateRainForest(image::Rgb<u8>),
    Snow(image::Rgb<u8>),
    Error(image::Rgb<u8>),
    DeepOcean(image::Rgb<u8>),
}


impl MapGenerator {
    pub fn new(seed: u32, map_width: u32, map_height: u32) -> Self {
        let mut map = Vec::new();
        for _ in 0..map_height {
            let row = vec![0.0; map_width as usize];
            map.push(row);
        }
        let heat_map = map.clone();
        Self {
            seed,
            map_width,
            map_height,
            height_map: map,
            heat_map,
            noise: vec![],
        }
    }

    pub fn generateMap(&mut self, persistence: f64, lacunarity: f64, octaves: i32) {
        let mut rng = StdRng::seed_from_u64(self.seed as u64);
        // Initializes the noise functions and sets there seed based on the seed passed in by the user.
        if self.noise.is_empty() {
            self.noise.push(OpenSimplex::new().set_seed(rng.next_u32()));
            self.noise.push(OpenSimplex::new().set_seed(rng.next_u32()));
        } else {
            for i in &mut self.noise {
                i.set_seed(rng.next_u32());
            }
        }
        // Originally tried to multi-thread this but ran into issues with Rust's specifications for multithreading.
        let map = &mut self.height_map;
        let map_height = self.map_height as usize;
        let noise = &self.noise[0];
        let map_width = self.map_width as usize;
        noiseGen(
            map,
            map_height,
            noise,
            map_width,
            persistence,
            lacunarity,
            octaves,
            (-0.5, 1.0)
        );
        let map = &mut self.heat_map;
        let map_height = self.map_height as usize;
        let noise = &self.noise[1];
        let map_width = self.map_width as usize;
        noiseGen(
            map,
            map_height,
            noise,
            map_width,
            persistence,
            lacunarity,
            octaves,
            (0.0, 1.0)
        );

        let map = &mut self.height_map;
        let moisture_map = &mut self.heat_map;

        save_image(map, moisture_map, "altitude.png", map_width, map_height);
    }
}
/// Generates a linear interpolation
pub fn lerp(minValue:f64, maxValue:f64, scaledValue:f64, val: (f64, f64)) -> f64 {
    (val.0 * (maxValue - scaledValue) + val.1 * (scaledValue - minValue))/(maxValue - minValue)
}
/// Generates the noise map used to create the world map.
pub fn noiseGen(
    map: &mut Vec<Vec<f64>>,
    map_height: usize,
    vec_noise: &OpenSimplex,
    map_width: usize,
    persistence: f64,
    lacunarity: f64,
    octaves: i32,
    range: (f64, f64),
) {
    let mut maxNoiseHeight = f64::MIN;
    let mut minNoiseHeight = f64::MAX;
    let scale_x = 8.0 / (map_width) as f64;
    let scale_y = 8.0 / (map_height) as f64;
    for y in 0..map_height {
        for x in 0..map_width {
            let mut amplitude: f64 = 1.0;
            let mut totalAmplitude: f64 = 0.0;
            let mut frequency: f64 = 0.5;
            let mut noiseHeight: f64 = 0.0;
            for _ in 0..octaves {
                let sampleX = x as f64 * scale_x * frequency;
                let sampleY = y as f64 * scale_y * frequency;
                let value = vec_noise.get([sampleX, sampleY]);
                noiseHeight += amplitude * (1.0 - value.abs());
                totalAmplitude += amplitude;
                amplitude *= persistence;
                frequency *= lacunarity;
            }

            noiseHeight /= totalAmplitude;
            noiseHeight = noiseHeight.powf(2.0);
            if noiseHeight < minNoiseHeight {
                minNoiseHeight = noiseHeight;
            } else if noiseHeight > maxNoiseHeight {
                maxNoiseHeight = noiseHeight;
            }
            map[y][x] = noiseHeight;
        }
    }
    for y in 0..map_height as usize {
        for x in 0..map_width as usize {
            map[y][x] = lerp(minNoiseHeight, maxNoiseHeight, map[y][x], range);
        }
    }
}
/// Assigns a biome to each pixel in the map and assigns a pixel to each of these biomes so that we can save the format.
/// Tries to generate the map as quickly as possible so the map can be passed to the ui quickly.
pub fn save_image(map: &mut Vec<Vec<f64>>, moisture_map: &mut Vec<Vec<f64>>, path: &str, map_width: usize, map_height: usize) {
    let mut imgbuf = image::ImageBuffer::new(map_width as u32, map_height as u32);
    let mut biomeMap: Vec<Vec<Biome>> = Vec::new();
    for _ in 0..map_height {
        let row = vec![Biome::Error(image::Rgb([252, 0, 0])); map_width];
        biomeMap.push(row);
    }
    for y in 0..map_height{
        for x in 0..map_width {
            if map[y][x] < 0.0 {
                let percentage = map[y][x].abs() / 0.1 + 0.5;
                biomeMap[y][x] = Biome::DeepOcean(image::Rgb(scale_array([2, 69, 204], 1.0/percentage)));
            }
            else if map[y][x] < 0.25 {
                let percentage = 0.1 + map[y][x] / 0.1 + 0.3;
                biomeMap[y][x] = match moisture_map[y][x] {
                    p if p <= 0.25 => Biome::SubtropicalDesert(image::Rgb(scale_array([178, 171, 92], percentage))),
                    p if p <= 0.75 => Biome::TemperateDesert(image::Rgb(scale_array([234, 228, 166], percentage))),
                    p if p <= 1.0 => Biome::ScorchedLand(image::Rgb(scale_array([181, 160, 143], percentage))),
                    _ => Biome::Error(image::Rgb([252, 0, 0])),
                }
            } else if map[y][x] < 0.5 {
                let percentage = map[y][x] / 0.5 + 0.1;
                biomeMap[y][x] = match moisture_map[y][x] {
                    p if p <= 0.5 => Biome::Grassland(image::Rgb(scale_array([154, 198, 135], percentage))),
                    p if p <= 0.75 => Biome::Shrubland(image::Rgb(scale_array([144, 168, 67], percentage))),
                    p if p <= 1.0 => Biome::BareLand(image::Rgb(scale_array([168, 114, 42], percentage))),
                    _ => Biome::Error(image::Rgb([252, 0, 0])),
                }
            } else if map[y][x] < 0.75 {
                let percentage = map[y][x] / 0.75;
                biomeMap[y][x] = match moisture_map[y][x] {
                    p if p <= 0.25 => Biome::TropicalSeasonalForest(image::Rgb(scale_array([7, 130, 40], percentage))),
                    p if p <= 0.5 => Biome::TemperateDecidousForest(image::Rgb(scale_array([30, 93, 19], percentage))),
                    p if p <= 0.75 => Biome::Taiga(image::Rgb(scale_array([157, 230, 232], percentage+0.125))),
                    p if p <= 1.0 => Biome::Tundra(image::Rgb(scale_array([141, 206, 252], percentage+0.125))),
                    _ => Biome::Error(image::Rgb([252, 0, 0])),
                }
            } else {
                let percentage = map[y][x] / 1.0;
                biomeMap[y][x] = match moisture_map[y][x] {
                    p if p <= 0.25 => Biome::TropicalRainForest(image::Rgb(scale_array([3, 173, 77], percentage))),
                    p if p <= 0.5 => Biome::TemperateRainForest(image::Rgb(scale_array([0, 142, 61], percentage))),
                    p if p <= 0.75 => Biome::Taiga(image::Rgb(scale_array([157, 230, 232], percentage+0.125))),
                    p if p > 0.75 => Biome::Snow(image::Rgb(scale_array([194, 255, 255], percentage+0.125))),
                    _ => Biome::Error(image::Rgb([252, 0, 0])),
                }
            }
        }
    }
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = match biomeMap[y as usize][x as usize] {
            Biome::SubtropicalDesert(p) => p,
            Biome::TemperateDesert(p) => p,
            Biome::ScorchedLand(p) => p,
            Biome::Grassland(p) => p,
            Biome::Shrubland(p) => p,
            Biome::BareLand(p) => p,
            Biome::TropicalSeasonalForest(p) => p,
            Biome::TemperateDecidousForest(p) => p,
            Biome::Taiga(p) => p,
            Biome::Tundra(p) => p,
            Biome::TropicalRainForest(p) => p,
            Biome::TemperateRainForest(p) => p,
            Biome::Snow(p) => p,
            Biome::DeepOcean(p) => p,
            Biome::Error(p) => {println!("{}", map[y as usize][x as usize]); p},
        };
    }

    imgbuf
        .save_with_format(path, image::ImageFormat::Png)
        .unwrap();
}

pub fn scale_array (array: [u8; 3], scale: f64) -> [u8; 3] {
    let mut local: [u8; 3] = [0, 0, 0];
    if scale == 0.0 || scale > 1.0 {
        return array;
    }

    let mut count: usize = 0;
    for item in array.iter() {
        local[count] = (*item as f64 * scale) as u8;
        count += 1 as usize;
    }
    local
}