# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/mmqgis_gridify_form.ui'
#
# Created by: PyQt5 UI code generator 5.7
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_mmqgis_gridify_form(object):
    def setupUi(self, mmqgis_gridify_form):
        mmqgis_gridify_form.setObjectName("mmqgis_gridify_form")
        mmqgis_gridify_form.resize(481, 214)
        self.verticalLayout = QtWidgets.QVBoxLayout(mmqgis_gridify_form)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label = QtWidgets.QLabel(mmqgis_gridify_form)
        self.label.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)
        self.input_layer_name = QtWidgets.QComboBox(mmqgis_gridify_form)
        self.input_layer_name.setObjectName("input_layer_name")
        self.verticalLayout.addWidget(self.input_layer_name)
        self.gridLayout = QtWidgets.QGridLayout()
        self.gridLayout.setObjectName("gridLayout")
        self.label_2 = QtWidgets.QLabel(mmqgis_gridify_form)
        self.label_2.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_2.setObjectName("label_2")
        self.gridLayout.addWidget(self.label_2, 0, 0, 1, 1)
        self.label_3 = QtWidgets.QLabel(mmqgis_gridify_form)
        self.label_3.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_3.setObjectName("label_3")
        self.gridLayout.addWidget(self.label_3, 0, 1, 1, 1)
        self.horizontal_spacing = QtWidgets.QLineEdit(mmqgis_gridify_form)
        self.horizontal_spacing.setObjectName("horizontal_spacing")
        self.gridLayout.addWidget(self.horizontal_spacing, 1, 0, 1, 1)
        self.vertical_spacing = QtWidgets.QLineEdit(mmqgis_gridify_form)
        self.vertical_spacing.setObjectName("vertical_spacing")
        self.gridLayout.addWidget(self.vertical_spacing, 1, 1, 1, 1)
        self.verticalLayout.addLayout(self.gridLayout)
        self.label_4 = QtWidgets.QLabel(mmqgis_gridify_form)
        self.label_4.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_4.setObjectName("label_4")
        self.verticalLayout.addWidget(self.label_4)
        self.output_file_name = gui.QgsFileWidget(mmqgis_gridify_form)
        self.output_file_name.setObjectName("output_file_name")
        self.verticalLayout.addWidget(self.output_file_name)
        self.status = QtWidgets.QProgressBar(mmqgis_gridify_form)
        self.status.setProperty("value", 24)
        self.status.setObjectName("status")
        self.verticalLayout.addWidget(self.status)
        self.buttonBox = QtWidgets.QDialogButtonBox(mmqgis_gridify_form)
        self.buttonBox.setOrientation(QtCore.Qt.Horizontal)
        self.buttonBox.setStandardButtons(QtWidgets.QDialogButtonBox.Apply|QtWidgets.QDialogButtonBox.Close)
        self.buttonBox.setCenterButtons(True)
        self.buttonBox.setObjectName("buttonBox")
        self.verticalLayout.addWidget(self.buttonBox)

        self.retranslateUi(mmqgis_gridify_form)
        self.buttonBox.accepted.connect(mmqgis_gridify_form.accept)
        self.buttonBox.rejected.connect(mmqgis_gridify_form.reject)
        QtCore.QMetaObject.connectSlotsByName(mmqgis_gridify_form)

    def retranslateUi(self, mmqgis_gridify_form):
        _translate = QtCore.QCoreApplication.translate
        mmqgis_gridify_form.setWindowTitle(_translate("mmqgis_gridify_form", "Gridify"))
        self.label.setText(_translate("mmqgis_gridify_form", "Input Layer"))
        self.label_2.setText(_translate("mmqgis_gridify_form", "Horizontal Spacing"))
        self.label_3.setText(_translate("mmqgis_gridify_form", "Vertical Spacing"))
        self.label_4.setText(_translate("mmqgis_gridify_form", "Output File Name"))

from qgis import gui
