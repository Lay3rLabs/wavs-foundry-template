///Module containing a contract's types and functions.
/**

```solidity
library ISimpleTrigger {
    type TriggerId is uint64;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISimpleTrigger {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TriggerId(u64);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<TriggerId> for u64 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<64>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(self).0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
            }
        }
        #[automatically_derived]
        impl TriggerId {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u64) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u64 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for TriggerId {
            type RustType = u64;
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::type_check(
                    token,
                )
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::detokenize(
                    token,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TriggerId {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISimpleTrigger`](self) contract instance.

    See the [wrapper's documentation](`ISimpleTriggerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISimpleTriggerInstance<T, P, N> {
        ISimpleTriggerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISimpleTrigger`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ISimpleTrigger`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISimpleTriggerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISimpleTriggerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISimpleTriggerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ISimpleTriggerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ISimpleTrigger`](self) contract instance.

        See the [wrapper's documentation](`ISimpleTriggerInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self { address, provider, _network_transport: ::core::marker::PhantomData }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> ISimpleTriggerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISimpleTriggerInstance<T, P, N> {
            ISimpleTriggerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ISimpleTriggerInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ISimpleTriggerInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library ISimpleTrigger {
    type TriggerId is uint64;
}

interface SimpleSubmit {
    constructor(address serviceManager);

    function getData(ISimpleTrigger.TriggerId triggerId) external view returns (bytes memory data);
    function getSignature(ISimpleTrigger.TriggerId triggerId) external view returns (bytes memory signature);
    function handleSignedData(bytes memory data, bytes memory signature) external;
    function isValidTriggerId(ISimpleTrigger.TriggerId triggerId) external view returns (bool);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "serviceManager",
        "type": "address",
        "internalType": "contract ILayerServiceManager"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getData",
    "inputs": [
      {
        "name": "triggerId",
        "type": "uint64",
        "internalType": "ISimpleTrigger.TriggerId"
      }
    ],
    "outputs": [
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getSignature",
    "inputs": [
      {
        "name": "triggerId",
        "type": "uint64",
        "internalType": "ISimpleTrigger.TriggerId"
      }
    ],
    "outputs": [
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "handleSignedData",
    "inputs": [
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isValidTriggerId",
    "inputs": [
      {
        "name": "triggerId",
        "type": "uint64",
        "internalType": "ISimpleTrigger.TriggerId"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod SimpleSubmit {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608034606f57601f6107f138819003918201601f19168301916001600160401b03831184841017607357808492602094604052833981010312606f57516001600160a01b03811690819003606f575f80546001600160a01b03191691909117905560405161076990816100888239f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe6080806040526004361015610012575f80fd5b5f905f3560e01c908163489584421461052b57508063a127f1881461044d578063aa32d9f4146104105763ed0226481461004a575f80fd5b3461040c57604036600319011261040c576004356001600160401b03811161040c5761007a90369060040161062a565b91906024356001600160401b03811161040c5761009b90369060040161062a565b5f549094906001600160a01b0316803b1561040c575f6040518092632a3e7f3b60e01b82526040600483015281806100ed6100da60448301898c6106c4565b8281036003190160248401528c8a6106c4565b03915afa8015610401576103ec575b508201916020818403126103e8578035906001600160401b0382116103e45701936040858403126103e85760405194604086018681106001600160401b038211176103d05760405280356001600160401b03811681036103cc5786526020810135906001600160401b0382116103cc570183601f820112156103e4578035906001600160401b0382116103d057604051946101a1601f8401601f19166020018761068f565b828652602083830101116103cc5781869260208093018388013785010152602085019283526001600160401b038551168452600360205260408420916001600160401b0382116103b8576101ff826101f98554610657565b856106e4565b8490601f83116001146103555761022d929186918361034a575b50508160011b915f199060031b1c19161790565b90555b51916001600160401b03815116825260026020526040822083516001600160401b0381116103365761026c816102668454610657565b846106e4565b6020601f82116001146102cc5790806102a39286976001600160401b0397926102c15750508160011b915f199060031b1c19161790565b90555b51168152600160205260408120600160ff1982541617905580f35b015190505f80610219565b82855280852095601f198316865b81811061031e5750916001600160401b03969791846001959410610306575b505050811b0190556102a6565b01515f1960f88460031b161c191690555f80806102f9565b838301518955600190980197602093840193016102da565b634e487b7160e01b84526041600452602484fd5b013590505f80610219565b8386526020862091601f198416875b8181106103a05750908460019594939210610387575b505050811b019055610230565b01355f19600384901b60f8161c191690555f808061037a565b91936020600181928787013581550195019201610364565b634e487b7160e01b85526041600452602485fd5b8580fd5b634e487b7160e01b86526041600452602486fd5b8480fd5b8380fd5b6103f99194505f9061068f565b5f925f6100fc565b6040513d5f823e3d90fd5b5f80fd5b3461040c57602036600319011261040c576001600160401b036104316105cd565b165f526001602052602060ff60405f2054166040519015158152f35b3461040c57602036600319011261040c576001600160401b0361046e6105cd565b165f52600260205260405f20604051905f9080549061048c82610657565b808552916001811690811561050457506001146104c4575b6104c0846104b48186038261068f565b604051918291826105e3565b0390f35b5f90815260208120939250905b8082106104ea575090915081016020016104b4826104a4565b9192600181602092548385880101520191019092916104d1565b60ff191660208087019190915292151560051b850190920192506104b491508390506104a4565b3461040c57602036600319011261040c576001600160401b0361054c6105cd565b165f52600360205260405f205f9080549061056682610657565b8085529160018116908115610504575060011461058d576104c0846104b48186038261068f565b5f90815260208120939250905b8082106105b3575090915081016020016104b4826104a4565b91926001816020925483858801015201910190929161059a565b600435906001600160401b038216820361040c57565b9190916020815282518060208301525f5b818110610614575060409293505f838284010152601f8019910116010190565b80602080928701015160408286010152016105f4565b9181601f8401121561040c578235916001600160401b03831161040c576020838186019501011161040c57565b90600182811c92168015610685575b602083101461067157565b634e487b7160e01b5f52602260045260245ffd5b91607f1691610666565b90601f801991011681019081106001600160401b038211176106b057604052565b634e487b7160e01b5f52604160045260245ffd5b908060209392818452848401375f828201840152601f01601f1916010190565b601f82116106f157505050565b5f5260205f20906020601f840160051c83019310610729575b601f0160051c01905b81811061071e575050565b5f8155600101610713565b909150819061070a56fea2646970667358221220592e7eac49febcacdf71865d53cca13413ef93af482ac93e4b7f0720f48723bd64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x804`oW`\x1Fa\x07\xF18\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`sW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12`oWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03`oW_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa\x07i\x90\x81a\0\x88\x829\xF3[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81cH\x95\x84B\x14a\x05+WP\x80c\xA1'\xF1\x88\x14a\x04MW\x80c\xAA2\xD9\xF4\x14a\x04\x10Wc\xED\x02&H\x14a\0JW_\x80\xFD[4a\x04\x0CW`@6`\x03\x19\x01\x12a\x04\x0CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04\x0CWa\0z\x906\x90`\x04\x01a\x06*V[\x91\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04\x0CWa\0\x9B\x906\x90`\x04\x01a\x06*V[_T\x90\x94\x90`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x0CW_`@Q\x80\x92c*>\x7F;`\xE0\x1B\x82R`@`\x04\x83\x01R\x81\x80a\0\xEDa\0\xDA`D\x83\x01\x89\x8Ca\x06\xC4V[\x82\x81\x03`\x03\x19\x01`$\x84\x01R\x8C\x8Aa\x06\xC4V[\x03\x91Z\xFA\x80\x15a\x04\x01Wa\x03\xECW[P\x82\x01\x91` \x81\x84\x03\x12a\x03\xE8W\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xE4W\x01\x93`@\x85\x84\x03\x12a\x03\xE8W`@Q\x94`@\x86\x01\x86\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xD0W`@R\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x03\xCCW\x86R` \x81\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xCCW\x01\x83`\x1F\x82\x01\x12\x15a\x03\xE4W\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xD0W`@Q\x94a\x01\xA1`\x1F\x84\x01`\x1F\x19\x16` \x01\x87a\x06\x8FV[\x82\x86R` \x83\x83\x01\x01\x11a\x03\xCCW\x81\x86\x92` \x80\x93\x01\x83\x88\x017\x85\x01\x01R` \x85\x01\x92\x83R`\x01`\x01`@\x1B\x03\x85Q\x16\x84R`\x03` R`@\x84 \x91`\x01`\x01`@\x1B\x03\x82\x11a\x03\xB8Wa\x01\xFF\x82a\x01\xF9\x85Ta\x06WV[\x85a\x06\xE4V[\x84\x90`\x1F\x83\x11`\x01\x14a\x03UWa\x02-\x92\x91\x86\x91\x83a\x03JW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[Q\x91`\x01`\x01`@\x1B\x03\x81Q\x16\x82R`\x02` R`@\x82 \x83Q`\x01`\x01`@\x1B\x03\x81\x11a\x036Wa\x02l\x81a\x02f\x84Ta\x06WV[\x84a\x06\xE4V[` `\x1F\x82\x11`\x01\x14a\x02\xCCW\x90\x80a\x02\xA3\x92\x86\x97`\x01`\x01`@\x1B\x03\x97\x92a\x02\xC1WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[Q\x16\x81R`\x01` R`@\x81 `\x01`\xFF\x19\x82T\x16\x17\x90U\x80\xF3[\x01Q\x90P_\x80a\x02\x19V[\x82\x85R\x80\x85 \x95`\x1F\x19\x83\x16\x86[\x81\x81\x10a\x03\x1EWP\x91`\x01`\x01`@\x1B\x03\x96\x97\x91\x84`\x01\x95\x94\x10a\x03\x06W[PPP\x81\x1B\x01\x90Ua\x02\xA6V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x02\xF9V[\x83\x83\x01Q\x89U`\x01\x90\x98\x01\x97` \x93\x84\x01\x93\x01a\x02\xDAV[cNH{q`\xE0\x1B\x84R`A`\x04R`$\x84\xFD[\x015\x90P_\x80a\x02\x19V[\x83\x86R` \x86 \x91`\x1F\x19\x84\x16\x87[\x81\x81\x10a\x03\xA0WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x03\x87W[PPP\x81\x1B\x01\x90Ua\x020V[\x015_\x19`\x03\x84\x90\x1B`\xF8\x16\x1C\x19\x16\x90U_\x80\x80a\x03zV[\x91\x93` `\x01\x81\x92\x87\x87\x015\x81U\x01\x95\x01\x92\x01a\x03dV[cNH{q`\xE0\x1B\x85R`A`\x04R`$\x85\xFD[\x85\x80\xFD[cNH{q`\xE0\x1B\x86R`A`\x04R`$\x86\xFD[\x84\x80\xFD[\x83\x80\xFD[a\x03\xF9\x91\x94P_\x90a\x06\x8FV[_\x92_a\0\xFCV[`@Q=_\x82>=\x90\xFD[_\x80\xFD[4a\x04\x0CW` 6`\x03\x19\x01\x12a\x04\x0CW`\x01`\x01`@\x1B\x03a\x041a\x05\xCDV[\x16_R`\x01` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04\x0CW` 6`\x03\x19\x01\x12a\x04\x0CW`\x01`\x01`@\x1B\x03a\x04na\x05\xCDV[\x16_R`\x02` R`@_ `@Q\x90_\x90\x80T\x90a\x04\x8C\x82a\x06WV[\x80\x85R\x91`\x01\x81\x16\x90\x81\x15a\x05\x04WP`\x01\x14a\x04\xC4W[a\x04\xC0\x84a\x04\xB4\x81\x86\x03\x82a\x06\x8FV[`@Q\x91\x82\x91\x82a\x05\xE3V[\x03\x90\xF3[_\x90\x81R` \x81 \x93\x92P\x90[\x80\x82\x10a\x04\xEAWP\x90\x91P\x81\x01` \x01a\x04\xB4\x82a\x04\xA4V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x04\xD1V[`\xFF\x19\x16` \x80\x87\x01\x91\x90\x91R\x92\x15\x15`\x05\x1B\x85\x01\x90\x92\x01\x92Pa\x04\xB4\x91P\x83\x90Pa\x04\xA4V[4a\x04\x0CW` 6`\x03\x19\x01\x12a\x04\x0CW`\x01`\x01`@\x1B\x03a\x05La\x05\xCDV[\x16_R`\x03` R`@_ _\x90\x80T\x90a\x05f\x82a\x06WV[\x80\x85R\x91`\x01\x81\x16\x90\x81\x15a\x05\x04WP`\x01\x14a\x05\x8DWa\x04\xC0\x84a\x04\xB4\x81\x86\x03\x82a\x06\x8FV[_\x90\x81R` \x81 \x93\x92P\x90[\x80\x82\x10a\x05\xB3WP\x90\x91P\x81\x01` \x01a\x04\xB4\x82a\x04\xA4V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x05\x9AV[`\x045\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x04\x0CWV[\x91\x90\x91` \x81R\x82Q\x80` \x83\x01R_[\x81\x81\x10a\x06\x14WP`@\x92\x93P_\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x80` \x80\x92\x87\x01\x01Q`@\x82\x86\x01\x01R\x01a\x05\xF4V[\x91\x81`\x1F\x84\x01\x12\x15a\x04\x0CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x04\x0CW` \x83\x81\x86\x01\x95\x01\x01\x11a\x04\x0CWV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x06\x85W[` \x83\x10\x14a\x06qWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x06fV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB0W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`\x1F\x82\x11a\x06\xF1WPPPV[_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x07)W[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x07\x1EWPPV[_\x81U`\x01\x01a\x07\x13V[\x90\x91P\x81\x90a\x07\nV\xFE\xA2dipfsX\"\x12 Y.~\xACI\xFE\xBC\xAC\xDFq\x86]S\xCC\xA14\x13\xEF\x93\xAFH*\xC9>K\x7F\x07 \xF4\x87#\xBDdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080806040526004361015610012575f80fd5b5f905f3560e01c908163489584421461052b57508063a127f1881461044d578063aa32d9f4146104105763ed0226481461004a575f80fd5b3461040c57604036600319011261040c576004356001600160401b03811161040c5761007a90369060040161062a565b91906024356001600160401b03811161040c5761009b90369060040161062a565b5f549094906001600160a01b0316803b1561040c575f6040518092632a3e7f3b60e01b82526040600483015281806100ed6100da60448301898c6106c4565b8281036003190160248401528c8a6106c4565b03915afa8015610401576103ec575b508201916020818403126103e8578035906001600160401b0382116103e45701936040858403126103e85760405194604086018681106001600160401b038211176103d05760405280356001600160401b03811681036103cc5786526020810135906001600160401b0382116103cc570183601f820112156103e4578035906001600160401b0382116103d057604051946101a1601f8401601f19166020018761068f565b828652602083830101116103cc5781869260208093018388013785010152602085019283526001600160401b038551168452600360205260408420916001600160401b0382116103b8576101ff826101f98554610657565b856106e4565b8490601f83116001146103555761022d929186918361034a575b50508160011b915f199060031b1c19161790565b90555b51916001600160401b03815116825260026020526040822083516001600160401b0381116103365761026c816102668454610657565b846106e4565b6020601f82116001146102cc5790806102a39286976001600160401b0397926102c15750508160011b915f199060031b1c19161790565b90555b51168152600160205260408120600160ff1982541617905580f35b015190505f80610219565b82855280852095601f198316865b81811061031e5750916001600160401b03969791846001959410610306575b505050811b0190556102a6565b01515f1960f88460031b161c191690555f80806102f9565b838301518955600190980197602093840193016102da565b634e487b7160e01b84526041600452602484fd5b013590505f80610219565b8386526020862091601f198416875b8181106103a05750908460019594939210610387575b505050811b019055610230565b01355f19600384901b60f8161c191690555f808061037a565b91936020600181928787013581550195019201610364565b634e487b7160e01b85526041600452602485fd5b8580fd5b634e487b7160e01b86526041600452602486fd5b8480fd5b8380fd5b6103f99194505f9061068f565b5f925f6100fc565b6040513d5f823e3d90fd5b5f80fd5b3461040c57602036600319011261040c576001600160401b036104316105cd565b165f526001602052602060ff60405f2054166040519015158152f35b3461040c57602036600319011261040c576001600160401b0361046e6105cd565b165f52600260205260405f20604051905f9080549061048c82610657565b808552916001811690811561050457506001146104c4575b6104c0846104b48186038261068f565b604051918291826105e3565b0390f35b5f90815260208120939250905b8082106104ea575090915081016020016104b4826104a4565b9192600181602092548385880101520191019092916104d1565b60ff191660208087019190915292151560051b850190920192506104b491508390506104a4565b3461040c57602036600319011261040c576001600160401b0361054c6105cd565b165f52600360205260405f205f9080549061056682610657565b8085529160018116908115610504575060011461058d576104c0846104b48186038261068f565b5f90815260208120939250905b8082106105b3575090915081016020016104b4826104a4565b91926001816020925483858801015201910190929161059a565b600435906001600160401b038216820361040c57565b9190916020815282518060208301525f5b818110610614575060409293505f838284010152601f8019910116010190565b80602080928701015160408286010152016105f4565b9181601f8401121561040c578235916001600160401b03831161040c576020838186019501011161040c57565b90600182811c92168015610685575b602083101461067157565b634e487b7160e01b5f52602260045260245ffd5b91607f1691610666565b90601f801991011681019081106001600160401b038211176106b057604052565b634e487b7160e01b5f52604160045260245ffd5b908060209392818452848401375f828201840152601f01601f1916010190565b601f82116106f157505050565b5f5260205f20906020601f840160051c83019310610729575b601f0160051c01905b81811061071e575050565b5f8155600101610713565b909150819061070a56fea2646970667358221220592e7eac49febcacdf71865d53cca13413ef93af482ac93e4b7f0720f48723bd64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81cH\x95\x84B\x14a\x05+WP\x80c\xA1'\xF1\x88\x14a\x04MW\x80c\xAA2\xD9\xF4\x14a\x04\x10Wc\xED\x02&H\x14a\0JW_\x80\xFD[4a\x04\x0CW`@6`\x03\x19\x01\x12a\x04\x0CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x04\x0CWa\0z\x906\x90`\x04\x01a\x06*V[\x91\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\x04\x0CWa\0\x9B\x906\x90`\x04\x01a\x06*V[_T\x90\x94\x90`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x0CW_`@Q\x80\x92c*>\x7F;`\xE0\x1B\x82R`@`\x04\x83\x01R\x81\x80a\0\xEDa\0\xDA`D\x83\x01\x89\x8Ca\x06\xC4V[\x82\x81\x03`\x03\x19\x01`$\x84\x01R\x8C\x8Aa\x06\xC4V[\x03\x91Z\xFA\x80\x15a\x04\x01Wa\x03\xECW[P\x82\x01\x91` \x81\x84\x03\x12a\x03\xE8W\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xE4W\x01\x93`@\x85\x84\x03\x12a\x03\xE8W`@Q\x94`@\x86\x01\x86\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xD0W`@R\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x03\xCCW\x86R` \x81\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xCCW\x01\x83`\x1F\x82\x01\x12\x15a\x03\xE4W\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xD0W`@Q\x94a\x01\xA1`\x1F\x84\x01`\x1F\x19\x16` \x01\x87a\x06\x8FV[\x82\x86R` \x83\x83\x01\x01\x11a\x03\xCCW\x81\x86\x92` \x80\x93\x01\x83\x88\x017\x85\x01\x01R` \x85\x01\x92\x83R`\x01`\x01`@\x1B\x03\x85Q\x16\x84R`\x03` R`@\x84 \x91`\x01`\x01`@\x1B\x03\x82\x11a\x03\xB8Wa\x01\xFF\x82a\x01\xF9\x85Ta\x06WV[\x85a\x06\xE4V[\x84\x90`\x1F\x83\x11`\x01\x14a\x03UWa\x02-\x92\x91\x86\x91\x83a\x03JW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[Q\x91`\x01`\x01`@\x1B\x03\x81Q\x16\x82R`\x02` R`@\x82 \x83Q`\x01`\x01`@\x1B\x03\x81\x11a\x036Wa\x02l\x81a\x02f\x84Ta\x06WV[\x84a\x06\xE4V[` `\x1F\x82\x11`\x01\x14a\x02\xCCW\x90\x80a\x02\xA3\x92\x86\x97`\x01`\x01`@\x1B\x03\x97\x92a\x02\xC1WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[Q\x16\x81R`\x01` R`@\x81 `\x01`\xFF\x19\x82T\x16\x17\x90U\x80\xF3[\x01Q\x90P_\x80a\x02\x19V[\x82\x85R\x80\x85 \x95`\x1F\x19\x83\x16\x86[\x81\x81\x10a\x03\x1EWP\x91`\x01`\x01`@\x1B\x03\x96\x97\x91\x84`\x01\x95\x94\x10a\x03\x06W[PPP\x81\x1B\x01\x90Ua\x02\xA6V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x02\xF9V[\x83\x83\x01Q\x89U`\x01\x90\x98\x01\x97` \x93\x84\x01\x93\x01a\x02\xDAV[cNH{q`\xE0\x1B\x84R`A`\x04R`$\x84\xFD[\x015\x90P_\x80a\x02\x19V[\x83\x86R` \x86 \x91`\x1F\x19\x84\x16\x87[\x81\x81\x10a\x03\xA0WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x03\x87W[PPP\x81\x1B\x01\x90Ua\x020V[\x015_\x19`\x03\x84\x90\x1B`\xF8\x16\x1C\x19\x16\x90U_\x80\x80a\x03zV[\x91\x93` `\x01\x81\x92\x87\x87\x015\x81U\x01\x95\x01\x92\x01a\x03dV[cNH{q`\xE0\x1B\x85R`A`\x04R`$\x85\xFD[\x85\x80\xFD[cNH{q`\xE0\x1B\x86R`A`\x04R`$\x86\xFD[\x84\x80\xFD[\x83\x80\xFD[a\x03\xF9\x91\x94P_\x90a\x06\x8FV[_\x92_a\0\xFCV[`@Q=_\x82>=\x90\xFD[_\x80\xFD[4a\x04\x0CW` 6`\x03\x19\x01\x12a\x04\x0CW`\x01`\x01`@\x1B\x03a\x041a\x05\xCDV[\x16_R`\x01` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x04\x0CW` 6`\x03\x19\x01\x12a\x04\x0CW`\x01`\x01`@\x1B\x03a\x04na\x05\xCDV[\x16_R`\x02` R`@_ `@Q\x90_\x90\x80T\x90a\x04\x8C\x82a\x06WV[\x80\x85R\x91`\x01\x81\x16\x90\x81\x15a\x05\x04WP`\x01\x14a\x04\xC4W[a\x04\xC0\x84a\x04\xB4\x81\x86\x03\x82a\x06\x8FV[`@Q\x91\x82\x91\x82a\x05\xE3V[\x03\x90\xF3[_\x90\x81R` \x81 \x93\x92P\x90[\x80\x82\x10a\x04\xEAWP\x90\x91P\x81\x01` \x01a\x04\xB4\x82a\x04\xA4V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x04\xD1V[`\xFF\x19\x16` \x80\x87\x01\x91\x90\x91R\x92\x15\x15`\x05\x1B\x85\x01\x90\x92\x01\x92Pa\x04\xB4\x91P\x83\x90Pa\x04\xA4V[4a\x04\x0CW` 6`\x03\x19\x01\x12a\x04\x0CW`\x01`\x01`@\x1B\x03a\x05La\x05\xCDV[\x16_R`\x03` R`@_ _\x90\x80T\x90a\x05f\x82a\x06WV[\x80\x85R\x91`\x01\x81\x16\x90\x81\x15a\x05\x04WP`\x01\x14a\x05\x8DWa\x04\xC0\x84a\x04\xB4\x81\x86\x03\x82a\x06\x8FV[_\x90\x81R` \x81 \x93\x92P\x90[\x80\x82\x10a\x05\xB3WP\x90\x91P\x81\x01` \x01a\x04\xB4\x82a\x04\xA4V[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x05\x9AV[`\x045\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x04\x0CWV[\x91\x90\x91` \x81R\x82Q\x80` \x83\x01R_[\x81\x81\x10a\x06\x14WP`@\x92\x93P_\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x80` \x80\x92\x87\x01\x01Q`@\x82\x86\x01\x01R\x01a\x05\xF4V[\x91\x81`\x1F\x84\x01\x12\x15a\x04\x0CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x04\x0CW` \x83\x81\x86\x01\x95\x01\x01\x11a\x04\x0CWV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x06\x85W[` \x83\x10\x14a\x06qWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x06fV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB0W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`\x1F\x82\x11a\x06\xF1WPPPV[_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x07)W[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x07\x1EWPPV[_\x81U`\x01\x01a\x07\x13V[\x90\x91P\x81\x90a\x07\nV\xFE\xA2dipfsX\"\x12 Y.~\xACI\xFE\xBC\xAC\xDFq\x86]S\xCC\xA14\x13\xEF\x93\xAFH*\xC9>K\x7F\x07 \xF4\x87#\xBDdsolcC\0\x08\x1C\x003",
    );
    /**Constructor`.
    ```solidity
    constructor(address serviceManager);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub serviceManager: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.serviceManager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceManager: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.serviceManager,
                ),)
            }
        }
    };
    /**Function with signature `getData(uint64)` and selector `0xa127f188`.
    ```solidity
    function getData(ISimpleTrigger.TriggerId triggerId) external view returns (bytes memory data);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDataCall {
        pub triggerId: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getData(uint64)`](getDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDataReturn {
        pub data: alloy::sol_types::private::Bytes,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISimpleTrigger::TriggerId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: getDataCall) -> Self {
                    (value.triggerId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { triggerId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getDataReturn) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { data: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDataCall {
            type Parameters<'a> = (ISimpleTrigger::TriggerId,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDataReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getData(uint64)";
            const SELECTOR: [u8; 4] = [161u8, 39u8, 241u8, 136u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<ISimpleTrigger::TriggerId as alloy_sol_types::SolType>::tokenize(&self.triggerId),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getSignature(uint64)` and selector `0x48958442`.
    ```solidity
    function getSignature(ISimpleTrigger.TriggerId triggerId) external view returns (bytes memory signature);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSignatureCall {
        pub triggerId: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getSignature(uint64)`](getSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSignatureReturn {
        pub signature: alloy::sol_types::private::Bytes,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISimpleTrigger::TriggerId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getSignatureCall> for UnderlyingRustTuple<'_> {
                fn from(value: getSignatureCall) -> Self {
                    (value.triggerId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { triggerId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getSignatureReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getSignatureReturn) -> Self {
                    (value.signature,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { signature: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSignatureCall {
            type Parameters<'a> = (ISimpleTrigger::TriggerId,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSignatureReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSignature(uint64)";
            const SELECTOR: [u8; 4] = [72u8, 149u8, 132u8, 66u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<ISimpleTrigger::TriggerId as alloy_sol_types::SolType>::tokenize(&self.triggerId),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `handleSignedData(bytes,bytes)` and selector `0xed022648`.
    ```solidity
    function handleSignedData(bytes memory data, bytes memory signature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct handleSignedDataCall {
        pub data: alloy::sol_types::private::Bytes,
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`handleSignedData(bytes,bytes)`](handleSignedDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct handleSignedDataReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Bytes, alloy::sol_types::sol_data::Bytes);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Bytes, alloy::sol_types::private::Bytes);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<handleSignedDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: handleSignedDataCall) -> Self {
                    (value.data, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for handleSignedDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { data: tuple.0, signature: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<handleSignedDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: handleSignedDataReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for handleSignedDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for handleSignedDataCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Bytes, alloy::sol_types::sol_data::Bytes);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = handleSignedDataReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "handleSignedData(bytes,bytes)";
            const SELECTOR: [u8; 4] = [237u8, 2u8, 38u8, 72u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `isValidTriggerId(uint64)` and selector `0xaa32d9f4`.
    ```solidity
    function isValidTriggerId(ISimpleTrigger.TriggerId triggerId) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidTriggerIdCall {
        pub triggerId: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`isValidTriggerId(uint64)`](isValidTriggerIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidTriggerIdReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISimpleTrigger::TriggerId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidTriggerIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: isValidTriggerIdCall) -> Self {
                    (value.triggerId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isValidTriggerIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { triggerId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidTriggerIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isValidTriggerIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isValidTriggerIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isValidTriggerIdCall {
            type Parameters<'a> = (ISimpleTrigger::TriggerId,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isValidTriggerIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isValidTriggerId(uint64)";
            const SELECTOR: [u8; 4] = [170u8, 50u8, 217u8, 244u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<ISimpleTrigger::TriggerId as alloy_sol_types::SolType>::tokenize(&self.triggerId),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`SimpleSubmit`](self) function calls.
    pub enum SimpleSubmitCalls {
        getData(getDataCall),
        getSignature(getSignatureCall),
        handleSignedData(handleSignedDataCall),
        isValidTriggerId(isValidTriggerIdCall),
    }
    #[automatically_derived]
    impl SimpleSubmitCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [72u8, 149u8, 132u8, 66u8],
            [161u8, 39u8, 241u8, 136u8],
            [170u8, 50u8, 217u8, 244u8],
            [237u8, 2u8, 38u8, 72u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SimpleSubmitCalls {
        const NAME: &'static str = "SimpleSubmitCalls";
        const MIN_DATA_LENGTH: usize = 32usize;
        const COUNT: usize = 4usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::getData(_) => <getDataCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getSignature(_) => <getSignatureCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::handleSignedData(_) => {
                    <handleSignedDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isValidTriggerId(_) => {
                    <isValidTriggerIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            )
                -> alloy_sol_types::Result<SimpleSubmitCalls>] = &[
                {
                    fn getSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SimpleSubmitCalls> {
                        <getSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SimpleSubmitCalls::getSignature)
                    }
                    getSignature
                },
                {
                    fn getData(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SimpleSubmitCalls> {
                        <getDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SimpleSubmitCalls::getData)
                    }
                    getData
                },
                {
                    fn isValidTriggerId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SimpleSubmitCalls> {
                        <isValidTriggerIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SimpleSubmitCalls::isValidTriggerId)
                    }
                    isValidTriggerId
                },
                {
                    fn handleSignedData(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SimpleSubmitCalls> {
                        <handleSignedDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SimpleSubmitCalls::handleSignedData)
                    }
                    handleSignedData
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::getData(inner) => {
                    <getDataCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getSignature(inner) => {
                    <getSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::handleSignedData(inner) => {
                    <handleSignedDataCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isValidTriggerId(inner) => {
                    <isValidTriggerIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::getData(inner) => {
                    <getDataCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getSignature(inner) => {
                    <getSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::handleSignedData(inner) => {
                    <handleSignedDataCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::isValidTriggerId(inner) => {
                    <isValidTriggerIdCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SimpleSubmit`](self) contract instance.

    See the [wrapper's documentation](`SimpleSubmitInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SimpleSubmitInstance<T, P, N> {
        SimpleSubmitInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        serviceManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<SimpleSubmitInstance<T, P, N>>>
    {
        SimpleSubmitInstance::<T, P, N>::deploy(provider, serviceManager)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
    and constructor arguments, if any.

    This is a simple wrapper around creating a `RawCallBuilder` with the data set to
    the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        serviceManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        SimpleSubmitInstance::<T, P, N>::deploy_builder(provider, serviceManager)
    }
    /**A [`SimpleSubmit`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`SimpleSubmit`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SimpleSubmitInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SimpleSubmitInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SimpleSubmitInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > SimpleSubmitInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`SimpleSubmit`](self) contract instance.

        See the [wrapper's documentation](`SimpleSubmitInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self { address, provider, _network_transport: ::core::marker::PhantomData }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

        Returns a new instance of the contract, if the deployment was successful.

        For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            serviceManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<SimpleSubmitInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, serviceManager);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
        and constructor arguments, if any.

        This is a simple wrapper around creating a `RawCallBuilder` with the data set to
        the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            serviceManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        serviceManager,
                    })[..],
                ]
                .concat()
                .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> SimpleSubmitInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SimpleSubmitInstance<T, P, N> {
            SimpleSubmitInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > SimpleSubmitInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`getData`] function.
        pub fn getData(
            &self,
            triggerId: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getDataCall, N> {
            self.call_builder(&getDataCall { triggerId })
        }
        ///Creates a new call builder for the [`getSignature`] function.
        pub fn getSignature(
            &self,
            triggerId: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getSignatureCall, N> {
            self.call_builder(&getSignatureCall { triggerId })
        }
        ///Creates a new call builder for the [`handleSignedData`] function.
        pub fn handleSignedData(
            &self,
            data: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, handleSignedDataCall, N> {
            self.call_builder(&handleSignedDataCall { data, signature })
        }
        ///Creates a new call builder for the [`isValidTriggerId`] function.
        pub fn isValidTriggerId(
            &self,
            triggerId: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, isValidTriggerIdCall, N> {
            self.call_builder(&isValidTriggerIdCall { triggerId })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > SimpleSubmitInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
