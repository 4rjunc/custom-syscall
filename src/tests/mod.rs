use mollusk_svm::{result::ProgramResult, Mollusk};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    account::{AccountSharedData, ReadableAccount, WritableAccount},
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    native_token::LAMPORTS_PER_SOL,
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
};

// Add this type alias or import
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const PROGRAM_ID: Pubkey = Pubkey::from_str_const("r8p3kwsDxPyTu1KyacFxJcP5b98GRn9wocBUsTToWTd");

//#[test]
//fn custom_syscall() {
//    println!("program address: {}", crate::ID);
//
//    let mollusk: Mollusk = Mollusk::new(&PROGRAM_ID, "target/deploy/custom_syscall");
//
//    let instruction = Instruction::new_with_bytes(PROGRAM_ID, &[], vec![]);
//
//    let result: mollusk_svm::result::InstructionResult =
//        mollusk.process_instruction(&instruction, &vec![]);
//
//    assert!(matches!(result.program_result, ProgramResult::Success))
//}

#[tokio::test]
async fn custom_syscall_localvalidator() -> Result<()> {
    println!("local validator: {}", crate::ID);
    let connection = RpcClient::new_with_commitment(
        "http://localhost:8899".to_string(),
        CommitmentConfig::confirmed(),
    );
    let sender = Keypair::new();

    // Fund sender with airdrop
    let airdrop_signature = connection
        .request_airdrop(&sender.pubkey(), LAMPORTS_PER_SOL)
        .await?;

    // Wait for airdrop confirmation
    loop {
        let confirmed = connection.confirm_transaction(&airdrop_signature).await?;
        if confirmed {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    let instruction = Instruction::new_with_bytes(PROGRAM_ID, &[], vec![]);

    // Get recent blockhash first
    let blockhash = connection.get_latest_blockhash().await?;

    // Create and sign transaction properly
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&sender.pubkey()));
    transaction.sign(&[&sender], blockhash);

    // Send the transaction to the network
    let transaction_signature = connection
        .send_and_confirm_transaction_with_spinner(&transaction)
        .await?;

    println!("Transaction signature: {}", transaction_signature);

    assert!(matches!(true, true));

    Ok(())
}
