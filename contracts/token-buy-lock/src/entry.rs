// Import from alloc core instead of from std since we are in no-std mode.
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use core::result::Result;

// Import CKB syscalls and structures.
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
// use ckb_std::debug;
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::{bytes::Bytes, prelude::*};
use ckb_std::high_level::{load_script, load_cell_lock_hash, load_cell_type_hash, load_cell_data, QueryIter};

// Import our local error codes.
use crate::error::Error;

// Constants
const SCRIPT_HASH_LEN: usize = 32; // Number of bytes for a lock hash. (Blake2b 256-bit 32 bytes)
const SUDT_DATA_LEN: usize = 16; // SUDT uses a u128, which is 16 bytes.

/// Determine if owner mode is enabled.
fn check_owner_mode() -> Result<bool, Error>
{
	// Multiple cells can have multiple owners since the owner hash is stored in data and not in the args.
	// All owners must be checked.

	// Find all the unique owners.
	let mut owner_lock_hashes = BTreeSet::new();
	for data in QueryIter::new(load_cell_data, Source::GroupInput)
	{
		// If the data is less than the required length, return an error.
		if data.len() < SCRIPT_HASH_LEN + SUDT_DATA_LEN
		{
			return Err(Error::DataLength);
		}

		owner_lock_hashes.insert(data[0..SCRIPT_HASH_LEN].to_vec());
	}

	// Find all input lock hashes.
	let mut input_lock_hashes = BTreeSet::new();
	for input_lock_hash in QueryIter::new(load_cell_lock_hash, Source::Input)
	{
		input_lock_hashes.insert(input_lock_hash.to_vec());
	}

	// Check if all owner lock hashes are present in the input lock hashes, indicating owner mode.
	for owner_lock_hash in owner_lock_hashes
	{
		// If any owner hash is not present, owner mode is not enabled.
		if !input_lock_hashes.contains(&owner_lock_hash)
		{
			return Ok(false)
		}
	}

	Ok(true)
}

/// Calculate the required SUDT tokens from all token buy cells, grouped by owner lock hash..
fn calculate_token_buy_cell_requirements() -> Result<BTreeMap<Vec<u8>, u128>, Error>
{
	let mut requirements = BTreeMap::new();

	// Cycle though all Token Buy Lock cell data.
	let cell_data = QueryIter::new(load_cell_data, Source::GroupInput);
	for data in cell_data
	{
		// If the data is less than the required length, return an error.
		if data.len() < SCRIPT_HASH_LEN + SUDT_DATA_LEN
		{
			return Err(Error::DataLength);
		}

		// Extract owner lock hash and buy amount.
		let owner_lock_hash = data[0..SCRIPT_HASH_LEN].to_vec();
		let mut buffer = [0u8; SUDT_DATA_LEN];
		buffer.copy_from_slice(&data[SCRIPT_HASH_LEN..SCRIPT_HASH_LEN+SUDT_DATA_LEN]);
		let buy_amount = u128::from_le_bytes(buffer);

		// Add key if it doesn't already exist.
		if !requirements.contains_key(&owner_lock_hash)
		{
			requirements.insert(owner_lock_hash.clone(), 0);
		}

		// Add buy amount to requirements for current owner lock hash.
		let amount = requirements.get_mut(&owner_lock_hash).unwrap();
		*amount += buy_amount;
	}

	Ok(requirements)
}

/// Count the number of tokens in the specified source with the specified lock hash and type hash.
fn determine_token_amount(source: Source, lock_hash: &[u8], type_hash: &[u8]) -> Result<u128, Error>
{
	// Track the number of tokens that are counted.
	let mut total_token_amount = 0;

	// Cycle through the data in each cell within the specified source.
	let cell_data = QueryIter::new(load_cell_data, source);
	for (i, data) in cell_data.enumerate()
	{
		// Extract the type script hash from the current cell.
		let cell_type_hash = load_cell_type_hash(i, source)?;
        if cell_type_hash.is_none() { continue; }
        let cell_type_hash = cell_type_hash.unwrap();

		// Extract the lock script hash from the current cell.
		let cell_lock_hash = load_cell_lock_hash(i, source)?;

        // Check that the length of the data is >= 16 bytes, the size of a u128 and that the lock hash on the cell matches the specified.
		if cell_lock_hash == lock_hash && cell_type_hash == type_hash
		{
			// If the data is less than 16 bytes, then return an encoding error.
			if data.len() < SUDT_DATA_LEN
			{
				return Err(Error::Encoding);
			}

			// Convert the binary data in the cell to a u128 value.
			let mut buffer = [0u8; SUDT_DATA_LEN];
			buffer.copy_from_slice(&data[0..SUDT_DATA_LEN]);
			let amount = u128::from_le_bytes(buffer);

			// Add the amount of tokens in the cell to the total amount of tokens.
			total_token_amount += amount;
		}
	}

	// Return the total amount of tokens found in the specified source.
	Ok(total_token_amount)
}

/// Validate the token buy requirements against the output token amounts.
fn validate_token_buy_requirements(token_buy_requirements: &BTreeMap<Vec<u8>, u128>, sudt_type_script_hash: &[u8]) -> Result<(), Error>
{
	for (owner_lock_hash, buy_amount) in token_buy_requirements
	{
		// Count the number of tokens being sent to the owner lock hash.
		let output_token_amount = determine_token_amount(Source::Output, &owner_lock_hash[..], sudt_type_script_hash)?;

		// If not enough tokens we provided, return an error.
		if output_token_amount < *buy_amount
		{
			// debug!("{:?} {} {:?} {}", owner_lock_hash, *buy_amount, sudt_type_script_hash, output_token_amount);
			return Err(Error::Amount);
		}
	}

	Ok(())
}

// Main entry point.
pub fn main() -> Result<(), Error>
{
	// Load the currently executing script and get the args.
	let script = load_script()?;
	let args: Bytes = script.args().unpack();

	// Verify the the arguments length matches the length of a single Blake2b hash.
	if args.len() != SCRIPT_HASH_LEN
	{
		return Err(Error::ArgsLength);
	}

	// Check if the script is being run by the owner and immediately return success if true.
	if check_owner_mode()?
	{
		return Ok(());
	}

	// Verify the number of Token Buy Lock cells in the transaction is one.
	let token_buy_requirements = calculate_token_buy_cell_requirements()?;

	// Ensure that the token buy requirements have been met.
	validate_token_buy_requirements(&token_buy_requirements, &args[0..SCRIPT_HASH_LEN])?;

	// No errors were found during validation. Return success.
	Ok(())
}
