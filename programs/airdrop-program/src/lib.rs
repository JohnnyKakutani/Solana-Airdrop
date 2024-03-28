use anchor_lang::prelude::*;

declare_id!("HerreDvW9RAun1Vq18XCvfddSn9P2cseH9DbVRqdi9j1");

#[program]
pub mod airdrop_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
