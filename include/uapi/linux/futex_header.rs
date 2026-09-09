/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency intent: uses Linux ABI integer types __u32 and __u64. */

/* Second argument to futex syscall */
pub const FUTEX_WAIT: u32 = 0;
pub const FUTEX_WAKE: u32 = 1;
pub const FUTEX_FD: u32 = 2;
pub const FUTEX_REQUEUE: u32 = 3;
pub const FUTEX_CMP_REQUEUE: u32 = 4;
pub const FUTEX_WAKE_OP: u32 = 5;
pub const FUTEX_LOCK_PI: u32 = 6;
pub const FUTEX_UNLOCK_PI: u32 = 7;
pub const FUTEX_TRYLOCK_PI: u32 = 8;
pub const FUTEX_WAIT_BITSET: u32 = 9;
pub const FUTEX_WAKE_BITSET: u32 = 10;
pub const FUTEX_WAIT_REQUEUE_PI: u32 = 11;
pub const FUTEX_CMP_REQUEUE_PI: u32 = 12;
pub const FUTEX_LOCK_PI2: u32 = 13;

pub const FUTEX_PRIVATE_FLAG: u32 = 128;
pub const FUTEX_CLOCK_REALTIME: u32 = 256;
pub const FUTEX_ROBUST_UNLOCK: u32 = 512;
pub const FUTEX_ROBUST_LIST32: u32 = 1024;
pub const FUTEX_CMD_MASK: u32 = !(FUTEX_PRIVATE_FLAG | FUTEX_CLOCK_REALTIME | FUTEX_ROBUST_UNLOCK | FUTEX_ROBUST_LIST32);

pub const FUTEX_WAIT_PRIVATE: u32 = FUTEX_WAIT | FUTEX_PRIVATE_FLAG;
pub const FUTEX_WAKE_PRIVATE: u32 = FUTEX_WAKE | FUTEX_PRIVATE_FLAG;
pub const FUTEX_REQUEUE_PRIVATE: u32 = FUTEX_REQUEUE | FUTEX_PRIVATE_FLAG;
pub const FUTEX_CMP_REQUEUE_PRIVATE: u32 = FUTEX_CMP_REQUEUE | FUTEX_PRIVATE_FLAG;
pub const FUTEX_WAKE_OP_PRIVATE: u32 = FUTEX_WAKE_OP | FUTEX_PRIVATE_FLAG;
pub const FUTEX_LOCK_PI_PRIVATE: u32 = FUTEX_LOCK_PI | FUTEX_PRIVATE_FLAG;
pub const FUTEX_LOCK_PI2_PRIVATE: u32 = FUTEX_LOCK_PI2 | FUTEX_PRIVATE_FLAG;
pub const FUTEX_UNLOCK_PI_PRIVATE: u32 = FUTEX_UNLOCK_PI | FUTEX_PRIVATE_FLAG;
pub const FUTEX_TRYLOCK_PI_PRIVATE: u32 = FUTEX_TRYLOCK_PI | FUTEX_PRIVATE_FLAG;
pub const FUTEX_WAIT_BITSET_PRIVATE: u32 = FUTEX_WAIT_BITSET | FUTEX_PRIVATE_FLAG;
pub const FUTEX_WAKE_BITSET_PRIVATE: u32 = FUTEX_WAKE_BITSET | FUTEX_PRIVATE_FLAG;
pub const FUTEX_WAIT_REQUEUE_PI_PRIVATE: u32 = FUTEX_WAIT_REQUEUE_PI | FUTEX_PRIVATE_FLAG;
pub const FUTEX_CMP_REQUEUE_PI_PRIVATE: u32 = FUTEX_CMP_REQUEUE_PI | FUTEX_PRIVATE_FLAG;

/* Operations to unlock a futex, clear the robust list pending op pointer and
 * wake waiters. */
pub const FUTEX_UNLOCK_PI_LIST64: u32 = FUTEX_UNLOCK_PI | FUTEX_ROBUST_UNLOCK;
pub const FUTEX_UNLOCK_PI_LIST64_PRIVATE: u32 = FUTEX_UNLOCK_PI_LIST64 | FUTEX_PRIVATE_FLAG;
pub const FUTEX_UNLOCK_PI_LIST32: u32 = FUTEX_UNLOCK_PI | FUTEX_ROBUST_UNLOCK | FUTEX_ROBUST_LIST32;
pub const FUTEX_UNLOCK_PI_LIST32_PRIVATE: u32 = FUTEX_UNLOCK_PI_LIST32 | FUTEX_PRIVATE_FLAG;
pub const FUTEX_UNLOCK_WAKE_LIST64: u32 = FUTEX_WAKE | FUTEX_ROBUST_UNLOCK;
pub const FUTEX_UNLOCK_WAKE_LIST64_PRIVATE: u32 = FUTEX_UNLOCK_WAKE_LIST64 | FUTEX_PRIVATE_FLAG;
pub const FUTEX_UNLOCK_WAKE_LIST32: u32 = FUTEX_WAKE | FUTEX_ROBUST_UNLOCK | FUTEX_ROBUST_LIST32;
pub const FUTEX_UNLOCK_WAKE_LIST32_PRIVATE: u32 = FUTEX_UNLOCK_WAKE_LIST32 | FUTEX_PRIVATE_FLAG;
pub const FUTEX_UNLOCK_BITSET_LIST64: u32 = FUTEX_WAKE_BITSET | FUTEX_ROBUST_UNLOCK;
pub const FUTEX_UNLOCK_BITSET_LIST64_PRIVATE: u32 = FUTEX_UNLOCK_BITSET_LIST64 | FUTEX_PRIVATE_FLAG;
pub const FUTEX_UNLOCK_BITSET_LIST32: u32 = FUTEX_WAKE_BITSET | FUTEX_ROBUST_UNLOCK | FUTEX_ROBUST_LIST32;
pub const FUTEX_UNLOCK_BITSET_LIST32_PRIVATE: u32 = FUTEX_UNLOCK_BITSET_LIST32 | FUTEX_PRIVATE_FLAG;

/* Flags for futex2 syscalls. These are also interpreted as a bitfield:
 * size:2, numa:1, reserved:4, private:1. */
pub const FUTEX2_SIZE_U8: u32 = 0x00;
pub const FUTEX2_SIZE_U16: u32 = 0x01;
pub const FUTEX2_SIZE_U32: u32 = 0x02;
pub const FUTEX2_SIZE_U64: u32 = 0x03;
pub const FUTEX2_NUMA: u32 = 0x04;
pub const FUTEX2_MPOL: u32 = 0x08;
pub const FUTEX2_PRIVATE: u32 = FUTEX_PRIVATE_FLAG;
pub const FUTEX2_SIZE_MASK: u32 = 0x03;

/* do not use */
pub const FUTEX_32: u32 = FUTEX2_SIZE_U32; /* historical accident :-( */

/* When FUTEX2_NUMA doubles the futex word, the second word is a node value. */
pub const FUTEX_NO_NODE: i32 = -1;
pub const FUTEX_WAITV_MAX: u32 = 128;

#[repr(C)]
pub struct futex_waitv {
    pub val: __u64,
    pub uaddr: __u64,
    pub flags: __u32,
    pub __reserved: __u32,
}

/* Support for robust futexes: the kernel cleans up held futexes at thread exit time. */
#[repr(C)]
pub struct robust_list {
    pub next: *mut robust_list,
}

#[repr(C)]
pub struct robust_list_head {
    pub list: robust_list,
    pub futex_offset: libc::c_long,
    pub list_op_pending: *mut robust_list,
}

pub const FUTEX_WAITERS: u32 = 0x80000000;
pub const FUTEX_OWNER_DIED: u32 = 0x40000000;
pub const FUTEX_TID_MASK: u32 = 0x3fffffff;
pub const ROBUST_LIST_LIMIT: u32 = 2048;
pub const FUTEX_ROBUST_MOD_PI: usize = 0x1;
pub const FUTEX_ROBUST_MOD_MASK: usize = FUTEX_ROBUST_MOD_PI;
pub const FUTEX_BITSET_MATCH_ANY: u32 = 0xffffffff;

pub const FUTEX_OP_SET: u32 = 0; /* *(int *)UADDR2 = OPARG; */
pub const FUTEX_OP_ADD: u32 = 1; /* *(int *)UADDR2 += OPARG; */
pub const FUTEX_OP_OR: u32 = 2; /* *(int *)UADDR2 |= OPARG; */
pub const FUTEX_OP_ANDN: u32 = 3; /* *(int *)UADDR2 &= ~OPARG; */
pub const FUTEX_OP_XOR: u32 = 4; /* *(int *)UADDR2 ^= OPARG; */
pub const FUTEX_OP_OPARG_SHIFT: u32 = 8; /* Use (1 << OPARG) instead of OPARG. */
pub const FUTEX_OP_CMP_EQ: u32 = 0;
pub const FUTEX_OP_CMP_NE: u32 = 1;
pub const FUTEX_OP_CMP_LT: u32 = 2;
pub const FUTEX_OP_CMP_LE: u32 = 3;
pub const FUTEX_OP_CMP_GT: u32 = 4;
pub const FUTEX_OP_CMP_GE: u32 = 5;

/* FUTEX_WAKE_OP atomically reads oldval, stores oldval OP OPARG, and wakes
 * UADDR2 if oldval CMP CMPARG. */
#[inline]
pub const fn FUTEX_OP(op: u32, oparg: u32, cmp: u32, cmparg: u32) -> u32 {
    ((op & 0xf) << 28) | ((cmp & 0xf) << 24) | ((oparg & 0xfff) << 12) | (cmparg & 0xfff)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
