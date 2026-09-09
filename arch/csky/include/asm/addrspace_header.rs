/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_CSKY_ADDRSPACE_H

pub const KSEG0: usize = 0x8000_0000usize;

macro_rules! KSEG0ADDR {
    ($a:expr) => {
        ((($a as usize) & 0x1fff_ffffusize) | KSEG0)
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
