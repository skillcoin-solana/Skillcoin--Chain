use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserAccount {
    pub name: String,      // 4 + 256
    pub authority: Pubkey, // 32
    pub last_offer_id: u8, // 1
    pub offer_count: u8,   // 1
}

#[account]
#[derive(Default)]
pub struct Offer {
    pub id: u8,              // 1
    pub title: String,       // 4 + 256
    pub description: String, // 4 + 2048
    pub image: String,       // 4 + 2048
    pub price: f64,          // 8
    pub user: Pubkey,        // 32
    pub authority: Pubkey,   // 32
}
