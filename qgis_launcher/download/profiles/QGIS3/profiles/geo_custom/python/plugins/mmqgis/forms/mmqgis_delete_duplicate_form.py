# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/mmqgis_delete_duplicate_form.ui'
#
# Created by: PyQt5 UI code generator 5.7
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_mmqgis_delete_duplicate_form(object):
    def setupUi(self, mmqgis_delete_duplicate_form):
        mmqgis_delete_duplicate_form.setObjectName("mmqgis_delete_duplicate_form")
        mmqgis_delete_duplicate_form.setWindowModality(QtCore.Qt.ApplicationModal)
        mmqgis_delete_duplicate_form.setEnabled(True)
        mmqgis_delete_duplicate_form.resize(351, 164)
        mmqgis_delete_duplicate_form.setMouseTracking(False)
        self.verticalLayout = QtWidgets.QVBoxLayout(mmqgis_delete_duplicate_form)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label_4 = QtWidgets.QLabel(mmqgis_delete_duplicate_form)
        self.label_4.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_4.setObjectName("label_4")
        self.verticalLayout.addWidget(self.label_4)
        self.input_layer_name = QtWidgets.QComboBox(mmqgis_delete_duplicate_form)
        self.input_layer_name.setObjectName("input_layer_name")
        self.verticalLayout.addWidget(self.input_layer_name)
        self.label = QtWidgets.QLabel(mmqgis_delete_duplicate_form)
        self.label.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)
        self.output_file_name = gui.QgsFileWidget(mmqgis_delete_duplicate_form)
        self.output_file_name.setObjectName("output_file_name")
        self.verticalLayout.addWidget(self.output_file_name)
        self.status = QtWidgets.QProgressBar(mmqgis_delete_duplicate_form)
        self.status.setProperty("value", 24)
        self.status.setObjectName("status")
        self.verticalLayout.addWidget(self.status)
        self.buttonBox = QtWidgets.QDialogButtonBox(mmqgis_delete_duplicate_form)
        self.buttonBox.setOrientation(QtCore.Qt.Horizontal)
        self.buttonBox.setStandardButtons(QtWidgets.QDialogButtonBox.Apply|QtWidgets.QDialogButtonBox.Close)
        self.buttonBox.setCenterButtons(True)
        self.buttonBox.setObjectName("buttonBox")
        self.verticalLayout.addWidget(self.buttonBox)

        self.retranslateUi(mmqgis_delete_duplicate_form)
        self.buttonBox.accepted.connect(mmqgis_delete_duplicate_form.accept)
        self.buttonBox.rejected.connect(mmqgis_delete_duplicate_form.reject)
        QtCore.QMetaObject.connectSlotsByName(mmqgis_delete_duplicate_form)
        mmqgis_delete_duplicate_form.setTabOrder(self.input_layer_name, self.buttonBox)

    def retranslateUi(self, mmqgis_delete_duplicate_form):
        _translate = QtCore.QCoreApplication.translate
        mmqgis_delete_duplicate_form.setWindowTitle(_translate("mmqgis_delete_duplicate_form", "Delete Duplicate Geometries"))
        self.label_4.setText(_translate("mmqgis_delete_duplicate_form", "Input Layer"))
        self.label.setText(_translate("mmqgis_delete_duplicate_form", "Output File Name"))

from qgis import gui
