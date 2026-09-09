/* SPDX-License-Identifier: GPL-2.0 */
/* bootstd.h:  Bootloader system call interface
 *
 * (c) 1999, Rt-Control, Inc.
 */

/* Calling conventions compatible to (uC)linux/68k. */

pub const NR_BSC: i32 = 21; /* last used bootloader system call */

pub const __BN_reset: i32 = 0; /* reset and start the bootloader */
pub const __BN_test: i32 = 1; /* tests the system call interface */
pub const __BN_exec: i32 = 2; /* executes a bootloader image */
pub const __BN_exit: i32 = 3; /* terminates a bootloader image */
pub const __BN_program: i32 = 4; /* program FLASH from a chain */
pub const __BN_erase: i32 = 5; /* erase sector(s) of FLASH */
pub const __BN_open: i32 = 6;
pub const __BN_write: i32 = 7;
pub const __BN_read: i32 = 8;
pub const __BN_close: i32 = 9;
pub const __BN_mmap: i32 = 10; /* map a file descriptor into memory */
pub const __BN_munmap: i32 = 11; /* remove a file to memory mapping */
pub const __BN_gethwaddr: i32 = 12; /* get the hardware address of my interfaces */
pub const __BN_getserialnum: i32 = 13; /* get the serial number of this board */
pub const __BN_getbenv: i32 = 14; /* get a bootloader envvar */
pub const __BN_setbenv: i32 = 15; /* get a bootloader envvar */
pub const __BN_setpmask: i32 = 16; /* set the protection mask */
pub const __BN_readenv: i32 = 17; /* read environment variables */
pub const __BN_flash_chattr_range: i32 = 18;
pub const __BN_flash_erase_range: i32 = 19;
pub const __BN_flash_write_range: i32 = 20;

extern "C" {
    pub static mut errno: i32;
}

macro_rules! __bsc_return {
    ($ty:ty, $res:expr) => {{
        let mut __res = $res;
        if (::__core::mem::transmute::<_, usize>(__res)) >=
            (::__core::mem::transmute::<_, usize>(-64isize))
        {
            /* let errno be a function, preserve res in d0 */
            let __err = -__res;
            unsafe { errno = __err as i32; }
            __res = -1;
        }
        return __res as $ty;
    }};
}

/* The following macros emit the 68k bootloader trap call. */
macro_rules! _bsc0 {
    ($ty:ty, $name:ident) => {
        pub unsafe fn $name() -> $ty {
            let mut __res: isize = __BN_$name as isize;
            core::arch::asm!("trap #2", inout("d0") __res);
            __bsc_return!($ty, __res);
        }
    };
}

macro_rules! _bsc1 {
    ($ty:ty, $name:ident, $atype:ty, $a:ident) => {
        pub unsafe fn $name($a: $atype) -> $ty {
            let mut __res: isize = __BN_$name as isize;
            let __a: isize = $a as isize;
            core::arch::asm!("trap #2", inout("d0") __res, in("d1") __a);
            __bsc_return!($ty, __res);
        }
    };
}

macro_rules! _bsc2 {
    ($ty:ty, $name:ident, $atype:ty, $a:ident, $btype:ty, $b:ident) => {
        pub unsafe fn $name($a: $atype, $b: $btype) -> $ty {
            let mut __res: isize = __BN_$name as isize;
            let __a: isize = $a as isize;
            let __b: isize = $b as isize;
            core::arch::asm!("trap #2", inout("d0") __res, in("d1") __a, in("d2") __b);
            __bsc_return!($ty, __res);
        }
    };
}

macro_rules! _bsc3 {
    ($ty:ty, $name:ident, $atype:ty, $a:ident, $btype:ty, $b:ident, $ctype:ty, $c:ident) => {
        pub unsafe fn $name($a: $atype, $b: $btype, $c: $ctype) -> $ty {
            let mut __res: isize = __BN_$name as isize;
            let __a: isize = $a as isize;
            let __b: isize = $b as isize;
            let __c: isize = $c as isize;
            core::arch::asm!("trap #2", inout("d0") __res, in("d1") __a, in("d2") __b, in("d3") __c);
            __bsc_return!($ty, __res);
        }
    };
}

macro_rules! _bsc4 {
    ($ty:ty, $name:ident, $atype:ty, $a:ident, $btype:ty, $b:ident, $ctype:ty, $c:ident, $dtype:ty, $d:ident) => {
        pub unsafe fn $name($a: $atype, $b: $btype, $c: $ctype, $d: $dtype) -> $ty {
            let mut __res: isize = __BN_$name as isize;
            let __a: isize = $a as isize; let __b: isize = $b as isize;
            let __c: isize = $c as isize; let __d: isize = $d as isize;
            core::arch::asm!("trap #2", inout("d0") __res, in("d1") __a, in("d2") __b, in("d3") __c, in("d4") __d);
            __bsc_return!($ty, __res);
        }
    };
}

macro_rules! _bsc5 {
    ($ty:ty, $name:ident, $atype:ty, $a:ident, $btype:ty, $b:ident, $ctype:ty, $c:ident, $dtype:ty, $d:ident, $etype:ty, $e:ident) => {
        pub unsafe fn $name($a: $atype, $b: $btype, $c: $ctype, $d: $dtype, $e: $etype) -> $ty {
            let mut __res: isize = __BN_$name as isize;
            let __a: isize = $a as isize; let __b: isize = $b as isize;
            let __c: isize = $c as isize; let __d: isize = $d as isize; let __e: isize = $e as isize;
            core::arch::asm!("trap #2", inout("d0") __res, in("d1") __a, in("d2") __b, in("d3") __c, in("d4") __d, in("d5") __e);
            __bsc_return!($ty, __res);
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
