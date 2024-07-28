use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
};

use bs58;
use std::io::{self, BufRead};

const RPC_URL: &str = "https://api.devnet.solana.com";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet byte array:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");

        let stdin = io::stdin();

        let base58 = stdin.lock().lines().next().unwrap().unwrap();

        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec();

        match wallet {
            Ok(w) => println!("{:?}", w),
            Err(e) => println!("Error: {:?}", e),
        }
    }

    #[test]
    fn airdrop() {
        let kp = match read_keypair_file("dev-wallet.json") {
            Ok(kp) => kp,
            Err(e) => {
                println!("Error: {:?}", e);
                return;
            }
        };

        println!("Wallet loaded: {:?}", kp.pubkey());

        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&kp.pubkey(), 2 * 1_000_000_000u64) {
            Ok(sig) => {
                println!(
                    "Airdrop successful: https://explorer.solana.com/tx/{}?cluster=devnet",
                    sig.to_string()
                );
            }
            Err(e) => {
                println!("Ooops, something went wrong {:?}", e);
            }
        }
    }

    #[test]
    fn transfer() {}
}
