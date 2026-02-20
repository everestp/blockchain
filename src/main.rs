use std::time::SystemTime;



#[derive(Debug)]
struct  Block{
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
    fn new(nonce:i32 , previous_hash:Vec<u8>)-> Self{
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
fn print(&self){
    //format value of hex
    println!("nonce : {}",self.nonce);

    println!("Timestamp :{:x}",self.time_stamps);

    //print vector , ask thje compiler  to do it
    println!("previous haxsh :{:?}",self.previous_hash);
    println!(" transaction:{:?}",self.transactions);

}

}

#[derive(Debug)]
struct  BlockChain{
    transaction_pool : Vec<Vec<u8>>,
    chain : Vec<Block>,
}

impl BlockChain{
    fn new() -> Self {
        let mut bc = BlockChain { 
            transaction_pool : Vec::<Vec<u8>>::new(),
            chain:Vec::<Block>::new(),
        };

        bc.create_block(0 , "Hash for very first block ".to_string().into_bytes());
        return bc;

    } 

    fn create_block(&mut self , nonce:i32 , previous_hash:Vec<u8>){
        let b = Block::new(nonce, previous_hash);
        self.chain.push(b);
    }
    fn print(&self){
        // using  iterator  to loop over the vector
        for(i ,block) in self.chain.iter().enumerate(){
           println!(" {} Chain {} {}","=".repeat(25),i,"=".repeat(25));
           block.print();
        }
        println!("{}","*".repeat(25))
    }
}

fn main() {

    // let b = Block::new(0, "this is first block".to_string().into_bytes());
    // println!("Hello, world!");
    // b.print();
    // println!("the first block is :{:?}",b);
    let mut block_chain = BlockChain::new();
    println!("Block chain :{:?}",block_chain);
    block_chain.print();
    block_chain.create_block(1, "hash-1".to_string().into_bytes());
     block_chain.create_block(2, "hash-2".to_string().into_bytes());
      block_chain.print();
}


