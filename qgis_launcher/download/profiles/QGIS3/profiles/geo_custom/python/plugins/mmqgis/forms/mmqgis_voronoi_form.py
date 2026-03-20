# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/mmqgis_voronoi_form.ui'
#
# Created by: PyQt5 UI code generator 5.7
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_mmqgis_voronoi_form(object):
    def setupUi(self, mmqgis_voronoi_form):
        mmqgis_voronoi_form.setObjectName("mmqgis_voronoi_form")
        mmqgis_voronoi_form.setWindowModality(QtCore.Qt.ApplicationModal)
        mmqgis_voronoi_form.setEnabled(True)
        mmqgis_voronoi_form.resize(383, 164)
        mmqgis_voronoi_form.setMouseTracking(False)
        self.verticalLayout = QtWidgets.QVBoxLayout(mmqgis_voronoi_form)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label_4 = QtWidgets.QLabel(mmqgis_voronoi_form)
        self.label_4.setObjectName("label_4")
        self.verticalLayout.addWidget(self.label_4)
        self.input_layer_name = QtWidgets.QComboBox(mmqgis_voronoi_form)
        self.input_layer_name.setObjectName("input_layer_name")
        self.verticalLayout.addWidget(self.input_layer_name)
        self.label = QtWidgets.QLabel(mmqgis_voronoi_form)
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)
        self.output_file_name = gui.QgsFileWidget(mmqgis_voronoi_form)
        self.output_file_name.setObjectName("output_file_name")
        self.verticalLayout.addWidget(self.output_file_name)
        self.status = QtWidgets.QProgressBar(mmqgis_voronoi_form)
        self.status.setProperty("value", 24)
        self.status.setObjectName("status")
        self.verticalLayout.addWidget(self.status)
        self.buttonBox = QtWidgets.QDialogButtonBox(mmqgis_voronoi_form)
        self.buttonBox.setOrientation(QtCore.Qt.Horizontal)
        self.buttonBox.setStandardButtons(QtWidgets.QDialogButtonBox.Apply|QtWidgets.QDialogButtonBox.Close)
        self.buttonBox.setCenterButtons(True)
        self.buttonBox.setObjectName("buttonBox")
        self.verticalLayout.addWidget(self.buttonBox)

        self.retranslateUi(mmqgis_voronoi_form)
        self.buttonBox.accepted.connect(mmqgis_voronoi_form.accept)
        self.buttonBox.rejected.connect(mmqgis_voronoi_form.reject)
        QtCore.QMetaObject.connectSlotsByName(mmqgis_voronoi_form)
        mmqgis_voronoi_form.setTabOrder(self.input_layer_name, self.buttonBox)

    def retranslateUi(self, mmqgis_voronoi_form):
        _translate = QtCore.QCoreApplication.translate
        mmqgis_voronoi_form.setWindowTitle(_translate("mmqgis_voronoi_form", "Voronoi Diagram"))
        self.label_4.setText(_translate("mmqgis_voronoi_form", "Source Layer"))
        self.label.setText(_translate("mmqgis_voronoi_form", "Output File Name"))

from qgis import gui
