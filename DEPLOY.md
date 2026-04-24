# Solana 合约部署指南

## 部署到 Devnet

### 第一步：切换网络

```bash
solana config set --url devnet

# 确认配置
solana config get
# RPC URL 应显示 https://api.devnet.solana.com
```

---

### 第二步：确保钱包有足够 SOL

```bash
solana balance
```

余额不足时领取水龙头：

```bash
solana airdrop 2
```

> 如果命令行水龙头限流，去网页申请：https://faucet.solana.com

首次部署一个小程序大约消耗 2–4 SOL（devnet SOL 无实际价值）。

---

### 第三步：修改 Anchor.toml

```toml
[provider]
cluster = "devnet"          # 从 localnet 改为 devnet
wallet = "~/.config/solana/id.json"
```

---

### 第四步：构建

```bash
cd learn
anchor build
```

---

### 第五步：同步 Program ID（首次部署执行一次）

```bash
cd learn
anchor keys sync
```

作用：将 `Anchor.toml` 和 `lib.rs` 中的 `declare_id!` 与本地 keypair 文件对齐，避免 Program ID 不一致导致部署失败。

---

### 第六步：部署

```bash
anchor deploy
```

成功输出示例：

```
Program Id: 8sm8zjwUwhvBLxhv1RHd64fQvdAasxoE3j57NrqSxJdr
Deploy success
```

---

### 第七步：验证部署

在浏览器中查看：

```
https://explorer.solana.com/address/<Program ID>?cluster=devnet
```

---

### 部署后运行测试

`Anchor.toml` 设置为 devnet 后，测试也会连接 devnet：

```bash
anchor test --skip-local-validator
```

---

## 部署到 Mainnet

> ⚠️ 主网部署消耗真实 SOL，请谨慎操作。

```bash
# 切换到主网
solana config set --url mainnet-beta

# 修改 Anchor.toml
# cluster = "mainnet"

anchor build
anchor deploy
```

---

## 升级已部署的合约

合约代码修改后，重新构建并部署即可升级（Program ID 不变）：

```bash
anchor build
anchor deploy
```

> 注意：升级需要使用与初次部署相同的 keypair（即 `target/deploy/<program>-keypair.json`），否则无权限升级。

---

## 关闭合约（回收 SOL）

不再使用的合约可以关闭，回收部署时消耗的 rent SOL：

```bash
solana program close <Program ID> --bypass-warning
```

---

## 常用网络 RPC 地址

| 网络 | 命令 |
|------|------|
| 本地 | `solana config set --url localhost` |
| Devnet | `solana config set --url devnet` |
| Mainnet | `solana config set --url mainnet-beta` |

---

## 费用参考

| 操作 | 消耗 SOL（估算） |
|------|----------------|
| 首次部署（小程序） | 2–4 SOL |
| 升级部署 | 0.x SOL（视代码大小变化） |
| 关闭合约 | 退回大部分 rent SOL |