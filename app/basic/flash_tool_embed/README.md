# 烧录工具 Embed

这是一个使用 Embed 命令来处理嵌入式目标的示例。

在工作空间中执行需要将 `.cargo/config_embed.toml` 替换为 `.cargo/config.toml` 文件。

如果需要打印信息到终端注意配置 `Embed.toml` 文件。

## 执行指令

```shell
cargo embed --target thumbv7m-none-eabi -p flash_tool_embed
```

## 学习目标

- 了解 Embed 配置
- 了解 Embed 烧录命令
- 了解 RTT 终端打印
