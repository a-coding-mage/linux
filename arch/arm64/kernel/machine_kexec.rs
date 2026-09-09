// SPDX-License-Identifier: GPL-2.0-only
/*
 * kexec for arm64
 *
 * Copyright (C) Linaro.
 * Copyright (C) Huawei Futurewei Technologies.
 */

// Linux and arm64 headers provide the declarations used below.

/// kexec_image_info - For debugging output.
unsafe fn _kexec_image_info(func: *const c_char, line: c_int, kimage: *const kimage) {
    kexec_dprintk!("%s:%d:\n", func, line);
    kexec_dprintk!("  kexec kimage info:\n");
    kexec_dprintk!("    type:        %d\n", (*kimage).type_);
    kexec_dprintk!("    head:        %lx\n", (*kimage).head);
    kexec_dprintk!("    kern_reloc: %pa\n", &(*kimage).arch.kern_reloc);
    kexec_dprintk!("    el2_vectors: %pa\n", &(*kimage).arch.el2_vectors);
}

macro_rules! kexec_image_info {
    ($i:expr) => {
        _kexec_image_info(concat!(module_path!(), "\0").as_ptr() as *const c_char,
                          line!() as c_int, $i)
    };
}

pub unsafe fn machine_kexec_cleanup(kimage: *mut kimage) {
    /* Empty routine needed to avoid build errors. */
}

/// machine_kexec_prepare - Prepare for a kexec reboot.
///
/// Called from the core kexec code when a kernel image is loaded.
/// Forbid loading a kexec kernel if we have no way of hotplugging cpus or cpus
/// are stuck in the kernel. This avoids a panic once we hit machine_kexec().
pub unsafe fn machine_kexec_prepare(kimage: *mut kimage) -> c_int {
    if (*kimage).type_ != KEXEC_TYPE_CRASH && cpus_are_stuck_in_kernel() {
        pr_err!("Can't kexec: CPUs are stuck in the kernel.\n");
        return -EBUSY;
    }
    0
}

/// kexec_segment_flush - Helper to flush the kimage segments to PoC.
unsafe fn kexec_segment_flush(kimage: *const kimage) {
    let mut i: c_ulong = 0;
    pr_debug!("{}:\n", "kexec_segment_flush");
    while i < (*kimage).nr_segments {
        let segment = &(*kimage).segment[i as usize];
        pr_debug!("  segment[%lu]: %016lx - %016lx, 0x%lx bytes, %lu pages\n",
                  i, segment.mem, segment.mem + segment.memsz, segment.memsz,
                  segment.memsz / PAGE_SIZE);
        dcache_clean_inval_poc(phys_to_virt(segment.mem) as c_ulong,
                               phys_to_virt(segment.mem) as c_ulong + segment.memsz);
        i += 1;
    }
}

/* Allocates pages for kexec page table */
unsafe extern "C" fn kexec_page_alloc(arg: *mut c_void) -> *mut c_void {
    let kimage = arg as *mut kimage;
    let page = kimage_alloc_control_pages(kimage, 0);
    if page.is_null() {
        return core::ptr::null_mut();
    }
    let vaddr = page_address(page);
    memset(vaddr, 0, PAGE_SIZE);
    vaddr
}

pub unsafe fn machine_kexec_post_load(kimage: *mut kimage) -> c_int {
    let mut rc: c_int;
    let mut trans_pgd: *mut pgd_t = core::ptr::null_mut();
    let reloc_code = page_to_virt((*kimage).control_code_page);
    let reloc_size: c_long;
    let info = trans_pgd_info {
        trans_alloc_page: Some(kexec_page_alloc),
        trans_alloc_arg: kimage as *mut c_void,
    };

    /* If in place, relocation is not used, only flush next kernel */
    if (*kimage).head & IND_DONE != 0 {
        kexec_segment_flush(kimage);
        kexec_image_info!(kimage);
        return 0;
    }

    (*kimage).arch.el2_vectors = 0;
    if is_hyp_nvhe() {
        rc = trans_pgd_copy_el2_vectors(&info, &mut (*kimage).arch.el2_vectors);
        if rc != 0 { return rc; }
    }

    /* Create a copy of the linear map */
    rc = trans_pgd_create_copy(&info, &mut trans_pgd, PAGE_OFFSET, PAGE_END);
    if rc != 0 { return rc; }
    (*kimage).arch.ttbr1 = __pa(trans_pgd);
    (*kimage).arch.zero_page = __pa_symbol(empty_zero_page);

    reloc_size = __relocate_new_kernel_end - __relocate_new_kernel_start;
    memcpy(reloc_code, __relocate_new_kernel_start, reloc_size as usize);
    (*kimage).arch.kern_reloc = __pa(reloc_code);
    rc = trans_pgd_idmap_page(&info, &mut (*kimage).arch.ttbr0,
                              &mut (*kimage).arch.t0sz, reloc_code);
    if rc != 0 { return rc; }
    (*kimage).arch.phys_offset = virt_to_phys(kimage) as c_long - kimage as c_long;

    /* Flush the reloc_code in preparation for its execution. */
    dcache_clean_inval_poc(reloc_code as c_ulong, reloc_code as c_ulong + reloc_size as c_ulong);
    icache_inval_pou(reloc_code as usize, reloc_code as usize + reloc_size as usize);
    kexec_image_info!(kimage);
    0
}

/// machine_kexec - Do the kexec reboot.
pub unsafe fn machine_kexec(kimage: *mut kimage) {
    let in_kexec_crash = kimage == kexec_crash_image;
    let stuck_cpus = cpus_are_stuck_in_kernel();
    BUG_ON!(!in_kexec_crash && (stuck_cpus || num_online_cpus() > 1));
    WARN!(in_kexec_crash && (stuck_cpus || smp_crash_stop_failed()),
          "Some CPUs may be stale, kdump will be unreliable.\n");
    pr_info!("Bye!\n");
    local_daif_mask();

    if (*kimage).head & IND_DONE != 0 {
        cpu_install_idmap();
        let restart: unsafe extern "C" fn(bool, _, _, _, _) = core::mem::transmute(__pa_symbol(cpu_soft_restart));
        restart(is_hyp_nvhe(), (*kimage).start, (*kimage).arch.dtb_mem, 0, 0);
    } else {
        if is_hyp_nvhe() { __hyp_set_vectors((*kimage).arch.el2_vectors); }
        cpu_install_ttbr0((*kimage).arch.ttbr0, (*kimage).arch.t0sz);
        let kernel_reloc: unsafe extern "C" fn(*mut kimage) = core::mem::transmute((*kimage).arch.kern_reloc);
        kernel_reloc(kimage);
    }
    BUG!(); /* Should never get here. */
}

/// machine_crash_shutdown - shutdown non-crashing cpus and save registers
pub unsafe fn machine_crash_shutdown(regs: *mut pt_regs) {
    local_irq_disable();
    crash_smp_send_stop();
    crash_save_cpu(regs, smp_processor_id());
    machine_kexec_mask_interrupts();
    pr_info!("Starting crashdump kernel...\n");
}

#[cfg(all(CONFIG_CRASH_DUMP, CONFIG_HIBERNATION))]
pub unsafe fn crash_prepare_suspend() {
    if !kexec_crash_image.is_null() { arch_kexec_unprotect_crashkres(); }
}

#[cfg(all(CONFIG_CRASH_DUMP, CONFIG_HIBERNATION))]
pub unsafe fn crash_post_resume() {
    if !kexec_crash_image.is_null() { arch_kexec_protect_crashkres(); }
}

#[cfg(all(CONFIG_CRASH_DUMP, CONFIG_HIBERNATION))]
pub unsafe fn crash_is_nosave(pfn: c_ulong) -> bool {
    let addr: phys_addr_t;
    if crashk_res.end == 0 { return false; }
    addr = __pfn_to_phys(pfn);
    if addr < crashk_res.start || crashk_res.end < addr {
        if crashk_low_res.end == 0 { return false; }
        if addr < crashk_low_res.start || crashk_low_res.end < addr { return false; }
    }
    if kexec_crash_image.is_null() { return true; }
    for i in 0..(*kexec_crash_image).nr_segments as usize {
        let segment = &(*kexec_crash_image).segment[i];
        if addr >= segment.mem && addr < segment.mem + segment.memsz { return false; }
    }
    true
}

#[cfg(all(CONFIG_CRASH_DUMP, CONFIG_HIBERNATION))]
pub unsafe fn crash_free_reserved_phys_range(begin: c_ulong, end: c_ulong) {
    let mut addr = begin;
    while addr < end {
        free_reserved_page(phys_to_page(addr));
        addr += PAGE_SIZE;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
