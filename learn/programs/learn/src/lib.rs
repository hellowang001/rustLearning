use anchor_lang::prelude::*;
use anchor_lang::solana_program::rent::Rent;
// use anchor_lang::solana_program::program as solana_program;

use anchor_lang::system_program;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use raydium_cpmm_cpi::{
    cpi,
    program::RaydiumCpmm,
    states::{AmmConfig, ObservationState, PoolState},
};
/// sha256("global:swap_base_input")[0..8]
declare_id!("8sm8zjwUwhvBLxhv1RHd64fQvdAasxoE3j57NrqSxJdr");

#[program]
pub mod keypair_vs_pda {

    use super::*;
    // 此函数部署一个新的SPL token, 精度 为9，并且铸造了100个单位的token

    /*
    pub fn create_and_mint_token(ctx:Context<CreateMint>)->Result<()>{
        let mint_amount = 100_000_000_000;// 100 颗 精度为9的数量
        //  .clone() 是因为 Account<'_, T> 不实现 Copy，直接赋值会发生 move，后续就无法再使用 ctx.accounts。
        let mint = ctx.accounts.new_mint.clone(); //
        let destination_ata = ctx.accounts.new_ata.clone();
        let authority = ctx.accounts.signer.clone();
        let token_program = ctx.accounts.token_program.clone();

        // 组装指令 -->CPI 要用
        let mint_to_instruction = MintTo {
            mint: mint.to_account_info(), // 铸造哪个代币
            to: destination_ata.to_account_info(), // 铸造到哪个 ATA
            authority: authority.to_account_info(), //  谁有权铸造（mint_authority）
        };
        //构建一个指向 Token Program 的 CPI 上下文。可以去看一下 mint_to这个方法所需要的入参
        let cpi_ctx = CpiContext::new(token_program.to_account_info(), mint_to_instruction);
        // 调用 Token Program 的 mint_to 指令，将 100 个 Token（精度为 9）铸造到 ATA。
        token::mint_to(cpi_ctx, mint_amount)?;
        // 看下mint_to 的定义，一个上下文cpi 一个u64数字
        //  pub fn mint_to<'info>(
        //     ctx: CpiContext<'_, '_, '_, 'info, MintTo<'info>>,
        //     amount: u64,
        // ) -> Result<()>
        Ok(())
    }
    */

    /*
    pub fn transfer_tokens(ctx: Context<TransferSpl>,amount:u64)->Result<()>{
        let source_ata = &ctx.accounts.from_ata;
        let destination_ata = &ctx.accounts.to_ata;
        let authority = &ctx.accounts.from;
        let token_program = &ctx.accounts.token_program;

        // 转移 spl 代币
        let cpi_accounts = Transfer{
            from:source_ata.to_account_info().clone(),
            to:destination_ata.to_account_info().clone(),
            authority:authority.to_account_info().clone(),
        };
        let cpi_ctx = CpiContext::new(token_program.to_account_info(),cpi_accounts);
        // 调用
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
    */

    /*
    pub fn get_balance(ctx:Context<GetBalance>)->Result<()>{
        // 获取 token 账户，所有者，及其余额
        let ata_pubkey = ctx.accounts.token_account.key();
        let owner = ctx.accounts.token_account.owner;
        let balances = ctx.accounts.token_account.amount; // 查看GetBalances 的TokenAccount 结构体就可以看到属性
        msg!("Token Account Address:{}",ata_pubkey);
        msg!("Token Account owner:{}",owner);
        msg!("Token Account balances:{}",balances);
        Ok(())
    }
    */

    /*

    // 初始化函数，将代币初始化
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // 设置管理员密钥
        ctx.accounts.admin_config.admin = ctx.accounts.admin.key();
        Ok(())
    }
    */

    /*

    // 铸造代币，出
    pub fn mint(ctx: Context<MintTokens>, lamports: u64) -> Result<()> {
        // 计算要mint 的 token 数量 (lamports * TOKEN_PER_SOL)
        let amount = lamports
            .checked_mul(TOKENS_PER_SOL)
            .ok_or(Errors::Overflow)?; // 如果溢出，返回溢出错误

        // 确保我们不超过最大供应量
        let current_supply = ctx.accounts.mint.supply;
        let new_supply = current_supply.checked_add(amount).ok_or(Errors::Overflow)?;
        require!(new_supply <= SUPPLY_CAP, Errors::SupplyLimit);

        // 将SOL 发送到 treasury 国库
        let transfer_instruction = Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
        };

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_instruction,
        );
        transfer(cpi_context, lamports)?;

        // 为mint PDA 创建签名者种子
        let bump = ctx.bumps.mint;
        let signer_seeds: &[&[&[u8]]] = &[&[b"token_mint".as_ref(), &[bump]]];

        // 使用Mint 作为其自身的授权机构来设置 Mint 指令
        let mint_to_instruction = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.buyer_ata.to_account_info(),
            authority: ctx.accounts.mint.to_account_info(),
        };

        // 使用 `new_with_signer` 创建 CPI 上下文 - 允许我们的 token sale 程序为 mint PDA 签名。这是因为 Solana 运行时验证了我们的程序使用这些种子和 bump 派生了 mint PDA
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            mint_to_instruction,
            signer_seeds,
        );
        mint_to(cpi_ctx, amount)?;

        Ok(())
    }

    */

    /*

    pub fn withdraw_funds(ctx: Context<WithdrawFunds>,amount:u64)->Result<()>{
        // 检查余额
        let treasury_balance = ctx.accounts.treasury.lamports();
        require!(treasury_balance>=amount,Errors::InsufficientFunds);

        // 为PDA 创建签名者种植
        let bump = ctx.bumps.treasury;
        let signer_seeds: &[&[&[u8]]] = &[&[b"treasury".as_ref(), &[bump]]];

        // 准备 CPI 上下文
        let transfer_instruction = Transfer{
            from:ctx.accounts.treasury.to_account_info(),
            to:ctx.accounts.admin.to_account_info(),
        };
        let cpi_ctx: CpiContext<'_, '_, '_, '_, Transfer<'_>> = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            transfer_instruction, signer_seeds,
        );
        transfer(cpi_ctx, amount)?;


        Ok(())
    }
    */

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // 初始化银行，就是存储所有用户的所有资产的地方

        let bank = &mut ctx.accounts.bank;

        bank.total_deposits = 0;
        msg!("银行初始化完成！");
        Ok(())
    }

    //创建用户账户--->给用户开户
    pub fn create_user_account(ctx: Context<CreateUserAccount>) -> Result<()> {
        // 初始化用户的账户，
        let user_accounr = &mut ctx.accounts.user_account;
        user_accounr.owner = ctx.accounts.user.key(); // 把 开户用户的拥有者标记为这个方法的调用者
        user_accounr.balance = 0;

        msg!("已为一下用户创建账户：{:?}", user_accounr.owner);

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        // 验证存款金额是否大于零
        require!(amount > 0, BankError::ZeroAmount);
        // 创建并执行系统 transfer 指令（通过 CPI）以将 SOL 从用户的钱包提取到银行账户
        let user = &ctx.accounts.user.key();
        // let bank = &ctx.accounts.bank.key();

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.bank.to_account_info(),
                },
            ),
            amount,
        )?;

        // 通过将存款金额添加到他们的 user_account PDA，安全地增加用户的账户余额
        // 更新账户余额
        let user_account = &mut ctx.accounts.user_account;
        user_account.balance = user_account
            .balance
            .checked_add(amount)
            .ok_or(BankError::Overflow)?;
        // 以相同金额更新银行的总存款
        let bank = &mut ctx.accounts.bank;
        bank.total_deposits = bank
            .total_deposits
            .checked_add(amount)
            .ok_or(BankError::Overflow)?;
        // 记录存款金额和用户地址
        msg!(" 为用户 {} 存了 {} lamports", user, amount);

        Ok(())
    }

    pub fn get_balance(ctx: Context<GetBalance>) -> Result<u64> {
        // 获取用户账户
        let user_account = &ctx.accounts.user_account;
        let balances = user_account.balance;
        msg!("用户{} 当前余额{}", user_account.owner, balances);
        Ok(balances)
    }
    pub fn calculate_interest(ctx: Context<GetBalance>) -> Result<u64> {
        let balance = ctx.accounts.user_account.balance;
        let interest = balance / 100; // 1% 利息
        Ok(interest)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        // 验证提款金额是否大于零
        require!(amount > 0, BankError::ZeroAmount);
        // 检查用户是否有足够的余额进行提款
        // 拿用户
        let user_account = &mut ctx.accounts.user_account;
        let bank = &mut ctx.accounts.bank;
        let user = ctx.accounts.user.key();

        require!(user_account.balance > 0, BankError::ZeroAmount);
        // 使用检查的算术更新用户账户余额和银行的总存款
        // 更新账户余额
        user_account.balance = user_account
            .balance
            .checked_sub(amount)
            .ok_or(BankError::Underflow)?;
        // 更新银行余额
        bank.total_deposits = bank
            .total_deposits
            .checked_sub(amount)
            .ok_or(BankError::Underflow)?;
        // 计算保持账户免租金所需的最低余额
        let rent = Rent::get()?;
        let user_account_info = user_account.to_account_info();
        let minimum_balance = rent.minimum_balance(user_account_info.data_len());
        // 确定保留免租金最低限额的安全转移金额
        // 计算安全转移金额(保留免租金最低限额)
        let available_lamports = user_account.get_lamports();
        let transfer_amount = amount.min(available_lamports.saturating_sub(minimum_balance));
        // 使用直接 lamport 操作转移 SOL（因为程序拥有这些账户）
        **user_account_info.try_borrow_mut_lamports()? -= transfer_amount;
        **ctx
            .accounts
            .user
            .to_account_info()
            .try_borrow_mut_lamports()? += transfer_amount;
        // 记录提款详细信息  immutable borrow occurs here cannot assign
        msg!("为 {} 提取了 {} lamports", user, amount);
        Ok(())
    }

    /// 通过原始 CPI 调用 Raydium CPMM 的 swap_base_input 指令
    /// amount_in:          投入的代币数量（最小单位）
    /// minimum_amount_out: 最少换出数量，低于则失败（滑点保护）
    pub fn proxy_swap_base_input(
        ctx: Context<ProxySwapBaseInput>,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<()> {
        // 第一步是构建cpi
        let cpi_accounts = cpi::accounts::Swap {
            payer: ctx.accounts.payer.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            amm_config: ctx.accounts.amm_config.to_account_info(),
            pool_state: ctx.accounts.pool_state.to_account_info(),
            input_token_account: ctx.accounts.input_token_account.to_account_info(),
            output_token_account: ctx.accounts.output_token_account.to_account_info(),
            input_vault: ctx.accounts.input_vault.to_account_info(),
            output_vault: ctx.accounts.output_vault.to_account_info(),
            input_token_program: ctx.accounts.input_token_program.to_account_info(),
            output_token_program: ctx.accounts.output_token_program.to_account_info(),
            input_token_mint: ctx.accounts.input_token_mint.to_account_info(),
            output_token_mint: ctx.accounts.output_token_mint.to_account_info(),
            observation_state: ctx.accounts.observation_state.to_account_info(),
        };
        // 第二步是组织上下文Account
        let cpi_context =
            CpiContext::new(ctx.accounts.cp_swap_program.to_account_info(), cpi_accounts);
        cpi::swap_base_input(cpi_context, amount_in, minimum_amount_out)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer=payer,
        space=8+Bank::INIT_SPACE,
    )]
    pub bank: Account<'info, Bank>, // 银行账户，初始化方法就是为了初始化银行，分配空间，存储数据

    #[account(mut)]
    pub payer: Signer<'info>, // 签名者，

    pub system_program: Program<'info, System>, // 有init 就必须要有系统程序，
}

#[derive(Accounts)]
pub struct CreateUserAccount<'info> {
    #[account(mut)]
    pub bank: Account<'info, Bank>, // 想要创建账户就必须把Bank 传进来，这个bank 就是一个随机的密钥对，而你下次要修改它数据的时候就传它的地址即可

    #[account(
        init,
        payer=user,
        space=8+UserAccount::INIT_SPACE,
        seeds=[b"bank_account",user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    // 当有一个初始化的account 的时候，就必须要有系统程序system_program
    pub system_program: Program<'info, System>,
}

// 用于跟踪所有用户的全部储蓄
#[account]
#[derive(InitSpace)]
pub struct Bank {
    pub total_deposits: u64, // 银行账户就是存储了一个金额
}

// 存储单个用户的 余额，
#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub owner: Pubkey, // 存储用户账户的所有者，一个地址
    pub balance: u64,  // 存储余额
}

// 充值用的，充值会用到银行，用户，签名，系统程序
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub bank: Account<'info, Bank>, // 因为已经被初始化了，所以直接引用即可

    #[account(
        mut,
        seeds=[b"bank_account",user.key().as_ref()],
        bump,
        constraint=user_account.owner==user.key()@BankError::UnauthorizedAccess // 加了一个地址判断
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// 提取用的，充值会用到银行，用户，签名，系统程序
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub bank: Account<'info, Bank>, // 因为已经被初始化了，所以直接引用即可

    #[account(
        mut,
        seeds=[b"bank_account",user.key().as_ref()],
        bump,
        constraint=user_account.owner==user.key()@BankError::UnauthorizedAccess // 加了一个地址判断
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// 查询余额用的,不需要和系统程序交互
#[derive(Accounts)]
pub struct GetBalance<'info> {
    pub bank: Account<'info, Bank>, // 因为已经被初始化了，所以直接引用即可

    #[account(
        seeds=[b"bank_account",user.key().as_ref()],
        bump,
        constraint=user_account.owner==user.key()@BankError::UnauthorizedAccess // 加了一个地址判断
    )]
    pub user_account: Account<'info, UserAccount>,
    pub user: Signer<'info>, // ✅ 改成 AccountInfo
}

#[error_code]
pub enum BankError {
    #[msg("金额必须大于零")]
    ZeroAmount,

    #[msg("提款余额不足")]
    InsufficientBalance,

    #[msg("算术溢出")]
    Overflow,

    #[msg("算术下溢")]
    Underflow,

    #[msg("银行帐户资金不足")]
    InsufficientFunds,

    #[msg("未经授权访问用户账户")]
    UnauthorizedAccess,
}

/// Raydium CPMM swap 所需的全部账户
/// 所有 vault/pool 账户地址都可以从 Raydium SDK 的 poolKeys 里读到
/// 使用 UncheckedAccount + /// CHECK: 注释，验证交由 Raydium 程序本身完成
#[derive(Accounts)]
pub struct ProxySwapBaseInput<'info> {
    pub cp_swap_program: Program<'info, RaydiumCpmm>,
    /// 发起 swap 的用户，支付手续费
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: 池子金库和 LP mint 的授权 PDA
    #[account(
        seeds = [
        raydium_cpmm_cpi::AUTH_SEED.as_bytes(),
        ],
        seeds::program = cp_swap_program.key(),
        bump,
    )]
    pub authority: UncheckedAccount<'info>,
    /// 读取协议手续费配置的工厂状态账户
    #[account(address = pool_state.load()?.amm_config)]
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// 执行 swap 的池子程序账户
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// 用户的 input token 账户（付款方）
    #[account(mut)]
    pub input_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// 用户的 output token 账户（收款方）
    #[account(mut)]
    pub output_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// 池子的 input token 金库账户
    #[account(
        mut,
        constraint = input_vault.key() == pool_state.load()?.token_0_vault || input_vault.key() == pool_state.load()?.token_1_vault
    )]
    pub input_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// 池子的 output token 金库账户
    #[account(
        mut,
        constraint = output_vault.key() == pool_state.load()?.token_0_vault || output_vault.key() == pool_state.load()?.token_1_vault
    )]
    pub output_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// input token 的 SPL Token 程序（支持 Token 或 Token-2022）
    pub input_token_program: Interface<'info, TokenInterface>,

    /// output token 的 SPL Token 程序（支持 Token 或 Token-2022）
    pub output_token_program: Interface<'info, TokenInterface>,

    /// input token 的 mint 账户
    #[account(
        address = input_vault.mint
    )]
    pub input_token_mint: Box<InterfaceAccount<'info, Mint>>,

    /// output token 的 mint 账户
    #[account(
        address = output_vault.mint
    )]
    pub output_token_mint: Box<InterfaceAccount<'info, Mint>>,
    /// 最新价格预言机观测数据账户（用于 TWAP）
    #[account(mut, address = pool_state.load()?.observation_key)]
    pub observation_state: AccountLoader<'info, ObservationState>,
}

/*
#[derive(Accounts)]
pub struct Initialize<'info> {
    // 管理员
    #[account(mut)]
    pub admin: Signer<'info>,

    // 管理员配置
    #[account(
        init,
        payer=admin,
        space = 8+AdminConfig::INIT_SPACE,
    )]
    pub admin_config: Account<'info, AdminConfig>,

    #[account(
        init,
        payer = admin,
        seeds = [b"token_mint"],
        bump,
        mint::decimals=9,
        mint::authority=mint.key(),
    )]
    pub mint: Account<'info, Mint>,

    /// CHECK:treasury 的PDA
    #[account(
        seeds=[b"treasury"],
        bump
    )]
    pub treasury: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
 */

/*

#[account]
#[derive(InitSpace)]
pub struct AdminConfig {
    pub admin: Pubkey,
}
 */

/*

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>, // 购买者是一个签名类型，带签名的

    // mint账户
    #[account(
        mut,
        seeds=[b"token_mint"],
        bump,
    )]
    pub mint: Account<'info, Mint>,

    // 购买者的ata账户
    #[account(
        mut,
        token::mint=mint,
        token::authority=buyer,
    )]
    pub buyer_ata: Account<'info, TokenAccount>,

    /// CHECK: treasury 的 PDA
    #[account(
        mut,
        seeds = [b"treasury"],
        bump,
    )]
    pub treasury: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

 */

/*

#[derive(Accounts)]
pub struct WithdrawFunds<'info>{
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        constraint=admin_config.admin==admin.key() @ Errors::UnauthorizedAccess // 确保签名者已获授权
    )]
    pub admin_config:Account<'info,AdminConfig>,

    /// CHECK: treasury 的 PDA
    #[account(
        mut,
        seeds =[b"treasury"],
        bump
    )]
    pub treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

}
 */

/*

#[error_code]
pub enum Errors {
    #[msg("达到最大的token 供应量限制")]
    SupplyLimit,
    #[msg("数学溢出")]
    Overflow,

    #[msg("只有管理员才能提款")]
    UnauthorizedAccess,

    #[msg("treasury 中没有足够的 SOL")]
    InsufficientFunds,
}
 */

/*
#[derive(Accounts)]
pub struct CreateMint<'info> {

    #[account(mut)]
    pub signer: Signer<'info>, // 签名

    #[account(
    init,//Anchor 自动创建 + 初始化，省去手动 create_account
    payer = signer, // rent 由 signer 支付
    mint::decimals = 9, // 精度9
    mint::authority = signer,//signer 有增发权
    // Commenting out or removing this line permanently disables the freeze authority.注释掉或删除此行将永久禁用冻结权限
    mint::freeze_authority = signer, // 冻结权限 signer 有冻结权
    // 当创建一个没有冻结权限的 token 时，Solana 会阻止任何未来的更新。
    //  这使得 token 更加去中心化，因为没有任何权限可以冻结用户的 ATA。
    seeds=[b"my_mint",signer.key().as_ref()], // PDA 种子 一种盐值 可以派生出来PDA
    bump)]
    pub new_mint: Account<'info, Mint>, //要创建的代币 Mint Account

    #[account(init,
    payer = signer,
    associated_token::mint = new_mint, //  属于哪个代币
    associated_token::authority = signer)] // signer 是 ATA 的 权限，只有signer 才能修改ATA，signer 持有该代币的 ATA（口袋）
    pub new_ata: Account<'info, TokenAccount>, //ATA 的类型为 TokenAccount，它表示 Solana 上的 ATA。就是谁的ata账户
    // 为什么ATA没有种子，因为是确定的user_wallet_address + token_mint_address => associated_token_account_address。因此我们不必传递种子和 bump
    // 没有有指定 mint 账户和 ATA 的 space，因为 Anchor 也会在后台为我们添加空间
    // 这代表 SPL Token Program (TokenkegQfeZ…) 它拥有和管理所有 mint 账户和关联的 token 账户。
    pub token_program: Program<'info, Token>, // 用于创建 mint 和铸造 Token， SPL Token 程序
    pub associated_token_program: Program<'info, AssociatedToken>, // ATA 程序，只用于创建用户的 ATA
    pub system_program: Program<'info, System>, // 为账户分配空间和管理租金
}
*/

/*

#[derive(Accounts)]
pub struct TransferSpl<'info>{
    pub from: Signer<'info>,

    #[account(mut)]
    pub from_ata: Account<'info,TokenAccount>,

    #[account(mut)]
    pub to_ata: Account<'info,TokenAccount>,

    pub token_program: Program<'info,Token>,// 正在与tokenProgram交互

}
*/

/*
#[derive(Accounts)]
pub struct GetBalance<'info>{
    #[account(mut)]
    pub token_account:Account<'info,TokenAccount>
}
*/

// pub struct Account {
//     /// The mint associated with this account
//     pub mint: Pubkey,
//     /// The owner of this account.
//     pub owner: Pubkey,
//     /// The amount of tokens this account holds.
//     pub amount: u64,
//     /// If `delegate` is `Some` then `delegated_amount` represents
//     /// the amount authorized by the delegate
//     pub delegate: COption<Pubkey>,
//     /// The account's state
//     pub state: AccountState,
//     /// If `is_native.is_some`, this is a native token, and the value logs the
//     /// rent-exempt reserve. An Account is required to be rent-exempt, so
//     /// the value is used by the Processor to ensure that wrapped SOL
//     /// accounts do not drop below this threshold.
//     pub is_native: COption<u64>,
//     /// The amount delegated
//     pub delegated_amount: u64,
//     /// Optional authority to close the account.
//     pub close_authority: COption<Pubkey>,
// }

/*
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space=size_of::<Pda>() + 8, seeds=[], bump)]
    pub pda: Account<'info, Pda>,
    #[account(mut)]
    pub signer: Signer<'info>,


    pub system_program: Program<'info, System>,

}

 */

/*

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub pda: Account<'info, Pda>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut,address=Pubkey::from_str("28QdAWmR5Tn5NsifTBvB8DxJDXN5eaQbFVwHactRxmff").unwrap())]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub pda: Account<'info, Pda>,
}

#[derive(Accounts)]
pub struct Delete<'info>{
    #[account(mut,close = signer,seeds=[],bump)]
    pub pda:Account<'info, Pda>,

    #[account(mut)]
    pub signer: Signer<'info>,
}


#[account]
pub struct Pda{
    pub x:u32,
}

 */
