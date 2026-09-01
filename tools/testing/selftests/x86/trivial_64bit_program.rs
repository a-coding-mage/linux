// SPDX-License-Identifier: GPL-2.0-only
/*
 * Trivial program to check that we have a valid 64-bit build environment.
 * Copyright (c) 2015 Andy Lutomirski
 */

#[cfg(not(target_arch = "x86_64"))]
compile_error!("wrong architecture");

fn main() {
    println!();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
