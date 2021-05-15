// Import from alloc core instead of from std since we are in no-std mode.
use alloc::vec::Vec;
use core::result::Result;

// Import CKB syscalls and structures.
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::{bytes::Bytes, prelude::*};
use ckb_std::high_level::{load_script, load_cell_lock_hash, load_cell_type_hash, load_cell_data, QueryIter};

// Import our local error codes.
use crate::error::Error;

// Constants
const SCRIPT_HASH_LEN: usize = 32; // Number of bytes for a lock hash. (Blake2b 256-bit 32 bytes)
const SUDT_DATA_LEN: usize = 16; // SUDT uses a u128, which is 16 bytes.

/// Determine if owner mode is enabled.
fn check_owner_mode(args: &Bytes) -> Result<bool, Error>
{
	// Compare the Lock Script Hash from the script args with the Lock Scripts
	// of each input cell to determine if a match exists.
	let is_owner_mode = QueryIter::new(load_cell_lock_hash, Source::Input)
		.find(|lock_hash|args[0..SCRIPT_HASH_LEN]==lock_hash[..]).is_some();

	// Return the owner mode status.
	Ok(is_owner_mode)
}

/// Count the number of Token Buy Lock cells in the transaction.
fn count_token_buy_lock_cells() -> Result<usize, Error>
{
	let count = QueryIter::new(load_cell_lock_hash, Source::GroupInput).map(|_|1).sum();

	Ok(count)
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

// Load the amount of SUDT tokens that are expected in exchange for CKB.
fn load_expected_token_amount(data: &Vec<u8>) -> u128
{
	// Convert the binary data in the cell to a u128 value.
    let mut buffer = [0u8; SUDT_DATA_LEN];
    buffer.copy_from_slice(&data[SCRIPT_HASH_LEN..SCRIPT_HASH_LEN+SUDT_DATA_LEN]);
    let amount = u128::from_le_bytes(buffer);

    amount
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
	if check_owner_mode(&args)?
	{
		return Ok(());
	}

	// Verify the number of Token Buy Lock cells in the transaction is one.
	if count_token_buy_lock_cells()? != 1
	{
		return Err(Error::TransactionStructure);
	}

	// Load the data from the Token Buy Lock cell and verify it is of a proper length.
	let token_buy_lock_data = load_cell_data(0, Source::GroupInput)?;
	if token_buy_lock_data.len() < SCRIPT_HASH_LEN + SUDT_DATA_LEN
	{
		return Err(Error::DataLength);
	}

	// Count the number of tokens being sent to the owner lock hash.
	let output_token_amount = determine_token_amount(Source::Output, &args[0..SCRIPT_HASH_LEN], &token_buy_lock_data[0..SCRIPT_HASH_LEN])?;

    // Load the expected SUDT token amount from the args.
    let expected_token_amount = load_expected_token_amount(&token_buy_lock_data);

	// If the amount of input tokens is less than the amount of expected output tokens, return an error.   
	if output_token_amount < expected_token_amount
	{
		return Err(Error::Amount);
	}

	// No errors were found during validation. Return success.
	Ok(())
}
