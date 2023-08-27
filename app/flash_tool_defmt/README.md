# 烧录工具 probe-run

这是一个使用 probe-run 命令来处理嵌入式目标的示例。在这里也会了解到使用 defmt 进行终端打印的用法。

在工作空间中执行需要将 `.cargo/config_defmt.toml` 替换为 `.cargo/config.toml` 文件。

## 执行指令

```shell
cargo rp flash_tool_defmt
# or
DEFMT_LOG=trace cargo rp flash_tool_defmt
```

## 学习目标

- 了解 probe-run 配置
- 了解 probe-run 烧录命令
- 了解 Defmt RTT 终端打印
