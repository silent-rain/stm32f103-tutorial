fn main() {
    let mut b = freertos_cargo_build::Builder::new();

    // Path to FreeRTOS kernel or set ENV "FREERTOS_SRC" instead
    b.freertos("./FreeRTOS-KernelV10.6.1");
    // Location of `FreeRTOSConfig.h`
    b.freertos_config("src");
    // Port dir relativ to 'FreeRTOS-Kernel/portable'
    b.freertos_port("GCC/ARM_CM3".to_owned());
    // Set the heap_?.c allocator to use from
    // 'FreeRTOS-Kernel/portable/MemMang' (Default: heap_4.c)
    b.heap::<&str>("heap_4.c".to_owned());
    // Optional additional C-Code to be compiled
    // b.get_cc().file("More.c");

    b.compile().unwrap_or_else(|e| panic!("{}", e.to_string()));
}
