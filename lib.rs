
use anchor_lang::prelude::*;

#[program]
pub mod transfer_hook_whitelist {
    use super::*;

    pub fn validate_transfer(ctx: Context<ValidateTransfer>) -> Result<()> {
        let allowed_receiver = ctx.accounts.whitelist.allowed_receiver;
        let recipient = ctx.accounts.to.key();
        require!(recipient == allowed_receiver, TransferError::UnauthorizedReceiver);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ValidateTransfer<'info> {
    #[account()]
    pub from: AccountInfo<'info>,
    #[account()]
    pub to: AccountInfo<'info>,
    pub whitelist: Account<'info, Whitelist>,
}

#[account]
pub struct Whitelist {
    pub allowed_receiver: Pubkey,
}

#[error_code]
pub enum TransferError {
    #[msg("This recipient is not authorized to receive tokens.")]
    UnauthorizedReceiver,
}
