# GDB调试

## 指定配置启动 GDB
```shell
arm-none-eabi-gdb -iex 'add-auto-load-safe-path .' -q target/thumbv7m-none-eabi/debug/hello
```

## 调试案例
```shell
$ # 在一个不同的终端上
$ arm-none-eabi-gdb -q target/thumbv7m-none-eabi/debug/hello
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


## 指令解释
- break main: 在此处设置断点
- delete <breakpoint-num>: 删除所需的断点
- continue/c: 继续执行程序，直到遇到断点
- layout src: 使用GDB的文本用户界面 (TUI)
- tui disable: 离开TUI模式
- print x: 使用print命令检查这些堆栈/局部变量
- print &x: 打印变量x的地址
- next: 继续执行程序
- info locals: 打印所有局部变量
- layout asm: 命令切换到反汇编视图
- stepi: 将打印语句和处理器下一步将执行的指令的行号
- Ctrl+C: 如果您错误地使用了next或continue命令，并且GDB卡住了，您可以通过点击Ctrl+C来取消卡住。
- disassemble /m: 命令围绕您当前所在的行反汇编程序。
- monitor reset: 重启程序
- load: 重新加载程序
- quit: 退出 GDB

