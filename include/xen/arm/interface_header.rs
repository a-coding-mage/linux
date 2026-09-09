/* SPDX-License-Identifier: GPL-2.0 */
/******************************************************************************
 * Guest OS interface to ARM Xen.
 *
 * Stefano Stabellini <stefano.stabellini@eu.citrix.com>, Citrix, 2012
 */

/* #include <linux/types.h> */

pub type uint64_aligned_t = u64;

#[macro_export]
macro_rules! __DEFINE_GUEST_HANDLE {
    ($name:ident, $ty:ty) => {
        #[repr(C)]
        pub union __guest_handle_$name {
            pub p: *mut $ty,
            pub q: uint64_aligned_t,
        }
    };
}

#[macro_export]
macro_rules! DEFINE_GUEST_HANDLE_STRUCT {
    ($name:ident) => {
        __DEFINE_GUEST_HANDLE!($name, $name);
    };
}

#[macro_export]
macro_rules! DEFINE_GUEST_HANDLE {
    ($name:ident) => {
        __DEFINE_GUEST_HANDLE!($name, $name);
    };
}

#[macro_export]
macro_rules! GUEST_HANDLE {
    ($name:ident) => {
        __guest_handle_$name
    };
}

#[macro_export]
macro_rules! set_xen_guest_handle {
    ($hnd:expr, $val:expr) => {{
        if core::mem::size_of_val(&$hnd) == 8 {
            unsafe {
                *(core::ptr::addr_of_mut!($hnd) as *mut u64) = 0;
            }
        }
        unsafe {
            $hnd.p = $val;
        }
    }};
}

/* __HYPERVISOR_platform_op_raw is an alias for __HYPERVISOR_platform_op. */

/* Explicitly size integers that represent pfns in the interface with
 * Xen so that we can have one ABI that works for 32 and 64 bit guests.
 * Note that this means that the xen_pfn_t type may be capable of
 * representing pfn's which the guest cannot represent in its own pfn
 * type. However since pfn space is controlled by the guest this is
 * fine since it simply wouldn't be able to create any sure pfns in the
 * first place.
 */
pub type xen_pfn_t = u64;
pub const PRI_xen_pfn: &str = "llx";
pub type xen_ulong_t = u64;
pub const PRI_xen_ulong: &str = "llx";
pub type xen_long_t = i64;
pub const PRI_xen_long: &str = "llx";

/* Guest handles for primitive C types. */
__DEFINE_GUEST_HANDLE!(uchar, u8);
__DEFINE_GUEST_HANDLE!(uint, u32);
__DEFINE_GUEST_HANDLE!(char, i8);
__DEFINE_GUEST_HANDLE!(int, i32);
__DEFINE_GUEST_HANDLE!(void, core::ffi::c_void);
__DEFINE_GUEST_HANDLE!(uint64_t, u64);
__DEFINE_GUEST_HANDLE!(uint32_t, u32);
__DEFINE_GUEST_HANDLE!(xen_pfn_t, xen_pfn_t);
__DEFINE_GUEST_HANDLE!(xen_ulong_t, xen_ulong_t);

/* Maximum number of virtual CPUs in multi-processor guests. */
pub const MAX_VIRT_CPUS: u32 = 1;

#[repr(C)]
pub struct arch_vcpu_info {}

#[repr(C)]
pub struct arch_shared_info {}

/* TODO: Move pvclock definitions some place arch independent */
#[repr(C, packed)]
pub struct pvclock_vcpu_time_info {
    pub version: u32,
    pub pad0: u32,
    pub tsc_timestamp: u64,
    pub system_time: u64,
    pub tsc_to_system_mul: u32,
    pub tsc_shift: i8,
    pub flags: u8,
    pub pad: [u8; 2],
} /* 32 bytes */

/* It is OK to have a 12 bytes struct with no padding because it is packed */
#[repr(C, packed)]
pub struct pvclock_wall_clock {
    pub version: u32,
    pub sec: u32,
    pub nsec: u32,
    pub sec_hi: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
