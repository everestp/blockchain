
use std::{time::SystemTime, vec};

use sha2::{Digest, Sha256};



#[derive(Debug)]
 pub struct  Block{
    nonce:i32,
    previous_hash:Vec<u8>,
    time_stamps :u128,
    transactions:Vec<Vec<u8>>

}
impl Block {
    //method for the struct , class methods
    //Two kind of method ,
    //One kind ->Static method = which  not reading or writing  into field of the block

    //Self is alias name for object , if we change the name of struct 
    //then we do not  need to change the name inside here
    pub fn new(nonce:i32 , previous_hash:Vec<u8>)-> Self{
        //this method will take control; of the input of previous_hash
         let time_now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
         Block { 
            nonce:nonce,
            previous_hash:previous_hash,
            time_stamps:time_now.as_nanos(),
            transactions : Vec::<Vec<u8>>::new()

          }

    }

// struct method which is need to read or write to  field to stuct
//self  which reference to the struct instance
 pub fn print(&self){
    //format value of hex
    println!("nonce : {}",self.nonce);

    println!("Timestamp :{:x}",self.time_stamps);

    //print vector , ask thje compiler  to do it
    println!("previous haxsh :{:?}",self.previous_hash);
    println!(" transaction:{:?}",self.transactions);

}
  pub fn hash(&self)-> Vec<u8>{
        let mut bin = Vec::<u8>::new();
        bin.extend(self.nonce.to_be_bytes());
        bin.extend(self.previous_hash.clone());
        bin.extend(self.time_stamps.to_be_bytes());
        for tx in self.transactions.iter(){
            bin.extend(tx.clone());
        }

     let mut hasher = Sha256::new();
     hasher.update(bin);

     return hasher.finalize().to_vec();
     
    }

}

#[derive(Debug)]
 pub struct  BlockChain{
    transaction_pool : Vec<Vec<u8>>,
    chain : Vec<Block>,
}

impl BlockChain{
   pub fn new() -> Self {
        let mut bc = BlockChain { 
            transaction_pool : Vec::<Vec<u8>>::new(),
            chain:Vec::<Block>::new(),
        };

        bc.create_block(0 , vec![0 as u8 ,32]);
        return bc;

    } 

   pub  fn create_block(&mut self , nonce:i32 , previous_hash:Vec<u8>){
        let b = Block::new(nonce, previous_hash);
        self.chain.push(b);
    }
    pub fn print(&self){
        // using  iterator  to loop over the vector
        for(i ,block) in self.chain.iter().enumerate(){
           println!(" {} Chain {} {}","=".repeat(25),i,"=".repeat(25));
           block.print();
        }
        println!("{}","*".repeat(25))
    }

     pub fn  last_block(&self) -> &Block{
        if self.chain.len() > 1 {
            return  &self.chain[self.chain.len()-1];
        } else{
            return  &self.chain[0];
        }
    }

  
}
