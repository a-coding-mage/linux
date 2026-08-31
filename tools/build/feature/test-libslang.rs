// SPDX-License-Identifier: GPL-2.0
// C dependency: <slang.h>

unsafe extern "C" {
    fn SLsmg_init_smg() -> i32;
}

fn main() -> i32 {
    unsafe { SLsmg_init_smg() }
}
