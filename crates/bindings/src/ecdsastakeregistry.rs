///Module containing a contract's types and functions.
/**

```solidity
library ISignatureUtils {
    struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISignatureUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureWithSaltAndExpiry {
        pub signature: alloy::sol_types::private::Bytes,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignatureWithSaltAndExpiry>
        for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithSaltAndExpiry) -> Self {
                (value.signature, value.salt, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SignatureWithSaltAndExpiry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signature: tuple.0,
                    salt: tuple.1,
                    expiry: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignatureWithSaltAndExpiry {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self>
        for SignatureWithSaltAndExpiry {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SignatureWithSaltAndExpiry {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SignatureWithSaltAndExpiry {
            const NAME: &'static str = "SignatureWithSaltAndExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithSaltAndExpiry(bytes signature,bytes32 salt,uint256 expiry)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.salt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.expiry)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignatureWithSaltAndExpiry {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.salt)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expiry,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.salt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.expiry,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISignatureUtilsInstance<T, P, N> {
        ISignatureUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISignatureUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ISignatureUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISignatureUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISignatureUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISignatureUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISignatureUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
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
    impl<T, P: ::core::clone::Clone, N> ISignatureUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISignatureUtilsInstance<T, P, N> {
            ISignatureUtilsInstance {
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
    > ISignatureUtilsInstance<T, P, N> {
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
    > ISignatureUtilsInstance<T, P, N> {
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
library ISignatureUtils {
    struct SignatureWithSaltAndExpiry {
        bytes signature;
        bytes32 salt;
        uint256 expiry;
    }
}

interface ECDSAStakeRegistry {
    struct Quorum {
        StrategyParams[] strategies;
    }
    struct StrategyParams {
        address strategy;
        uint96 multiplier;
    }

    error InsufficientSignedStake();
    error InsufficientWeight();
    error InvalidLength();
    error InvalidQuorum();
    error InvalidReferenceBlock();
    error InvalidSignature();
    error InvalidSignedWeight();
    error InvalidThreshold();
    error LengthMismatch();
    error MustUpdateAllOperators();
    error NotSorted();
    error OperatorAlreadyRegistered();
    error OperatorNotRegistered();

    event Initialized(uint8 version);
    event MinimumWeightUpdated(uint256 _old, uint256 _new);
    event OperatorDeregistered(address indexed _operator, address indexed _avs);
    event OperatorRegistered(address indexed _operator, address indexed _avs);
    event OperatorWeightUpdated(address indexed _operator, uint256 oldWeight, uint256 newWeight);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event QuorumUpdated(Quorum _old, Quorum _new);
    event SigningKeyUpdate(address indexed operator, uint256 indexed updateBlock, address indexed newSigningKey, address oldSigningKey);
    event ThresholdWeightUpdated(uint256 _thresholdWeight);
    event TotalWeightUpdated(uint256 oldTotalWeight, uint256 newTotalWeight);
    event UpdateMinimumWeight(uint256 oldMinimumWeight, uint256 newMinimumWeight);

    constructor(address _delegationManager);

    function deregisterOperator() external;
    function getLastCheckpointOperatorWeight(address _operator) external view returns (uint256);
    function getLastCheckpointThresholdWeight() external view returns (uint256);
    function getLastCheckpointThresholdWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
    function getLastCheckpointTotalWeight() external view returns (uint256);
    function getLastCheckpointTotalWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
    function getLastestOperatorSigningKey(address _operator) external view returns (address);
    function getOperatorSigningKeyAtBlock(address _operator, uint256 _blockNumber) external view returns (address);
    function getOperatorWeight(address _operator) external view returns (uint256);
    function getOperatorWeightAtBlock(address _operator, uint32 _blockNumber) external view returns (uint256);
    function initialize(address _serviceManager, uint256 _thresholdWeight, Quorum memory _quorum) external;
    function isValidSignature(bytes32 _dataHash, bytes memory _signatureData) external view returns (bytes4);
    function minimumWeight() external view returns (uint256);
    function operatorRegistered(address _operator) external view returns (bool);
    function owner() external view returns (address);
    function quorum() external view returns (Quorum memory);
    function registerOperatorWithSignature(ISignatureUtils.SignatureWithSaltAndExpiry memory _operatorSignature, address _signingKey) external;
    function renounceOwnership() external;
    function transferOwnership(address newOwner) external;
    function updateMinimumWeight(uint256 _newMinimumWeight, address[] memory _operators) external;
    function updateOperatorSigningKey(address _newSigningKey) external;
    function updateOperators(address[] memory _operators) external;
    function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory) external;
    function updateQuorumConfig(Quorum memory _quorum, address[] memory _operators) external;
    function updateStakeThreshold(uint256 _thresholdWeight) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_delegationManager",
        "type": "address",
        "internalType": "contract IDelegationManager"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperator",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getLastCheckpointOperatorWeight",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointThresholdWeight",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointThresholdWeightAtBlock",
    "inputs": [
      {
        "name": "_blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointTotalWeight",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointTotalWeightAtBlock",
    "inputs": [
      {
        "name": "_blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastestOperatorSigningKey",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      }
    ],
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
    "name": "getOperatorSigningKeyAtBlock",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_blockNumber",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "name": "getOperatorWeight",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorWeightAtBlock",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_serviceManager",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_thresholdWeight",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_quorum",
        "type": "tuple",
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isValidSignature",
    "inputs": [
      {
        "name": "_dataHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_signatureData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "minimumWeight",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operatorRegistered",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
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
    "name": "owner",
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
    "name": "quorum",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "registerOperatorWithSignature",
    "inputs": [
      {
        "name": "_operatorSignature",
        "type": "tuple",
        "internalType": "struct ISignatureUtils.SignatureWithSaltAndExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "expiry",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "_signingKey",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateMinimumWeight",
    "inputs": [
      {
        "name": "_newMinimumWeight",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperatorSigningKey",
    "inputs": [
      {
        "name": "_newSigningKey",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperators",
    "inputs": [
      {
        "name": "_operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperatorsForQuorum",
    "inputs": [
      {
        "name": "operatorsPerQuorum",
        "type": "address[][]",
        "internalType": "address[][]"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateQuorumConfig",
    "inputs": [
      {
        "name": "_quorum",
        "type": "tuple",
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      },
      {
        "name": "_operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateStakeThreshold",
    "inputs": [
      {
        "name": "_thresholdWeight",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MinimumWeightUpdated",
    "inputs": [
      {
        "name": "_old",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "_new",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorDeregistered",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "_avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorRegistered",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "_avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorWeightUpdated",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "oldWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "QuorumUpdated",
    "inputs": [
      {
        "name": "_old",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      },
      {
        "name": "_new",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SigningKeyUpdate",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "updateBlock",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "newSigningKey",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "oldSigningKey",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ThresholdWeightUpdated",
    "inputs": [
      {
        "name": "_thresholdWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TotalWeightUpdated",
    "inputs": [
      {
        "name": "oldTotalWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newTotalWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UpdateMinimumWeight",
    "inputs": [
      {
        "name": "oldMinimumWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newMinimumWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "InsufficientSignedStake",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientWeight",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidQuorum",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidReferenceBlock",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignedWeight",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidThreshold",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MustUpdateAllOperators",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotSorted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorAlreadyRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorNotRegistered",
    "inputs": []
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
pub mod ECDSAStakeRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a034606657601f61242138819003918201601f19168301916001600160401b03831184841017606a57808492602094604052833981010312606657516001600160a01b03811681036066576080526040516123a2908161007f82396080518161135e0152f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f803560e01c8062cf2ab514610f0c5780630dba339414610ee35780631626ba7e14610bd75780631703a01814610b875780631e4cd85e14610b5e578063314f3a4914610b445780633b242e4a14610b0d5780633d5611f61461092457806340bf2fb7146109065780635140a548146107fa5780635e1042e8146107b45780635ef533291461078f578063696255be14610714578063715018a6146106b7578063743c31f41461066f578063857dc1901461055a5780638da5cb5b14610531578063955f2d90146104e457806398ec1ac9146104b8578063ab11899514610323578063b933fa74146102bb578063cdcd358114610270578063dec5d1f6146101fe578063ec7fbb31146101bf5763f2fde38b1461012c575f80fd5b346101bc5760203660031901126101bc57610145610faf565b61014d611785565b6001600160a01b038116156101685761016590611995565b80f35b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b80fd5b50346101bc5760203660031901126101bc5760209060ff906040906001600160a01b036101ea610faf565b168152606e84522054166040519015158152f35b50346101bc5760403660031901126101bc576004356001600160401b03811161026c5761022f903690600401611100565b602435906001600160401b03821161026857610263610255610165933690600401610fc5565b9161025e611785565b611ac2565b6114d7565b8280fd5b5080fd5b50346101bc5760203660031901126101bc576020906001600160a01b03610295610faf565b168152606a8252604090206001600160a01b03906102b290611752565b16604051908152f35b50346101bc57806003193601126101bc57606c5490816102ed57602091505b6040516001600160e01b03919091168152f35b5f19820191821161030f5750610306602091606c611526565b5054811c6102da565b634e487b7160e01b81526011600452602490fd5b50346101bc5760603660031901126101bc5761033d610faf565b6044356001600160401b0381116102685761035c903690600401611100565b82549060ff8260081c1615928380946104ab575b8015610494575b156104385760ff19831660011785556103c79284610427575b506103a160ff865460081c16611a62565b60018060a01b03166001600160601b0360a01b606854161760685561025e6024356117dd565b6103e060ff835460081c166103db81611a62565b611a62565b6103e933611995565b6103f05780f35b61ff001981541681557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160018152a180f35b61ffff19166101011785555f610390565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b50303b1580156103775750600160ff841614610377565b50600160ff841610610370565b50346101bc5760203660031901126101bc5760206104dc6104d7610faf565b61129e565b604051908152f35b50346101bc5760403660031901126101bc576104fe610faf565b6024359063ffffffff8216809203610268576001600160a01b03168252606d602090815260409092206104dc91906116b5565b50346101bc57806003193601126101bc576033546040516001600160a01b039091168152602090f35b50346101bc57806003193601126101bc57338152606e60205260ff6040822054161561066057606554801561064c575f1901606555338152606e6020526040812060ff1981541690556105b46105af33611c27565b611e24565b505060685481906001600160a01b0316803b15610649578180916024604051809481936351b27a6d60e11b83523360048401525af1801561063e57610629575b506068546001600160a01b0316337f31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed5808380a380f35b8161063391610f77565b6101bc57805f6105f4565b6040513d84823e3d90fd5b50fd5b634e487b7160e01b82526011600452602482fd5b6325ec6c1f60e01b8152600490fd5b50346101bc5760203660031901126101bc57610689610faf565b338252606e60205260ff604083205416156106a85761016590336119dd565b6325ec6c1f60e01b8252600482fd5b50346101bc57806003193601126101bc576106d0611785565b603380546001600160a01b0319811690915581906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b50346101bc5760403660031901126101bc576024356004356001600160401b038211610268577f713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f604061076e610165943690600401610fc5565b92610777611785565b606754908060675582519182526020820152a16114d7565b50346101bc5760203660031901126101bc576107a9611785565b6101656004356117dd565b50346101bc5760403660031901126101bc576020906001600160a01b036107d9610faf565b168152606a8252604090206001600160a01b03906102b290602435906116b5565b50346101bc5760403660031901126101bc576004356001600160401b03811161026c573660238201121561026c5780600401359061083782610f98565b906108456040519283610f77565b828252602082019260051b8101602401833682116109025760248301905b8282106108cf57505050506024356001600160401b0381116108cb5761088d903690600401611061565b5051156108b757518051606554036108a857610165906114d7565b63169efb5b60e11b8252600482fd5b634e487b7160e01b82526032600452602482fd5b8380fd5b81356001600160401b0381116108fe576020916108f3839260243691890101610fc5565b815201910190610863565b8780fd5b8580fd5b50346101bc57806003193601126101bc576020606754604051908152f35b5034610ad2576040366003190112610ad2576004356001600160401b038111610ad25760606003198236030112610ad257604051606081018181106001600160401b03821117610af95760405281600401356001600160401b038111610ad2576109949060043691850101611061565b8152602480830135602083019081526044909301356040830190815290356001600160a01b0381168103610ad257335f52606e60205260ff60405f205416610aea57606554905f198214610ad6576001610a179201606555335f52606e60205260405f20600160ff19825416179055610a0f6105af33611c27565b5050336119dd565b6068546001600160a01b0316803b15610ad257610a635f809460405196879586948593639926ee7d60e01b855233600486015260406024860152516060604486015260a4850190611760565b9151606484015251608483015203925af18015610ac757610ab4575b506068546001600160a01b0316337fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c18380a380f35b610ac091505f90610f77565b5f80610a7f565b6040513d5f823e3d90fd5b5f80fd5b634e487b7160e01b5f52601160045260245ffd5b6342ee68b560e01b5f5260045ffd5b634e487b7160e01b5f52604160045260245ffd5b34610ad2576020366003190112610ad2576001600160a01b03610b2e610faf565b165f52606d60205260206104dc60405f20611752565b34610ad2575f366003190112610ad25760206104dc611745565b34610ad2576020366003190112610ad25760206104dc63ffffffff610b81611033565b16611624565b34610ad2575f366003190112610ad2576060604051610ba581610f41565b52610bd3604051610bb581610f41565b610bbd6111f2565b81526040519182916020835260208301906110a7565b0390f35b34610ad2576040366003190112610ad2576004356024356001600160401b038111610ad257610c0a903690600401611061565b8051810160608260208301920312610ad25760208201516001600160401b038111610ad25782019181603f84011215610ad2576020830151610c4b81610f98565b93610c596040519586610f77565b8185526020808087019360051b8301010190848211610ad257604001915b818310610ec35750505060408101516001600160401b038111610ad257810182603f82011215610ad257602081015190610cb082610f98565b93610cbe6040519586610f77565b8285526020808087019460051b8401010191818311610ad25760408101935b838510610e565788888860608901519163ffffffff8316809303610ad2578051915f809582518503610e47578415610e385743861015945f949093925b848610610d8457888888610d7557610d318161153b565b8211610d6657610d4090611624565b11610d5757604051630b135d3f60e11b8152602090f35b63e121632f60e01b5f5260045ffd5b634b05a0f760e11b5f5260045ffd5b63e64f180f60e01b5f5260045ffd5b949793949293926001600160a01b03610d9d8a84611269565b511696610d7557865f52606a6020528660018060a01b03610dc18a60405f206116b5565b16916001600160a01b03161015610e2957610de89083610de18b88611269565b51916120b5565b15610e1a576001610e1087945f985f52606d602052610e0a8a60405f206116b5565b90611291565b9801949392610d1a565b638baa579f60e01b5f5260045ffd5b63ba50f91160e01b5f5260045ffd5b63251f56a160e21b5f5260045ffd5b631fec674760e31b5f5260045ffd5b84516001600160401b038111610ad2576020908301019083603f83011215610ad257602082015190610e8782611046565b610e946040519182610f77565b8281526040848401018610610ad257610eb8602094938594604086850191016111d1565b815201940193610cdd565b82516001600160a01b0381168103610ad257815260209283019201610c77565b34610ad2576020366003190112610ad25760206104dc63ffffffff610f06611033565b1661153b565b34610ad2576020366003190112610ad2576004356001600160401b038111610ad257610263610f3f913690600401610fc5565b005b602081019081106001600160401b03821117610af957604052565b604081019081106001600160401b03821117610af957604052565b90601f801991011681019081106001600160401b03821117610af957604052565b6001600160401b038111610af95760051b60200190565b600435906001600160a01b0382168203610ad257565b9080601f83011215610ad257813590610fdd82610f98565b92610feb6040519485610f77565b82845260208085019360051b820101918211610ad257602001915b8183106110135750505090565b82356001600160a01b0381168103610ad257815260209283019201611006565b6004359063ffffffff82168203610ad257565b6001600160401b038111610af957601f01601f191660200190565b81601f82011215610ad25780359061107882611046565b926110866040519485610f77565b82845260208383010111610ad257815f926020809301838601378301015290565b602060408184019251938281528451809452019201905f5b8181106110cc5750505090565b825180516001600160a01b031685526020908101516001600160601b031681860152604090940193909201916001016110bf565b9190602083820312610ad25760405161111881610f41565b80938035906001600160401b038211610ad2570182601f82011215610ad25780359061114382610f98565b936111516040519586610f77565b82855260208086019360061b83010191818311610ad257602001925b82841061117b575050505052565b604084830312610ad2576040519061119282610f5c565b84356001600160a01b0381168103610ad25782526020850135906001600160601b0382168203610ad2578260209283604095015281520193019261116d565b5f5b8381106111e25750505f910152565b81810151838201526020016111d3565b606654906111ff82610f98565b9161120d6040519384610f77565b80835260665f9081525f8051602061234d833981519152602085015b8383106112365750505050565b60016020819260405161124881610f5c565b8554848060a01b038116825260a01c83820152815201920192019190611229565b805182101561127d5760209160051b010190565b634e487b7160e01b5f52603260045260245ffd5b91908201809211610ad657565b6112a66111f2565b805190925f6112cd6112b784610f98565b936112c56040519586610f77565b808552610f98565b6020840190601f19013682375f5b865181101561130d576001906001600160a01b036112f9828a611269565b5151166113068288611269565b52016112db565b5060408051639004134760e01b81526001600160a01b0390961660048701526024860152925160448501819052919390929091829160648301915f5b81811061149a57505f939283900391508290507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610ac7575f9161140a575b505f915b84518310156113ec576113ad8383611269565b51906001600160601b0360206113c38689611269565b5101511691828102928184041490151715610ad6576001916113e491611291565b92019161139a565b915050612710919250046067548110155f146114055790565b505f90565b90503d805f833e61141b8183610f77565b810190602081830312610ad2578051906001600160401b038211610ad257019080601f83011215610ad257815161145181610f98565b9261145f6040519485610f77565b81845260208085019260051b820101928311610ad257602001905b82821061148a575050505f611396565b815181526020918201910161147a565b82516001600160a01b0316845285945060209384019390920191600101611349565b9190915f8382019384129112908015821691151617610ad657565b905f805b83518210156115155760019061150d906115076001600160a01b036115008689611269565b5116611c27565b906114bc565b9101906114db565b9050611522919250611e24565b5050565b805482101561127d575f5260205f2001905f90565b438110156115e057606b54905f905b8282106115875750508061156757505f5b6001600160e01b031690565b5f198101908111610ad65761157d90606b611526565b505460201c61155b565b909161159a81841860011c828516611291565b908263ffffffff6115ac84606b611526565b50541611156115be5750915b9061154a565b925060018101809111156115b857634e487b7160e01b5f52601160045260245ffd5b606460405162461bcd60e51b815260206004820152602060248201527f436865636b706f696e74733a20626c6f636b206e6f7420796574206d696e65646044820152fd5b438110156115e057606c54905f905b82821061165c5750508061164657505f90565b5f198101908111610ad65761157d90606c611526565b909161166f81841860011c828516611291565b908263ffffffff61168184606c611526565b50541611156116935750915b90611633565b9250600181018091111561168d57634e487b7160e01b5f52601160045260245ffd5b90438110156115e0578154905f905b8282106116ed575050806116d957505f919050565b5f198101908111610ad65761157d91611526565b909161170081841860011c828516611291565b908263ffffffff6117118488611526565b50541611156117235750915b906116c4565b9250600181018091111561171d57634e487b7160e01b5f52601160045260245ffd5b606b548061156757505f90565b8054806116d957505f919050565b90602091611779815180928185528580860191016111d1565b601f01601f1916010190565b6033546001600160a01b0316330361179957565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b606c54906117eb606c611752565b908215158061196f575b1561186357611803816121cd565b915f198401938411610ad6576118596020936118417f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b96606c611526565b509063ffffffff82549181199060201b169116179055565b50604051908152a1565b63ffffffff431161191b57611877816121cd565b916040519361188585610f5c565b4363ffffffff1685526001600160e01b039093166020850190815292600160401b811015610af9578060016118bf9201606c55606c611526565b9490946119085751925163ffffffff90931663ffffffff19602094851b16179093557f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b92611859565b634e487b7160e01b5f525f60045260245ffd5b60405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b6064820152608490fd5b505f198301838111610ad65761198b63ffffffff91606c611526565b50541643146117f5565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b6001600160a01b039081165f818152606a6020526040902090929190611a0290611752565b6001600160a01b0390921692911690828214611a5d57805f52606a602052611a2d8360405f20611f52565b50506040519182527fd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea131500260204393a4565b505050565b15611a6957565b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b611acb8161203b565b15611c005760405191611add83610f41565b611ae56111f2565b83526066545f60665580611bc6575b505f5b82518051821015611b735781611b0c91611269565b519060665491600160401b831015610af9576001830160665560665483101561127d5760665f5280516020919091015160a01b6001600160a01b0319166001600160a01b0391909116175f8051602061234d83398151915290920191909155600101611af7565b505091611bb390611bc17f23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e936040519384936040855260408501906110a7565b9083820360208501526110a7565b0390a1565b60665f525f8051602061234d833981519152015f8051602061234d8339815191525b818110611bf55750611af4565b5f8155600101611be8565b63d173577960e01b5f5260045ffd5b81810392915f138015828513169184121617610ad657565b6001600160a01b0381165f818152606d602052604081209092908390611c4c90611752565b8094845f52606e60205260ff60405f205416155f14611dc657611c70929350611c0f565b928315611dc057825f52606d60205260405f2090815492611c9083611752565b84151580611d9b575b15611cf457611ca75f6121cd565b945f198101908111610ad6577f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe59495611841611ce492604097611526565b505b82519182526020820152a290565b63ffffffff431161191b57611d085f6121cd565b9360405195611d1687610f5c565b4363ffffffff1687526001600160e01b039095166020870190815294600160401b811015610af957611d4d91600182018155611526565b9590956119085751935160201b63ffffffff191663ffffffff94909416939093179093557f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe59492604092611ce4565b505f198501858111610ad657611db663ffffffff9186611526565b5054164314611c99565b50505090565b505050611dd29061129e565b91611ddd8184611c0f565b928315611dc0577f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe59491604091845f52606d602052611e1d81845f20611f52565b5050611ce6565b90611e37611e30611745565b92836114bc565b90606b54611e45606b611752565b9080151580611f2c575b15611ea857611e5d846121cd565b5f198201918211610ad657611841611e7692606b611526565b507f86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b60408051858152846020820152a1565b63ffffffff431161191b57611ebc846121cd565b60405191611ec983610f5c565b4363ffffffff1683526001600160e01b039091166020830190815290600160401b811015610af957806001611f039201606b55606b611526565b919091611908579151915160201b63ffffffff191663ffffffff92909216919091179055611e76565b505f198101818111610ad657611f4863ffffffff91606b611526565b5054164314611e4f565b9190918054611f6082611752565b9181151580612016575b15611f9557611f78856121cd565b905f198301928311610ad657611f919261184191611526565b9190565b63ffffffff431161191b57611fa9856121cd565b9060405192611fb784610f5c565b4363ffffffff1684526001600160e01b039092166020840190815291600160401b811015610af957611fee91600182018155611526565b919091611908579151915160201b63ffffffff191663ffffffff929092169190911790559190565b505f198201828111610ad65761203163ffffffff9183611526565b5054164314611f6a565b51905f80805b845182101561209e576001600160a01b0361205c8387611269565b515116906001600160a01b0316811115610e2957612095600191936001600160601b03602061208b868a611269565b5101511690611291565b91019091612041565b509192506127101490506120b0575f90565b600190565b9190916120c2828461223c565b60058110156121b9571590816121a3575b5061219b575f9261210d61211b85946040519283916020830195630b135d3f60e11b87526024840152604060448401526064830190611760565b03601f198101835282610f77565b51915afa3d15612194573d61212f81611046565b9061213d6040519283610f77565b81523d5f602083013e5b81612188575b81612156575090565b9050602081805181010312610ad257602001516001600160e01b0319811690819003610ad257630b135d3f60e11b1490565b8051602014915061214d565b6060612147565b505050600190565b6001600160a01b0383811691161490505f6120d3565b634e487b7160e01b5f52602160045260245ffd5b6001600160e01b0381116121e7576001600160e01b031690565b60405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b6064820152608490fd5b81516041810361226857509061226491602082015190606060408401519301515f1a906122aa565b9091565b6040036122a15760406020830151920151918260ff1c91601b8301809311610ad657612264936001600160ff1b03169260ff16906122aa565b50505f90600290565b907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116123415760ff1690601b82141580612336575b61232b576020935f93608093604051938452868401526040830152606082015282805260015afa15610ac7575f516001600160a01b0381161561232357905f90565b505f90600190565b505050505f90600490565b50601c8214156122e1565b505050505f9060039056fe46501879b8ca8525e8c2fd519e2fbfcfa2ebea26501294aa02cbfcfb12e94354a2646970667358221220e3c3c7955f19f894dee76d8a1721dfc8696603f5d611cac194baf33d16d06bb364736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA04`fW`\x1Fa$!8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17`jW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12`fWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03`fW`\x80R`@Qa#\xA2\x90\x81a\0\x7F\x829`\x80Q\x81a\x13^\x01R\xF3[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_\x805`\xE0\x1C\x80b\xCF*\xB5\x14a\x0F\x0CW\x80c\r\xBA3\x94\x14a\x0E\xE3W\x80c\x16&\xBA~\x14a\x0B\xD7W\x80c\x17\x03\xA0\x18\x14a\x0B\x87W\x80c\x1EL\xD8^\x14a\x0B^W\x80c1O:I\x14a\x0BDW\x80c;$.J\x14a\x0B\rW\x80c=V\x11\xF6\x14a\t$W\x80c@\xBF/\xB7\x14a\t\x06W\x80cQ@\xA5H\x14a\x07\xFAW\x80c^\x10B\xE8\x14a\x07\xB4W\x80c^\xF53)\x14a\x07\x8FW\x80cibU\xBE\x14a\x07\x14W\x80cqP\x18\xA6\x14a\x06\xB7W\x80ct<1\xF4\x14a\x06oW\x80c\x85}\xC1\x90\x14a\x05ZW\x80c\x8D\xA5\xCB[\x14a\x051W\x80c\x95_-\x90\x14a\x04\xE4W\x80c\x98\xEC\x1A\xC9\x14a\x04\xB8W\x80c\xAB\x11\x89\x95\x14a\x03#W\x80c\xB93\xFAt\x14a\x02\xBBW\x80c\xCD\xCD5\x81\x14a\x02pW\x80c\xDE\xC5\xD1\xF6\x14a\x01\xFEW\x80c\xEC\x7F\xBB1\x14a\x01\xBFWc\xF2\xFD\xE3\x8B\x14a\x01,W_\x80\xFD[4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCWa\x01Ea\x0F\xAFV[a\x01Ma\x17\x85V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x01hWa\x01e\x90a\x19\x95V[\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[\x80\xFD[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCW` \x90`\xFF\x90`@\x90`\x01`\x01`\xA0\x1B\x03a\x01\xEAa\x0F\xAFV[\x16\x81R`n\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lWa\x02/\x906\x90`\x04\x01a\x11\0V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02hWa\x02ca\x02Ua\x01e\x936\x90`\x04\x01a\x0F\xC5V[\x91a\x02^a\x17\x85V[a\x1A\xC2V[a\x14\xD7V[\x82\x80\xFD[P\x80\xFD[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCW` \x90`\x01`\x01`\xA0\x1B\x03a\x02\x95a\x0F\xAFV[\x16\x81R`j\x82R`@\x90 `\x01`\x01`\xA0\x1B\x03\x90a\x02\xB2\x90a\x17RV[\x16`@Q\x90\x81R\xF3[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW`lT\x90\x81a\x02\xEDW` \x91P[`@Q`\x01`\x01`\xE0\x1B\x03\x91\x90\x91\x16\x81R\xF3[_\x19\x82\x01\x91\x82\x11a\x03\x0FWPa\x03\x06` \x91`la\x15&V[PT\x81\x1Ca\x02\xDAV[cNH{q`\xE0\x1B\x81R`\x11`\x04R`$\x90\xFD[P4a\x01\xBCW``6`\x03\x19\x01\x12a\x01\xBCWa\x03=a\x0F\xAFV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x02hWa\x03\\\x906\x90`\x04\x01a\x11\0V[\x82T\x90`\xFF\x82`\x08\x1C\x16\x15\x92\x83\x80\x94a\x04\xABW[\x80\x15a\x04\x94W[\x15a\x048W`\xFF\x19\x83\x16`\x01\x17\x85Ua\x03\xC7\x92\x84a\x04'W[Pa\x03\xA1`\xFF\x86T`\x08\x1C\x16a\x1AbV[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`hT\x16\x17`hUa\x02^`$5a\x17\xDDV[a\x03\xE0`\xFF\x83T`\x08\x1C\x16a\x03\xDB\x81a\x1AbV[a\x1AbV[a\x03\xE93a\x19\x95V[a\x03\xF0W\x80\xF3[a\xFF\0\x19\x81T\x16\x81U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\x01\x81R\xA1\x80\xF3[a\xFF\xFF\x19\x16a\x01\x01\x17\x85U_a\x03\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[P0;\x15\x80\x15a\x03wWP`\x01`\xFF\x84\x16\x14a\x03wV[P`\x01`\xFF\x84\x16\x10a\x03pV[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCW` a\x04\xDCa\x04\xD7a\x0F\xAFV[a\x12\x9EV[`@Q\x90\x81R\xF3[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCWa\x04\xFEa\x0F\xAFV[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x02hW`\x01`\x01`\xA0\x1B\x03\x16\x82R`m` \x90\x81R`@\x90\x92 a\x04\xDC\x91\x90a\x16\xB5V[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW3\x81R`n` R`\xFF`@\x82 T\x16\x15a\x06`W`eT\x80\x15a\x06LW_\x19\x01`eU3\x81R`n` R`@\x81 `\xFF\x19\x81T\x16\x90Ua\x05\xB4a\x05\xAF3a\x1C'V[a\x1E$V[PP`hT\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x06IW\x81\x80\x91`$`@Q\x80\x94\x81\x93cQ\xB2zm`\xE1\x1B\x83R3`\x04\x84\x01RZ\xF1\x80\x15a\x06>Wa\x06)W[P`hT`\x01`\x01`\xA0\x1B\x03\x163\x7F1\xE0\xAD\xFE\xC7\x1B\xCC\xEE7\xB6\xE8:\x90\xC2\xFE\xDB\x17\xD8\xF1i?\xEE\x86<Gq\xE7\xBF\xE2\xAE\xD5\x80\x83\x80\xA3\x80\xF3[\x81a\x063\x91a\x0FwV[a\x01\xBCW\x80_a\x05\xF4V[`@Q=\x84\x82>=\x90\xFD[P\xFD[cNH{q`\xE0\x1B\x82R`\x11`\x04R`$\x82\xFD[c%\xECl\x1F`\xE0\x1B\x81R`\x04\x90\xFD[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCWa\x06\x89a\x0F\xAFV[3\x82R`n` R`\xFF`@\x83 T\x16\x15a\x06\xA8Wa\x01e\x903a\x19\xDDV[c%\xECl\x1F`\xE0\x1B\x82R`\x04\x82\xFD[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCWa\x06\xD0a\x17\x85V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW`$5`\x045`\x01`\x01`@\x1B\x03\x82\x11a\x02hW\x7Fq<\xA5;\x88\xD6\xEBc\xF5\xB1\x85L\xB8\xCB\xDDsn\xC5\x1E\xDA\"^Fy\x1A\xA9)\x8B\x01`d\x8F`@a\x07na\x01e\x946\x90`\x04\x01a\x0F\xC5V[\x92a\x07wa\x17\x85V[`gT\x90\x80`gU\x82Q\x91\x82R` \x82\x01R\xA1a\x14\xD7V[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCWa\x07\xA9a\x17\x85V[a\x01e`\x045a\x17\xDDV[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW` \x90`\x01`\x01`\xA0\x1B\x03a\x07\xD9a\x0F\xAFV[\x16\x81R`j\x82R`@\x90 `\x01`\x01`\xA0\x1B\x03\x90a\x02\xB2\x90`$5\x90a\x16\xB5V[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lW6`#\x82\x01\x12\x15a\x02lW\x80`\x04\x015\x90a\x087\x82a\x0F\x98V[\x90a\x08E`@Q\x92\x83a\x0FwV[\x82\x82R` \x82\x01\x92`\x05\x1B\x81\x01`$\x01\x836\x82\x11a\t\x02W`$\x83\x01\x90[\x82\x82\x10a\x08\xCFWPPPP`$5`\x01`\x01`@\x1B\x03\x81\x11a\x08\xCBWa\x08\x8D\x906\x90`\x04\x01a\x10aV[PQ\x15a\x08\xB7WQ\x80Q`eT\x03a\x08\xA8Wa\x01e\x90a\x14\xD7V[c\x16\x9E\xFB[`\xE1\x1B\x82R`\x04\x82\xFD[cNH{q`\xE0\x1B\x82R`2`\x04R`$\x82\xFD[\x83\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x08\xFEW` \x91a\x08\xF3\x83\x92`$6\x91\x89\x01\x01a\x0F\xC5V[\x81R\x01\x91\x01\x90a\x08cV[\x87\x80\xFD[\x85\x80\xFD[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW` `gT`@Q\x90\x81R\xF3[P4a\n\xD2W`@6`\x03\x19\x01\x12a\n\xD2W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W```\x03\x19\x826\x03\x01\x12a\n\xD2W`@Q``\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@R\x81`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2Wa\t\x94\x90`\x046\x91\x85\x01\x01a\x10aV[\x81R`$\x80\x83\x015` \x83\x01\x90\x81R`D\x90\x93\x015`@\x83\x01\x90\x81R\x905`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W3_R`n` R`\xFF`@_ T\x16a\n\xEAW`eT\x90_\x19\x82\x14a\n\xD6W`\x01a\n\x17\x92\x01`eU3_R`n` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90Ua\n\x0Fa\x05\xAF3a\x1C'V[PP3a\x19\xDDV[`hT`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\n\xD2Wa\nc_\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\x99&\xEE}`\xE0\x1B\x85R3`\x04\x86\x01R`@`$\x86\x01RQ```D\x86\x01R`\xA4\x85\x01\x90a\x17`V[\x91Q`d\x84\x01RQ`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\n\xC7Wa\n\xB4W[P`hT`\x01`\x01`\xA0\x1B\x03\x163\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x83\x80\xA3\x80\xF3[a\n\xC0\x91P_\x90a\x0FwV[_\x80a\n\x7FV[`@Q=_\x82>=\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[cB\xEEh\xB5`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W`\x01`\x01`\xA0\x1B\x03a\x0B.a\x0F\xAFV[\x16_R`m` R` a\x04\xDC`@_ a\x17RV[4a\n\xD2W_6`\x03\x19\x01\x12a\n\xD2W` a\x04\xDCa\x17EV[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W` a\x04\xDCc\xFF\xFF\xFF\xFFa\x0B\x81a\x103V[\x16a\x16$V[4a\n\xD2W_6`\x03\x19\x01\x12a\n\xD2W```@Qa\x0B\xA5\x81a\x0FAV[Ra\x0B\xD3`@Qa\x0B\xB5\x81a\x0FAV[a\x0B\xBDa\x11\xF2V[\x81R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x10\xA7V[\x03\x90\xF3[4a\n\xD2W`@6`\x03\x19\x01\x12a\n\xD2W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2Wa\x0C\n\x906\x90`\x04\x01a\x10aV[\x80Q\x81\x01``\x82` \x83\x01\x92\x03\x12a\n\xD2W` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W\x82\x01\x91\x81`?\x84\x01\x12\x15a\n\xD2W` \x83\x01Qa\x0CK\x81a\x0F\x98V[\x93a\x0CY`@Q\x95\x86a\x0FwV[\x81\x85R` \x80\x80\x87\x01\x93`\x05\x1B\x83\x01\x01\x01\x90\x84\x82\x11a\n\xD2W`@\x01\x91[\x81\x83\x10a\x0E\xC3WPPP`@\x81\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W\x81\x01\x82`?\x82\x01\x12\x15a\n\xD2W` \x81\x01Q\x90a\x0C\xB0\x82a\x0F\x98V[\x93a\x0C\xBE`@Q\x95\x86a\x0FwV[\x82\x85R` \x80\x80\x87\x01\x94`\x05\x1B\x84\x01\x01\x01\x91\x81\x83\x11a\n\xD2W`@\x81\x01\x93[\x83\x85\x10a\x0EVW\x88\x88\x88``\x89\x01Q\x91c\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\n\xD2W\x80Q\x91_\x80\x95\x82Q\x85\x03a\x0EGW\x84\x15a\x0E8WC\x86\x10\x15\x94_\x94\x90\x93\x92[\x84\x86\x10a\r\x84W\x88\x88\x88a\ruWa\r1\x81a\x15;V[\x82\x11a\rfWa\r@\x90a\x16$V[\x11a\rWW`@Qc\x0B\x13]?`\xE1\x1B\x81R` \x90\xF3[c\xE1!c/`\xE0\x1B_R`\x04_\xFD[cK\x05\xA0\xF7`\xE1\x1B_R`\x04_\xFD[c\xE6O\x18\x0F`\xE0\x1B_R`\x04_\xFD[\x94\x97\x93\x94\x92\x93\x92`\x01`\x01`\xA0\x1B\x03a\r\x9D\x8A\x84a\x12iV[Q\x16\x96a\ruW\x86_R`j` R\x86`\x01\x80`\xA0\x1B\x03a\r\xC1\x8A`@_ a\x16\xB5V[\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x10\x15a\x0E)Wa\r\xE8\x90\x83a\r\xE1\x8B\x88a\x12iV[Q\x91a \xB5V[\x15a\x0E\x1AW`\x01a\x0E\x10\x87\x94_\x98_R`m` Ra\x0E\n\x8A`@_ a\x16\xB5V[\x90a\x12\x91V[\x98\x01\x94\x93\x92a\r\x1AV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[c\xBAP\xF9\x11`\xE0\x1B_R`\x04_\xFD[c%\x1FV\xA1`\xE2\x1B_R`\x04_\xFD[c\x1F\xECgG`\xE3\x1B_R`\x04_\xFD[\x84Q`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W` \x90\x83\x01\x01\x90\x83`?\x83\x01\x12\x15a\n\xD2W` \x82\x01Q\x90a\x0E\x87\x82a\x10FV[a\x0E\x94`@Q\x91\x82a\x0FwV[\x82\x81R`@\x84\x84\x01\x01\x86\x10a\n\xD2Wa\x0E\xB8` \x94\x93\x85\x94`@\x86\x85\x01\x91\x01a\x11\xD1V[\x81R\x01\x94\x01\x93a\x0C\xDDV[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W\x81R` \x92\x83\x01\x92\x01a\x0CwV[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W` a\x04\xDCc\xFF\xFF\xFF\xFFa\x0F\x06a\x103V[\x16a\x15;V[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2Wa\x02ca\x0F?\x916\x90`\x04\x01a\x0F\xC5V[\0[` \x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@RV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\n\xF9W`\x05\x1B` \x01\x90V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\xD2WV[\x90\x80`\x1F\x83\x01\x12\x15a\n\xD2W\x815\x90a\x0F\xDD\x82a\x0F\x98V[\x92a\x0F\xEB`@Q\x94\x85a\x0FwV[\x82\x84R` \x80\x85\x01\x93`\x05\x1B\x82\x01\x01\x91\x82\x11a\n\xD2W` \x01\x91[\x81\x83\x10a\x10\x13WPPP\x90V[\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W\x81R` \x92\x83\x01\x92\x01a\x10\x06V[`\x045\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\n\xD2WV[`\x01`\x01`@\x1B\x03\x81\x11a\n\xF9W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x81`\x1F\x82\x01\x12\x15a\n\xD2W\x805\x90a\x10x\x82a\x10FV[\x92a\x10\x86`@Q\x94\x85a\x0FwV[\x82\x84R` \x83\x83\x01\x01\x11a\n\xD2W\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[` `@\x81\x84\x01\x92Q\x93\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x10\xCCWPPP\x90V[\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x86\x01R`@\x90\x94\x01\x93\x90\x92\x01\x91`\x01\x01a\x10\xBFV[\x91\x90` \x83\x82\x03\x12a\n\xD2W`@Qa\x11\x18\x81a\x0FAV[\x80\x93\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\n\xD2W\x01\x82`\x1F\x82\x01\x12\x15a\n\xD2W\x805\x90a\x11C\x82a\x0F\x98V[\x93a\x11Q`@Q\x95\x86a\x0FwV[\x82\x85R` \x80\x86\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\n\xD2W` \x01\x92[\x82\x84\x10a\x11{WPPPPRV[`@\x84\x83\x03\x12a\n\xD2W`@Q\x90a\x11\x92\x82a\x0F\\V[\x845`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W\x82R` \x85\x015\x90`\x01`\x01``\x1B\x03\x82\x16\x82\x03a\n\xD2W\x82` \x92\x83`@\x95\x01R\x81R\x01\x93\x01\x92a\x11mV[_[\x83\x81\x10a\x11\xE2WPP_\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x11\xD3V[`fT\x90a\x11\xFF\x82a\x0F\x98V[\x91a\x12\r`@Q\x93\x84a\x0FwV[\x80\x83R`f_\x90\x81R_\x80Q` a#M\x839\x81Q\x91R` \x85\x01[\x83\x83\x10a\x126WPPPPV[`\x01` \x81\x92`@Qa\x12H\x81a\x0F\\V[\x85T\x84\x80`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1C\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x12)V[\x80Q\x82\x10\x15a\x12}W` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x82\x01\x80\x92\x11a\n\xD6WV[a\x12\xA6a\x11\xF2V[\x80Q\x90\x92_a\x12\xCDa\x12\xB7\x84a\x0F\x98V[\x93a\x12\xC5`@Q\x95\x86a\x0FwV[\x80\x85Ra\x0F\x98V[` \x84\x01\x90`\x1F\x19\x016\x827_[\x86Q\x81\x10\x15a\x13\rW`\x01\x90`\x01`\x01`\xA0\x1B\x03a\x12\xF9\x82\x8Aa\x12iV[QQ\x16a\x13\x06\x82\x88a\x12iV[R\x01a\x12\xDBV[P`@\x80Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16`\x04\x87\x01R`$\x86\x01R\x92Q`D\x85\x01\x81\x90R\x91\x93\x90\x92\x90\x91\x82\x91`d\x83\x01\x91_[\x81\x81\x10a\x14\x9AWP_\x93\x92\x83\x90\x03\x91P\x82\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\n\xC7W_\x91a\x14\nW[P_\x91[\x84Q\x83\x10\x15a\x13\xECWa\x13\xAD\x83\x83a\x12iV[Q\x90`\x01`\x01``\x1B\x03` a\x13\xC3\x86\x89a\x12iV[Q\x01Q\x16\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a\n\xD6W`\x01\x91a\x13\xE4\x91a\x12\x91V[\x92\x01\x91a\x13\x9AV[\x91PPa'\x10\x91\x92P\x04`gT\x81\x10\x15_\x14a\x14\x05W\x90V[P_\x90V[\x90P=\x80_\x83>a\x14\x1B\x81\x83a\x0FwV[\x81\x01\x90` \x81\x83\x03\x12a\n\xD2W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\n\xD2W\x01\x90\x80`\x1F\x83\x01\x12\x15a\n\xD2W\x81Qa\x14Q\x81a\x0F\x98V[\x92a\x14_`@Q\x94\x85a\x0FwV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\n\xD2W` \x01\x90[\x82\x82\x10a\x14\x8AWPPP_a\x13\x96V[\x81Q\x81R` \x91\x82\x01\x91\x01a\x14zV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x13IV[\x91\x90\x91_\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\n\xD6WV[\x90_\x80[\x83Q\x82\x10\x15a\x15\x15W`\x01\x90a\x15\r\x90a\x15\x07`\x01`\x01`\xA0\x1B\x03a\x15\0\x86\x89a\x12iV[Q\x16a\x1C'V[\x90a\x14\xBCV[\x91\x01\x90a\x14\xDBV[\x90Pa\x15\"\x91\x92Pa\x1E$V[PPV[\x80T\x82\x10\x15a\x12}W_R` _ \x01\x90_\x90V[C\x81\x10\x15a\x15\xE0W`kT\x90_\x90[\x82\x82\x10a\x15\x87WPP\x80a\x15gWP_[`\x01`\x01`\xE0\x1B\x03\x16\x90V[_\x19\x81\x01\x90\x81\x11a\n\xD6Wa\x15}\x90`ka\x15&V[PT` \x1Ca\x15[V[\x90\x91a\x15\x9A\x81\x84\x18`\x01\x1C\x82\x85\x16a\x12\x91V[\x90\x82c\xFF\xFF\xFF\xFFa\x15\xAC\x84`ka\x15&V[PT\x16\x11\x15a\x15\xBEWP\x91[\x90a\x15JV[\x92P`\x01\x81\x01\x80\x91\x11\x15a\x15\xB8WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R\xFD[C\x81\x10\x15a\x15\xE0W`lT\x90_\x90[\x82\x82\x10a\x16\\WPP\x80a\x16FWP_\x90V[_\x19\x81\x01\x90\x81\x11a\n\xD6Wa\x15}\x90`la\x15&V[\x90\x91a\x16o\x81\x84\x18`\x01\x1C\x82\x85\x16a\x12\x91V[\x90\x82c\xFF\xFF\xFF\xFFa\x16\x81\x84`la\x15&V[PT\x16\x11\x15a\x16\x93WP\x91[\x90a\x163V[\x92P`\x01\x81\x01\x80\x91\x11\x15a\x16\x8DWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90C\x81\x10\x15a\x15\xE0W\x81T\x90_\x90[\x82\x82\x10a\x16\xEDWPP\x80a\x16\xD9WP_\x91\x90PV[_\x19\x81\x01\x90\x81\x11a\n\xD6Wa\x15}\x91a\x15&V[\x90\x91a\x17\0\x81\x84\x18`\x01\x1C\x82\x85\x16a\x12\x91V[\x90\x82c\xFF\xFF\xFF\xFFa\x17\x11\x84\x88a\x15&V[PT\x16\x11\x15a\x17#WP\x91[\x90a\x16\xC4V[\x92P`\x01\x81\x01\x80\x91\x11\x15a\x17\x1DWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`kT\x80a\x15gWP_\x90V[\x80T\x80a\x16\xD9WP_\x91\x90PV[\x90` \x91a\x17y\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x11\xD1V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17\x99WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`lT\x90a\x17\xEB`la\x17RV[\x90\x82\x15\x15\x80a\x19oW[\x15a\x18cWa\x18\x03\x81a!\xCDV[\x91_\x19\x84\x01\x93\x84\x11a\n\xD6Wa\x18Y` \x93a\x18A\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x96`la\x15&V[P\x90c\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90` \x1B\x16\x91\x16\x17\x90UV[P`@Q\x90\x81R\xA1V[c\xFF\xFF\xFF\xFFC\x11a\x19\x1BWa\x18w\x81a!\xCDV[\x91`@Q\x93a\x18\x85\x85a\x0F\\V[Cc\xFF\xFF\xFF\xFF\x16\x85R`\x01`\x01`\xE0\x1B\x03\x90\x93\x16` \x85\x01\x90\x81R\x92`\x01`@\x1B\x81\x10\x15a\n\xF9W\x80`\x01a\x18\xBF\x92\x01`lU`la\x15&V[\x94\x90\x94a\x19\x08WQ\x92Qc\xFF\xFF\xFF\xFF\x90\x93\x16c\xFF\xFF\xFF\xFF\x19` \x94\x85\x1B\x16\x17\x90\x93U\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x92a\x18YV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[P_\x19\x83\x01\x83\x81\x11a\n\xD6Wa\x19\x8Bc\xFF\xFF\xFF\xFF\x91`la\x15&V[PT\x16C\x14a\x17\xF5V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`j` R`@\x90 \x90\x92\x91\x90a\x1A\x02\x90a\x17RV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x91\x16\x90\x82\x82\x14a\x1A]W\x80_R`j` Ra\x1A-\x83`@_ a\x1FRV[PP`@Q\x91\x82R\x7F\xD0a\x16\x82R\xF4As6X\xF0\x9EM\x8F[-\x99\x8E\xD4\xEF$\xA2\xBB\xFDl\xEC\xA5.\xA11P\x02` C\x93\xA4V[PPPV[\x15a\x1AiWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[a\x1A\xCB\x81a ;V[\x15a\x1C\0W`@Q\x91a\x1A\xDD\x83a\x0FAV[a\x1A\xE5a\x11\xF2V[\x83R`fT_`fU\x80a\x1B\xC6W[P_[\x82Q\x80Q\x82\x10\x15a\x1BsW\x81a\x1B\x0C\x91a\x12iV[Q\x90`fT\x91`\x01`@\x1B\x83\x10\x15a\n\xF9W`\x01\x83\x01`fU`fT\x83\x10\x15a\x12}W`f_R\x80Q` \x91\x90\x91\x01Q`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17_\x80Q` a#M\x839\x81Q\x91R\x90\x92\x01\x91\x90\x91U`\x01\x01a\x1A\xF7V[PP\x91a\x1B\xB3\x90a\x1B\xC1\x7F#\xAA\xD4\xE6\x17D\xEC\xE1d\x13\n\xA4\x15\xC1an\x80\x13k\x0F\x07p\xE5e\x89C\x8B\x90\xB2i&^\x93`@Q\x93\x84\x93`@\x85R`@\x85\x01\x90a\x10\xA7V[\x90\x83\x82\x03` \x85\x01Ra\x10\xA7V[\x03\x90\xA1V[`f_R_\x80Q` a#M\x839\x81Q\x91R\x01_\x80Q` a#M\x839\x81Q\x91R[\x81\x81\x10a\x1B\xF5WPa\x1A\xF4V[_\x81U`\x01\x01a\x1B\xE8V[c\xD1sWy`\xE0\x1B_R`\x04_\xFD[\x81\x81\x03\x92\x91_\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\n\xD6WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R`m` R`@\x81 \x90\x92\x90\x83\x90a\x1CL\x90a\x17RV[\x80\x94\x84_R`n` R`\xFF`@_ T\x16\x15_\x14a\x1D\xC6Wa\x1Cp\x92\x93Pa\x1C\x0FV[\x92\x83\x15a\x1D\xC0W\x82_R`m` R`@_ \x90\x81T\x92a\x1C\x90\x83a\x17RV[\x84\x15\x15\x80a\x1D\x9BW[\x15a\x1C\xF4Wa\x1C\xA7_a!\xCDV[\x94_\x19\x81\x01\x90\x81\x11a\n\xD6W\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x95a\x18Aa\x1C\xE4\x92`@\x97a\x15&V[P[\x82Q\x91\x82R` \x82\x01R\xA2\x90V[c\xFF\xFF\xFF\xFFC\x11a\x19\x1BWa\x1D\x08_a!\xCDV[\x93`@Q\x95a\x1D\x16\x87a\x0F\\V[Cc\xFF\xFF\xFF\xFF\x16\x87R`\x01`\x01`\xE0\x1B\x03\x90\x95\x16` \x87\x01\x90\x81R\x94`\x01`@\x1B\x81\x10\x15a\n\xF9Wa\x1DM\x91`\x01\x82\x01\x81Ua\x15&V[\x95\x90\x95a\x19\x08WQ\x93Q` \x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x93\x90\x93\x17\x90\x93U\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x92`@\x92a\x1C\xE4V[P_\x19\x85\x01\x85\x81\x11a\n\xD6Wa\x1D\xB6c\xFF\xFF\xFF\xFF\x91\x86a\x15&V[PT\x16C\x14a\x1C\x99V[PPP\x90V[PPPa\x1D\xD2\x90a\x12\x9EV[\x91a\x1D\xDD\x81\x84a\x1C\x0FV[\x92\x83\x15a\x1D\xC0W\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x91`@\x91\x84_R`m` Ra\x1E\x1D\x81\x84_ a\x1FRV[PPa\x1C\xE6V[\x90a\x1E7a\x1E0a\x17EV[\x92\x83a\x14\xBCV[\x90`kTa\x1EE`ka\x17RV[\x90\x80\x15\x15\x80a\x1F,W[\x15a\x1E\xA8Wa\x1E]\x84a!\xCDV[_\x19\x82\x01\x91\x82\x11a\n\xD6Wa\x18Aa\x1Ev\x92`ka\x15&V[P\x7F\x86\xDC\xF8k\x12\xDF\xEE\xDE\xA7J\xE90\r\xBD\xAA\x19;\xCC\xE5\x80\x93i\xC8\x17~\xA2\xF4\xEA\xAAer\x9B`@\x80Q\x85\x81R\x84` \x82\x01R\xA1V[c\xFF\xFF\xFF\xFFC\x11a\x19\x1BWa\x1E\xBC\x84a!\xCDV[`@Q\x91a\x1E\xC9\x83a\x0F\\V[Cc\xFF\xFF\xFF\xFF\x16\x83R`\x01`\x01`\xE0\x1B\x03\x90\x91\x16` \x83\x01\x90\x81R\x90`\x01`@\x1B\x81\x10\x15a\n\xF9W\x80`\x01a\x1F\x03\x92\x01`kU`ka\x15&V[\x91\x90\x91a\x19\x08W\x91Q\x91Q` \x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1EvV[P_\x19\x81\x01\x81\x81\x11a\n\xD6Wa\x1FHc\xFF\xFF\xFF\xFF\x91`ka\x15&V[PT\x16C\x14a\x1EOV[\x91\x90\x91\x80Ta\x1F`\x82a\x17RV[\x91\x81\x15\x15\x80a \x16W[\x15a\x1F\x95Wa\x1Fx\x85a!\xCDV[\x90_\x19\x83\x01\x92\x83\x11a\n\xD6Wa\x1F\x91\x92a\x18A\x91a\x15&V[\x91\x90V[c\xFF\xFF\xFF\xFFC\x11a\x19\x1BWa\x1F\xA9\x85a!\xCDV[\x90`@Q\x92a\x1F\xB7\x84a\x0F\\V[Cc\xFF\xFF\xFF\xFF\x16\x84R`\x01`\x01`\xE0\x1B\x03\x90\x92\x16` \x84\x01\x90\x81R\x91`\x01`@\x1B\x81\x10\x15a\n\xF9Wa\x1F\xEE\x91`\x01\x82\x01\x81Ua\x15&V[\x91\x90\x91a\x19\x08W\x91Q\x91Q` \x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x91\x90V[P_\x19\x82\x01\x82\x81\x11a\n\xD6Wa 1c\xFF\xFF\xFF\xFF\x91\x83a\x15&V[PT\x16C\x14a\x1FjV[Q\x90_\x80\x80[\x84Q\x82\x10\x15a \x9EW`\x01`\x01`\xA0\x1B\x03a \\\x83\x87a\x12iV[QQ\x16\x90`\x01`\x01`\xA0\x1B\x03\x16\x81\x11\x15a\x0E)Wa \x95`\x01\x91\x93`\x01`\x01``\x1B\x03` a \x8B\x86\x8Aa\x12iV[Q\x01Q\x16\x90a\x12\x91V[\x91\x01\x90\x91a AV[P\x91\x92Pa'\x10\x14\x90Pa \xB0W_\x90V[`\x01\x90V[\x91\x90\x91a \xC2\x82\x84a\"<V[`\x05\x81\x10\x15a!\xB9W\x15\x90\x81a!\xA3W[Pa!\x9BW_\x92a!\ra!\x1B\x85\x94`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01R`@`D\x84\x01R`d\x83\x01\x90a\x17`V[\x03`\x1F\x19\x81\x01\x83R\x82a\x0FwV[Q\x91Z\xFA=\x15a!\x94W=a!/\x81a\x10FV[\x90a!=`@Q\x92\x83a\x0FwV[\x81R=_` \x83\x01>[\x81a!\x88W[\x81a!VWP\x90V[\x90P` \x81\x80Q\x81\x01\x03\x12a\n\xD2W` \x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x90\x81\x90\x03a\n\xD2Wc\x0B\x13]?`\xE1\x1B\x14\x90V[\x80Q` \x14\x91Pa!MV[``a!GV[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_a \xD3V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x01`\x01`\xE0\x1B\x03\x81\x11a!\xE7W`\x01`\x01`\xE0\x1B\x03\x16\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[\x81Q`A\x81\x03a\"hWP\x90a\"d\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a\"\xAAV[\x90\x91V[`@\x03a\"\xA1W`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a\n\xD6Wa\"d\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90a\"\xAAV[PP_\x90`\x02\x90V[\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a#AW`\xFF\x16\x90`\x1B\x82\x14\x15\x80a#6W[a#+W` \x93_\x93`\x80\x93`@Q\x93\x84R\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\n\xC7W_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a##W\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x82\x14\x15a\"\xE1V[PPPP_\x90`\x03\x90V\xFEFP\x18y\xB8\xCA\x85%\xE8\xC2\xFDQ\x9E/\xBF\xCF\xA2\xEB\xEA&P\x12\x94\xAA\x02\xCB\xFC\xFB\x12\xE9CT\xA2dipfsX\"\x12 \xE3\xC3\xC7\x95_\x19\xF8\x94\xDE\xE7m\x8A\x17!\xDF\xC8if\x03\xF5\xD6\x11\xCA\xC1\x94\xBA\xF3=\x16\xD0k\xB3dsolcC\0\x08\x1A\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f803560e01c8062cf2ab514610f0c5780630dba339414610ee35780631626ba7e14610bd75780631703a01814610b875780631e4cd85e14610b5e578063314f3a4914610b445780633b242e4a14610b0d5780633d5611f61461092457806340bf2fb7146109065780635140a548146107fa5780635e1042e8146107b45780635ef533291461078f578063696255be14610714578063715018a6146106b7578063743c31f41461066f578063857dc1901461055a5780638da5cb5b14610531578063955f2d90146104e457806398ec1ac9146104b8578063ab11899514610323578063b933fa74146102bb578063cdcd358114610270578063dec5d1f6146101fe578063ec7fbb31146101bf5763f2fde38b1461012c575f80fd5b346101bc5760203660031901126101bc57610145610faf565b61014d611785565b6001600160a01b038116156101685761016590611995565b80f35b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b80fd5b50346101bc5760203660031901126101bc5760209060ff906040906001600160a01b036101ea610faf565b168152606e84522054166040519015158152f35b50346101bc5760403660031901126101bc576004356001600160401b03811161026c5761022f903690600401611100565b602435906001600160401b03821161026857610263610255610165933690600401610fc5565b9161025e611785565b611ac2565b6114d7565b8280fd5b5080fd5b50346101bc5760203660031901126101bc576020906001600160a01b03610295610faf565b168152606a8252604090206001600160a01b03906102b290611752565b16604051908152f35b50346101bc57806003193601126101bc57606c5490816102ed57602091505b6040516001600160e01b03919091168152f35b5f19820191821161030f5750610306602091606c611526565b5054811c6102da565b634e487b7160e01b81526011600452602490fd5b50346101bc5760603660031901126101bc5761033d610faf565b6044356001600160401b0381116102685761035c903690600401611100565b82549060ff8260081c1615928380946104ab575b8015610494575b156104385760ff19831660011785556103c79284610427575b506103a160ff865460081c16611a62565b60018060a01b03166001600160601b0360a01b606854161760685561025e6024356117dd565b6103e060ff835460081c166103db81611a62565b611a62565b6103e933611995565b6103f05780f35b61ff001981541681557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160018152a180f35b61ffff19166101011785555f610390565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b50303b1580156103775750600160ff841614610377565b50600160ff841610610370565b50346101bc5760203660031901126101bc5760206104dc6104d7610faf565b61129e565b604051908152f35b50346101bc5760403660031901126101bc576104fe610faf565b6024359063ffffffff8216809203610268576001600160a01b03168252606d602090815260409092206104dc91906116b5565b50346101bc57806003193601126101bc576033546040516001600160a01b039091168152602090f35b50346101bc57806003193601126101bc57338152606e60205260ff6040822054161561066057606554801561064c575f1901606555338152606e6020526040812060ff1981541690556105b46105af33611c27565b611e24565b505060685481906001600160a01b0316803b15610649578180916024604051809481936351b27a6d60e11b83523360048401525af1801561063e57610629575b506068546001600160a01b0316337f31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed5808380a380f35b8161063391610f77565b6101bc57805f6105f4565b6040513d84823e3d90fd5b50fd5b634e487b7160e01b82526011600452602482fd5b6325ec6c1f60e01b8152600490fd5b50346101bc5760203660031901126101bc57610689610faf565b338252606e60205260ff604083205416156106a85761016590336119dd565b6325ec6c1f60e01b8252600482fd5b50346101bc57806003193601126101bc576106d0611785565b603380546001600160a01b0319811690915581906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b50346101bc5760403660031901126101bc576024356004356001600160401b038211610268577f713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f604061076e610165943690600401610fc5565b92610777611785565b606754908060675582519182526020820152a16114d7565b50346101bc5760203660031901126101bc576107a9611785565b6101656004356117dd565b50346101bc5760403660031901126101bc576020906001600160a01b036107d9610faf565b168152606a8252604090206001600160a01b03906102b290602435906116b5565b50346101bc5760403660031901126101bc576004356001600160401b03811161026c573660238201121561026c5780600401359061083782610f98565b906108456040519283610f77565b828252602082019260051b8101602401833682116109025760248301905b8282106108cf57505050506024356001600160401b0381116108cb5761088d903690600401611061565b5051156108b757518051606554036108a857610165906114d7565b63169efb5b60e11b8252600482fd5b634e487b7160e01b82526032600452602482fd5b8380fd5b81356001600160401b0381116108fe576020916108f3839260243691890101610fc5565b815201910190610863565b8780fd5b8580fd5b50346101bc57806003193601126101bc576020606754604051908152f35b5034610ad2576040366003190112610ad2576004356001600160401b038111610ad25760606003198236030112610ad257604051606081018181106001600160401b03821117610af95760405281600401356001600160401b038111610ad2576109949060043691850101611061565b8152602480830135602083019081526044909301356040830190815290356001600160a01b0381168103610ad257335f52606e60205260ff60405f205416610aea57606554905f198214610ad6576001610a179201606555335f52606e60205260405f20600160ff19825416179055610a0f6105af33611c27565b5050336119dd565b6068546001600160a01b0316803b15610ad257610a635f809460405196879586948593639926ee7d60e01b855233600486015260406024860152516060604486015260a4850190611760565b9151606484015251608483015203925af18015610ac757610ab4575b506068546001600160a01b0316337fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c18380a380f35b610ac091505f90610f77565b5f80610a7f565b6040513d5f823e3d90fd5b5f80fd5b634e487b7160e01b5f52601160045260245ffd5b6342ee68b560e01b5f5260045ffd5b634e487b7160e01b5f52604160045260245ffd5b34610ad2576020366003190112610ad2576001600160a01b03610b2e610faf565b165f52606d60205260206104dc60405f20611752565b34610ad2575f366003190112610ad25760206104dc611745565b34610ad2576020366003190112610ad25760206104dc63ffffffff610b81611033565b16611624565b34610ad2575f366003190112610ad2576060604051610ba581610f41565b52610bd3604051610bb581610f41565b610bbd6111f2565b81526040519182916020835260208301906110a7565b0390f35b34610ad2576040366003190112610ad2576004356024356001600160401b038111610ad257610c0a903690600401611061565b8051810160608260208301920312610ad25760208201516001600160401b038111610ad25782019181603f84011215610ad2576020830151610c4b81610f98565b93610c596040519586610f77565b8185526020808087019360051b8301010190848211610ad257604001915b818310610ec35750505060408101516001600160401b038111610ad257810182603f82011215610ad257602081015190610cb082610f98565b93610cbe6040519586610f77565b8285526020808087019460051b8401010191818311610ad25760408101935b838510610e565788888860608901519163ffffffff8316809303610ad2578051915f809582518503610e47578415610e385743861015945f949093925b848610610d8457888888610d7557610d318161153b565b8211610d6657610d4090611624565b11610d5757604051630b135d3f60e11b8152602090f35b63e121632f60e01b5f5260045ffd5b634b05a0f760e11b5f5260045ffd5b63e64f180f60e01b5f5260045ffd5b949793949293926001600160a01b03610d9d8a84611269565b511696610d7557865f52606a6020528660018060a01b03610dc18a60405f206116b5565b16916001600160a01b03161015610e2957610de89083610de18b88611269565b51916120b5565b15610e1a576001610e1087945f985f52606d602052610e0a8a60405f206116b5565b90611291565b9801949392610d1a565b638baa579f60e01b5f5260045ffd5b63ba50f91160e01b5f5260045ffd5b63251f56a160e21b5f5260045ffd5b631fec674760e31b5f5260045ffd5b84516001600160401b038111610ad2576020908301019083603f83011215610ad257602082015190610e8782611046565b610e946040519182610f77565b8281526040848401018610610ad257610eb8602094938594604086850191016111d1565b815201940193610cdd565b82516001600160a01b0381168103610ad257815260209283019201610c77565b34610ad2576020366003190112610ad25760206104dc63ffffffff610f06611033565b1661153b565b34610ad2576020366003190112610ad2576004356001600160401b038111610ad257610263610f3f913690600401610fc5565b005b602081019081106001600160401b03821117610af957604052565b604081019081106001600160401b03821117610af957604052565b90601f801991011681019081106001600160401b03821117610af957604052565b6001600160401b038111610af95760051b60200190565b600435906001600160a01b0382168203610ad257565b9080601f83011215610ad257813590610fdd82610f98565b92610feb6040519485610f77565b82845260208085019360051b820101918211610ad257602001915b8183106110135750505090565b82356001600160a01b0381168103610ad257815260209283019201611006565b6004359063ffffffff82168203610ad257565b6001600160401b038111610af957601f01601f191660200190565b81601f82011215610ad25780359061107882611046565b926110866040519485610f77565b82845260208383010111610ad257815f926020809301838601378301015290565b602060408184019251938281528451809452019201905f5b8181106110cc5750505090565b825180516001600160a01b031685526020908101516001600160601b031681860152604090940193909201916001016110bf565b9190602083820312610ad25760405161111881610f41565b80938035906001600160401b038211610ad2570182601f82011215610ad25780359061114382610f98565b936111516040519586610f77565b82855260208086019360061b83010191818311610ad257602001925b82841061117b575050505052565b604084830312610ad2576040519061119282610f5c565b84356001600160a01b0381168103610ad25782526020850135906001600160601b0382168203610ad2578260209283604095015281520193019261116d565b5f5b8381106111e25750505f910152565b81810151838201526020016111d3565b606654906111ff82610f98565b9161120d6040519384610f77565b80835260665f9081525f8051602061234d833981519152602085015b8383106112365750505050565b60016020819260405161124881610f5c565b8554848060a01b038116825260a01c83820152815201920192019190611229565b805182101561127d5760209160051b010190565b634e487b7160e01b5f52603260045260245ffd5b91908201809211610ad657565b6112a66111f2565b805190925f6112cd6112b784610f98565b936112c56040519586610f77565b808552610f98565b6020840190601f19013682375f5b865181101561130d576001906001600160a01b036112f9828a611269565b5151166113068288611269565b52016112db565b5060408051639004134760e01b81526001600160a01b0390961660048701526024860152925160448501819052919390929091829160648301915f5b81811061149a57505f939283900391508290507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610ac7575f9161140a575b505f915b84518310156113ec576113ad8383611269565b51906001600160601b0360206113c38689611269565b5101511691828102928184041490151715610ad6576001916113e491611291565b92019161139a565b915050612710919250046067548110155f146114055790565b505f90565b90503d805f833e61141b8183610f77565b810190602081830312610ad2578051906001600160401b038211610ad257019080601f83011215610ad257815161145181610f98565b9261145f6040519485610f77565b81845260208085019260051b820101928311610ad257602001905b82821061148a575050505f611396565b815181526020918201910161147a565b82516001600160a01b0316845285945060209384019390920191600101611349565b9190915f8382019384129112908015821691151617610ad657565b905f805b83518210156115155760019061150d906115076001600160a01b036115008689611269565b5116611c27565b906114bc565b9101906114db565b9050611522919250611e24565b5050565b805482101561127d575f5260205f2001905f90565b438110156115e057606b54905f905b8282106115875750508061156757505f5b6001600160e01b031690565b5f198101908111610ad65761157d90606b611526565b505460201c61155b565b909161159a81841860011c828516611291565b908263ffffffff6115ac84606b611526565b50541611156115be5750915b9061154a565b925060018101809111156115b857634e487b7160e01b5f52601160045260245ffd5b606460405162461bcd60e51b815260206004820152602060248201527f436865636b706f696e74733a20626c6f636b206e6f7420796574206d696e65646044820152fd5b438110156115e057606c54905f905b82821061165c5750508061164657505f90565b5f198101908111610ad65761157d90606c611526565b909161166f81841860011c828516611291565b908263ffffffff61168184606c611526565b50541611156116935750915b90611633565b9250600181018091111561168d57634e487b7160e01b5f52601160045260245ffd5b90438110156115e0578154905f905b8282106116ed575050806116d957505f919050565b5f198101908111610ad65761157d91611526565b909161170081841860011c828516611291565b908263ffffffff6117118488611526565b50541611156117235750915b906116c4565b9250600181018091111561171d57634e487b7160e01b5f52601160045260245ffd5b606b548061156757505f90565b8054806116d957505f919050565b90602091611779815180928185528580860191016111d1565b601f01601f1916010190565b6033546001600160a01b0316330361179957565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b606c54906117eb606c611752565b908215158061196f575b1561186357611803816121cd565b915f198401938411610ad6576118596020936118417f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b96606c611526565b509063ffffffff82549181199060201b169116179055565b50604051908152a1565b63ffffffff431161191b57611877816121cd565b916040519361188585610f5c565b4363ffffffff1685526001600160e01b039093166020850190815292600160401b811015610af9578060016118bf9201606c55606c611526565b9490946119085751925163ffffffff90931663ffffffff19602094851b16179093557f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b92611859565b634e487b7160e01b5f525f60045260245ffd5b60405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b6064820152608490fd5b505f198301838111610ad65761198b63ffffffff91606c611526565b50541643146117f5565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b6001600160a01b039081165f818152606a6020526040902090929190611a0290611752565b6001600160a01b0390921692911690828214611a5d57805f52606a602052611a2d8360405f20611f52565b50506040519182527fd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea131500260204393a4565b505050565b15611a6957565b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b611acb8161203b565b15611c005760405191611add83610f41565b611ae56111f2565b83526066545f60665580611bc6575b505f5b82518051821015611b735781611b0c91611269565b519060665491600160401b831015610af9576001830160665560665483101561127d5760665f5280516020919091015160a01b6001600160a01b0319166001600160a01b0391909116175f8051602061234d83398151915290920191909155600101611af7565b505091611bb390611bc17f23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e936040519384936040855260408501906110a7565b9083820360208501526110a7565b0390a1565b60665f525f8051602061234d833981519152015f8051602061234d8339815191525b818110611bf55750611af4565b5f8155600101611be8565b63d173577960e01b5f5260045ffd5b81810392915f138015828513169184121617610ad657565b6001600160a01b0381165f818152606d602052604081209092908390611c4c90611752565b8094845f52606e60205260ff60405f205416155f14611dc657611c70929350611c0f565b928315611dc057825f52606d60205260405f2090815492611c9083611752565b84151580611d9b575b15611cf457611ca75f6121cd565b945f198101908111610ad6577f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe59495611841611ce492604097611526565b505b82519182526020820152a290565b63ffffffff431161191b57611d085f6121cd565b9360405195611d1687610f5c565b4363ffffffff1687526001600160e01b039095166020870190815294600160401b811015610af957611d4d91600182018155611526565b9590956119085751935160201b63ffffffff191663ffffffff94909416939093179093557f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe59492604092611ce4565b505f198501858111610ad657611db663ffffffff9186611526565b5054164314611c99565b50505090565b505050611dd29061129e565b91611ddd8184611c0f565b928315611dc0577f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe59491604091845f52606d602052611e1d81845f20611f52565b5050611ce6565b90611e37611e30611745565b92836114bc565b90606b54611e45606b611752565b9080151580611f2c575b15611ea857611e5d846121cd565b5f198201918211610ad657611841611e7692606b611526565b507f86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b60408051858152846020820152a1565b63ffffffff431161191b57611ebc846121cd565b60405191611ec983610f5c565b4363ffffffff1683526001600160e01b039091166020830190815290600160401b811015610af957806001611f039201606b55606b611526565b919091611908579151915160201b63ffffffff191663ffffffff92909216919091179055611e76565b505f198101818111610ad657611f4863ffffffff91606b611526565b5054164314611e4f565b9190918054611f6082611752565b9181151580612016575b15611f9557611f78856121cd565b905f198301928311610ad657611f919261184191611526565b9190565b63ffffffff431161191b57611fa9856121cd565b9060405192611fb784610f5c565b4363ffffffff1684526001600160e01b039092166020840190815291600160401b811015610af957611fee91600182018155611526565b919091611908579151915160201b63ffffffff191663ffffffff929092169190911790559190565b505f198201828111610ad65761203163ffffffff9183611526565b5054164314611f6a565b51905f80805b845182101561209e576001600160a01b0361205c8387611269565b515116906001600160a01b0316811115610e2957612095600191936001600160601b03602061208b868a611269565b5101511690611291565b91019091612041565b509192506127101490506120b0575f90565b600190565b9190916120c2828461223c565b60058110156121b9571590816121a3575b5061219b575f9261210d61211b85946040519283916020830195630b135d3f60e11b87526024840152604060448401526064830190611760565b03601f198101835282610f77565b51915afa3d15612194573d61212f81611046565b9061213d6040519283610f77565b81523d5f602083013e5b81612188575b81612156575090565b9050602081805181010312610ad257602001516001600160e01b0319811690819003610ad257630b135d3f60e11b1490565b8051602014915061214d565b6060612147565b505050600190565b6001600160a01b0383811691161490505f6120d3565b634e487b7160e01b5f52602160045260245ffd5b6001600160e01b0381116121e7576001600160e01b031690565b60405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b6064820152608490fd5b81516041810361226857509061226491602082015190606060408401519301515f1a906122aa565b9091565b6040036122a15760406020830151920151918260ff1c91601b8301809311610ad657612264936001600160ff1b03169260ff16906122aa565b50505f90600290565b907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116123415760ff1690601b82141580612336575b61232b576020935f93608093604051938452868401526040830152606082015282805260015afa15610ac7575f516001600160a01b0381161561232357905f90565b505f90600190565b505050505f90600490565b50601c8214156122e1565b505050505f9060039056fe46501879b8ca8525e8c2fd519e2fbfcfa2ebea26501294aa02cbfcfb12e94354a2646970667358221220e3c3c7955f19f894dee76d8a1721dfc8696603f5d611cac194baf33d16d06bb364736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_\x805`\xE0\x1C\x80b\xCF*\xB5\x14a\x0F\x0CW\x80c\r\xBA3\x94\x14a\x0E\xE3W\x80c\x16&\xBA~\x14a\x0B\xD7W\x80c\x17\x03\xA0\x18\x14a\x0B\x87W\x80c\x1EL\xD8^\x14a\x0B^W\x80c1O:I\x14a\x0BDW\x80c;$.J\x14a\x0B\rW\x80c=V\x11\xF6\x14a\t$W\x80c@\xBF/\xB7\x14a\t\x06W\x80cQ@\xA5H\x14a\x07\xFAW\x80c^\x10B\xE8\x14a\x07\xB4W\x80c^\xF53)\x14a\x07\x8FW\x80cibU\xBE\x14a\x07\x14W\x80cqP\x18\xA6\x14a\x06\xB7W\x80ct<1\xF4\x14a\x06oW\x80c\x85}\xC1\x90\x14a\x05ZW\x80c\x8D\xA5\xCB[\x14a\x051W\x80c\x95_-\x90\x14a\x04\xE4W\x80c\x98\xEC\x1A\xC9\x14a\x04\xB8W\x80c\xAB\x11\x89\x95\x14a\x03#W\x80c\xB93\xFAt\x14a\x02\xBBW\x80c\xCD\xCD5\x81\x14a\x02pW\x80c\xDE\xC5\xD1\xF6\x14a\x01\xFEW\x80c\xEC\x7F\xBB1\x14a\x01\xBFWc\xF2\xFD\xE3\x8B\x14a\x01,W_\x80\xFD[4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCWa\x01Ea\x0F\xAFV[a\x01Ma\x17\x85V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x01hWa\x01e\x90a\x19\x95V[\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[\x80\xFD[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCW` \x90`\xFF\x90`@\x90`\x01`\x01`\xA0\x1B\x03a\x01\xEAa\x0F\xAFV[\x16\x81R`n\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lWa\x02/\x906\x90`\x04\x01a\x11\0V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02hWa\x02ca\x02Ua\x01e\x936\x90`\x04\x01a\x0F\xC5V[\x91a\x02^a\x17\x85V[a\x1A\xC2V[a\x14\xD7V[\x82\x80\xFD[P\x80\xFD[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCW` \x90`\x01`\x01`\xA0\x1B\x03a\x02\x95a\x0F\xAFV[\x16\x81R`j\x82R`@\x90 `\x01`\x01`\xA0\x1B\x03\x90a\x02\xB2\x90a\x17RV[\x16`@Q\x90\x81R\xF3[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW`lT\x90\x81a\x02\xEDW` \x91P[`@Q`\x01`\x01`\xE0\x1B\x03\x91\x90\x91\x16\x81R\xF3[_\x19\x82\x01\x91\x82\x11a\x03\x0FWPa\x03\x06` \x91`la\x15&V[PT\x81\x1Ca\x02\xDAV[cNH{q`\xE0\x1B\x81R`\x11`\x04R`$\x90\xFD[P4a\x01\xBCW``6`\x03\x19\x01\x12a\x01\xBCWa\x03=a\x0F\xAFV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x02hWa\x03\\\x906\x90`\x04\x01a\x11\0V[\x82T\x90`\xFF\x82`\x08\x1C\x16\x15\x92\x83\x80\x94a\x04\xABW[\x80\x15a\x04\x94W[\x15a\x048W`\xFF\x19\x83\x16`\x01\x17\x85Ua\x03\xC7\x92\x84a\x04'W[Pa\x03\xA1`\xFF\x86T`\x08\x1C\x16a\x1AbV[`\x01\x80`\xA0\x1B\x03\x16`\x01`\x01``\x1B\x03`\xA0\x1B`hT\x16\x17`hUa\x02^`$5a\x17\xDDV[a\x03\xE0`\xFF\x83T`\x08\x1C\x16a\x03\xDB\x81a\x1AbV[a\x1AbV[a\x03\xE93a\x19\x95V[a\x03\xF0W\x80\xF3[a\xFF\0\x19\x81T\x16\x81U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\x01\x81R\xA1\x80\xF3[a\xFF\xFF\x19\x16a\x01\x01\x17\x85U_a\x03\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[P0;\x15\x80\x15a\x03wWP`\x01`\xFF\x84\x16\x14a\x03wV[P`\x01`\xFF\x84\x16\x10a\x03pV[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCW` a\x04\xDCa\x04\xD7a\x0F\xAFV[a\x12\x9EV[`@Q\x90\x81R\xF3[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCWa\x04\xFEa\x0F\xAFV[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x02hW`\x01`\x01`\xA0\x1B\x03\x16\x82R`m` \x90\x81R`@\x90\x92 a\x04\xDC\x91\x90a\x16\xB5V[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW3\x81R`n` R`\xFF`@\x82 T\x16\x15a\x06`W`eT\x80\x15a\x06LW_\x19\x01`eU3\x81R`n` R`@\x81 `\xFF\x19\x81T\x16\x90Ua\x05\xB4a\x05\xAF3a\x1C'V[a\x1E$V[PP`hT\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x06IW\x81\x80\x91`$`@Q\x80\x94\x81\x93cQ\xB2zm`\xE1\x1B\x83R3`\x04\x84\x01RZ\xF1\x80\x15a\x06>Wa\x06)W[P`hT`\x01`\x01`\xA0\x1B\x03\x163\x7F1\xE0\xAD\xFE\xC7\x1B\xCC\xEE7\xB6\xE8:\x90\xC2\xFE\xDB\x17\xD8\xF1i?\xEE\x86<Gq\xE7\xBF\xE2\xAE\xD5\x80\x83\x80\xA3\x80\xF3[\x81a\x063\x91a\x0FwV[a\x01\xBCW\x80_a\x05\xF4V[`@Q=\x84\x82>=\x90\xFD[P\xFD[cNH{q`\xE0\x1B\x82R`\x11`\x04R`$\x82\xFD[c%\xECl\x1F`\xE0\x1B\x81R`\x04\x90\xFD[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCWa\x06\x89a\x0F\xAFV[3\x82R`n` R`\xFF`@\x83 T\x16\x15a\x06\xA8Wa\x01e\x903a\x19\xDDV[c%\xECl\x1F`\xE0\x1B\x82R`\x04\x82\xFD[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCWa\x06\xD0a\x17\x85V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW`$5`\x045`\x01`\x01`@\x1B\x03\x82\x11a\x02hW\x7Fq<\xA5;\x88\xD6\xEBc\xF5\xB1\x85L\xB8\xCB\xDDsn\xC5\x1E\xDA\"^Fy\x1A\xA9)\x8B\x01`d\x8F`@a\x07na\x01e\x946\x90`\x04\x01a\x0F\xC5V[\x92a\x07wa\x17\x85V[`gT\x90\x80`gU\x82Q\x91\x82R` \x82\x01R\xA1a\x14\xD7V[P4a\x01\xBCW` 6`\x03\x19\x01\x12a\x01\xBCWa\x07\xA9a\x17\x85V[a\x01e`\x045a\x17\xDDV[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW` \x90`\x01`\x01`\xA0\x1B\x03a\x07\xD9a\x0F\xAFV[\x16\x81R`j\x82R`@\x90 `\x01`\x01`\xA0\x1B\x03\x90a\x02\xB2\x90`$5\x90a\x16\xB5V[P4a\x01\xBCW`@6`\x03\x19\x01\x12a\x01\xBCW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02lW6`#\x82\x01\x12\x15a\x02lW\x80`\x04\x015\x90a\x087\x82a\x0F\x98V[\x90a\x08E`@Q\x92\x83a\x0FwV[\x82\x82R` \x82\x01\x92`\x05\x1B\x81\x01`$\x01\x836\x82\x11a\t\x02W`$\x83\x01\x90[\x82\x82\x10a\x08\xCFWPPPP`$5`\x01`\x01`@\x1B\x03\x81\x11a\x08\xCBWa\x08\x8D\x906\x90`\x04\x01a\x10aV[PQ\x15a\x08\xB7WQ\x80Q`eT\x03a\x08\xA8Wa\x01e\x90a\x14\xD7V[c\x16\x9E\xFB[`\xE1\x1B\x82R`\x04\x82\xFD[cNH{q`\xE0\x1B\x82R`2`\x04R`$\x82\xFD[\x83\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x08\xFEW` \x91a\x08\xF3\x83\x92`$6\x91\x89\x01\x01a\x0F\xC5V[\x81R\x01\x91\x01\x90a\x08cV[\x87\x80\xFD[\x85\x80\xFD[P4a\x01\xBCW\x80`\x03\x196\x01\x12a\x01\xBCW` `gT`@Q\x90\x81R\xF3[P4a\n\xD2W`@6`\x03\x19\x01\x12a\n\xD2W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W```\x03\x19\x826\x03\x01\x12a\n\xD2W`@Q``\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@R\x81`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2Wa\t\x94\x90`\x046\x91\x85\x01\x01a\x10aV[\x81R`$\x80\x83\x015` \x83\x01\x90\x81R`D\x90\x93\x015`@\x83\x01\x90\x81R\x905`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W3_R`n` R`\xFF`@_ T\x16a\n\xEAW`eT\x90_\x19\x82\x14a\n\xD6W`\x01a\n\x17\x92\x01`eU3_R`n` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90Ua\n\x0Fa\x05\xAF3a\x1C'V[PP3a\x19\xDDV[`hT`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\n\xD2Wa\nc_\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\x99&\xEE}`\xE0\x1B\x85R3`\x04\x86\x01R`@`$\x86\x01RQ```D\x86\x01R`\xA4\x85\x01\x90a\x17`V[\x91Q`d\x84\x01RQ`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\n\xC7Wa\n\xB4W[P`hT`\x01`\x01`\xA0\x1B\x03\x163\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x83\x80\xA3\x80\xF3[a\n\xC0\x91P_\x90a\x0FwV[_\x80a\n\x7FV[`@Q=_\x82>=\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[cB\xEEh\xB5`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W`\x01`\x01`\xA0\x1B\x03a\x0B.a\x0F\xAFV[\x16_R`m` R` a\x04\xDC`@_ a\x17RV[4a\n\xD2W_6`\x03\x19\x01\x12a\n\xD2W` a\x04\xDCa\x17EV[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W` a\x04\xDCc\xFF\xFF\xFF\xFFa\x0B\x81a\x103V[\x16a\x16$V[4a\n\xD2W_6`\x03\x19\x01\x12a\n\xD2W```@Qa\x0B\xA5\x81a\x0FAV[Ra\x0B\xD3`@Qa\x0B\xB5\x81a\x0FAV[a\x0B\xBDa\x11\xF2V[\x81R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x10\xA7V[\x03\x90\xF3[4a\n\xD2W`@6`\x03\x19\x01\x12a\n\xD2W`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2Wa\x0C\n\x906\x90`\x04\x01a\x10aV[\x80Q\x81\x01``\x82` \x83\x01\x92\x03\x12a\n\xD2W` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W\x82\x01\x91\x81`?\x84\x01\x12\x15a\n\xD2W` \x83\x01Qa\x0CK\x81a\x0F\x98V[\x93a\x0CY`@Q\x95\x86a\x0FwV[\x81\x85R` \x80\x80\x87\x01\x93`\x05\x1B\x83\x01\x01\x01\x90\x84\x82\x11a\n\xD2W`@\x01\x91[\x81\x83\x10a\x0E\xC3WPPP`@\x81\x01Q`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W\x81\x01\x82`?\x82\x01\x12\x15a\n\xD2W` \x81\x01Q\x90a\x0C\xB0\x82a\x0F\x98V[\x93a\x0C\xBE`@Q\x95\x86a\x0FwV[\x82\x85R` \x80\x80\x87\x01\x94`\x05\x1B\x84\x01\x01\x01\x91\x81\x83\x11a\n\xD2W`@\x81\x01\x93[\x83\x85\x10a\x0EVW\x88\x88\x88``\x89\x01Q\x91c\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\n\xD2W\x80Q\x91_\x80\x95\x82Q\x85\x03a\x0EGW\x84\x15a\x0E8WC\x86\x10\x15\x94_\x94\x90\x93\x92[\x84\x86\x10a\r\x84W\x88\x88\x88a\ruWa\r1\x81a\x15;V[\x82\x11a\rfWa\r@\x90a\x16$V[\x11a\rWW`@Qc\x0B\x13]?`\xE1\x1B\x81R` \x90\xF3[c\xE1!c/`\xE0\x1B_R`\x04_\xFD[cK\x05\xA0\xF7`\xE1\x1B_R`\x04_\xFD[c\xE6O\x18\x0F`\xE0\x1B_R`\x04_\xFD[\x94\x97\x93\x94\x92\x93\x92`\x01`\x01`\xA0\x1B\x03a\r\x9D\x8A\x84a\x12iV[Q\x16\x96a\ruW\x86_R`j` R\x86`\x01\x80`\xA0\x1B\x03a\r\xC1\x8A`@_ a\x16\xB5V[\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x10\x15a\x0E)Wa\r\xE8\x90\x83a\r\xE1\x8B\x88a\x12iV[Q\x91a \xB5V[\x15a\x0E\x1AW`\x01a\x0E\x10\x87\x94_\x98_R`m` Ra\x0E\n\x8A`@_ a\x16\xB5V[\x90a\x12\x91V[\x98\x01\x94\x93\x92a\r\x1AV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[c\xBAP\xF9\x11`\xE0\x1B_R`\x04_\xFD[c%\x1FV\xA1`\xE2\x1B_R`\x04_\xFD[c\x1F\xECgG`\xE3\x1B_R`\x04_\xFD[\x84Q`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2W` \x90\x83\x01\x01\x90\x83`?\x83\x01\x12\x15a\n\xD2W` \x82\x01Q\x90a\x0E\x87\x82a\x10FV[a\x0E\x94`@Q\x91\x82a\x0FwV[\x82\x81R`@\x84\x84\x01\x01\x86\x10a\n\xD2Wa\x0E\xB8` \x94\x93\x85\x94`@\x86\x85\x01\x91\x01a\x11\xD1V[\x81R\x01\x94\x01\x93a\x0C\xDDV[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W\x81R` \x92\x83\x01\x92\x01a\x0CwV[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W` a\x04\xDCc\xFF\xFF\xFF\xFFa\x0F\x06a\x103V[\x16a\x15;V[4a\n\xD2W` 6`\x03\x19\x01\x12a\n\xD2W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\n\xD2Wa\x02ca\x0F?\x916\x90`\x04\x01a\x0F\xC5V[\0[` \x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@RV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\n\xF9W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\n\xF9W`\x05\x1B` \x01\x90V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\xD2WV[\x90\x80`\x1F\x83\x01\x12\x15a\n\xD2W\x815\x90a\x0F\xDD\x82a\x0F\x98V[\x92a\x0F\xEB`@Q\x94\x85a\x0FwV[\x82\x84R` \x80\x85\x01\x93`\x05\x1B\x82\x01\x01\x91\x82\x11a\n\xD2W` \x01\x91[\x81\x83\x10a\x10\x13WPPP\x90V[\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W\x81R` \x92\x83\x01\x92\x01a\x10\x06V[`\x045\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\n\xD2WV[`\x01`\x01`@\x1B\x03\x81\x11a\n\xF9W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x81`\x1F\x82\x01\x12\x15a\n\xD2W\x805\x90a\x10x\x82a\x10FV[\x92a\x10\x86`@Q\x94\x85a\x0FwV[\x82\x84R` \x83\x83\x01\x01\x11a\n\xD2W\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[` `@\x81\x84\x01\x92Q\x93\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x10\xCCWPPP\x90V[\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x86\x01R`@\x90\x94\x01\x93\x90\x92\x01\x91`\x01\x01a\x10\xBFV[\x91\x90` \x83\x82\x03\x12a\n\xD2W`@Qa\x11\x18\x81a\x0FAV[\x80\x93\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\n\xD2W\x01\x82`\x1F\x82\x01\x12\x15a\n\xD2W\x805\x90a\x11C\x82a\x0F\x98V[\x93a\x11Q`@Q\x95\x86a\x0FwV[\x82\x85R` \x80\x86\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\n\xD2W` \x01\x92[\x82\x84\x10a\x11{WPPPPRV[`@\x84\x83\x03\x12a\n\xD2W`@Q\x90a\x11\x92\x82a\x0F\\V[\x845`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\n\xD2W\x82R` \x85\x015\x90`\x01`\x01``\x1B\x03\x82\x16\x82\x03a\n\xD2W\x82` \x92\x83`@\x95\x01R\x81R\x01\x93\x01\x92a\x11mV[_[\x83\x81\x10a\x11\xE2WPP_\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x11\xD3V[`fT\x90a\x11\xFF\x82a\x0F\x98V[\x91a\x12\r`@Q\x93\x84a\x0FwV[\x80\x83R`f_\x90\x81R_\x80Q` a#M\x839\x81Q\x91R` \x85\x01[\x83\x83\x10a\x126WPPPPV[`\x01` \x81\x92`@Qa\x12H\x81a\x0F\\V[\x85T\x84\x80`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1C\x83\x82\x01R\x81R\x01\x92\x01\x92\x01\x91\x90a\x12)V[\x80Q\x82\x10\x15a\x12}W` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x82\x01\x80\x92\x11a\n\xD6WV[a\x12\xA6a\x11\xF2V[\x80Q\x90\x92_a\x12\xCDa\x12\xB7\x84a\x0F\x98V[\x93a\x12\xC5`@Q\x95\x86a\x0FwV[\x80\x85Ra\x0F\x98V[` \x84\x01\x90`\x1F\x19\x016\x827_[\x86Q\x81\x10\x15a\x13\rW`\x01\x90`\x01`\x01`\xA0\x1B\x03a\x12\xF9\x82\x8Aa\x12iV[QQ\x16a\x13\x06\x82\x88a\x12iV[R\x01a\x12\xDBV[P`@\x80Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16`\x04\x87\x01R`$\x86\x01R\x92Q`D\x85\x01\x81\x90R\x91\x93\x90\x92\x90\x91\x82\x91`d\x83\x01\x91_[\x81\x81\x10a\x14\x9AWP_\x93\x92\x83\x90\x03\x91P\x82\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\n\xC7W_\x91a\x14\nW[P_\x91[\x84Q\x83\x10\x15a\x13\xECWa\x13\xAD\x83\x83a\x12iV[Q\x90`\x01`\x01``\x1B\x03` a\x13\xC3\x86\x89a\x12iV[Q\x01Q\x16\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a\n\xD6W`\x01\x91a\x13\xE4\x91a\x12\x91V[\x92\x01\x91a\x13\x9AV[\x91PPa'\x10\x91\x92P\x04`gT\x81\x10\x15_\x14a\x14\x05W\x90V[P_\x90V[\x90P=\x80_\x83>a\x14\x1B\x81\x83a\x0FwV[\x81\x01\x90` \x81\x83\x03\x12a\n\xD2W\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\n\xD2W\x01\x90\x80`\x1F\x83\x01\x12\x15a\n\xD2W\x81Qa\x14Q\x81a\x0F\x98V[\x92a\x14_`@Q\x94\x85a\x0FwV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\n\xD2W` \x01\x90[\x82\x82\x10a\x14\x8AWPPP_a\x13\x96V[\x81Q\x81R` \x91\x82\x01\x91\x01a\x14zV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x13IV[\x91\x90\x91_\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\n\xD6WV[\x90_\x80[\x83Q\x82\x10\x15a\x15\x15W`\x01\x90a\x15\r\x90a\x15\x07`\x01`\x01`\xA0\x1B\x03a\x15\0\x86\x89a\x12iV[Q\x16a\x1C'V[\x90a\x14\xBCV[\x91\x01\x90a\x14\xDBV[\x90Pa\x15\"\x91\x92Pa\x1E$V[PPV[\x80T\x82\x10\x15a\x12}W_R` _ \x01\x90_\x90V[C\x81\x10\x15a\x15\xE0W`kT\x90_\x90[\x82\x82\x10a\x15\x87WPP\x80a\x15gWP_[`\x01`\x01`\xE0\x1B\x03\x16\x90V[_\x19\x81\x01\x90\x81\x11a\n\xD6Wa\x15}\x90`ka\x15&V[PT` \x1Ca\x15[V[\x90\x91a\x15\x9A\x81\x84\x18`\x01\x1C\x82\x85\x16a\x12\x91V[\x90\x82c\xFF\xFF\xFF\xFFa\x15\xAC\x84`ka\x15&V[PT\x16\x11\x15a\x15\xBEWP\x91[\x90a\x15JV[\x92P`\x01\x81\x01\x80\x91\x11\x15a\x15\xB8WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R\xFD[C\x81\x10\x15a\x15\xE0W`lT\x90_\x90[\x82\x82\x10a\x16\\WPP\x80a\x16FWP_\x90V[_\x19\x81\x01\x90\x81\x11a\n\xD6Wa\x15}\x90`la\x15&V[\x90\x91a\x16o\x81\x84\x18`\x01\x1C\x82\x85\x16a\x12\x91V[\x90\x82c\xFF\xFF\xFF\xFFa\x16\x81\x84`la\x15&V[PT\x16\x11\x15a\x16\x93WP\x91[\x90a\x163V[\x92P`\x01\x81\x01\x80\x91\x11\x15a\x16\x8DWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90C\x81\x10\x15a\x15\xE0W\x81T\x90_\x90[\x82\x82\x10a\x16\xEDWPP\x80a\x16\xD9WP_\x91\x90PV[_\x19\x81\x01\x90\x81\x11a\n\xD6Wa\x15}\x91a\x15&V[\x90\x91a\x17\0\x81\x84\x18`\x01\x1C\x82\x85\x16a\x12\x91V[\x90\x82c\xFF\xFF\xFF\xFFa\x17\x11\x84\x88a\x15&V[PT\x16\x11\x15a\x17#WP\x91[\x90a\x16\xC4V[\x92P`\x01\x81\x01\x80\x91\x11\x15a\x17\x1DWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`kT\x80a\x15gWP_\x90V[\x80T\x80a\x16\xD9WP_\x91\x90PV[\x90` \x91a\x17y\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x11\xD1V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x17\x99WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`lT\x90a\x17\xEB`la\x17RV[\x90\x82\x15\x15\x80a\x19oW[\x15a\x18cWa\x18\x03\x81a!\xCDV[\x91_\x19\x84\x01\x93\x84\x11a\n\xD6Wa\x18Y` \x93a\x18A\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x96`la\x15&V[P\x90c\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90` \x1B\x16\x91\x16\x17\x90UV[P`@Q\x90\x81R\xA1V[c\xFF\xFF\xFF\xFFC\x11a\x19\x1BWa\x18w\x81a!\xCDV[\x91`@Q\x93a\x18\x85\x85a\x0F\\V[Cc\xFF\xFF\xFF\xFF\x16\x85R`\x01`\x01`\xE0\x1B\x03\x90\x93\x16` \x85\x01\x90\x81R\x92`\x01`@\x1B\x81\x10\x15a\n\xF9W\x80`\x01a\x18\xBF\x92\x01`lU`la\x15&V[\x94\x90\x94a\x19\x08WQ\x92Qc\xFF\xFF\xFF\xFF\x90\x93\x16c\xFF\xFF\xFF\xFF\x19` \x94\x85\x1B\x16\x17\x90\x93U\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x92a\x18YV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[P_\x19\x83\x01\x83\x81\x11a\n\xD6Wa\x19\x8Bc\xFF\xFF\xFF\xFF\x91`la\x15&V[PT\x16C\x14a\x17\xF5V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`j` R`@\x90 \x90\x92\x91\x90a\x1A\x02\x90a\x17RV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x91\x16\x90\x82\x82\x14a\x1A]W\x80_R`j` Ra\x1A-\x83`@_ a\x1FRV[PP`@Q\x91\x82R\x7F\xD0a\x16\x82R\xF4As6X\xF0\x9EM\x8F[-\x99\x8E\xD4\xEF$\xA2\xBB\xFDl\xEC\xA5.\xA11P\x02` C\x93\xA4V[PPPV[\x15a\x1AiWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[a\x1A\xCB\x81a ;V[\x15a\x1C\0W`@Q\x91a\x1A\xDD\x83a\x0FAV[a\x1A\xE5a\x11\xF2V[\x83R`fT_`fU\x80a\x1B\xC6W[P_[\x82Q\x80Q\x82\x10\x15a\x1BsW\x81a\x1B\x0C\x91a\x12iV[Q\x90`fT\x91`\x01`@\x1B\x83\x10\x15a\n\xF9W`\x01\x83\x01`fU`fT\x83\x10\x15a\x12}W`f_R\x80Q` \x91\x90\x91\x01Q`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17_\x80Q` a#M\x839\x81Q\x91R\x90\x92\x01\x91\x90\x91U`\x01\x01a\x1A\xF7V[PP\x91a\x1B\xB3\x90a\x1B\xC1\x7F#\xAA\xD4\xE6\x17D\xEC\xE1d\x13\n\xA4\x15\xC1an\x80\x13k\x0F\x07p\xE5e\x89C\x8B\x90\xB2i&^\x93`@Q\x93\x84\x93`@\x85R`@\x85\x01\x90a\x10\xA7V[\x90\x83\x82\x03` \x85\x01Ra\x10\xA7V[\x03\x90\xA1V[`f_R_\x80Q` a#M\x839\x81Q\x91R\x01_\x80Q` a#M\x839\x81Q\x91R[\x81\x81\x10a\x1B\xF5WPa\x1A\xF4V[_\x81U`\x01\x01a\x1B\xE8V[c\xD1sWy`\xE0\x1B_R`\x04_\xFD[\x81\x81\x03\x92\x91_\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\n\xD6WV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R`m` R`@\x81 \x90\x92\x90\x83\x90a\x1CL\x90a\x17RV[\x80\x94\x84_R`n` R`\xFF`@_ T\x16\x15_\x14a\x1D\xC6Wa\x1Cp\x92\x93Pa\x1C\x0FV[\x92\x83\x15a\x1D\xC0W\x82_R`m` R`@_ \x90\x81T\x92a\x1C\x90\x83a\x17RV[\x84\x15\x15\x80a\x1D\x9BW[\x15a\x1C\xF4Wa\x1C\xA7_a!\xCDV[\x94_\x19\x81\x01\x90\x81\x11a\n\xD6W\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x95a\x18Aa\x1C\xE4\x92`@\x97a\x15&V[P[\x82Q\x91\x82R` \x82\x01R\xA2\x90V[c\xFF\xFF\xFF\xFFC\x11a\x19\x1BWa\x1D\x08_a!\xCDV[\x93`@Q\x95a\x1D\x16\x87a\x0F\\V[Cc\xFF\xFF\xFF\xFF\x16\x87R`\x01`\x01`\xE0\x1B\x03\x90\x95\x16` \x87\x01\x90\x81R\x94`\x01`@\x1B\x81\x10\x15a\n\xF9Wa\x1DM\x91`\x01\x82\x01\x81Ua\x15&V[\x95\x90\x95a\x19\x08WQ\x93Q` \x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x93\x90\x93\x17\x90\x93U\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x92`@\x92a\x1C\xE4V[P_\x19\x85\x01\x85\x81\x11a\n\xD6Wa\x1D\xB6c\xFF\xFF\xFF\xFF\x91\x86a\x15&V[PT\x16C\x14a\x1C\x99V[PPP\x90V[PPPa\x1D\xD2\x90a\x12\x9EV[\x91a\x1D\xDD\x81\x84a\x1C\x0FV[\x92\x83\x15a\x1D\xC0W\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x91`@\x91\x84_R`m` Ra\x1E\x1D\x81\x84_ a\x1FRV[PPa\x1C\xE6V[\x90a\x1E7a\x1E0a\x17EV[\x92\x83a\x14\xBCV[\x90`kTa\x1EE`ka\x17RV[\x90\x80\x15\x15\x80a\x1F,W[\x15a\x1E\xA8Wa\x1E]\x84a!\xCDV[_\x19\x82\x01\x91\x82\x11a\n\xD6Wa\x18Aa\x1Ev\x92`ka\x15&V[P\x7F\x86\xDC\xF8k\x12\xDF\xEE\xDE\xA7J\xE90\r\xBD\xAA\x19;\xCC\xE5\x80\x93i\xC8\x17~\xA2\xF4\xEA\xAAer\x9B`@\x80Q\x85\x81R\x84` \x82\x01R\xA1V[c\xFF\xFF\xFF\xFFC\x11a\x19\x1BWa\x1E\xBC\x84a!\xCDV[`@Q\x91a\x1E\xC9\x83a\x0F\\V[Cc\xFF\xFF\xFF\xFF\x16\x83R`\x01`\x01`\xE0\x1B\x03\x90\x91\x16` \x83\x01\x90\x81R\x90`\x01`@\x1B\x81\x10\x15a\n\xF9W\x80`\x01a\x1F\x03\x92\x01`kU`ka\x15&V[\x91\x90\x91a\x19\x08W\x91Q\x91Q` \x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1EvV[P_\x19\x81\x01\x81\x81\x11a\n\xD6Wa\x1FHc\xFF\xFF\xFF\xFF\x91`ka\x15&V[PT\x16C\x14a\x1EOV[\x91\x90\x91\x80Ta\x1F`\x82a\x17RV[\x91\x81\x15\x15\x80a \x16W[\x15a\x1F\x95Wa\x1Fx\x85a!\xCDV[\x90_\x19\x83\x01\x92\x83\x11a\n\xD6Wa\x1F\x91\x92a\x18A\x91a\x15&V[\x91\x90V[c\xFF\xFF\xFF\xFFC\x11a\x19\x1BWa\x1F\xA9\x85a!\xCDV[\x90`@Q\x92a\x1F\xB7\x84a\x0F\\V[Cc\xFF\xFF\xFF\xFF\x16\x84R`\x01`\x01`\xE0\x1B\x03\x90\x92\x16` \x84\x01\x90\x81R\x91`\x01`@\x1B\x81\x10\x15a\n\xF9Wa\x1F\xEE\x91`\x01\x82\x01\x81Ua\x15&V[\x91\x90\x91a\x19\x08W\x91Q\x91Q` \x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x91\x90V[P_\x19\x82\x01\x82\x81\x11a\n\xD6Wa 1c\xFF\xFF\xFF\xFF\x91\x83a\x15&V[PT\x16C\x14a\x1FjV[Q\x90_\x80\x80[\x84Q\x82\x10\x15a \x9EW`\x01`\x01`\xA0\x1B\x03a \\\x83\x87a\x12iV[QQ\x16\x90`\x01`\x01`\xA0\x1B\x03\x16\x81\x11\x15a\x0E)Wa \x95`\x01\x91\x93`\x01`\x01``\x1B\x03` a \x8B\x86\x8Aa\x12iV[Q\x01Q\x16\x90a\x12\x91V[\x91\x01\x90\x91a AV[P\x91\x92Pa'\x10\x14\x90Pa \xB0W_\x90V[`\x01\x90V[\x91\x90\x91a \xC2\x82\x84a\"<V[`\x05\x81\x10\x15a!\xB9W\x15\x90\x81a!\xA3W[Pa!\x9BW_\x92a!\ra!\x1B\x85\x94`@Q\x92\x83\x91` \x83\x01\x95c\x0B\x13]?`\xE1\x1B\x87R`$\x84\x01R`@`D\x84\x01R`d\x83\x01\x90a\x17`V[\x03`\x1F\x19\x81\x01\x83R\x82a\x0FwV[Q\x91Z\xFA=\x15a!\x94W=a!/\x81a\x10FV[\x90a!=`@Q\x92\x83a\x0FwV[\x81R=_` \x83\x01>[\x81a!\x88W[\x81a!VWP\x90V[\x90P` \x81\x80Q\x81\x01\x03\x12a\n\xD2W` \x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x90\x81\x90\x03a\n\xD2Wc\x0B\x13]?`\xE1\x1B\x14\x90V[\x80Q` \x14\x91Pa!MV[``a!GV[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_a \xD3V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x01`\x01`\xE0\x1B\x03\x81\x11a!\xE7W`\x01`\x01`\xE0\x1B\x03\x16\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[\x81Q`A\x81\x03a\"hWP\x90a\"d\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a\"\xAAV[\x90\x91V[`@\x03a\"\xA1W`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a\n\xD6Wa\"d\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90a\"\xAAV[PP_\x90`\x02\x90V[\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a#AW`\xFF\x16\x90`\x1B\x82\x14\x15\x80a#6W[a#+W` \x93_\x93`\x80\x93`@Q\x93\x84R\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\n\xC7W_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a##W\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x82\x14\x15a\"\xE1V[PPPP_\x90`\x03\x90V\xFEFP\x18y\xB8\xCA\x85%\xE8\xC2\xFDQ\x9E/\xBF\xCF\xA2\xEB\xEA&P\x12\x94\xAA\x02\xCB\xFC\xFB\x12\xE9CT\xA2dipfsX\"\x12 \xE3\xC3\xC7\x95_\x19\xF8\x94\xDE\xE7m\x8A\x17!\xDF\xC8if\x03\xF5\xD6\x11\xCA\xC1\x94\xBA\xF3=\x16\xD0k\xB3dsolcC\0\x08\x1A\x003",
    );
    /**```solidity
struct Quorum { StrategyParams[] strategies; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Quorum {
        pub strategies: alloy::sol_types::private::Vec<
            <StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<StrategyParams>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Quorum> for UnderlyingRustTuple<'_> {
            fn from(value: Quorum) -> Self {
                (value.strategies,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Quorum {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { strategies: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Quorum {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Quorum {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Quorum {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Quorum {
            const NAME: &'static str = "Quorum";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Quorum(StrategyParams[] strategies)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <StrategyParams as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <StrategyParams as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                <alloy::sol_types::sol_data::Array<
                    StrategyParams,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                    .0
                    .to_vec()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Quorum {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        StrategyParams,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Array<
                    StrategyParams,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct StrategyParams { address strategy; uint96 multiplier; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyParams {
        pub strategy: alloy::sol_types::private::Address,
        pub multiplier: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U96,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StrategyParams> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyParams) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    multiplier: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.multiplier),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StrategyParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for StrategyParams {
            const NAME: &'static str = "StrategyParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyParams(address strategy,uint96 multiplier)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.multiplier)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StrategyParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategy,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.multiplier,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategy,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.multiplier,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**Custom error with signature `InsufficientSignedStake()` and selector `0xe121632f`.
```solidity
error InsufficientSignedStake();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientSignedStake {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientSignedStake> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientSignedStake) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientSignedStake {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientSignedStake {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientSignedStake()";
            const SELECTOR: [u8; 4] = [225u8, 33u8, 99u8, 47u8];
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
    /**Custom error with signature `InsufficientWeight()` and selector `0xa8792fd1`.
```solidity
error InsufficientWeight();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientWeight {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientWeight> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientWeight) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientWeight {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientWeight {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientWeight()";
            const SELECTOR: [u8; 4] = [168u8, 121u8, 47u8, 209u8];
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
    /**Custom error with signature `InvalidLength()` and selector `0x947d5a84`.
```solidity
error InvalidLength();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidLength {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidLength> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidLength()";
            const SELECTOR: [u8; 4] = [148u8, 125u8, 90u8, 132u8];
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
    /**Custom error with signature `InvalidQuorum()` and selector `0xd1735779`.
```solidity
error InvalidQuorum();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidQuorum {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidQuorum> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidQuorum) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidQuorum {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidQuorum {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidQuorum()";
            const SELECTOR: [u8; 4] = [209u8, 115u8, 87u8, 121u8];
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
    /**Custom error with signature `InvalidReferenceBlock()` and selector `0xe64f180f`.
```solidity
error InvalidReferenceBlock();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidReferenceBlock {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidReferenceBlock> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidReferenceBlock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidReferenceBlock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidReferenceBlock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidReferenceBlock()";
            const SELECTOR: [u8; 4] = [230u8, 79u8, 24u8, 15u8];
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
    /**Custom error with signature `InvalidSignature()` and selector `0x8baa579f`.
```solidity
error InvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignature {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignature()";
            const SELECTOR: [u8; 4] = [139u8, 170u8, 87u8, 159u8];
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
    /**Custom error with signature `InvalidSignedWeight()` and selector `0x960b41ee`.
```solidity
error InvalidSignedWeight();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignedWeight {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidSignedWeight> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignedWeight) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignedWeight {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignedWeight {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignedWeight()";
            const SELECTOR: [u8; 4] = [150u8, 11u8, 65u8, 238u8];
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
    /**Custom error with signature `InvalidThreshold()` and selector `0xaabd5a09`.
```solidity
error InvalidThreshold();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidThreshold {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidThreshold()";
            const SELECTOR: [u8; 4] = [170u8, 189u8, 90u8, 9u8];
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
    /**Custom error with signature `LengthMismatch()` and selector `0xff633a38`.
```solidity
error LengthMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LengthMismatch {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<LengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: LengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LengthMismatch()";
            const SELECTOR: [u8; 4] = [255u8, 99u8, 58u8, 56u8];
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
    /**Custom error with signature `MustUpdateAllOperators()` and selector `0x2d3df6b6`.
```solidity
error MustUpdateAllOperators();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MustUpdateAllOperators {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MustUpdateAllOperators> for UnderlyingRustTuple<'_> {
            fn from(value: MustUpdateAllOperators) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MustUpdateAllOperators {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MustUpdateAllOperators {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MustUpdateAllOperators()";
            const SELECTOR: [u8; 4] = [45u8, 61u8, 246u8, 182u8];
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
    /**Custom error with signature `NotSorted()` and selector `0xba50f911`.
```solidity
error NotSorted();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotSorted {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotSorted> for UnderlyingRustTuple<'_> {
            fn from(value: NotSorted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotSorted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotSorted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotSorted()";
            const SELECTOR: [u8; 4] = [186u8, 80u8, 249u8, 17u8];
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
    /**Custom error with signature `OperatorAlreadyRegistered()` and selector `0x42ee68b5`.
```solidity
error OperatorAlreadyRegistered();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorAlreadyRegistered {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorAlreadyRegistered>
        for UnderlyingRustTuple<'_> {
            fn from(value: OperatorAlreadyRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OperatorAlreadyRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorAlreadyRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorAlreadyRegistered()";
            const SELECTOR: [u8; 4] = [66u8, 238u8, 104u8, 181u8];
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
    /**Custom error with signature `OperatorNotRegistered()` and selector `0x25ec6c1f`.
```solidity
error OperatorNotRegistered();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorNotRegistered {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorNotRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorNotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorNotRegistered()";
            const SELECTOR: [u8; 4] = [37u8, 236u8, 108u8, 31u8];
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
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
```solidity
event Initialized(uint8 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                )
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `MinimumWeightUpdated(uint256,uint256)` and selector `0x713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f`.
```solidity
event MinimumWeightUpdated(uint256 _old, uint256 _new);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MinimumWeightUpdated {
        #[allow(missing_docs)]
        pub _old: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _new: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for MinimumWeightUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MinimumWeightUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                113u8,
                60u8,
                165u8,
                59u8,
                136u8,
                214u8,
                235u8,
                99u8,
                245u8,
                177u8,
                133u8,
                76u8,
                184u8,
                203u8,
                221u8,
                115u8,
                110u8,
                197u8,
                30u8,
                218u8,
                34u8,
                94u8,
                70u8,
                121u8,
                26u8,
                169u8,
                41u8,
                139u8,
                1u8,
                96u8,
                100u8,
                143u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _old: data.0, _new: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._old),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._new),
                )
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MinimumWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MinimumWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MinimumWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorDeregistered(address,address)` and selector `0x31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed580`.
```solidity
event OperatorDeregistered(address indexed _operator, address indexed _avs);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorDeregistered {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _avs: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorDeregistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorDeregistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                49u8,
                224u8,
                173u8,
                254u8,
                199u8,
                27u8,
                204u8,
                238u8,
                55u8,
                182u8,
                232u8,
                58u8,
                144u8,
                194u8,
                254u8,
                219u8,
                23u8,
                216u8,
                241u8,
                105u8,
                63u8,
                238u8,
                134u8,
                60u8,
                71u8,
                113u8,
                231u8,
                191u8,
                226u8,
                174u8,
                213u8,
                128u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _operator: topics.1,
                    _avs: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self._operator.clone(), self._avs.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorDeregistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorDeregistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorDeregistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorRegistered(address,address)` and selector `0xa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c1`.
```solidity
event OperatorRegistered(address indexed _operator, address indexed _avs);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorRegistered {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _avs: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorRegistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8,
                83u8,
                219u8,
                97u8,
                42u8,
                245u8,
                158u8,
                85u8,
                33u8,
                214u8,
                171u8,
                146u8,
                132u8,
                220u8,
                62u8,
                45u8,
                6u8,
                175u8,
                40u8,
                110u8,
                177u8,
                177u8,
                183u8,
                183u8,
                113u8,
                252u8,
                228u8,
                113u8,
                108u8,
                25u8,
                242u8,
                193u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _operator: topics.1,
                    _avs: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self._operator.clone(), self._avs.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorRegistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorRegistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorRegistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorWeightUpdated(address,uint256,uint256)` and selector `0x88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe594`.
```solidity
event OperatorWeightUpdated(address indexed _operator, uint256 oldWeight, uint256 newWeight);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorWeightUpdated {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorWeightUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorWeightUpdated(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                136u8,
                119u8,
                13u8,
                200u8,
                98u8,
                228u8,
                122u8,
                126u8,
                213u8,
                134u8,
                144u8,
                120u8,
                87u8,
                235u8,
                27u8,
                117u8,
                228u8,
                197u8,
                255u8,
                200u8,
                183u8,
                7u8,
                199u8,
                238u8,
                16u8,
                235u8,
                116u8,
                214u8,
                136u8,
                95u8,
                229u8,
                148u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _operator: topics.1,
                    oldWeight: data.0,
                    newWeight: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldWeight),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newWeight),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self._operator.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `QuorumUpdated(((address,uint96)[]),((address,uint96)[]))` and selector `0x23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e`.
```solidity
event QuorumUpdated(Quorum _old, Quorum _new);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumUpdated {
        #[allow(missing_docs)]
        pub _old: <Quorum as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _new: <Quorum as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for QuorumUpdated {
            type DataTuple<'a> = (Quorum, Quorum);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "QuorumUpdated(((address,uint96)[]),((address,uint96)[]))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                35u8,
                170u8,
                212u8,
                230u8,
                23u8,
                68u8,
                236u8,
                225u8,
                100u8,
                19u8,
                10u8,
                164u8,
                21u8,
                193u8,
                97u8,
                110u8,
                128u8,
                19u8,
                107u8,
                15u8,
                7u8,
                112u8,
                229u8,
                101u8,
                137u8,
                67u8,
                139u8,
                144u8,
                178u8,
                105u8,
                38u8,
                94u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _old: data.0, _new: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._old),
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._new),
                )
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for QuorumUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SigningKeyUpdate(address,uint256,address,address)` and selector `0xd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea1315002`.
```solidity
event SigningKeyUpdate(address indexed operator, uint256 indexed updateBlock, address indexed newSigningKey, address oldSigningKey);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SigningKeyUpdate {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub updateBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newSigningKey: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldSigningKey: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SigningKeyUpdate {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SigningKeyUpdate(address,uint256,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                208u8,
                97u8,
                22u8,
                130u8,
                82u8,
                244u8,
                65u8,
                115u8,
                54u8,
                88u8,
                240u8,
                158u8,
                77u8,
                143u8,
                91u8,
                45u8,
                153u8,
                142u8,
                212u8,
                239u8,
                36u8,
                162u8,
                187u8,
                253u8,
                108u8,
                236u8,
                165u8,
                46u8,
                161u8,
                49u8,
                80u8,
                2u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    updateBlock: topics.2,
                    newSigningKey: topics.3,
                    oldSigningKey: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.oldSigningKey,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.operator.clone(),
                    self.updateBlock.clone(),
                    self.newSigningKey.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.updateBlock);
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newSigningKey,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SigningKeyUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SigningKeyUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SigningKeyUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ThresholdWeightUpdated(uint256)` and selector `0x9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b`.
```solidity
event ThresholdWeightUpdated(uint256 _thresholdWeight);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ThresholdWeightUpdated {
        #[allow(missing_docs)]
        pub _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ThresholdWeightUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ThresholdWeightUpdated(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                147u8,
                36u8,
                247u8,
                229u8,
                167u8,
                192u8,
                40u8,
                136u8,
                8u8,
                166u8,
                52u8,
                204u8,
                222u8,
                68u8,
                184u8,
                233u8,
                121u8,
                103u8,
                100u8,
                116u8,
                178u8,
                46u8,
                41u8,
                238u8,
                157u8,
                213u8,
                105u8,
                181u8,
                94u8,
                121u8,
                26u8,
                75u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _thresholdWeight: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._thresholdWeight),
                )
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ThresholdWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ThresholdWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ThresholdWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TotalWeightUpdated(uint256,uint256)` and selector `0x86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b`.
```solidity
event TotalWeightUpdated(uint256 oldTotalWeight, uint256 newTotalWeight);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TotalWeightUpdated {
        #[allow(missing_docs)]
        pub oldTotalWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newTotalWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TotalWeightUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TotalWeightUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                134u8,
                220u8,
                248u8,
                107u8,
                18u8,
                223u8,
                238u8,
                222u8,
                167u8,
                74u8,
                233u8,
                48u8,
                13u8,
                189u8,
                170u8,
                25u8,
                59u8,
                204u8,
                229u8,
                128u8,
                147u8,
                105u8,
                200u8,
                23u8,
                126u8,
                162u8,
                244u8,
                234u8,
                170u8,
                101u8,
                114u8,
                155u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldTotalWeight: data.0,
                    newTotalWeight: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldTotalWeight),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newTotalWeight),
                )
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TotalWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TotalWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TotalWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `UpdateMinimumWeight(uint256,uint256)` and selector `0x1ea42186b305fa37310450d9fb87ea1e8f0c7f447e771479e3b27634bfe84dc1`.
```solidity
event UpdateMinimumWeight(uint256 oldMinimumWeight, uint256 newMinimumWeight);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UpdateMinimumWeight {
        #[allow(missing_docs)]
        pub oldMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for UpdateMinimumWeight {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UpdateMinimumWeight(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                30u8,
                164u8,
                33u8,
                134u8,
                179u8,
                5u8,
                250u8,
                55u8,
                49u8,
                4u8,
                80u8,
                217u8,
                251u8,
                135u8,
                234u8,
                30u8,
                143u8,
                12u8,
                127u8,
                68u8,
                126u8,
                119u8,
                20u8,
                121u8,
                227u8,
                178u8,
                118u8,
                52u8,
                191u8,
                232u8,
                77u8,
                193u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldMinimumWeight: data.0,
                    newMinimumWeight: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldMinimumWeight),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMinimumWeight),
                )
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UpdateMinimumWeight {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpdateMinimumWeight> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UpdateMinimumWeight) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _delegationManager);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _delegationManager: alloy::sol_types::private::Address,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
                    (value._delegationManager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _delegationManager: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                        &self._delegationManager,
                    ),
                )
            }
        }
    };
    /**Function with signature `deregisterOperator()` and selector `0x857dc190`.
```solidity
function deregisterOperator() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorCall {}
    ///Container type for the return parameters of the [`deregisterOperator()`](deregisterOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperator()";
            const SELECTOR: [u8; 4] = [133u8, 125u8, 193u8, 144u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointOperatorWeight(address)` and selector `0x3b242e4a`.
```solidity
function getLastCheckpointOperatorWeight(address _operator) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointOperatorWeightCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getLastCheckpointOperatorWeight(address)`](getLastCheckpointOperatorWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointOperatorWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointOperatorWeightCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointOperatorWeightCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointOperatorWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointOperatorWeightReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointOperatorWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointOperatorWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointOperatorWeightCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointOperatorWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointOperatorWeight(address)";
            const SELECTOR: [u8; 4] = [59u8, 36u8, 46u8, 74u8];
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
                        &self._operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointThresholdWeight()` and selector `0xb933fa74`.
```solidity
function getLastCheckpointThresholdWeight() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightCall {}
    ///Container type for the return parameters of the [`getLastCheckpointThresholdWeight()`](getLastCheckpointThresholdWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointThresholdWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointThresholdWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointThresholdWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointThresholdWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointThresholdWeight()";
            const SELECTOR: [u8; 4] = [185u8, 51u8, 250u8, 116u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointThresholdWeightAtBlock(uint32)` and selector `0x1e4cd85e`.
```solidity
function getLastCheckpointThresholdWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightAtBlockCall {
        pub _blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getLastCheckpointThresholdWeightAtBlock(uint32)`](getLastCheckpointThresholdWeightAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightAtBlockReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightAtBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightAtBlockCall) -> Self {
                    (value._blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointThresholdWeightAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _blockNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightAtBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointThresholdWeightAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointThresholdWeightAtBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointThresholdWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointThresholdWeightAtBlock(uint32)";
            const SELECTOR: [u8; 4] = [30u8, 76u8, 216u8, 94u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._blockNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointTotalWeight()` and selector `0x314f3a49`.
```solidity
function getLastCheckpointTotalWeight() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightCall {}
    ///Container type for the return parameters of the [`getLastCheckpointTotalWeight()`](getLastCheckpointTotalWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointTotalWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointTotalWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointTotalWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointTotalWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointTotalWeight()";
            const SELECTOR: [u8; 4] = [49u8, 79u8, 58u8, 73u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointTotalWeightAtBlock(uint32)` and selector `0x0dba3394`.
```solidity
function getLastCheckpointTotalWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightAtBlockCall {
        pub _blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getLastCheckpointTotalWeightAtBlock(uint32)`](getLastCheckpointTotalWeightAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightAtBlockReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightAtBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightAtBlockCall) -> Self {
                    (value._blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointTotalWeightAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _blockNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightAtBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointTotalWeightAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointTotalWeightAtBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointTotalWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointTotalWeightAtBlock(uint32)";
            const SELECTOR: [u8; 4] = [13u8, 186u8, 51u8, 148u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._blockNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastestOperatorSigningKey(address)` and selector `0xcdcd3581`.
```solidity
function getLastestOperatorSigningKey(address _operator) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastestOperatorSigningKeyCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getLastestOperatorSigningKey(address)`](getLastestOperatorSigningKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastestOperatorSigningKeyReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastestOperatorSigningKeyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastestOperatorSigningKeyCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastestOperatorSigningKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastestOperatorSigningKeyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastestOperatorSigningKeyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastestOperatorSigningKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastestOperatorSigningKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastestOperatorSigningKeyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastestOperatorSigningKey(address)";
            const SELECTOR: [u8; 4] = [205u8, 205u8, 53u8, 129u8];
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
                        &self._operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorSigningKeyAtBlock(address,uint256)` and selector `0x5e1042e8`.
```solidity
function getOperatorSigningKeyAtBlock(address _operator, uint256 _blockNumber) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSigningKeyAtBlockCall {
        pub _operator: alloy::sol_types::private::Address,
        pub _blockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getOperatorSigningKeyAtBlock(address,uint256)`](getOperatorSigningKeyAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSigningKeyAtBlockReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorSigningKeyAtBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSigningKeyAtBlockCall) -> Self {
                    (value._operator, value._blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSigningKeyAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _operator: tuple.0,
                        _blockNumber: tuple.1,
                    }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorSigningKeyAtBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSigningKeyAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSigningKeyAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSigningKeyAtBlockCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSigningKeyAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorSigningKeyAtBlock(address,uint256)";
            const SELECTOR: [u8; 4] = [94u8, 16u8, 66u8, 232u8];
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
                        &self._operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._blockNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorWeight(address)` and selector `0x98ec1ac9`.
```solidity
function getOperatorWeight(address _operator) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorWeight(address)`](getOperatorWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorWeightCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorWeight(address)";
            const SELECTOR: [u8; 4] = [152u8, 236u8, 26u8, 201u8];
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
                        &self._operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorWeightAtBlock(address,uint32)` and selector `0x955f2d90`.
```solidity
function getOperatorWeightAtBlock(address _operator, uint32 _blockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightAtBlockCall {
        pub _operator: alloy::sol_types::private::Address,
        pub _blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getOperatorWeightAtBlock(address,uint32)`](getOperatorWeightAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightAtBlockReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightAtBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightAtBlockCall) -> Self {
                    (value._operator, value._blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorWeightAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _operator: tuple.0,
                        _blockNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightAtBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorWeightAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorWeightAtBlockCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorWeightAtBlock(address,uint32)";
            const SELECTOR: [u8; 4] = [149u8, 95u8, 45u8, 144u8];
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
                        &self._operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._blockNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize(address,uint256,((address,uint96)[]))` and selector `0xab118995`.
```solidity
function initialize(address _serviceManager, uint256 _thresholdWeight, Quorum memory _quorum) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _serviceManager: alloy::sol_types::private::Address,
        pub _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
        pub _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`initialize(address,uint256,((address,uint96)[]))`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                Quorum,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                <Quorum as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._serviceManager, value._thresholdWeight, value._quorum)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _serviceManager: tuple.0,
                        _thresholdWeight: tuple.1,
                        _quorum: tuple.2,
                    }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                Quorum,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,uint256,((address,uint96)[]))";
            const SELECTOR: [u8; 4] = [171u8, 17u8, 137u8, 149u8];
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
                        &self._serviceManager,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._thresholdWeight),
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._quorum),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`.
```solidity
function isValidSignature(bytes32 _dataHash, bytes memory _signatureData) external view returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidSignatureCall {
        pub _dataHash: alloy::sol_types::private::FixedBytes<32>,
        pub _signatureData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`isValidSignature(bytes32,bytes)`](isValidSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidSignatureReturn {
        pub _0: alloy::sol_types::private::FixedBytes<4>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidSignatureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureCall) -> Self {
                    (value._dataHash, value._signatureData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _dataHash: tuple.0,
                        _signatureData: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidSignatureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isValidSignatureCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isValidSignatureReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isValidSignature(bytes32,bytes)";
            const SELECTOR: [u8; 4] = [22u8, 38u8, 186u8, 126u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._dataHash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._signatureData,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `minimumWeight()` and selector `0x40bf2fb7`.
```solidity
function minimumWeight() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumWeightCall {}
    ///Container type for the return parameters of the [`minimumWeight()`](minimumWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minimumWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: minimumWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minimumWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: minimumWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minimumWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = minimumWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minimumWeight()";
            const SELECTOR: [u8; 4] = [64u8, 191u8, 47u8, 183u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `operatorRegistered(address)` and selector `0xec7fbb31`.
```solidity
function operatorRegistered(address _operator) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorRegisteredCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`operatorRegistered(address)`](operatorRegisteredCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorRegisteredReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorRegisteredCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorRegisteredCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorRegisteredCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorRegisteredReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorRegisteredReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorRegisteredReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorRegisteredCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorRegisteredReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorRegistered(address)";
            const SELECTOR: [u8; 4] = [236u8, 127u8, 187u8, 49u8];
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
                        &self._operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `quorum()` and selector `0x1703a018`.
```solidity
function quorum() external view returns (Quorum memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumCall {}
    ///Container type for the return parameters of the [`quorum()`](quorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumReturn {
        pub _0: <Quorum as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: quorumCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Quorum,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Quorum as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: quorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumReturn;
            type ReturnTuple<'a> = (Quorum,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorum()";
            const SELECTOR: [u8; 4] = [23u8, 3u8, 160u8, 24u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `registerOperatorWithSignature((bytes,bytes32,uint256),address)` and selector `0x3d5611f6`.
```solidity
function registerOperatorWithSignature(ISignatureUtils.SignatureWithSaltAndExpiry memory _operatorSignature, address _signingKey) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorWithSignatureCall {
        pub _operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        pub _signingKey: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`registerOperatorWithSignature((bytes,bytes32,uint256),address)`](registerOperatorWithSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorWithSignatureReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                ISignatureUtils::SignatureWithSaltAndExpiry,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorWithSignatureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorWithSignatureCall) -> Self {
                    (value._operatorSignature, value._signingKey)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorWithSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _operatorSignature: tuple.0,
                        _signingKey: tuple.1,
                    }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorWithSignatureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorWithSignatureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorWithSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorWithSignatureCall {
            type Parameters<'a> = (
                ISignatureUtils::SignatureWithSaltAndExpiry,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorWithSignatureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperatorWithSignature((bytes,bytes32,uint256),address)";
            const SELECTOR: [u8; 4] = [61u8, 86u8, 17u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ISignatureUtils::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self._operatorSignature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._signingKey,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
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
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateMinimumWeight(uint256,address[])` and selector `0x696255be`.
```solidity
function updateMinimumWeight(uint256 _newMinimumWeight, address[] memory _operators) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateMinimumWeightCall {
        pub _newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
        pub _operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`updateMinimumWeight(uint256,address[])`](updateMinimumWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateMinimumWeightReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateMinimumWeightCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateMinimumWeightCall) -> Self {
                    (value._newMinimumWeight, value._operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateMinimumWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _newMinimumWeight: tuple.0,
                        _operators: tuple.1,
                    }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateMinimumWeightReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateMinimumWeightReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateMinimumWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateMinimumWeightCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateMinimumWeightReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateMinimumWeight(uint256,address[])";
            const SELECTOR: [u8; 4] = [105u8, 98u8, 85u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._newMinimumWeight),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._operators),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateOperatorSigningKey(address)` and selector `0x743c31f4`.
```solidity
function updateOperatorSigningKey(address _newSigningKey) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorSigningKeyCall {
        pub _newSigningKey: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`updateOperatorSigningKey(address)`](updateOperatorSigningKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorSigningKeyReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorSigningKeyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorSigningKeyCall) -> Self {
                    (value._newSigningKey,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorSigningKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _newSigningKey: tuple.0 }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorSigningKeyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorSigningKeyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorSigningKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorSigningKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorSigningKeyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorSigningKey(address)";
            const SELECTOR: [u8; 4] = [116u8, 60u8, 49u8, 244u8];
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
                        &self._newSigningKey,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateOperators(address[])` and selector `0x00cf2ab5`.
```solidity
function updateOperators(address[] memory _operators) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsCall {
        pub _operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`updateOperators(address[])`](updateOperatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsCall) -> Self {
                    (value._operators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operators: tuple.0 }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperators(address[])";
            const SELECTOR: [u8; 4] = [0u8, 207u8, 42u8, 181u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._operators),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateOperatorsForQuorum(address[][],bytes)` and selector `0x5140a548`.
```solidity
function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumCall {
        pub operatorsPerQuorum: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        >,
        pub _1: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`updateOperatorsForQuorum(address[][],bytes)`](updateOperatorsForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                >,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsForQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumCall) -> Self {
                    (value.operatorsPerQuorum, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorsForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorsPerQuorum: tuple.0,
                        _1: tuple.1,
                    }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsForQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorsForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsForQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorsForQuorum(address[][],bytes)";
            const SELECTOR: [u8; 4] = [81u8, 64u8, 165u8, 72u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Address,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorsPerQuorum),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateQuorumConfig(((address,uint96)[]),address[])` and selector `0xdec5d1f6`.
```solidity
function updateQuorumConfig(Quorum memory _quorum, address[] memory _operators) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateQuorumConfigCall {
        pub _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
        pub _operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`updateQuorumConfig(((address,uint96)[]),address[])`](updateQuorumConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateQuorumConfigReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                Quorum,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Quorum as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateQuorumConfigCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateQuorumConfigCall) -> Self {
                    (value._quorum, value._operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateQuorumConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _quorum: tuple.0,
                        _operators: tuple.1,
                    }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateQuorumConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateQuorumConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateQuorumConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateQuorumConfigCall {
            type Parameters<'a> = (
                Quorum,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateQuorumConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateQuorumConfig(((address,uint96)[]),address[])";
            const SELECTOR: [u8; 4] = [222u8, 197u8, 209u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._quorum),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._operators),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateStakeThreshold(uint256)` and selector `0x5ef53329`.
```solidity
function updateStakeThreshold(uint256 _thresholdWeight) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateStakeThresholdCall {
        pub _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateStakeThreshold(uint256)`](updateStakeThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateStakeThresholdReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateStakeThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateStakeThresholdCall) -> Self {
                    (value._thresholdWeight,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateStakeThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _thresholdWeight: tuple.0 }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateStakeThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateStakeThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateStakeThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateStakeThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateStakeThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateStakeThreshold(uint256)";
            const SELECTOR: [u8; 4] = [94u8, 245u8, 51u8, 41u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._thresholdWeight),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`ECDSAStakeRegistry`](self) function calls.
    pub enum ECDSAStakeRegistryCalls {
        deregisterOperator(deregisterOperatorCall),
        getLastCheckpointOperatorWeight(getLastCheckpointOperatorWeightCall),
        getLastCheckpointThresholdWeight(getLastCheckpointThresholdWeightCall),
        getLastCheckpointThresholdWeightAtBlock(
            getLastCheckpointThresholdWeightAtBlockCall,
        ),
        getLastCheckpointTotalWeight(getLastCheckpointTotalWeightCall),
        getLastCheckpointTotalWeightAtBlock(getLastCheckpointTotalWeightAtBlockCall),
        getLastestOperatorSigningKey(getLastestOperatorSigningKeyCall),
        getOperatorSigningKeyAtBlock(getOperatorSigningKeyAtBlockCall),
        getOperatorWeight(getOperatorWeightCall),
        getOperatorWeightAtBlock(getOperatorWeightAtBlockCall),
        initialize(initializeCall),
        isValidSignature(isValidSignatureCall),
        minimumWeight(minimumWeightCall),
        operatorRegistered(operatorRegisteredCall),
        owner(ownerCall),
        quorum(quorumCall),
        registerOperatorWithSignature(registerOperatorWithSignatureCall),
        renounceOwnership(renounceOwnershipCall),
        transferOwnership(transferOwnershipCall),
        updateMinimumWeight(updateMinimumWeightCall),
        updateOperatorSigningKey(updateOperatorSigningKeyCall),
        updateOperators(updateOperatorsCall),
        updateOperatorsForQuorum(updateOperatorsForQuorumCall),
        updateQuorumConfig(updateQuorumConfigCall),
        updateStakeThreshold(updateStakeThresholdCall),
    }
    #[automatically_derived]
    impl ECDSAStakeRegistryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 207u8, 42u8, 181u8],
            [13u8, 186u8, 51u8, 148u8],
            [22u8, 38u8, 186u8, 126u8],
            [23u8, 3u8, 160u8, 24u8],
            [30u8, 76u8, 216u8, 94u8],
            [49u8, 79u8, 58u8, 73u8],
            [59u8, 36u8, 46u8, 74u8],
            [61u8, 86u8, 17u8, 246u8],
            [64u8, 191u8, 47u8, 183u8],
            [81u8, 64u8, 165u8, 72u8],
            [94u8, 16u8, 66u8, 232u8],
            [94u8, 245u8, 51u8, 41u8],
            [105u8, 98u8, 85u8, 190u8],
            [113u8, 80u8, 24u8, 166u8],
            [116u8, 60u8, 49u8, 244u8],
            [133u8, 125u8, 193u8, 144u8],
            [141u8, 165u8, 203u8, 91u8],
            [149u8, 95u8, 45u8, 144u8],
            [152u8, 236u8, 26u8, 201u8],
            [171u8, 17u8, 137u8, 149u8],
            [185u8, 51u8, 250u8, 116u8],
            [205u8, 205u8, 53u8, 129u8],
            [222u8, 197u8, 209u8, 246u8],
            [236u8, 127u8, 187u8, 49u8],
            [242u8, 253u8, 227u8, 139u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ECDSAStakeRegistryCalls {
        const NAME: &'static str = "ECDSAStakeRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::deregisterOperator(_) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointOperatorWeight(_) => {
                    <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointThresholdWeight(_) => {
                    <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointThresholdWeightAtBlock(_) => {
                    <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointTotalWeight(_) => {
                    <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointTotalWeightAtBlock(_) => {
                    <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastestOperatorSigningKey(_) => {
                    <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorSigningKeyAtBlock(_) => {
                    <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorWeight(_) => {
                    <getOperatorWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorWeightAtBlock(_) => {
                    <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isValidSignature(_) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minimumWeight(_) => {
                    <minimumWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorRegistered(_) => {
                    <operatorRegisteredCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::quorum(_) => <quorumCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::registerOperatorWithSignature(_) => {
                    <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateMinimumWeight(_) => {
                    <updateMinimumWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorSigningKey(_) => {
                    <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperators(_) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorsForQuorum(_) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateQuorumConfig(_) => {
                    <updateQuorumConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateStakeThreshold(_) => {
                    <updateStakeThresholdCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls>] = &[
                {
                    fn updateOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateOperators)
                    }
                    updateOperators
                },
                {
                    fn getLastCheckpointTotalWeightAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointTotalWeightAtBlock,
                            )
                    }
                    getLastCheckpointTotalWeightAtBlock
                },
                {
                    fn isValidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <isValidSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::isValidSignature)
                    }
                    isValidSignature
                },
                {
                    fn quorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <quorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::quorum)
                    }
                    quorum
                },
                {
                    fn getLastCheckpointThresholdWeightAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointThresholdWeightAtBlock,
                            )
                    }
                    getLastCheckpointThresholdWeightAtBlock
                },
                {
                    fn getLastCheckpointTotalWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getLastCheckpointTotalWeight)
                    }
                    getLastCheckpointTotalWeight
                },
                {
                    fn getLastCheckpointOperatorWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointOperatorWeight,
                            )
                    }
                    getLastCheckpointOperatorWeight
                },
                {
                    fn registerOperatorWithSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::registerOperatorWithSignature)
                    }
                    registerOperatorWithSignature
                },
                {
                    fn minimumWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <minimumWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::minimumWeight)
                    }
                    minimumWeight
                },
                {
                    fn updateOperatorsForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateOperatorsForQuorum)
                    }
                    updateOperatorsForQuorum
                },
                {
                    fn getOperatorSigningKeyAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getOperatorSigningKeyAtBlock)
                    }
                    getOperatorSigningKeyAtBlock
                },
                {
                    fn updateStakeThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateStakeThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateStakeThreshold)
                    }
                    updateStakeThreshold
                },
                {
                    fn updateMinimumWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateMinimumWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateMinimumWeight)
                    }
                    updateMinimumWeight
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn updateOperatorSigningKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateOperatorSigningKey)
                    }
                    updateOperatorSigningKey
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::owner)
                    }
                    owner
                },
                {
                    fn getOperatorWeightAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getOperatorWeightAtBlock)
                    }
                    getOperatorWeightAtBlock
                },
                {
                    fn getOperatorWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getOperatorWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getOperatorWeight)
                    }
                    getOperatorWeight
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getLastCheckpointThresholdWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointThresholdWeight,
                            )
                    }
                    getLastCheckpointThresholdWeight
                },
                {
                    fn getLastestOperatorSigningKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getLastestOperatorSigningKey)
                    }
                    getLastestOperatorSigningKey
                },
                {
                    fn updateQuorumConfig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateQuorumConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateQuorumConfig)
                    }
                    updateQuorumConfig
                },
                {
                    fn operatorRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <operatorRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::operatorRegistered)
                    }
                    operatorRegistered
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::transferOwnership)
                    }
                    transferOwnership
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointOperatorWeight(inner) => {
                    <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointThresholdWeight(inner) => {
                    <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointThresholdWeightAtBlock(inner) => {
                    <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointTotalWeight(inner) => {
                    <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointTotalWeightAtBlock(inner) => {
                    <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastestOperatorSigningKey(inner) => {
                    <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorSigningKeyAtBlock(inner) => {
                    <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorWeight(inner) => {
                    <getOperatorWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorWeightAtBlock(inner) => {
                    <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::minimumWeight(inner) => {
                    <minimumWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorRegistered(inner) => {
                    <operatorRegisteredCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::quorum(inner) => {
                    <quorumCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registerOperatorWithSignature(inner) => {
                    <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateMinimumWeight(inner) => {
                    <updateMinimumWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorSigningKey(inner) => {
                    <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateQuorumConfig(inner) => {
                    <updateQuorumConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateStakeThreshold(inner) => {
                    <updateStakeThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointOperatorWeight(inner) => {
                    <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointThresholdWeight(inner) => {
                    <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointThresholdWeightAtBlock(inner) => {
                    <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointTotalWeight(inner) => {
                    <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointTotalWeightAtBlock(inner) => {
                    <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastestOperatorSigningKey(inner) => {
                    <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorSigningKeyAtBlock(inner) => {
                    <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorWeight(inner) => {
                    <getOperatorWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorWeightAtBlock(inner) => {
                    <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::minimumWeight(inner) => {
                    <minimumWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorRegistered(inner) => {
                    <operatorRegisteredCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::quorum(inner) => {
                    <quorumCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registerOperatorWithSignature(inner) => {
                    <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateMinimumWeight(inner) => {
                    <updateMinimumWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorSigningKey(inner) => {
                    <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateQuorumConfig(inner) => {
                    <updateQuorumConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateStakeThreshold(inner) => {
                    <updateStakeThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ECDSAStakeRegistry`](self) custom errors.
    pub enum ECDSAStakeRegistryErrors {
        InsufficientSignedStake(InsufficientSignedStake),
        InsufficientWeight(InsufficientWeight),
        InvalidLength(InvalidLength),
        InvalidQuorum(InvalidQuorum),
        InvalidReferenceBlock(InvalidReferenceBlock),
        InvalidSignature(InvalidSignature),
        InvalidSignedWeight(InvalidSignedWeight),
        InvalidThreshold(InvalidThreshold),
        LengthMismatch(LengthMismatch),
        MustUpdateAllOperators(MustUpdateAllOperators),
        NotSorted(NotSorted),
        OperatorAlreadyRegistered(OperatorAlreadyRegistered),
        OperatorNotRegistered(OperatorNotRegistered),
    }
    #[automatically_derived]
    impl ECDSAStakeRegistryErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [37u8, 236u8, 108u8, 31u8],
            [45u8, 61u8, 246u8, 182u8],
            [66u8, 238u8, 104u8, 181u8],
            [139u8, 170u8, 87u8, 159u8],
            [148u8, 125u8, 90u8, 132u8],
            [150u8, 11u8, 65u8, 238u8],
            [168u8, 121u8, 47u8, 209u8],
            [170u8, 189u8, 90u8, 9u8],
            [186u8, 80u8, 249u8, 17u8],
            [209u8, 115u8, 87u8, 121u8],
            [225u8, 33u8, 99u8, 47u8],
            [230u8, 79u8, 24u8, 15u8],
            [255u8, 99u8, 58u8, 56u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ECDSAStakeRegistryErrors {
        const NAME: &'static str = "ECDSAStakeRegistryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 13usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::InsufficientSignedStake(_) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientWeight(_) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidLength(_) => {
                    <InvalidLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidQuorum(_) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidReferenceBlock(_) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignedWeight(_) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidThreshold(_) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LengthMismatch(_) => {
                    <LengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MustUpdateAllOperators(_) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotSorted(_) => <NotSorted as alloy_sol_types::SolError>::SELECTOR,
                Self::OperatorAlreadyRegistered(_) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors>] = &[
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::OperatorNotRegistered)
                    }
                    OperatorNotRegistered
                },
                {
                    fn MustUpdateAllOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::MustUpdateAllOperators)
                    }
                    MustUpdateAllOperators
                },
                {
                    fn OperatorAlreadyRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::OperatorAlreadyRegistered)
                    }
                    OperatorAlreadyRegistered
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn InvalidLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidLength)
                    }
                    InvalidLength
                },
                {
                    fn InvalidSignedWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidSignedWeight as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidSignedWeight)
                    }
                    InvalidSignedWeight
                },
                {
                    fn InsufficientWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InsufficientWeight as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InsufficientWeight)
                    }
                    InsufficientWeight
                },
                {
                    fn InvalidThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidThreshold)
                    }
                    InvalidThreshold
                },
                {
                    fn NotSorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <NotSorted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::NotSorted)
                    }
                    NotSorted
                },
                {
                    fn InvalidQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidQuorum as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidQuorum)
                    }
                    InvalidQuorum
                },
                {
                    fn InsufficientSignedStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InsufficientSignedStake as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InsufficientSignedStake)
                    }
                    InsufficientSignedStake
                },
                {
                    fn InvalidReferenceBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidReferenceBlock)
                    }
                    InvalidReferenceBlock
                },
                {
                    fn LengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <LengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::LengthMismatch)
                    }
                    LengthMismatch
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::InsufficientSignedStake(inner) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientWeight(inner) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidLength(inner) => {
                    <InvalidLength as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidQuorum(inner) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidReferenceBlock(inner) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignedWeight(inner) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidThreshold(inner) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LengthMismatch(inner) => {
                    <LengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MustUpdateAllOperators(inner) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorAlreadyRegistered(inner) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InsufficientSignedStake(inner) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientWeight(inner) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidLength(inner) => {
                    <InvalidLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidQuorum(inner) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidReferenceBlock(inner) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSignedWeight(inner) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidThreshold(inner) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LengthMismatch(inner) => {
                    <LengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MustUpdateAllOperators(inner) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OperatorAlreadyRegistered(inner) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ECDSAStakeRegistry`](self) events.
    pub enum ECDSAStakeRegistryEvents {
        Initialized(Initialized),
        MinimumWeightUpdated(MinimumWeightUpdated),
        OperatorDeregistered(OperatorDeregistered),
        OperatorRegistered(OperatorRegistered),
        OperatorWeightUpdated(OperatorWeightUpdated),
        OwnershipTransferred(OwnershipTransferred),
        QuorumUpdated(QuorumUpdated),
        SigningKeyUpdate(SigningKeyUpdate),
        ThresholdWeightUpdated(ThresholdWeightUpdated),
        TotalWeightUpdated(TotalWeightUpdated),
        UpdateMinimumWeight(UpdateMinimumWeight),
    }
    #[automatically_derived]
    impl ECDSAStakeRegistryEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                30u8,
                164u8,
                33u8,
                134u8,
                179u8,
                5u8,
                250u8,
                55u8,
                49u8,
                4u8,
                80u8,
                217u8,
                251u8,
                135u8,
                234u8,
                30u8,
                143u8,
                12u8,
                127u8,
                68u8,
                126u8,
                119u8,
                20u8,
                121u8,
                227u8,
                178u8,
                118u8,
                52u8,
                191u8,
                232u8,
                77u8,
                193u8,
            ],
            [
                35u8,
                170u8,
                212u8,
                230u8,
                23u8,
                68u8,
                236u8,
                225u8,
                100u8,
                19u8,
                10u8,
                164u8,
                21u8,
                193u8,
                97u8,
                110u8,
                128u8,
                19u8,
                107u8,
                15u8,
                7u8,
                112u8,
                229u8,
                101u8,
                137u8,
                67u8,
                139u8,
                144u8,
                178u8,
                105u8,
                38u8,
                94u8,
            ],
            [
                49u8,
                224u8,
                173u8,
                254u8,
                199u8,
                27u8,
                204u8,
                238u8,
                55u8,
                182u8,
                232u8,
                58u8,
                144u8,
                194u8,
                254u8,
                219u8,
                23u8,
                216u8,
                241u8,
                105u8,
                63u8,
                238u8,
                134u8,
                60u8,
                71u8,
                113u8,
                231u8,
                191u8,
                226u8,
                174u8,
                213u8,
                128u8,
            ],
            [
                113u8,
                60u8,
                165u8,
                59u8,
                136u8,
                214u8,
                235u8,
                99u8,
                245u8,
                177u8,
                133u8,
                76u8,
                184u8,
                203u8,
                221u8,
                115u8,
                110u8,
                197u8,
                30u8,
                218u8,
                34u8,
                94u8,
                70u8,
                121u8,
                26u8,
                169u8,
                41u8,
                139u8,
                1u8,
                96u8,
                100u8,
                143u8,
            ],
            [
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
            ],
            [
                134u8,
                220u8,
                248u8,
                107u8,
                18u8,
                223u8,
                238u8,
                222u8,
                167u8,
                74u8,
                233u8,
                48u8,
                13u8,
                189u8,
                170u8,
                25u8,
                59u8,
                204u8,
                229u8,
                128u8,
                147u8,
                105u8,
                200u8,
                23u8,
                126u8,
                162u8,
                244u8,
                234u8,
                170u8,
                101u8,
                114u8,
                155u8,
            ],
            [
                136u8,
                119u8,
                13u8,
                200u8,
                98u8,
                228u8,
                122u8,
                126u8,
                213u8,
                134u8,
                144u8,
                120u8,
                87u8,
                235u8,
                27u8,
                117u8,
                228u8,
                197u8,
                255u8,
                200u8,
                183u8,
                7u8,
                199u8,
                238u8,
                16u8,
                235u8,
                116u8,
                214u8,
                136u8,
                95u8,
                229u8,
                148u8,
            ],
            [
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ],
            [
                147u8,
                36u8,
                247u8,
                229u8,
                167u8,
                192u8,
                40u8,
                136u8,
                8u8,
                166u8,
                52u8,
                204u8,
                222u8,
                68u8,
                184u8,
                233u8,
                121u8,
                103u8,
                100u8,
                116u8,
                178u8,
                46u8,
                41u8,
                238u8,
                157u8,
                213u8,
                105u8,
                181u8,
                94u8,
                121u8,
                26u8,
                75u8,
            ],
            [
                164u8,
                83u8,
                219u8,
                97u8,
                42u8,
                245u8,
                158u8,
                85u8,
                33u8,
                214u8,
                171u8,
                146u8,
                132u8,
                220u8,
                62u8,
                45u8,
                6u8,
                175u8,
                40u8,
                110u8,
                177u8,
                177u8,
                183u8,
                183u8,
                113u8,
                252u8,
                228u8,
                113u8,
                108u8,
                25u8,
                242u8,
                193u8,
            ],
            [
                208u8,
                97u8,
                22u8,
                130u8,
                82u8,
                244u8,
                65u8,
                115u8,
                54u8,
                88u8,
                240u8,
                158u8,
                77u8,
                143u8,
                91u8,
                45u8,
                153u8,
                142u8,
                212u8,
                239u8,
                36u8,
                162u8,
                187u8,
                253u8,
                108u8,
                236u8,
                165u8,
                46u8,
                161u8,
                49u8,
                80u8,
                2u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ECDSAStakeRegistryEvents {
        const NAME: &'static str = "ECDSAStakeRegistryEvents";
        const COUNT: usize = 11usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <MinimumWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <MinimumWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::MinimumWeightUpdated)
                }
                Some(
                    <OperatorDeregistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorDeregistered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorDeregistered)
                }
                Some(
                    <OperatorRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorRegistered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorRegistered)
                }
                Some(
                    <OperatorWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorWeightUpdated)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<QuorumUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::QuorumUpdated)
                }
                Some(<SigningKeyUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SigningKeyUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SigningKeyUpdate)
                }
                Some(
                    <ThresholdWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ThresholdWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ThresholdWeightUpdated)
                }
                Some(
                    <TotalWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TotalWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TotalWeightUpdated)
                }
                Some(
                    <UpdateMinimumWeight as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UpdateMinimumWeight as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::UpdateMinimumWeight)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for ECDSAStakeRegistryEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MinimumWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SigningKeyUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ThresholdWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TotalWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpdateMinimumWeight(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MinimumWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SigningKeyUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ThresholdWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TotalWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpdateMinimumWeight(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ECDSAStakeRegistry`](self) contract instance.

See the [wrapper's documentation](`ECDSAStakeRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ECDSAStakeRegistryInstance<T, P, N> {
        ECDSAStakeRegistryInstance::<T, P, N>::new(address, provider)
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
        _delegationManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<ECDSAStakeRegistryInstance<T, P, N>>,
    > {
        ECDSAStakeRegistryInstance::<T, P, N>::deploy(provider, _delegationManager)
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
        _delegationManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        ECDSAStakeRegistryInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _delegationManager)
    }
    /**A [`ECDSAStakeRegistry`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ECDSAStakeRegistry`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ECDSAStakeRegistryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ECDSAStakeRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ECDSAStakeRegistryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ECDSAStakeRegistryInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ECDSAStakeRegistry`](self) contract instance.

See the [wrapper's documentation](`ECDSAStakeRegistryInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _delegationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<ECDSAStakeRegistryInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _delegationManager);
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
            _delegationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _delegationManager,
                        },
                    )[..],
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
    impl<T, P: ::core::clone::Clone, N> ECDSAStakeRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ECDSAStakeRegistryInstance<T, P, N> {
            ECDSAStakeRegistryInstance {
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
    > ECDSAStakeRegistryInstance<T, P, N> {
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
        ///Creates a new call builder for the [`deregisterOperator`] function.
        pub fn deregisterOperator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorCall, N> {
            self.call_builder(&deregisterOperatorCall {})
        }
        ///Creates a new call builder for the [`getLastCheckpointOperatorWeight`] function.
        pub fn getLastCheckpointOperatorWeight(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getLastCheckpointOperatorWeightCall,
            N,
        > {
            self.call_builder(
                &getLastCheckpointOperatorWeightCall {
                    _operator,
                },
            )
        }
        ///Creates a new call builder for the [`getLastCheckpointThresholdWeight`] function.
        pub fn getLastCheckpointThresholdWeight(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getLastCheckpointThresholdWeightCall,
            N,
        > {
            self.call_builder(
                &getLastCheckpointThresholdWeightCall {
                },
            )
        }
        ///Creates a new call builder for the [`getLastCheckpointThresholdWeightAtBlock`] function.
        pub fn getLastCheckpointThresholdWeightAtBlock(
            &self,
            _blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getLastCheckpointThresholdWeightAtBlockCall,
            N,
        > {
            self.call_builder(
                &getLastCheckpointThresholdWeightAtBlockCall {
                    _blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getLastCheckpointTotalWeight`] function.
        pub fn getLastCheckpointTotalWeight(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointTotalWeightCall, N> {
            self.call_builder(
                &getLastCheckpointTotalWeightCall {
                },
            )
        }
        ///Creates a new call builder for the [`getLastCheckpointTotalWeightAtBlock`] function.
        pub fn getLastCheckpointTotalWeightAtBlock(
            &self,
            _blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getLastCheckpointTotalWeightAtBlockCall,
            N,
        > {
            self.call_builder(
                &getLastCheckpointTotalWeightAtBlockCall {
                    _blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getLastestOperatorSigningKey`] function.
        pub fn getLastestOperatorSigningKey(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastestOperatorSigningKeyCall, N> {
            self.call_builder(
                &getLastestOperatorSigningKeyCall {
                    _operator,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorSigningKeyAtBlock`] function.
        pub fn getOperatorSigningKeyAtBlock(
            &self,
            _operator: alloy::sol_types::private::Address,
            _blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSigningKeyAtBlockCall, N> {
            self.call_builder(
                &getOperatorSigningKeyAtBlockCall {
                    _operator,
                    _blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorWeight`] function.
        pub fn getOperatorWeight(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorWeightCall, N> {
            self.call_builder(&getOperatorWeightCall { _operator })
        }
        ///Creates a new call builder for the [`getOperatorWeightAtBlock`] function.
        pub fn getOperatorWeightAtBlock(
            &self,
            _operator: alloy::sol_types::private::Address,
            _blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorWeightAtBlockCall, N> {
            self.call_builder(
                &getOperatorWeightAtBlockCall {
                    _operator,
                    _blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _serviceManager: alloy::sol_types::private::Address,
            _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
            _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    _serviceManager,
                    _thresholdWeight,
                    _quorum,
                },
            )
        }
        ///Creates a new call builder for the [`isValidSignature`] function.
        pub fn isValidSignature(
            &self,
            _dataHash: alloy::sol_types::private::FixedBytes<32>,
            _signatureData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, isValidSignatureCall, N> {
            self.call_builder(
                &isValidSignatureCall {
                    _dataHash,
                    _signatureData,
                },
            )
        }
        ///Creates a new call builder for the [`minimumWeight`] function.
        pub fn minimumWeight(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, minimumWeightCall, N> {
            self.call_builder(&minimumWeightCall {})
        }
        ///Creates a new call builder for the [`operatorRegistered`] function.
        pub fn operatorRegistered(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorRegisteredCall, N> {
            self.call_builder(
                &operatorRegisteredCall {
                    _operator,
                },
            )
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`quorum`] function.
        pub fn quorum(&self) -> alloy_contract::SolCallBuilder<T, &P, quorumCall, N> {
            self.call_builder(&quorumCall {})
        }
        ///Creates a new call builder for the [`registerOperatorWithSignature`] function.
        pub fn registerOperatorWithSignature(
            &self,
            _operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
            _signingKey: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            registerOperatorWithSignatureCall,
            N,
        > {
            self.call_builder(
                &registerOperatorWithSignatureCall {
                    _operatorSignature,
                    _signingKey,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`updateMinimumWeight`] function.
        pub fn updateMinimumWeight(
            &self,
            _newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
            _operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateMinimumWeightCall, N> {
            self.call_builder(
                &updateMinimumWeightCall {
                    _newMinimumWeight,
                    _operators,
                },
            )
        }
        ///Creates a new call builder for the [`updateOperatorSigningKey`] function.
        pub fn updateOperatorSigningKey(
            &self,
            _newSigningKey: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorSigningKeyCall, N> {
            self.call_builder(
                &updateOperatorSigningKeyCall {
                    _newSigningKey,
                },
            )
        }
        ///Creates a new call builder for the [`updateOperators`] function.
        pub fn updateOperators(
            &self,
            _operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsCall, N> {
            self.call_builder(&updateOperatorsCall { _operators })
        }
        ///Creates a new call builder for the [`updateOperatorsForQuorum`] function.
        pub fn updateOperatorsForQuorum(
            &self,
            operatorsPerQuorum: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            >,
            _1: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsForQuorumCall, N> {
            self.call_builder(
                &updateOperatorsForQuorumCall {
                    operatorsPerQuorum,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`updateQuorumConfig`] function.
        pub fn updateQuorumConfig(
            &self,
            _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
            _operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateQuorumConfigCall, N> {
            self.call_builder(
                &updateQuorumConfigCall {
                    _quorum,
                    _operators,
                },
            )
        }
        ///Creates a new call builder for the [`updateStakeThreshold`] function.
        pub fn updateStakeThreshold(
            &self,
            _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateStakeThresholdCall, N> {
            self.call_builder(
                &updateStakeThresholdCall {
                    _thresholdWeight,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ECDSAStakeRegistryInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`MinimumWeightUpdated`] event.
        pub fn MinimumWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MinimumWeightUpdated, N> {
            self.event_filter::<MinimumWeightUpdated>()
        }
        ///Creates a new event filter for the [`OperatorDeregistered`] event.
        pub fn OperatorDeregistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorDeregistered, N> {
            self.event_filter::<OperatorDeregistered>()
        }
        ///Creates a new event filter for the [`OperatorRegistered`] event.
        pub fn OperatorRegistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorRegistered, N> {
            self.event_filter::<OperatorRegistered>()
        }
        ///Creates a new event filter for the [`OperatorWeightUpdated`] event.
        pub fn OperatorWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorWeightUpdated, N> {
            self.event_filter::<OperatorWeightUpdated>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`QuorumUpdated`] event.
        pub fn QuorumUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, QuorumUpdated, N> {
            self.event_filter::<QuorumUpdated>()
        }
        ///Creates a new event filter for the [`SigningKeyUpdate`] event.
        pub fn SigningKeyUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SigningKeyUpdate, N> {
            self.event_filter::<SigningKeyUpdate>()
        }
        ///Creates a new event filter for the [`ThresholdWeightUpdated`] event.
        pub fn ThresholdWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ThresholdWeightUpdated, N> {
            self.event_filter::<ThresholdWeightUpdated>()
        }
        ///Creates a new event filter for the [`TotalWeightUpdated`] event.
        pub fn TotalWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TotalWeightUpdated, N> {
            self.event_filter::<TotalWeightUpdated>()
        }
        ///Creates a new event filter for the [`UpdateMinimumWeight`] event.
        pub fn UpdateMinimumWeight_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UpdateMinimumWeight, N> {
            self.event_filter::<UpdateMinimumWeight>()
        }
    }
}
