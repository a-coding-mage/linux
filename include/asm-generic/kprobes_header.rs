/* SPDX-License-Identifier: GPL-2.0 */

// These declarations correspond to the C header's kernel/non-assembler guard.
// The original CONFIG_KPROBES condition is preserved through Rust cfg intent.

#[cfg(all(feature = "kernel", not(feature = "assembler"), feature = "kprobes"))]
macro_rules! __NOKPROBE_SYMBOL {
    ($fname:ident) => {
        // C: a used unsigned-long symbol in the linker section
        // "_kprobe_blacklist", initialized with the address of $fname.
        #[used]
        #[link_section = "_kprobe_blacklist"]
        static _KBL_ADDR: usize = $fname as usize;
    };
}

#[cfg(all(feature = "kernel", not(feature = "assembler"), feature = "kprobes"))]
macro_rules! NOKPROBE_SYMBOL {
    ($fname:ident) => {
        __NOKPROBE_SYMBOL!($fname);
    };
}

#[cfg(all(feature = "kernel", not(feature = "assembler"), feature = "kprobes"))]
macro_rules! __kprobes {
    ($item:item) => {
        #[allow(non_snake_case)]
        #[link_section = ".kprobes.text"]
        $item
    };
}

#[cfg(all(feature = "kernel", not(feature = "assembler"), feature = "kprobes"))]
macro_rules! nokprobe_inline {
    ($item:item) => {
        #[inline(always)]
        $item
    };
}

#[cfg(all(feature = "kernel", not(feature = "assembler"), not(feature = "kprobes")))]
macro_rules! NOKPROBE_SYMBOL {
    ($fname:ident) => {};
}

#[cfg(all(feature = "kernel", not(feature = "assembler"), not(feature = "kprobes")))]
macro_rules! __kprobes {
    ($item:item) => {
        $item
    };
}

#[cfg(all(feature = "kernel", not(feature = "assembler"), not(feature = "kprobes")))]
macro_rules! nokprobe_inline {
    ($item:item) => {
        #[inline]
        $item
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
