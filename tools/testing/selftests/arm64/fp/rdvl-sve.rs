// SPDX-License-Identifier: GPL-2.0-only

// Depends on declarations from "rdvl.h".
extern "C" {
    fn rdvl_sve() -> ::std::os::raw::c_int;
}

fn main() {
    let vl: ::std::os::raw::c_int = unsafe { rdvl_sve() };

    println!("{}", vl);
}
