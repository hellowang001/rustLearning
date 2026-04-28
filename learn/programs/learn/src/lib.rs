use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token::{self, mint_to, Mint, MintTo, Token, TokenAccount}; // 铸币帐户创建/处理所需
                                                                           // 导入 `calculate` 模块或库
                                                                           // pub mod calculate;
                                                                           // use std::mem::size_of;
                                                                           // use anchor_lang::system_program;
                                                                           // use std::str::FromStr;
declare_id!("8sm8zjwUwhvBLxhv1RHd64fQvdAasxoE3j57NrqSxJdr");
// 定义几个常量作为代币的属性
const TOKENS_PER_SOL: u64 = 100; // 单价，每个sol 换多少颗 代币

const SUPPLY_CAP: u64 = 1000e9 as u64; // 最大供应量 1000个 9精度

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

    // 初始化函数，将代币初始化
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // 设置管理员密钥
        ctx.accounts.admin_config.admin = ctx.accounts.admin.key();
        Ok(())
    }

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
}

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

#[account]
#[derive(InitSpace)]
pub struct AdminConfig {
    pub admin: Pubkey,
}

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
