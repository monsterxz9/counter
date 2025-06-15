// lib.rs
#![allow(deprecated)]
// 引入 Anchor 框架的核心库，prelude::* 包含了最常用的一系列工具
use anchor_lang::prelude::*;

// 这是程序的唯一 ID，当你编译时 Anchor 会自动生成一个新的并填在这里。
// 现在先用这个占位符，之后我们会替换它。
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// #[program] 宏是 Anchor 的魔法核心，它会将这个模块里的所有公共函数
// 转换为可以从客户端调用的 Solana 指令。
#[program]
pub mod my_solana_dapp {
    use super::*;

    // `initialize` 指令，用于创建我们的计数器账户
    // Context<Initialize> 包含了我们需要的所有账户信息
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // 从上下文中获取对 counter 账户的可变引用
        let counter_account = &mut ctx.accounts.counter;
        // 将这个账户里的 count 字段初始化为 0
        counter_account.count = 0;
        msg!("计数器已初始化，当前值为: 0"); // msg! 宏可以在链上日志中打印消息，便于调试
        Ok(())
    }

    // `increment` 指令，用于将计数值加 1
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter;
        counter_account.count += 1;
        msg!("计数器值增加, 当前值为: {}", counter_account.count);
        Ok(())
    }

    // `decrement` 指令，用于将计数值减 1
    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter;
        // 添加一个安全检查，确保计数值不会变成负数
        require!(counter_account.count > 0, ErrorCode::CannotDecrementToZero);
        counter_account.count -= 1;
        msg!("计数器值减少, 当前值为: {}", counter_account.count);
        Ok(())
    }
}

// `#[derive(Accounts)]` 宏用于定义指令需要传入的账户结构。
// Anchor 会自动处理账户的反序列化和安全校验。

// `initialize` 指令需要的账户
#[derive(Accounts)]
pub struct Initialize<'info> {
    // `#[account(init, payer = user, space = 8 + 8)]` 是账户约束的集合
    // `init`: 指示 Anchor 创建这个新账户。
    // `payer = user`: 指定 `user` 账户为创建新账户支付租金(rent)。
    // `space = 8 + 8`: 指定新账户需要分配的空间大小（字节）。
    //    - 8 字节: Anchor 为每个账户自动添加的鉴别器 (discriminator)，用于区分账户类型。
    //    - 8 字节: 我们自己定义的 `count` 字段 (u64 类型是 8 字节)。
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    // `#[account(mut)]` 表示这个账户在指令执行期间是可变的（因为要扣除租金）。
    #[account(mut)]
    pub user: Signer<'info>,
    // `SystemProgram` 是创建账户时必须传入的 Solana 系统程序。
    pub system_program: Program<'info, System>,
}

// `increment` 指令需要的账户
#[derive(Accounts)]
pub struct Increment<'info> {
    // 我们只需要传入要修改的 counter 账户。
    // `#[account(mut)]` 表示我们要修改这个账户里的数据。
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// `decrement` 指令需要的账户
#[derive(Accounts)]
pub struct Decrement<'info> {
    // 和 increment 一样，需要对 counter 账户进行可变引用。
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// `#[account]` 宏定义了我们链上数据的存储结构。
// 这就是我们 `counter` 账户实际存储的样子。
#[account]
pub struct Counter {
    pub count: u64, // 无符号 64 位整数，用来存储计数值
}

// 定义自定义错误码，让客户端能更好地理解链上发生的错误。
#[error_code]
pub enum ErrorCode {
    #[msg("Cannot decrement the counter because it is already at 0.")]
    CannotDecrementToZero,
}
