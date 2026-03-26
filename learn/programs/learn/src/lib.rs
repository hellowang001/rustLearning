use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("8sm8zjwUwhvBLxhv1RHd64fQvdAasxoE3j57NrqSxJdr");

#[program]
pub mod learn {
    use super::*;

    /*
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        let rent =Rent::get()?;
        let empty_account_size = 0;
        let lamports = rent.minimum_balance(empty_account_size);

        msg!("创建空账户的最小租金 : {} lamports", lamports);
        msg!("约等于: {} SOL ", lamports as f64 / 1_000_000_000.0);
        let account_size = 3;
        let lamports_byte = rent.minimum_balance(account_size);
        msg!("创建 {} 个字节的账户的租金 : {} lamports", account_size,lamports_byte);
        msg!("约等于: {} SOL ", lamports_byte as f64 / 1_000_000_000.0);
        Ok(())
    }
    pub fn read_balance(ctx: Context<ReadBalance>) -> Result<()> {
        let balance = ctx.accounts.acct.to_account_info().lamports();

        msg!("余额以 Lamports 表示为{}",balance);
        msg!("约等于: {} SOL ", balance as f64 / 1_000_000_000.0);

        Ok(())

    }

     */
    pub fn send_sol(ctx: Context<SendSol>,amount:u64)->Result<()>{
        //   Solana 上每个程序只能执行自己的逻辑。你的 learn 程序想转 SOL，
        //  但转账是 System Program 的功能，所以需要"跨程序调用"去调用它。CpiContext
        //   就是描述这次调用所需的信息。
        //  就像你（learn 程序）委托银行（System Program）帮你转账，
        // CpiContext 就是那张转账委托单，上面写明了：找哪家银行、从谁的账户转、转到谁的账户。
        let cpi_context=CpiContext::new(
            //
            ctx.accounts.system_program.to_account_info(),

            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
            }
        );
        //  system_program::transfer(cpi_context, amount)? 才是真正"提交委托单"执行转账
        system_program::transfer(cpi_context, amount)?;
        Ok(())

    }
}

#[derive(Accounts)]
pub struct Initialize  {
}


#[derive(Accounts)]
pub struct SendSol <'info> {
    /// CHECK: 不读取或者写入这个账户
    #[account(mut)]
    recipient:UncheckedAccount<'info>,
    system_program: Program<'info, System>,

    #[account(mut)]
    signer:Signer<'info>,
}

#[error_code]
pub enum Errors {
    #[msg("transfer failed")]
    TransferFailed,
}


/*
#[derive(Accounts)]
pub struct ReadBalance <'info> {
    /// CHECK:` 尽管我们读取了这个账户的余额，但是并没有用到这个信息
    pub acct:UncheckedAccount<'info>,
}


 */



/*

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

 */