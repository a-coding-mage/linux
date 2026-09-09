/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <asm/io.h>

extern "C" {
    pub static mut uml_physmem: usize;
}

// C macro: #define MAX_DMA_ADDRESS (uml_physmem)
#[macro_export]
macro_rules! MAX_DMA_ADDRESS {
    () => {
        unsafe { $crate::uml_physmem }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
