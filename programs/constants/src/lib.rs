use anchor_lang::prelude::*;

declare_id!("8eEkNRoCiGYyiJNoEfvXrFQcLNtem6gPMSXb8sooRB4D");

pub mod constants;
pub mod states;
use crate::constants::*;
use crate::states::*;

#[program]
pub mod skillcoin_chain {
    use super::*;

    pub fn init_user(ctx: Context<InitUser>, name: String) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;
        user_account.name = name;
        user_account.offer_count = 0;
        user_account.last_offer_id = 0;
        user_account.authority = authority.key();

        Ok(())
    }

    pub fn create_offer(
        ctx: Context<CreateOffer>,
        title: String,
        description: String,
        image: String,
        price: f64,
    ) -> Result<()> {
        let offer = &mut ctx.accounts.offer;
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        offer.id = user_account.last_offer_id;
        offer.title = title;
        offer.description = description;
        offer.image = image;
        offer.price = price;
        offer.user = user_account.key();
        offer.authority = authority.key();

        user_account.last_offer_id = user_account.last_offer_id.checked_add(1).unwrap();
        user_account.offer_count = user_account.offer_count.checked_add(1).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitUser<'info> {
    #[account(
        init,
        seeds=[USER_SEED,authority.key.as_ref()],
        bump,
        payer=authority,
        space= 298 + 8
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct CreateOffer<'info> {
    #[account(
        init,
        seeds=[OFFER_SEED, authority.key().as_ref(), &[user_account.last_offer_id as u8].as_ref()],
        bump,
        payer=authority,
        space=4437+8
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,
        seeds=[USER_SEED, authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
