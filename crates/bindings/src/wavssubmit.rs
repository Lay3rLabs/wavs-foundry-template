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

interface WavsSubmit {
    constructor();

    function getData(ISimpleTrigger.TriggerId triggerId) external view returns (bytes memory data);
    function getServiceManager() external view returns (address);
    function getSignature(ISimpleTrigger.TriggerId triggerId) external view returns (bytes memory signature);
    function handleSignedData(bytes memory data, bytes memory signature) external;
    function isValidTriggerId(ISimpleTrigger.TriggerId triggerId) external view returns (bool);
    function setServiceManager(address newServiceManager) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [],
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
    "name": "getServiceManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
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
  },
  {
    "type": "function",
    "name": "setServiceManager",
    "inputs": [
      {
        "name": "newServiceManager",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
pub mod WavsSubmit {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608080604052346026575f80546001600160a01b031916331790556107d5908161002b8239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f3560e01c90816348958442146105cb575080634dda0b43146105a35780639b41bf23146104fc578063a127f1881461041e578063aa32d9f4146103e15763ed0226481461005e575f80fd5b3461037f57604036600319011261037f576004356001600160401b03811161037f5761008e9036906004016106ca565b906024356001600160401b03811161037f576100ae9036906004016106ca565b60015491939092916001600160a01b031633036103835781019060208183031261037f578035906001600160401b03821161037f57019160408383031261037f5760405192604084018481106001600160401b038211176102fd5760405280356001600160401b038116810361037f5784526020810135906001600160401b03821161037f570182601f8201121561037f578035906001600160401b0382116102fd5760405193610169601f8401601f19166020018661072f565b8285526020838301011161037f57815f9260208093018387013784010152602083019182526001600160401b038351165f52600460205260405f20906001600160401b0381116102fd576101c7816101c184546106f7565b84610750565b5f601f821160011461031c5781906101f7939495965f92610311575b50508160011b915f199060031b1c19161790565b90555b51906001600160401b038151165f52600360205260405f208251926001600160401b0384116102fd57610231846101c184546106f7565b602090601f8511600114610292579380610268926001600160401b03965f926102875750508160011b915f199060031b1c19161790565b90555b51165f908152600260205260409020805460ff19166001179055005b015190505f806101e3565b90601f19851691835f52815f20925f5b8181106102e557509160019391876001600160401b039894106102cd575b505050811b01905561026b565b01515f1960f88460031b161c191690555f80806102c0565b929360206001819287860151815501950193016102a2565b634e487b7160e01b5f52604160045260245ffd5b013590505f806101e3565b601f19821695835f5260205f20915f5b8881106103675750836001959697981061034e575b505050811b0190556101fa565b01355f19600384901b60f8161c191690555f8080610341565b9092602060018192868601358155019401910161032c565b5f80fd5b60405162461bcd60e51b815260206004820152603060248201527f4f6e6c79207468652073657276696365206d616e616765722063616e2063616c60448201526f36103a3434b990333ab731ba34b7b71760811b6064820152608490fd5b3461037f57602036600319011261037f576001600160401b0361040261066d565b165f526002602052602060ff60405f2054166040519015158152f35b3461037f57602036600319011261037f576001600160401b0361043f61066d565b165f52600360205260405f20604051905f9080549061045d826106f7565b80855291600181169081156104d55750600114610495575b610491846104858186038261072f565b60405191829182610683565b0390f35b5f90815260208120939250905b8082106104bb5750909150810160200161048582610475565b9192600181602092548385880101520191019092916104a2565b60ff191660208087019190915292151560051b850190920192506104859150839050610475565b3461037f57602036600319011261037f576004356001600160a01b0381169081900361037f575f546001600160a01b0316330361054f576bffffffffffffffffffffffff60a01b60015416176001555f80f35b60405162461bcd60e51b815260206004820152602660248201527f4f6e6c7920746865206f776e65722063616e2063616c6c20746869732066756e60448201526531ba34b7b71760d11b6064820152608490fd5b3461037f575f36600319011261037f576001546040516001600160a01b039091168152602090f35b3461037f57602036600319011261037f576001600160401b036105ec61066d565b165f52600460205260405f205f90805490610606826106f7565b80855291600181169081156104d5575060011461062d57610491846104858186038261072f565b5f90815260208120939250905b8082106106535750909150810160200161048582610475565b91926001816020925483858801015201910190929161063a565b600435906001600160401b038216820361037f57565b9190916020815282518060208301525f5b8181106106b4575060409293505f838284010152601f8019910116010190565b8060208092870101516040828601015201610694565b9181601f8401121561037f578235916001600160401b03831161037f576020838186019501011161037f57565b90600182811c92168015610725575b602083101461071157565b634e487b7160e01b5f52602260045260245ffd5b91607f1691610706565b90601f801991011681019081106001600160401b038211176102fd57604052565b601f821161075d57505050565b5f5260205f20906020601f840160051c83019310610795575b601f0160051c01905b81811061078a575050565b5f815560010161077f565b909150819061077656fea2646970667358221220f7b2cbb911e3578241b77a5b5ccd7d041b0a708bf6293360de2454c39796384c64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R4`&W_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90Ua\x07\xD5\x90\x81a\0+\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81cH\x95\x84B\x14a\x05\xCBWP\x80cM\xDA\x0BC\x14a\x05\xA3W\x80c\x9BA\xBF#\x14a\x04\xFCW\x80c\xA1'\xF1\x88\x14a\x04\x1EW\x80c\xAA2\xD9\xF4\x14a\x03\xE1Wc\xED\x02&H\x14a\0^W_\x80\xFD[4a\x03\x7FW`@6`\x03\x19\x01\x12a\x03\x7FW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\x7FWa\0\x8E\x906\x90`\x04\x01a\x06\xCAV[\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x7FWa\0\xAE\x906\x90`\x04\x01a\x06\xCAV[`\x01T\x91\x93\x90\x92\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\x03\x83W\x81\x01\x90` \x81\x83\x03\x12a\x03\x7FW\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\x7FW\x01\x91`@\x83\x83\x03\x12a\x03\x7FW`@Q\x92`@\x84\x01\x84\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02\xFDW`@R\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x03\x7FW\x84R` \x81\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\x7FW\x01\x82`\x1F\x82\x01\x12\x15a\x03\x7FW\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xFDW`@Q\x93a\x01i`\x1F\x84\x01`\x1F\x19\x16` \x01\x86a\x07/V[\x82\x85R` \x83\x83\x01\x01\x11a\x03\x7FW\x81_\x92` \x80\x93\x01\x83\x87\x017\x84\x01\x01R` \x83\x01\x91\x82R`\x01`\x01`@\x1B\x03\x83Q\x16_R`\x04` R`@_ \x90`\x01`\x01`@\x1B\x03\x81\x11a\x02\xFDWa\x01\xC7\x81a\x01\xC1\x84Ta\x06\xF7V[\x84a\x07PV[_`\x1F\x82\x11`\x01\x14a\x03\x1CW\x81\x90a\x01\xF7\x93\x94\x95\x96_\x92a\x03\x11W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[Q\x90`\x01`\x01`@\x1B\x03\x81Q\x16_R`\x03` R`@_ \x82Q\x92`\x01`\x01`@\x1B\x03\x84\x11a\x02\xFDWa\x021\x84a\x01\xC1\x84Ta\x06\xF7V[` \x90`\x1F\x85\x11`\x01\x14a\x02\x92W\x93\x80a\x02h\x92`\x01`\x01`@\x1B\x03\x96_\x92a\x02\x87WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[Q\x16_\x90\x81R`\x02` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\0[\x01Q\x90P_\x80a\x01\xE3V[\x90`\x1F\x19\x85\x16\x91\x83_R\x81_ \x92_[\x81\x81\x10a\x02\xE5WP\x91`\x01\x93\x91\x87`\x01`\x01`@\x1B\x03\x98\x94\x10a\x02\xCDW[PPP\x81\x1B\x01\x90Ua\x02kV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x02\xC0V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x02\xA2V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x015\x90P_\x80a\x01\xE3V[`\x1F\x19\x82\x16\x95\x83_R` _ \x91_[\x88\x81\x10a\x03gWP\x83`\x01\x95\x96\x97\x98\x10a\x03NW[PPP\x81\x1B\x01\x90Ua\x01\xFAV[\x015_\x19`\x03\x84\x90\x1B`\xF8\x16\x1C\x19\x16\x90U_\x80\x80a\x03AV[\x90\x92` `\x01\x81\x92\x86\x86\x015\x81U\x01\x94\x01\x91\x01a\x03,V[_\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FOnly the service manager can cal`D\x82\x01Ro6\x10:44\xB9\x903:\xB71\xBA4\xB7\xB7\x17`\x81\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03\x7FW` 6`\x03\x19\x01\x12a\x03\x7FW`\x01`\x01`@\x1B\x03a\x04\x02a\x06mV[\x16_R`\x02` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\x7FW` 6`\x03\x19\x01\x12a\x03\x7FW`\x01`\x01`@\x1B\x03a\x04?a\x06mV[\x16_R`\x03` R`@_ `@Q\x90_\x90\x80T\x90a\x04]\x82a\x06\xF7V[\x80\x85R\x91`\x01\x81\x16\x90\x81\x15a\x04\xD5WP`\x01\x14a\x04\x95W[a\x04\x91\x84a\x04\x85\x81\x86\x03\x82a\x07/V[`@Q\x91\x82\x91\x82a\x06\x83V[\x03\x90\xF3[_\x90\x81R` \x81 \x93\x92P\x90[\x80\x82\x10a\x04\xBBWP\x90\x91P\x81\x01` \x01a\x04\x85\x82a\x04uV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x04\xA2V[`\xFF\x19\x16` \x80\x87\x01\x91\x90\x91R\x92\x15\x15`\x05\x1B\x85\x01\x90\x92\x01\x92Pa\x04\x85\x91P\x83\x90Pa\x04uV[4a\x03\x7FW` 6`\x03\x19\x01\x12a\x03\x7FW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x03\x7FW_T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x05OWk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B`\x01T\x16\x17`\x01U_\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOnly the owner can call this fun`D\x82\x01Re1\xBA4\xB7\xB7\x17`\xD1\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03\x7FW_6`\x03\x19\x01\x12a\x03\x7FW`\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\x7FW` 6`\x03\x19\x01\x12a\x03\x7FW`\x01`\x01`@\x1B\x03a\x05\xECa\x06mV[\x16_R`\x04` R`@_ _\x90\x80T\x90a\x06\x06\x82a\x06\xF7V[\x80\x85R\x91`\x01\x81\x16\x90\x81\x15a\x04\xD5WP`\x01\x14a\x06-Wa\x04\x91\x84a\x04\x85\x81\x86\x03\x82a\x07/V[_\x90\x81R` \x81 \x93\x92P\x90[\x80\x82\x10a\x06SWP\x90\x91P\x81\x01` \x01a\x04\x85\x82a\x04uV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x06:V[`\x045\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x03\x7FWV[\x91\x90\x91` \x81R\x82Q\x80` \x83\x01R_[\x81\x81\x10a\x06\xB4WP`@\x92\x93P_\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x80` \x80\x92\x87\x01\x01Q`@\x82\x86\x01\x01R\x01a\x06\x94V[\x91\x81`\x1F\x84\x01\x12\x15a\x03\x7FW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03\x7FW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03\x7FWV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x07%W[` \x83\x10\x14a\x07\x11WV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x07\x06V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02\xFDW`@RV[`\x1F\x82\x11a\x07]WPPPV[_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x07\x95W[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x07\x8AWPPV[_\x81U`\x01\x01a\x07\x7FV[\x90\x91P\x81\x90a\x07vV\xFE\xA2dipfsX\"\x12 \xF7\xB2\xCB\xB9\x11\xE3W\x82A\xB7z[\\\xCD}\x04\x1B\np\x8B\xF6)3`\xDE$T\xC3\x97\x968LdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080806040526004361015610012575f80fd5b5f3560e01c90816348958442146105cb575080634dda0b43146105a35780639b41bf23146104fc578063a127f1881461041e578063aa32d9f4146103e15763ed0226481461005e575f80fd5b3461037f57604036600319011261037f576004356001600160401b03811161037f5761008e9036906004016106ca565b906024356001600160401b03811161037f576100ae9036906004016106ca565b60015491939092916001600160a01b031633036103835781019060208183031261037f578035906001600160401b03821161037f57019160408383031261037f5760405192604084018481106001600160401b038211176102fd5760405280356001600160401b038116810361037f5784526020810135906001600160401b03821161037f570182601f8201121561037f578035906001600160401b0382116102fd5760405193610169601f8401601f19166020018661072f565b8285526020838301011161037f57815f9260208093018387013784010152602083019182526001600160401b038351165f52600460205260405f20906001600160401b0381116102fd576101c7816101c184546106f7565b84610750565b5f601f821160011461031c5781906101f7939495965f92610311575b50508160011b915f199060031b1c19161790565b90555b51906001600160401b038151165f52600360205260405f208251926001600160401b0384116102fd57610231846101c184546106f7565b602090601f8511600114610292579380610268926001600160401b03965f926102875750508160011b915f199060031b1c19161790565b90555b51165f908152600260205260409020805460ff19166001179055005b015190505f806101e3565b90601f19851691835f52815f20925f5b8181106102e557509160019391876001600160401b039894106102cd575b505050811b01905561026b565b01515f1960f88460031b161c191690555f80806102c0565b929360206001819287860151815501950193016102a2565b634e487b7160e01b5f52604160045260245ffd5b013590505f806101e3565b601f19821695835f5260205f20915f5b8881106103675750836001959697981061034e575b505050811b0190556101fa565b01355f19600384901b60f8161c191690555f8080610341565b9092602060018192868601358155019401910161032c565b5f80fd5b60405162461bcd60e51b815260206004820152603060248201527f4f6e6c79207468652073657276696365206d616e616765722063616e2063616c60448201526f36103a3434b990333ab731ba34b7b71760811b6064820152608490fd5b3461037f57602036600319011261037f576001600160401b0361040261066d565b165f526002602052602060ff60405f2054166040519015158152f35b3461037f57602036600319011261037f576001600160401b0361043f61066d565b165f52600360205260405f20604051905f9080549061045d826106f7565b80855291600181169081156104d55750600114610495575b610491846104858186038261072f565b60405191829182610683565b0390f35b5f90815260208120939250905b8082106104bb5750909150810160200161048582610475565b9192600181602092548385880101520191019092916104a2565b60ff191660208087019190915292151560051b850190920192506104859150839050610475565b3461037f57602036600319011261037f576004356001600160a01b0381169081900361037f575f546001600160a01b0316330361054f576bffffffffffffffffffffffff60a01b60015416176001555f80f35b60405162461bcd60e51b815260206004820152602660248201527f4f6e6c7920746865206f776e65722063616e2063616c6c20746869732066756e60448201526531ba34b7b71760d11b6064820152608490fd5b3461037f575f36600319011261037f576001546040516001600160a01b039091168152602090f35b3461037f57602036600319011261037f576001600160401b036105ec61066d565b165f52600460205260405f205f90805490610606826106f7565b80855291600181169081156104d5575060011461062d57610491846104858186038261072f565b5f90815260208120939250905b8082106106535750909150810160200161048582610475565b91926001816020925483858801015201910190929161063a565b600435906001600160401b038216820361037f57565b9190916020815282518060208301525f5b8181106106b4575060409293505f838284010152601f8019910116010190565b8060208092870101516040828601015201610694565b9181601f8401121561037f578235916001600160401b03831161037f576020838186019501011161037f57565b90600182811c92168015610725575b602083101461071157565b634e487b7160e01b5f52602260045260245ffd5b91607f1691610706565b90601f801991011681019081106001600160401b038211176102fd57604052565b601f821161075d57505050565b5f5260205f20906020601f840160051c83019310610795575b601f0160051c01905b81811061078a575050565b5f815560010161077f565b909150819061077656fea2646970667358221220f7b2cbb911e3578241b77a5b5ccd7d041b0a708bf6293360de2454c39796384c64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81cH\x95\x84B\x14a\x05\xCBWP\x80cM\xDA\x0BC\x14a\x05\xA3W\x80c\x9BA\xBF#\x14a\x04\xFCW\x80c\xA1'\xF1\x88\x14a\x04\x1EW\x80c\xAA2\xD9\xF4\x14a\x03\xE1Wc\xED\x02&H\x14a\0^W_\x80\xFD[4a\x03\x7FW`@6`\x03\x19\x01\x12a\x03\x7FW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\x7FWa\0\x8E\x906\x90`\x04\x01a\x06\xCAV[\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x7FWa\0\xAE\x906\x90`\x04\x01a\x06\xCAV[`\x01T\x91\x93\x90\x92\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\x03\x83W\x81\x01\x90` \x81\x83\x03\x12a\x03\x7FW\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\x7FW\x01\x91`@\x83\x83\x03\x12a\x03\x7FW`@Q\x92`@\x84\x01\x84\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02\xFDW`@R\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x03\x7FW\x84R` \x81\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\x7FW\x01\x82`\x1F\x82\x01\x12\x15a\x03\x7FW\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\xFDW`@Q\x93a\x01i`\x1F\x84\x01`\x1F\x19\x16` \x01\x86a\x07/V[\x82\x85R` \x83\x83\x01\x01\x11a\x03\x7FW\x81_\x92` \x80\x93\x01\x83\x87\x017\x84\x01\x01R` \x83\x01\x91\x82R`\x01`\x01`@\x1B\x03\x83Q\x16_R`\x04` R`@_ \x90`\x01`\x01`@\x1B\x03\x81\x11a\x02\xFDWa\x01\xC7\x81a\x01\xC1\x84Ta\x06\xF7V[\x84a\x07PV[_`\x1F\x82\x11`\x01\x14a\x03\x1CW\x81\x90a\x01\xF7\x93\x94\x95\x96_\x92a\x03\x11W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[Q\x90`\x01`\x01`@\x1B\x03\x81Q\x16_R`\x03` R`@_ \x82Q\x92`\x01`\x01`@\x1B\x03\x84\x11a\x02\xFDWa\x021\x84a\x01\xC1\x84Ta\x06\xF7V[` \x90`\x1F\x85\x11`\x01\x14a\x02\x92W\x93\x80a\x02h\x92`\x01`\x01`@\x1B\x03\x96_\x92a\x02\x87WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[Q\x16_\x90\x81R`\x02` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\0[\x01Q\x90P_\x80a\x01\xE3V[\x90`\x1F\x19\x85\x16\x91\x83_R\x81_ \x92_[\x81\x81\x10a\x02\xE5WP\x91`\x01\x93\x91\x87`\x01`\x01`@\x1B\x03\x98\x94\x10a\x02\xCDW[PPP\x81\x1B\x01\x90Ua\x02kV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x02\xC0V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x02\xA2V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x015\x90P_\x80a\x01\xE3V[`\x1F\x19\x82\x16\x95\x83_R` _ \x91_[\x88\x81\x10a\x03gWP\x83`\x01\x95\x96\x97\x98\x10a\x03NW[PPP\x81\x1B\x01\x90Ua\x01\xFAV[\x015_\x19`\x03\x84\x90\x1B`\xF8\x16\x1C\x19\x16\x90U_\x80\x80a\x03AV[\x90\x92` `\x01\x81\x92\x86\x86\x015\x81U\x01\x94\x01\x91\x01a\x03,V[_\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FOnly the service manager can cal`D\x82\x01Ro6\x10:44\xB9\x903:\xB71\xBA4\xB7\xB7\x17`\x81\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03\x7FW` 6`\x03\x19\x01\x12a\x03\x7FW`\x01`\x01`@\x1B\x03a\x04\x02a\x06mV[\x16_R`\x02` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\x7FW` 6`\x03\x19\x01\x12a\x03\x7FW`\x01`\x01`@\x1B\x03a\x04?a\x06mV[\x16_R`\x03` R`@_ `@Q\x90_\x90\x80T\x90a\x04]\x82a\x06\xF7V[\x80\x85R\x91`\x01\x81\x16\x90\x81\x15a\x04\xD5WP`\x01\x14a\x04\x95W[a\x04\x91\x84a\x04\x85\x81\x86\x03\x82a\x07/V[`@Q\x91\x82\x91\x82a\x06\x83V[\x03\x90\xF3[_\x90\x81R` \x81 \x93\x92P\x90[\x80\x82\x10a\x04\xBBWP\x90\x91P\x81\x01` \x01a\x04\x85\x82a\x04uV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x04\xA2V[`\xFF\x19\x16` \x80\x87\x01\x91\x90\x91R\x92\x15\x15`\x05\x1B\x85\x01\x90\x92\x01\x92Pa\x04\x85\x91P\x83\x90Pa\x04uV[4a\x03\x7FW` 6`\x03\x19\x01\x12a\x03\x7FW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x03\x7FW_T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x05OWk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B`\x01T\x16\x17`\x01U_\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOnly the owner can call this fun`D\x82\x01Re1\xBA4\xB7\xB7\x17`\xD1\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03\x7FW_6`\x03\x19\x01\x12a\x03\x7FW`\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\x7FW` 6`\x03\x19\x01\x12a\x03\x7FW`\x01`\x01`@\x1B\x03a\x05\xECa\x06mV[\x16_R`\x04` R`@_ _\x90\x80T\x90a\x06\x06\x82a\x06\xF7V[\x80\x85R\x91`\x01\x81\x16\x90\x81\x15a\x04\xD5WP`\x01\x14a\x06-Wa\x04\x91\x84a\x04\x85\x81\x86\x03\x82a\x07/V[_\x90\x81R` \x81 \x93\x92P\x90[\x80\x82\x10a\x06SWP\x90\x91P\x81\x01` \x01a\x04\x85\x82a\x04uV[\x91\x92`\x01\x81` \x92T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x92\x91a\x06:V[`\x045\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x03\x7FWV[\x91\x90\x91` \x81R\x82Q\x80` \x83\x01R_[\x81\x81\x10a\x06\xB4WP`@\x92\x93P_\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x80` \x80\x92\x87\x01\x01Q`@\x82\x86\x01\x01R\x01a\x06\x94V[\x91\x81`\x1F\x84\x01\x12\x15a\x03\x7FW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03\x7FW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03\x7FWV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x07%W[` \x83\x10\x14a\x07\x11WV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x07\x06V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02\xFDW`@RV[`\x1F\x82\x11a\x07]WPPPV[_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x07\x95W[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x07\x8AWPPV[_\x81U`\x01\x01a\x07\x7FV[\x90\x91P\x81\x90a\x07vV\xFE\xA2dipfsX\"\x12 \xF7\xB2\xCB\xB9\x11\xE3W\x82A\xB7z[\\\xCD}\x04\x1B\np\x8B\xF6)3`\xDE$T\xC3\x97\x968LdsolcC\0\x08\x1C\x003",
    );
    /**Constructor`.
    ```solidity
    constructor();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {}
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
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
    /**Function with signature `getServiceManager()` and selector `0x4dda0b43`.
    ```solidity
    function getServiceManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getServiceManagerCall {}
    ///Container type for the return parameters of the [`getServiceManager()`](getServiceManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getServiceManagerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
            impl ::core::convert::From<getServiceManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: getServiceManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getServiceManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<getServiceManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getServiceManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getServiceManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getServiceManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getServiceManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getServiceManager()";
            const SELECTOR: [u8; 4] = [77u8, 218u8, 11u8, 67u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
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
    /**Function with signature `setServiceManager(address)` and selector `0x9b41bf23`.
    ```solidity
    function setServiceManager(address newServiceManager) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setServiceManagerCall {
        pub newServiceManager: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setServiceManager(address)`](setServiceManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setServiceManagerReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
            impl ::core::convert::From<setServiceManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: setServiceManagerCall) -> Self {
                    (value.newServiceManager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setServiceManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newServiceManager: tuple.0 }
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
            impl ::core::convert::From<setServiceManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setServiceManagerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setServiceManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setServiceManagerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setServiceManagerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setServiceManager(address)";
            const SELECTOR: [u8; 4] = [155u8, 65u8, 191u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.newServiceManager,
                ),)
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
    ///Container for all the [`WavsSubmit`](self) function calls.
    pub enum WavsSubmitCalls {
        getData(getDataCall),
        getServiceManager(getServiceManagerCall),
        getSignature(getSignatureCall),
        handleSignedData(handleSignedDataCall),
        isValidTriggerId(isValidTriggerIdCall),
        setServiceManager(setServiceManagerCall),
    }
    #[automatically_derived]
    impl WavsSubmitCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [72u8, 149u8, 132u8, 66u8],
            [77u8, 218u8, 11u8, 67u8],
            [155u8, 65u8, 191u8, 35u8],
            [161u8, 39u8, 241u8, 136u8],
            [170u8, 50u8, 217u8, 244u8],
            [237u8, 2u8, 38u8, 72u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for WavsSubmitCalls {
        const NAME: &'static str = "WavsSubmitCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 6usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::getData(_) => <getDataCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getServiceManager(_) => {
                    <getServiceManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getSignature(_) => <getSignatureCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::handleSignedData(_) => {
                    <handleSignedDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isValidTriggerId(_) => {
                    <isValidTriggerIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setServiceManager(_) => {
                    <setServiceManagerCall as alloy_sol_types::SolCall>::SELECTOR
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
            static DECODE_SHIMS: &[fn(&[u8], bool) -> alloy_sol_types::Result<WavsSubmitCalls>] = &[
                {
                    fn getSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsSubmitCalls> {
                        <getSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(WavsSubmitCalls::getSignature)
                    }
                    getSignature
                },
                {
                    fn getServiceManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsSubmitCalls> {
                        <getServiceManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(WavsSubmitCalls::getServiceManager)
                    }
                    getServiceManager
                },
                {
                    fn setServiceManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsSubmitCalls> {
                        <setServiceManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(WavsSubmitCalls::setServiceManager)
                    }
                    setServiceManager
                },
                {
                    fn getData(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsSubmitCalls> {
                        <getDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(WavsSubmitCalls::getData)
                    }
                    getData
                },
                {
                    fn isValidTriggerId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsSubmitCalls> {
                        <isValidTriggerIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(WavsSubmitCalls::isValidTriggerId)
                    }
                    isValidTriggerId
                },
                {
                    fn handleSignedData(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsSubmitCalls> {
                        <handleSignedDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(WavsSubmitCalls::handleSignedData)
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
                Self::getServiceManager(inner) => {
                    <getServiceManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::setServiceManager(inner) => {
                    <setServiceManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::getData(inner) => {
                    <getDataCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getServiceManager(inner) => {
                    <getServiceManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::setServiceManager(inner) => {
                    <setServiceManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`WavsSubmit`](self) contract instance.

    See the [wrapper's documentation](`WavsSubmitInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> WavsSubmitInstance<T, P, N> {
        WavsSubmitInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<WavsSubmitInstance<T, P, N>>>
    {
        WavsSubmitInstance::<T, P, N>::deploy(provider)
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
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        WavsSubmitInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`WavsSubmit`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`WavsSubmit`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct WavsSubmitInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for WavsSubmitInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("WavsSubmitInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > WavsSubmitInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`WavsSubmit`](self) contract instance.

        See the [wrapper's documentation](`WavsSubmitInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self { address, provider, _network_transport: ::core::marker::PhantomData }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

        Returns a new instance of the contract, if the deployment was successful.

        For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(provider: P) -> alloy_contract::Result<WavsSubmitInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
        and constructor arguments, if any.

        This is a simple wrapper around creating a `RawCallBuilder` with the data set to
        the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
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
    impl<T, P: ::core::clone::Clone, N> WavsSubmitInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> WavsSubmitInstance<T, P, N> {
            WavsSubmitInstance {
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
        > WavsSubmitInstance<T, P, N>
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
        ///Creates a new call builder for the [`getServiceManager`] function.
        pub fn getServiceManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getServiceManagerCall, N> {
            self.call_builder(&getServiceManagerCall {})
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
        ///Creates a new call builder for the [`setServiceManager`] function.
        pub fn setServiceManager(
            &self,
            newServiceManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setServiceManagerCall, N> {
            self.call_builder(&setServiceManagerCall { newServiceManager })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > WavsSubmitInstance<T, P, N>
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
