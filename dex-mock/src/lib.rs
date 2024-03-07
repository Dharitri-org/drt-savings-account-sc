#![no_std]

dharitri_wasm::imports!();
dharitri_wasm::derive_imports!();

const MOAX_DECIMALS: u64 = 1_000_000_000_000_000_000;

#[dharitri_wasm::contract]
pub trait DexMock {
    #[init]
    fn init(&self) {}

    #[payable("*")]
    #[endpoint]
    fn deposit(&self) {}

    #[payable("*")]
    #[endpoint(swapTokensFixedInput)]
    fn swap_tokens_fixed_input(
        &self,
        #[payment_token] _token_in: TokenIdentifier,
        #[payment_amount] amount_in: BigUint,
        token_out: TokenIdentifier,
        _amount_out_min: BigUint,
        opt_accept_funds_func: OptionalValue<ManagedBuffer>,
    ) -> DctTokenPayment<Self::Api> {
        let caller = self.blockchain().get_caller();
        let amount_out = amount_in * 100u64 / MOAX_DECIMALS;
        let func = match opt_accept_funds_func {
            OptionalValue::Some(f) => f,
            OptionalValue::None => ManagedBuffer::default(),
        };

        let _ = Self::Api::send_api_impl().direct_dct_execute(
            &caller,
            &token_out,
            &amount_out,
            self.blockchain().get_gas_left(),
            &func,
            &ManagedArgBuffer::new_empty(),
        );

        DctTokenPayment {
            token_identifier: token_out,
            token_nonce: 0,
            amount: amount_out,
            token_type: DctTokenType::Fungible,
        }
    }
}
