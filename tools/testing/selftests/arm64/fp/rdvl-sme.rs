// SPDX-License-Identifier: GPL-2.0-only

// C dependency intent: #include <stdio.h>
// C dependency intent: #include "rdvl.h"

extern "C" {
    fn rdvl_sme() -> ::std::os::raw::c_int;
}

fn main() {
    let vl: ::std::os::raw::c_int = unsafe { rdvl_sme() };

    println!("{}", vl);

    std::process::exit(0);
}
