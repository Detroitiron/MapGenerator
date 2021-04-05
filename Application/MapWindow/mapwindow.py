# This Python file uses the following encoding: utf-8
import sys
import os
from PySide6.QtCore import Qt
from PySide6.QtGui import QIntValidator, QDoubleValidator, QImage, QIcon
from PySide6.QtWidgets import QApplication, QMainWindow, QGraphicsView, QGraphicsScene, QHBoxLayout, QVBoxLayout, \
    QLabel, QPushButton, QLineEdit, QWidget, QMessageBox, QFileDialog
from PySide6.QtGui import QPixmap


class LabeledLineEdit(QHBoxLayout):
    def __init__(self, label, init_string):
        QHBoxLayout.__init__(self)
        self.label = QLabel(label)
        self.lineEdit = QLineEdit(init_string)
        self.addWidget(self.label)
        self.addWidget(self.lineEdit)

    def text(self):
        return self.lineEdit.text()

class MapWindow(QMainWindow):
    def __init__(self):
        QMainWindow.__init__(self)
        self.view = QGraphicsView()
        self.scene = QGraphicsScene()
        self.pixMap = QPixmap("altitude.png")
        self.imItem = self.scene.addPixmap(self.pixMap)
        self.view.setScene(self.scene)
        self.layoutView = QHBoxLayout()
        self.layoutView.addWidget(self.view)
        self.layoutControls = QVBoxLayout()
        # Controls for the seeds
        self.seedLabel = LabeledLineEdit("seed", "generic")
        # Controls for the size of the image
        self.widthLabel = LabeledLineEdit("Width", "1920")
        self.intValidator = QIntValidator(512,3000)
        self.widthLabel.lineEdit.setValidator(self.intValidator)
        self.sizeLayout = QHBoxLayout()
        self.sizeLayout.addLayout(self.widthLabel)
        self.heightLabel = LabeledLineEdit("Height", "1080")
        self.heightLabel.lineEdit.setValidator(self.intValidator)
        self.sizeLayout.addLayout(self.heightLabel)
        # Persistence and Lacunarity
        self.octaveLayout = QHBoxLayout()
        self.doubleValidator = QDoubleValidator(0.1, 10.0, 2)
        self.persistenceLabel = LabeledLineEdit("Persistence", "0.5")
        self.persistenceLabel.lineEdit.setValidator(self.doubleValidator)
        self.lacunarityLabel = LabeledLineEdit("Lacunarity", "2.0")
        self.lacunarityLabel.lineEdit.setValidator(self.doubleValidator)
        self.octaveLayout.addLayout(self.persistenceLabel)
        self.octaveLayout.addLayout(self.lacunarityLabel)

        self.layoutControls.addLayout(self.seedLabel)
        self.layoutControls.addLayout(self.sizeLayout)
        self.layoutControls.addLayout(self.octaveLayout)
        self.buttonLayout = QHBoxLayout()
        self.submitButton = QPushButton("Submit")
        self.saveIcon = QIcon("save.png")
        self.saveButton = QPushButton("Save")
        self.saveButton.setIcon(self.saveIcon)
        self.saveButton.clicked.connect(self.savePixmap)
        self.submitButton.clicked.connect(self.generateMap)
        self.buttonLayout.addWidget(self.submitButton)
        self.buttonLayout.addWidget(self.saveButton)
        self.layoutControls.addLayout(self.buttonLayout)
        self.layoutView.addLayout(self.layoutControls)
        self.layoutView.setStretchFactor(self.view, 4)
        self.layoutView.setStretchFactor(self.layoutControls, 1)
        self.widget = QWidget()
        self.widget.setLayout(self.layoutView)
        self.setCentralWidget(self.widget)

    def generateMap(self):
        seed = self.seedLabel.text()
        width = self.widthLabel.text()
        height = self.heightLabel.text()
        persistence = self.persistenceLabel.text()
        lacunarity = self.lacunarityLabel.text()
        run = os.system(f'.\\worldGenerator -s "{seed}" -w {width} -h {height} -p {persistence} -l {lacunarity}')
        print(run)
        if run == 0:
            image = QImage("altitude.png")
            if image.isNull():
                QMessageBox.information(self, "Image Viewer", "Cannot load 'altitude.png'")

            self.imItem.setPixmap(QPixmap("altitude.png"))
            print("file loaded")
        else:
            print(f'Error code {run} encountered.')

    def resizePixmap(self):
        size = self.view.size()
        self.pixMap.scaled(size, Qt.KeepAspectRatio, Qt.SmoothTransformation)

    def eventResize(self, event):
        self.resizePixmap()

    def savePixmap(self):
        fileName = QFileDialog.getSaveFileName(self, u"Save Image", os.getcwd(), u"Png Files (*.png)")
        self.pixMap.save(fileName[0])

if __name__ == "__main__":
    os.system(f'.\\worldGenerator')
    app = QApplication([])
    window = MapWindow()
    window.show()

    sys.exit(app.exec_())
