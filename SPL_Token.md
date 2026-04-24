# SPL Token 完整指南

## 目录

1. [SPL Token vs ERC20](#spl-token-vs-erc20)
2. [账户体系](#账户体系)
3. [核心指令详解](#核心指令详解)
4. [ATA 机制](#ata-机制)
5. [代码示例](#代码示例)

---

## SPL Token vs ERC20

### 相同点

| 方面 | SPL Token | ERC20 |
|------|-----------|-------|
| 目标 | 同质化代币标准 | 同质化代币标准 |
| 核心功能 | transfer / mint / burn | transfer / mint / burn |
| 元数据 | name, symbol, decimals | name, symbol, decimals |
| 授权机制 | delegate (approve) | approve + allowance |

### 核心差异

#### 1. 账户模型 vs 合约模型

**ERC20** — 合约内维护一张 mapping 表：

```solidity
// 所有余额都在同一个合约的 storage 里
mapping(address => uint256) public balances;
```

**SPL Token** — 每个持币地址有独立的 Token Account（链上账户）：

```
Mint Account         ← 代币定义（supply, decimals, mint_authority）
    ↓
Token Account (用户A) ← 存放用户 A 持有的该代币余额
Token Account (用户B) ← 存放用户 B 持有的该代币余额
```

#### 2. 程序是共享的

| | ERC20 | SPL Token |
|---|---|---|
| 逻辑在哪 | 每个代币自己部署一个合约 | 所有代币共用同一个 `spl-token` 程序 |
| 地址含义 | 合约地址 = 代币标识 | **Mint 账户地址** = 代币标识 |

ERC20 每发行一个新代币 → 部署一个新合约。
SPL Token 每发行一个新代币 → 只需创建一个新的 **Mint Account**，程序逻辑复用。

#### 3. Rent（租金）

SPL Token 引入了链上存储的 rent 机制：

- 创建 Mint Account 或 Token Account 都需要支付 SOL 作为 rent（约 0.002 SOL）
- ERC20 没有这个概念，存储成本体现在 gas 里

#### 4. 权限体系

| 权限 | SPL Token | ERC20 |
|------|-----------|-------|
| 铸币权 | `mint_authority`（可撤销/转移） | 合约 owner 自定义 |
| 冻结权 | `freeze_authority`（可冻结账户） | 无标准，需自己实现 |
| 关闭账户 | `close_account` 可回收 rent | 无此概念 |

---

## 账户体系

SPL Token 涉及三种账户：

```
┌─────────────────────────────────────────────────────┐
│                    spl-token 程序                    │
│         TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss4        │
└──────────────┬──────────────────────────────────────┘
               │ 管理
               ▼
┌─────────────────────────┐
│      Mint Account        │  ← 代币本身的定义
│  - supply (总供应量)      │
│  - decimals              │
│  - mint_authority        │  ← 谁能增发
│  - freeze_authority      │  ← 谁能冻结
└──────────┬──────────────┘
           │ 1:N
           ▼
┌─────────────────────────┐
│     Token Account        │  ← 某个用户持有该代币的"口袋"
│  - mint (属于哪个代币)    │
│  - owner (谁控制这个账户) │
│  - amount (余额)         │
│  - delegate              │
│  - state (frozen?)       │
└─────────────────────────┘
```

### Mint Account

```
地址: 任意公钥（创建时生成）
大小: 82 bytes（固定）
数据:
  mint_authority:   Option<Pubkey>   // 增发权限，None = 永久锁死供应量
  supply:           u64              // 当前流通总量
  decimals:         u8               // 精度，如 USDC = 6
  is_initialized:   bool
  freeze_authority: Option<Pubkey>   // 冻结权限
```

### Token Account

```
地址: 任意公钥（通常用 ATA 派生）
大小: 165 bytes（固定）
数据:
  mint:              Pubkey        // 对应哪个 Mint
  owner:             Pubkey        // 谁有控制权
  amount:            u64           // 余额（整数，需除以 10^decimals）
  delegate:          Option<Pubkey>
  state:             AccountState  // Initialized / Frozen / Uninitialized
  is_native:         Option<u64>   // 是否是 Wrapped SOL
  delegated_amount:  u64
  close_authority:   Option<Pubkey>
```

---

## ATA 机制

### 什么是 ATA

ATA（Associated Token Account）是每个用户对每种代币的标准"口袋"，地址由以下方式确定性派生：

```
ATA 地址 = PDA(
    seeds = [owner_pubkey, token_program_id, mint_pubkey],
    program = associated_token_program
)
```

```
用户 Alice (wallet: AAAx...)
  ├── USDC ATA:  PDA(AAAx... + USDC mint)  → amount: 100_000000
  └── JUP ATA:   PDA(AAAx... + JUP mint)   → amount: 50_000000000
```

### 安全保证

任何人都能为别人创建 ATA，但 ATA 程序在代码层面强制规定：

```
ATA.owner           = 被创建的那个钱包地址（Bob）
ATA.close_authority = Bob
```

不管是谁付钱创建的，owner 永远是 Bob，付款人无法劫持。

### 转账完整流程

```
Alice → Bob 转代币：

1. 用 Bob 钱包地址 + Mint 地址  离线推导出 Bob 的 ATA 地址
2. 检查该 ATA 是否存在
   - 不存在 → Alice 出钱帮 Bob 创建（付 ~0.002 SOL rent）
3. 调用 Transfer 指令，目标 = Bob 的 ATA
```

### Wrapped SOL（wSOL）

SOL 不是 SPL Token，但可以包装成 wSOL 参与 DeFi：

```
wSOL Mint = So11111111111111111111111111111111111111112（固定地址）

创建 wSOL Token Account → 往里转 SOL → amount 就等于 lamports
关闭 wSOL Token Account → SOL 返回 owner
```

---

## 核心指令详解

> 每个指令分三层展示：
> 1. **spl-token Rust 函数签名**（构造 Instruction 的 helper）
> 2. **Anchor CPI 写法**（在 Anchor 合约里调用）
> 3. **说明**

### 指令分类

| 类别 | 指令 |
|------|------|
| 初始化 | `initialize_mint` / `initialize_account` |
| 转账 | `transfer` / `transfer_checked` |
| 增发/销毁 | `mint_to` / `mint_to_checked` / `burn` / `burn_checked` |
| 授权 | `approve` / `approve_checked` / `revoke` |
| 权限管理 | `set_authority` |
| 账户管理 | `close_account` / `freeze_account` / `thaw_account` |

---

### 一、初始化指令

#### `initialize_mint`

创建新代币（初始化 Mint Account）。

```rust
// spl-token crate
pub fn initialize_mint(
    token_program_id: &Pubkey,
    mint_pubkey:      &Pubkey,           // 已分配 82 bytes 空间的账户
    mint_authority:   &Pubkey,           // 增发权限
    freeze_authority: Option<&Pubkey>,   // 冻结权限，None = 不支持冻结
    decimals:         u8,
) -> Result<Instruction, ProgramError>
```

```rust
// Anchor 合约里通常不需要手动调用，
// 直接在 #[derive(Accounts)] 中声明即可自动初始化：
#[account(
    init,
    payer = payer,
    mint::decimals = 6,
    mint::authority = payer,
)]
pub mint: Account<'info, Mint>,
```

> 注意：调用前必须先用 `system_program::create_account` 分配 82 bytes 空间并转入 rent。

---

#### `initialize_account`

创建 Token Account（用户的代币"口袋"）。

```rust
// spl-token crate
pub fn initialize_account(
    token_program_id: &Pubkey,
    account_pubkey:   &Pubkey,  // 已分配 165 bytes 空间的账户
    mint_pubkey:      &Pubkey,
    owner_pubkey:     &Pubkey,  // 谁拥有这个 Token Account
) -> Result<Instruction, ProgramError>
```

```rust
// Anchor 里同样用 init 自动处理：
#[account(
    init,
    payer = payer,
    token::mint = mint,
    token::authority = payer,
)]
pub token_account: Account<'info, TokenAccount>,

// 或者使用 ATA（推荐）：
#[account(
    init_if_needed,
    payer = payer,
    associated_token::mint = mint,
    associated_token::authority = user,
)]
pub ata: Account<'info, TokenAccount>,
```

---

### 二、转账指令

#### `transfer`

```rust
// spl-token crate
pub fn transfer(
    token_program_id: &Pubkey,
    source_pubkey:      &Pubkey,
    destination_pubkey: &Pubkey,
    authority_pubkey:   &Pubkey,  // source 的 owner 或 delegate
    signer_pubkeys:     &[&Pubkey],  // 多签时填入，单签传 &[]
    amount:             u64,         // 原始整数，未考虑 decimals
) -> Result<Instruction, ProgramError>
```

```rust
// Anchor CPI
token::transfer(
    CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from:      ctx.accounts.source.to_account_info(),
            to:        ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
    ),
    amount,
)?;
```

---

#### `transfer_checked` ← 推荐使用

比 `transfer` 多校验 mint 和 decimals，防止精度错误。

```rust
// spl-token crate
pub fn transfer_checked(
    token_program_id:   &Pubkey,
    source_pubkey:      &Pubkey,
    mint_pubkey:        &Pubkey,  // ← 额外传入 mint 做校验
    destination_pubkey: &Pubkey,
    authority_pubkey:   &Pubkey,
    signer_pubkeys:     &[&Pubkey],
    amount:             u64,
    decimals:           u8,       // ← 必须与 mint.decimals 一致，否则报错
) -> Result<Instruction, ProgramError>
```

```rust
// Anchor CPI
token::transfer_checked(
    CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::TransferChecked {
            from:      ctx.accounts.source.to_account_info(),
            mint:      ctx.accounts.mint.to_account_info(),
            to:        ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
    ),
    amount,
    decimals,
)?;
```

> **为什么需要 Checked 版本？**
> USDC decimals=6，转 1 USDC 应传 `1_000_000`，若传 `1` 则只转了 0.000001 USDC。
> Checked 版本强制声明 decimals，程序会验证，避免金额单位错误。

---

### 三、增发 / 销毁指令

#### `mint_to` / `mint_to_checked`

```rust
// spl-token crate
pub fn mint_to(
    token_program_id: &Pubkey,
    mint_pubkey:      &Pubkey,
    account_pubkey:   &Pubkey,   // 目标 Token Account
    owner_pubkey:     &Pubkey,   // mint_authority
    signer_pubkeys:   &[&Pubkey],
    amount:           u64,
) -> Result<Instruction, ProgramError>

pub fn mint_to_checked(
    token_program_id: &Pubkey,
    mint_pubkey:      &Pubkey,
    account_pubkey:   &Pubkey,
    owner_pubkey:     &Pubkey,
    signer_pubkeys:   &[&Pubkey],
    amount:           u64,
    decimals:         u8,        // ← Checked 版本多此参数
) -> Result<Instruction, ProgramError>
```

```rust
// Anchor CPI
token::mint_to(
    CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::MintTo {
            mint:      ctx.accounts.mint.to_account_info(),
            to:        ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        },
    ),
    amount,
)?;
```

执行后：`mint.supply += amount`，`destination.amount += amount`

---

#### `burn` / `burn_checked`

```rust
// spl-token crate
pub fn burn(
    token_program_id: &Pubkey,
    account_pubkey:   &Pubkey,   // 要销毁的 Token Account
    mint_pubkey:      &Pubkey,
    authority_pubkey: &Pubkey,   // account 的 owner 或 delegate
    signer_pubkeys:   &[&Pubkey],
    amount:           u64,
) -> Result<Instruction, ProgramError>

pub fn burn_checked(/* 同上，多一个 decimals: u8 */)
```

```rust
// Anchor CPI
token::burn(
    CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Burn {
            mint:      ctx.accounts.mint.to_account_info(),
            from:      ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
    ),
    amount,
)?;
```

执行后：`mint.supply -= amount`，`account.amount -= amount`

---

### 四、授权指令

#### `approve` / `approve_checked`

授权 delegate 可花费最多 N 个代币。

```rust
// spl-token crate
pub fn approve(
    token_program_id:  &Pubkey,
    source_pubkey:     &Pubkey,   // 被授权的 Token Account
    delegate_pubkey:   &Pubkey,   // 被授权方
    owner_pubkey:      &Pubkey,   // source 的 owner
    signer_pubkeys:    &[&Pubkey],
    amount:            u64,
) -> Result<Instruction, ProgramError>

pub fn approve_checked(/* 同上，多 mint_pubkey: &Pubkey, decimals: u8 */)
```

```rust
// Anchor CPI
token::approve(
    CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Approve {
            to:       ctx.accounts.token_account.to_account_info(),
            delegate: ctx.accounts.delegate.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        },
    ),
    amount,
)?;
```

执行后：`token_account.delegate = delegate`，`delegated_amount = amount`

> 每次 approve 会**覆盖**之前的授权，不叠加。

---

#### `revoke`

撤销当前 delegate 授权。

```rust
// spl-token crate
pub fn revoke(
    token_program_id: &Pubkey,
    source_pubkey:    &Pubkey,   // Token Account
    owner_pubkey:     &Pubkey,
    signer_pubkeys:   &[&Pubkey],
) -> Result<Instruction, ProgramError>
```

```rust
// Anchor CPI
token::revoke(
    CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Revoke {
            source:    ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        },
    ),
)?;
```

执行后：`delegate = None`，`delegated_amount = 0`

---

### 五、权限管理

#### `set_authority`

转移或撤销某种权限。

```rust
// spl-token crate
pub fn set_authority(
    token_program_id:    &Pubkey,
    owned_pubkey:        &Pubkey,          // Mint 或 Token Account
    new_authority:       Option<&Pubkey>,  // None = 永久撤销（不可逆！）
    authority_type:      AuthorityType,
    owner_pubkey:        &Pubkey,          // 当前权限持有人
    signer_pubkeys:      &[&Pubkey],
) -> Result<Instruction, ProgramError>
```

`AuthorityType` 枚举：

| 值 | 作用对象 | 含义 |
|---|---|---|
| `MintTokens` | Mint | 增发权限 |
| `FreezeAccount` | Mint | 冻结权限 |
| `AccountOwner` | Token Account | owner |
| `CloseAccount` | Token Account | close_authority |

```rust
// Anchor CPI —— 以放弃增发权限为例（锁死供应量）
token::set_authority(
    CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::SetAuthority {
            account_or_mint: ctx.accounts.mint.to_account_info(),
            current_authority: ctx.accounts.mint_authority.to_account_info(),
        },
    ),
    AuthorityType::MintTokens,
    None,  // 传 None = 永久撤销，不可恢复
)?;
```

---

### 六、账户管理指令

#### `close_account`

关闭 Token Account，回收 rent SOL（余额必须为 0）。

```rust
// spl-token crate
pub fn close_account(
    token_program_id: &Pubkey,
    account_pubkey:   &Pubkey,      // 要关闭的 Token Account
    destination_pubkey: &Pubkey,    // rent SOL 返还到这里
    owner_pubkey:     &Pubkey,      // close_authority 或 owner
    signer_pubkeys:   &[&Pubkey],
) -> Result<Instruction, ProgramError>
```

```rust
// Anchor CPI
token::close_account(
    CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::CloseAccount {
            account:     ctx.accounts.token_account.to_account_info(),
            destination: ctx.accounts.destination.to_account_info(),
            authority:   ctx.accounts.owner.to_account_info(),
        },
    ),
)?;
```

---

#### `freeze_account` / `thaw_account`

冻结/解冻 Token Account（需要 freeze_authority）。

```rust
// spl-token crate
pub fn freeze_account(
    token_program_id: &Pubkey,
    account_pubkey:   &Pubkey,
    mint_pubkey:      &Pubkey,
    owner_pubkey:     &Pubkey,   // freeze_authority
    signer_pubkeys:   &[&Pubkey],
) -> Result<Instruction, ProgramError>

pub fn thaw_account(/* 参数相同 */) -> Result<Instruction, ProgramError>
```

```rust
// Anchor CPI —— 冻结
token::freeze_account(
    CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::FreezeAccount {
            account:   ctx.accounts.token_account.to_account_info(),
            mint:      ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.freeze_authority.to_account_info(),
        },
    ),
)?;
```

冻结后 `account.state = Frozen`，无法转账/增发/销毁，直到调用 `thaw_account`。

---

### Checked vs 非 Checked 指令

| 场景 | 建议 |
|------|------|
| 生产环境 / DeFi 协议 | 始终用 Checked 版本 |
| Anchor 合约内部调用 | 用 Checked，明确声明 decimals |
| 个人脚本 / 快速测试 | 普通版本即可 |

---

### 完整创建代币流程

```rust
// 1. 创建并初始化 Mint（Anchor 用 init 约束自动完成）
system_program::create_account(...)   // 分配 82 bytes
initialize_mint(...)

// 2. 创建 Token Account / ATA（Anchor 用 init / init_if_needed 自动完成）
system_program::create_account(...)   // 分配 165 bytes
initialize_account(...)

// 3. 增发
mint_to_checked(...)    // mint.supply += amount

// 4. 转账
transfer_checked(...)   // source.amount -= amount, destination.amount += amount

// 5. 用完关闭 Token Account，回收 rent
close_account(...)
```

---

## 代码示例

### TypeScript（@solana/spl-token）

```typescript
import {
    createMint,
    getOrCreateAssociatedTokenAccount,
    mintTo,
    transfer,
} from "@solana/spl-token";

// 1. 创建 Mint Account
const mint = await createMint(
    connection,
    payer,            // 付 rent 的人
    mintAuthority,    // 增发权限
    freezeAuthority,  // 冻结权限（null = 无冻结功能）
    decimals          // 精度
);

// 2. 为用户创建 ATA（已存在则直接返回）
const ata = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    mint,
    userWallet.publicKey
);

// 3. 增发
await mintTo(
    connection,
    payer,
    mint,
    ata.address,              // 目标 Token Account
    mintAuthority,
    1000 * 10 ** decimals
);

// 4. 转账
await transfer(
    connection,
    payer,
    sourceATA,
    destinationATA,
    owner,
    100 * 10 ** decimals
);
```

---

## Token-2022 扩展（新标准）

程序地址：`TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`

| 扩展 | 功能 |
|------|------|
| `TransferFee` | 每次转账自动扣手续费给协议 |
| `TransferHook` | 转账时触发自定义程序逻辑 |
| `ConfidentialTransfer` | 隐私转账（金额加密） |
| `PermanentDelegate` | 永久委托，可无限花费任何账户 |
| `MintCloseAuthority` | 允许关闭 Mint Account |
| `NonTransferable` | 灵魂绑定代币（SBT） |
| `MetadataPointer` + `TokenMetadata` | 链上元数据，无需 Metaplex |
