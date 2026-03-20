# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/mmqgis_text_to_float_form.ui'
#
# Created by: PyQt5 UI code generator 5.7
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_mmqgis_text_to_float_form(object):
    def setupUi(self, mmqgis_text_to_float_form):
        mmqgis_text_to_float_form.setObjectName("mmqgis_text_to_float_form")
        mmqgis_text_to_float_form.setWindowModality(QtCore.Qt.ApplicationModal)
        mmqgis_text_to_float_form.setEnabled(True)
        mmqgis_text_to_float_form.resize(415, 315)
        mmqgis_text_to_float_form.setMouseTracking(False)
        self.verticalLayout = QtWidgets.QVBoxLayout(mmqgis_text_to_float_form)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label_4 = QtWidgets.QLabel(mmqgis_text_to_float_form)
        self.label_4.setObjectName("label_4")
        self.verticalLayout.addWidget(self.label_4)
        self.input_layer_name = QtWidgets.QComboBox(mmqgis_text_to_float_form)
        self.input_layer_name.setObjectName("input_layer_name")
        self.verticalLayout.addWidget(self.input_layer_name)
        self.label_5 = QtWidgets.QLabel(mmqgis_text_to_float_form)
        self.label_5.setObjectName("label_5")
        self.verticalLayout.addWidget(self.label_5)
        self.field_names = QtWidgets.QListWidget(mmqgis_text_to_float_form)
        self.field_names.setSelectionMode(QtWidgets.QAbstractItemView.ExtendedSelection)
        self.field_names.setObjectName("field_names")
        self.verticalLayout.addWidget(self.field_names)
        self.label = QtWidgets.QLabel(mmqgis_text_to_float_form)
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)
        self.output_file_name = gui.QgsFileWidget(mmqgis_text_to_float_form)
        self.output_file_name.setObjectName("output_file_name")
        self.verticalLayout.addWidget(self.output_file_name)
        self.status = QtWidgets.QProgressBar(mmqgis_text_to_float_form)
        self.status.setProperty("value", 24)
        self.status.setObjectName("status")
        self.verticalLayout.addWidget(self.status)
        self.buttonBox = QtWidgets.QDialogButtonBox(mmqgis_text_to_float_form)
        self.buttonBox.setOrientation(QtCore.Qt.Horizontal)
        self.buttonBox.setStandardButtons(QtWidgets.QDialogButtonBox.Apply|QtWidgets.QDialogButtonBox.Close)
        self.buttonBox.setCenterButtons(True)
        self.buttonBox.setObjectName("buttonBox")
        self.verticalLayout.addWidget(self.buttonBox)

        self.retranslateUi(mmqgis_text_to_float_form)
        self.buttonBox.accepted.connect(mmqgis_text_to_float_form.accept)
        self.buttonBox.rejected.connect(mmqgis_text_to_float_form.reject)
        QtCore.QMetaObject.connectSlotsByName(mmqgis_text_to_float_form)
        mmqgis_text_to_float_form.setTabOrder(self.input_layer_name, self.field_names)
        mmqgis_text_to_float_form.setTabOrder(self.field_names, self.buttonBox)

    def retranslateUi(self, mmqgis_text_to_float_form):
        _translate = QtCore.QCoreApplication.translate
        mmqgis_text_to_float_form.setWindowTitle(_translate("mmqgis_text_to_float_form", "Text to Float"))
        self.label_4.setText(_translate("mmqgis_text_to_float_form", "Source Layer"))
        self.label_5.setText(_translate("mmqgis_text_to_float_form", "Fields to Convert"))
        self.label.setText(_translate("mmqgis_text_to_float_form", "Output File Name"))

from qgis import gui
