// SPDX-License-Identifier: GPL-2.0
/*
 * Author: Andrei Vagin <avagin@openvz.org>
 * Author: Dmitry Safonov <dima@arista.com>
 */

// Dependencies are supplied by the surrounding kernel translation unit.

unsafe fn offset_from_ts(off: timespec64) -> timens_offset {
    let mut ret: timens_offset = core::mem::zeroed();

    ret.sec = off.tv_sec;
    ret.nsec = off.tv_nsec;

    ret
}

/*
 * A time namespace VVAR page has the same layout as the VVAR page which
 * contains the system wide VDSO data.
 *
 * For a normal task the VVAR pages are installed in the normal ordering:
 *     VVAR
 *     PVCLOCK
 *     HVCLOCK
 *     TIMENS   <- Not really required
 *
 * Now for a timens task the pages are installed in the following order:
 *     TIMENS
 *     PVCLOCK
 *     HVCLOCK
 *     VVAR
 *
 * The check for vdso_clock->clock_mode is in the unlikely path of
 * the seq begin magic. So for the non-timens case most of the time
 * 'seq' is even, so the branch is not taken.
 *
 * If 'seq' is odd, i.e. a concurrent update is in progress, the extra check
 * for vdso_clock->clock_mode is a non-issue. The task is spin waiting for the
 * update to finish and for 'seq' to become even anyway.
 *
 * Timens page has vdso_clock->clock_mode set to VDSO_CLOCKMODE_TIMENS which
 * enforces the time namespace handling path.
 */
unsafe fn timens_setup_vdso_clock_data(vc: *mut vdso_clock, ns: *mut time_namespace) {
    let offset = (*vc).offset;
    let monotonic = offset_from_ts((*ns).offsets.monotonic);
    let boottime = offset_from_ts((*ns).offsets.boottime);

    (*vc).seq = 1;
    (*vc).clock_mode = VDSO_CLOCKMODE_TIMENS;
    *offset.add(CLOCK_MONOTONIC as usize) = monotonic;
    *offset.add(CLOCK_MONOTONIC_RAW as usize) = monotonic;
    *offset.add(CLOCK_MONOTONIC_COARSE as usize) = monotonic;
    *offset.add(CLOCK_BOOTTIME as usize) = boottime;
    *offset.add(CLOCK_BOOTTIME_ALARM as usize) = boottime;
}

pub unsafe fn find_timens_vvar_page(vma: *mut vm_area_struct) -> *mut page {
    if likely((*vma).vm_mm == (*current).mm) {
        return (*(*current).nsproxy).time_ns.vvar_page;
    }

    /*
     * vvar_fault() protects this from being called through remote interfaces like
     * /proc/$pid/mem or process_vm_{readv,writev}().
     */
    WARN(1, "vvar_page accessed remotely");

    core::ptr::null_mut()
}

unsafe fn timens_set_vvar_page(task: *mut task_struct, ns: *mut time_namespace) {
    let mut vdata: *mut vdso_time_data;
    let mut vc: *mut vdso_clock;
    let mut i: u32;

    if ns == &raw mut init_time_ns {
        return;
    }

    /* Fast-path, taken by every task in namespace except the first. */
    if likely((*ns).frozen_offsets) {
        return;
    }

    // guard(mutex)(&timens_offset_lock);
    /* Nothing to-do: vvar_page has been already initialized. */
    if (*ns).frozen_offsets {
        return;
    }

    (*ns).frozen_offsets = true;
    vdata = page_address((*ns).vvar_page);
    vc = (*vdata).clock_data;

    i = 0;
    while i < CS_BASES {
        timens_setup_vdso_clock_data(vc.add(i as usize), ns);
        i += 1;
    }

    // CONFIG_POSIX_AUX_CLOCKS controls this block at build time.
    if IS_ENABLED(CONFIG_POSIX_AUX_CLOCKS) {
        i = 0;
        while i < core::mem::size_of_val(&(*vdata).aux_clock_data) as u32 {
            timens_setup_vdso_clock_data((*vdata).aux_clock_data.as_mut_ptr().add(i as usize), ns);
            i += 1;
        }
    }
}

/*
 * The vvar page layout depends on whether a task belongs to the root or
 * non-root time namespace. Whenever a task changes its namespace, the VVAR
 * page tables are cleared and then they will be re-faulted with a
 * corresponding layout.
 * See also the comment near timens_setup_vdso_clock_data() for details.
 */
unsafe fn vdso_join_timens(task: *mut task_struct, ns: *mut time_namespace) -> i32 {
    let mm = (*task).mm;
    let mut vma: *mut vm_area_struct;
    // VMA_ITERATOR(vmi, mm, 0);

    // guard(mmap_read_lock)(mm);
    // for_each_vma(vmi, vma) {
    vma = core::ptr::null_mut();
    while !vma.is_null() {
        if vma_is_special_mapping(vma, &raw const vdso_vvar_mapping) {
            zap_vma(vma);
        }
        // Advance vma using the surrounding kernel VMA iterator.
        break;
    }
    0
}

pub unsafe fn timens_commit(tsk: *mut task_struct, ns: *mut time_namespace) {
    timens_set_vvar_page(tsk, ns);
    vdso_join_timens(tsk, ns);
}

pub unsafe fn timens_vdso_alloc_vvar_page(ns: *mut time_namespace) -> i32 {
    (*ns).vvar_page = alloc_page(GFP_KERNEL_ACCOUNT | __GFP_ZERO);
    if (*ns).vvar_page.is_null() {
        return -ENOMEM;
    }

    0
}

pub unsafe fn timens_vdso_free_vvar_page(ns: *mut time_namespace) {
    __free_page((*ns).vvar_page);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
