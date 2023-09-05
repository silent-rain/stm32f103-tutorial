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
}
