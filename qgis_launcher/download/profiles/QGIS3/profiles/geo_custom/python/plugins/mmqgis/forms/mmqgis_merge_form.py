# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/mmqgis_merge_form.ui'
#
# Created by: PyQt5 UI code generator 5.7
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_mmqgis_merge_form(object):
    def setupUi(self, mmqgis_merge_form):
        mmqgis_merge_form.setObjectName("mmqgis_merge_form")
        mmqgis_merge_form.resize(410, 323)
        self.verticalLayout = QtWidgets.QVBoxLayout(mmqgis_merge_form)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label = QtWidgets.QLabel(mmqgis_merge_form)
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)
        self.input_layer_names = QtWidgets.QListWidget(mmqgis_merge_form)
        self.input_layer_names.setSelectionMode(QtWidgets.QAbstractItemView.ExtendedSelection)
        self.input_layer_names.setObjectName("input_layer_names")
        item = QtWidgets.QListWidgetItem()
        self.input_layer_names.addItem(item)
        item = QtWidgets.QListWidgetItem()
        self.input_layer_names.addItem(item)
        item = QtWidgets.QListWidgetItem()
        self.input_layer_names.addItem(item)
        self.verticalLayout.addWidget(self.input_layer_names)
        self.label_2 = QtWidgets.QLabel(mmqgis_merge_form)
        self.label_2.setObjectName("label_2")
        self.verticalLayout.addWidget(self.label_2)
        self.output_file_name = gui.QgsFileWidget(mmqgis_merge_form)
        self.output_file_name.setObjectName("output_file_name")
        self.verticalLayout.addWidget(self.output_file_name)
        self.status = QtWidgets.QProgressBar(mmqgis_merge_form)
        self.status.setProperty("value", 24)
        self.status.setObjectName("status")
        self.verticalLayout.addWidget(self.status)
        self.buttonBox = QtWidgets.QDialogButtonBox(mmqgis_merge_form)
        self.buttonBox.setOrientation(QtCore.Qt.Horizontal)
        self.buttonBox.setStandardButtons(QtWidgets.QDialogButtonBox.Apply|QtWidgets.QDialogButtonBox.Close)
        self.buttonBox.setCenterButtons(True)
        self.buttonBox.setObjectName("buttonBox")
        self.verticalLayout.addWidget(self.buttonBox)

        self.retranslateUi(mmqgis_merge_form)
        self.buttonBox.accepted.connect(mmqgis_merge_form.accept)
        self.buttonBox.rejected.connect(mmqgis_merge_form.reject)
        QtCore.QMetaObject.connectSlotsByName(mmqgis_merge_form)

    def retranslateUi(self, mmqgis_merge_form):
        _translate = QtCore.QCoreApplication.translate
        mmqgis_merge_form.setWindowTitle(_translate("mmqgis_merge_form", "Dialog"))
        self.label.setText(_translate("mmqgis_merge_form", "Select Source Layers"))
        __sortingEnabled = self.input_layer_names.isSortingEnabled()
        self.input_layer_names.setSortingEnabled(False)
        item = self.input_layer_names.item(0)
        item.setText(_translate("mmqgis_merge_form", "alpha"))
        item = self.input_layer_names.item(1)
        item.setText(_translate("mmqgis_merge_form", "beta"))
        item = self.input_layer_names.item(2)
        item.setText(_translate("mmqgis_merge_form", "gamma"))
        self.input_layer_names.setSortingEnabled(__sortingEnabled)
        self.label_2.setText(_translate("mmqgis_merge_form", "Output File Name"))

from qgis import gui
