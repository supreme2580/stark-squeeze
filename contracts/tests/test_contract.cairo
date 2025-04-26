use starknet::ContractAddress;

use snforge_std::{declare, ContractClassTrait, DeclareResultTrait};

use contracts::IHelloStarknetSafeDispatcher;
use contracts::IHelloStarknetSafeDispatcherTrait;
use contracts::IHelloStarknetDispatcher;
use contracts::IHelloStarknetDispatcherTrait;
use core::byte_array::ByteArray;

fn deploy_contract(name: ByteArray) -> ContractAddress {
    let contract = declare(name).unwrap().contract_class();
    let (contract_address, _) = contract.deploy(@ArrayTrait::new()).unwrap();
    contract_address
}

#[test]
fn test_increase_balance() {
    let contract_address = deploy_contract("HelloStarknet");

    let dispatcher = IHelloStarknetDispatcher { contract_address };

    let balance_before = dispatcher.get_balance();
    assert(balance_before == 0, 'Invalid balance');

    dispatcher.increase_balance(42);

    let balance_after = dispatcher.get_balance();
    assert(balance_after == 42, 'Invalid balance');
}

#[test]
#[feature("safe_dispatcher")]
fn test_cannot_increase_balance_with_zero_value() {
    let contract_address = deploy_contract("HelloStarknet");

    let safe_dispatcher = IHelloStarknetSafeDispatcher { contract_address };

    let balance_before = safe_dispatcher.get_balance().unwrap();
    assert(balance_before == 0, 'Invalid balance');

    match safe_dispatcher.increase_balance(0) {
        Result::Ok(_) => core::panic_with_felt252('Should have panicked'),
        Result::Err(panic_data) => {
            assert(*panic_data.at(0) == 'Amount cannot be 0', *panic_data.at(0));
        }
    };
}

#[test]
fn test_process_file_metadata() {
    let contract_address = deploy_contract("HelloStarknet");
    let dispatcher = IHelloStarknetDispatcher { contract_address };
    
    // Create test data parameters
    let data_size = 16; // Simulating size of binary data
    let file_type = 'txt';
    let original_size = 100; // Simulating original uncompressed size
    
    // Call process_file_metadata - should run without errors
    dispatcher.process_file_metadata(data_size, file_type, original_size);
    
    // If we reached here without errors, the test passed
    assert(true, 'Test passed');
}
