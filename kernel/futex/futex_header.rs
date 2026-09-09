/* SPDX-License-Identifier: GPL-2.0 */

// Translated from futex.h. Linux header dependencies are supplied externally.

pub const FLAGS_SIZE_8: u32 = 0x0000;
pub const FLAGS_SIZE_16: u32 = 0x0001;
pub const FLAGS_SIZE_32: u32 = 0x0002;
pub const FLAGS_SIZE_64: u32 = 0x0003;
pub const FLAGS_SIZE_MASK: u32 = 0x0003;

// CONFIG_MMU selects FLAGS_SHARED = 0x0010; NOMMU selects zero.
pub const FLAGS_SHARED: u32 = 0x0010;
pub const FLAGS_CLOCKRT: u32 = 0x0020;
pub const FLAGS_HAS_TIMEOUT: u32 = 0x0040;
pub const FLAGS_NUMA: u32 = 0x0080;
pub const FLAGS_STRICT: u32 = 0x0100;
pub const FLAGS_MPOL: u32 = 0x0200;
pub const FLAGS_ROBUST_UNLOCK: u32 = 0x0400;
pub const FLAGS_ROBUST_LIST32: u32 = 0x0800;

pub unsafe fn futex_to_flags(op: u32) -> u32 {
    let mut flags = FLAGS_SIZE_32;
    if (op & FUTEX_PRIVATE_FLAG) == 0 { flags |= FLAGS_SHARED; }
    if (op & FUTEX_CLOCK_REALTIME) != 0 { flags |= FLAGS_CLOCKRT; }
    if (op & FUTEX_ROBUST_UNLOCK) != 0 { flags |= FLAGS_ROBUST_UNLOCK; }
    if (op & FUTEX_ROBUST_LIST32) != 0 { flags |= FLAGS_ROBUST_LIST32; }
    flags
}

pub const FUTEX2_VALID_MASK: u32 = FUTEX2_SIZE_MASK | FUTEX2_NUMA | FUTEX2_MPOL | FUTEX2_PRIVATE;

pub unsafe fn futex2_to_flags(flags2: u32) -> u32 {
    let mut flags = flags2 & FUTEX2_SIZE_MASK;
    if (flags2 & FUTEX2_PRIVATE) == 0 { flags |= FLAGS_SHARED; }
    if (flags2 & FUTEX2_NUMA) != 0 { flags |= FLAGS_NUMA; }
    if (flags2 & FUTEX2_MPOL) != 0 { flags |= FLAGS_MPOL; }
    flags
}

pub const fn futex_size(flags: u32) -> u32 { 1 << (flags & FLAGS_SIZE_MASK) }

pub unsafe fn futex_flags_valid(flags: u32) -> bool {
    // CONFIG_64BIT and in_compat_syscall() are supplied by the target kernel.
    if (!IS_ENABLED_CONFIG_64BIT || in_compat_syscall()) && (flags & FLAGS_SIZE_MASK) == FLAGS_SIZE_64 { return false; }
    if (flags & FLAGS_SIZE_MASK) != FLAGS_SIZE_32 { return false; }
    if (flags & FLAGS_NUMA) != 0 {
        let bits = 8 * futex_size(flags);
        let mut max = !0u64;
        max >>= 64 - bits;
        if nr_node_ids >= max { return false; }
    }
    true
}

pub const fn futex_validate_input(flags: u32, val: u64) -> bool {
    let bits = 8 * futex_size(flags);
    if bits < 64 && (val >> bits) != 0 { return false; }
    true
}

// CONFIG_FAIL_FUTEX provides the external implementation; otherwise this returns false.
pub unsafe fn should_fail_futex(_fshared: bool) -> bool { false }

pub unsafe fn futex_key_is_private(key: *mut futex_key) -> bool {
    !(unsafe { (*key).both.offset } & (FUT_OFF_INODE | FUT_OFF_MMSHARED)) != 0
}

#[repr(C)]
pub struct futex_hash_bucket {
    pub waiters: atomic_t,
    pub lock: spinlock_t,
    pub chain: list_head,
}

#[repr(C)]
pub struct futex_pi_state {
    pub list: list_head,
    pub pi_mutex: rt_mutex_base,
    pub owner: *mut task_struct,
    pub refcount: refcount_t,
    pub key: futex_key,
}

pub struct futex_q;
pub type futex_wake_fn = unsafe extern "C" fn(*mut wake_q_head, *mut futex_q);

#[repr(C)]
pub struct futex_q {
    pub list: plist_node,
    pub task: *mut task_struct,
    pub lock_ptr: *mut spinlock_t,
    pub wake: Option<futex_wake_fn>,
    pub wake_data: *mut core::ffi::c_void,
    pub key: futex_key,
    pub pi_state: *mut futex_pi_state,
    pub rt_waiter: *mut rt_mutex_waiter,
    pub requeue_pi_key: *mut futex_key,
    pub bitset: u32,
    pub requeue_state: atomic_t,
    pub drop_fph: *mut futex_private_hash,
    // CONFIG_PREEMPT_RT: pub requeue_wait: rcuwait,
}

unsafe extern "C" {
    pub static futex_q_init: futex_q;
    pub fn get_futex_key(uaddr: *mut u32, flags: u32, key: *mut futex_key, rw: futex_access) -> i32;
    pub fn futex_q_lockptr_lock(q: *mut futex_q);
    pub fn futex_setup_timer(time: *mut ktime_t, timeout: *mut hrtimer_sleeper, flags: i32, range_ns: u64) -> *mut hrtimer_sleeper;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum futex_access { FUTEX_READ, FUTEX_WRITE }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
