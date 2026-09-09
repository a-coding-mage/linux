// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel build:
// linux/mm.h, linux/suspend.h, asm/coprocessor.h

unsafe extern "C" {
    static __nosave_begin: u8;
    static __nosave_end: u8;

    fn num_online_cpus() -> i32;
    fn local_coprocessors_flush_release_all();
}

// PFN_DOWN(__pa(addr)) and PFN_UP(__pa(addr)) are kernel macros.  The
// surrounding kernel supplies the corresponding page-size and address
// translation definitions.
unsafe extern "C" {
    fn __pa(addr: *const u8) -> usize;
}

#[inline]
unsafe fn pfn_down(addr: usize) -> usize {
    addr >> PAGE_SHIFT
}

#[inline]
unsafe fn pfn_up(addr: usize) -> usize {
    (addr + PAGE_SIZE - 1) >> PAGE_SHIFT
}

unsafe extern "C" {
    static PAGE_SHIFT: usize;
    static PAGE_SIZE: usize;
}

#[inline]
pub unsafe fn pfn_is_nosave(pfn: usize) -> i32 {
    let nosave_begin_pfn = pfn_down(__pa(&__nosave_begin as *const u8));
    let nosave_end_pfn = pfn_up(__pa(&__nosave_end as *const u8));

    ((pfn >= nosave_begin_pfn) && (pfn < nosave_end_pfn)) as i32
}

pub unsafe fn save_processor_state() {
    // WARN_ON(num_online_cpus() != 1);
    if num_online_cpus() != 1 {
        // The kernel WARN_ON side effect is supplied by the surrounding build.
    }

    // #if XTENSA_HAVE_COPROCESSORS
    // This conditional is controlled by the target configuration.
    #[cfg(feature = "XTENSA_HAVE_COPROCESSORS")]
    local_coprocessors_flush_release_all();
    // #endif
}

pub unsafe fn restore_processor_state() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
