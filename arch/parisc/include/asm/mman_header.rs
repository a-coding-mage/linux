/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// `file`, `MAP_STACK`, and `VM_GROWSUP`.

/* PARISC cannot allow mdwe as it needs writable stacks */
#[inline]
pub const fn arch_memory_deny_write_exec_supported() -> bool {
    false
}

// C macro alias:
// #define arch_memory_deny_write_exec_supported arch_memory_deny_write_exec_supported

#[inline]
pub unsafe fn arch_calc_vm_flag_bits(
    file: *mut file,
    flags: core::ffi::c_ulong,
) -> core::ffi::c_ulong {
    // The stack on parisc grows upwards, so if userspace requests memory
    // for a stack, mark it with VM_GROWSUP so that the stack expansion in
    // the fault handler will work.
    let _ = file;
    if flags & MAP_STACK != 0 {
        return VM_GROWSUP;
    }

    0
}

// C macro alias:
// #define arch_calc_vm_flag_bits(file, flags) arch_calc_vm_flag_bits(file, flags)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
