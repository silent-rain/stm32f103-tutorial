# Defmt 单元测试

## 安装依赖

- flip-link:

```shell
cargo install flip-link
```

- probe-run:

```shell
# make sure to install v0.2.0 or later
cargo install probe-run
```

- cargo-generate（可选）:

```shell
cargo install cargo-generate
```

## 运行测试

- 运行这些单元测试

```shell
cargo test --lib

cargo test --target thumbv7m-none-eabi -p testsuite

cargo test --target thumbv7m-none-eabi -p testsuite probe-run -- --chip STM32F103C8
```

- 集成测试
  > 集成测试驻留在该 tests 目录中；
  > 最初的一组集成测试位于 tests/integration.rs.
  > cargo test --test integration 将运行这些集成测试。
  > 请注意，标志的参数--test 必须与目录中测试文件的名称匹配 tests。

```shell
cargo test --test integration
```

## 设备调试

```shell
# 完整指令
cargo run --target thumbv7m-none-eabi -p testsuite_replace_embed probe-run -- --chip STM32F103C8 trace

# 简写, 配置的有 `.cargo/config.toml` 文件
cargo run -p testsuite_replace_embed
# 别名
cargo rp testsuite_replace_embed

# 日志级别
DEFMT_LOG=trace cargo run -p testsuite_replace_embed

```

## 设置日志级别

```shell
export DEFMT_LOG=info

# or
DEFMT_LOG=trace cargo rb levels
```

## 相关文档

- [defmt app-template](https://github.com/knurling-rs/app-template)
- [defmt](https://github.com/knurling-rs/defmt)
