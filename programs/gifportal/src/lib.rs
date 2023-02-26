use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use anchor_spl::token::{Token, InitializeMint, MintTo, Burn, Transfer};
use std::str::FromStr;
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v3};

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
        TBD Create funtion to transfer tokens from accounts to vault (our account) to be used for tournament registration
        Should ATAs be created directly from the client during the baseAccount creation? In that case, the program would only get the address
        Shoud ATAs be created by the program? More complexity and no added value?
        
        TBD Create function to transfer tokens from vault to accounts (from an ATA owned by our wallet)
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

    /* 
        TBD At the moment the function is receiving the computed PDA. Should it receive the public key of the user?
    */
    pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
        let account_to_claim = &mut ctx.accounts.base_account;
        let signer_key = ctx.accounts.payer.to_account_info().key();

        if account_to_claim.owner == signer_key && account_to_claim.total_gifs != 0 {
            let cpi_accounts = MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            };
    
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
            token::mint_to(cpi_ctx, account_to_claim.total_gifs)?;
            account_to_claim.total_gifs = 0;
        }

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

        let mut awarded_accounts = 0;
        for account in ctx.remaining_accounts.iter() {

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

    /*
        for details regarding the metadata account and master edition account, refer to metaplex docs at
        https://docs.metaplex.com/programs/token-metadata/accounts
     */
    pub fn mint_nft(ctx: Context<MintNFT>, creator_key: Pubkey, uri: String, title: String) -> Result<()> {
        msg!("Initializing Mint NFT");
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        msg!("CPI Accounts Assigned");
        let cpi_program = ctx.accounts.token_program.to_account_info();
        msg!("CPI Program Assigned");
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        msg!("CPI Context Assigned");
        token::mint_to(cpi_ctx, 1)?;
        msg!("Token Minted !!!");
        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Account Info Assigned");
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: ctx.accounts.mint_authority.key(),
                verified: false,
                share: 0,
            },
        ];
        msg!("Creator Assigned");
        let symbol = std::string::ToString::to_string("VINCI");
        invoke(
            &create_metadata_accounts_v3(
                ctx.accounts.token_metadata_program.key(), //program_id
                ctx.accounts.metadata.key(), //metadata_account
                ctx.accounts.mint.key(), //mint
                ctx.accounts.mint_authority.key(), //mint_authority
                ctx.accounts.payer.key(), //payer
                ctx.accounts.payer.key(), //update_authority
                title, //name
                symbol, //symbol
                uri, //uri
                Some(creator), //creators
                500, //seller_fee_basis_points
                true, //update_authority_is_signer
                false, //is_mutable
                None, //collection
                None, //uses
                None, //collection_details
            ),
            account_info.as_slice(),
        )?;
        msg!("Metadata Account Created !!!");
        let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Master Edition Account Infos Assigned");
        invoke(
            &create_master_edition_v3(
                ctx.accounts.token_metadata_program.key(), //program_id
                ctx.accounts.master_edition.key(), //edition
                ctx.accounts.mint.key(), //mint
                ctx.accounts.payer.key(), //update_authority
                ctx.accounts.mint_authority.key(), //mint_authority
                ctx.accounts.metadata.key(), //metadata (metadata_account)
                ctx.accounts.payer.key(), //payer
                Some(0), //max_supply
            ),
            master_edition_infos.as_slice(),
        )?;
        msg!("Master Edition Nft Minted !!!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, seeds = [b"Placeholder_39", user.key().as_ref()], bump, payer = user, space = 9000)]
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

#[derive(Accounts)]
pub struct PayTournament<'info> {
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)] //Is mut needed?? To be checked, as we dont modify the account!
    pub mint: UncheckedAccount<'info>, //Token Account (Represents the token)
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>, //Destination of the mint. The token that we want to send to tokens to!
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: UncheckedAccount<'info>, //Authority to mint the token (Shall be the Signer as well)
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    //#[account(mut)]
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub owner: Pubkey,
    pub gif_list: Vec<ItemStruct>
}
