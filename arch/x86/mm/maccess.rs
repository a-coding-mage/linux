// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation unit:
// linux/uaccess.h, linux/kernel.h, asm/vsyscall.h

#[cfg(target_arch = "x86_64")]
pub unsafe fn copy_from_kernel_nofault_allowed(
    unsafe_src: *const core::ffi::c_void,
    size: usize,
) -> bool {
    let vaddr = unsafe_src as usize;

    /*
     * Do not allow userspace addresses.  This disallows
     * normal userspace and the userspace guard page:
     */
    if vaddr < TASK_SIZE_MAX + PAGE_SIZE {
        return false;
    }

    /*
     * Reading from the vsyscall page may cause an unhandled fault in
     * certain cases.  Though it is at an address above TASK_SIZE_MAX, it is
     * usually considered as a user space address.
     */
    if is_vsyscall_vaddr(vaddr) {
        return false;
    }

    /*
     * Allow everything during early boot before 'x86_virt_bits'
     * is initialized.  Needed for instruction decoding in early
     * exception handlers.
     */
    if boot_cpu_data.x86_virt_bits == 0 {
        return true;
    }

    __is_canonical_address(vaddr, boot_cpu_data.x86_virt_bits)
}

#[cfg(not(target_arch = "x86_64"))]
pub unsafe fn copy_from_kernel_nofault_allowed(
    unsafe_src: *const core::ffi::c_void,
    size: usize,
) -> bool {
    unsafe_src as usize >= TASK_SIZE_MAX
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
