# GDB调试

## 调试案例
```shell
$ # 在一个不同的终端上
$ arm-none-eabi-gdb -q target/thumbv7m-none-eabi/debug/app
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
