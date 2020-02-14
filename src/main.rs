fn main() {
    let digits: u32 = 5;

    let init_n = 10u32.pow(digits - 1) + 1;
    let mut n = init_n;


    //THE NUM OF PLAINDROMS WITH x DIGITS IS (  9 * 10^(floor of(x-1)/2) )
 
         gen(7);
 }

// fn read_from_file() -> u32 {
//     let mut file = File::open("output.txt").expect("Unable to open file");

//     let buffer = BufReader::new(file);

//     let mut res: u32 = 0;

//     for line in buffer.lines() {
//         let num: u32 = line.unwrap().trim().parse().unwrap();
//         if num % 5 == 3 {
//             res += 1;
//             println!("num: {}", num);
//         }
//     }
//     res
// }

fn gen_rec(n: u32, lvl: u32, nr_of_digits: u32, buffer: &mut BufWriter<std::fs::File>) {
    let mut n = n;
    let mut lvl = lvl;

    let is_odd = nr_of_digits % 2 == 1;

    if lvl == 0 {
        // most inner level
        let temp = nr_of_digits / 2;

        let add = if is_odd {
            10u32.pow(temp)
        } else {
            11 * 10u32.pow(temp - 1)
        };

        // special case when the num_of_digits is equals to 2
        let i = if nr_of_digits == 2 { 1 } else { 0 };
        for _ in i..=9 {
            writeln!(buffer, "{}", n).expect("Unable to writeln!");
            // println!("{}", n);
            n += add;
        }
    } else if lvl == (nr_of_digits - 1) / 2 {
        //most outter level
        let add = 10u32.pow(nr_of_digits - 1) + 1;

        for _ in 1..=9 {
            gen_rec(n, lvl - 1, nr_of_digits, buffer);
            n += add;
        }
    } else {
        // other inner levels
        let tmp = nr_of_digits / 2;
        let add = if is_odd {
            10u32.pow(tmp + lvl) + 10u32.pow(tmp - lvl)
        } else {
            10u32.pow(tmp + lvl) + 10u32.pow(tmp - lvl - 1)
        };
        println!("***********{}", add);
        for _ in 0..=9 {
            gen_rec(n, lvl - 1, nr_of_digits, buffer);
            n += add;
        }
    }
}

use std::fs::File;

use std::io::prelude::*;
use std::io::{BufRead, BufReader, BufWriter};

fn gen(nr_of_digits: u32) {
    let outer_lvl = (nr_of_digits - 1) / 2;

    let init = if nr_of_digits == 1 {
        0
    } else {
        10u32.pow(nr_of_digits - 1) + 1
    };

    let mut file = File::create("output.txt").expect("Unable to create file");
    let mut buffer = BufWriter::new(file);

    gen_rec(init, outer_lvl, nr_of_digits, &mut buffer);
}

// fn gen_rec_odd(n: u32, lvl: u32, nr_of_digits: u32, buffer: &mut BufWriter<std::fs::File>) {
//     let mut n = n;
//     let mut lvl = lvl;

//     if lvl == 0 {
//         // most inner level
//         // match nr_of_digits{
//         //     1=> { add = 1},
//         //     3=> { add = 010},
//         //     5=> { add = 00100},
//         //     ...
//         // }

//         let add = 10u32.pow(nr_of_digits / 2);
//         for _ in 0..=9 {
//             writeln!(buffer, "{}", n).expect("Unable to writeln!");

//             println!("{}", n);
//             n += add;
//         }
//     } else if lvl == (nr_of_digits - 1) / 2 {
//         //most outter level
//         // match nr_of_digits{
//         //     3=> { add = 101},
//         //     5=> { add = 10001},
//         //     7=> { add = 1000001},
//         //     ...
//         // }
//         let add = 10u32.pow(nr_of_digits - 1) + 1;

//         for _ in 1..=9 {
//             gen_rec_odd(n, lvl - 1, nr_of_digits, buffer);
//             n += add;
//         }
//     } else {
//         // other inner levels

//         // match nr_of_digits{
//         //     5=> { add = 1010 },
//         //     7=> { add = 10100  or 100010 },
//         //     9=> { add = 101000 or 1000100 or  10000010 },
//         //     ...
//         // }
//         let tmp = nr_of_digits / 2;
//         let add = 10u32.pow(tmp + lvl) + 10u32.pow(tmp - lvl);
//         println!("***********{}", add);

//         for _ in 0..=9 {
//             gen_rec_odd(n, lvl - 1, nr_of_digits, buffer);
//             n += add;
//         }
//     }
// }
// fn gen_rec_even(n: u32, lvl: u32, nr_of_digits: u32, buffer: &mut BufWriter<std::fs::File>) {
//     let mut n = n;
//     let mut lvl = lvl;

//     if lvl == 0 {
//         // most inner level
//         // match nr_of_digits{
//         //     2=> { add = 11},
//         //     4=> { add = 110},
//         //     6=> { add = 1100},
//         //     ...
//         // }

//         let temp = (nr_of_digits - 1) / 2;
//         let add = 11 * 10u32.pow(temp);
//         for _ in 0..=9 {
//             writeln!(buffer, "{}", n).expect("Unable to writeln!");

//             println!("{}", n);
//             n += add;
//         }
//     } else if lvl == (nr_of_digits - 1) / 2 {
//         //most outter level
//         // match nr_of_digits{
//         //     4=> { add = 1001},
//         //     6=> { add = 100001},
//         //     8=> { add = 10000001},
//         //     ...
//         // }
//         let add = 10u32.pow(nr_of_digits - 1) + 1;

//         for _ in 1..=9 {
//             gen_rec_even(n, lvl - 1, nr_of_digits, buffer);
//             n += add;
//         }
//     } else {
//         // other inner levels

//         // match nr_of_digits{
//         //     6=> { add = 10010},
//         //     8=> { add = 100100  or 1000010},
//         //    10=> { add = 1001000 or 10000100 or  100000010 },
//         //     ...
//         // }
//         let tmp = nr_of_digits / 2;
//         let add = 10u32.pow(tmp + lvl) + 10u32.pow(tmp - lvl - 1);
//         println!("***********{}", add);

//         for _ in 0..=9 {
//             gen_rec_even(n, lvl - 1, nr_of_digits, buffer);
//             n += add;
//         }
//     }
// }
