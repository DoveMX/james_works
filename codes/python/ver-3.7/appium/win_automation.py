# -*- coding: utf-8 -*-
import sys
from sys import version_info
from pprint import pprint
from subprocess import Popen
import uiautomation

import pywinauto
from pywinauto.application import Application
from pywinauto import Desktop

"""
主要目的是操作Android模拟器多开机制，
以下内容需要点击处理：

一、克隆新的模拟器
二、修改新的模拟器中的配置
（1）手机的运营商选择
（2）手机的型号选择
（3）手机的号码及内置ID点击生成
（4）手机的网络Wifi名称及网络MAC地址生成
（5）启动模拟器，并变更地理信息
三、启动后，交给Appium来处理
四、关闭Appium测试脚本，关闭该模拟器，重新执行第二部分内容
"""

# 环境检查，由于本例子中操作的是Win32位应用程序，所以，需要判断Python的版本是否是32位的，不能使用64位的
# 说明，操作64位的应用程序，按情况下，应该是使用Python的64位版本

# 系统默认中安装的位置是：
# 32位：
# "C:\Program Files (x86)\Python37-32\python.exe"
# "C:\Program Files (x86)\Python37-32\Scripts\pip.exe"

print("所使用的Python路径：%s" % sys.executable)


def mustBePython_32Bit():

    return False


def process_by_pywinauto():
    """
    核心处理
    """
    print("启动 win 自动化操作 ...")

    # 先查找是否 “逍遥安卓 - 多开管理器” 已经运行
    app = Application().connect(title_re="*MEmuConsole",
                                path="C:\Program Files\Microvirt\MEmu\MEmuConsole.exe", class_name="Qt5QWindowIcon")

    if not app:
        print("没有找到 “逍遥安卓 - 多开管理器”")
        return

    pprint(app.windows())

    app_window = app.window(best_match="MEmuConsole", class_name="Qt5QWindowIcon")
    app_window = app.window( class_name="Qt5QWindowIcon")
    pprint(app_window)

    # 设置为窗口焦点
    pprint("设置焦点")
    app_window.set_focus()

    pprint(app_window.children())

    #customControl = app_window.child_window(control_type="CustomControl")
    #pprint(customControl)

    #customControl.print_control_identifiers(depth=4)


    pass


    # 找到后，查看模拟器的运行情况

    # 点击模拟器的相关配置按钮

def process_by_uiautomaiton():
    pprint("call process_by_uiautomaiton")

    pprint(uiautomation.GetRootControl())

    pass

if __name__ == "__main__":
    use_methods = 1
    if use_methods == 1:
        process_by_pywinauto()
    elif use_methods == 2:
        process_by_uiautomaiton()
    else:
        pass