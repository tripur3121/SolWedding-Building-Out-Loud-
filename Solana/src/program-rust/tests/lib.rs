use borsh::BorshDeserialize;
use helloworld::{process_instruction, GreetingAccount};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
use std::mem;

#[tokio::test]
async fn test_helloworld() {
    let program_id = Pubkey::new_unique();
    let greeted_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "helloworld", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );
    program_test.add_account(
        greeted_pubkey,
        Account {
            lamports: 5,
            data: vec![0_u8; mem::size_of::<u32>()],
            owner: program_id,
            ..Account::default()
        },
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Verify account has zero greetings
    let greeted_account = banks_client
        .get_account(greeted_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    assert_eq!(
        GreetingAccount::try_from_slice(&greeted_account.data)
            .unwrap()
            .counter,
        0
    );

    // Greet once
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[0], // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(greeted_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account has one greeting
    let greeted_account = banks_client
        .get_account(greeted_pubkey)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    assert_eq!(
        GreetingAccount::try_from_slice(&greeted_account.data)
            .unwrap()
            .counter,
        1
    );

    // Rust Program
    use std::io;

fn main() {
    let mut input = String::new();
    println!("Would you like to register your wedding on Solana Blockchain? (y/n)");
    match io::stdin().read_line(&mut input) {
        Ok(y) => {
            println!("{} ", input);
            // println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }
    let mut answer = String::new();
    println!("Enter your ID user1, Please");
    match io::stdin().read_line(&mut answer) {
        Ok(y) => {
            println!("user1:{} ", answer);
        }
        Err(error) => println!("error: {}", error),
    }
    let mut user = String::new();
    println!("Enter your ID user2, Please");
    match io::stdin().read_line(&mut user) {
        Ok(y) => {
            println!("user2:{} ", user);
        }
        Err(error) => println!("error: {}", error),
    }
}
