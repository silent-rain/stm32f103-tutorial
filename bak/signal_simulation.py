# -*- encoding:utf-8 -*-
"""信号模拟


# 外设
uart6: Python.PythonPeripheral @ sysbus 0x7000F410
    size: 0x4
    initable: true
    filename: "/home/one/Documents/code/RustEmbedProject/stm32f103-tutorial/bak/signal_simulation.py"

# 写入文件
scripts/platforms/cpus/stm32f103.repl
"""

print("xxx")
if request.isInit:
    lastVal = 0
elif request.isRead:
    request.value = lastVal
elif request.isWrite:
    lastVal = request.value

self.NoisyLog("%s on REPEATER at 0x%x, value 0x%x" %
              (str(request.type), request.offset, request.value))
