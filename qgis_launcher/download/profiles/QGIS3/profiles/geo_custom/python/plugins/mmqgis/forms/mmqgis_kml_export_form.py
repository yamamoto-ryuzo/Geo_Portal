# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/mmqgis_kml_export_form.ui'
#
# Created by: PyQt5 UI code generator 5.7
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_mmqgis_kml_export_form(object):
    def setupUi(self, mmqgis_kml_export_form):
        mmqgis_kml_export_form.setObjectName("mmqgis_kml_export_form")
        mmqgis_kml_export_form.setWindowModality(QtCore.Qt.ApplicationModal)
        mmqgis_kml_export_form.setEnabled(True)
        mmqgis_kml_export_form.resize(508, 395)
        mmqgis_kml_export_form.setMouseTracking(False)
        self.verticalLayout = QtWidgets.QVBoxLayout(mmqgis_kml_export_form)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label_4 = QtWidgets.QLabel(mmqgis_kml_export_form)
        self.label_4.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_4.setObjectName("label_4")
        self.verticalLayout.addWidget(self.label_4)
        self.input_layer_name = QtWidgets.QComboBox(mmqgis_kml_export_form)
        self.input_layer_name.setObjectName("input_layer_name")
        self.verticalLayout.addWidget(self.input_layer_name)
        self.gridLayout = QtWidgets.QGridLayout()
        self.gridLayout.setObjectName("gridLayout")
        self.separator = QtWidgets.QComboBox(mmqgis_kml_export_form)
        self.separator.setObjectName("separator")
        self.gridLayout.addWidget(self.separator, 2, 1, 1, 1)
        self.name_field = QtWidgets.QComboBox(mmqgis_kml_export_form)
        self.name_field.setObjectName("name_field")
        self.gridLayout.addWidget(self.name_field, 2, 0, 1, 1)
        self.label_3 = QtWidgets.QLabel(mmqgis_kml_export_form)
        self.label_3.setObjectName("label_3")
        self.gridLayout.addWidget(self.label_3, 1, 0, 1, 1)
        self.label_5 = QtWidgets.QLabel(mmqgis_kml_export_form)
        self.label_5.setObjectName("label_5")
        self.gridLayout.addWidget(self.label_5, 1, 1, 1, 1)
        self.verticalLayout.addLayout(self.gridLayout)
        self.label_2 = QtWidgets.QLabel(mmqgis_kml_export_form)
        self.label_2.setObjectName("label_2")
        self.verticalLayout.addWidget(self.label_2)
        self.description = QtWidgets.QTextEdit(mmqgis_kml_export_form)
        self.description.setObjectName("description")
        self.verticalLayout.addWidget(self.description)
        self.export_data = QtWidgets.QCheckBox(mmqgis_kml_export_form)
        self.export_data.setObjectName("export_data")
        self.verticalLayout.addWidget(self.export_data)
        self.label = QtWidgets.QLabel(mmqgis_kml_export_form)
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)
        self.output_file_name = gui.QgsFileWidget(mmqgis_kml_export_form)
        self.output_file_name.setObjectName("output_file_name")
        self.verticalLayout.addWidget(self.output_file_name)
        self.status = QtWidgets.QProgressBar(mmqgis_kml_export_form)
        self.status.setProperty("value", 24)
        self.status.setObjectName("status")
        self.verticalLayout.addWidget(self.status)
        self.buttonBox = QtWidgets.QDialogButtonBox(mmqgis_kml_export_form)
        self.buttonBox.setOrientation(QtCore.Qt.Horizontal)
        self.buttonBox.setStandardButtons(QtWidgets.QDialogButtonBox.Apply|QtWidgets.QDialogButtonBox.Close)
        self.buttonBox.setCenterButtons(True)
        self.buttonBox.setObjectName("buttonBox")
        self.verticalLayout.addWidget(self.buttonBox)

        self.retranslateUi(mmqgis_kml_export_form)
        self.buttonBox.accepted.connect(mmqgis_kml_export_form.accept)
        self.buttonBox.rejected.connect(mmqgis_kml_export_form.reject)
        QtCore.QMetaObject.connectSlotsByName(mmqgis_kml_export_form)
        mmqgis_kml_export_form.setTabOrder(self.input_layer_name, self.name_field)
        mmqgis_kml_export_form.setTabOrder(self.name_field, self.separator)
        mmqgis_kml_export_form.setTabOrder(self.separator, self.buttonBox)

    def retranslateUi(self, mmqgis_kml_export_form):
        _translate = QtCore.QCoreApplication.translate
        mmqgis_kml_export_form.setWindowTitle(_translate("mmqgis_kml_export_form", "Google Maps KML Export"))
        self.label_4.setText(_translate("mmqgis_kml_export_form", "Input Layer"))
        self.label_3.setText(_translate("mmqgis_kml_export_form", "Placemark Name Field"))
        self.label_5.setText(_translate("mmqgis_kml_export_form", "Description Separator"))
        self.label_2.setText(_translate("mmqgis_kml_export_form", "Placemark Description HTML"))
        self.export_data.setText(_translate("mmqgis_kml_export_form", "Export attributes as <Data> (importable by QGIS)"))
        self.label.setText(_translate("mmqgis_kml_export_form", "Output KML File"))
        self.output_file_name.setFilter(_translate("mmqgis_kml_export_form", "Shapefile (*.shp)"))

from qgis import gui
