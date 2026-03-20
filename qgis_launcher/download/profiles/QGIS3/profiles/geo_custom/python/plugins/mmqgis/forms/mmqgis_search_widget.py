# -*- coding: utf-8 -*-

# Form implementation generated from reading ui file 'forms/mmqgis_search_widget.ui'
#
# Created by: PyQt5 UI code generator 5.7
#
# WARNING! All changes made in this file will be lost!

from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_mmqgis_search_widget(object):
    def setupUi(self, mmqgis_search_widget):
        mmqgis_search_widget.setObjectName("mmqgis_search_widget")
        mmqgis_search_widget.resize(405, 282)
        self.dockWidgetContents = QtWidgets.QWidget()
        self.dockWidgetContents.setObjectName("dockWidgetContents")
        self.formLayout = QtWidgets.QFormLayout(self.dockWidgetContents)
        self.formLayout.setContentsMargins(0, 0, 0, 0)
        self.formLayout.setObjectName("formLayout")
        self.label_4 = QtWidgets.QLabel(self.dockWidgetContents)
        self.label_4.setObjectName("label_4")
        self.formLayout.setWidget(0, QtWidgets.QFormLayout.LabelRole, self.label_4)
        self.input_layer_names = QtWidgets.QComboBox(self.dockWidgetContents)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.MinimumExpanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.input_layer_names.sizePolicy().hasHeightForWidth())
        self.input_layer_names.setSizePolicy(sizePolicy)
        self.input_layer_names.setObjectName("input_layer_names")
        self.formLayout.setWidget(0, QtWidgets.QFormLayout.FieldRole, self.input_layer_names)
        self.label_3 = QtWidgets.QLabel(self.dockWidgetContents)
        self.label_3.setObjectName("label_3")
        self.formLayout.setWidget(1, QtWidgets.QFormLayout.LabelRole, self.label_3)
        self.label_6 = QtWidgets.QLabel(self.dockWidgetContents)
        self.label_6.setObjectName("label_6")
        self.formLayout.setWidget(2, QtWidgets.QFormLayout.LabelRole, self.label_6)
        self.comparison = QtWidgets.QComboBox(self.dockWidgetContents)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.comparison.sizePolicy().hasHeightForWidth())
        self.comparison.setSizePolicy(sizePolicy)
        self.comparison.setObjectName("comparison")
        self.formLayout.setWidget(2, QtWidgets.QFormLayout.FieldRole, self.comparison)
        self.label_8 = QtWidgets.QLabel(self.dockWidgetContents)
        self.label_8.setObjectName("label_8")
        self.formLayout.setWidget(3, QtWidgets.QFormLayout.LabelRole, self.label_8)
        self.value = QtWidgets.QLineEdit(self.dockWidgetContents)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.value.sizePolicy().hasHeightForWidth())
        self.value.setSizePolicy(sizePolicy)
        self.value.setObjectName("value")
        self.formLayout.setWidget(3, QtWidgets.QFormLayout.FieldRole, self.value)
        self.search = QtWidgets.QPushButton(self.dockWidgetContents)
        self.search.setObjectName("search")
        self.formLayout.setWidget(4, QtWidgets.QFormLayout.LabelRole, self.search)
        self.results = QtWidgets.QListWidget(self.dockWidgetContents)
        sizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.results.sizePolicy().hasHeightForWidth())
        self.results.setSizePolicy(sizePolicy)
        self.results.setSelectionMode(QtWidgets.QAbstractItemView.ExtendedSelection)
        self.results.setObjectName("results")
        self.formLayout.setWidget(4, QtWidgets.QFormLayout.FieldRole, self.results)
        self.attributes = QtWidgets.QComboBox(self.dockWidgetContents)
        self.attributes.setObjectName("attributes")
        self.formLayout.setWidget(1, QtWidgets.QFormLayout.FieldRole, self.attributes)
        mmqgis_search_widget.setWidget(self.dockWidgetContents)

        self.retranslateUi(mmqgis_search_widget)
        QtCore.QMetaObject.connectSlotsByName(mmqgis_search_widget)

    def retranslateUi(self, mmqgis_search_widget):
        _translate = QtCore.QCoreApplication.translate
        mmqgis_search_widget.setWindowTitle(_translate("mmqgis_search_widget", "MMQGIS Search"))
        self.label_4.setText(_translate("mmqgis_search_widget", "Layer"))
        self.label_3.setText(_translate("mmqgis_search_widget", "Attribute(s)"))
        self.label_6.setText(_translate("mmqgis_search_widget", "Comparison"))
        self.label_8.setText(_translate("mmqgis_search_widget", "Value"))
        self.search.setText(_translate("mmqgis_search_widget", "Search"))

