# MapGenerator
## Project Description
This project was created as a proof of concept for a class on Computer Graphics. There are two parts to this program the application which shows the end results to the user and the rust code that generates the image using Simplex noise as well as a set number of octaves.

### Application
The Application uses PyQt to provide a GUI for the map generator which also displays the saved image from the rust code.

### Rust Code
The Rust code uses Simplex noise to generate the different values for the pixels as well as assigning a humidity and altitude value to the map to provide for a more varied palette. Currently saves the image to pass it back to the GUI, would probably be more efficient to create either a rest server or another form of communication between the frontend and the backend to pass the image.
