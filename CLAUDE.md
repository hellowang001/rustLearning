# CLAUDE.md

本文件为 Claude Code（claude.ai/code）在此仓库中工作时提供指引。

## 项目概述

这是一个 Solana/Anchor 学习项目。`learn/` 目录包含一个 Anchor 工作区，用于实验各种 Solana 概念（rent、账户、PDA 等）。

## 学习目标

- 掌握 Solana 链上程序（智能合约）的开发模式
- 熟练使用 Anchor 框架编写 Rust 合约
- 理解 Solana 核心概念：账户模型、PDA、rent、CPI 等
- 学习 SPL Token 程序的使用与 CPI 调用
- 通过 TypeScript 测试验证合约行为

## 常用命令

所有命令在 `learn/` 目录下执行。

### 构建与测试

```bash
# 启动本地验证器（运行测试前必须先启动）
solana-test-validator

# 运行所有测试（连接已运行的本地验证器）
anchor test --skip-local-validator

# 仅构建
anchor build

# 按名称运行单个测试
yarn run ts-mocha -p ./tsconfig.json -t 1000000 "tests/**/*.ts" --grep "测试名称"
```

### Lint（TypeScript）

```bash
yarn lint        # 检查
yarn lint:fix    # 自动修复
```

## 项目结构

```
learn/                        # Anchor 工作区根目录
├── Anchor.toml               # 集群: localnet，钱包: ~/.config/solana/id.json
├── programs/learn/src/lib.rs # Solana 程序（Rust，Anchor 框架）
└── tests/learn.ts            # TypeScript 集成测试（@coral-xyz/anchor）
```

- **Program ID**: `8sm8zjwUwhvBLxhv1RHd64fQvdAasxoE3j57NrqSxJdr`
- **Rust 工具链**: `1.89.0`（通过 `rust-toolchain.toml` 固定版本）
- **Anchor 版本**: `^0.32.1`
- **包管理器**: `yarn`

## 工作模式

每个学习主题在 `lib.rs` 中实现为一个或多个 Anchor 指令，并在 `learn.ts` 中编写对应测试。旧的实验代码注释保留（不删除），供日后参考。