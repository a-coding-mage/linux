// SPDX-License-Identifier: GPL-2.0
// C dependency: <slang.h>

unsafe extern "C" {
    fn SLsmg_init_smg() -> i32;
}

fn main() -> i32 {
    unsafe { SLsmg_init_smg() }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
