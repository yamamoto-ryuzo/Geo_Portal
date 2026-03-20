# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/mmqgis_geometry_export_form.ui'
#
# Created by: PyQt5 UI code generator 5.7
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_mmqgis_geometry_export_form(object):
    def setupUi(self, mmqgis_geometry_export_form):
        mmqgis_geometry_export_form.setObjectName("mmqgis_geometry_export_form")
        mmqgis_geometry_export_form.setWindowModality(QtCore.Qt.ApplicationModal)
        mmqgis_geometry_export_form.setEnabled(True)
        mmqgis_geometry_export_form.resize(477, 293)
        mmqgis_geometry_export_form.setMouseTracking(False)
        self.verticalLayout = QtWidgets.QVBoxLayout(mmqgis_geometry_export_form)
        self.verticalLayout.setObjectName("verticalLayout")
        self.label_4 = QtWidgets.QLabel(mmqgis_geometry_export_form)
        self.label_4.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_4.setObjectName("label_4")
        self.verticalLayout.addWidget(self.label_4)
        self.input_layer_name = QtWidgets.QComboBox(mmqgis_geometry_export_form)
        self.input_layer_name.setObjectName("input_layer_name")
        self.verticalLayout.addWidget(self.input_layer_name)
        self.label_2 = QtWidgets.QLabel(mmqgis_geometry_export_form)
        self.label_2.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label_2.setObjectName("label_2")
        self.verticalLayout.addWidget(self.label_2)
        self.node_file_name = gui.QgsFileWidget(mmqgis_geometry_export_form)
        self.node_file_name.setObjectName("node_file_name")
        self.verticalLayout.addWidget(self.node_file_name)
        self.label = QtWidgets.QLabel(mmqgis_geometry_export_form)
        self.label.setAlignment(QtCore.Qt.AlignBottom|QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft)
        self.label.setObjectName("label")
        self.verticalLayout.addWidget(self.label)
        self.attribute_file_name = gui.QgsFileWidget(mmqgis_geometry_export_form)
        self.attribute_file_name.setObjectName("attribute_file_name")
        self.verticalLayout.addWidget(self.attribute_file_name)
        self.gridLayout = QtWidgets.QGridLayout()
        self.gridLayout.setObjectName("gridLayout")
        self.field_delimiter = QtWidgets.QComboBox(mmqgis_geometry_export_form)
        self.field_delimiter.setObjectName("field_delimiter")
        self.gridLayout.addWidget(self.field_delimiter, 1, 0, 1, 1)
        self.label_3 = QtWidgets.QLabel(mmqgis_geometry_export_form)
        self.label_3.setObjectName("label_3")
        self.gridLayout.addWidget(self.label_3, 0, 0, 1, 1)
        self.label_5 = QtWidgets.QLabel(mmqgis_geometry_export_form)
        self.label_5.setObjectName("label_5")
        self.gridLayout.addWidget(self.label_5, 0, 1, 1, 1)
        self.line_terminator = QtWidgets.QComboBox(mmqgis_geometry_export_form)
        self.line_terminator.setObjectName("line_terminator")
        self.gridLayout.addWidget(self.line_terminator, 1, 1, 1, 1)
        self.verticalLayout.addLayout(self.gridLayout)
        self.status = QtWidgets.QProgressBar(mmqgis_geometry_export_form)
        self.status.setProperty("value", 24)
        self.status.setObjectName("status")
        self.verticalLayout.addWidget(self.status)
        self.buttonBox = QtWidgets.QDialogButtonBox(mmqgis_geometry_export_form)
        self.buttonBox.setOrientation(QtCore.Qt.Horizontal)
        self.buttonBox.setStandardButtons(QtWidgets.QDialogButtonBox.Apply|QtWidgets.QDialogButtonBox.Close)
        self.buttonBox.setCenterButtons(True)
        self.buttonBox.setObjectName("buttonBox")
        self.verticalLayout.addWidget(self.buttonBox)

        self.retranslateUi(mmqgis_geometry_export_form)
        self.buttonBox.accepted.connect(mmqgis_geometry_export_form.accept)
        self.buttonBox.rejected.connect(mmqgis_geometry_export_form.reject)
        QtCore.QMetaObject.connectSlotsByName(mmqgis_geometry_export_form)
        mmqgis_geometry_export_form.setTabOrder(self.input_layer_name, self.field_delimiter)
        mmqgis_geometry_export_form.setTabOrder(self.field_delimiter, self.line_terminator)
        mmqgis_geometry_export_form.setTabOrder(self.line_terminator, self.buttonBox)

    def retranslateUi(self, mmqgis_geometry_export_form):
        _translate = QtCore.QCoreApplication.translate
        mmqgis_geometry_export_form.setWindowTitle(_translate("mmqgis_geometry_export_form", "Export Geometry to CSV"))
        self.label_4.setText(_translate("mmqgis_geometry_export_form", "Input Layer Name"))
        self.label_2.setText(_translate("mmqgis_geometry_export_form", "Output Nodes CSV File"))
        self.label.setText(_translate("mmqgis_geometry_export_form", "Output Attributes CSV File"))
        self.label_3.setText(_translate("mmqgis_geometry_export_form", "Delimiter"))
        self.label_5.setText(_translate("mmqgis_geometry_export_form", "Line Terminator"))

from qgis import gui
