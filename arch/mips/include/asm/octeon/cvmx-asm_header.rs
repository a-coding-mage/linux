/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2008 Cavium Networks
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2, or (at your option) any
 * later version.
 ***********************license end**************************************/

// C dependency: asm/octeon/octeon-model.h is supplied by the surrounding build.

pub const CVMX_SYNCW_STR: &str = "syncw\nsyncw\n";

// The original header selects the Cavium spelling on __OCTEON__ and the
// conservative SYNC spelling otherwise.  These macros retain that intent.
#[macro_export]
macro_rules! CVMX_SYNC { () => { unsafe { core::arch::asm!("sync", options(nostack, preserves_flags)); } }; }

#[cfg(target_arch = "mips")]
#[macro_export]
macro_rules! CVMX_SYNCIO { () => { unsafe { core::arch::asm!("nop", options(nostack, preserves_flags)); } }; }
#[cfg(not(target_arch = "mips"))]
#[macro_export]
macro_rules! CVMX_SYNCIO { () => { unsafe { core::arch::asm!("nop", options(nostack, preserves_flags)); } }; }

#[cfg(target_arch = "mips")]
#[macro_export]
macro_rules! CVMX_SYNCIOBDMA { () => { unsafe { core::arch::asm!("synciobdma", options(nostack, preserves_flags)); } }; }
#[cfg(not(target_arch = "mips"))]
#[macro_export]
macro_rules! CVMX_SYNCIOBDMA { () => { unsafe { core::arch::asm!("sync", options(nostack, preserves_flags)); } }; }

#[macro_export]
macro_rules! CVMX_SYNCIOALL { () => { unsafe { core::arch::asm!("nop", options(nostack, preserves_flags)); } }; }

#[macro_export]
macro_rules! CVMX_SYNCW { () => { unsafe { core::arch::asm!("syncw\n\tsyncw", options(nostack, preserves_flags)); } }; }
#[macro_export]
macro_rules! CVMX_SYNCWS { () => { $crate::CVMX_SYNCW!(); } }
#[macro_export]
macro_rules! CVMX_SYNCS { () => { $crate::CVMX_SYNC!(); } }

#[macro_export]
macro_rules! CVMX_PREPARE_FOR_STORE {
    ($address:expr, $offset:expr) => { unsafe { core::arch::asm!("pref 30, {off}({base})", off = const $offset, base = in(reg) $address, options(nostack, preserves_flags)); } };
}
#[macro_export]
macro_rules! CVMX_DONT_WRITE_BACK {
    ($address:expr, $offset:expr) => { unsafe { core::arch::asm!("pref 29, {off}({base})", off = const $offset, base = in(reg) $address, options(nostack, preserves_flags)); } };
}

#[macro_export]
macro_rules! CVMX_ICACHE_INVALIDATE { () => {{ $crate::CVMX_SYNC!(); unsafe { core::arch::asm!("synci 0($0)", options(nostack, preserves_flags)); } }}; }
#[macro_export]
macro_rules! CVMX_ICACHE_INVALIDATE2 { () => {{ $crate::CVMX_SYNC!(); unsafe { core::arch::asm!("cache 0, 0($0)", options(nostack, preserves_flags)); } }}; }
#[macro_export]
macro_rules! CVMX_DCACHE_INVALIDATE { () => {{ $crate::CVMX_SYNC!(); unsafe { core::arch::asm!("cache 9, 0($0)", options(nostack, preserves_flags)); } }}; }

#[macro_export]
macro_rules! CVMX_CACHE {
    ($op:expr, $address:expr, $offset:expr) => { unsafe { core::arch::asm!("cache {op}, {off}({base})", op = const $op, off = const $offset, base = in(reg) $address, options(nostack, preserves_flags)); } };
}
#[macro_export] macro_rules! CVMX_CACHE_LCKL2 { ($address:expr, $offset:expr) => { $crate::CVMX_CACHE!(31, $address, $offset); }; }
#[macro_export] macro_rules! CVMX_CACHE_WBIL2 { ($address:expr, $offset:expr) => { $crate::CVMX_CACHE!(23, $address, $offset); }; }
#[macro_export] macro_rules! CVMX_CACHE_WBIL2I { ($address:expr, $offset:expr) => { $crate::CVMX_CACHE!(3, $address, $offset); }; }
#[macro_export] macro_rules! CVMX_CACHE_LTGL2I { ($address:expr, $offset:expr) => { $crate::CVMX_CACHE!(7, $address, $offset); }; }

#[macro_export]
macro_rules! CVMX_POP { ($result:expr, $input:expr) => { unsafe { core::arch::asm!("pop {rd}, {rs}", rd = out(reg) $result, rs = in(reg) $input, options(nostack)); } }; }
#[macro_export]
macro_rules! CVMX_DPOP { ($result:expr, $input:expr) => { unsafe { core::arch::asm!("dpop {rd}, {rs}", rd = out(reg) $result, rs = in(reg) $input, options(nostack)); } }; }
#[macro_export]
macro_rules! CVMX_RDHWR { ($result:expr, $regstr:expr) => { unsafe { core::arch::asm!(concat!("rdhwr {rt},$", stringify!($regstr)), rt = out(reg) $result, options(nostack)); } }; }
#[macro_export]
macro_rules! CVMX_RDHWRNV { ($result:expr, $regstr:expr) => { unsafe { core::arch::asm!(concat!("rdhwr {rt},$", stringify!($regstr)), rt = out(reg) $result, options(nostack)); } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
