# -*- coding: utf-8 -*-

import sys
from sys import version_info
from appium import webdriver
from appium.webdriver.common.touch_action import TouchAction
from appium.webdriver.common.multi_action import MultiAction


import time
import random
import sys

print("所使用的Python路径：%s" % sys.executable)

global_config = {
    # 'max_urls_count': random.randint(12, 24),  # 一般情况下， 持续2个小时，是相当长的了
    'max_urls_count': random.randint(120, 240),  # 一般情况下， 持续2个小时，是相当长的了
    'run_count': 0,
}

def sys_exit(message):
    """
    系统退出
    """
    print(message)

    print("系统即将退出 ...")
    time.sleep(random.randint(3, 5))

    sys.exit(0)


def siample_touch(driver):
    try:
        eye1 = TouchAction(driver)
        eye1.press(x=random.randint(100, 150),
                y=random.randint(100, 150)).release()
        time.sleep(1)
    except Exception as err:
        print(err)
    finally:
        pass



def auto_scroll_page(driver):
    """
    自动从上到下滚动，处理页面滑动问题
    """
    print("启动自动滚动")
    total_width = driver.execute_script("return document.body.offsetWidth")
    total_height = driver.execute_script(
        "return document.body.parentNode.scrollHeight")
    viewport_width = driver.execute_script("return document.body.clientWidth")
    viewport_height = driver.execute_script("return window.innerHeight")
    print("Total: ({0}, {1}), Viewport: ({2},{3})".format(
        total_width, total_height, viewport_width, viewport_height))

    rectangles = []

    i = 0
    while i < total_height:
        ii = 0
        top_height = i + viewport_height

        if top_height > total_height:
            top_height = total_height

        while ii < total_width:
            top_width = ii + viewport_width

            if top_width > total_width:
                top_width = total_width

            print("Appending rectangle ({0},{1},{2},{3})".format(ii, i, top_width, top_height))
            rectangles.append((ii, i, top_width,top_height))

            ii = ii + viewport_width

        i = i + viewport_height

    previous = None
    part = 0

    for rectangle in rectangles:
        if not previous is None:
            driver.execute_script("window.scrollTo({0}, {1})".format(rectangle[0], rectangle[1]))
            print("Scrolled To ({0},{1})".format(rectangle[0], rectangle[1]))
            time.sleep(0.2)

        if rectangle[1] + viewport_height > total_height:
            offset = (rectangle[0], total_height - viewport_height)
        else:
            offset = (rectangle[0], rectangle[1])

        part = part + 1
        previous = rectangle

    print("Finishing chrome full page scroll workaround...")
    return True


def starup(want_open_url):

    has_error = False
    try:
        desired_caps = {
            'platformName': 'Android',
            'platformVersion': '5.1.1',
            'deviceName': 'Samsung Galaxy S7',
            # 'unicodeKeyboard': True,
            'browserName': 'Chrome',
            # 'appPackage': 'com.android.chrome',
            'clearSystemFiles': True,
            # 'noReset': True,
            # 'appActivity': 'BrowserActivity',
            'newCommandTimeout': 18000,
            # 'appWaitDuration': 30000,
            'chromeOptions': {
                # 为了防止出现Chrome的欢迎界面，你需要开启模拟器或者真机的开发者Debug选项。这个选项在手机或者模拟器的“设置”中
                'args': [
                    '--disable-fre',
                    '--disable-popup-blocking',
                    '--disable-infobars',
                    '--allow-running-insecure-content',
                    '--no-first-run',
                    '--disable-web-security',
                    # '--user-data-dir=/data/data/com.android.chrome/cache',
                    '--test-type'
                    ]
            },
        }

        print("启动 webdriver ...")
        driver = webdriver.Remote('http://localhost:4723/wd/hub', desired_caps)

        print("打开 %s" % want_open_url)
        driver.get(want_open_url)

        print("正在获取当前环境 ...")
        # 获取当前上下文环境
        # current_context = driver.current_context
        # contexts = driver.contexts

        # 可以滚动一下
        print("网页准备好，准备一下，可以滚动一下 ...")
        siample_touch(driver)
        auto_scroll_page(driver)

        # 休息一会
        print("先休息一会儿，让网页自己静静...")
        min_sleep_secs = random.randint(90, 150)
        time.sleep(min_sleep_secs)

        # 浏览完成后，可以关闭了
        print("浏览完成后，可以关闭该网页了...")

        # print(contexts)
        # print(current_context)

        # # 切换上下文
        # driver.switch_to.context("NATIVE_APP")

        # at = driver.current_context

        # print(at)

        # driver.find_element_by_id('com.android.browser:id/search_box_collapsed').click()
        # search_box = driver.find_element_by_id('com.android.browser:id/search_view')
        # search_box.click()
        # search_box.send_keys('hello toby')
        has_error = False
        global_config['run_count'] = global_config['run_count'] + 1
    except Exception as identifier:
        print(identifier)
        has_error = True
    finally:
        print("当前已经打开的网页数量 = %d" % global_config['run_count'])
        driver.quit()
        if has_error:
            # 重新开启刷屏处理
            time.sleep(random.randint(10, 15))
            boot()
        if global_config['run_count'] > global_config['max_urls_count']:
            sys_exit("打开的网页数量已经达到最大的要求")



def boot():
    with open("urls.txt", "r+") as fhandler:

        all_urls = []
        while True:
            file_url = fhandler.readline()
            if not file_url:
                break
            format_url = file_url.replace('\n', '')
            all_urls.append(format_url)

        sort_indexs = []
        for index in range(len(all_urls)):
            sort_indexs.append(index)

        print("当前需要处理的网页数量为： %d" % len(sort_indexs))

        # 让列表乱序处理
        random.shuffle(sort_indexs)
        for cur_index in sort_indexs:
            cur_url = all_urls[cur_index]
            starup(cur_url)
            print("准备测试下一个网页 ...")
            time.sleep(random.randint(10, 15))

if __name__ == "__main__":
    boot()
