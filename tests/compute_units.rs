use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    signer::keypair::Keypair,
    transaction::Transaction,
};
use std::str::FromStr;

/// Test program ID (valid base58 pubkey)
const PROGRAM_ID: &str = "11111111111111111111111111111112";

#[tokio::test]
async fn test_compute_units_standard_vs_assembly() {
    // Start the test environment
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let mut program_test = ProgramTest::new(
        "solana_pubkey_compare", // matches the crate name
        program_id,
        processor!(solana_pubkey_compare::process_instruction),
    );

    // Create test accounts
    let account1_keypair = Keypair::new();
    let account2_keypair = Keypair::new();
    let account3_keypair = Keypair::new(); // Different key for mismatch test

    // Add accounts to the test environment
    program_test.add_account(
        account1_keypair.pubkey(),
        Account {
            lamports: 1_000_000,
            data: vec![],
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    );
    
    program_test.add_account(
        account2_keypair.pubkey(),
        Account {
            lamports: 1_000_000,
            data: vec![],
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    );

    program_test.add_account(
        account3_keypair.pubkey(),
        Account {
            lamports: 1_000_000,
            data: vec![],
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    );

    let (banks_client, payer, recent_blockhash) = program_test.start().await;

    println!("🚀 Testing compute units for pubkey comparisons\n");

    // Test 1: Standard comparison with matching keys
    let instruction_standard_match = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(account1_keypair.pubkey(), false),
            AccountMeta::new_readonly(account1_keypair.pubkey(), false), // Same key
        ],
        data: vec![0], // 0 = standard comparison
    };

    let transaction_standard_match = Transaction::new_signed_with_payer(
        &[instruction_standard_match],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let result = banks_client.simulate_transaction(transaction_standard_match).await.unwrap();
    let standard_match_units = result.simulation_details.unwrap().units_consumed;
    println!("📊 Standard comparison (match): {} compute units", standard_match_units);

    // Test 2: Assembly comparison with matching keys
    let instruction_assembly_match = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(account1_keypair.pubkey(), false),
            AccountMeta::new_readonly(account1_keypair.pubkey(), false), // Same key
        ],
        data: vec![1], // 1 = assembly comparison
    };

    let transaction_assembly_match = Transaction::new_signed_with_payer(
        &[instruction_assembly_match],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let result = banks_client.simulate_transaction(transaction_assembly_match).await.unwrap();
    let assembly_match_units = result.simulation_details.unwrap().units_consumed;
    println!("⚡ Assembly comparison (match): {} compute units", assembly_match_units);

    // Test 3: Standard comparison with non-matching keys
    let instruction_standard_mismatch = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(account1_keypair.pubkey(), false),
            AccountMeta::new_readonly(account3_keypair.pubkey(), false), // Different key
        ],
        data: vec![0], // 0 = standard comparison
    };

    let transaction_standard_mismatch = Transaction::new_signed_with_payer(
        &[instruction_standard_mismatch],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let result = banks_client.simulate_transaction(transaction_standard_mismatch).await.unwrap();
    let standard_mismatch_units = result.simulation_details.unwrap().units_consumed;
    println!("📊 Standard comparison (mismatch): {} compute units", standard_mismatch_units);

    // Test 4: Assembly comparison with non-matching keys (should panic)
    let instruction_assembly_mismatch = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(account1_keypair.pubkey(), false),
            AccountMeta::new_readonly(account3_keypair.pubkey(), false), // Different key
        ],
        data: vec![1], // 1 = assembly comparison
    };

    let transaction_assembly_mismatch = Transaction::new_signed_with_payer(
        &[instruction_assembly_mismatch],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let result = banks_client.simulate_transaction(transaction_assembly_mismatch).await.unwrap();
    let assembly_mismatch_units = if result.result.as_ref().map_or(false, |r| r.is_err()) {
        result.simulation_details.unwrap().units_consumed
    } else {
        0 // Shouldn't happen, but just in case
    };
    println!("⚡ Assembly comparison (mismatch/panic): {} compute units", assembly_mismatch_units);

    // Analysis
    println!("\n📈 PERFORMANCE ANALYSIS:");
    println!("========================");
    
    if assembly_match_units < standard_match_units {
        let savings = standard_match_units - assembly_match_units;
        let percentage = (savings as f64 / standard_match_units as f64) * 100.0;
        println!("✅ Assembly comparison is MORE efficient:");
        println!("   Savings: {} compute units ({:.1}% faster)", savings, percentage);
    } else if assembly_match_units > standard_match_units {
        let overhead = assembly_match_units - standard_match_units;
        let percentage = (overhead as f64 / standard_match_units as f64) * 100.0;
        println!("❌ Assembly comparison has MORE overhead:");
        println!("   Extra cost: {} compute units ({:.1}% slower)", overhead, percentage);
    } else {
        println!("⚖️  Both methods use the same compute units");
    }

    println!("\n🔍 DETAILED BREAKDOWN:");
    println!("Standard (match):    {} CUs", standard_match_units);
    println!("Assembly (match):    {} CUs", assembly_match_units);
    println!("Standard (mismatch): {} CUs", standard_mismatch_units);
    println!("Assembly (mismatch): {} CUs", assembly_mismatch_units);

    // Multiple iterations test for better accuracy
    println!("\n🔄 RUNNING MULTIPLE ITERATIONS FOR ACCURACY...");
    let mut standard_total = 0u64;
    let mut assembly_total = 0u64;
    let iterations = 10;

    for _i in 0..iterations {
        // Standard comparison
        let tx_standard = Transaction::new_signed_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![
                    AccountMeta::new_readonly(account1_keypair.pubkey(), false),
                    AccountMeta::new_readonly(account1_keypair.pubkey(), false),
                ],
                data: vec![0],
            }],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        let result = banks_client.simulate_transaction(tx_standard).await.unwrap();
        standard_total += result.simulation_details.unwrap().units_consumed;

        // Assembly comparison  
        let tx_assembly = Transaction::new_signed_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![
                    AccountMeta::new_readonly(account1_keypair.pubkey(), false),
                    AccountMeta::new_readonly(account1_keypair.pubkey(), false),
                ],
                data: vec![1],
            }],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        let result = banks_client.simulate_transaction(tx_assembly).await.unwrap();
        assembly_total += result.simulation_details.unwrap().units_consumed;
    }

    let standard_avg = standard_total / iterations;
    let assembly_avg = assembly_total / iterations;

    println!("Average over {} iterations:", iterations);
    println!("Standard: {} CUs", standard_avg);
    println!("Assembly:  {} CUs", assembly_avg);

    if assembly_avg < standard_avg {
        let savings = standard_avg - assembly_avg;
        let percentage = (savings as f64 / standard_avg as f64) * 100.0;
        println!("🎯 Final Result: Assembly is {:.1}% more efficient ({} CU savings)", percentage, savings);
    } else {
        let overhead = assembly_avg - standard_avg;
        let percentage = (overhead as f64 / standard_avg as f64) * 100.0;
        println!("🎯 Final Result: Assembly has {:.1}% overhead ({} CU extra)", percentage, overhead);
    }

    // Assertions for the test
    assert!(standard_match_units > 0, "Standard comparison should consume compute units");
    assert!(assembly_match_units > 0, "Assembly comparison should consume compute units");
}