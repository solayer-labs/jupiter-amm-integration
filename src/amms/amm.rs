use std::collections::HashMap;

use anyhow::Result;
use jupiter_amm_interface::{
    AccountMap, Amm, AmmContext, KeyedAccount, Quote, QuoteParams, Swap, SwapAndAccountMetas,
    SwapParams,
};
use solana_sdk::instruction::AccountMeta;
use solana_sdk::{pubkey, pubkey::Pubkey};
use spl_token_swap::solana_program::program_pack::Pack;
use spl_token_swap::{curve::base::SwapCurve, state::SwapV1};

pub const SOLAYER_SOL: Pubkey = pubkey!("sSo14endRuUbvQaJS3dq36Q829a3A6BEfoeeRGJywEh");
pub const BYBIT_AVS_MINT: Pubkey = pubkey!("bybitztPBdV3KRMfwr8ysBf7xx15JvwvuNWa7Wj9Wgz");
pub const OKX_AVS_MINT: Pubkey = pubkey!("okxwLVtTcWuhyTdps5LQHCjRJ2nEbvSBXhEJBsWBUiV");
pub const KAMINO_FINANCE_AVS_MINT: Pubkey = pubkey!("kmnoqco9kFsSBSNZRtnxSxABQPKL65Y6HTpogBbAdpi");
pub const BITGET_AVS_MINT: Pubkey = pubkey!("BGSo18NXTWGtyNa5DBBP1ZCfUFRPWj6bECrPKakn8qN");
pub const SONIC_AVS_MINT: Pubkey = pubkey!("sonickAJFiVLcYXx25X9vpF293udaWqDMUCiGtk7dg2");
pub const HASH_KEY_CLOUD_AVS_MINT: Pubkey = pubkey!("hash4eTHsuZakZiHg5vfQwFtBaEhhC9SXRYsZm4Br7k");
pub const BONK_AVS_MINT: Pubkey = pubkey!("bonkABCQVasnhyVAvB2zYFSCRMGB6xKhpthKuCnsU5K");
pub const ALT_LAYER_AVS_MINT: Pubkey = pubkey!("6C41vb9AqJzmbWZ4zi6eCGJz3vSKrwjxfu8N77SRRtyr");

pub const BYBIT_AVS_ADDRESS: Pubkey = pubkey!("Hny1SeUgUHZEixUkPtMgcXq6xoeD8JSGMnUwrBqML5dY");
pub const OKX_AVS_ADDRESS: Pubkey = pubkey!("DCoTHVgbQDiwYE7n822jXHxMnjJTuESwF1iubDtnzAMX");
pub const KAMINO_FINANCE_AVS_ADDRESS: Pubkey = pubkey!("GnudctaPLkvjm1FQpZ2pBRLsWxbt9VWpoyq2gFCt3vfa");
pub const BITGET_AVS_ADDRESS: Pubkey = pubkey!("9PKigVr684uDNBfQKvGBrwGQ5KYjHQspTPcmLDv8aqS2");
pub const SONIC_AVS_ADDRESS: Pubkey = pubkey!("HBkJwH6rjUUBK1wNhBuYgo9Wnk1iCx2phduyxWCQj6uk");
pub const HASH_KEY_CLOUD_AVS_ADDRESS: Pubkey = pubkey!("745mkVyUsYe6FrSujnKQGiaLVAS6ac19dmU5XfNRzbwE");
pub const BONK_AVS_ADDRESS: Pubkey = pubkey!("E2VVTVBeaV8U197Mnvpa9skjaxPDDiHeTpGK1CkvW6fL");
pub const ALT_LAYER_AVS_ADDRESS: Pubkey = pubkey!("EBYsvMRRYnjbeGQ91mruwTBx8C4vtC8nUFhCGX4xmgHX");

lazy_static::lazy_static! {
    pub static ref AVS_MINT_TO_AVS_ADDRESS: HashMap<Pubkey, Pubkey> ={
        let mut m = HashMap::new();
        m.insert(BYBIT_AVS_MINT, BYBIT_AVS_ADDRESS);
        m.insert(OKX_AVS_MINT, OKX_AVS_ADDRESS);
        m.insert(KAMINO_FINANCE_AVS_MINT, KAMINO_FINANCE_AVS_ADDRESS);
        m.insert(BITGET_AVS_MINT, BITGET_AVS_ADDRESS);
        m.insert(SONIC_AVS_MINT, SONIC_AVS_ADDRESS);
        m.insert(HASH_KEY_CLOUD_AVS_MINT, HASH_KEY_CLOUD_AVS_ADDRESS);
        m.insert(BONK_AVS_MINT, BONK_AVS_ADDRESS);
        m.insert(ALT_LAYER_AVS_MINT, ALT_LAYER_AVS_ADDRESS);
        m
    };
}

pub struct SolayerEndoAVSAmm {
    key: Pubkey,
    label: String,
    state: SwapV1,
    reserve_mints: [Pubkey; 2],
    reserves: [u128; 2],
    program_id: Pubkey,
}

impl Clone for SolayerEndoAVSAmm {
    fn clone(&self) -> Self {
        SolayerEndoAVSAmm {
            key: self.key,
            label: self.label.clone(),
            state: SwapV1 {
                is_initialized: self.state.is_initialized,
                bump_seed: self.state.bump_seed,
                token_program_id: self.state.token_program_id,
                token_a: self.state.token_a,
                token_b: self.state.token_b,
                pool_mint: self.state.pool_mint,
                token_a_mint: self.state.token_a_mint,
                token_b_mint: self.state.token_b_mint,
                pool_fee_account: self.state.pool_fee_account,
                fees: self.state.fees.clone(),
                swap_curve: SwapCurve {
                    curve_type: self.state.swap_curve.curve_type,
                    calculator: self.state.swap_curve.calculator.clone(),
                },
            },
            reserve_mints: self.reserve_mints,
            program_id: self.program_id,
            reserves: self.reserves,
        }
    }
}

impl Amm for SolayerEndoAVSAmm {
    fn from_keyed_account(keyed_account: &KeyedAccount, _amm_context: &AmmContext) -> Result<Self> {
        // Skip the first byte which is version
        let state = SwapV1::unpack(&keyed_account.account.data[1..])?;
        let reserve_mints = [
            Pubkey::from(state.token_a_mint.to_bytes()),
            Pubkey::from(state.token_b_mint.to_bytes()),
        ];

        Ok(Self {
            key: keyed_account.key,
            label: "Solayer".into(),
            state,
            reserve_mints,
            program_id: keyed_account.account.owner,
            reserves: Default::default(),
        })
    }

    fn label(&self) -> String {
        self.label.clone()
    }

    fn program_id(&self) -> Pubkey {
        self.program_id
    }

    fn key(&self) -> Pubkey {
        self.key
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        self.reserve_mints.to_vec()
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![
            Pubkey::from(self.state.token_a.to_bytes()),
            Pubkey::from(self.state.token_b.to_bytes()),
        ]
    }

    fn update(&mut self, _account_map: &AccountMap) -> Result<()> {
        // no action is needed here
        Ok(())
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        // endoAVS token to sSOL would always be 1:1, no fee
        Ok(Quote {
            in_amount: quote_params.amount,
            out_amount: quote_params.amount,
            fee_amount: 0,
            fee_mint: quote_params.input_mint,
            ..Quote::default()
        })
    }

    /// Indicates which Swap has to be performed along with all the necessary account metas
    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        if swap_params.source_mint == SOLAYER_SOL {
            // delegate sSOL to endoAVS
            Ok(SwapAndAccountMetas {
                // TODO: this should be Swap::Solayer
                swap: Swap::TokenSwap,
                account_metas: vec![
                    // staker
                    AccountMeta::new(swap_params.token_transfer_authority, true),
                    // endoAvs
                    AccountMeta::new(*AVS_MINT_TO_AVS_ADDRESS.get(&swap_params.destination_mint).unwrap(), false),
                    // avsTokenMint
                    AccountMeta::new_readonly(swap_params.destination_mint, false),
                    // delegatedTokenVault
                    AccountMeta::new_readonly(swap_params.source_token_account, false),
                    // delegatedTokenMint
                    AccountMeta::new_readonly(swap_params.source_mint, false),
                    // stakerAvsTokenAccount
                    AccountMeta::new_readonly(swap_params.destination_token_account, false),
                    // tokenProgram
                    AccountMeta::new_readonly(spl_token::id(), false),
                    // associatedTokenProgram
                    AccountMeta::new_readonly(spl_associated_token_account::id(), false),
                    // systemProgram
                    AccountMeta::new_readonly(solana_system_program::id(), false)
                ],
            })
        } else {
            // undelegate endoAVS to sSOL
            Ok(SwapAndAccountMetas {
                // TODO: this should be Swap::Solayer
                swap: Swap::TokenSwap,
                account_metas: vec![
                    // staker
                    AccountMeta::new(swap_params.token_transfer_authority, true),
                    // endoAvs
                    AccountMeta::new(*AVS_MINT_TO_AVS_ADDRESS.get(&swap_params.source_mint).unwrap(), false),
                    // avsTokenMint
                    AccountMeta::new_readonly(swap_params.source_mint, false),
                    // delegatedTokenVault
                    AccountMeta::new_readonly(swap_params.destination_token_account, false),
                    // delegatedTokenMint
                    AccountMeta::new_readonly(swap_params.destination_mint, false),
                    // stakerAvsTokenAccount
                    AccountMeta::new_readonly(swap_params.source_token_account, false),
                    // tokenProgram
                    AccountMeta::new_readonly(spl_token::id(), false),
                    // associatedTokenProgram
                    AccountMeta::new_readonly(spl_associated_token_account::id(), false),
                    // systemProgram
                    AccountMeta::new_readonly(solana_system_program::id(), false)
                ],
            })
        }
    }

    // Indicates that whether ExactOut mode is supported
    fn supports_exact_out(&self) -> bool {
        true
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }

    // TODO: shorten this len
    fn get_accounts_len(&self) -> usize {
        32 // Default to a near whole legacy transaction to penalize no implementation
    }
}
