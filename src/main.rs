use std::env;
use std::error::Error;

use kucoin_rs::kucoin::client::{Kucoin, Credentials, KucoinEnv};

struct Withdrawal {
    address: String,
    amount: f64,
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    // let credentials = Credentials::new(
    //     "xxxxxxxxxxxxxXXXXXXxxx",           // API KEY
    //     "XXxxxxx-xxxxxx-xXxxxx-xxxx",       // SECRET KEY
    //     "xxxxxx"                            // PASSPHRASE
    // );
    // let api = Kucoin::new(KucoinEnv::Live, Some(credentials));
    let api = Kucoin::new(KucoinEnv::Live, None);

    let withdrawal_list = vec![
        Withdrawal {
            address: String::from("6DSudNrFeasRtUjAfDeCF8DFUFm1UiLFnbgvGQNRMPGj"),
            amount: 0.15,
        },
    ];

    let minDelay = 1000; // Minimum delay in milliseconds (5 seconds)
    let maxDelay = 5000; // Maximum delay in milliseconds (10 seconds)

    for withdrawal in &withdrawal_list {
        println!("Address: {}, Amount: {}", withdrawal.address, withdrawal.amount);
    }

    
    Ok(())
}
