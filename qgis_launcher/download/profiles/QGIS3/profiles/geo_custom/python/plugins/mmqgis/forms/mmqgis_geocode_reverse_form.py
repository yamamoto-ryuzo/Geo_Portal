# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/mmqgis_geocode_reverse_form.ui'
#
# Created by: PyQt5 UI code generator 5.7
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_mmqgis_geocode_reverse_form(object):
    def setupUi(self, mmqgis_geocode_reverse_form):
        mmqgis_geocode_reverse_form.setObjectName("mmqgis_geocode_reverse_form")
        mmqgis_geocode_reverse_form.setWindowModality(QtCore.Qt.ApplicationModal)
        mmqgis_geocode_reverse_form.setEnabled(True)
        mmqgis_geocode_reverse_form.resize(433, 308)
        mmqgis_geocode_reverse_form.setMouseTracking(False)
        self.verticalLayout = QtWidgets.QVBoxLayout(mmqgis_geocode_reverse_form)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label_4 = QtWidgets.QLabel(mmqgis_geocode_reverse_form)
        self.label_4.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_4.setObjectName("label_4")
        self.verticalLayout.addWidget(self.label_4)
        self.input_layer_name = QtWidgets.QComboBox(mmqgis_geocode_reverse_form)
        self.input_layer_name.setObjectName("input_layer_name")
        self.verticalLayout.addWidget(self.input_layer_name)
        self.label_3 = QtWidgets.QLabel(mmqgis_geocode_reverse_form)
        self.label_3.setObjectName("label_3")
        self.verticalLayout.addWidget(self.label_3)
        self.web_service = QtWidgets.QComboBox(mmqgis_geocode_reverse_form)
        self.web_service.setObjectName("web_service")
        self.verticalLayout.addWidget(self.web_service)
        self.api_key_label = QtWidgets.QLabel(mmqgis_geocode_reverse_form)
        self.api_key_label.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.api_key_label.setObjectName("api_key_label")
        self.verticalLayout.addWidget(self.api_key_label)
        self.api_key = QtWidgets.QLineEdit(mmqgis_geocode_reverse_form)
        self.api_key.setObjectName("api_key")
        self.verticalLayout.addWidget(self.api_key)
        self.label_6 = QtWidgets.QLabel(mmqgis_geocode_reverse_form)
        self.label_6.setObjectName("label_6")
        self.verticalLayout.addWidget(self.label_6)
        self.duplicate_handling = QtWidgets.QComboBox(mmqgis_geocode_reverse_form)
        self.duplicate_handling.setObjectName("duplicate_handling")
        self.verticalLayout.addWidget(self.duplicate_handling)
        self.label = QtWidgets.QLabel(mmqgis_geocode_reverse_form)
        self.label.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)
        self.output_file_name = gui.QgsFileWidget(mmqgis_geocode_reverse_form)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Preferred, QtWidgets.QSizePolicy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.output_file_name.sizePolicy().hasHeightForWidth())
        self.output_file_name.setSizePolicy(sizePolicy)
        self.output_file_name.setObjectName("output_file_name")
        self.verticalLayout.addWidget(self.output_file_name)
        self.status = QtWidgets.QProgressBar(mmqgis_geocode_reverse_form)
        self.status.setProperty("value", 24)
        self.status.setObjectName("status")
        self.verticalLayout.addWidget(self.status)
        self.buttonBox = QtWidgets.QDialogButtonBox(mmqgis_geocode_reverse_form)
        self.buttonBox.setOrientation(QtCore.Qt.Horizontal)
        self.buttonBox.setStandardButtons(QtWidgets.QDialogButtonBox.Apply|QtWidgets.QDialogButtonBox.Close)
        self.buttonBox.setCenterButtons(True)
        self.buttonBox.setObjectName("buttonBox")
        self.verticalLayout.addWidget(self.buttonBox)

        self.retranslateUi(mmqgis_geocode_reverse_form)
        self.buttonBox.accepted.connect(mmqgis_geocode_reverse_form.accept)
        self.buttonBox.rejected.connect(mmqgis_geocode_reverse_form.reject)
        QtCore.QMetaObject.connectSlotsByName(mmqgis_geocode_reverse_form)

    def retranslateUi(self, mmqgis_geocode_reverse_form):
        _translate = QtCore.QCoreApplication.translate
        mmqgis_geocode_reverse_form.setWindowTitle(_translate("mmqgis_geocode_reverse_form", "Reverse Geocode"))
        self.label_4.setText(_translate("mmqgis_geocode_reverse_form", "Input Layer Name"))
        self.label_3.setText(_translate("mmqgis_geocode_reverse_form", "Web Service"))
        self.api_key_label.setText(_translate("mmqgis_geocode_reverse_form", "Google API Key"))
        self.label_6.setText(_translate("mmqgis_geocode_reverse_form", "Duplicate Results Handling"))
        self.label.setText(_translate("mmqgis_geocode_reverse_form", "Output File Name"))

from qgis import gui
