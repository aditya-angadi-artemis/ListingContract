use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use metaplex_token_metadata;

declare_id!("8TmxZ2fp2a4bcpGL2asTGn5oSk1TEuWnxwHkDN13k137");

#[program]
pub mod quidproquo {
    use super::*;

    pub fn new(
        ctx: Context<Initialize>,
        _data_bump: u8,
        mk_cut: u64,
    ) -> ProgramResult {
        let data_acc = &mut ctx.accounts.data_acc;
        data_acc.market_place = ctx.accounts.beneficiary.key();
        data_acc.market_place_cut = mk_cut;

        Ok(())
    }

    //LIST FOR SALE
    pub fn make(
        ctx: Context<Make>,
        escrowed_maker_tokens_bump: u8,
        _offer_bump: u8,
        offer_made_on: i64,
        offer_taker_amount: u64,
       
    ) -> ProgramResult {
        let offer = &mut ctx.accounts.offer;
        offer.maker = ctx.accounts.offer_maker.key();
        offer.taker_amount = offer_taker_amount;
        offer.mint = ctx.accounts.maker_mint.to_account_info().key();
        offer.escrowed_maker_tokens_bump = escrowed_maker_tokens_bump;
        offer.offer_made_on = offer_made_on;
        offer.expired = false;

        // Transfer the NFT to the escrow account.
       
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.offer_makers_maker_tokens.to_account_info(),
                    to: ctx.accounts.escrowed_maker_tokens.to_account_info(),
                    // The offer_maker had to sign from the client
                    authority: ctx.accounts.offer_maker.to_account_info(),
                },
            ),
            1,
        )
        
    }

    pub fn update_offer(ctx: Context<Update>,  _offer_bump:u8, offer_made_on: i64, updated_offer_amount: u64, ) -> ProgramResult {

        let offer = &mut ctx.accounts.offer;

        if updated_offer_amount <= 0 {
            return Err(ProgramError::Custom(0x11));
        }

        if offer.expired == true {
            return Err(ProgramError::Custom(0x12));
        }

        offer.taker_amount = updated_offer_amount;
        msg!("Amount updated to {}", updated_offer_amount);
        Ok(())
    }

    //BUY token
    pub fn accept(ctx: Context<Accept>, _offer_bump:u8, offer_made_on:i64) -> ProgramResult {
        
        
        let offer = &mut ctx.accounts.offer;
        offer.expired = true;
       let mut taker_amount = ctx.accounts.offer.taker_amount;


      
       let market_cut = ctx.accounts.data_acc.market_place_cut * taker_amount / 1000;
       
       let mut sfb:u16 = 0; 
      
       if ctx.accounts.token_metadata_account.owner == ctx.accounts.system_program.key && ctx.accounts.token_metadata_account.lamports() == 0 {
            sfb = 0;
       } else {
            let result = metaplex_token_metadata::state::Metadata::from_account_info(&ctx.accounts.token_metadata_account);
      
           
            match result {
                Ok(metadata) => {
                    sfb = metadata.data.seller_fee_basis_points;
                 
                }
                 Err(e) => {
                     sfb = 0;
                 
                }
            }
        }
       let sfb_cut = sfb as u64 * taker_amount / 10000;
       taker_amount = taker_amount - (market_cut + sfb_cut);

        if *ctx.accounts.market_maker.key != ctx.accounts.data_acc.market_place {
            return Err(ProgramError::Custom(0x1));
        }
    
        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            ctx.accounts.offer_taker.key,
            ctx.accounts.offer_maker.key,
             taker_amount,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_ix,
            &[
                ctx.accounts.offer_taker.to_account_info(),
                ctx.accounts.offer_maker.to_account_info(),
                ctx.accounts.offer.to_account_info(),
            ],
        )?;

        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            ctx.accounts.offer_taker.key,
            ctx.accounts.market_maker.key,
             market_cut,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_ix,
            &[
                ctx.accounts.offer_taker.to_account_info(),
                ctx.accounts.market_maker.to_account_info(),
                ctx.accounts.offer.to_account_info(),
            ],
        )?;

        if sfb_cut > 0 {    
        
            if let Some(x) = metaplex_token_metadata::state::Metadata::from_account_info(&ctx.accounts.token_metadata_account)?.data.creators {
                let mut y = 0;

            for i in x {
                    if y == 0 {
                        if i.address != *ctx.accounts.creator0.key {
                            return Err(ProgramError::Custom(0x1));
                        }
                        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
                            ctx.accounts.offer_taker.key,
                            ctx.accounts.creator0.key,
                            (sfb_cut as u64 * i.share as u64)  / 100,
                        );
                        
                        anchor_lang::solana_program::program::invoke(
                            &transfer_ix,
                            &[
                                ctx.accounts.offer_taker.to_account_info(),
                                ctx.accounts.creator0.to_account_info(),
                                ctx.accounts.offer.to_account_info(),
                            ],
                        )?;
                    }
                    else if y == 1 {
                        if i.address != *ctx.accounts.creator1.key {
                            return Err(ProgramError::Custom(0x1));
                        }
                        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
                            ctx.accounts.offer_taker.key,
                            ctx.accounts.creator1.key,
                            (sfb_cut as u64 * i.share as u64) / 100,
                        );
                        
                        anchor_lang::solana_program::program::invoke(
                            &transfer_ix,
                            &[
                                ctx.accounts.offer_taker.to_account_info(),
                                ctx.accounts.creator1.to_account_info(),
                                ctx.accounts.offer.to_account_info(),
                            ],
                        )?;
                    }
                    else if y == 2 {
                        if i.address != *ctx.accounts.creator2.key {
                            return Err(ProgramError::Custom(0x1));
                        }
                        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
                            ctx.accounts.offer_taker.key,
                            ctx.accounts.creator2.key,
                            (sfb_cut as u64 * i.share as u64) / 100,
                        );
                        
                        anchor_lang::solana_program::program::invoke(
                            &transfer_ix,
                            &[
                                ctx.accounts.offer_taker.to_account_info(),
                                ctx.accounts.creator2.to_account_info(),
                                ctx.accounts.offer.to_account_info(),
                            ],
                        )?;
                    }
                    else if y == 3 {
                        if i.address != *ctx.accounts.creator3.key {
                            return Err(ProgramError::Custom(0x1));
                        }
                        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
                            ctx.accounts.offer_taker.key,
                            ctx.accounts.creator3.key,
                            (sfb_cut as u64 * i.share as u64) / 100,
                        );
                        
                        anchor_lang::solana_program::program::invoke(
                            &transfer_ix,
                            &[
                                ctx.accounts.offer_taker.to_account_info(),
                                ctx.accounts.creator3.to_account_info(),
                                ctx.accounts.offer.to_account_info(),
                            ],
                        )?;
                    }
                    else if y == 4 {
                        if i.address != *ctx.accounts.creator1.key {
                            return Err(ProgramError::Custom(0x1));
                        }
                        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
                            ctx.accounts.offer_taker.key,
                            ctx.accounts.creator4.key,
                            (sfb_cut as u64 * i.share as u64) / 100,
                        );
                        
                        anchor_lang::solana_program::program::invoke(
                            &transfer_ix,
                            &[
                                ctx.accounts.offer_taker.to_account_info(),
                                ctx.accounts.creator4.to_account_info(),
                                ctx.accounts.offer.to_account_info(),
                            ],
                        )?;
                    }
                    y = y + 1;

            }

            }

        }

            anchor_spl::token::transfer(
                            CpiContext::new_with_signer(
                                ctx.accounts.token_program.to_account_info(),
                                anchor_spl::token::Transfer {
                                    from: ctx.accounts.escrowed_maker_tokens.to_account_info(),
                                    to: ctx.accounts.offer_takers_maker_tokens.to_account_info(),
                                    authority: ctx.accounts.escrowed_maker_tokens.to_account_info(),
                                },
                                &[&[
                                    ctx.accounts.offer.key().as_ref(),
                                    &[ctx.accounts.offer.escrowed_maker_tokens_bump],
                                ]],
                            ),
                           
                          1,
            )?;

            anchor_spl::token::close_account(CpiContext::new_with_signer(
                            ctx.accounts.token_program.to_account_info(),
                            anchor_spl::token::CloseAccount {
                                account: ctx.accounts.escrowed_maker_tokens.to_account_info(),
                                destination: ctx.accounts.offer_maker.to_account_info(),
                                authority: ctx.accounts.escrowed_maker_tokens.to_account_info(),
                            },
                            &[&[
                                ctx.accounts.offer.key().as_ref(),
                                &[ctx.accounts.offer.escrowed_maker_tokens_bump],
                            ]],
            ))?;
          
            Ok(())
 

    }

    pub fn cancel(ctx: Context<Cancel>, _offer_bump:u8, offer_made_on: i64) -> ProgramResult {

        let offer = &mut ctx.accounts.offer;
        if offer.expired == true {
            return Err(ProgramError::Custom(0x11));

        }
        offer.expired = true;


        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.escrowed_maker_tokens.to_account_info(),
                    to: ctx.accounts.offer_makers_maker_tokens.to_account_info(),
                    authority: ctx.accounts.escrowed_maker_tokens.to_account_info(),
                },
                &[&[
                    ctx.accounts.offer.key().as_ref(),
                    &[ctx.accounts.offer.escrowed_maker_tokens_bump],
                ]],
            ),
            1,
        )?;

        anchor_spl::token::close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::CloseAccount {
                account: ctx.accounts.escrowed_maker_tokens.to_account_info(),
                destination: ctx.accounts.offer_maker.to_account_info(),
                authority: ctx.accounts.escrowed_maker_tokens.to_account_info(),
            },
            &[&[
                ctx.accounts.offer.key().as_ref(),
                &[ctx.accounts.offer.escrowed_maker_tokens_bump],
            ]],
        ))
    }

}


#[account]
pub struct Data {

    pub market_place: Pubkey,
    
    pub market_place_cut: u64,

}

#[account]
pub struct Offer {

    pub maker: Pubkey,
    
    pub taker_amount: u64,

    pub mint: Pubkey,

    pub escrowed_maker_tokens_bump: u8,

    pub offer_made_on: i64,

    pub expired: bool
}

#[derive(Accounts)]
#[instruction(data_bump: u8)]

pub struct Initialize<'info> {
    #[account(init, payer=payer, seeds = [b"data".as_ref()], bump = data_bump, space = 8 + 32 + 8 + 32 + 64 + 8)]
    pub data_acc: Account<'info, Data>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub beneficiary: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

#[derive(Accounts)]
#[instruction(escrowed_maker_tokens_bump: u8, offer_bump:u8, offer_made_on:i64)]
pub struct Make<'info> {
    #[account(init, payer = offer_maker, 
        seeds = [offer_maker.to_account_info().key.as_ref(), maker_mint.to_account_info().key.as_ref(), offer_made_on.to_be_bytes().as_ref()], 
        bump = offer_bump,  
        space = 300)]
    pub offer: Account<'info, Offer>,

    #[account(mut)]
    pub offer_maker: Signer<'info>,
    #[account(mut, constraint = offer_makers_maker_tokens.mint == maker_mint.key())]
    pub offer_makers_maker_tokens: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = offer_maker,
        seeds = [offer.key().as_ref()],
        bump = escrowed_maker_tokens_bump,
        token::mint = maker_mint,
        token::authority = escrowed_maker_tokens,
    )]
    pub escrowed_maker_tokens: Account<'info, TokenAccount>,

    pub maker_mint: Account<'info, Mint>,

    pub data_acc: Account<'info, Data>,
  
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(offer_bump:u8, offer_made_on:i64)]
pub struct Update<'info> {

    #[account(
        mut,
        seeds = [offer_maker.to_account_info().key.as_ref(), maker_mint.to_account_info().key.as_ref(), offer_made_on.to_be_bytes().as_ref()],
        bump = offer_bump,
        constraint = offer.maker == *offer_maker.key,
    )]
    pub offer: Account<'info, Offer>,

    #[account(mut)]
    pub offer_maker: Signer<'info>,

    pub maker_mint: Account<'info, Mint>,
  
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}


#[derive(Accounts)]
#[instruction(offer_bump:u8, offer_made_on:i64, stick_bump:u8)]
pub struct Accept<'info> {
    #[account(
        mut,
        seeds = [offer_maker.to_account_info().key.as_ref(), maker_mint.to_account_info().key.as_ref(), offer_made_on.to_be_bytes().as_ref()],
        bump = offer_bump,
        constraint = offer.maker == *offer_maker.key,
        close = offer_maker

    )]
    pub offer: Box<Account<'info, Offer>>,

    #[account(
        mut,
        seeds = [offer.key().as_ref()],
        bump = offer.escrowed_maker_tokens_bump
    )]
    pub escrowed_maker_tokens: Box<Account<'info, TokenAccount>>,

    pub maker_mint: Account<'info, Mint>,

    #[account(mut)]
    pub offer_maker: AccountInfo<'info>,
    #[account(mut)]
    pub offer_taker: Signer<'info>,

 
    #[account(init_if_needed, payer = offer_taker, associated_token::mint = maker_mint, associated_token::authority = offer_taker)]
    pub offer_takers_maker_tokens: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    
    #[account()]
    pub token_metadata_account: AccountInfo<'info>,
    
    #[account()]
    pub token_metadata_program: AccountInfo<'info>,

    #[account(mut)]
    pub market_maker: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

    pub data_acc: Account<'info, Data>,

     #[account(mut)]
    pub creator0: AccountInfo<'info>,

    #[account(mut)]
    pub creator1: AccountInfo<'info>,

    #[account(mut)]
    pub creator2: AccountInfo<'info>,

    #[account(mut)]
    pub creator3: AccountInfo<'info>,

    #[account(mut)]
    pub creator4: AccountInfo<'info>
}

#[derive(Accounts)]
#[instruction(offer_bump:u8, offer_made_on:i64)]
pub struct Cancel<'info> {
    #[account(
        mut,
        seeds = [offer_maker.to_account_info().key.as_ref(), maker_mint.to_account_info().key.as_ref(), offer_made_on.to_be_bytes().as_ref()],
        bump = offer_bump,
        constraint = offer.maker == *offer_maker.key,
        close = offer_maker
    )]
    pub offer: Account<'info, Offer>,

    #[account(mut)]
    pub offer_maker: Signer<'info>,

    #[account(mut)]
    pub offer_makers_maker_tokens: Account<'info, TokenAccount>,

    pub maker_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [offer.key().as_ref()],
        bump = offer.escrowed_maker_tokens_bump
    )]
    pub escrowed_maker_tokens: Account<'info, TokenAccount>,


    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

    pub data_acc: Account<'info, Data>,
}


