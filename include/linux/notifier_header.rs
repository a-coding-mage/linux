/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Routines to manage notifier chains for passing status changes to any
 * interested routines. We need this instead of hard coded call lists so
 * that modules can poke their nose into the innards. The network devices
 * needed them so here they are for the rest of you.
 *
 *                         Alan Cox <Alan.Cox@linux.org>
 */

// C header dependencies: linux/errno.h, linux/mutex.h, linux/rwsem.h,
// linux/srcu.h, and the kernel synchronization types/macros they provide.

pub type CInt = i32;
pub type CULong = usize;

pub type SpinlockT = core::ffi::c_void;
pub type RwSemaphore = core::ffi::c_void;
pub type Mutex = core::ffi::c_void;
pub type SrcuUsage = core::ffi::c_void;
pub type SrcuStruct = core::ffi::c_void;

#[repr(C)]
pub struct NotifierBlock {
    pub notifier_call: Option<unsafe extern "C" fn(*mut NotifierBlock, CULong, *mut core::ffi::c_void) -> CInt>,
    pub next: *mut NotifierBlock,
    pub priority: CInt,
}

#[repr(C)]
pub struct AtomicNotifierHead {
    pub lock: SpinlockT,
    pub head: *mut NotifierBlock,
}

#[repr(C)]
pub struct BlockingNotifierHead {
    pub rwsem: RwSemaphore,
    pub head: *mut NotifierBlock,
}

#[repr(C)]
pub struct RawNotifierHead {
    pub head: *mut NotifierBlock,
}

#[repr(C)]
pub struct SrcuNotifierHead {
    pub mutex: Mutex,
    pub srcuu: SrcuUsage,
    pub srcu: SrcuStruct,
    pub head: *mut NotifierBlock,
}

// Initialization macros depend on external kernel initialization routines.
#[macro_export]
macro_rules! atomic_init_notifier_head {
    ($name:expr) => {{
        unsafe { spin_lock_init(core::ptr::addr_of_mut!((*$name).lock)); }
        unsafe { (*$name).head = core::ptr::null_mut(); }
    }};
}
#[macro_export]
macro_rules! blocking_init_notifier_head {
    ($name:expr) => {{
        unsafe { init_rwsem(core::ptr::addr_of_mut!((*$name).rwsem)); }
        unsafe { (*$name).head = core::ptr::null_mut(); }
    }};
}
#[macro_export]
macro_rules! raw_init_notifier_head {
    ($name:expr) => {{ unsafe { (*$name).head = core::ptr::null_mut(); } }};
}

extern "C" {
    pub fn srcu_init_notifier_head(nh: *mut SrcuNotifierHead);
}

#[macro_export]
macro_rules! srcu_cleanup_notifier_head {
    ($name:expr) => {{ unsafe { cleanup_srcu_struct(core::ptr::addr_of_mut!((*$name).srcu)); } }};
}

// Static initializer macros are represented as direct Rust expressions;
// external kernel initializer values remain dependencies supplied elsewhere.
#[macro_export]
macro_rules! atomic_notifier_init { ($name:ident) => { AtomicNotifierHead { lock: unsafe { core::mem::zeroed() }, head: core::ptr::null_mut() } }; }
#[macro_export]
macro_rules! blocking_notifier_init { ($name:ident) => { BlockingNotifierHead { rwsem: unsafe { core::mem::zeroed() }, head: core::ptr::null_mut() } }; }
#[macro_export]
macro_rules! raw_notifier_init { ($name:ident) => { RawNotifierHead { head: core::ptr::null_mut() } }; }

// CONFIG_TREE_SRCU controls the original per-CPU SRCU storage at build time.
// The SRCU initializer's external kernel-specific fields are not defined here.

#[cfg(feature = "kernel")]
extern "C" {
    pub fn atomic_notifier_chain_register(nh: *mut AtomicNotifierHead, nb: *mut NotifierBlock) -> CInt;
    pub fn blocking_notifier_chain_register(nh: *mut BlockingNotifierHead, nb: *mut NotifierBlock) -> CInt;
    pub fn raw_notifier_chain_register(nh: *mut RawNotifierHead, nb: *mut NotifierBlock) -> CInt;
    pub fn srcu_notifier_chain_register(nh: *mut SrcuNotifierHead, nb: *mut NotifierBlock) -> CInt;
    pub fn atomic_notifier_chain_register_unique_prio(nh: *mut AtomicNotifierHead, nb: *mut NotifierBlock) -> CInt;
    pub fn blocking_notifier_chain_register_unique_prio(nh: *mut BlockingNotifierHead, nb: *mut NotifierBlock) -> CInt;
    pub fn atomic_notifier_chain_unregister(nh: *mut AtomicNotifierHead, nb: *mut NotifierBlock) -> CInt;
    pub fn blocking_notifier_chain_unregister(nh: *mut BlockingNotifierHead, nb: *mut NotifierBlock) -> CInt;
    pub fn raw_notifier_chain_unregister(nh: *mut RawNotifierHead, nb: *mut NotifierBlock) -> CInt;
    pub fn srcu_notifier_chain_unregister(nh: *mut SrcuNotifierHead, nb: *mut NotifierBlock) -> CInt;
    pub fn atomic_notifier_call_chain(nh: *mut AtomicNotifierHead, val: CULong, v: *mut core::ffi::c_void) -> CInt;
    pub fn blocking_notifier_call_chain(nh: *mut BlockingNotifierHead, val: CULong, v: *mut core::ffi::c_void) -> CInt;
    pub fn raw_notifier_call_chain(nh: *mut RawNotifierHead, val: CULong, v: *mut core::ffi::c_void) -> CInt;
    pub fn srcu_notifier_call_chain(nh: *mut SrcuNotifierHead, val: CULong, v: *mut core::ffi::c_void) -> CInt;
    pub fn blocking_notifier_call_chain_robust(nh: *mut BlockingNotifierHead, val_up: CULong, val_down: CULong, v: *mut core::ffi::c_void) -> CInt;
    pub fn raw_notifier_call_chain_robust(nh: *mut RawNotifierHead, val_up: CULong, val_down: CULong, v: *mut core::ffi::c_void) -> CInt;
    pub fn atomic_notifier_call_chain_is_empty(nh: *mut AtomicNotifierHead) -> bool;
}

pub const NOTIFY_DONE: CInt = 0x0000;
pub const NOTIFY_OK: CInt = 0x0001;
pub const NOTIFY_STOP_MASK: CInt = 0x8000;
pub const NOTIFY_BAD: CInt = NOTIFY_STOP_MASK | 0x0002;
pub const NOTIFY_STOP: CInt = NOTIFY_OK | NOTIFY_STOP_MASK;

pub const NETLINK_URELEASE: CInt = 0x0001;
pub const KBD_KEYCODE: CInt = 0x0001;
pub const KBD_UNBOUND_KEYCODE: CInt = 0x0002;
pub const KBD_UNICODE: CInt = 0x0003;
pub const KBD_KEYSYM: CInt = 0x0004;
pub const KBD_POST_KEYSYM: CInt = 0x0005;

#[inline]
pub fn notifier_from_errno(err: CInt) -> CInt {
    if err != 0 { NOTIFY_STOP_MASK | (NOTIFY_OK - err) } else { NOTIFY_OK }
}

#[inline]
pub fn notifier_to_errno(mut ret: CInt) -> CInt {
    ret &= !NOTIFY_STOP_MASK;
    if ret > NOTIFY_OK { NOTIFY_OK - ret } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
