# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/mmqgis_change_projection_form.ui'
#
# Created by: PyQt5 UI code generator 5.7
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_mmqgis_change_projection_form(object):
    def setupUi(self, mmqgis_change_projection_form):
        mmqgis_change_projection_form.setObjectName("mmqgis_change_projection_form")
        mmqgis_change_projection_form.resize(481, 391)
        self.verticalLayout = QtWidgets.QVBoxLayout(mmqgis_change_projection_form)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label = QtWidgets.QLabel(mmqgis_change_projection_form)
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)
        self.input_layer_name = QtWidgets.QComboBox(mmqgis_change_projection_form)
        self.input_layer_name.setObjectName("input_layer_name")
        self.verticalLayout.addWidget(self.input_layer_name)
        self.old_proj4 = QtWidgets.QTextEdit(mmqgis_change_projection_form)
        self.old_proj4.setObjectName("old_proj4")
        self.verticalLayout.addWidget(self.old_proj4)
        self.label_3 = QtWidgets.QLabel(mmqgis_change_projection_form)
        self.label_3.setObjectName("label_3")
        self.verticalLayout.addWidget(self.label_3)
        self.new_crs = gui.QgsProjectionSelectionWidget(mmqgis_change_projection_form)
        self.new_crs.setObjectName("new_crs")
        self.verticalLayout.addWidget(self.new_crs)
        self.new_proj4 = QtWidgets.QTextEdit(mmqgis_change_projection_form)
        self.new_proj4.setObjectName("new_proj4")
        self.verticalLayout.addWidget(self.new_proj4)
        self.label_2 = QtWidgets.QLabel(mmqgis_change_projection_form)
        self.label_2.setObjectName("label_2")
        self.verticalLayout.addWidget(self.label_2)
        self.output_file_name = gui.QgsFileWidget(mmqgis_change_projection_form)
        self.output_file_name.setObjectName("output_file_name")
        self.verticalLayout.addWidget(self.output_file_name)
        self.status = QtWidgets.QProgressBar(mmqgis_change_projection_form)
        self.status.setProperty("value", 24)
        self.status.setObjectName("status")
        self.verticalLayout.addWidget(self.status)
        self.buttonBox = QtWidgets.QDialogButtonBox(mmqgis_change_projection_form)
        self.buttonBox.setOrientation(QtCore.Qt.Horizontal)
        self.buttonBox.setStandardButtons(QtWidgets.QDialogButtonBox.Apply|QtWidgets.QDialogButtonBox.Close)
        self.buttonBox.setCenterButtons(True)
        self.buttonBox.setObjectName("buttonBox")
        self.verticalLayout.addWidget(self.buttonBox)

        self.retranslateUi(mmqgis_change_projection_form)
        self.buttonBox.accepted.connect(mmqgis_change_projection_form.accept)
        self.buttonBox.rejected.connect(mmqgis_change_projection_form.reject)
        QtCore.QMetaObject.connectSlotsByName(mmqgis_change_projection_form)

    def retranslateUi(self, mmqgis_change_projection_form):
        _translate = QtCore.QCoreApplication.translate
        mmqgis_change_projection_form.setWindowTitle(_translate("mmqgis_change_projection_form", "Change Projection"))
        self.label.setText(_translate("mmqgis_change_projection_form", "Input Layer Name"))
        self.label_3.setText(_translate("mmqgis_change_projection_form", "New Projection"))
        self.label_2.setText(_translate("mmqgis_change_projection_form", "Output File Name"))

from qgis import gui
