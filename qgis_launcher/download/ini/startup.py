# startup.py - QGIS起動時に自動実行されるスクリプト
# PORTAL_USERROLE 環境変数からユーザーロールを取得し、ロールに応じた設定を行う。
# ロール: Viewer / Editor / Administrator

import os
from qgis.core import QgsMessageLog, Qgis
from qgis.PyQt.QtCore import QTimer
from qgis.PyQt.QtWidgets import QToolBar

def apply_role_settings():
    from qgis.utils import iface
    if iface is None:
        return

    role = os.environ.get("PORTAL_USERROLE", "Viewer").strip()
    QgsMessageLog.logMessage(f"Portal role: {role}", "Portal", Qgis.Info)

    try:
        if role.lower() == "viewer":
            # 編集ツールバーを非表示
            toolbar = iface.mainWindow().findChild(QToolBar, "mDigitizeToolBar")
            if toolbar:
                toolbar.setVisible(False)
            # 編集モードへの切り替えを無効化
            iface.actionToggleEditing().setEnabled(False)
            iface.actionSaveEdits().setEnabled(False)

        elif role.lower() in ("editor", "administrator"):
            # 編集ツールバーを表示（Viewer から切り替えた場合に非表示になっている可能性がある）
            toolbar = iface.mainWindow().findChild(QToolBar, "mDigitizeToolBar")
            if toolbar:
                toolbar.setVisible(True)
            # 編集ツールを有効にする
            iface.actionToggleEditing().setEnabled(True)
            iface.actionSaveEdits().setEnabled(True)

    except Exception as e:
        QgsMessageLog.logMessage(f"startup.py error: {e}", "Portal", Qgis.Warning)

# iface が利用可能になるまで少し待ってから実行
try:
    QTimer.singleShot(500, apply_role_settings)
except Exception as e:
    QgsMessageLog.logMessage(f"startup.py init error: {e}", "Portal", Qgis.Warning)
