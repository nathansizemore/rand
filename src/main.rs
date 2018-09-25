// Copyright 2018 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.


extern crate rand;


use std::env;

use rand::Rng;


fn main() {
    let args: Vec<String> = env::args().collect();
    let digits = args[1].parse().unwrap();
    let mut rng = rand::thread_rng();

    let mut output = String::with_capacity(digits);
    for _ in 0..digits {
        let digit = rng.gen_range(0, 10);
        output.push_str(&digit.to_string());
    }

    println!("{}", output);
}
