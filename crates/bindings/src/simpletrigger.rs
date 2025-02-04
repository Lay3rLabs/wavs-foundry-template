///Module containing a contract's types and functions.
/**

```solidity
library ISimpleTrigger {
    type TriggerId is uint64;
    struct TriggerInfo { TriggerId triggerId; address creator; bytes data; }
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
    /**```solidity
    struct TriggerInfo { TriggerId triggerId; address creator; bytes data; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TriggerInfo {
        pub triggerId: <TriggerId as alloy::sol_types::SolType>::RustType,
        pub creator: alloy::sol_types::private::Address,
        pub data: alloy::sol_types::private::Bytes,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> =
            (TriggerId, alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Bytes);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <TriggerId as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Bytes,
        );
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
        impl ::core::convert::From<TriggerInfo> for UnderlyingRustTuple<'_> {
            fn from(value: TriggerInfo) -> Self {
                (value.triggerId, value.creator, value.data)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TriggerInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { triggerId: tuple.0, creator: tuple.1, data: tuple.2 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TriggerInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TriggerInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <TriggerId as alloy_sol_types::SolType>::tokenize(&self.triggerId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.creator,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for TriggerInfo {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for TriggerInfo {
            const NAME: &'static str = "TriggerInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TriggerInfo(uint64 triggerId,address creator,bytes data)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <TriggerId as alloy_sol_types::SolType>::eip712_data_word(
                            &self.triggerId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.creator,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.data,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TriggerInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <TriggerId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.triggerId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.creator,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.data,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <TriggerId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.triggerId,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.creator,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
    struct TriggerInfo {
        TriggerId triggerId;
        address creator;
        bytes data;
    }
}

interface SimpleTrigger {
    event NewTrigger(bytes);

    function addTrigger(bytes memory data) external;
    function getTrigger(ISimpleTrigger.TriggerId triggerId) external view returns (ISimpleTrigger.TriggerInfo memory);
    function nextTriggerId() external view returns (ISimpleTrigger.TriggerId);
    function triggerIdsByCreator(address, uint256) external view returns (ISimpleTrigger.TriggerId);
    function triggersById(ISimpleTrigger.TriggerId) external view returns (address creator, bytes memory data);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "addTrigger",
    "inputs": [
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getTrigger",
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
        "type": "tuple",
        "internalType": "struct ISimpleTrigger.TriggerInfo",
        "components": [
          {
            "name": "triggerId",
            "type": "uint64",
            "internalType": "ISimpleTrigger.TriggerId"
          },
          {
            "name": "creator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "data",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "nextTriggerId",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "ISimpleTrigger.TriggerId"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "triggerIdsByCreator",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "ISimpleTrigger.TriggerId"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "triggersById",
    "inputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "ISimpleTrigger.TriggerId"
      }
    ],
    "outputs": [
      {
        "name": "creator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "NewTrigger",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
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
pub mod SimpleTrigger {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60808060405234601557610679908161001a8239f35b5f80fdfe6080806040526004361015610012575f80fd5b5f3560e01c90816342227fa41461049857508063913b1fbf14610435578063ce289612146103d4578063e31e0788146101155763e328ed7714610053575f80fd5b34610111576020366003190112610111576004356001600160401b038116809103610111576001600160401b039060606040805161009081610528565b5f81525f60208201520152805f525f60205261010d60405f206100d46001808060a01b0383541692604051956100c587610528565b86526020860193845201610564565b9060408401918252604051948594602086525116602085015260018060a01b039051166040840152516060808401526080830190610604565b0390f35b5f80fd5b34610111576020366003190112610111576004356001600160401b03811161011157366023820112156101115780600401356001600160401b0381116102fb576040519161016d601f8301601f191660200184610543565b818352366024838301011161011157815f9260246020930183860137830101526002549060016001600160401b038316016001600160401b0381116103c0576001600160401b031680926001600160401b0319161760025560405160408101908082106001600160401b038311176102fb5760019160405233815260208101928352835f525f60205260405f2090828060a01b039051166bffffffffffffffffffffffff60a01b825416178155019181519283516001600160401b0381116102fb5761023982546104f0565b601f811161037b575b50602094601f821160011461031a579481929394955f9261030f575b50508160011b915f199060031b1c19161790555b335f52600160205260405f2091825491680100000000000000008310156102fb576102c7837f86eacd23610d81706516de1ed0476c87772fdf939c7c771fbbd7f0230d619e689560016102f6960181556104bb565b6001600160401b03829392549160031b92831b921b191617905551604051918291602083526020830190610604565b0390a1005b634e487b7160e01b5f52604160045260245ffd5b01519050858061025e565b601f19821695835f52805f20915f5b8881106103635750836001959697981061034b575b505050811b019055610272565b01515f1960f88460031b161c1916905585808061033e565b91926020600181928685015181550194019201610329565b825f5260205f20601f830160051c810191602084106103b6575b601f0160051c01905b8181106103ab5750610242565b5f815560010161039e565b9091508190610395565b634e487b7160e01b5f52601160045260245ffd5b34610111576020366003190112610111576004356001600160401b038116809103610111575f525f60205260405f206104186001808060a01b038354169201610564565b9061010d6040519283928352604060208401526040830190610604565b34610111576040366003190112610111576004356001600160a01b0381169081900361011157602435905f52600160205260405f20908154811015610111576104886001600160401b03916020936104bb565b90549060031b1c16604051908152f35b34610111575f366003190112610111576020906001600160401b03600254168152f35b91909180548310156104dc575f52601860205f208360021c019260031b1690565b634e487b7160e01b5f52603260045260245ffd5b90600182811c9216801561051e575b602083101461050a57565b634e487b7160e01b5f52602260045260245ffd5b91607f16916104ff565b606081019081106001600160401b038211176102fb57604052565b90601f801991011681019081106001600160401b038211176102fb57604052565b9060405191825f825492610577846104f0565b80845293600181169081156105e2575060011461059e575b5061059c92500383610543565b565b90505f9291925260205f20905f915b8183106105c657505090602061059c928201015f61058f565b60209193508060019154838589010152019101909184926105ad565b90506020925061059c94915060ff191682840152151560051b8201015f61058f565b91908251928382525f5b84811061062e575050825f602080949584010152601f8019910116010190565b8060208092840101518282860101520161060e56fea26469706673582212203cd43f041923417d267f60755826085da2fbe0ce71b9de804b84118e4f99e1ec64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R4`\x15Wa\x06y\x90\x81a\0\x1A\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81cB\"\x7F\xA4\x14a\x04\x98WP\x80c\x91;\x1F\xBF\x14a\x045W\x80c\xCE(\x96\x12\x14a\x03\xD4W\x80c\xE3\x1E\x07\x88\x14a\x01\x15Wc\xE3(\xEDw\x14a\0SW_\x80\xFD[4a\x01\x11W` 6`\x03\x19\x01\x12a\x01\x11W`\x045`\x01`\x01`@\x1B\x03\x81\x16\x80\x91\x03a\x01\x11W`\x01`\x01`@\x1B\x03\x90```@\x80Qa\0\x90\x81a\x05(V[_\x81R_` \x82\x01R\x01R\x80_R_` Ra\x01\r`@_ a\0\xD4`\x01\x80\x80`\xA0\x1B\x03\x83T\x16\x92`@Q\x95a\0\xC5\x87a\x05(V[\x86R` \x86\x01\x93\x84R\x01a\x05dV[\x90`@\x84\x01\x91\x82R`@Q\x94\x85\x94` \x86RQ\x16` \x85\x01R`\x01\x80`\xA0\x1B\x03\x90Q\x16`@\x84\x01RQ``\x80\x84\x01R`\x80\x83\x01\x90a\x06\x04V[\x03\x90\xF3[_\x80\xFD[4a\x01\x11W` 6`\x03\x19\x01\x12a\x01\x11W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\x11W6`#\x82\x01\x12\x15a\x01\x11W\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xFBW`@Q\x91a\x01m`\x1F\x83\x01`\x1F\x19\x16` \x01\x84a\x05CV[\x81\x83R6`$\x83\x83\x01\x01\x11a\x01\x11W\x81_\x92`$` \x93\x01\x83\x86\x017\x83\x01\x01R`\x02T\x90`\x01`\x01`\x01`@\x1B\x03\x83\x16\x01`\x01`\x01`@\x1B\x03\x81\x11a\x03\xC0W`\x01`\x01`@\x1B\x03\x16\x80\x92`\x01`\x01`@\x1B\x03\x19\x16\x17`\x02U`@Q`@\x81\x01\x90\x80\x82\x10`\x01`\x01`@\x1B\x03\x83\x11\x17a\x02\xFBW`\x01\x91`@R3\x81R` \x81\x01\x92\x83R\x83_R_` R`@_ \x90\x82\x80`\xA0\x1B\x03\x90Q\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x82T\x16\x17\x81U\x01\x91\x81Q\x92\x83Q`\x01`\x01`@\x1B\x03\x81\x11a\x02\xFBWa\x029\x82Ta\x04\xF0V[`\x1F\x81\x11a\x03{W[P` \x94`\x1F\x82\x11`\x01\x14a\x03\x1AW\x94\x81\x92\x93\x94\x95_\x92a\x03\x0FW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[3_R`\x01` R`@_ \x91\x82T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x02\xFBWa\x02\xC7\x83\x7F\x86\xEA\xCD#a\r\x81pe\x16\xDE\x1E\xD0Gl\x87w/\xDF\x93\x9C|w\x1F\xBB\xD7\xF0#\ra\x9Eh\x95`\x01a\x02\xF6\x96\x01\x81Ua\x04\xBBV[`\x01`\x01`@\x1B\x03\x82\x93\x92T\x91`\x03\x1B\x92\x83\x1B\x92\x1B\x19\x16\x17\x90UQ`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x06\x04V[\x03\x90\xA1\0[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x01Q\x90P\x85\x80a\x02^V[`\x1F\x19\x82\x16\x95\x83_R\x80_ \x91_[\x88\x81\x10a\x03cWP\x83`\x01\x95\x96\x97\x98\x10a\x03KW[PPP\x81\x1B\x01\x90Ua\x02rV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x85\x80\x80a\x03>V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x03)V[\x82_R` _ `\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a\x03\xB6W[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x03\xABWPa\x02BV[_\x81U`\x01\x01a\x03\x9EV[\x90\x91P\x81\x90a\x03\x95V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[4a\x01\x11W` 6`\x03\x19\x01\x12a\x01\x11W`\x045`\x01`\x01`@\x1B\x03\x81\x16\x80\x91\x03a\x01\x11W_R_` R`@_ a\x04\x18`\x01\x80\x80`\xA0\x1B\x03\x83T\x16\x92\x01a\x05dV[\x90a\x01\r`@Q\x92\x83\x92\x83R`@` \x84\x01R`@\x83\x01\x90a\x06\x04V[4a\x01\x11W`@6`\x03\x19\x01\x12a\x01\x11W`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x01\x11W`$5\x90_R`\x01` R`@_ \x90\x81T\x81\x10\x15a\x01\x11Wa\x04\x88`\x01`\x01`@\x1B\x03\x91` \x93a\x04\xBBV[\x90T\x90`\x03\x1B\x1C\x16`@Q\x90\x81R\xF3[4a\x01\x11W_6`\x03\x19\x01\x12a\x01\x11W` \x90`\x01`\x01`@\x1B\x03`\x02T\x16\x81R\xF3[\x91\x90\x91\x80T\x83\x10\x15a\x04\xDCW_R`\x18` _ \x83`\x02\x1C\x01\x92`\x03\x1B\x16\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x05\x1EW[` \x83\x10\x14a\x05\nWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x04\xFFV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02\xFBW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02\xFBW`@RV[\x90`@Q\x91\x82_\x82T\x92a\x05w\x84a\x04\xF0V[\x80\x84R\x93`\x01\x81\x16\x90\x81\x15a\x05\xE2WP`\x01\x14a\x05\x9EW[Pa\x05\x9C\x92P\x03\x83a\x05CV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a\x05\xC6WPP\x90` a\x05\x9C\x92\x82\x01\x01_a\x05\x8FV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\x05\xADV[\x90P` \x92Pa\x05\x9C\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a\x05\x8FV[\x91\x90\x82Q\x92\x83\x82R_[\x84\x81\x10a\x06.WPP\x82_` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x80` \x80\x92\x84\x01\x01Q\x82\x82\x86\x01\x01R\x01a\x06\x0EV\xFE\xA2dipfsX\"\x12 <\xD4?\x04\x19#A}&\x7F`uX&\x08]\xA2\xFB\xE0\xCEq\xB9\xDE\x80K\x84\x11\x8EO\x99\xE1\xECdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080806040526004361015610012575f80fd5b5f3560e01c90816342227fa41461049857508063913b1fbf14610435578063ce289612146103d4578063e31e0788146101155763e328ed7714610053575f80fd5b34610111576020366003190112610111576004356001600160401b038116809103610111576001600160401b039060606040805161009081610528565b5f81525f60208201520152805f525f60205261010d60405f206100d46001808060a01b0383541692604051956100c587610528565b86526020860193845201610564565b9060408401918252604051948594602086525116602085015260018060a01b039051166040840152516060808401526080830190610604565b0390f35b5f80fd5b34610111576020366003190112610111576004356001600160401b03811161011157366023820112156101115780600401356001600160401b0381116102fb576040519161016d601f8301601f191660200184610543565b818352366024838301011161011157815f9260246020930183860137830101526002549060016001600160401b038316016001600160401b0381116103c0576001600160401b031680926001600160401b0319161760025560405160408101908082106001600160401b038311176102fb5760019160405233815260208101928352835f525f60205260405f2090828060a01b039051166bffffffffffffffffffffffff60a01b825416178155019181519283516001600160401b0381116102fb5761023982546104f0565b601f811161037b575b50602094601f821160011461031a579481929394955f9261030f575b50508160011b915f199060031b1c19161790555b335f52600160205260405f2091825491680100000000000000008310156102fb576102c7837f86eacd23610d81706516de1ed0476c87772fdf939c7c771fbbd7f0230d619e689560016102f6960181556104bb565b6001600160401b03829392549160031b92831b921b191617905551604051918291602083526020830190610604565b0390a1005b634e487b7160e01b5f52604160045260245ffd5b01519050858061025e565b601f19821695835f52805f20915f5b8881106103635750836001959697981061034b575b505050811b019055610272565b01515f1960f88460031b161c1916905585808061033e565b91926020600181928685015181550194019201610329565b825f5260205f20601f830160051c810191602084106103b6575b601f0160051c01905b8181106103ab5750610242565b5f815560010161039e565b9091508190610395565b634e487b7160e01b5f52601160045260245ffd5b34610111576020366003190112610111576004356001600160401b038116809103610111575f525f60205260405f206104186001808060a01b038354169201610564565b9061010d6040519283928352604060208401526040830190610604565b34610111576040366003190112610111576004356001600160a01b0381169081900361011157602435905f52600160205260405f20908154811015610111576104886001600160401b03916020936104bb565b90549060031b1c16604051908152f35b34610111575f366003190112610111576020906001600160401b03600254168152f35b91909180548310156104dc575f52601860205f208360021c019260031b1690565b634e487b7160e01b5f52603260045260245ffd5b90600182811c9216801561051e575b602083101461050a57565b634e487b7160e01b5f52602260045260245ffd5b91607f16916104ff565b606081019081106001600160401b038211176102fb57604052565b90601f801991011681019081106001600160401b038211176102fb57604052565b9060405191825f825492610577846104f0565b80845293600181169081156105e2575060011461059e575b5061059c92500383610543565b565b90505f9291925260205f20905f915b8183106105c657505090602061059c928201015f61058f565b60209193508060019154838589010152019101909184926105ad565b90506020925061059c94915060ff191682840152151560051b8201015f61058f565b91908251928382525f5b84811061062e575050825f602080949584010152601f8019910116010190565b8060208092840101518282860101520161060e56fea26469706673582212203cd43f041923417d267f60755826085da2fbe0ce71b9de804b84118e4f99e1ec64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81cB\"\x7F\xA4\x14a\x04\x98WP\x80c\x91;\x1F\xBF\x14a\x045W\x80c\xCE(\x96\x12\x14a\x03\xD4W\x80c\xE3\x1E\x07\x88\x14a\x01\x15Wc\xE3(\xEDw\x14a\0SW_\x80\xFD[4a\x01\x11W` 6`\x03\x19\x01\x12a\x01\x11W`\x045`\x01`\x01`@\x1B\x03\x81\x16\x80\x91\x03a\x01\x11W`\x01`\x01`@\x1B\x03\x90```@\x80Qa\0\x90\x81a\x05(V[_\x81R_` \x82\x01R\x01R\x80_R_` Ra\x01\r`@_ a\0\xD4`\x01\x80\x80`\xA0\x1B\x03\x83T\x16\x92`@Q\x95a\0\xC5\x87a\x05(V[\x86R` \x86\x01\x93\x84R\x01a\x05dV[\x90`@\x84\x01\x91\x82R`@Q\x94\x85\x94` \x86RQ\x16` \x85\x01R`\x01\x80`\xA0\x1B\x03\x90Q\x16`@\x84\x01RQ``\x80\x84\x01R`\x80\x83\x01\x90a\x06\x04V[\x03\x90\xF3[_\x80\xFD[4a\x01\x11W` 6`\x03\x19\x01\x12a\x01\x11W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\x11W6`#\x82\x01\x12\x15a\x01\x11W\x80`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x02\xFBW`@Q\x91a\x01m`\x1F\x83\x01`\x1F\x19\x16` \x01\x84a\x05CV[\x81\x83R6`$\x83\x83\x01\x01\x11a\x01\x11W\x81_\x92`$` \x93\x01\x83\x86\x017\x83\x01\x01R`\x02T\x90`\x01`\x01`\x01`@\x1B\x03\x83\x16\x01`\x01`\x01`@\x1B\x03\x81\x11a\x03\xC0W`\x01`\x01`@\x1B\x03\x16\x80\x92`\x01`\x01`@\x1B\x03\x19\x16\x17`\x02U`@Q`@\x81\x01\x90\x80\x82\x10`\x01`\x01`@\x1B\x03\x83\x11\x17a\x02\xFBW`\x01\x91`@R3\x81R` \x81\x01\x92\x83R\x83_R_` R`@_ \x90\x82\x80`\xA0\x1B\x03\x90Q\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x82T\x16\x17\x81U\x01\x91\x81Q\x92\x83Q`\x01`\x01`@\x1B\x03\x81\x11a\x02\xFBWa\x029\x82Ta\x04\xF0V[`\x1F\x81\x11a\x03{W[P` \x94`\x1F\x82\x11`\x01\x14a\x03\x1AW\x94\x81\x92\x93\x94\x95_\x92a\x03\x0FW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[3_R`\x01` R`@_ \x91\x82T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x02\xFBWa\x02\xC7\x83\x7F\x86\xEA\xCD#a\r\x81pe\x16\xDE\x1E\xD0Gl\x87w/\xDF\x93\x9C|w\x1F\xBB\xD7\xF0#\ra\x9Eh\x95`\x01a\x02\xF6\x96\x01\x81Ua\x04\xBBV[`\x01`\x01`@\x1B\x03\x82\x93\x92T\x91`\x03\x1B\x92\x83\x1B\x92\x1B\x19\x16\x17\x90UQ`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x06\x04V[\x03\x90\xA1\0[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x01Q\x90P\x85\x80a\x02^V[`\x1F\x19\x82\x16\x95\x83_R\x80_ \x91_[\x88\x81\x10a\x03cWP\x83`\x01\x95\x96\x97\x98\x10a\x03KW[PPP\x81\x1B\x01\x90Ua\x02rV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x85\x80\x80a\x03>V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x03)V[\x82_R` _ `\x1F\x83\x01`\x05\x1C\x81\x01\x91` \x84\x10a\x03\xB6W[`\x1F\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x03\xABWPa\x02BV[_\x81U`\x01\x01a\x03\x9EV[\x90\x91P\x81\x90a\x03\x95V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[4a\x01\x11W` 6`\x03\x19\x01\x12a\x01\x11W`\x045`\x01`\x01`@\x1B\x03\x81\x16\x80\x91\x03a\x01\x11W_R_` R`@_ a\x04\x18`\x01\x80\x80`\xA0\x1B\x03\x83T\x16\x92\x01a\x05dV[\x90a\x01\r`@Q\x92\x83\x92\x83R`@` \x84\x01R`@\x83\x01\x90a\x06\x04V[4a\x01\x11W`@6`\x03\x19\x01\x12a\x01\x11W`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x01\x11W`$5\x90_R`\x01` R`@_ \x90\x81T\x81\x10\x15a\x01\x11Wa\x04\x88`\x01`\x01`@\x1B\x03\x91` \x93a\x04\xBBV[\x90T\x90`\x03\x1B\x1C\x16`@Q\x90\x81R\xF3[4a\x01\x11W_6`\x03\x19\x01\x12a\x01\x11W` \x90`\x01`\x01`@\x1B\x03`\x02T\x16\x81R\xF3[\x91\x90\x91\x80T\x83\x10\x15a\x04\xDCW_R`\x18` _ \x83`\x02\x1C\x01\x92`\x03\x1B\x16\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x05\x1EW[` \x83\x10\x14a\x05\nWV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a\x04\xFFV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02\xFBW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x02\xFBW`@RV[\x90`@Q\x91\x82_\x82T\x92a\x05w\x84a\x04\xF0V[\x80\x84R\x93`\x01\x81\x16\x90\x81\x15a\x05\xE2WP`\x01\x14a\x05\x9EW[Pa\x05\x9C\x92P\x03\x83a\x05CV[V[\x90P_\x92\x91\x92R` _ \x90_\x91[\x81\x83\x10a\x05\xC6WPP\x90` a\x05\x9C\x92\x82\x01\x01_a\x05\x8FV[` \x91\x93P\x80`\x01\x91T\x83\x85\x89\x01\x01R\x01\x91\x01\x90\x91\x84\x92a\x05\xADV[\x90P` \x92Pa\x05\x9C\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01_a\x05\x8FV[\x91\x90\x82Q\x92\x83\x82R_[\x84\x81\x10a\x06.WPP\x82_` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x80` \x80\x92\x84\x01\x01Q\x82\x82\x86\x01\x01R\x01a\x06\x0EV\xFE\xA2dipfsX\"\x12 <\xD4?\x04\x19#A}&\x7F`uX&\x08]\xA2\xFB\xE0\xCEq\xB9\xDE\x80K\x84\x11\x8EO\x99\xE1\xECdsolcC\0\x08\x1C\x003",
    );
    /**Event with signature `NewTrigger(bytes)` and selector `0x86eacd23610d81706516de1ed0476c87772fdf939c7c771fbbd7f0230d619e68`.
    ```solidity
    event NewTrigger(bytes);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct NewTrigger {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for NewTrigger {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "NewTrigger(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    134u8, 234u8, 205u8, 35u8, 97u8, 13u8, 129u8, 112u8, 101u8, 22u8, 222u8, 30u8,
                    208u8, 71u8, 108u8, 135u8, 119u8, 47u8, 223u8, 147u8, 156u8, 124u8, 119u8,
                    31u8, 187u8, 215u8, 240u8, 35u8, 13u8, 97u8, 158u8, 104u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                    &self._0,
                ),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewTrigger {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewTrigger> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewTrigger) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `addTrigger(bytes)` and selector `0xe31e0788`.
    ```solidity
    function addTrigger(bytes memory data) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addTriggerCall {
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`addTrigger(bytes)`](addTriggerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addTriggerReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
            impl ::core::convert::From<addTriggerCall> for UnderlyingRustTuple<'_> {
                fn from(value: addTriggerCall) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addTriggerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { data: tuple.0 }
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
            impl ::core::convert::From<addTriggerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addTriggerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addTriggerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addTriggerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = addTriggerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addTrigger(bytes)";
            const SELECTOR: [u8; 4] = [227u8, 30u8, 7u8, 136u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                    &self.data,
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
    /**Function with signature `getTrigger(uint64)` and selector `0xe328ed77`.
    ```solidity
    function getTrigger(ISimpleTrigger.TriggerId triggerId) external view returns (ISimpleTrigger.TriggerInfo memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTriggerCall {
        pub triggerId: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getTrigger(uint64)`](getTriggerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTriggerReturn {
        pub _0: <ISimpleTrigger::TriggerInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getTriggerCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTriggerCall) -> Self {
                    (value.triggerId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTriggerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { triggerId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISimpleTrigger::TriggerInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<ISimpleTrigger::TriggerInfo as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<getTriggerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTriggerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTriggerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTriggerCall {
            type Parameters<'a> = (ISimpleTrigger::TriggerId,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTriggerReturn;
            type ReturnTuple<'a> = (ISimpleTrigger::TriggerInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTrigger(uint64)";
            const SELECTOR: [u8; 4] = [227u8, 40u8, 237u8, 119u8];
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
    /**Function with signature `nextTriggerId()` and selector `0x42227fa4`.
    ```solidity
    function nextTriggerId() external view returns (ISimpleTrigger.TriggerId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextTriggerIdCall {}
    ///Container type for the return parameters of the [`nextTriggerId()`](nextTriggerIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextTriggerIdReturn {
        pub _0: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<nextTriggerIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: nextTriggerIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextTriggerIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<nextTriggerIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nextTriggerIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextTriggerIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nextTriggerIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = nextTriggerIdReturn;
            type ReturnTuple<'a> = (ISimpleTrigger::TriggerId,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nextTriggerId()";
            const SELECTOR: [u8; 4] = [66u8, 34u8, 127u8, 164u8];
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
    /**Function with signature `triggerIdsByCreator(address,uint256)` and selector `0x913b1fbf`.
    ```solidity
    function triggerIdsByCreator(address, uint256) external view returns (ISimpleTrigger.TriggerId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct triggerIdsByCreatorCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`triggerIdsByCreator(address,uint256)`](triggerIdsByCreatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct triggerIdsByCreatorReturn {
        pub _0: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
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
            impl ::core::convert::From<triggerIdsByCreatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: triggerIdsByCreatorCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for triggerIdsByCreatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
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
            impl ::core::convert::From<triggerIdsByCreatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: triggerIdsByCreatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for triggerIdsByCreatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for triggerIdsByCreatorCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = triggerIdsByCreatorReturn;
            type ReturnTuple<'a> = (ISimpleTrigger::TriggerId,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "triggerIdsByCreator(address,uint256)";
            const SELECTOR: [u8; 4] = [145u8, 59u8, 31u8, 191u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
    /**Function with signature `triggersById(uint64)` and selector `0xce289612`.
    ```solidity
    function triggersById(ISimpleTrigger.TriggerId) external view returns (address creator, bytes memory data);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct triggersByIdCall {
        pub _0: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`triggersById(uint64)`](triggersByIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct triggersByIdReturn {
        pub creator: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<triggersByIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: triggersByIdCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for triggersByIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Bytes);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Address, alloy::sol_types::private::Bytes);
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
            impl ::core::convert::From<triggersByIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: triggersByIdReturn) -> Self {
                    (value.creator, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for triggersByIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { creator: tuple.0, data: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for triggersByIdCall {
            type Parameters<'a> = (ISimpleTrigger::TriggerId,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = triggersByIdReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Bytes);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "triggersById(uint64)";
            const SELECTOR: [u8; 4] = [206u8, 40u8, 150u8, 18u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<ISimpleTrigger::TriggerId as alloy_sol_types::SolType>::tokenize(&self._0),)
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
    ///Container for all the [`SimpleTrigger`](self) function calls.
    pub enum SimpleTriggerCalls {
        addTrigger(addTriggerCall),
        getTrigger(getTriggerCall),
        nextTriggerId(nextTriggerIdCall),
        triggerIdsByCreator(triggerIdsByCreatorCall),
        triggersById(triggersByIdCall),
    }
    #[automatically_derived]
    impl SimpleTriggerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [66u8, 34u8, 127u8, 164u8],
            [145u8, 59u8, 31u8, 191u8],
            [206u8, 40u8, 150u8, 18u8],
            [227u8, 30u8, 7u8, 136u8],
            [227u8, 40u8, 237u8, 119u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SimpleTriggerCalls {
        const NAME: &'static str = "SimpleTriggerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 5usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::addTrigger(_) => <addTriggerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getTrigger(_) => <getTriggerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::nextTriggerId(_) => <nextTriggerIdCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::triggerIdsByCreator(_) => {
                    <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::triggersById(_) => <triggersByIdCall as alloy_sol_types::SolCall>::SELECTOR,
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
                -> alloy_sol_types::Result<SimpleTriggerCalls>] = &[
                {
                    fn nextTriggerId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SimpleTriggerCalls> {
                        <nextTriggerIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SimpleTriggerCalls::nextTriggerId)
                    }
                    nextTriggerId
                },
                {
                    fn triggerIdsByCreator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SimpleTriggerCalls> {
                        <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SimpleTriggerCalls::triggerIdsByCreator)
                    }
                    triggerIdsByCreator
                },
                {
                    fn triggersById(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SimpleTriggerCalls> {
                        <triggersByIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SimpleTriggerCalls::triggersById)
                    }
                    triggersById
                },
                {
                    fn addTrigger(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SimpleTriggerCalls> {
                        <addTriggerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SimpleTriggerCalls::addTrigger)
                    }
                    addTrigger
                },
                {
                    fn getTrigger(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SimpleTriggerCalls> {
                        <getTriggerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SimpleTriggerCalls::getTrigger)
                    }
                    getTrigger
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
                Self::addTrigger(inner) => {
                    <addTriggerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getTrigger(inner) => {
                    <getTriggerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::nextTriggerId(inner) => {
                    <nextTriggerIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::triggerIdsByCreator(inner) => {
                    <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::triggersById(inner) => {
                    <triggersByIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::addTrigger(inner) => {
                    <addTriggerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getTrigger(inner) => {
                    <getTriggerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::nextTriggerId(inner) => {
                    <nextTriggerIdCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::triggerIdsByCreator(inner) => {
                    <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::triggersById(inner) => {
                    <triggersByIdCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`SimpleTrigger`](self) events.
    pub enum SimpleTriggerEvents {
        NewTrigger(NewTrigger),
    }
    #[automatically_derived]
    impl SimpleTriggerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[[
            134u8, 234u8, 205u8, 35u8, 97u8, 13u8, 129u8, 112u8, 101u8, 22u8, 222u8, 30u8, 208u8,
            71u8, 108u8, 135u8, 119u8, 47u8, 223u8, 147u8, 156u8, 124u8, 119u8, 31u8, 187u8, 215u8,
            240u8, 35u8, 13u8, 97u8, 158u8, 104u8,
        ]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for SimpleTriggerEvents {
        const NAME: &'static str = "SimpleTriggerEvents";
        const COUNT: usize = 1usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<NewTrigger as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewTrigger as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::NewTrigger)
                }
                _ => alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                    name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                    log: alloy_sol_types::private::Box::new(
                        alloy_sol_types::private::LogData::new_unchecked(
                            topics.to_vec(),
                            data.to_vec().into(),
                        ),
                    ),
                }),
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for SimpleTriggerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::NewTrigger(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::NewTrigger(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SimpleTrigger`](self) contract instance.

    See the [wrapper's documentation](`SimpleTriggerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SimpleTriggerInstance<T, P, N> {
        SimpleTriggerInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<SimpleTriggerInstance<T, P, N>>>
    {
        SimpleTriggerInstance::<T, P, N>::deploy(provider)
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
        SimpleTriggerInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`SimpleTrigger`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`SimpleTrigger`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SimpleTriggerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SimpleTriggerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SimpleTriggerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > SimpleTriggerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`SimpleTrigger`](self) contract instance.

        See the [wrapper's documentation](`SimpleTriggerInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self { address, provider, _network_transport: ::core::marker::PhantomData }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

        Returns a new instance of the contract, if the deployment was successful.

        For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(provider: P) -> alloy_contract::Result<SimpleTriggerInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> SimpleTriggerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SimpleTriggerInstance<T, P, N> {
            SimpleTriggerInstance {
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
        > SimpleTriggerInstance<T, P, N>
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
        ///Creates a new call builder for the [`addTrigger`] function.
        pub fn addTrigger(
            &self,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, addTriggerCall, N> {
            self.call_builder(&addTriggerCall { data })
        }
        ///Creates a new call builder for the [`getTrigger`] function.
        pub fn getTrigger(
            &self,
            triggerId: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTriggerCall, N> {
            self.call_builder(&getTriggerCall { triggerId })
        }
        ///Creates a new call builder for the [`nextTriggerId`] function.
        pub fn nextTriggerId(&self) -> alloy_contract::SolCallBuilder<T, &P, nextTriggerIdCall, N> {
            self.call_builder(&nextTriggerIdCall {})
        }
        ///Creates a new call builder for the [`triggerIdsByCreator`] function.
        pub fn triggerIdsByCreator(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, triggerIdsByCreatorCall, N> {
            self.call_builder(&triggerIdsByCreatorCall { _0, _1 })
        }
        ///Creates a new call builder for the [`triggersById`] function.
        pub fn triggersById(
            &self,
            _0: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, triggersByIdCall, N> {
            self.call_builder(&triggersByIdCall { _0 })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > SimpleTriggerInstance<T, P, N>
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
        ///Creates a new event filter for the [`NewTrigger`] event.
        pub fn NewTrigger_filter(&self) -> alloy_contract::Event<T, &P, NewTrigger, N> {
            self.event_filter::<NewTrigger>()
        }
    }
}
