use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
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
            airdrop_info.preSaleRate = 50;
            airdrop_info.publicSaleRate = 25;
            airdrop_info.discountRate = 25;
            airdrop_info.roundStatus = AirDropRound::None;
            airdrop_info.solAmount = 0;
            airdrop_info.tokenAmount = 0;
        }
        Ok(msg!("Initialize Success"))
    }

    pub fn create_user_info(ctx: Context<CreateUserInfos>, pubkey: Pubkey) -> Result<()> {
        let user_info = &mut ctx.accounts.user_info;
        if !user_info.is_initialized {
            user_info.is_initialized = true;
            user_info.is_whitelisted = true;
            user_info.pubkey = pubkey;
            user_info.solAmount = 0;
            user_info.tokenAmount = 0;
        }
        Ok(msg!("Create Whitelist UserInfo {}", pubkey))
    }

    pub fn change_round_status(ctx: Context<ChangeRoundStatus>, status: u8, start_time: u64, end_time: u64) -> Result<()> {
        let airdrop_info = &mut ctx.accounts.airdrop_info;
        match status {
            0 => {
                airdrop_info.roundStatus = AirDropRound::None;
            },
            1 => {
                airdrop_info.roundStatus = AirDropRound::PreSale {
                    start_time: start_time,
                    end_time: end_time,
                }
            },
            2 => {
                airdrop_info.roundStatus = AirDropRound::PublicSale {
                    start_time: start_time,
                    end_time: end_time,
                }
            },
            _ => { return Ok(msg!("UnKnown Command")); }
        }
        Ok(msg!("Successfully Changed RoundStatus"))
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

    #[account(init_if_needed, seeds=[b"user", user.key().as_ref() ], bump, payer = admin, space = 50 )]
    pub user_info: Account<'info, UserInfo>,

    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(mut)]
    pub user: AccountInfo<'info>,

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

    #[account(init_if_needed, seeds=[b"user", user.key().as_ref() ], bump, payer = admin, space = 50 )]
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
}

#[account]
pub struct AirDropInfos {
    is_initialized: bool,
    is_completed: bool,
    preSaleRate: u64,
    publicSaleRate: u64,
    discountRate: u64,
    roundStatus: AirDropRound,
    solAmount: u64,
    tokenAmount: u64
}

#[account]
pub struct UserInfo {
    is_whitelisted: bool,
    is_initialized: bool,
    pubkey: Pubkey,
    tokenAmount: u64,
    solAmount: u64,
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
pub enum Error_code {
    #[msg("Not Active Round")]
    NotActiveSale
    #[msg("Not WhiteListed")]
    InvalidWhiteList
    #[msg("Not Collection Completed yet")]
    InvalidClaim
}