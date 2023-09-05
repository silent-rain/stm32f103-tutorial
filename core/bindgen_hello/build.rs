use std::path::PathBuf;

use bindgen::CargoCallbacks;

fn main() {
    cc::Build::new()
        // 编译单个文件
        // .file("hello/hello.c")
        // 正则匹配多个编译文件
        .files(glob::glob("hello/*.c").unwrap().filter_map(Result::ok))
        // 将多个目录添加到包含路径。-I
        .includes(&["hello"])
        // 配置所有对象文件和静态文件的输出目录
        // .out_dir("out_dir")
        // 设置编译目标, 默认会自动从环境中抓取 TARGET 变量
        // .target("thumbv7m-none-eabi")
        // 定义是否应为货物发出元数据，使其能够 自动链接二进制文件。
        .cargo_metadata(true)
        // 运行编译器，生成output文件
        .compile("hello");

    // bindgen:：Builder是bindgen的主要入口点，允许您为生成的绑定构建选项。
    let bindings = bindgen::Builder::default()
        // 我们要为其生成绑定的输入标头。
        .header("hello/hello.h")
        .header("hello/bar.h")
        .use_core()
        .bitfield_enum("USE_STDPERIPH_DRIVER")
        .bitfield_enum("STM32F10X_MD")
        // 当包含的任何头文件发生变化时，告诉货物使已构建的板条箱失效。
        .parse_callbacks(Box::new(CargoCallbacks))
        // 完成构建器并生成绑定。
        .generate()
        .expect("Unable to generate bindings");

    // 将绑定写入 bindings.rs 文件。
    let out_path = PathBuf::from("./hello");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
