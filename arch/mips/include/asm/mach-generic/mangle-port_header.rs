/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003, 2004 Ralf Baechle
 */

macro_rules! __swizzle_addr_b {
    ($port:expr) => { $port };
}
macro_rules! __swizzle_addr_w {
    ($port:expr) => { $port };
}
macro_rules! __swizzle_addr_l {
    ($port:expr) => { $port };
}
macro_rules! __swizzle_addr_q {
    ($port:expr) => { $port };
}

/*
 * Sane hardware offers swapping of PCI/ISA I/O space accesses in hardware;
 * less sane hardware forces software to fiddle with this...
 *
 * Regardless, if the host bus endianness mismatches that of PCI/ISA, then
 * you can't have the numerical value of data and byte addresses within
 * multibyte quantities both preserved at the same time.  Hence two
 * variations of functions: non-prefixed ones that preserve the value
 * and prefixed ones that preserve byte addresses.  The latters are
 * typically used for moving raw data between a peripheral and memory (cf.
 * string I/O functions), hence the "__mem_" prefix.
 */

/* CONFIG_SWAP_IO_SPACE build-time condition from the original header. */
#[cfg(feature = "CONFIG_SWAP_IO_SPACE")]
mod config_swap_io_space {
    macro_rules! ioswabb {
        ($a:expr, $x:expr) => { $x };
    }
    macro_rules! __mem_ioswabb {
        ($a:expr, $x:expr) => { $x };
    }
    macro_rules! ioswabw {
        ($a:expr, $x:expr) => { le16_to_cpu($x as u16) };
    }
    macro_rules! __mem_ioswabw {
        ($a:expr, $x:expr) => { $x };
    }
    macro_rules! ioswabl {
        ($a:expr, $x:expr) => { le32_to_cpu($x as u32) };
    }
    macro_rules! __mem_ioswabl {
        ($a:expr, $x:expr) => { $x };
    }
    macro_rules! ioswabq {
        ($a:expr, $x:expr) => { le64_to_cpu($x as u64) };
    }
    macro_rules! __mem_ioswabq {
        ($a:expr, $x:expr) => { $x };
    }
}

#[cfg(not(feature = "CONFIG_SWAP_IO_SPACE"))]
mod config_no_swap_io_space {
    macro_rules! ioswabb {
        ($a:expr, $x:expr) => { $x };
    }
    macro_rules! __mem_ioswabb {
        ($a:expr, $x:expr) => { $x };
    }
    macro_rules! ioswabw {
        ($a:expr, $x:expr) => { $x };
    }
    macro_rules! __mem_ioswabw {
        ($a:expr, $x:expr) => { cpu_to_le16($x) as u16 };
    }
    macro_rules! ioswabl {
        ($a:expr, $x:expr) => { $x };
    }
    macro_rules! __mem_ioswabl {
        ($a:expr, $x:expr) => { cpu_to_le32($x) as u32 };
    }
    macro_rules! ioswabq {
        ($a:expr, $x:expr) => { $x };
    }
    macro_rules! __mem_ioswabq {
        ($a:expr, $x:expr) => { cpu_to_le64($x) as u64 };
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
