/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// asm/lowcore.h, asm/page.h, linux/kmsan.h, linux/mmzone.h, linux/stddef.h

#[cfg(not(feature = "module"))]
pub unsafe fn is_lowcore_addr(addr: *mut core::ffi::c_void) -> bool {
    let lowcore = get_lowcore() as *mut core::ffi::c_void;
    (addr as usize) >= (lowcore as usize)
        && (addr as usize) < (lowcore as usize + core::mem::size_of_val(&*get_lowcore()))
}

#[cfg(not(feature = "module"))]
pub unsafe fn arch_kmsan_get_meta_or_null(
    mut addr: *mut core::ffi::c_void,
    is_origin: bool,
) -> *mut core::ffi::c_void {
    if is_lowcore_addr(addr) {
        /*
         * Different lowcores accessed via S390_lowcore are described
         * by the same struct page. Resolve the prefix manually in
         * order to get a distinct struct page.
         */
        addr = (addr as usize
            + lowcore_ptr[raw_smp_processor_id()] as usize
            - get_lowcore() as usize) as *mut core::ffi::c_void;
        if KMSAN_WARN_ON(is_lowcore_addr(addr)) {
            return core::ptr::null_mut();
        }
        return kmsan_get_metadata(addr, is_origin);
    }
    core::ptr::null_mut()
}

#[cfg(not(feature = "module"))]
pub unsafe fn kmsan_virt_addr_valid(addr: *mut core::ffi::c_void) -> bool {
    let ret: bool;

    /*
     * pfn_valid() relies on RCU, and may call into the scheduler on exiting
     * the critical section. However, this would result in recursion with
     * KMSAN. Therefore, disable preemption here, and re-enable preemption
     * below while suppressing reschedules to avoid recursion.
     *
     * Note, this sacrifices occasionally breaking scheduling guarantees.
     * Although, a kernel compiled with KMSAN has already given up on any
     * performance guarantees due to being heavily instrumented.
     */
    preempt_disable();
    ret = virt_addr_valid(addr);
    preempt_enable_no_resched();

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
