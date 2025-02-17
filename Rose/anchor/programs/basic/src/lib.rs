use anchor_lang::prelude::*;

declare_id!("6z68wfurCMYkZG51s1Et9BJEd9nJGUusjHXNt4dGbNNF");

#[program]
pub mod basic {
    use super::*;

    pub fn greet(_ctx: Context<Initialize>) -> Result<()> {

        let from_pubkey = _ctx.Accounts.sender.to_account_info();
        let to_pubkey = _ctx.Accounts.recipient.to_account_info();
        
        msg!("Sere Mejor!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
