/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM is `lock`.
// The Linux tracepoint and lockdep includes are supplied by other translation units.

/// Flags for `contention_begin`.
pub const LCB_F_SPIN: ::core::ffi::c_uint = 1u32 << 0;
pub const LCB_F_READ: ::core::ffi::c_uint = 1u32 << 1;
pub const LCB_F_WRITE: ::core::ffi::c_uint = 1u32 << 2;
pub const LCB_F_RT: ::core::ffi::c_uint = 1u32 << 3;
pub const LCB_F_PERCPU: ::core::ffi::c_uint = 1u32 << 4;
pub const LCB_F_MUTEX: ::core::ffi::c_uint = 1u32 << 5;

// Under CONFIG_LOCKDEP, the following trace events are declared:
//
// lock_acquire(lock: *mut lockdep_map, subclass: c_uint, trylock: c_int,
//              read: c_int, check: c_int, next_lock: *mut lockdep_map,
//              ip: c_ulong), recording flags `(trylock != 0) | ((read != 0) << 1)`,
//              the lock name, and the lockdep address.
// lock_release(lock: *mut lockdep_map, ip: c_ulong)
//
// Under CONFIG_LOCK_STAT, the following additional events use the same layout:
// lock_contended(lock: *mut lockdep_map, ip: c_ulong)
// lock_acquired(lock: *mut lockdep_map, ip: c_ulong)

// The tracepoint framework supplies the declarations for `contention_begin` and
// `contention_end`:
//
// contention_begin(lock: *mut c_void, flags: c_uint), recording lock_addr and flags
// and printing the named LCB_* flags.
// contention_end(lock: *mut c_void, ret: c_int), recording lock_addr and ret.

extern "C" {
    pub fn arch_contended_release_trace_reg() -> ::core::ffi::c_int;
    pub fn arch_contended_release_trace_unreg();
}

// The tracepoint framework declares `contended_release(lock: *mut c_void)`,
// recording lock_addr, printing it, and registering the two functions above.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
