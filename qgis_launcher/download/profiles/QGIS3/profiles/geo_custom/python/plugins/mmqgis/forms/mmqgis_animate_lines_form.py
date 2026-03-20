# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/mmqgis_animate_lines_form.ui'
#
# Created by: PyQt5 UI code generator 5.7
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_mmqgis_animate_lines_form(object):
    def setupUi(self, mmqgis_animate_lines_form):
        mmqgis_animate_lines_form.setObjectName("mmqgis_animate_lines_form")
        mmqgis_animate_lines_form.setWindowModality(QtCore.Qt.ApplicationModal)
        mmqgis_animate_lines_form.setEnabled(True)
        mmqgis_animate_lines_form.resize(480, 335)
        mmqgis_animate_lines_form.setMouseTracking(False)
        self.verticalLayout = QtWidgets.QVBoxLayout(mmqgis_animate_lines_form)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label_5 = QtWidgets.QLabel(mmqgis_animate_lines_form)
        self.label_5.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_5.setObjectName("label_5")
        self.verticalLayout.addWidget(self.label_5)
        self.print_layout = QtWidgets.QComboBox(mmqgis_animate_lines_form)
        self.print_layout.setObjectName("print_layout")
        self.verticalLayout.addWidget(self.print_layout)
        self.label_4 = QtWidgets.QLabel(mmqgis_animate_lines_form)
        self.label_4.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_4.setObjectName("label_4")
        self.verticalLayout.addWidget(self.label_4)
        self.input_layer_name = QtWidgets.QComboBox(mmqgis_animate_lines_form)
        self.input_layer_name.setObjectName("input_layer_name")
        self.verticalLayout.addWidget(self.input_layer_name)
        self.label_6 = QtWidgets.QLabel(mmqgis_animate_lines_form)
        self.label_6.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_6.setObjectName("label_6")
        self.verticalLayout.addWidget(self.label_6)
        self.timing = QtWidgets.QComboBox(mmqgis_animate_lines_form)
        self.timing.setObjectName("timing")
        self.timing.addItem("")
        self.timing.addItem("")
        self.verticalLayout.addWidget(self.timing)
        self.label_7 = QtWidgets.QLabel(mmqgis_animate_lines_form)
        self.label_7.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_7.setObjectName("label_7")
        self.verticalLayout.addWidget(self.label_7)
        self.frame_count = QtWidgets.QLineEdit(mmqgis_animate_lines_form)
        self.frame_count.setReadOnly(False)
        self.frame_count.setObjectName("frame_count")
        self.verticalLayout.addWidget(self.frame_count)
        self.label = QtWidgets.QLabel(mmqgis_animate_lines_form)
        self.label.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)
        self.frame_directory = gui.QgsFileWidget(mmqgis_animate_lines_form)
        self.frame_directory.setObjectName("frame_directory")
        self.verticalLayout.addWidget(self.frame_directory)
        self.status = QtWidgets.QProgressBar(mmqgis_animate_lines_form)
        self.status.setProperty("value", 24)
        self.status.setObjectName("status")
        self.verticalLayout.addWidget(self.status)
        self.buttonBox = QtWidgets.QDialogButtonBox(mmqgis_animate_lines_form)
        self.buttonBox.setOrientation(QtCore.Qt.Horizontal)
        self.buttonBox.setStandardButtons(QtWidgets.QDialogButtonBox.Apply|QtWidgets.QDialogButtonBox.Close)
        self.buttonBox.setCenterButtons(True)
        self.buttonBox.setObjectName("buttonBox")
        self.verticalLayout.addWidget(self.buttonBox)

        self.retranslateUi(mmqgis_animate_lines_form)
        self.buttonBox.accepted.connect(mmqgis_animate_lines_form.accept)
        self.buttonBox.rejected.connect(mmqgis_animate_lines_form.reject)
        QtCore.QMetaObject.connectSlotsByName(mmqgis_animate_lines_form)
        mmqgis_animate_lines_form.setTabOrder(self.input_layer_name, self.timing)
        mmqgis_animate_lines_form.setTabOrder(self.timing, self.frame_count)
        mmqgis_animate_lines_form.setTabOrder(self.frame_count, self.buttonBox)

    def retranslateUi(self, mmqgis_animate_lines_form):
        _translate = QtCore.QCoreApplication.translate
        mmqgis_animate_lines_form.setWindowTitle(_translate("mmqgis_animate_lines_form", "Animate Lines"))
        self.label_5.setText(_translate("mmqgis_animate_lines_form", "Print Layout"))
        self.label_4.setText(_translate("mmqgis_animate_lines_form", "Animation Layer"))
        self.label_6.setText(_translate("mmqgis_animate_lines_form", "Timing"))
        self.timing.setItemText(0, _translate("mmqgis_animate_lines_form", "Different Line Speeds Animated Over Full Duration"))
        self.timing.setItemText(1, _translate("mmqgis_animate_lines_form", "One Line Speed Determined By Longest Line"))
        self.label_7.setText(_translate("mmqgis_animate_lines_form", "Frame Count"))
        self.frame_count.setText(_translate("mmqgis_animate_lines_form", "50"))
        self.label.setText(_translate("mmqgis_animate_lines_form", "Frame Image Output Directory"))

from qgis import gui
