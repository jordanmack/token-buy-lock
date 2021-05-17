use super::*;
use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context};
use ckb_tool::{ckb_error::assert_error_eq, ckb_script::ScriptError};
use ckb_tool::ckb_types::{bytes::Bytes, packed::*, prelude::*};
use ckb_tool::ckb_types::core::{TransactionBuilder};

// Constants
const MAX_CYCLES: u64 = 100_000_000;

// Error Codes
const ERROR_AMOUNT: i8 = 5;
const ERROR_ARGS_LEN: i8 = 6;
const ERROR_DATA_LEN: i8 = 7;

#[test]
fn test_buy_exact()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));
	let out_point_token_buy_lock = context.deploy_cell(Loader::default().load_binary("token-buy-lock"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();
	let token_buy_lock_dep = CellDep::new_builder().out_point(out_point_token_buy_lock.clone()).build();

	// Prepare Identities
	let buyer_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![0u8; 1])).expect("script");
	let buyer_lock_script_hash: [u8; 32] = buyer_lock_script.calc_script_hash().unpack();
	let seller_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![1u8; 1])).expect("script");
	// let seller_lock_script_hash: [u8; 32] = seller_lock_script.calc_script_hash().unpack();
	let sudt_owner_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![3u8; 1])).expect("script");
	let sudt_owner_lock_script_hash: [u8; 32] = sudt_owner_lock_script.calc_script_hash().unpack();

	// Prepare SUDT Type Script
	let script_args: Bytes = sudt_owner_lock_script_hash.to_vec().into();
	let sudt_type_script = context.build_script(&out_point_sudt, script_args).expect("script");
	let sudt_type_script_hash: [u8; 32] = sudt_type_script.calc_script_hash().unpack();

	// Prepare Token Buy Lock Script
	let script_args: Bytes = sudt_type_script_hash.to_vec().into();
	let token_buy_lock_script = context.build_script(&out_point_token_buy_lock, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.extend(buyer_lock_script_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let data = 9_000u128.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	
	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 100u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 8_900u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.cell_dep(token_buy_lock_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_buy_extra()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));
	let out_point_token_buy_lock = context.deploy_cell(Loader::default().load_binary("token-buy-lock"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();
	let token_buy_lock_dep = CellDep::new_builder().out_point(out_point_token_buy_lock.clone()).build();

	// Prepare Identities
	let buyer_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![0u8; 1])).expect("script");
	let buyer_lock_script_hash: [u8; 32] = buyer_lock_script.calc_script_hash().unpack();
	let seller_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![1u8; 1])).expect("script");
	// let seller_lock_script_hash: [u8; 32] = seller_lock_script.calc_script_hash().unpack();
	let sudt_owner_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![3u8; 1])).expect("script");
	let sudt_owner_lock_script_hash: [u8; 32] = sudt_owner_lock_script.calc_script_hash().unpack();

	// Prepare SUDT Type Script
	let script_args: Bytes = sudt_owner_lock_script_hash.to_vec().into();
	let sudt_type_script = context.build_script(&out_point_sudt, script_args).expect("script");
	let sudt_type_script_hash: [u8; 32] = sudt_type_script.calc_script_hash().unpack();

	// Prepare Token Buy Lock Script
	let script_args: Bytes = sudt_type_script_hash.to_vec().into();
	let token_buy_lock_script = context.build_script(&out_point_token_buy_lock, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.extend(buyer_lock_script_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let data = 9_000u128.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	
	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 1_000u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 8_000u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.cell_dep(token_buy_lock_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_buy_short()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));
	let out_point_token_buy_lock = context.deploy_cell(Loader::default().load_binary("token-buy-lock"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();
	let token_buy_lock_dep = CellDep::new_builder().out_point(out_point_token_buy_lock.clone()).build();

	// Prepare Identities
	let buyer_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![0u8; 1])).expect("script");
	let buyer_lock_script_hash: [u8; 32] = buyer_lock_script.calc_script_hash().unpack();
	let seller_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![1u8; 1])).expect("script");
	// let seller_lock_script_hash: [u8; 32] = seller_lock_script.calc_script_hash().unpack();
	let sudt_owner_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![3u8; 1])).expect("script");
	let sudt_owner_lock_script_hash: [u8; 32] = sudt_owner_lock_script.calc_script_hash().unpack();

	// Prepare SUDT Type Script
	let script_args: Bytes = sudt_owner_lock_script_hash.to_vec().into();
	let sudt_type_script = context.build_script(&out_point_sudt, script_args).expect("script");
	let sudt_type_script_hash: [u8; 32] = sudt_type_script.calc_script_hash().unpack();

	// Prepare Token Buy Lock Script
	let script_args: Bytes = sudt_type_script_hash.to_vec().into();
	let token_buy_lock_script = context.build_script(&out_point_token_buy_lock, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.extend(buyer_lock_script_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let data = 9_000u128.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	
	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 99u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 8_901u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.cell_dep(token_buy_lock_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_AMOUNT).input_lock_script(0));
}

#[test]
fn test_buy_wrong_tokens()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));
	let out_point_token_buy_lock = context.deploy_cell(Loader::default().load_binary("token-buy-lock"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();
	let token_buy_lock_dep = CellDep::new_builder().out_point(out_point_token_buy_lock.clone()).build();

	// Prepare Identities
	let buyer_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![0u8; 1])).expect("script");
	let buyer_lock_script_hash: [u8; 32] = buyer_lock_script.calc_script_hash().unpack();
	let seller_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![1u8; 1])).expect("script");
	// let seller_lock_script_hash: [u8; 32] = seller_lock_script.calc_script_hash().unpack();
	let sudt_owner_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![3u8; 1])).expect("script");
	let sudt_owner_lock_script_hash: [u8; 32] = sudt_owner_lock_script.calc_script_hash().unpack();
	let sudt_owner_lock_script2 = context.build_script(&out_point_always_success, Bytes::from(vec![4u8; 1])).expect("script");
	let sudt_owner_lock_script2_hash: [u8; 32] = sudt_owner_lock_script2.calc_script_hash().unpack();

	// Prepare SUDT Type Script
	let script_args: Bytes = sudt_owner_lock_script_hash.to_vec().into();
	let sudt_type_script = context.build_script(&out_point_sudt, script_args).expect("script");
	let sudt_type_script_hash: [u8; 32] = sudt_type_script.calc_script_hash().unpack();

	// Prepare SUDT Type Script2
	let script_args: Bytes = sudt_owner_lock_script2_hash.to_vec().into();
	let sudt_type_script2 = context.build_script(&out_point_sudt, script_args).expect("script");
	// let sudt_type_script2_hash: [u8; 32] = sudt_type_script.calc_script_hash().unpack();

	// Prepare Token Buy Lock Script
	let script_args: Bytes = sudt_type_script_hash.to_vec().into();
	let token_buy_lock_script = context.build_script(&out_point_token_buy_lock, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.extend(buyer_lock_script_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let data = 9_000u128.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script2.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	
	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).type_(Some(sudt_type_script2.clone()).pack()).build();
	outputs.push(output);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script2.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 100u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 8_900u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.cell_dep(token_buy_lock_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_AMOUNT).input_lock_script(0));
}

#[test]
fn test_buy_invalid_args()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));
	let out_point_token_buy_lock = context.deploy_cell(Loader::default().load_binary("token-buy-lock"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();
	let token_buy_lock_dep = CellDep::new_builder().out_point(out_point_token_buy_lock.clone()).build();

	// Prepare Identities
	let buyer_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![0u8; 1])).expect("script");
	let buyer_lock_script_hash: [u8; 32] = buyer_lock_script.calc_script_hash().unpack();
	let seller_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![1u8; 1])).expect("script");
	// let seller_lock_script_hash: [u8; 32] = seller_lock_script.calc_script_hash().unpack();
	let sudt_owner_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![3u8; 1])).expect("script");
	let sudt_owner_lock_script_hash: [u8; 32] = sudt_owner_lock_script.calc_script_hash().unpack();

	// Prepare SUDT Type Script
	let script_args: Bytes = sudt_owner_lock_script_hash.to_vec().into();
	let sudt_type_script = context.build_script(&out_point_sudt, script_args).expect("script");
	// let sudt_type_script_hash: [u8; 32] = sudt_type_script.calc_script_hash().unpack();

	// Prepare Token Buy Lock Script
	let script_args: Bytes = [0u8; 16].to_vec().into(); // Invalid args.
	let token_buy_lock_script = context.build_script(&out_point_token_buy_lock, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.extend(buyer_lock_script_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let data = 9_000u128.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	
	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 100u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 8_900u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.cell_dep(token_buy_lock_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_ARGS_LEN).input_lock_script(0));
}

#[test]
fn test_buy_invalid_data()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));
	let out_point_token_buy_lock = context.deploy_cell(Loader::default().load_binary("token-buy-lock"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();
	let token_buy_lock_dep = CellDep::new_builder().out_point(out_point_token_buy_lock.clone()).build();

	// Prepare Identities
	let buyer_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![0u8; 1])).expect("script");
	let buyer_lock_script_hash: [u8; 32] = buyer_lock_script.calc_script_hash().unpack();
	let seller_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![1u8; 1])).expect("script");
	// let seller_lock_script_hash: [u8; 32] = seller_lock_script.calc_script_hash().unpack();
	let sudt_owner_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![3u8; 1])).expect("script");
	let sudt_owner_lock_script_hash: [u8; 32] = sudt_owner_lock_script.calc_script_hash().unpack();

	// Prepare SUDT Type Script
	let script_args: Bytes = sudt_owner_lock_script_hash.to_vec().into();
	let sudt_type_script = context.build_script(&out_point_sudt, script_args).expect("script");
	let sudt_type_script_hash: [u8; 32] = sudt_type_script.calc_script_hash().unpack();

	// Prepare Token Buy Lock Script
	let script_args: Bytes = sudt_type_script_hash.to_vec().into();
	let token_buy_lock_script = context.build_script(&out_point_token_buy_lock, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.extend(buyer_lock_script_hash.to_vec());
	data.extend(100u64.to_le_bytes().to_vec()); // Incorrect data length (u64 vs u128).
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let data = 9_000u128.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	
	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 100u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 8_900u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.cell_dep(token_buy_lock_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_DATA_LEN).input_lock_script(0));
}

#[test]
fn test_buy_multiple_buy_cells_same_owner()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));
	let out_point_token_buy_lock = context.deploy_cell(Loader::default().load_binary("token-buy-lock"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();
	let token_buy_lock_dep = CellDep::new_builder().out_point(out_point_token_buy_lock.clone()).build();

	// Prepare Identities
	let buyer_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![0u8; 1])).expect("script");
	let buyer_lock_script_hash: [u8; 32] = buyer_lock_script.calc_script_hash().unpack();
	let seller_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![1u8; 1])).expect("script");
	// let seller_lock_script_hash: [u8; 32] = seller_lock_script.calc_script_hash().unpack();
	let sudt_owner_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![3u8; 1])).expect("script");
	let sudt_owner_lock_script_hash: [u8; 32] = sudt_owner_lock_script.calc_script_hash().unpack();

	// Prepare SUDT Type Script
	let script_args: Bytes = sudt_owner_lock_script_hash.to_vec().into();
	let sudt_type_script = context.build_script(&out_point_sudt, script_args).expect("script");
	let sudt_type_script_hash: [u8; 32] = sudt_type_script.calc_script_hash().unpack();

	// Prepare Token Buy Lock Script
	let script_args: Bytes = sudt_type_script_hash.to_vec().into();
	let token_buy_lock_script = context.build_script(&out_point_token_buy_lock, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.extend(buyer_lock_script_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input.clone()); // Add two Token Buy Lock cells.
	inputs.push(input);
	let data = 9_000u128.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	
	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 200u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 8_800u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.cell_dep(token_buy_lock_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_buy_multiple_buy_cells_different_owners()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));
	let out_point_token_buy_lock = context.deploy_cell(Loader::default().load_binary("token-buy-lock"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();
	let token_buy_lock_dep = CellDep::new_builder().out_point(out_point_token_buy_lock.clone()).build();

	// Prepare Identities
	let buyer_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![0u8; 1])).expect("script");
	let buyer_lock_script_hash: [u8; 32] = buyer_lock_script.calc_script_hash().unpack();
	let buyer_lock_script2 = context.build_script(&out_point_always_success, Bytes::from(vec![2u8; 1])).expect("script");
	let buyer_lock_script2_hash: [u8; 32] = buyer_lock_script2.calc_script_hash().unpack();
	let seller_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![3u8; 1])).expect("script");
	// let seller_lock_script_hash: [u8; 32] = seller_lock_script.calc_script_hash().unpack();
	let sudt_owner_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![4u8; 1])).expect("script");
	let sudt_owner_lock_script_hash: [u8; 32] = sudt_owner_lock_script.calc_script_hash().unpack();

	// Prepare SUDT Type Script
	let script_args: Bytes = sudt_owner_lock_script_hash.to_vec().into();
	let sudt_type_script = context.build_script(&out_point_sudt, script_args).expect("script");
	let sudt_type_script_hash: [u8; 32] = sudt_type_script.calc_script_hash().unpack();

	// Prepare Token Buy Lock Script
	let script_args: Bytes = sudt_type_script_hash.to_vec().into();
	let token_buy_lock_script = context.build_script(&out_point_token_buy_lock, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.extend(buyer_lock_script_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let mut data = vec!();
	data.extend(buyer_lock_script2_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let data = 9_000u128.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	
	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script2.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 100u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 100u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 8_800u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.cell_dep(token_buy_lock_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_buy_multiple_buy_cells_different_owners_extra()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));
	let out_point_token_buy_lock = context.deploy_cell(Loader::default().load_binary("token-buy-lock"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();
	let token_buy_lock_dep = CellDep::new_builder().out_point(out_point_token_buy_lock.clone()).build();

	// Prepare Identities
	let buyer_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![0u8; 1])).expect("script");
	let buyer_lock_script_hash: [u8; 32] = buyer_lock_script.calc_script_hash().unpack();
	let buyer_lock_script2 = context.build_script(&out_point_always_success, Bytes::from(vec![2u8; 1])).expect("script");
	let buyer_lock_script2_hash: [u8; 32] = buyer_lock_script2.calc_script_hash().unpack();
	let seller_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![3u8; 1])).expect("script");
	// let seller_lock_script_hash: [u8; 32] = seller_lock_script.calc_script_hash().unpack();
	let sudt_owner_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![4u8; 1])).expect("script");
	let sudt_owner_lock_script_hash: [u8; 32] = sudt_owner_lock_script.calc_script_hash().unpack();

	// Prepare SUDT Type Script
	let script_args: Bytes = sudt_owner_lock_script_hash.to_vec().into();
	let sudt_type_script = context.build_script(&out_point_sudt, script_args).expect("script");
	let sudt_type_script_hash: [u8; 32] = sudt_type_script.calc_script_hash().unpack();

	// Prepare Token Buy Lock Script
	let script_args: Bytes = sudt_type_script_hash.to_vec().into();
	let token_buy_lock_script = context.build_script(&out_point_token_buy_lock, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.extend(buyer_lock_script_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let mut data = vec!();
	data.extend(buyer_lock_script2_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let data = 9_000u128.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	
	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script2.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 101u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 101u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 8_798u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.cell_dep(token_buy_lock_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_buy_multiple_buy_cells_different_owners_short()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));
	let out_point_token_buy_lock = context.deploy_cell(Loader::default().load_binary("token-buy-lock"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();
	let token_buy_lock_dep = CellDep::new_builder().out_point(out_point_token_buy_lock.clone()).build();

	// Prepare Identities
	let buyer_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![0u8; 1])).expect("script");
	let buyer_lock_script_hash: [u8; 32] = buyer_lock_script.calc_script_hash().unpack();
	let buyer_lock_script2 = context.build_script(&out_point_always_success, Bytes::from(vec![2u8; 1])).expect("script");
	let buyer_lock_script2_hash: [u8; 32] = buyer_lock_script2.calc_script_hash().unpack();
	let seller_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![3u8; 1])).expect("script");
	// let seller_lock_script_hash: [u8; 32] = seller_lock_script.calc_script_hash().unpack();
	let sudt_owner_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![4u8; 1])).expect("script");
	let sudt_owner_lock_script_hash: [u8; 32] = sudt_owner_lock_script.calc_script_hash().unpack();

	// Prepare SUDT Type Script
	let script_args: Bytes = sudt_owner_lock_script_hash.to_vec().into();
	let sudt_type_script = context.build_script(&out_point_sudt, script_args).expect("script");
	let sudt_type_script_hash: [u8; 32] = sudt_type_script.calc_script_hash().unpack();

	// Prepare Token Buy Lock Script
	let script_args: Bytes = sudt_type_script_hash.to_vec().into();
	let token_buy_lock_script = context.build_script(&out_point_token_buy_lock, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.extend(buyer_lock_script_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let mut data = vec!();
	data.extend(buyer_lock_script2_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let data = 9_000u128.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	
	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script2.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).type_(Some(sudt_type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 101u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 99u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 8_800u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.cell_dep(token_buy_lock_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_AMOUNT).input_lock_script(0));
}

#[test]
fn test_buy_withdrawal_by_owner()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));
	let out_point_token_buy_lock = context.deploy_cell(Loader::default().load_binary("token-buy-lock"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();
	let token_buy_lock_dep = CellDep::new_builder().out_point(out_point_token_buy_lock.clone()).build();

	// Prepare Identities
	let buyer_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![0u8; 1])).expect("script");
	let buyer_lock_script_hash: [u8; 32] = buyer_lock_script.calc_script_hash().unpack();
	// let seller_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![1u8; 1])).expect("script");
	// let seller_lock_script_hash: [u8; 32] = seller_lock_script.calc_script_hash().unpack();
	let sudt_owner_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![3u8; 1])).expect("script");
	let sudt_owner_lock_script_hash: [u8; 32] = sudt_owner_lock_script.calc_script_hash().unpack();

	// Prepare SUDT Type Script
	let script_args: Bytes = sudt_owner_lock_script_hash.to_vec().into();
	let sudt_type_script = context.build_script(&out_point_sudt, script_args).expect("script");
	let sudt_type_script_hash: [u8; 32] = sudt_type_script.calc_script_hash().unpack();

	// Prepare Token Buy Lock Script
	let script_args: Bytes = sudt_type_script_hash.to_vec().into();
	let token_buy_lock_script = context.build_script(&out_point_token_buy_lock, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.extend(buyer_lock_script_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	
	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	outputs_data.push(Bytes::new());

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.cell_dep(token_buy_lock_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_buy_withdrawal_by_multiple_owners()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));
	let out_point_token_buy_lock = context.deploy_cell(Loader::default().load_binary("token-buy-lock"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();
	let token_buy_lock_dep = CellDep::new_builder().out_point(out_point_token_buy_lock.clone()).build();

	// Prepare Identities
	let buyer_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![0u8; 1])).expect("script");
	let buyer_lock_script_hash: [u8; 32] = buyer_lock_script.calc_script_hash().unpack();
	let buyer_lock_script2 = context.build_script(&out_point_always_success, Bytes::from(vec![1u8; 1])).expect("script");
	let buyer_lock_script2_hash: [u8; 32] = buyer_lock_script2.calc_script_hash().unpack();
	// let seller_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![2u8; 1])).expect("script");
	// let seller_lock_script_hash: [u8; 32] = seller_lock_script.calc_script_hash().unpack();
	let sudt_owner_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![3u8; 1])).expect("script");
	let sudt_owner_lock_script_hash: [u8; 32] = sudt_owner_lock_script.calc_script_hash().unpack();

	// Prepare SUDT Type Script
	let script_args: Bytes = sudt_owner_lock_script_hash.to_vec().into();
	let sudt_type_script = context.build_script(&out_point_sudt, script_args).expect("script");
	let sudt_type_script_hash: [u8; 32] = sudt_type_script.calc_script_hash().unpack();

	// Prepare Token Buy Lock Script
	let script_args: Bytes = sudt_type_script_hash.to_vec().into();
	let token_buy_lock_script = context.build_script(&out_point_token_buy_lock, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.extend(buyer_lock_script_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let mut data = vec!();
	data.extend(buyer_lock_script2_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script2.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	
	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	outputs_data.push(Bytes::new());

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.cell_dep(token_buy_lock_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_buy_withdrawal_by_multiple_owners_invalid()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));
	let out_point_token_buy_lock = context.deploy_cell(Loader::default().load_binary("token-buy-lock"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();
	let token_buy_lock_dep = CellDep::new_builder().out_point(out_point_token_buy_lock.clone()).build();

	// Prepare Identities
	let buyer_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![0u8; 1])).expect("script");
	let buyer_lock_script_hash: [u8; 32] = buyer_lock_script.calc_script_hash().unpack();
	let buyer_lock_script2 = context.build_script(&out_point_always_success, Bytes::from(vec![1u8; 1])).expect("script");
	let buyer_lock_script2_hash: [u8; 32] = buyer_lock_script2.calc_script_hash().unpack();
	// let seller_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![2u8; 1])).expect("script");
	// let seller_lock_script_hash: [u8; 32] = seller_lock_script.calc_script_hash().unpack();
	let sudt_owner_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![3u8; 1])).expect("script");
	let sudt_owner_lock_script_hash: [u8; 32] = sudt_owner_lock_script.calc_script_hash().unpack();

	// Prepare SUDT Type Script
	let script_args: Bytes = sudt_owner_lock_script_hash.to_vec().into();
	let sudt_type_script = context.build_script(&out_point_sudt, script_args).expect("script");
	let sudt_type_script_hash: [u8; 32] = sudt_type_script.calc_script_hash().unpack();

	// Prepare Token Buy Lock Script
	let script_args: Bytes = sudt_type_script_hash.to_vec().into();
	let token_buy_lock_script = context.build_script(&out_point_token_buy_lock, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.extend(buyer_lock_script_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let mut data = vec!();
	data.extend(buyer_lock_script2_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	// let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script2.clone()).build(), Bytes::new());
	// let input = CellInput::new_builder().previous_output(input_out_point).build();
	// inputs.push(input);
	
	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(buyer_lock_script.clone()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	outputs_data.push(Bytes::new());

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.cell_dep(token_buy_lock_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_AMOUNT).input_lock_script(0));
}

#[test]
fn test_buy_withdrawal_by_not_owner()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));
	let out_point_token_buy_lock = context.deploy_cell(Loader::default().load_binary("token-buy-lock"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();
	let token_buy_lock_dep = CellDep::new_builder().out_point(out_point_token_buy_lock.clone()).build();

	// Prepare Identities
	let buyer_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![0u8; 1])).expect("script");
	let buyer_lock_script_hash: [u8; 32] = buyer_lock_script.calc_script_hash().unpack();
	let seller_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![1u8; 1])).expect("script");
	// let seller_lock_script_hash: [u8; 32] = seller_lock_script.calc_script_hash().unpack();
	let sudt_owner_lock_script = context.build_script(&out_point_always_success, Bytes::from(vec![3u8; 1])).expect("script");
	let sudt_owner_lock_script_hash: [u8; 32] = sudt_owner_lock_script.calc_script_hash().unpack();

	// Prepare SUDT Type Script
	let script_args: Bytes = sudt_owner_lock_script_hash.to_vec().into();
	let sudt_type_script = context.build_script(&out_point_sudt, script_args).expect("script");
	let sudt_type_script_hash: [u8; 32] = sudt_type_script.calc_script_hash().unpack();

	// Prepare Token Buy Lock Script
	let script_args: Bytes = sudt_type_script_hash.to_vec().into();
	let token_buy_lock_script = context.build_script(&out_point_token_buy_lock, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.extend(buyer_lock_script_hash.to_vec());
	data.extend(100u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(token_buy_lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	
	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(seller_lock_script.clone()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	outputs_data.push(Bytes::new());

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.cell_dep(token_buy_lock_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_AMOUNT).input_lock_script(0));
	// let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// // println!("consume cycles: {}", cycles);
}
