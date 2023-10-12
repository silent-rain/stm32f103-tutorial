# GDB 调试

## 指令解释

- target remote：连接到 OpenOCD 的 GDB 服务器
- file <file>: 加载可执行文件到 GDB 中，指定要调试的程序
- run(r): 运行程序，可以带参数
- break main: 在此处设置断点
- break(b): 设置断点，使程序在指定的源代码行中断
- watch <表达式>: 监视变量，当变量值发生变化时暂停程序
- info(i) <选项>: 显示程序状态信息
- info break: 列出所有当前的断点
- info locals: 打印所有局部变量
- backtrace(bt): 显示函数调用栈
- delete <breakpoint-num>: 删除所需的断点
- continue(c): 继续执行程序，直到遇到断点
- step(s): 逐过程单步执行程序，进入函数内部
- stepi: 单步执行汇编指令
- next(n): 单步执行程序，不进入函数内部
- print(p) <x>: 使用 print 命令检查这些堆栈/局部变量
- print(p) &x: 打印变量 x 的地址
- list: 查看源码
- list mian: 指定位置查看
- layout asm: 命令切换到反汇编视图
- layout src: 使用 GDB 的文本用户界面 (TUI)
- tui disable: 离开 TUI 模式
- Ctrl+C: 如果错误地使用了 next 或 continue 命令并且 GDB 卡住了，通过 Ctrl+C 来取消卡住。
- disassemble /m: 命令围绕您当前所在的行反汇编程序。
- monitor reset: 软复位目标设备
- monitor halt：暂停目标设备
- monitor resume：继续运行目标设备
- monitor reset halt：停止目标设备并进行软复位
- load: 加载可执行文件到目标设备
- quit(q): 退出 GDB

## 指定配置启动 GDB

指定当前目录下的.gdbinit 配置文件启动

```shell
arm-none-eabi-gdb -iex 'add-auto-load-safe-path .' -q target/thumbv7m-none-eabi/debug/hello
```

## 调试案例 1

```shell
$ # 在一个不同的终端上
$ arm-none-eabi-gdb -iex 'add-auto-load-safe-path .' -q target/thumbv7m-none-eabi/debug/hello
Reading symbols from target/thumbv7m-none-eabi/debug/app...done.

(gdb) target remote :3333
Remote debugging using :3333
Reset () at src/main.rs:8
8       pub unsafe extern "C" fn Reset() -> ! {

(gdb) # the SP has the initial value we programmed in the vector table
(gdb) print/x $sp
$1 = 0x20010000

(gdb) step
9           let _x = 42;

(gdb) step
12          loop {}

(gdb) # next we inspect the stack variable `_x`
(gdb) print _x
$2 = 42

(gdb) print &_x
$3 = (i32 *) 0x2000fffc

(gdb) quit

```

## 调试案例 2

- 如果仿真卡死, 在 Renode 中进行重启仿真
  ```shell
  (machine-0) machine Reset
  ```

```shell
# arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/turns_user_led

(gdb) target remote :3333
Remote debugging using :3333
0x08000130 in Reset ()

(gdb) load
Loading section .vector_table, size 0x130 lma 0x8000000
Loading section .text, size 0x1b78 lma 0x8000130
Loading section .rodata, size 0x55c lma 0x8001cb0
Start address 0x08000130, load size 8708
Transfer rate: 566 KB/sec, 2177 bytes/write.

(gdb) list main
10      use cortex_m_semihosting::hprintln;
11      use stm32f1xx_hal as _;
12
13      use cortex_m_rt::entry;
14
15      #[entry]
16      fn main() -> ! {
17          let mut _y;
18          let x = 42;
19          _y = x;

(gdb) break 18
Breakpoint 1 at 0x8000218: file app/hello/src/main.rs, line 18.

(gdb) c
Continuing.

Breakpoint 1, hello::__cortex_m_rt_main () at app/hello/src/main.rs:18
18          let x = 42;


(gdb) step
19          _y = x;


(gdb) print x
$1 = 42


(gdb) quit

```
