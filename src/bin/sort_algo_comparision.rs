#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use clap::Parser;
use rand::Rng;
use sorting_algorithims::sort::{merge_sort, q_merge_sort};
use std::time::SystemTime;

const LENGTH: usize = 10_u64.pow(7) as usize;
const RUNS: usize = 100;

fn main() {
    println!(
        "Running {} iterations with array length {:e}\n",
        RUNS, LENGTH
    );

    let mut rng = rand::rng();
    let mut list: Vec<u8> = Vec::with_capacity(LENGTH);

    for _ in 0..LENGTH {
        list.push(rng.random_range(1..=100));
    }

    println!(
        "First and last three element in unsort list {:?}, {:?}\n",
        list.first_chunk::<3>().unwrap(),
        list.last_chunk::<3>().unwrap()
    );

    let mut unsort_1 = list.clone();
    let mut unsort_2 = list.clone();
    let mut unsort_3 = list.clone();

    // merge sort
    println!("Sorting with merge sort function");

    let length = unsort_2.len();
    let now = SystemTime::now();
    merge_sort(&mut unsort_1, 0, length - 1);
    let done = now.elapsed().unwrap().as_millis();

    println!("List sorted in {} ms", done);
    println!(
        "First and last three element in sort list {:?}, {:?}\n",
        unsort_1.first_chunk::<3>().unwrap(),
        unsort_1.last_chunk::<3>().unwrap()
    );

    // quaternary  merge sort
    println!("Sorting with quaternary merge sort function");

    let length = unsort_2.len();
    let now = SystemTime::now();
    q_merge_sort(&mut unsort_2, 0, length - 1);
    let done = now.elapsed().unwrap().as_millis();

    println!("List sorted in {} ms", done);
    println!(
        "First and last three element in sort list {:?}, {:?}\n",
        unsort_2.first_chunk::<3>().unwrap(),
        unsort_2.last_chunk::<3>().unwrap()
    );

    // Rust inbuild sort
    println!("Sorting with inbuild Rust function");

    let now = SystemTime::now();
    unsort_3.sort();
    let done = now.elapsed().unwrap().as_millis();

    println!("List sorted in {} ms", done);
    println!(
        "First and last three element in sort list {:?}, {:?}",
        unsort_3.first_chunk::<3>().unwrap(),
        unsort_3.last_chunk::<3>().unwrap()
    );
}
