// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// #include <linux/nsfs.h>    // struct ns_id_req
// #include <linux/types.h>   // __u64
// #include <sys/syscall.h>   // syscall numbers
// #include <unistd.h>        // syscall()

pub type __u64 = u64;
pub type size_t = usize;

// Provided by <linux/nsfs.h> in the original C header.
#[repr(C)]
pub struct ns_id_req {
    _private: [u8; 0],
}

#[cfg(target_arch = "alpha")]
pub const __NR_listns: core::ffi::c_long = 580;

// Original C condition:
// #elif defined _MIPS_SIM
//   #if _MIPS_SIM == _MIPS_SIM_ABI32 /* o32 */
//   #if _MIPS_SIM == _MIPS_SIM_NABI32 /* n32 */
//   #if _MIPS_SIM == _MIPS_SIM_ABI64 /* n64 */
#[cfg(all(target_arch = "mips", target_pointer_width = "32"))]
pub const __NR_listns: core::ffi::c_long = 4470;

#[cfg(all(target_arch = "mips64", target_pointer_width = "64"))]
pub const __NR_listns: core::ffi::c_long = 5470;

#[cfg(not(any(
    target_arch = "alpha",
    all(target_arch = "mips", target_pointer_width = "32"),
    all(target_arch = "mips64", target_pointer_width = "64")
)))]
pub const __NR_listns: core::ffi::c_long = 470;

unsafe extern "C" {
    pub fn syscall(num: core::ffi::c_long, ...) -> core::ffi::c_long;
}

pub unsafe fn sys_listns(
    req: *const ns_id_req,
    ns_ids: *mut __u64,
    nr_ns_ids: size_t,
    flags: core::ffi::c_uint,
) -> core::ffi::c_int {
    unsafe { syscall(__NR_listns, req, ns_ids, nr_ns_ids, flags) as core::ffi::c_int }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
