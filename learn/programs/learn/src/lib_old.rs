use anchor_lang::prelude::*;
// 导入 `calculate` 模块或库
// pub mod calculate;
declare_id!("ApfcbYE4FcrTPwkYZe25jn89XyQEmi22XQB1of1bqLG5");
const OWNER: &str = "28QdAWmR5Tn5NsifTBvB8DxJDXN5eaQbFVwHactRxmff";
/*
#[program]
pub mod day_2 {
    use std::f32::consts::E;

    use super::*;
    // use chrono::*;
    /*
    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("你发送了{} 和 {} 还有消息 {}", a, b, message);
        Ok(())
    }
     */
    // use anchor_lang::prelude::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // let clock:Clock = Clock::get()?;
        // // ?运算符用于从Result<T, E>枚举中提取数据，如果函数执行成功，则返回OK(T)变体，或者如果发生错误则抛出Err(E)
        // msg!("区块时间戳: {}",clock.unix_timestamp);
        msg!("嗨，我是所有者。");

        Ok(())
    }

    // pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
    //     msg!("你的数组：{:?},", arr);
    //     Ok(())
    // }

    pub fn overflotest(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        msg!("你发送了 a={} 和 b={} a-b={}", a, b, a - b);
        Ok(())
        /*
        日志显示溢出了
        Logs:
            [
            "Program ApfcbYE4FcrTPwkYZe25jn89XyQEmi22XQB1of1bqLG5 invoke [1]",
            "Program log: Instruction: Overflotest",
            "Program log: panicked at programs/day_2/src/lib.rs:23:46:\nattempt to subtract with overflow",
            "Program ApfcbYE4FcrTPwkYZe25jn89XyQEmi22XQB1of1bqLG5 consumed 1564 of 200000 compute units",
            "Program ApfcbYE4FcrTPwkYZe25jn89XyQEmi22XQB1of1bqLG5 failed: SBF program panicked"
            ].
        如果把 检查关掉，就会在执行的时候溢出了
        Transaction executed in slot 4083:
        Signature: 4tDGSb5jfk5o9ZEHFDsdTYsn21Lt3FQTeH97MDAqKLW1EdGtfwJ3pNvkKgQ4CEL5SpLnYMT6sKfumXUuyhPxQAzJ
        Status: Ok
        Log Messages:
        Program ApfcbYE4FcrTPwkYZe25jn89XyQEmi22XQB1of1bqLG5 invoke [1]
        Program log: Instruction: Overflotest
        Program log: 你发送了 a=0 和 b=1 a-b=18446744073709551615
        Program ApfcbYE4FcrTPwkYZe25jn89XyQEmi22XQB1of1bqLG5 consumed 1169 of 200000 compute units
        Program ApfcbYE4FcrTPwkYZe25jn89XyQEmi22XQB1of1bqLG5 success
         */
    }
    /*
    pub fn add(ctx:Context<Initialize>,a:f64,b:f64,)->Result<()>{
        let result = a + b;
        msg!("加法运算: {} + {} = {}", a, b, result);
        Ok(())
    }
    pub fn subtract(ctx: Context<Initialize>, a: f64, b: f64) -> Result<()> {
        let result = a - b;
        msg!("减法运算: {} - {} = {}", a, b, result);
        Ok(())
    }

    pub fn multiply(ctx: Context<Initialize>, a: f64, b: f64) -> Result<()> {
        let result = a * b;
        msg!("乘法运算: {} × {} = {}", a, b, result);

        Ok(())
    }

    pub fn divide(ctx: Context<Initialize>, a: f64, b: f64) -> Result<()> {
        // 检查除数是否为零
        if b == 0.0 {
            return Err(ErrorCode::DivisionByZero.into());
        }
        Ok(())
    }
    pub fn sqrt(ctx: Context<Initialize>, a: f64) -> Result<()> {
        // 检查是否为负数
        if a < 0.0 {
            return Err(ErrorCode::NegativeNumber.into());
        }

        let result = a.sqrt();
        msg!("平方根运算: √{} = {}", a, result);

        Ok(())
    }
    pub fn log10(ctx: Context<Initialize>, a: f64) -> Result<()> {
    // 检查是否为非正数
    if a <= 0.0 {
        return Err(ErrorCode::InvalidLogarithm.into());
    }
        let result = a.log10();
        msg!("常用对数运算: log₁₀({}) = {}", a, result);

        Ok(())
    }
    
    pub fn empty(ctx: Context<NonEmptyAccountExample>) -> Result<()> {
        // 查看IDL Account
        msg!("打印NonEmptyAccountExample");

        Ok(())
    }
    pub fn age_checker(ctx: Context<Initialize>, age: u64) -> Result<()> {
        // if else
        // if age>=18{
        //     msg!(" 你已满18岁,实际为:{}",age);
        // } else {
        //     msg!("你未满18岁");
        // }
        // 三元运算
        // let result= if age>=18{"你已满18 岁以上"}else {"你未满18"};
        // msg!("{:?}",result);
        match age {
            1 => {
                // 如果年龄等于 1 则执行的代码块
                msg!("年龄是1");
            }
            2 | 3 => {
                msg!("年龄是1");
            }
            4..=6 => {
                // 如果年龄在 4 到 6（包括）之间则执行的代码块
                msg!("年龄在 4 到 6 之间");
            }
            _ => {
                // 任何其他年龄的代码块
                msg!("年龄是其他值");
            }
        }
        Ok(())
    }*/
    /*
    pub fn loop_over(ctx: Context<Initialize>)->Result<()>{
        // for i in 0..5{
        //     msg!("for 循环 第 {} 个",i);
        // }
        for i in (0..10).step_by(2){
            msg!("步进为2 的for 循环 第 {} 个",i);
        }
        Ok(())
    }
    */

    /*
    use std::collections::HashMap;
    pub fn hash_map_test(ctx: Context<Initialize>,key:String,value:String)->Result<()> {
        // 初始化映射
        let mut my_map = HashMap::new();
        // 传入键值
        my_map.insert(key.to_string(), value.to_string());
        // 看下是否生效
        msg!("我的名字是{}",my_map[&key]);
        Ok(())
    }

     */
    /*
    pub fn my_struct(_ctx:Context<Initialize>,name:String,age:u64)->Result<()> {
            // 定义结构体
        struct Person{
            my_name:String,
            my_age:u64,
        }
        // 使用结构体
        let mut personl:Person=Person{
            my_name:name,
            my_age:age,
        };
        msg!("{} 的年龄为 {} 岁",personl.my_name,personl.my_age);
        // 访问和修改结构体字段
        personl.my_name="Bob".to_string();
        personl.my_age=18;
        msg!("修改后 {} 的年龄为 {} 岁",personl.my_name,personl.my_age);

        Ok(())
    }

     */
    /* 
    pub fn print_struct(ctx: Context<Initialize>,a:String,b: u8)->Result<()> {
        msg!("name = {},age ={}",a,b);

    struct Person {
        name: String,
        age: u8,
    }

    impl Person {
        fn new(name:String,age:u8)->Self{
            Person { name, age }
        }
        fn can_drink(&self)->bool{
            if self.age>=21 as u8{
                return true;
            }
            return false;
        }
        fn age_in_one_year(&self)->u8{
            return &self.age+1;
        }
        // fn print_name(&self) {
        //     msg!("self name ={}",self.name)
        // }
    }
    let person = Person::new(a, b);
    msg!("{:?}",person.can_drink());
    msg!("{:?}",person.age_in_one_year());
    msg!("{:?}",person.name);
    Ok(())
        
    }
    */

    /*
    pub fn  add_two(_ctx:Context<Initialize>,x:u64,y:u64)->Result<()> {
        // 调用 calculate.rs 中的 `add` 函数
        let result = calculate::add(x, y);
        msg!(" {} + {} = {} ",x,y,result);
        
        Ok(())
    } 
    */
    /*
    pub fn get_day_week(_ctx:Context<Initialize>)->Result<()> {
        let clock = Clock::get()?;
        let time_stamp =clock.unix_timestamp; //拿当前时间戳
        let data_time = chrono::NaiveDateTime::from_timestamp_opt(time_stamp,0).unwrap();
        let day_of_the_week=data_time.weekday();
        let day=data_time.day();
        let year=data_time.year();
        let mon=data_time.month();
        msg!(" 时间戳 = {},年 {} 月 {} 日 {} 星期 {} ",time_stamp,year,mon,day,day_of_the_week);

        Ok(())
    }
     */
    /*
    pub fn get_clock(_ctx: Context<Initialize>) ->Result<()>{
        // 获取 Clock sysvar
        let clock = Clock::get()?;
        msg!( "clock = {:?} ",clock);
        Ok(())
    }
    pub fn get_epoch(_ctx:Context<Initialize>)->Result<()> {
        let epoch_schedule =EpochSchedule::get()?;
        msg!("epoch schedule : {:?} ",epoch_schedule);

        Ok(())
    }
     */
    /*
    pub fn print_emit(_ctx:Context<Initialize>)->Result<()> {
        emit!(MyEvent{value:11});
        emit!(MySecondEvent{value:15,message:"hello world".to_string()});
        Ok(())
    }
    */

    /*
    pub fn get_signer(ctx: Context<Initialize>) -> Result<()> {
        let the_signer1 : &mut Signer = &mut ctx.accounts.signer1;
        msg!("Signer 1 = {:?}", *the_signer1.key);
        Ok(())
    }
    pub fn get_more_signer(ctx: Context<Initialize>) -> Result<()> {
        let the_signer1 : &mut Signer = &mut ctx.accounts.signer1;
        let the_signer2 : &mut Signer = &mut ctx.accounts.signer2;
        msg!("Signer 1 = {:?}", *the_signer1.key);
        msg!("Signer 2 = {:?}", *the_signer2.key);
        Ok(())
    }
     */

    /*
    #[access_control(check(&ctx))]
    pub fn only_owner(ctx: Context<OnlyOwner>) -> Result<()> {
        msg!("嗨，我是所有者,只有所有者才能访问这个函数");
        Ok(())
    }

     */
    pub fn spend(_ctx: Context<Initialize>) -> Result<()> {
        // 这消耗 600 CU（类型默认为 Vec<i32>）
        let mut a = Vec::new();
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);

        // 这消耗 618 CU
        let mut a: Vec<u64> = Vec::new();
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);

        // 这消耗 600 CU（与第一个相同，但类型已明确表示）
        let mut a: Vec<i32> = Vec::new();
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);

        // 这消耗 618 CU（与 u64 占用相同的空间）
        let mut a: Vec<i64> = Vec::new();
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);

        // 这消耗 459 CU
        let mut a: Vec<u8> = Vec::new();
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);

        Ok(())
    }
}
/*
fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    require_keys_eq!(
            ctx.accounts.signer_account.key(),
            OWNER.parse::<Pubkey>().unwrap(),
            ErrorCode::NotOwner
        );
    Ok(())
}

 */
// #[derive(Accounts)]
// pub struct NonEmptyAccountExample<'info> {
//     signer: Signer<'info>,
//     another_signer: Signer<'info>,
// }

 */
/*
#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,key:u64) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, new_x: u64) -> Result<()> {
        ctx.accounts.my_storage.x=new_x;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct Initialize<'info> {
    // #[]是一个属性宏，用于增强 my_storage 用的，account()带参数的属性增强宏
    // payer = signer, // 谁在为分配存储支付 SOL,签名者被指定为mut,表示可变，因为他们的账户余额将会变化
    // 即会从他们账户扣除一些SOL，因此，我们将他们的账户注释为 “可变”，这个signer为my_storage 支付
    // space 空间，表示将会占用多少空间，+8表示除了 结构所需直接，额外加8个字节
    // seeds = [] 和bump ,种子一个程序可以拥有多个账户，它通过“种子”在账户自己按进行区分，该宗旨用于计算“鉴别符”，
    // “鉴别符”占8个自己额，这就是为什么除了我们结构所需要的空间外还要额外分配8个直接，bump目前可以看作是样板代码
    // 这些基本都是样板代码，不必过于纠结
    #[account(init,// 初始化
              payer = signer, // 谁在为分配存储支付 SOL,签名者被指定为mut,表示可变，因为他们的账户余额将会变化
              space=size_of::<MyStorage>() + 8,// 空间，表示将会占用多少空间，+8后面说
              seeds = [&key.to_le_bytes().as_ref()],
              bump)]
    pub my_storage: Account<'info, MyStorage>,// my_storage 在初始化MyStorage这个结构体

    #[account(mut)]
    pub signer: Signer<'info>, // 负责为结构的存储支付 “gas 费”的钱包，就是签名者嘛

    pub system_program: Program<'info, System>, // 系统程序
    // 内置于Solana运行时的程序（有点像以太坊的预编译），它从一个账户向另一个账户转移SOL,
    // 现在我们需要将SOL从支付MyStruct 存储的签名者那里转移，因此 system program 总是初始化事物的一部分。
}

#[account]
pub struct MyStorage {
    x: u64,
}
#[account]
pub struct Val{
    value:u64,
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut,seeds = [],bump)]
    pub my_storage: Account<'info, MyStorage>,
}


 */


/*
#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account:Signer<'info>,
}
#[derive(Accounts)]
pub struct Signer<'info> {
    #[account(mut)]
    pub signer1: Signer<'info>,
}

 */



/*
// 事件打印
#[event]
pub struct MyEvent {
pub value: u64,
}

#[event]
pub struct MySecondEvent {
pub value: u64,
pub message: String,
}

 */

/*

/// 自定义错误类型
#[error_code]
pub enum ErrorCode {
#[msg("除数不能为零")]
DivisionByZero,

#[msg("不能计算负数的平方根")]
NegativeNumber,

#[msg("对数运算的参数必须大于零")]
InvalidLogarithm,

#[msg("只有所有者才可以调用此函数")]
NotOwner,
}


 */