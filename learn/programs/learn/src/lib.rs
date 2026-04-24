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
        let mint = ctx.accounts.new_mint.clone();
        let destination_ata = ctx.accounts.new_ata.clone();
        let authority = ctx.accounts.signer.clone();
        let token_program = ctx.accounts.token_program.clone();

        // 组装指令
        let mint_to_instruction = MintTo {
            mint: mint.to_account_info(),
            to: destination_ata.to_account_info(),
            authority: authority.to_account_info(),
        };
        //构建一个指向 Token Program 的 CPI 上下文。
        let cpi_ctx = CpiContext::new(token_program.to_account_info(), mint_to_instruction);
        // 调用 Token Program 的 mint_to 指令，将 100 个 Token（精度为 9）铸造到 ATA。
        token::mint_to(cpi_ctx, mint_amount)?;
        Ok(())
    }


}


#[derive(Accounts)]
pub struct CreateMint<'info> {

    #[account(mut)]
    pub signer: Signer<'info>, // 签名

    #[account(
    init,
    payer = signer,
    mint::decimals = 9,
    mint::authority = signer,
    // Commenting out or removing this line permanently disables the freeze authority.注释掉或删除此行将永久禁用冻结权限
    mint::freeze_authority = signer,
    // 当创建一个没有冻结权限的 token 时，Solana 会阻止任何未来的更新。
    //  这使得 token 更加去中心化，因为没有任何权限可以冻结用户的 ATA。
    seeds=[b"my_mint",signer.key().as_ref()],
    bump)]
    pub new_mint: Account<'info, Mint>,

    #[account(init,
    payer = signer,
    associated_token::mint = new_mint,
    associated_token::authority = signer)]
    pub new_ata: Account<'info, TokenAccount>,

    // 这代表 SPL Token Program (TokenkegQfeZ…) 它拥有和管理所有 mint 账户和关联的 token 账户。
    pub token_program: Program<'info, Token>,
    //  这代表 ATA 程序 (ATokenGPvbdGV...),它只负责创建 ATA
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}


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