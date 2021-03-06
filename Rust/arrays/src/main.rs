use std::{thread, time::Duration};

fn main(){
    let mut target= 0;
    let mut input = String::new();
    println!("Enter the sum you wish to calculate");
    std::io::stdin().read_line(&mut input).expect("Not a valid string");
    target = input.trim().parse().expect("Not a valid number");

    let mut result: u32 = 0;
    let nums = [8, 7, 2, 5, 3, 1];
    let mut pair = false;
    for i in 0..nums.len() { 
        for j in 0..nums.len() { 
            //println!("{:?}{:?}",nums[i],nums[j]);
            result = nums[i] + nums[j];
            if result == target { 
                pair = true;
                println!("pair found ({}, {})", nums[i], nums[j]); 
            } 
            else { 
            result = 0;
            }
        }
    }
    if pair == false { println!("pair found not found");}
    thread::sleep(Duration::from_secs(5))
}