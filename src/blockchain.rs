use super::*;
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidFirstBlock,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn update_with_block (&self, block: Block) -> Result<(), BlockValidationError>{
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
    }
    if let Some((coinbase, transactions)) = block.transactions.split_first() {
        if !coinbase.is_coinbase(){
            return Err(BlockValidationErr:InvalidCoinbaseTransaction);
        }
        let mut block_spent = HashSet::new();
    }

    let mut block_spent: HashSet<Hash> = HashSet::new();
    Ok(()) 
    }
}
