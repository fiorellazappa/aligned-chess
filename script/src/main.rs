extern crate chess_engine;
use chess_engine::*;
use std::{
    convert::TryFrom,
    io::{stdin, stdout, Write},
};

mod gameparse;
use gameparse::parse_chess_input;

use aligned_sdk::core::types::{
    AlignedVerificationData, Network, PriceEstimate, ProvingSystemId, VerificationData,
};
use aligned_sdk::sdk::{deposit_to_aligned, estimate_fee};
use aligned_sdk::sdk::{get_next_nonce, submit_and_wait_verification};
use clap::Parser;
use dialoguer::Confirm;
use ethers::abi::{encode, Token};
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, Bytes, H160, U256};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sp1_sdk::{ProverClient, SP1Stdin};

const LOOKAHEAD: i32 = 5;
const CHESS_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

abigen!(VerifierContract, "VerifierContract.json",);

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    mode: String,

    #[clap(long)]
    board: String,

    #[clap(short, long, default_value = "[]")]
    moves: String,

    #[arg(short, long)]
    keystore_path: String,
    #[arg(
        short,
        long,
        default_value = "https://ethereum-holesky-rpc.publicnode.com"
    )]
    rpc_url: String,
    #[arg(short, long, default_value = "wss://batcher.alignedlayer.com")]
    batcher_url: String,
    #[arg(short, long, default_value = "holesky")]
    network: Network,
    #[arg(
        short,
        long,
        default_value = "0xaA7FB0a4B3A1A3623B7C10B5e4a7b41e78f70643"
    )]
    verifier_contract_address: H160,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    // Parse the command line arguments.
    let args = Args::parse();

    let moves: Vec<String> =
        serde_json::from_str(&args.moves).map_err(|e| format!("Failed to parse moves: {}", e))?;
    println!("{:?}", &args.board);

    //let board = serde_json::from_str(&args.board);

    let board_data: Value =
        serde_json::from_str(&args.board).map_err(|e| format!("Failed to parse board: {}", e))?;

    // Parse the moves JSON string into a Value
    let moves: Value =
        serde_json::from_str(&args.moves).map_err(|e| format!("Failed to parse moves: {}", e))?;

    let json_input = json!({
        "w": board_data["w"],
        "b": board_data["b"],
        "moves": moves
    })
    .to_string();

    let (board, moves) = parse_chess_input(&json_input)?;
    let mut b = board;

    let cpu = Color::Black;

    println!("{}", b);
    let mut history = vec![];

    let mut i = 0;
    let mut output_moves: Vec<String> = vec![];
    loop {
        let mut s = String::new();
        let mut h_move = String::new();

        if b.get_turn_color() != cpu {
            if args.mode == "interactive" {
                s = input(">>> ");
                s = s.trim().to_string();
                if s.is_empty() {
                    println!("You must provide a move.");
                    continue;
                }
                h_move = s.clone();
            } else {
                if (i >= moves.len()) {
                    panic!("No move provided");
                }
                s = moves[i].clone();
                s = s.trim().to_string();
                i += 1;
                if s.is_empty() {
                    panic!("No move provided");
                }
            }
        }

        let m = if s.is_empty() {
            println!("Waiting for CPU to choose best move...");
            get_cpu_move(&b)
        } else {
            let s_clone = s.clone(); // Clone s before moving it
            match Move::try_from(s_clone) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            }
        };

        match b.play_move(m) {
            GameResult::Continuing(next_board) => {
                b = next_board;
                println!("{}", b);
                history.push(b);
                if (args.mode == "interactive" && b.get_turn_color() != cpu) {
                    output_moves.push(h_move.clone());
                }
            }

            GameResult::Victory(winner) => {
                println!("{}", b);
                println!("{} loses. {} is victorious.", !winner, winner);
                if args.mode == "interactive" {
                    output_moves.push(h_move.clone());
                    println!("Moves: {:?}", output_moves);
                }
                break;
            }

            GameResult::IllegalMove(x) => {
                eprintln!("{} is an illegal move.", x);
            }

            GameResult::Stalemate => {
                println!("Drawn game.");
                break;
            }
        }
    }

    for m in history {
        println!("{}", m);
    }

    // PROOVE!

    let rpc_url = args.rpc_url.clone();

    let keystore_password = rpassword::prompt_password("Enter keystore password: ")
        .expect("Failed to read keystore password");

    let provider =
        Provider::<Http>::try_from(rpc_url.as_str()).expect("Failed to connect to provider");

    let chain_id = provider
        .get_chainid()
        .await
        .expect("Failed to get chain_id");

    let wallet = LocalWallet::decrypt_keystore(args.keystore_path, &keystore_password)
        .expect("Failed to decrypt keystore")
        .with_chain_id(chain_id.as_u64());

    let signer = SignerMiddleware::new(provider.clone(), wallet.clone());

    if Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Do you want to deposit 0.004eth in Aligned ?\nIf you already deposited Ethereum to Aligned before, this is not needed")
        .interact()
        .expect("Failed to read user input") {   

        deposit_to_aligned(U256::from(4000000000000000u128), signer.clone(), args.network).await
        .expect("Failed to pay for proof submission");
    }

    let moves_to_use = if args.mode == "interactive" {
        output_moves
        //serde_json::to_value(output_moves).map_err(|e| format!("Failed to convert output moves: {}", e))?
    } else {
        moves
    };

    // Crear el JSON combinado
    let combined = json!({
        "w": board_data["w"],
        "b": board_data["b"],
        "moves": moves_to_use
    });

    let json_input = combined.to_string();

    println!("{:?}", json_input);
    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    // moves is the serde output
    stdin.write(&json_input);
    // Setup the prover client.
    let client = ProverClient::new();
    // Setup the program for proving.
    let (pk, vk) = client.setup(CHESS_ELF);

    let proof = client
        .prove(&pk, stdin)
        .run()
        .expect("failed to generate proof");

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");

    println!("Generating Proof ");

    // Serialize proof into bincode (format used by sp1)
    let proof = bincode::serialize(&proof).expect("Failed to serialize proof");


    let verification_data = VerificationData {
        proving_system: ProvingSystemId::SP1,
        proof,
        proof_generator_addr: wallet.address(),
        vm_program_code: Some(CHESS_ELF.to_vec()),
        verification_key: None,
        pub_input: None,
    };

    let max_fee = estimate_fee(&rpc_url, PriceEstimate::Default)
        .await
        .expect("failed to fetch gas price from the blockchain");

    let max_fee_string = ethers::utils::format_units(max_fee, 18).unwrap();

    if !Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt(format!("Aligned will use at most {max_fee_string} eth to verify your proof. Do you want to continue?"))
        .interact()
        .expect("Failed to read user input")
    {       return Ok(());
    }

    let nonce = get_next_nonce(&rpc_url, wallet.address(), args.network)
        .await
        .expect("Failed to get next nonce");

    println!("Submitting your proof...");

    let aligned_verification_data = submit_and_wait_verification(
        &args.batcher_url,
        &rpc_url,
        args.network,
        &verification_data,
        max_fee,
        wallet.clone(),
        nonce,
    )
    .await
    .unwrap();

   
    println!(
        "Proof submitted and verified successfully on batch {}",
        hex::encode(aligned_verification_data.batch_merkle_root)
    );
    println!("Claiming NFT prize...");

    claim_nft_with_verified_proof(
        &aligned_verification_data,
        signer,
        &args.verifier_contract_address,
    )
    .await
    .expect("Claiming of NFT failed ...");


    Ok(())
}

fn input(prompt: impl std::fmt::Display) -> String {
    let mut s = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    s
}

fn get_cpu_move(b: &Board) -> Move {
    let (m, count, _) = b.get_best_next_move(LOOKAHEAD);

    print!("CPU evaluated {} moves before choosing to ", count);
    match m {
        Move::Piece(from, to) | Move::Promotion(from, to, _) => {
            match (b.get_piece(from), b.get_piece(to)) {
                (Some(piece), Some(takes)) => println!(
                    "take {}({}) with {}({})",
                    takes.get_name(),
                    to,
                    piece.get_name(),
                    from
                ),
                (Some(piece), None) => {
                    println!("move {}({}) to {}", piece.get_name(), from, to)
                }
                _ => println!("move {} to {}", from, to),
            }
        }
        Move::KingSideCastle => {
            println!("castle kingside")
        }
        Move::QueenSideCastle => {
            println!("castle queenside")
        }
        Move::Resign => println!("resign"),
    }
    m
}


async fn claim_nft_with_verified_proof(
    aligned_verification_data: &AlignedVerificationData,
    signer: SignerMiddleware<Provider<Http>, LocalWallet>,
    verifier_contract_addr: &Address,
) -> anyhow::Result<()> {
    let verifier_contract = VerifierContract::new(*verifier_contract_addr, signer.into());

    let index_in_batch = U256::from(aligned_verification_data.index_in_batch);
    let merkle_path = Bytes::from(
        aligned_verification_data
        .batch_inclusion_proof
        .merkle_path
        .as_slice()
        .into_iter() 
        .flatten() 
        .copied() 
        .collect::<Vec<u8>>(),
    );

    let receipt = verifier_contract
        .verify_batch_inclusion(
            aligned_verification_data
                .verification_data_commitment
                .proof_commitment,
            aligned_verification_data
                .verification_data_commitment
                .pub_input_commitment,
            aligned_verification_data
                .verification_data_commitment
                .proving_system_aux_data_commitment,
            aligned_verification_data
                .verification_data_commitment
                .proof_generator_addr,
            aligned_verification_data.batch_merkle_root,
            merkle_path,
            index_in_batch,
        )
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to send tx {}", e))?
        .await
        .map_err(|e| anyhow::anyhow!("Failed to submit tx {}", e))?;

    match receipt {
        Some(receipt) => {
            println!(
                "Prize claimed successfully. Transaction hash: {:x}",
                receipt.transaction_hash
            );
            Ok(())
        }
        None => {
            anyhow::bail!("Failed to claim prize: no receipt");
        }
    }
}