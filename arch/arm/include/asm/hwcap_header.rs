/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of: #include <uapi/asm/hwcap.h>

// This yields a mask that user programs can use to figure out what
// instruction set this cpu supports.
//
// These declarations are present only when not compiling assembly in the
// original header. Rust source is not assembled, so the declarations are
// retained here unconditionally.
#[allow(improper_ctypes)]
extern "C" {
    pub static mut elf_hwcap: u32;
    pub static mut elf_hwcap2: u32;
}

// Equivalent to the C aliases #define ELF_HWCAP (elf_hwcap) and
// #define ELF_HWCAP2 (elf_hwcap2).
#[macro_export]
macro_rules! ELF_HWCAP {
    () => {
        $crate::elf_hwcap
    };
}

#[macro_export]
macro_rules! ELF_HWCAP2 {
    () => {
        $crate::elf_hwcap2
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
