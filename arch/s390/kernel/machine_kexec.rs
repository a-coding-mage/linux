// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2005, 2011
 *
 * Author(s): Rolf Adelsberger,
 *           Michael Holzheu <holzheu@linux.vnet.ibm.com>
 */

type RelocateKernelT = unsafe extern "C" fn(usize, usize, usize);
type PurgatoryT = unsafe extern "C" fn(i32) -> i32;

unsafe extern "C" {
    static relocate_kernel: u8;
    static relocate_kernel_len: u64;
}

#[cfg(feature = "CONFIG_CRASH_DUMP")]
unsafe fn __do_machine_kdump(data: *mut core::ffi::c_void) {
    let image = data as *mut kimage;
    let purgatory = (*image).start as PurgatoryT;
    // store_status() saved the prefix register to lowcore
    let prefix = (*get_lowcore()).prefixreg_save_area as usize;

    // Now do the reset
    s390_reset_system();

    // Copy dump CPU store status info to absolute zero.
    // This need to be done *after* s390_reset_system set the
    // prefix register of this CPU to zero
    core::ptr::copy_nonoverlapping(
        phys_to_virt(prefix.wrapping_add(__LC_FPREGS_SAVE_AREA)) as *const u8,
        absolute_pointer((*get_lowcore()).floating_pt_save_area) as *mut u8,
        512,
    );

    call_nodat_1(purgatory, 1);

    // Die if kdump returns
    disabled_wait();
}

#[cfg(feature = "CONFIG_CRASH_DUMP")]
unsafe fn __machine_kdump(image: *mut core::ffi::c_void) {
    let mut mcesa: *mut mcesa;
    let mut cr2_old: ctlreg2;
    let mut cr2_new: ctlreg2;
    let this_cpu: i32;

    lgr_info_log();
    // Get status of the other CPUs
    this_cpu = smp_find_processor_id(stap());
    for_each_online_cpu!(cpu => {
        if cpu == this_cpu { return; }
        if smp_store_status(cpu) { return; }
    });
    // Store status of the boot CPU
    mcesa = __va((*get_lowcore()).mcesad & MCESA_ORIGIN_MASK) as *mut mcesa;
    if cpu_has_vx() {
        save_vx_regs((*mcesa).vector_save_area as *mut vector128);
    }
    if cpu_has_gs() {
        local_ctl_store(2, &mut cr2_old.reg);
        cr2_new = cr2_old;
        cr2_new.gse = 1;
        local_ctl_load(2, &cr2_new.reg);
        save_gs_cb((*mcesa).guarded_storage_save_area as *mut gs_cb);
        local_ctl_load(2, &cr2_old.reg);
    }
    /*
     * To create a good backchain for this CPU in the dump store_status
     * is passed the address of a function. The address is saved into the
     * PSW save area of the boot CPU and the function is invoked as a
     * tail call of store_status. The backchain in the dump will look
     * like this:
     *   restart_int_handler ->  __machine_kexec -> __do_machine_kdump
     * The call to store_status() will not return.
     */
    store_status(__do_machine_kdump, image);
}

/* Check if kdump checksums are valid: We call purgatory with parameter "0" */
unsafe fn kdump_csum_valid(image: *mut kimage) -> bool {
    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    {
        let purgatory = (*image).start as PurgatoryT;
        return call_nodat_1(purgatory, 0) == 0;
    }
    #[cfg(not(feature = "CONFIG_CRASH_DUMP"))]
    { let _ = image; false }
}

#[cfg(feature = "CONFIG_CRASH_DUMP")]
unsafe fn crash_free_reserved_phys_range(begin: usize, end: usize) {
    let mut addr = begin;
    while addr < end {
        free_reserved_page(pfn_to_page(addr >> PAGE_SHIFT));
        addr = addr.wrapping_add(PAGE_SIZE);
    }
    let size = begin.wrapping_sub(crashk_res.start);
    if size != 0 { os_info_crashkernel_add(crashk_res.start, size); }
    else { os_info_crashkernel_add(0, 0); }
}

#[cfg(feature = "CONFIG_CRASH_DUMP")]
unsafe fn crash_protect_pages(protect: i32) {
    if crashk_res.end == 0 { return; }
    let size = resource_size(&crashk_res);
    if protect != 0 { set_memory_ro(crashk_res.start, size >> PAGE_SHIFT); }
    else { set_memory_rw(crashk_res.start, size >> PAGE_SHIFT); }
}

#[cfg(feature = "CONFIG_CRASH_DUMP")]
unsafe fn arch_kexec_protect_crashkres() { crash_protect_pages(1); }
#[cfg(feature = "CONFIG_CRASH_DUMP")]
unsafe fn arch_kexec_unprotect_crashkres() { crash_protect_pages(0); }

unsafe fn machine_kexec_prepare_kdump() -> i32 {
    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    {
        if machine_is_vm() {
            diag10_range(PFN_DOWN!(crashk_res.start), PFN_DOWN!(crashk_res.end - crashk_res.start + 1));
        }
        return 0;
    }
    #[cfg(not(feature = "CONFIG_CRASH_DUMP"))]
    { -EINVAL }
}

unsafe fn machine_kexec_prepare(image: *mut kimage) -> i32 {
    if (*image).type_ == KEXEC_TYPE_CRASH { return machine_kexec_prepare_kdump(); }
    if (*image).type_ != KEXEC_TYPE_DEFAULT { return -EINVAL; }
    let reboot_code_buffer = page_to_virt((*image).control_code_page) as *mut u8;
    core::ptr::copy_nonoverlapping(&relocate_kernel, reboot_code_buffer, relocate_kernel_len as usize);
    0
}

unsafe fn machine_kexec_cleanup(_image: *mut kimage) {}
unsafe fn machine_shutdown() {}
unsafe fn machine_crash_shutdown(_regs: *mut pt_regs) { set_os_info_reipl_block(); }

unsafe fn __do_machine_kexec(data: *mut core::ffi::c_void) {
    let image = data as *mut kimage;
    let data_mover = page_to_phys((*image).control_code_page);
    let entry = virt_to_phys(&(*image).head as *const _);
    let mut diag308_subcode = DIAG308_CLEAR_RESET;
    if sclp.has_iplcc { diag308_subcode |= DIAG308_FLAG_EI; }
    s390_reset_system();
    call_nodat_3(data_mover as RelocateKernelT, entry, (*image).start, diag308_subcode);
    // Die if kexec returns
    disabled_wait();
}

unsafe fn __machine_kexec(data: *mut core::ffi::c_void) {
    pfault_fini();
    tracing_off();
    debug_locks_off();
    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    if (*(data as *mut kimage)).type_ == KEXEC_TYPE_CRASH { __machine_kdump(data); }
    __do_machine_kexec(data);
}

unsafe fn machine_kexec(image: *mut kimage) {
    if (*image).type_ == KEXEC_TYPE_CRASH && !kdump_csum_valid(image) { return; }
    tracer_disable();
    smp_send_stop();
    smp_call_ipl_cpu(__machine_kexec, image as *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
