// use kucoin_rs::kucoin::client::{Kucoin, Credentials, KucoinEnv};
// use kucoin_rs::kucoin::error::APIError;
use kucoin_rs::kucoin::client::{Kucoin, KucoinEnv};
use kucoin_rs::failure;
use failure::format_err;
use rand::distributions::Uniform;
use rand::distributions::Distribution;

struct Withdrawal {
    address: String,
    amount: i32,
}

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    
    let _currency = "SOL";
    let _acct_type = "main";
    let _chain = "OMNI";
    
    // let kucoin_client = Kucoin::new(KucoinEnv::Live, None).expect("failed to get found Kucoin sdk");

    // let _checked_sol_balance = check_balance(kucoin_client.clone(), _currency, _acct_type).await;
    // let _withdrawal_fee = check_withdrawal_fee(kucoin_client.clone(), _currency, _chain).await;

    // match _withdrawal_fee {
    //     Ok(fee) => println!("Withdrawal fee: {}", fee),
    //     Err(e) => eprintln!("Failed to get withdrawal fee: {}", e),
    // }

    // let address = "6DSudNrFeasRtUjAfDeCF8DFUFm1UiLFnbgvGQNRMPGj";
    // let amount = 1; // Example amount
    
    // withdraw_sol(kucoin_client.clone(), address, amount).await?;

    execute_withdrawals(1000, 5000, "SOL", "main", "OMNI").await?;
    Ok(())
}


async fn check_balance(
    kucoin_client: Kucoin,
    currency: &str,
    acct_type: &str  //Option<&str>
) -> Result<i32, failure::Error> {
    match kucoin_client.get_accounts_list(Some(currency), Some(acct_type)).await {
        Ok(result) => {
            println!("Available SOL ACCOUNTS: {:?}", result);
            match result.data {
                None => {
                    println!("accounts.data is None !");
                    return Err(format_err!("Withdrawal fee data not found").into());
                }
                Some(data) => {
                    let sol_account = data.iter().find(|account| account.currency == "SOL");
                    if let Some(account) = sol_account {
                        println!("Available SOL: {}", account.available);
                        let number: i32 = account.available.parse().unwrap_or(0);
                        return Ok(number);
                    } else {
                        return Ok(0); // Return 0 if no SOL account is found
                    }
                },
            }
        },
        Err(e) => {
            eprintln!("Error getting accounts list: {:?}", e);
            return Err(e.into());
        },
    }

}

async fn check_withdrawal_fee(
    kucoin_client: Kucoin,
    currency: &str,
    chain: &str  //Option<&str>
) -> Result<i32, failure::Error> {
    match kucoin_client.get_withdrawals_quotas(currency, Some(chain)).await {
        Ok(result) => {
            println!("get_withdrawals_quotas RESULT: {:?}", result);
            match result.data {
                None => {
                    println!("result.data of withdrawal_quotas is None !");
                    // let err_box = format_err!("Withdrawal fee data not found").into();
                    return Err(format_err!("Withdrawal fee data not found").into());
                },
                Some(data) => {
                    let fee = data.withdrawal_min_fee.parse().unwrap_or(0);
                    println!("Withdrawal fee for SOL: {}", fee);
                    return Ok(fee);
                },
            }           
        },
        Err(e) => {
            eprintln!("Error fetching withdrawal fee: {}", e);
            return Err(e.into());
        }
    }
}

async fn withdraw_sol(
    kucoin_client: Kucoin,
    address: &str,
    amount: i32
) -> Result<(), failure::Error> {
    
    match kucoin_client.apply_withdrawal("SOL", address, amount, Some("sol"), None, None, None).await {
        Ok(result) => println!("Withdrawal to {} of {} SOL successful: {:?}", address, amount, result),
        Err(e) => eprintln!("Failed to withdraw to {}: {:?}", address, e),
    }

    Ok(())
}


async fn execute_withdrawals(
    min_delay: u64,
    max_delay: u64, 
    _currency: &str, 
    _acct_type: &str, 
    _chain: &str
)-> Result<(), failure::Error> {
    let kucoin_client = Kucoin::new(KucoinEnv::Live, None)?;

    let check_balance_result = check_balance(kucoin_client.clone(), _currency, _acct_type).await?;
    let check_withdrawal_fee_result = check_withdrawal_fee(kucoin_client.clone(), _currency, _chain).await?;

    let mut remaining_balance = check_balance_result;
    let mut withdrawal_fee = check_withdrawal_fee_result;
    let withdrawal_list = vec![
        Withdrawal { address: "address1".to_string(), amount: 10 },
        Withdrawal { address: "address2".to_string(), amount: 20 },
        // Add more withdrawals as needed
    ];

    for withdrawal in withdrawal_list {
        let total_cost = withdrawal.amount + withdrawal_fee;

        if withdrawal_fee == 0 || total_cost <= remaining_balance {
            withdraw_sol(kucoin_client.clone(), &withdrawal.address, withdrawal.amount).await?;
            
            remaining_balance = remaining_balance - total_cost;
            println!(
                "Withdrawal of {} SOL to {} was successful. Remaining balance: {}",
                withdrawal.amount, withdrawal.address, remaining_balance
            );

            let delay = Uniform::new(min_delay, max_delay + 1).sample(&mut rand::thread_rng());
            println!("Waiting for {} seconds before next withdrawal...", delay / 1000);
            tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;

        } else {
            println!(
                "Not enough balance to withdraw {} SOL to {}. Skipping.",
                withdrawal.amount, withdrawal.address
            );
        }
    }

    println!("All withdrawals processed.");
    Ok(())
}