/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* const.h: Macros for dealing with constants.  */

/* Some constant macros are used in both assembler and
 * C code.  Therefore we cannot annotate them always with
 * 'UL' and other type specifiers unilaterally.  We
 * use the following macros to deal with this.
 *
 * Similarly, _AT() will cast an expression with a type in C, but
 * leave it unchanged in asm.
 */

/* C header guard removed in Rust translation. */

/* __ASSEMBLY__ branch intent:
 * In assembly, _AC(X,Y) and _AT(T,X) leave X unchanged.  This Rust
 * translation represents the non-assembly C-facing forms.
 */
macro_rules! __AC {
    ($x:expr, $y:ident) => {
        ($x)
    };
}

macro_rules! _AC {
    ($x:expr, $y:ident) => {
        __AC!($x, $y)
    };
}

macro_rules! _AT {
    ($t:ty, $x:expr) => {
        (($x) as $t)
    };
}

macro_rules! _UL {
    ($x:expr) => {
        (_AC!($x, UL))
    };
}

macro_rules! _ULL {
    ($x:expr) => {
        (_AC!($x, ULL))
    };
}

macro_rules! _BITUL {
    ($x:expr) => {
        (_UL!(1) << ($x))
    };
}

macro_rules! _BITULL {
    ($x:expr) => {
        (_ULL!(1) << ($x))
    };
}

/*
 * Missing asm support
 *
 * __BIT128() would not work in the asm code, as it shifts an
 * 'unsigned __int128' data type as direct representation of
 * 128 bit constants is not supported in the gcc compiler, as
 * they get silently truncated.
 *
 * TODO: Please revisit this implementation when gcc compiler
 * starts representing 128 bit constants directly like long
 * and unsigned long etc. Subsequently drop the comment for
 * GENMASK_U128() which would then start supporting asm code.
 */
macro_rules! _BIT128 {
    ($x:expr) => {
        ((1u128) << ($x))
    };
}

macro_rules! __ALIGN_KERNEL {
    ($x:expr, $a:expr) => {
        __ALIGN_KERNEL_MASK!($x, (($a) as _) - 1)
    };
}

macro_rules! __ALIGN_KERNEL_MASK {
    ($x:expr, $mask:expr) => {
        ((($x) + ($mask)) & !($mask))
    };
}

macro_rules! __KERNEL_DIV_ROUND_UP {
    ($n:expr, $d:expr) => {
        ((($n) + ($d) - 1) / ($d))
    };
}

/*
 * Divide positive or negative dividend by positive or negative divisor
 * and round to closest integer. Result is undefined for negative
 * divisors if the dividend variable type is unsigned and for negative
 * dividends if the divisor variable type is unsigned.
 */
macro_rules! __KERNEL_DIV_ROUND_CLOSEST {
    ($x:expr, $divisor:expr) => {{
        let __x = $x;
        let __d = $divisor;

        if ((-1 as _) > 0) || ((-1 as _) > 0) || (((__x) > 0) == ((__d) > 0)) {
            (((__x) + ((__d) / 2)) / (__d))
        } else {
            (((__x) - ((__d) / 2)) / (__d))
        }
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
