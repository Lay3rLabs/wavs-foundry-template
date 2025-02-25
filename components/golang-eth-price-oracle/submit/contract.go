// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package submitcontract

import (
	"errors"
	"math/big"
	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
)

// Reference imports to suppress errors if they are not otherwise used.
var (
	_ = errors.New
	_ = big.NewInt
	_ = strings.NewReader
	_ = ethereum.NotFound
	_ = bind.Bind
	_ = common.Big1
	_ = types.BloomLookup
	_ = event.NewSubscription
	_ = abi.ConvertType
)

// SubmitContractMetaData contains all meta data concerning the SubmitContract contract.
var SubmitContractMetaData = &bind.MetaData{
	ABI: "[{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"_triggerInfo\",\"type\":\"bytes\"}],\"name\":\"NewTrigger\",\"type\":\"event\"}]",
}

// SubmitContractABI is the input ABI used to generate the binding from.
// Deprecated: Use SubmitContractMetaData.ABI instead.
var SubmitContractABI = SubmitContractMetaData.ABI

// SubmitContract is an auto generated Go binding around an Ethereum contract.
type SubmitContract struct {
	SubmitContractCaller     // Read-only binding to the contract
	SubmitContractTransactor // Write-only binding to the contract
	SubmitContractFilterer   // Log filterer for contract events
}

// SubmitContractCaller is an auto generated read-only Go binding around an Ethereum contract.
type SubmitContractCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// SubmitContractTransactor is an auto generated write-only Go binding around an Ethereum contract.
type SubmitContractTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// SubmitContractFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type SubmitContractFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// SubmitContractSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type SubmitContractSession struct {
	Contract     *SubmitContract   // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// SubmitContractCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type SubmitContractCallerSession struct {
	Contract *SubmitContractCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts         // Call options to use throughout this session
}

// SubmitContractTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type SubmitContractTransactorSession struct {
	Contract     *SubmitContractTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts         // Transaction auth options to use throughout this session
}

// SubmitContractRaw is an auto generated low-level Go binding around an Ethereum contract.
type SubmitContractRaw struct {
	Contract *SubmitContract // Generic contract binding to access the raw methods on
}

// SubmitContractCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type SubmitContractCallerRaw struct {
	Contract *SubmitContractCaller // Generic read-only contract binding to access the raw methods on
}

// SubmitContractTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type SubmitContractTransactorRaw struct {
	Contract *SubmitContractTransactor // Generic write-only contract binding to access the raw methods on
}

// NewSubmitContract creates a new instance of SubmitContract, bound to a specific deployed contract.
func NewSubmitContract(address common.Address, backend bind.ContractBackend) (*SubmitContract, error) {
	contract, err := bindSubmitContract(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &SubmitContract{SubmitContractCaller: SubmitContractCaller{contract: contract}, SubmitContractTransactor: SubmitContractTransactor{contract: contract}, SubmitContractFilterer: SubmitContractFilterer{contract: contract}}, nil
}

// NewSubmitContractCaller creates a new read-only instance of SubmitContract, bound to a specific deployed contract.
func NewSubmitContractCaller(address common.Address, caller bind.ContractCaller) (*SubmitContractCaller, error) {
	contract, err := bindSubmitContract(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &SubmitContractCaller{contract: contract}, nil
}

// NewSubmitContractTransactor creates a new write-only instance of SubmitContract, bound to a specific deployed contract.
func NewSubmitContractTransactor(address common.Address, transactor bind.ContractTransactor) (*SubmitContractTransactor, error) {
	contract, err := bindSubmitContract(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &SubmitContractTransactor{contract: contract}, nil
}

// NewSubmitContractFilterer creates a new log filterer instance of SubmitContract, bound to a specific deployed contract.
func NewSubmitContractFilterer(address common.Address, filterer bind.ContractFilterer) (*SubmitContractFilterer, error) {
	contract, err := bindSubmitContract(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &SubmitContractFilterer{contract: contract}, nil
}

// bindSubmitContract binds a generic wrapper to an already deployed contract.
func bindSubmitContract(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := SubmitContractMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_SubmitContract *SubmitContractRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _SubmitContract.Contract.SubmitContractCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_SubmitContract *SubmitContractRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _SubmitContract.Contract.SubmitContractTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_SubmitContract *SubmitContractRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _SubmitContract.Contract.SubmitContractTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_SubmitContract *SubmitContractCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _SubmitContract.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_SubmitContract *SubmitContractTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _SubmitContract.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_SubmitContract *SubmitContractTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _SubmitContract.Contract.contract.Transact(opts, method, params...)
}

// SubmitContractNewTriggerIterator is returned from FilterNewTrigger and is used to iterate over the raw logs and unpacked data for NewTrigger events raised by the SubmitContract contract.
type SubmitContractNewTriggerIterator struct {
	Event *SubmitContractNewTrigger // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *SubmitContractNewTriggerIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(SubmitContractNewTrigger)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(SubmitContractNewTrigger)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *SubmitContractNewTriggerIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *SubmitContractNewTriggerIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// SubmitContractNewTrigger represents a NewTrigger event raised by the SubmitContract contract.
type SubmitContractNewTrigger struct {
	TriggerInfo []byte
	Raw         types.Log // Blockchain specific contextual infos
}

// FilterNewTrigger is a free log retrieval operation binding the contract event 0x86eacd23610d81706516de1ed0476c87772fdf939c7c771fbbd7f0230d619e68.
//
// Solidity: event NewTrigger(bytes _triggerInfo)
func (_SubmitContract *SubmitContractFilterer) FilterNewTrigger(opts *bind.FilterOpts) (*SubmitContractNewTriggerIterator, error) {

	logs, sub, err := _SubmitContract.contract.FilterLogs(opts, "NewTrigger")
	if err != nil {
		return nil, err
	}
	return &SubmitContractNewTriggerIterator{contract: _SubmitContract.contract, event: "NewTrigger", logs: logs, sub: sub}, nil
}

// WatchNewTrigger is a free log subscription operation binding the contract event 0x86eacd23610d81706516de1ed0476c87772fdf939c7c771fbbd7f0230d619e68.
//
// Solidity: event NewTrigger(bytes _triggerInfo)
func (_SubmitContract *SubmitContractFilterer) WatchNewTrigger(opts *bind.WatchOpts, sink chan<- *SubmitContractNewTrigger) (event.Subscription, error) {

	logs, sub, err := _SubmitContract.contract.WatchLogs(opts, "NewTrigger")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(SubmitContractNewTrigger)
				if err := _SubmitContract.contract.UnpackLog(event, "NewTrigger", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseNewTrigger is a log parse operation binding the contract event 0x86eacd23610d81706516de1ed0476c87772fdf939c7c771fbbd7f0230d619e68.
//
// Solidity: event NewTrigger(bytes _triggerInfo)
func (_SubmitContract *SubmitContractFilterer) ParseNewTrigger(log types.Log) (*SubmitContractNewTrigger, error) {
	event := new(SubmitContractNewTrigger)
	if err := _SubmitContract.contract.UnpackLog(event, "NewTrigger", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
