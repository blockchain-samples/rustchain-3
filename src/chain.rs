use std::fmt;

const HASH_LEN: usize = 32;

type BlockIndex = usize;
pub struct Hash {
    bytes: [u8; HASH_LEN],
}

impl Hash {
    fn zeros() -> Hash {
        Hash {
            bytes: [0; HASH_LEN],
        }
    }

    fn from_bytes<I>(iter: I) -> Hash
    where
        I: Iterator<Item = u8>,
    {
        // assert_eq!(iter.len(), HASH_LEN);
        let mut bytes = [0; HASH_LEN];
        for (idx, b) in iter.enumerate() {
            // Will panic if more than 32 bytes
            bytes[idx] = b;
        }
        Hash { bytes: bytes }
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use data_encoding::HEXLOWER;
        write!(f, "{}", HEXLOWER.encode(&self.bytes))
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use data_encoding::HEXLOWER;
        write!(f, "{}", HEXLOWER.encode(&self.bytes))
    }
}


#[derive(Debug)]
pub struct Block {
    pub index: BlockIndex,
    pub timestamp: usize,
    pub transactions: Vec<Transaction>,
    pub proof: usize,
    pub previous_hash: Hash,
}

impl Block {
    pub fn hash(&self) -> Hash {
        use blake2::{Blake2s, Digest};

        let mut hasher = Blake2s::default();


        // TODO: Replace by consistent serialisation (e.g. using Serde)
        hasher.input(format!("{:?}", self.index).as_bytes());
        hasher.input(format!("{:?}", self.timestamp).as_bytes());
        hasher.input(format!("{:?}", self.transactions).as_bytes());
        hasher.input(format!("{:?}", self.proof).as_bytes());
        hasher.input(format!("{}", self.previous_hash).as_bytes());

        Hash::from_bytes(hasher.result().into_iter())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: usize,
}

impl Transaction {
    pub fn new<T: AsRef<str>>(from: T, to: T, amount: usize) -> Transaction {
        Transaction {
            sender: from.as_ref().to_string(),
            recipient: to.as_ref().to_string(),
            amount: amount,
        }
    }
}

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub current_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut bc = Blockchain {
            chain: Vec::new(),
            current_transactions: Vec::new(),
        };
        bc.add_block_with_previous_hash(42, Hash::zeros());
        bc
    }
    pub fn add_block_with_previous_hash(&mut self, proof: usize, previous_hash: Hash) {
        let block = Block {
            index: self.chain.len() + 1,
            timestamp: 0, // HACK: For the time being
            transactions: self.current_transactions.clone(),
            proof: proof,
            previous_hash: previous_hash,
        };

        // Reset the current list of transactions
        self.current_transactions = Vec::new(); // TODO: Delete elements instead of reallocating

        self.chain.push(block)
    }

    pub fn add_block(&mut self, proof: usize) {
        let previous_hash = self.last_block().hash();
        self.add_block_with_previous_hash(proof, previous_hash);
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> BlockIndex {
        self.current_transactions.push(transaction);

        self.last_block().index + 1
    }

    pub fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
}

pub fn proof_of_work(last_proof: usize) -> usize {
    let mut proof = 0;
    while !valid_proof(last_proof, proof) {
        proof += 1;
    }
    proof
}

fn valid_proof(last_proof: usize, proof: usize) -> bool {
    let guess = format!("{}{}", last_proof, proof);
    use blake2::{Blake2s, Digest};
    let mut hasher = Blake2s::default();
    hasher.input(guess.as_bytes());
    let guess_hash = format!("{}", Hash::from_bytes(hasher.result().into_iter()));
    &guess_hash[..2] == "00"
}
