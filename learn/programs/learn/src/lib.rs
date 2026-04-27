use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;// Needed for ATA creation 创建 ATA 所需
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer}; // 铸币帐户创建/处理所需
// 导入 `calculate` 模块或库
// pub mod calculate;
// use std::mem::size_of;
// use anchor_lang::system_program;
// use std::str::FromStr;
declare_id!("8sm8zjwUwhvBLxhv1RHd64fQvdAasxoE3j57NrqSxJdr");

#[program]
pub mod keypair_vs_pda{
    use super::*;
    // 此函数部署一个新的SPL token, 精度 为9，并且铸造了100个单位的token
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


}


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


#[derive(Accounts)]
pub struct TransferSpl<'info>{
    pub from: Signer<'info>,

    #[account(mut)]
    pub from_ata: Account<'info,TokenAccount>,

    #[account(mut)]
    pub to_ata: Account<'info,TokenAccount>,

    pub token_program: Program<'info,Token>,// 正在与tokenProgram交互

}

#[derive(Accounts)]
pub struct GetBalance<'info>{
    #[account(mut)]
    pub token_account:Account<'info,TokenAccount>
}
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