use merkletree::merkle::MerkleTree;
use merkletree::store::VecStore;
use sha2::{Digest, Sha256};

pub struct MerkleTreeState {
    tree: MerkleTree<[u8; 32], Sha256, VecStore<[u8; 32]>>,
}

impl MerkleTreeState {
    pub fn new() -> Self {
        let empty_leaves = vec![[0; 32]; 4]; 
        let tree = MerkleTree::from_leaves(&empty_leaves);
        MerkleTreeState { tree }
    }

    pub fn update_state(&mut self, txs: &[&str]) -> [u8; 32] {
        let new_leaves: Vec<[u8; 32]> = txs
            .iter()
            .map(|tx| Sha256::digest(tx.as_bytes()).into())
            .collect();

        self.tree = MerkleTree::from_leaves(&new_leaves);
        self.tree.root()
    }
}
