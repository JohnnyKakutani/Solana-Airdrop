use anchor_lang::prelude::*;

declare_id!("HerreDvW9RAun1Vq18XCvfddSn9P2cseH9DbVRqdi9j1");

#[program]
pub mod airdrop_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let airdrop_info = &mut ctx.accounts.airdrop_info;
        if !airdrop_info {
            airdrop_info.is_initialized = true;
            airdrop_info.is_completed = false;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init_if_needed, seeds=[b"airdrop", admin.key().as_ref, bump, payer = admin, space = 67])]
    pub airdrop_info: Account<'info, AirDropInfos>,
    
    #[account(mut)]
    pub admin: Signer<'info>

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateUserInfos<'info> {

    #[account(init_if_needed, seeds=[b"user", user.key().as_ref()], bump, payer = admin, space = 50)]
    pub user_info: Account<'info. UserInfo>,

    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(mut)]
    pub user: AccountInfo<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AirDropInfos {
    is_initialized: bool;
    is_completed: bool;
    preSaleRate: u64;
    publicSaleRate: u64;
    discountRate: u64;
    roundStatus: AirDropRound;
    solAmount: u64;
    tokenAmount: u64;
}

#[account]
pub struct UserInfo {
    is_whitelisted: bool;
    is_initialized: bool;
    pubkey: Pubkey;
    tokenAmount: u64;
    solAmount: u64;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub enum AirDropRound {
    None,
    PreSale {
        start_time: u64;
        end_time: u64;
    },
    PublicSale {
        start_time: u64;
        end_time: u64;
    },
}
