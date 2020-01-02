use near_bindgen::env;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};

pub type MerkleHash = [u8; 32];

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct MerklePathItem {
    pub hash: MerkleHash,
    pub direction: Direction,
}

pub type MerklePath = Vec<MerklePathItem>;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Right,
}

pub fn hash(item: &[u8]) -> MerkleHash {
    let vec = env::sha256(item);
    let mut res = [0; 32];
    res.copy_from_slice(&vec);
    res
}

pub fn combine_hash(hash1: MerkleHash, hash2: MerkleHash) -> MerkleHash {
    let mut combined: Vec<u8> = hash1.to_vec();
    combined.append(&mut hash2.to_vec());
    hash(&combined)
}

/// Merklize an array of items. If the array is empty, returns hash of 0
#[allow(unused)]
pub fn merklize<T: BorshSerialize>(arr: &[T]) -> (MerkleHash, Vec<MerklePath>) {
    if arr.is_empty() {
        return (MerkleHash::default(), vec![]);
    }
    let mut len = (arr.len() as u32).next_power_of_two();
    let mut hashes: Vec<_> = (0..len)
        .map(|i| {
            if i < arr.len() as u32 {
                hash(&arr[i as usize].try_to_vec().expect("Failed to serialize"))
            } else {
                hash(&[0])
            }
        })
        .collect();
    // degenerate case
    if len == 1 {
        return (hashes[0], vec![vec![]]);
    }
    let mut paths: Vec<MerklePath> = (0..arr.len())
        .map(|i| {
            if i % 2 == 0 {
                vec![MerklePathItem { hash: hashes[(i + 1) as usize], direction: Direction::Right }]
            } else {
                vec![MerklePathItem { hash: hashes[(i - 1) as usize], direction: Direction::Left }]
            }
        })
        .collect();

    let mut counter = 1;
    while len > 1 {
        len /= 2;
        counter *= 2;
        for i in 0..len {
            let hash = combine_hash(hashes[2 * i as usize], hashes[(2 * i + 1) as usize]);
            hashes[i as usize] = hash;
            if len > 1 {
                if i % 2 == 0 {
                    for j in 0..counter {
                        let index = ((i + 1) * counter + j) as usize;
                        if index < arr.len() {
                            paths[index].push(MerklePathItem { hash, direction: Direction::Left });
                        }
                    }
                } else {
                    for j in 0..counter {
                        let index = ((i - 1) * counter + j) as usize;
                        if index < arr.len() {
                            paths[index].push(MerklePathItem { hash, direction: Direction::Right });
                        }
                    }
                }
            }
        }
    }
    (hashes[0], paths)
}

/// Verify merkle path for given item and corresponding path.
pub fn verify_path<T: BorshSerialize>(root: MerkleHash, path: &MerklePath, item: &T) -> bool {
    let mut hash = hash(&item.try_to_vec().expect("Failed to serialize"));
    for item in path {
        match item.direction {
            Direction::Left => {
                hash = combine_hash(item.hash, hash);
            }
            Direction::Right => {
                hash = combine_hash(hash, item.hash);
            }
        }
    }
    hash == root
}
