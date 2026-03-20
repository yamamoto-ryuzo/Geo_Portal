# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/mmqgis_attribute_join_form.ui'
#
# Created by: PyQt5 UI code generator 5.7
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_mmqgis_attribute_join_form(object):
    def setupUi(self, mmqgis_attribute_join_form):
        mmqgis_attribute_join_form.setObjectName("mmqgis_attribute_join_form")
        mmqgis_attribute_join_form.setWindowModality(QtCore.Qt.ApplicationModal)
        mmqgis_attribute_join_form.setEnabled(True)
        mmqgis_attribute_join_form.resize(456, 308)
        mmqgis_attribute_join_form.setMouseTracking(False)
        self.verticalLayout = QtWidgets.QVBoxLayout(mmqgis_attribute_join_form)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label_4 = QtWidgets.QLabel(mmqgis_attribute_join_form)
        self.label_4.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_4.setObjectName("label_4")
        self.verticalLayout.addWidget(self.label_4)
        self.input_layer_name = QtWidgets.QComboBox(mmqgis_attribute_join_form)
        self.input_layer_name.setObjectName("input_layer_name")
        self.verticalLayout.addWidget(self.input_layer_name)
        self.label_3 = QtWidgets.QLabel(mmqgis_attribute_join_form)
        self.label_3.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_3.setObjectName("label_3")
        self.verticalLayout.addWidget(self.label_3)
        self.input_layer_attribute = QtWidgets.QComboBox(mmqgis_attribute_join_form)
        self.input_layer_attribute.setObjectName("input_layer_attribute")
        self.verticalLayout.addWidget(self.input_layer_attribute)
        self.label_2 = QtWidgets.QLabel(mmqgis_attribute_join_form)
        self.label_2.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_2.setObjectName("label_2")
        self.verticalLayout.addWidget(self.label_2)
        self.input_csv_name = gui.QgsFileWidget(mmqgis_attribute_join_form)
        self.input_csv_name.setObjectName("input_csv_name")
        self.verticalLayout.addWidget(self.input_csv_name)
        self.label_6 = QtWidgets.QLabel(mmqgis_attribute_join_form)
        self.label_6.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_6.setObjectName("label_6")
        self.verticalLayout.addWidget(self.label_6)
        self.input_csv_attribute = QtWidgets.QComboBox(mmqgis_attribute_join_form)
        self.input_csv_attribute.setObjectName("input_csv_attribute")
        self.verticalLayout.addWidget(self.input_csv_attribute)
        self.label = QtWidgets.QLabel(mmqgis_attribute_join_form)
        self.label.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)
        self.output_file_name = gui.QgsFileWidget(mmqgis_attribute_join_form)
        self.output_file_name.setObjectName("output_file_name")
        self.verticalLayout.addWidget(self.output_file_name)
        self.status = QtWidgets.QProgressBar(mmqgis_attribute_join_form)
        self.status.setProperty("value", 24)
        self.status.setObjectName("status")
        self.verticalLayout.addWidget(self.status)
        self.buttonBox = QtWidgets.QDialogButtonBox(mmqgis_attribute_join_form)
        self.buttonBox.setOrientation(QtCore.Qt.Horizontal)
        self.buttonBox.setStandardButtons(QtWidgets.QDialogButtonBox.Apply|QtWidgets.QDialogButtonBox.Close)
        self.buttonBox.setCenterButtons(True)
        self.buttonBox.setObjectName("buttonBox")
        self.verticalLayout.addWidget(self.buttonBox)

        self.retranslateUi(mmqgis_attribute_join_form)
        self.buttonBox.accepted.connect(mmqgis_attribute_join_form.accept)
        self.buttonBox.rejected.connect(mmqgis_attribute_join_form.reject)
        QtCore.QMetaObject.connectSlotsByName(mmqgis_attribute_join_form)
        mmqgis_attribute_join_form.setTabOrder(self.input_csv_attribute, self.buttonBox)

    def retranslateUi(self, mmqgis_attribute_join_form):
        _translate = QtCore.QCoreApplication.translate
        mmqgis_attribute_join_form.setWindowTitle(_translate("mmqgis_attribute_join_form", "Join by Attribute"))
        self.label_4.setText(_translate("mmqgis_attribute_join_form", "Join to Layer"))
        self.label_3.setText(_translate("mmqgis_attribute_join_form", "Join Layer Attribute"))
        self.label_2.setText(_translate("mmqgis_attribute_join_form", "Input CSV File (UTF-8)"))
        self.input_csv_name.setFilter(_translate("mmqgis_attribute_join_form", "CSV File (*.csv *.txt)"))
        self.label_6.setText(_translate("mmqgis_attribute_join_form", "CSV File Field"))
        self.label.setText(_translate("mmqgis_attribute_join_form", "Output File Name"))
        self.output_file_name.setFilter(_translate("mmqgis_attribute_join_form", "Shapefile (*.shp)"))

from qgis import gui
