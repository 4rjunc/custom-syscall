#  Custom Syscall In Agave Validator 

this is simple PoW on creating a custom Syscall in agave validator. my goal was to make simple syscall from program that will return a string or number. 
```rust
msg!("sol_get_magic_number {:?}", sol_get_magic_number()); // this return 2002 number
```

> first step i follow while building on complex tech is to acheive a simple working code and build complex stuff after it. 

clone the these of anza repos 
- [agave](https://github.com/anza-xyz/agave.git)
- [solana-sdk](https://github.com/anza-xyz/solana-sdk)
- [system](https://github.com/solana-program/system)
- [stake](https://github.com/solana-program/stake)

first i tried to find the registered sycalls in `solana-sdk` repo. there are couple of it. i searched for a simple one and found this `fn sol_remaining_compute_units() -> u64` ([here](https://github.com/4rjunc/solana-sdk/blob/62c9fc28cd556a2d5c4695b5b550c6967e2e6962/define-syscall/src/definitions.rs#L30)). so i searched for the function name `sol_remaining_compute_units()` in solana-sdk and agave repo and understood the parts of code where `sol_remaining_compute_units()` name mentioned and logic defined. 

Now I know where all the a syscall needs to defined and registered. so i began to implement a simple syscall named `sol_get_magic_number()` which will return a number `2002`. there will be better of doing this mine is not systematic. im sharing the wat how i made changes to the code. i have checked out to a new branch name `custom-syscall` in locally cloned agave and solana-sdk repo. 

1. Define syscall in `solana-sdk/define-syscall`

```rust
// CUSTOM SYSCALL
define_syscall!(fn sol_get_magic_number() -> u64);
// CUSTOM SYSCALL
```
[here](https://github.com/4rjunc/solana-sdk/blob/62c9fc28cd556a2d5c4695b5b550c6967e2e6962/define-syscall/src/definitions.rs#L32C20-L32C40)

2. In agave repo `agave/program/bpf_loader`

you can find the `sol_remaining_compute_units` name nearby my code changes

 - ```rust
        // CUSTOM SYSCALL: Accessing the magic number
        register_feature_gated_function!(
            result,
            remaining_compute_units_syscall_enabled,
            "sol_get_magic_number",
            SyscallGetMagicNumnberSysvar::vm
        )?;
    ```

    [here](https://github.com/4rjunc/agave/blob/526a10c73f1de9ef36bae5e277fc054d3aee85fe/programs/bpf_loader/src/syscalls/mod.rs#L523)

 - define the logic 
 ```rust
// custom sysvar: jus returns a number
declare_builtin_function!(
    /// Get a magic-number sysvar
    SyscallGetMagicNumnberSysvar,
    fn rust(
        _invoke_context: &mut InvokeContext,
        _arg1: u64,
        _arg2: u64,
        _arg3: u64,
        _arg4: u64,
        _arg5: u64,
        _memory_mapping: &mut MemoryMapping,
    ) -> Result<u64, Error> {
        Ok(2002)
    }
);
```
[here](https://github.com/4rjunc/agave/blob/526a10c73f1de9ef36bae5e277fc054d3aee85fe/programs/bpf_loader/src/syscalls/mod.rs#L1934C1-L1949C3)

 
3. I came back to `solana-sdk` repo. because `sol_remaining_compute_units` can be imported to program from `use solana_program::compute_units::sol_remaining_compute_units`
i have to find where the code makes it importable. like before i searched for `sol_remaining_compute_units` keyword which led me to `solana-sdk/program` (the solana_program crate). 

 - added in `definitions.rs`

   ```rust 
        pub use solana_define_syscall::definitions::{
    sol_alt_bn128_compression, sol_alt_bn128_group_op, sol_big_mod_exp, sol_blake3,
    sol_curve_group_op, sol_curve_multiscalar_mul, sol_curve_pairing_map, sol_curve_validate_point,
    sol_get_clock_sysvar, sol_get_epoch_rewards_sysvar, sol_get_epoch_schedule_sysvar,
    sol_get_epoch_stake, sol_get_fees_sysvar, sol_get_last_restart_slot, **sol_get_magic_number**, <-- HERE!!!
    sol_get_rent_sysvar, sol_get_sysvar, sol_keccak256, sol_remaining_compute_units,
};
   ```

   [here](https://github.com/4rjunc/solana-sdk/blob/62c9fc28cd556a2d5c4695b5b550c6967e2e6962/program/src/syscalls/definitions.rs#L13C74-L13C94)


 -  created a new file named `magic_number.rs` in `solana-sdk/program/src`

    ```rust
    /// Return the magic number
#[inline]
pub fn sol_get_magic_number() -> u64 {
    #[cfg(target_os = "solana")]
    unsafe {
        crate::syscalls::sol_get_magic_number()
    }

    #[cfg(not(target_os = "solana"))]
    {
        crate::program_stubs::sol_get_magic_number()
    }
}
``` 

 [here](https://github.com/4rjunc/solana-sdk/blob/custom-syscall/program/src/magic_number.rs)
 
 - added in `solana-sdk/program/lib.rs`
 ```rust
    pub mod magic_number;
  ```

  [here](https://github.com/4rjunc/solana-sdk/blob/62c9fc28cd556a2d5c4695b5b550c6967e2e6962/program/src/lib.rs#L485k)

 - next in `solana-sdk/sysvar/program_stubs.rs` 

    ```rust
    fn sol_get_magic_number(&self) -> u64 {
        sol_log("MAGIC NUMBER DEFAULT TO 0");
        0
    }
    ```

   [here](https://github.com/4rjunc/solana-sdk/blob/62c9fc28cd556a2d5c4695b5b550c6967e2e6962/sysvar/src/program_stubs.rs#L37)

   ```rust
    pub fn sol_get_magic_number() -> u64 {
        SYSCALL_STUBS.read().unwrap().sol_get_magic_number()
    }
   ```

   [here](https://github.com/4rjunc/solana-sdk/blob/62c9fc28cd556a2d5c4695b5b550c6967e2e6962/sysvar/src/program_stubs.rs#L140)


all code changes are done, now adding these dependencies to the sample solana-program. i used the program in repo to call this syscall. here you have the `sol_remaining_compute_units` and custom one `sol_get_magic_number`
```rust
use solana_program::{
    account_info::AccountInfo, compute_units::sol_remaining_compute_units, declare_id, entrypoint,
    entrypoint::ProgramResult, magic_number::sol_get_magic_number, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

#[cfg(test)]
mod tests;

declare_id!("r8p3kwsDxPyTu1KyacFxJcP5b98GRn9wocBUsTToWTd");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    if program_id.ne(&crate::ID) {
        return Err(ProgramError::IncorrectProgramId);
    }
    msg!(
        "sol_remaining_compute_units {:?}",
        sol_remaining_compute_units()
    );
    msg!("sol_get_magic_number {:?}", sol_get_magic_number()); // CUSTOM ONE
    Ok(())
}
```

`Cargo.toml` looks like this. Initially i tried to add dependencies from my forked github repo. but i got me more errors. so i added the dependencies by local path like this.

```
[package]
name = "custom-syscall"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0.98"
solana-program = {path="/Users/arjunc/Documents/solana/svm/solana-sdk/program", version="2.3.0"}
solana-sysvar-id = {path="/Users/arjunc/Documents/solana/svm/solana-sdk/sysvar-id", version="2.2.1"}
tokio = "1.46.1"

[dev-dependencies]
mollusk-svm = "0.4.1"
solana-sdk = "2.3.1"
solana-client = "2.3.5"

[lib]
crate-type = ["cdylib", "lib"]
```

But cargo build-sbf was giving out versionig issues like this 
```
error[E0277]: the trait bound `StakeHistory: SysvarId` is not satisfied
   --> src/stake_history.rs:61:17
    |
61  | impl Sysvar for StakeHistory {
    |                 ^^^^^^^^^^^^ the trait `SysvarId` is not implemented for `StakeHistory`
    |
```

Which got fixed by changing `stake` and `system` dependencies in `Cargo.toml` of local `solana-sdk`

```toml
solana-stake-interface = { path="/Users/arjunc/Documents/solana/svm/stake/interface", version = "1.2.0" }
solana-system-interface = { path="/Users/arjunc/Documents/solana/svm/system/interface", version="1.0"}
```
[here](https://github.com/4rjunc/solana-sdk/blob/62c9fc28cd556a2d5c4695b5b550c6967e2e6962/Cargo.toml#L298C1-L299C103)

also changed one line in `stake/inteface/Cargo.toml` to 
```toml
solana-sysvar-id = { path="/Users/arjunc/Documents/solana/svm/solana-sdk/sysvar-id" , version="2.2.1"}
```
i haven't pushed it because of single line change. 

last toml changes are to `agave`

```rust 
solana-define-syscall = { path = "/Users/arjunc/Documents/solana/svm/solana-sdk/define-syscall"}
```

[here](https://github.com/4rjunc/agave/blob/e1c80c5b4848b181eac42ed6d113aa2994e57896/Cargo.toml#L418)

these are code changes now lets run the local validator, build, deploy and a send transaction to the program. 

to build local validator run this from `agave` dir. mine is macbook air m1, 8gb 2020 model. in some laptop this wont work
```zsh 
 cargo run -p agave-validator --bin solana-test-validator
```

build the program, in program's dir run
```zsh
cargo build-sbf

```

now deploy 
    
    -   ```zsh
        solana config set -ul // set to localhost 
        ```

    - run this deploy command from agave repo
        ```rust 
            cargo run -p solana-cli -- program deploy ~/Documents/solana/svm/custom-syscall/target/deploy/custom_syscall.so --program-id ~/Documents/solana/svm/custom-syscall/target/deploy/custom_syscall-keypair.json
        ```

        if you run the `solana deploy program` from your program's repo it wont work because the installed cli dont know about newly added syscall. it will return this error 

        ```zsh 
        Error: ELF error: ELF error: Unresolved symbol (sol_get_magic_number) at instruction #187 (ELF file offset 0x5d8)
        ```

now run the test code to send transaction to program from program's repo
```
$ cargo run test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.04s
     Running unittests src/lib.rs (target/debug/deps/custom_syscall-6e5586ec56b9774c)

running 2 tests
test test_id ... ok
local validator: r8p3kwsDxPyTu1KyacFxJcP5b98GRn9wocBUsTToWTd
Transaction signature: 2ArWbUJAGE9xU4ED3S2m367cfyCTsSVcshTc9ENV2GDWdS85C1PX7Njobdc5puT53ggYduusiCuigCZFYHTx59co
test tests::custom_syscall_localvalidator ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.94s

   Doc-tests custom_syscall

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

to see the logged message add the local rpc url to https://explorer.solana.com/ 's custom url. i tried it brave it did't worked, for me it worked in chrome. search the transaction hash. then the magic was displayed in the logs like this 

!(logged)[./logged.png]



