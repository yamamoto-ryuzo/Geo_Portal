# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/mmqgis_animate_sequence_form.ui'
#
# Created by: PyQt5 UI code generator 5.7
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_mmqgis_animate_sequence_form(object):
    def setupUi(self, mmqgis_animate_sequence_form):
        mmqgis_animate_sequence_form.setObjectName("mmqgis_animate_sequence_form")
        mmqgis_animate_sequence_form.setWindowModality(QtCore.Qt.ApplicationModal)
        mmqgis_animate_sequence_form.setEnabled(True)
        mmqgis_animate_sequence_form.resize(484, 400)
        mmqgis_animate_sequence_form.setMouseTracking(False)
        self.verticalLayout_4 = QtWidgets.QVBoxLayout(mmqgis_animate_sequence_form)
        self.verticalLayout_4.setObjectName("verticalLayout_4")
        self.label_6 = QtWidgets.QLabel(mmqgis_animate_sequence_form)
        self.label_6.setObjectName("label_6")
        self.verticalLayout_4.addWidget(self.label_6)
        self.print_layout = QtWidgets.QComboBox(mmqgis_animate_sequence_form)
        self.print_layout.setObjectName("print_layout")
        self.verticalLayout_4.addWidget(self.print_layout)
        self.label_5 = QtWidgets.QLabel(mmqgis_animate_sequence_form)
        self.label_5.setObjectName("label_5")
        self.verticalLayout_4.addWidget(self.label_5)
        self.input_layer_names = QtWidgets.QListWidget(mmqgis_animate_sequence_form)
        self.input_layer_names.setSelectionMode(QtWidgets.QAbstractItemView.ExtendedSelection)
        self.input_layer_names.setObjectName("input_layer_names")
        self.verticalLayout_4.addWidget(self.input_layer_names)
        self.cumulative = QtWidgets.QCheckBox(mmqgis_animate_sequence_form)
        self.cumulative.setObjectName("cumulative")
        self.verticalLayout_4.addWidget(self.cumulative)
        self.label = QtWidgets.QLabel(mmqgis_animate_sequence_form)
        self.label.setObjectName("label")
        self.verticalLayout_4.addWidget(self.label)
        self.frame_directory = gui.QgsFileWidget(mmqgis_animate_sequence_form)
        self.frame_directory.setObjectName("frame_directory")
        self.verticalLayout_4.addWidget(self.frame_directory)
        self.status = QtWidgets.QProgressBar(mmqgis_animate_sequence_form)
        self.status.setProperty("value", 24)
        self.status.setObjectName("status")
        self.verticalLayout_4.addWidget(self.status)
        self.buttonBox = QtWidgets.QDialogButtonBox(mmqgis_animate_sequence_form)
        self.buttonBox.setOrientation(QtCore.Qt.Horizontal)
        self.buttonBox.setStandardButtons(QtWidgets.QDialogButtonBox.Apply|QtWidgets.QDialogButtonBox.Close)
        self.buttonBox.setCenterButtons(True)
        self.buttonBox.setObjectName("buttonBox")
        self.verticalLayout_4.addWidget(self.buttonBox)

        self.retranslateUi(mmqgis_animate_sequence_form)
        self.buttonBox.accepted.connect(mmqgis_animate_sequence_form.accept)
        self.buttonBox.rejected.connect(mmqgis_animate_sequence_form.reject)
        QtCore.QMetaObject.connectSlotsByName(mmqgis_animate_sequence_form)

    def retranslateUi(self, mmqgis_animate_sequence_form):
        _translate = QtCore.QCoreApplication.translate
        mmqgis_animate_sequence_form.setWindowTitle(_translate("mmqgis_animate_sequence_form", "Animate Sequence"))
        self.label_6.setText(_translate("mmqgis_animate_sequence_form", "Print Layout"))
        self.label_5.setText(_translate("mmqgis_animate_sequence_form", "Layers to Animate"))
        self.cumulative.setText(_translate("mmqgis_animate_sequence_form", "Cumulative"))
        self.label.setText(_translate("mmqgis_animate_sequence_form", "Frame Image Output Directory"))

from qgis import gui
