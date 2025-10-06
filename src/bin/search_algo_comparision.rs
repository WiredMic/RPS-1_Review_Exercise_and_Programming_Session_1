#![allow(unused_imports)]
#![allow(dead_code)]
use clap::Parser;
use rand::Rng;
use sorting_algorithims::search::{binary_search, linear_search, quaternary_search};
use std::time::SystemTime;

const GROUP_ALIAS: &'static str = "ES-25-ESD-3-120";

// Algotrithm names
const LINEAR_SEARCH: &'static str = "Linear Search";
const BINARY_SEARCH: &'static str = "Binary Search";
const QUATERNARY_SEARCH: &'static str = "Quaternaty Search";
const RUST_SEARCH: &'static str = "Inbuilt Rust Search";

const LENGTH: usize = 10_u64.pow(7) as usize;
const RUNS: usize = 100;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    #[arg(short = 't', long = "target-num", default_value_t = 105)]
    target_num: u8,
}

fn main() {
    let args = Cli::parse();

    println!(
        "Running {} iterations with array length {} with a target of {:?}\n",
        RUNS, LENGTH, args.target_num
    );

    let mut rng = rand::rng();
    let mut list: Vec<u8> = Vec::with_capacity(LENGTH);

    for _ in 0..LENGTH {
        list.push(rng.random_range(1..=100));
    }

    println!(
        "First three element in usort list {:?}",
        list.first_chunk::<3>().unwrap()
    );
    println!(
        "Last three element in usort list {:?}",
        list.last_chunk::<3>().unwrap()
    );
    list.sort();
    println!("List sorted");
    println!(
        "First three element in sort list {:?}",
        list.first_chunk::<3>().unwrap()
    );
    println!(
        "Last three element in sort list {:?}\n",
        list.last_chunk::<3>().unwrap()
    );

    // linear search
    let mut linear_res = std::result::Result::Err("Linear search was run 0 times");
    let mut elapsed = 0;
    for _index in 0..RUNS {
        let now = SystemTime::now();
        linear_res = linear_search(&list, &args.target_num);
        elapsed += now.elapsed().unwrap().as_nanos();
    }
    let l_done = elapsed as usize / RUNS;
    match linear_res {
        Ok(res) => {
            println!(
                "Linear Search found target {} at index {}",
                args.target_num, res
            );
        }
        Err(error_string) => {
            println!("Linear Search returned error: {}", error_string);
        }
    }
    println!(
        "Linear search ran {} times with length of {}. avg is {} ns\n",
        RUNS, LENGTH, l_done
    );

    // binary search
    let mut binary_res = std::result::Result::Err("Binary search was run 0 times");
    let mut elapsed = 0;
    for _index in 0..RUNS {
        let now = SystemTime::now();
        binary_res = binary_search(&list, &args.target_num);
        elapsed += now.elapsed().unwrap().as_nanos();
    }
    let b_done = elapsed as usize / RUNS;
    match binary_res {
        Ok(res) => {
            println!(
                "Binary Search found target {} at index {}",
                args.target_num, res
            );
        }
        Err(error_string) => {
            println!("Binary Search returned error: {}", error_string);
        }
    }
    println!(
        "Binary search ran {} times with length of {}. avg is {} ns\n",
        RUNS, LENGTH, b_done
    );

    // quaternary search
    let mut quaternary_res = std::result::Result::Err("Quaternary search was run 0 times");
    let mut elapsed = 0;
    for _index in 0..RUNS {
        let now = SystemTime::now();
        quaternary_res = quaternary_search(&list, 0, list.len() - 1, &args.target_num);
        elapsed += now.elapsed().unwrap().as_nanos();
    }
    let q_done = elapsed as usize / RUNS;
    match quaternary_res {
        Ok(res) => {
            println!(
                "Quaternary Search found target {} at index {}",
                args.target_num, res
            );
        }
        Err(error_string) => {
            println!("Quaternary Search returned error: {}", error_string);
        }
    }

    println!(
        "Quaternary Search ran {} times with length of {}. avg is {} ns\n",
        RUNS, LENGTH, q_done
    );

    // inbuild rust search
    let mut elapsed = 0;
    for index in 0..RUNS {
        let now = SystemTime::now();
        let rust_res = list.binary_search(&args.target_num);
        elapsed += now.elapsed().unwrap().as_nanos();
        if index == RUNS - 1 {
            match rust_res {
                Ok(res) => {
                    println!("Rust B is {}", res);
                }
                Err(_err) => {
                    println!("Rust B finished with err",);
                }
            }
        }
    }
    let rust_done = elapsed as usize / RUNS;
    println!(
        "Inbuild Rust b ran {} times with length of {}. avg is {} ns",
        RUNS, LENGTH, rust_done
    );

    println!(
        "[{}] - {} - {:e} - {} ns.",
        GROUP_ALIAS, LINEAR_SEARCH, LENGTH, l_done
    );
    println!(
        "[{}] - {} - {:e} - {} ns.",
        GROUP_ALIAS, BINARY_SEARCH, LENGTH, b_done
    );
    println!(
        "[{}] - {} - {:e} - {} ns.",
        GROUP_ALIAS, QUATERNARY_SEARCH, LENGTH, q_done
    );
    println!(
        "[{}] - {} - {:e} - {} ns.",
        GROUP_ALIAS, RUST_SEARCH, LENGTH, rust_done
    );
}
