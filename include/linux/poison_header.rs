/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Architectures might want to move the poison pointer offset
 * into some well-recognized area such as 0xdead000000000000,
 * that is also not mappable by user-space exploits.
 *
 * CONFIG_ILLEGAL_POINTER_VALUE is a build-time configuration value supplied
 * by the surrounding build; when it is unavailable, the offset is zero.
 */
#[cfg(CONFIG_ILLEGAL_POINTER_VALUE)]
pub const POISON_POINTER_DELTA: usize = CONFIG_ILLEGAL_POINTER_VALUE as usize;
#[cfg(not(CONFIG_ILLEGAL_POINTER_VALUE))]
pub const POISON_POINTER_DELTA: usize = 0;

/*
 * These are non-NULL pointers that will result in page faults
 * under normal circumstances, used to verify that nobody uses
 * non-initialized list entries.
 */
pub const LIST_POISON1: *mut core::ffi::c_void =
    (0x100usize + POISON_POINTER_DELTA) as *mut core::ffi::c_void;
pub const LIST_POISON2: *mut core::ffi::c_void =
    (0x122usize + POISON_POINTER_DELTA) as *mut core::ffi::c_void;

pub const TIMER_ENTRY_STATIC: *mut core::ffi::c_void =
    (0x300usize + POISON_POINTER_DELTA) as *mut core::ffi::c_void;

pub const PAGE_POISON: u8 = 0xaa;

pub const TAIL_MAPPING: *mut core::ffi::c_void =
    (0x400usize + POISON_POINTER_DELTA) as *mut core::ffi::c_void;

/*
 * Magic nums for obj red zoning.
 * Placed in the first word before and the first word after an obj.
 */
pub const SLUB_RED_INACTIVE: u8 = 0xbb; /* when obj is inactive */
pub const SLUB_RED_ACTIVE: u8 = 0xcc; /* when obj is active */

/* ...and for poisoning */
pub const POISON_INUSE: u8 = 0x5a; /* for use-uninitialised poisoning */
pub const POISON_FREE: u8 = 0x6b; /* for use-after-free poisoning */
pub const POISON_END: u8 = 0xa5; /* end-byte of poisoning */

pub const POISON_FREE_INITMEM: u8 = 0xcc;

pub const JBD_POISON_FREE: u8 = 0x5b;
pub const JBD2_POISON_FREE: u8 = 0x5c;

pub const POOL_POISON_FREED: u8 = 0xa7; /* !inuse */
pub const POOL_POISON_ALLOCATED: u8 = 0xa9; /* !initted */

pub const ATM_POISON_FREE: u8 = 0x12;
pub const ATM_POISON: u32 = 0xdeadbeef;

pub const MUTEX_DEBUG_INIT: u8 = 0x11;
pub const MUTEX_DEBUG_FREE: u8 = 0x22;
pub const MUTEX_POISON_WW_CTX: *mut core::ffi::c_void =
    (0x500usize + POISON_POINTER_DELTA) as *mut core::ffi::c_void;

pub const KEY_DESTROY: u8 = 0xbd;

/*
 * page_pool uses additional free bits within this value to store data, see
 * the definition of PP_DMA_INDEX_MASK in mm.h
 */
pub const PP_SIGNATURE: usize = 0x40usize + POISON_POINTER_DELTA;

pub const SKB_LIST_POISON_NEXT: *mut core::ffi::c_void =
    (0x800usize + POISON_POINTER_DELTA) as *mut core::ffi::c_void;
pub const NET_PTR_POISON: *mut core::ffi::c_void =
    (0x801usize + POISON_POINTER_DELTA) as *mut core::ffi::c_void;

pub const BPF_PTR_POISON: *mut core::ffi::c_void =
    (0xeB9Fusize + POISON_POINTER_DELTA) as *mut core::ffi::c_void;

pub const VFS_PTR_POISON: *mut core::ffi::c_void =
    (0xF5usize + POISON_POINTER_DELTA) as *mut core::ffi::c_void;

pub const STACK_DEPOT_POISON: *mut core::ffi::c_void =
    (0xD390usize + POISON_POINTER_DELTA) as *mut core::ffi::c_void;

pub const IO_URING_PTR_POISON: *mut core::ffi::c_void =
    (0x1091usize + POISON_POINTER_DELTA) as *mut core::ffi::c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
