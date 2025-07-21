use mollusk_svm::{result::ProgramResult, Mollusk};
use solana_sdk::{
    account::{AccountSharedData, ReadableAccount, WritableAccount},
    instruction::Instruction,
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
};
#[test]
fn custom_syscall() {
    println!("program addres: {}", crate::ID);
    let _fake_address = Pubkey::new_from_array([1u8; 32]);
    let mollusk: Mollusk = Mollusk::new(&crate::ID, "target/deploy/custom_syscall");

    // Create our
    let instruction = Instruction::new_with_bytes(crate::ID, &[], vec![]);

    let result: mollusk_svm::result::InstructionResult =
        mollusk.process_instruction(&instruction, &vec![]);

    assert!(matches!(result.program_result, ProgramResult::Success))
}
