/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux memory-management headers.

/* Note that the following two definitions are _highly_ dependent
   on the contexts in which they are used in the kernel.  I personally
   think it is criminal how loosely defined these macros are.  */

/* We need to flush the kernel's icache after loading modules.  The
   only other use of this macro is in load_aout_interp which is not
   used on Alpha.

   Note that this definition should *not* be used for userspace
   icache flushing.  While functional, it is _way_ overkill.  The
   icache is tagged with ASNs and it suffices to allocate a new ASN
   for the process.  */

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline(always)]
pub unsafe fn flush_icache_range(_start: usize, _end: usize) {
    unsafe { imb(); }
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn flush_icache_range(_start: usize, _end: usize) {
    unsafe { smp_imb(); }
}

#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" {
    pub fn smp_imb();
}

/* We need to flush the userspace icache after setting breakpoints in
   ptrace.

   Instead of indiscriminately using imb, take advantage of the fact
   that icache entries are tagged with the ASN and load a new mm context.  */
/* ??? Ought to use this in arch/alpha/kernel/signal.c too.  */

#[cfg(not(feature = "CONFIG_SMP"))]
unsafe extern "C" {
    pub fn __load_new_mm_context(mm: *mut crate::mm_struct);
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_icache_user_page(
    vma: *mut crate::vm_area_struct,
    _page: *mut crate::page,
    _addr: usize,
    _len: i32,
) {
    unsafe {
        if ((*vma).vm_flags & crate::VM_EXEC) != 0 {
            let mm = (*vma).vm_mm;
            if (*crate::current).active_mm == mm {
                __load_new_mm_context(mm);
            } else {
                (*mm).context[crate::smp_processor_id()] = 0;
            }
        }
    }
}

#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" {
    pub fn flush_icache_user_page(
        vma: *mut crate::vm_area_struct,
        page: *mut crate::page,
        addr: usize,
        len: i32,
    );
}

/* Both implementations of flush_icache_user_page flush the entire
 * address space, so one call, no matter how many pages.
 */
#[inline]
pub unsafe fn flush_icache_pages(
    vma: *mut crate::vm_area_struct,
    page: *mut crate::page,
    _nr: u32,
) {
    unsafe { flush_icache_user_page(vma, page, 0, 0); }
}

unsafe extern "C" {
    pub fn imb();
}

// The C header also includes asm-generic/cacheflush.h; its declarations are
// supplied by the corresponding translated dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
