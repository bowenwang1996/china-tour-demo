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
const MAX_GAS: u64 = 1_000_000_000_000;

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
}


#[near_bindgen]
impl ScholarshipContract {
    pub fn record_root(&mut self, root: MerkleHash, block_index: BlockIndex) {
        if env::predecessor_account_id() == "score-contract" {
            self.state_roots.insert(&block_index, &root);
        }
    }

    pub fn scholarship(&mut self, name: String, block_index: BlockIndex) -> Promise {
        ext_score_contract::prove_score(name, block_index, &"score-contract".to_string(), 0, MAX_GAS).then(
            ext::check_scholarship_result(&env::current_account_id(), 0, MAX_GAS)
        )
    }

    #[callback_args(proof)]
    pub fn check_scholarship_result(&mut self, proof: Option<ScoreWithProof>) -> String {
        if let Some(proof) = proof {
            if let Some(root) = self.state_roots.get(&proof.block_index) {
                if Self::verify_proof(root, &proof) {
                    if proof.score >= SCHOLARSHIP_THRESHOLD {
                        return "Scholarship granted".to_string();
                    } else {
                        return "Score not high enough".to_string();
                    }
                } else {
                    return "Score proof verification failed".to_string();
                }
            } else {
                let keys: Vec<_> = self.state_roots.keys().collect();
                return format!("root doesn't exist, proof {:?}, state_roots: {:?}", proof, keys);
            }
        }
        "Proof is none".to_string()
    }

    fn verify_proof(root: MerkleHash, proof: &ScoreWithProof) -> bool {
        verify_path(root, &proof.proof, &((proof.name.clone(), proof.block_index), proof.score))
    }

    //pub fn simple_call(&mut self, account_id: String, message: String) {
    //    ext_status_message::set_status(message, &account_id, 0, 1_000_000);
    //}
    //pub fn complex_call(&mut self, account_id: String, message: String) -> Promise {
    //    // 1) call status_message to record a message from the signer.
    //    // 2) call status_message to retrieve the message of the signer.
    //    // 3) return that message as its own result.
    //    // Note, for a contract to simply call another contract (1) is sufficient.
    //    ext_status_message::set_status(message, &account_id, 0, 1_000_000).then(
    //        ext_status_message::get_status(env::signer_account_id(), &account_id, 0, 1_000_000),
    //    )
    //}
}
