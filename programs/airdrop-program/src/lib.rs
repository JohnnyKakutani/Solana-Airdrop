use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use anchor_lang::system_program;
use solana_program::sysvar::clock::Clock;

declare_id!("HerreDvW9RAun1Vq18XCvfddSn9P2cseH9DbVRqdi9j1");

#[program]
pub mod airdrop_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let airdrop_info = &mut ctx.accounts.airdrop_info;
        if !airdrop_info.is_initialized {
            airdrop_info.is_initialized = true;
            airdrop_info.is_completed = false;
            airdrop_info.pre_sale_rate = 50;
            airdrop_info.public_sale_rate = 25;
            airdrop_info.discount_rate = 25;
            airdrop_info.round_status = AirDropRound::None;
            airdrop_info.sol_amount = 0;
            airdrop_info.token_amount = 0;
        }
        Ok(msg!("Initialize Success"))
    }

    pub fn create_user_info(ctx: Context<CreateUserInfos>, pubkey: Pubkey) -> Result<()> {
        let user_info = &mut ctx.accounts.user_info;
        if !user_info.is_initialized {
            user_info.is_initialized = true;
            user_info.is_whitelisted = true;
            user_info.sol_amount = 0;
            user_info.token_amount = 0;
        }
        Ok(msg!("Create Whitelist UserInfo {}", pubkey))
    }

    pub fn change_round_status(ctx: Context<ChangeRoundStatus>, status: u8, start_time: u64, end_time: u64) -> Result<()> {
        let airdrop_info = &mut ctx.accounts.airdrop_info;
        match status {
            0 => {
                airdrop_info.round_status = AirDropRound::None;
            },
            1 => {
                airdrop_info.round_status = AirDropRound::PreSale {
                    start_time: start_time,
                    end_time: end_time,
                }
            },
            2 => {
                airdrop_info.round_status = AirDropRound::PublicSale {
                    start_time: start_time,
                    end_time: end_time,
                }
            },
            _ => { return Ok(msg!("UnKnown Command")); }
        }
        Ok(msg!("Successfully Changed RoundStatus"))
    }

    pub fn buy_token(ctx: Context<BuyToken>, amount: u64) -> Result<()> {
        let airdrop_info = &mut ctx.accounts.airdrop_info;
        let user_info = &mut ctx.accounts.user_info;
        if !user_info.is_initialized {
            user_info.is_initialized = true;
            user_info.is_whitelisted = false;
            user_info.sol_amount = 0;
            user_info.token_amount = 0;
        }
        let currenttimestamp = Clock::get().unwrap().unix_timestamp as u64;
        if let AirDropRound::PreSale { start_time, end_time } = &mut airdrop_info.round_status {
            if currenttimestamp > *start_time && currenttimestamp < *end_time && user_info.is_whitelisted {
                let sol_amount = amount / airdrop_info.pre_sale_rate;
                // transfer sol from user to admin
                let cpi_context = CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.user.to_account_info(),
                        to: ctx.accounts.admin.to_account_info(),
                    },
                );
                system_program::transfer(cpi_context, sol_amount)?;
                // transfer custom token from admin to user
                let cpi_accounts = Transfer {
                    from: ctx.accounts.admin_token_address.to_account_info(),
                    to: ctx.accounts.user_token_address.to_account_info(),
                    authority: ctx.accounts.admin.to_account_info(),
                };
        
                let cpi_program = ctx.accounts.token_program.to_account_info();
                let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
                token::transfer(cpi_ctx, amount)?;
                user_info.sol_amount += sol_amount;
                user_info.token_amount += amount;
                airdrop_info.sol_amount += sol_amount;
                airdrop_info.token_amount += amount;
            }
            else {
                return err!(ErrorCode::NotActiveSale);
            }
        } else if let AirDropRound::PublicSale { start_time, end_time } = &mut airdrop_info.round_status {
            if currenttimestamp > *start_time && currenttimestamp < *end_time {
                let sol_amount = 
                    if user_info.is_whitelisted { 
                        amount / airdrop_info.public_sale_rate 
                    } else { 
                        amount / airdrop_info.public_sale_rate / (100 - airdrop_info.discount_rate) 
                    };
                // transfer sol from user to admin
                let cpi_context = CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.user.to_account_info(),
                        to: ctx.accounts.admin.to_account_info(),
                    },
                );
                system_program::transfer(cpi_context, sol_amount)?;
                // transfer custom token from admin to user
                let cpi_accounts = Transfer {
                    from: ctx.accounts.admin_token_address.to_account_info(),
                    to: ctx.accounts.user_token_address.to_account_info(),
                    authority: ctx.accounts.admin.to_account_info(),
                };
        
                let cpi_program = ctx.accounts.token_program.to_account_info();
                let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
                token::transfer(cpi_ctx, amount)?;
                user_info.sol_amount += sol_amount;
                user_info.token_amount += amount;
                airdrop_info.sol_amount += sol_amount;
                airdrop_info.token_amount += amount;
            }
            else {
                return err!(ErrorCode::NotActiveSale);
            }
        }  else {

        }
        Ok(msg!("Buy Token"))
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init_if_needed, seeds=[b"airdrop", admin.key().as_ref() ], bump, payer = admin, space = 67 )]
    pub airdrop_info: Account<'info, AirDropInfos>,
    
    #[account(mut)]
    pub admin: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateUserInfos<'info> {

    #[account(init_if_needed, seeds=[b"user", user.key().as_ref(), admin.key().as_ref() ], bump, payer = admin, space = 50 )]
    pub user_info: Account<'info, UserInfo>,

    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub user: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangeRoundStatus<'info> {
    #[account(mut, seeds=[b"airdrop", admin.key().as_ref() ], bump )]
    pub airdrop_info: Account<'info, AirDropInfos>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(mut, seeds=[b"airdrop", admin.key().as_ref() ], bump )]
    pub airdrop_info: Account<'info, AirDropInfos>,

    #[account(init_if_needed, seeds=[b"user", user.key().as_ref(), admin.key().as_ref() ], bump, payer = admin, space = 50 )]
    pub user_info: Account<'info, UserInfo>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token_address: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub admin_token_address: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AirDropInfos {
    is_initialized: bool,
    is_completed: bool,
    pre_sale_rate: u64,
    public_sale_rate: u64,
    discount_rate: u64,
    round_status: AirDropRound,
    sol_amount: u64,
    token_amount: u64
}

#[account]
pub struct UserInfo {
    is_whitelisted: bool,
    is_initialized: bool,
    token_amount: u64,
    sol_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AirDropRound {
    None,
    PreSale {
        start_time: u64,
        end_time: u64,
    },
    PublicSale {
        start_time: u64,
        end_time: u64,
    },
}

#[error_code]
pub enum ErrorCode {
    #[msg("Not Active Round")]
    NotActiveSale,
    #[msg("Not WhiteListed")]
    InvalidWhiteList,
    #[msg("Not Collection Completed yet")]
    InvalidClaim
}