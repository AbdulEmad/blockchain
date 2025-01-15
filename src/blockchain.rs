use super::*;
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidFirstBlock,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn update_with_block(&self, block: Block) -> Result<(), BlockValidationError> {
        let i = self.blocks.len();
        if block.index != i as u32 {
            return Err(BlockValidationError::MistmatchedIndex);
        } else if !block::check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationErrror::InvalidHash);
        } else if i != 0 {
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationError::AchronologicalTimestamp);
            } else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationError::MismatchedPreviousHash);
            }
        } else {
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationError::InvalidFirstBlock);
            }
        }
        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }
            let mut block_spent = HashSet::new();

            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();
                if !(&input_hashes - &self.unspent_outputs).is_empty()
                    || &(&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockChainValidationErr::invalidInput);
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                if output_value > input_value {
                    return Err(BlockValidationError::insufficientInputValue);
                }

                let fee = input_value - output_value;
                total_fee += fee;
                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes);
            }

            if coinbase.output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs
                .retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }
        Ok(())
    }
}
