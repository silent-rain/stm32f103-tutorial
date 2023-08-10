# GDB 调试

## 指定配置启动 GDB

指定当前目录下的.gdbinit 配置文件启动

```shell
arm-none-eabi-gdb -iex 'add-auto-load-safe-path .' -q target/thumbv7m-none-eabi/debug/hello
```

## 调试案例

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

## 案例2
- 如果仿真卡死, 在Renode 中进行重启仿真 
  ```shell
  (machine-0) machine Reset
  ```


```shell
# arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/hello

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


## 指令解释

- break main: 在此处设置断点
- info break: 列出所有当前的断点
- delete <breakpoint-num>: 删除所需的断点
- continue/c: 继续执行程序，直到遇到断点
- layout src: 使用 GDB 的文本用户界面 (TUI)
- tui disable: 离开 TUI 模式
- print x: 使用 print 命令检查这些堆栈/局部变量
- print &x: 打印变量 x 的地址
- list: 查看源码
- list mian: 指定位置查看
- next: 继续执行程序
- info locals: 打印所有局部变量
- layout asm: 命令切换到反汇编视图
- stepi: 将打印语句和处理器下一步将执行的指令的行号
- Ctrl+C: 如果您错误地使用了 next 或 continue 命令，并且 GDB 卡住了，您可以通过点击 Ctrl+C 来取消卡住。
- disassemble /m: 命令围绕您当前所在的行反汇编程序。
- monitor reset: 重启程序
- load: 重新加载程序
- quit: 退出 GDB
