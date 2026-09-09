/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel and SGX translation units:
// linux/kref.h, linux/mmu_notifier.h, linux/radix-tree.h, linux/rwsem.h,
// linux/sched.h, linux/workqueue.h, uapi/asm/sgx.h, and sgx.h.

pub const SGX_EINIT_SPIN_COUNT: u32 = 20;
pub const SGX_EINIT_SLEEP_COUNT: u32 = 50;
pub const SGX_EINIT_SLEEP_TIME: u32 = 20;

extern "C" {
    pub static mut sgx_attributes_reserved_mask: u64;
    pub static mut sgx_xfrm_reserved_mask: u64;
    pub static mut sgx_misc_reserved_mask: u32;

    pub static sgx_provision_fops: file_operations;

    pub fn sgx_ioctl(filep: *mut file, cmd: u32, arg: c_ulong) -> c_long;
    pub fn sgx_drv_init() -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
