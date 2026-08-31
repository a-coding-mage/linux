// SPDX-License-Identifier: GPL-2.0
/*
 * Test interpreter for the bound-interpreter case of the binfmt_misc_bpf
 * selftest. Two copies are installed at different paths and bound to one
 * entry under different names; printing argv[0] - the path the kernel ran
 * this copy under - tells the harness which of them the load program picked.
 */

use std::env;

fn main() {
    let argv0 = env::args().next().unwrap_or_else(|| "".to_string());

    println!("BIND_RAN {}", argv0);
}
