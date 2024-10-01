use anyhow::Result;
use jupiter_amm_interface::{
    try_get_account_data, AccountMap, Amm, AmmContext, AmmUserSetup, KeyedAccount, Quote,
    QuoteParams, Swap, SwapAndAccountMetas, SwapParams,
};
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use spl_token_swap::solana_program::program_pack::Pack;
use spl_token_swap::{
    curve::{base::SwapCurve, constant_price::ConstantPriceCurve},
    state::SwapV1,
};

pub struct SolayerEndoAVSAmm {
    key: Pubkey,
    label: String,
    state: SwapV1,
    reserve_mints: [Pubkey; 2],
    reserves: [u128; 2],
    program_id: Pubkey,
}

impl SolayerEndoAVSAmm {
    fn get_authority(&self) -> Pubkey {
        None
    }
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

    fn update(&mut self, account_map: &AccountMap) -> Result<()> {
        
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {}

    /// Indicates which Swap has to be performed along with all the necessary account metas
    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        let SwapParams {
            token_transfer_authority,
            source_token_account,
            destination_token_account,
            source_mint,
            ..
        } = swap_params;

        let (swap_source, swap_destination) =
            if *source_mint == Pubkey::from(self.state.token_a_mint.to_bytes()) {
                (self.state.token_a, self.state.token_b)
            } else {
                (self.state.token_b, self.state.token_a)
            };

        Ok(SwapAndAccountMetas {
            swap: Swap::TokenSwap,
            account_metas: vec![
                AccountMeta::new_readonly(self.program_id, false),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new_readonly(self.key, false),
                AccountMeta::new_readonly(self.get_authority(), false),
                AccountMeta::new_readonly(*token_transfer_authority, false),
                AccountMeta::new_readonly(*source_token_account, false),
                AccountMeta::new_readonly(Pubkey::from(swap_source.to_bytes()), false),
                AccountMeta::new_readonly(Pubkey::from(swap_destination.to_bytes()), false),
                AccountMeta::new_readonly(*destination_token_account, false),
                AccountMeta::new_readonly(Pubkey::from(self.state.pool_mint.to_bytes()), false),
                AccountMeta::new_readonly(Pubkey::from(self.state.pool_fee_account.to_bytes()), false),
            ],
        })
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
