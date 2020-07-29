use std::env;
use std::process::Command;
use rand::{thread_rng, Rng};

const DISPLAY_AMOUNT: usize = 5;

type BlockData<'a> = (&'a String, &'a String);

fn main() {

    let mut block_list: Vec<BlockData> = vec![];
    let args: Vec<String> = env::args().collect();
    let mut rng = thread_rng();

    // Block list init
    for i in 0..(args.len() - 1) / 2 {
        let data: BlockData = ( &args[i * 2 + 1], &args[i * 2 + 2] );
        block_list.push( data )
    }
    let weight_total = {
        let mut total: usize = 0;
        for i in 0..block_list.len() {
            total += block_list[i].1.parse::<usize>().unwrap();
        }
        total
    };

    // Main loop
    loop {
        println!(" ");
        for _i in 0..DISPLAY_AMOUNT {
            let roll = rng.gen_range(0, weight_total);
            // println!("{:?}", roll);
            let mut running: usize = 0;
            let mut block = &String::from("");
            for i in 0..block_list.len() {
                let weight =  block_list[i].1.parse::<usize>().unwrap();
                if roll < weight + running {
                    block = block_list[i].0;
                    break;
                } else {
                    running += weight;
                }
            }
            println!("{}", block );
        }
        println!(" ");
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
    }
}