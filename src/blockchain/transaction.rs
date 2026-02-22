use crate::blockchain::*;
use std::fmt;
#[derive(Debug, Serialize)]
pub struct Transaction {
   pub sender_address : Vec<u8>,
   pub recipient_address :Vec<u8>,
   pub value : f64,
}
impl  Transaction {
    pub fn new(sender:Vec<u8>, receipient:Vec<u8>,value:f64)-> Transaction{
        Transaction {
             sender_address: sender,
             recipient_address: receipient, 
             value
            }
    }
}
impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Repeat 40 dashes
        let separator = "-".repeat(40);

        write!(
            f,
            "{}\nSender address : {:?}\nRecipient address : {:?}\nValue : {}\n{}",
            separator,
            self.sender_address,
            self.recipient_address,
            self.value,
            separator
        )
    }
}

impl Serialization<Transaction> for Transaction {
    fn deserialization(bytes: Vec<u8>) -> Transaction {
        let mut pos = 0;

        // Deserialize sender_address
        let len_sender = usize::from_be_bytes(bytes[pos..pos+8].try_into().unwrap());
        pos += 8;
        let sender_address = bytes[pos..pos+len_sender].to_vec();
        pos += len_sender;

        // Deserialize recipient_address
        let len_recipient = usize::from_be_bytes(bytes[pos..pos+8].try_into().unwrap());
        pos += 8;
        let recipient_address = bytes[pos..pos+len_recipient].to_vec();
        pos += len_recipient;

        // Deserialize value (assuming it's u64)
        let len_value = usize::from_be_bytes(bytes[pos..pos+8].try_into().unwrap());
        pos += 8;
        let value = f64::from_be_bytes(bytes[pos..pos+len_value].try_into().unwrap());
       

        Transaction {
            sender_address,
            recipient_address,
            value,
        }
    }

    fn serialization(&self) -> Vec<u8> {
        let mut bin = Vec::<u8>::new();

        let len_sender = self.sender_address.len() as u64;
        bin.extend(&len_sender.to_be_bytes());
        bin.extend(&self.sender_address);

        let len_recipient = self.recipient_address.len() as u64;
        bin.extend(&len_recipient.to_be_bytes());
        bin.extend(&self.recipient_address);

        let value_bytes = self.value.to_be_bytes();
        let len_value = value_bytes.len() as u64;
        bin.extend(&len_value.to_be_bytes());
        bin.extend(&value_bytes);

        bin
    }
}
    
