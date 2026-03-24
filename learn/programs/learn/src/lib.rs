use anchor_lang::prelude::*;

declare_id!("8sm8zjwUwhvBLxhv1RHd64fQvdAasxoE3j57NrqSxJdr");

#[program]
pub mod learn {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,key:u64) -> Result<()> {
        Ok(())
    }
    pub fn set(ctx: Context<Set>, key:u64) -> Result<()> {
        ctx.accounts.val.value = key;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(key:u64)]
pub struct Initialize<'info> {
    #[account(init,
            payer=signer,
            space=size_of::<Val>()+8,
            seeds=[&key.to_le_bytes().as_ref()],//seeds 参数 key 可以看作是一个类似于 Solidity 构造形式的“键”：
            bump)]
    val:Account<'info, Val>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}


#[account]
pub struct Val {
    value:u64,
}
#[derive(Accounts)]
#[instruction(key:u64)]
pub struct Set<'info> {
    #[account(mut)]
    val:Account<'info, Val>,
}