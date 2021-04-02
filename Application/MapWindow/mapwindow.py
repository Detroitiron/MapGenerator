# This Python file uses the following encoding: utf-8
import sys
import os
import threading
from PySide6.QtCore import QEvent, Qt
from PySide6.QtGui import QIntValidator, QDoubleValidator
from PySide6.QtWidgets import QApplication, QMainWindow, QGraphicsView, QGraphicsScene, QHBoxLayout, QVBoxLayout, \
    QLabel, QPushButton, QLineEdit, QWidget
from PySide6.QtGui import QPixmap, QResizeEvent

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
        self.scene.addPixmap(self.pixMap)
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
        self.submitButton = QPushButton("Submit")
        self.submitButton.clicked.connect(self.generateMap())
        self.layoutControls.addWidget(self.submitButton)
        self.layoutView.addLayout(self.layoutControls)
        self.widget = QWidget()
        self.widget.setLayout(self.layoutView)
        self.setCentralWidget(self.widget)

    def generateMap(self):
        seed = self.seedLabel.text()
        width = self.widthLabel.text()
        height = self.heightLabel.text()
        persistence = self.persistenceLabel.text()
        lacunarity = self.lacunarityLabel.text()
        run = os.system(f'.\\worldGenerator -s {seed} -w {width} -h {height} -p {persistence} -l {lacunarity}')
        if run == 0:
            self.pixMap.load("altitude.png")
        else:
            print(f'Error code {run} encountered.')

    def resizePixmap(self):
        size = self.scene.size()
        print(size)
        self.pixMap.scaled(size, Qt.KeepAspectRatio, Qt.SmoothTransformation)

    def eventResize(self, event):
        self.resizePixmap()


if __name__ == "__main__":
    os.system(f'.\\worldGenerator')
    app = QApplication([])
    window = MapWindow()
    window.show()

    sys.exit(app.exec_())
