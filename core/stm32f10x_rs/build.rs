use std::path::PathBuf;

use bindgen::{CargoCallbacks, EnumVariation};

fn main() {
    // 编译文件
    let files = [
        "stm32f10x/Library/stm32f10x_rcc.c",
        "stm32f10x/Library/stm32f10x_gpio.c",
        "stm32f10x/System/delay.c",
    ];

    // 包含路径
    let includes = [
        "stm32f10x/Start",
        "stm32f10x/Library",
        "stm32f10x/System",
        "stm32f10x/Conf",
    ];
    let gen_includes = [
        "-Istm32f10x/Start",
        "-Istm32f10x/Library",
        "-Istm32f10x/System",
        "-Istm32f10x/Conf",
    ];
    let gen_defines = ["-DUSE_STDPERIPH_DRIVER", "-DSTM32F10X_MD"];

    cc::Build::new()
        .includes(&includes)
        // .flag("-ffreestanding")
        // .files(glob::glob("hello/*.c").unwrap().filter_map(Result::ok))
        .files(&files)
        .define("USE_STDPERIPH_DRIVER", None)
        .define("STM32F10X_MD", None)
        // .target("thumbv7m-none-eabi")
        .cargo_metadata(true)
        // 运行编译器，生成output文件
        .compile("stm32f10x_rs");

    let bindings = bindgen::Builder::default()
        .header("stm32f10x/Conf/stm32f10x_conf.h")
        .header("stm32f10x/Start/stm32f10x.h")
        .header("stm32f10x/System/delay.h")
        // 常量
        // .constified_enum("RCC_APB2Periph")
        // 枚举
        .bitfield_enum("FunctionalState")
        .generate_comments(true)
        .use_core()
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: true,
        })
        .parse_callbacks(Box::new(CargoCallbacks))
        .clang_args(gen_includes)
        .clang_args(gen_defines)
        .generate()
        .expect("Unable to generate bindings");

    // 将绑定写入 bindings.rs 文件。
    let out_path = PathBuf::from("./src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
