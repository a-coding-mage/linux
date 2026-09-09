/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/refcount.h and asm/processor.h

#[repr(C)]
pub struct io_bitmap {
    pub sequence: u64,
    pub refcnt: refcount_t,
    /* The maximum number of bytes to copy so all zero bits are covered */
    pub max: ::core::ffi::c_uint,
    pub bitmap: [::core::ffi::c_ulong; IO_BITMAP_LONGS],
}

pub struct task_struct;

#[cfg(CONFIG_X86_IOPL_IOPERM)]
extern "C" {
    pub fn io_bitmap_share(tsk: *mut task_struct);
    pub fn io_bitmap_exit(tsk: *mut task_struct);
}

#[cfg(CONFIG_X86_IOPL_IOPERM)]
#[inline]
pub unsafe fn native_tss_invalidate_io_bitmap() {
    /*
     * Invalidate the I/O bitmap by moving io_bitmap_base outside the
     * TSS limit so any subsequent I/O access from user space will
     * trigger a #GP.
     *
     * This is correct even when VMEXIT rewrites the TSS limit
     * to 0x67 as the only requirement is that the base points
     * outside the limit.
     */
    this_cpu_write(
        cpu_tss_rw.x86_tss.io_bitmap_base,
        IO_BITMAP_OFFSET_INVALID,
    );
}

#[cfg(CONFIG_X86_IOPL_IOPERM)]
extern "C" {
    pub fn native_tss_update_io_bitmap();
}

#[cfg(all(CONFIG_X86_IOPL_IOPERM, CONFIG_PARAVIRT_XXL))]
// Dependency supplied by asm/paravirt.h.
extern "C" {
    pub fn tss_update_io_bitmap();
    pub fn tss_invalidate_io_bitmap();
}

#[cfg(all(CONFIG_X86_IOPL_IOPERM, not(CONFIG_PARAVIRT_XXL)))]
#[inline]
pub unsafe fn tss_update_io_bitmap() {
    native_tss_update_io_bitmap();
}

#[cfg(all(CONFIG_X86_IOPL_IOPERM, not(CONFIG_PARAVIRT_XXL)))]
#[inline]
pub unsafe fn tss_invalidate_io_bitmap() {
    native_tss_invalidate_io_bitmap();
}

#[cfg(not(CONFIG_X86_IOPL_IOPERM))]
#[inline]
pub fn io_bitmap_share(_tsk: *mut task_struct) {}

#[cfg(not(CONFIG_X86_IOPL_IOPERM))]
#[inline]
pub fn io_bitmap_exit(_tsk: *mut task_struct) {}

#[cfg(not(CONFIG_X86_IOPL_IOPERM))]
#[inline]
pub fn tss_update_io_bitmap() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
