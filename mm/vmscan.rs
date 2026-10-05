// SPDX-License-Identifier: GPL-2.0-only
//! Reclaim policy and mechanics, transcribed from native mm/vmscan.c.
//! Native headers remain the authority for layouts, bitfields and constants.
use bindings as b;
use bindings::*;
use core::cmp::{max, min};
use core::mem::{align_of, offset_of, size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null, null_mut};
use kernel::bindings::vmscan_native as bindings;
use kernel::ffi::{c_char as CChar, c_long as Long, c_ulong as ULong, c_void as Void};
include!("vmscan_primitive_aliases.rs");
include!("vmscan_constants.rs");
include!("vmscan_classic.rs");
include!("vmscan_mglru.rs");
include!("vmscan_direct.rs");
const _: () = {
    assert!(size_of::<scan_control>() == RUST_VS_SIZEOF_SCAN_CONTROL as usize);
    assert!(align_of::<scan_control>() == RUST_VS_ALIGNOF_SCAN_CONTROL as usize);
    assert!(PGSTEAL_DIRECT - PGSTEAL_KSWAPD == PGDEMOTE_DIRECT - PGDEMOTE_KSWAPD);
    assert!(PGSTEAL_DIRECT - PGSTEAL_KSWAPD == PGSCAN_DIRECT - PGSCAN_KSWAPD);
    assert!(PGSTEAL_KHUGEPAGED - PGSTEAL_KSWAPD == PGDEMOTE_KHUGEPAGED - PGDEMOTE_KSWAPD);
    assert!(PGSTEAL_KHUGEPAGED - PGSTEAL_KSWAPD == PGSCAN_KHUGEPAGED - PGSCAN_KSWAPD);
    assert!(PGSTEAL_PROACTIVE - PGSTEAL_KSWAPD == PGDEMOTE_PROACTIVE - PGDEMOTE_KSWAPD);
    assert!(PGSTEAL_PROACTIVE - PGSTEAL_KSWAPD == PGSCAN_PROACTIVE - PGSCAN_KSWAPD);
};

// Reconciled against frozen native e1d84f501551943a11f4c5271e9f5c85d7e15168.
// Historical initial transcription: d482bb509b7d065808de40ce78b5bca39f40b783
// SOURCE-COMMIT: e1d84f501551943a11f4c5271e9f5c85d7e15168
