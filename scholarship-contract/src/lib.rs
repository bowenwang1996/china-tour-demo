use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::{
    callback_args,
    env,
    ext_contract,
    near_bindgen,
    Promise,
};
use near_bindgen::collections::Map;
use serde::{Serialize, Deserialize};

mod merkle;
use merkle::{MerkleHash, MerklePath, verify_path};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const SCHOLARSHIP_THRESHOLD: u64 = 90;
const SCHOLARSHIP_AMOUNT: u128 = 10 * NEAR_BASE;
const NEAR_BASE: u128 = 1_000_000_000_000_000_000_000_000;
const MAX_GAS: u64 = 1_000_000_000_000_000_000;

type BlockIndex = u64;
type Score = u64;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct ScholarshipContract {
    state_roots: Map<BlockIndex, MerkleHash>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreWithProof {
    pub name: String,
    pub block_index: BlockIndex,
    pub score: Score,
    pub proof: MerklePath,
}

#[ext_contract]
pub trait ExtScoreContract {
    fn prove_score(&mut self, name: String, block_index: BlockIndex) -> Option<ScoreWithProof>;
}

#[ext_contract(ext)]
pub trait ExtContract {
    fn check_scholarship_result(&mut self);
    fn grant_scholarship(&mut self, name: String);
}


#[near_bindgen]
impl ScholarshipContract {
    pub fn record_root(&mut self, root: MerkleHash, block_index: BlockIndex) {
        if env::predecessor_account_id() == "score-contract" {
            self.state_roots.insert(&block_index, &root);
        }
    }

    pub fn scholarship(&mut self, name: String, block_index: BlockIndex) -> Promise {
        ext_score_contract::prove_score(name.clone(), block_index, &"score-contract".to_string(), 0, MAX_GAS).then(
            ext::check_scholarship_result(&env::current_account_id(), 0, MAX_GAS)
        ).then(
            ext::grant_scholarship(name, &env::current_account_id(), 0, MAX_GAS)
        )
    }

    #[callback_args(proof)]
    pub fn check_scholarship_result(&mut self, proof: Option<ScoreWithProof>) -> Result<bool, String> {
        if let Some(proof) = proof {
            if let Some(root) = self.state_roots.get(&proof.block_index) {
                if Self::verify_proof(root, &proof) {
                    return Ok(proof.score >= SCHOLARSHIP_THRESHOLD);
                } else {
                    return Err("Score proof verification failed".to_string());
                }
            } else {
                let keys: Vec<_> = self.state_roots.keys().collect();
                return Err(format!("root doesn't exist, proof {:?}, state_roots: {:?}", proof, keys));
            }
        }
        Err("Proof doesn't exist".to_string())
    }

    #[callback_args(check_result)]
    pub fn grant_scholarship(&mut self, name: String, check_result: Result<bool, String>) {
        if let Ok(true) = check_result {
            Promise::new(name).transfer(SCHOLARSHIP_AMOUNT);
        }
    }

    fn verify_proof(root: MerkleHash, proof: &ScoreWithProof) -> bool {
        verify_path(root, &proof.proof, &((proof.name.clone(), proof.block_index), proof.score))
    }
}
