// Translated from include/linux/linkage.h.
// C header guard and `#include <linux/export.h>` are omitted; that dependency
// remains external to this translated header.

macro_rules! SYM_FUNC_START {
    ($x:ident) => {
        ::core::arch::global_asm!(concat!(".globl ", stringify!($x), "\n", stringify!($x), ":"));
    };
}

macro_rules! SYM_FUNC_END {
    ($x:ident) => {};
}

macro_rules! SYM_DATA_START {
    ($x:ident) => {
        ::core::arch::global_asm!(concat!(".globl ", stringify!($x), "\n", stringify!($x), ":"));
    };
}

macro_rules! SYM_DATA_START_LOCAL {
    ($x:ident) => {
        ::core::arch::global_asm!(concat!(stringify!($x), ":"));
    };
}

macro_rules! SYM_DATA_END {
    ($x:ident) => {};
}
