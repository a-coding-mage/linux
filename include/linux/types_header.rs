/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/types.h. Types supplied by uapi and other kernel
// headers are intentionally referenced rather than redefined here.

#[macro_export]
macro_rules! DECLARE_BITMAP {
    ($name:ident, $bits:expr) => {
        pub static mut $name: [::core::ffi::c_ulong; BITS_TO_LONGS!($bits)] =
            [0; BITS_TO_LONGS!($bits)];
    };
}

#[cfg(has_int128)]
pub type s128 = i128;
#[cfg(has_int128)]
pub type u128 = u128;

pub type __kernel_dev_t = u32;
pub type fd_set = __kernel_fd_set;
pub type dev_t = __kernel_dev_t;
pub type ino_t = __kernel_ulong_t;
pub type mode_t = __kernel_mode_t;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = __kernel_off_t;
pub type pid_t = __kernel_pid_t;
pub type daddr_t = __kernel_daddr_t;
pub type key_t = __kernel_key_t;
pub type suseconds_t = __kernel_suseconds_t;
pub type timer_t = __kernel_timer_t;
pub type clockid_t = __kernel_clockid_t;
pub type mqd_t = __kernel_mqd_t;
pub type bool = core::ffi::c_int;

pub type uid_t = __kernel_uid32_t;
pub type gid_t = __kernel_gid32_t;
pub type uid16_t = __kernel_uid16_t;
pub type gid16_t = __kernel_gid16_t;
pub type uintptr_t = ::core::ffi::c_ulong;
pub type intptr_t = ::core::ffi::c_long;

#[cfg(CONFIG_HAVE_UID16)]
pub type old_uid_t = __kernel_old_uid_t;
#[cfg(CONFIG_HAVE_UID16)]
pub type old_gid_t = __kernel_old_gid_t;

#[cfg(target_env = "gnu")]
pub type loff_t = __kernel_loff_t;
#[cfg(target_env = "gnu")]
pub type uoff_t = __kernel_uoff_t;

pub type size_t = __kernel_size_t;
pub type ssize_t = __kernel_ssize_t;
pub type ptrdiff_t = __kernel_ptrdiff_t;
pub type clock_t = __kernel_clock_t;
pub type caddr_t = __kernel_caddr_t;

/* bsd */
pub type u_char = u8;
pub type u_short = u16;
pub type u_int = ::core::ffi::c_uint;
pub type u_long = ::core::ffi::c_ulong;

/* sysv */
pub type unchar = u8;
pub type ushort = u16;
pub type uint = ::core::ffi::c_uint;
pub type ulong = ::core::ffi::c_ulong;
pub type ullong = ::core::ffi::c_ulonglong;

pub type u_int8_t = u8;
pub type int8_t = i8;
pub type u_int16_t = u16;
pub type int16_t = i16;
pub type u_int32_t = u32;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type u_int64_t = u64;
pub type int64_t = i64;

pub type aligned_u64 = __aligned_u64;
pub type aligned_s64 = __aligned_s64;
pub type aligned_be64 = __aligned_be64;
pub type aligned_le64 = __aligned_le64;

/* Nanosecond scalar representation for kernel time values */
pub type ktime_t = i64;

pub type sector_t = u64;
pub type blkcnt_t = u64;

/* generic data direction definitions */
pub const READ: i32 = 0;
pub const WRITE: i32 = 1;

pub type pgoff_t = ::core::ffi::c_ulong;

#[cfg(CONFIG_ARCH_DMA_ADDR_T_64BIT)]
pub type dma_addr_t = u64;
#[cfg(not(CONFIG_ARCH_DMA_ADDR_T_64BIT))]
pub type dma_addr_t = u32;

pub type gfp_t = ::core::ffi::c_uint;
pub type slab_flags_t = ::core::ffi::c_uint;
pub type fmode_t = ::core::ffi::c_uint;
pub type blk_mode_t = ::core::ffi::c_uint;
pub type fop_flags_t = ::core::ffi::c_uint;

#[cfg(CONFIG_PHYS_ADDR_T_64BIT)]
pub type phys_addr_t = u64;
#[cfg(not(CONFIG_PHYS_ADDR_T_64BIT))]
pub type phys_addr_t = u32;

#[repr(C)]
pub struct phys_vec {
    pub paddr: phys_addr_t,
    pub len: size_t,
}

pub type resource_size_t = phys_addr_t;
pub type irq_hw_number_t = ::core::ffi::c_ulong;

#[repr(C)]
pub struct atomic_t {
    pub counter: ::core::ffi::c_int,
}

#[macro_export]
macro_rules! ATOMIC_INIT { ($i:expr) => { $i }; }

#[cfg(CONFIG_64BIT)]
#[repr(C)]
pub struct atomic64_t { pub counter: i64 }

#[repr(C)]
pub struct rcuref_t { pub refcnt: atomic_t }

#[macro_export]
macro_rules! RCUREF_INIT { ($i:expr) => { rcuref_t { refcnt: atomic_t { counter: ($i) - 1 } } }; }

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hlist_head { pub first: *mut hlist_node }

#[repr(C)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}

#[repr(C)]
pub struct ustat {
    pub f_tfree: __kernel_daddr_t,
    #[cfg(CONFIG_ARCH_32BIT_USTAT_F_TINODE)]
    pub f_tinode: ::core::ffi::c_uint,
    #[cfg(not(CONFIG_ARCH_32BIT_USTAT_F_TINODE))]
    pub f_tinode: ::core::ffi::c_ulong,
    pub f_fname: [::core::ffi::c_char; 6],
    pub f_fpack: [::core::ffi::c_char; 6],
}

#[repr(C)]
pub struct kcov_common_handle_id {
    #[cfg(CONFIG_KCOV)]
    pub val: u64,
}

#[repr(C, align(8))]
pub struct callback_head {
    pub next: *mut callback_head,
    pub func: Option<unsafe extern "C" fn(head: *mut callback_head)>,
}
pub type rcu_head = callback_head;

#[cfg(CONFIG_KVFREE_RCU_BATCHED)]
#[repr(C)]
pub struct kvfree_rcu_head { pub next: *mut kvfree_rcu_head }
#[cfg(not(CONFIG_KVFREE_RCU_BATCHED))]
#[repr(C)]
pub struct kvfree_rcu_head { pub head: rcu_head }

pub type rcu_callback_t = Option<unsafe extern "C" fn(head: *mut rcu_head)>;
pub type call_rcu_func_t = Option<unsafe extern "C" fn(head: *mut rcu_head, func: rcu_callback_t)>;
pub type swap_r_func_t = Option<unsafe extern "C" fn(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void, size: i32, priv_: *const core::ffi::c_void)>;
pub type swap_func_t = Option<unsafe extern "C" fn(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void, size: i32)>;
pub type cmp_r_func_t = Option<unsafe extern "C" fn(a: *const core::ffi::c_void, b: *const core::ffi::c_void, priv_: *const core::ffi::c_void) -> i32>;
pub type cmp_func_t = Option<unsafe extern "C" fn(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32>;

#[repr(C)]
pub struct rcuwait {
    pub task: *mut task_struct,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
