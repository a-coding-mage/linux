/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003, 2004 Ralf Baechle
 */

// __ASM_MACH_IP27_MANGLE_PORT_H

macro_rules! __swizzle_addr_b {
    ($port:expr) => { ($port) ^ 3 };
}

macro_rules! __swizzle_addr_w {
    ($port:expr) => { ($port) ^ 2 };
}

macro_rules! __swizzle_addr_l {
    ($port:expr) => { $port };
}

macro_rules! __swizzle_addr_q {
    ($port:expr) => { $port };
}

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


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
