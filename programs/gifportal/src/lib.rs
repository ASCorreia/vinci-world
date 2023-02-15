use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, InitializeMint, MintTo, Burn, Transfer};
use std::str::FromStr;

//declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
declare_id!("8Ecahw39DA3GPcxP2PShkThCG3gdbhRwDUeAgjbjZPS9");

#[program]
pub mod gifportal {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        //let pubkey = Pubkey::from_str("6eGKgDhFAaLYkxoDMyx2NU4RyrSKfCXdRmqtjT7zodxQ").unwrap();
        let pubkey = Pubkey::from_str("AHYic562KhgtAEkb1rSesqS87dFYRcfXb4WwWus3Zc9C").unwrap();
        let base_account = &mut ctx.accounts.base_account;
        let result = base_account.key();
        msg!(&result.to_string());
        base_account.total_gifs = 0;
        base_account.owner = pubkey;

        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct{
            ammount: gif_link.to_string(),
            user_address: *user.to_account_info().key
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn remove_gif(ctx: Context<RemoveGif>, user_address: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;

        for n in 0..base_account.gif_list.len() {
            if user_address == base_account.gif_list[n].user_address.to_string()
            {
                base_account.total_gifs -= 1;
                base_account.gif_list.remove(n);
            }
        }
        Ok(())
    }

    /*
        TBD Create function to mint token according to baseaccount ammount. Try using two different contexts (Improve later)
        Accounts to have the PubKey of the user? In that case, a simple "mint operation" can send the tokens to the appropriate account
        Accounts to not have the PubKey of the user? In that case, two contexts might be needed, or more information from the client
        
        TBD Create funtion to transfer tokens from accounts to vault (our account) to be used for tournament registration
        
        TBD Create function to transfer tokens from vault to accounts
        Signer should be our wallet, signed from the client
    */
    
    pub fn mint_token(ctx: Context<MintToken>, ammount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts); //add a CPI context with signer (CpiContext::new_with_signer) for the user to sign (signer_seeds? Try PubKey)

        token::mint_to(cpi_ctx, ammount)?;
        Ok(())
    }

    pub fn burn_token(ctx: Context<BurnToken>, ammount: u64) -> Result<()> {
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::burn(cpi_ctx, ammount)?;
        Ok(())
    }

    ///pay_tournament function will deserialize the provided remaining accounts in order to add the rewarded ammount to the appropriate account
    pub fn pay_tournament(ctx:Context<PayTournament>, ammount: u64) -> Result<()>
    {   
        let win_accounts: usize;

        let total_accounts: usize = ctx.remaining_accounts.len();
        if total_accounts != 1 as usize {
            panic!("Total accounts is {}", total_accounts);
        }

        if total_accounts  >= 1 && total_accounts <= 10 { //1 to be replaced by appropriate number
            win_accounts = 1;
        }
        else if total_accounts > 10 && total_accounts <= 24 {
            win_accounts = 6;
        }
        else {
            win_accounts = 10;
        }
        for mut account in ctx.remaining_accounts.iter() {
            let mut awarded_accounts = 0;

            let _account_key = account.key();
            let mut data = account.try_borrow_mut_data()?;
            //let data_to_write = data.as_ref();

            //Deserialize the data from the account and save it in an Account variable
            let mut account_to_write = BaseAccount::try_deserialize(&mut data.as_ref()).expect("Error Deserializing Data");

            if ctx.accounts.user.is_signer == true  && ctx.accounts.user.to_account_info().key() == account_to_write.owner {
                if awarded_accounts != win_accounts {
                    account_to_write.total_gifs += ammount;
                    awarded_accounts += 1;
                }
            }
           
            //Serialize the data back
            account_to_write.try_serialize(&mut data.as_mut())?;

        }
        Ok(())
    }

    pub fn add_ammount(ctx: Context<AddAmount>, ammount: u64) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs += ammount;
        Ok(())
    }

    pub fn remove_ammount(ctx: Context<RemoveAmmount>, ammount: u64) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        if ammount < base_account.total_gifs
        {
            base_account.total_gifs -= ammount;
        }
        else
        {
            base_account.total_gifs = 0;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, seeds = [b"Placeholder_13", user.key().as_ref()], bump, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct RemoveGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)] //Is mut needed?? To be checked, as we dont modify the account!
    pub mint: UncheckedAccount<'info>, //Token Account (Represents the token)
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>, //Destination of the mint. The token that we want to send to tokens to!
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: UncheckedAccount<'info> //Authority to mint the token
}

#[derive(Accounts)]
pub struct BurnToken<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)] //Is mut needed?? To be checked, as we dont modify the account!
    pub mint: UncheckedAccount<'info>, //Token Account (Represents the token)
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>, //Destination of the mint. The token that we want to send tokens to!
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: UncheckedAccount<'info> //Authority to mint the token
}

#[derive(Accounts)]
pub struct AddAmount<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>
}

#[derive(Accounts)]
pub struct RemoveAmmount<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub ammount: String,
    pub user_address: Pubkey
}

pub struct PayTournament<'info> {
    #[account(mut)]
    pub user: Signer<'info>
}


#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub owner: Pubkey,
    pub gif_list: Vec<ItemStruct>
}
