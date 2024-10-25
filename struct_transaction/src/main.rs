fn main() {
   let sender = String::from("178LHoUYmQH6H28VYfb1say5U2xyCWYwpP");
   let receiver = String::from("1MGfAZDDegJDvob83Bo1o7Eohifmx8Jeiz");
   let amount: u32 = 1;

   new_transaction(sender,receiver,amount);
}

struct Transaction {
    sender: String,
    receiver: String,
    amount: u32,
}

fn new_transaction (sender: String, receiver: String, amount: u32) {
    let transaction = Transaction {
        sender: sender,
        receiver: receiver,
        amount: amount,
    };
    if amount == 0 {
        println!("Transaction failed");
        println!("The amount can't be 0");
    } else {
        println!(" Transaction approved!");
        println!("Valor de {} SOL enviados de {} para {}", transaction.amount, transaction.sender, transaction.receiver);
    }
}