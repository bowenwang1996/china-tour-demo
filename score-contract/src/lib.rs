use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::{
    env,
    ext_contract,
    near_bindgen,
    Promise,
};
use near_bindgen::collections::Map;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};

mod merkle;
use merkle::{MerkleHash, MerklePath, merklize};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const MAX_GAS: u64 = 1_000_000_000_000;

type BlockIndex = u64;
type Score = u64;


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct ScoreContract {
    pub scores: Map<String, BTreeMap<BlockIndex, Score>>,
    pub proofs: Map<(String, BlockIndex), MerklePath>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ScoreWithProof {
    pub name: String,
    pub block_index: BlockIndex,
    pub score: Score,
    pub proof: MerklePath,
}

#[ext_contract(scholarship)]
pub trait ExtScholarshipContract {
    fn record_root(&mut self, root: MerkleHash, block_index: BlockIndex);
}

#[near_bindgen]
impl ScoreContract {
    pub fn record_score(&mut self, name: String, score: u64) -> Promise {
        let block_index = env::block_index();
        let mut map = self.scores.remove(&name).unwrap_or_else(BTreeMap::new);
        map.insert(block_index, score);
        self.scores.insert(&name, &map);
        let root = self.compute_root();
        scholarship::record_root(root, block_index, &"scholarship-contract".to_string(), 0, MAX_GAS)
    }

    fn compute_root(&mut self) -> MerkleHash {
        let mut kv_pairs = vec![];
        for key in self.scores.keys() {
            let value = self.scores.get(&key).unwrap();
            for (k, v) in value {
                kv_pairs.push(((key.clone(), k), v));
            }
        }
        let (root, paths) = merklize(&kv_pairs);
        for (key, path) in kv_pairs.iter().zip(paths.into_iter()) {
            let (key, _) = key;
            self.proofs.insert(key, &path);
        }
        root
    }

    pub fn prove_score(&self, name: String, block_index: BlockIndex) -> Option<ScoreWithProof> {
        self.scores.get(&name).and_then(|map| {
            map.range((Unbounded, Included(block_index))).rev().next().and_then(|(index, score)| {
                self.proofs.get(&(name.clone(), *index)).map(|proof| {
                    ScoreWithProof {
                        name,
                        block_index: *index,
                        score: *score,
                        proof
                    }
                })
            })
        })
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

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_bindgen::MockedBlockchain;
    use near_bindgen::{testing_env, VMContext};
    use crate::merkle::verify_path;

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(14),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn test_prove_score() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = ScoreContract::default();
        contract.record_score("bowen".to_string(), 0);
        contract.record_score("illia".to_string(), 100);
        let score_with_proof = contract.prove_score("bowen".to_string(), 100).unwrap();
        assert_eq!(score_with_proof.name, "bowen".to_string());
        assert_eq!(score_with_proof.block_index, 0);
        assert_eq!(score_with_proof.score, 0);
        let root = contract.compute_root();
        assert!(verify_path(
            root,
            &score_with_proof.proof,
            &((score_with_proof.name, score_with_proof.block_index), score_with_proof.score)
        ));

    }
}
