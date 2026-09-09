// SPDX-License-Identifier: GPL-2.0
/*
 * machine_kexec.rs - handle transition of Linux booting another kernel
 */

// C header dependencies are supplied by the surrounding kernel translation.

unsafe extern "C" {
    fn relocate_new_kernel();
    static relocate_new_kernel_size: core::ffi::c_uint;
}

static mut waiting_for_crash_ipi: atomic_t = atomic_t::new(0);

/*
 * Provide a dummy crash_notes definition while crash dump arrives to arm.
 * This prevents breakage of crash_notes attribute in kernel/ksysfs.c.
 */

#[no_mangle]
pub unsafe extern "C" fn machine_kexec_prepare(image: *mut kimage) -> core::ffi::c_int {
    let mut current_segment: *mut kexec_segment;
    let mut header: __be32 = 0;
    let mut i: core::ffi::c_int;
    let mut err: core::ffi::c_int;

    (*image).arch.kernel_r2 = (*image).start
        .wrapping_sub(KEXEC_ARM_ZIMAGE_OFFSET)
        .wrapping_add(KEXEC_ARM_ATAGS_OFFSET);

    /*
     * Validate that if the current HW supports SMP, then the SW supports
     * and implements CPU hotplug for the current HW. If not, we won't be
     * able to kexec reliably, so fail the prepare operation.
     */
    if num_possible_cpus() > 1 && platform_can_secondary_boot()
        && !platform_can_cpu_hotplug()
    {
        return -EINVAL;
    }

    /*
     * No segment at default ATAGs address. try to locate
     * a dtb using magic.
     */
    i = 0;
    while i < (*image).nr_segments {
        current_segment = (*image).segment.add(i as usize);

        if !memblock_is_region_memory(
            idmap_to_phys((*current_segment).mem),
            (*current_segment).memsz,
        ) {
            return -EINVAL;
        }

        err = get_user(
            &mut header,
            (*current_segment).buf as *const __be32,
        );
        if err != 0 {
            return err;
        }

        if header == cpu_to_be32(OF_DT_HEADER) {
            (*image).arch.kernel_r2 = (*current_segment).mem;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn machine_kexec_cleanup(_image: *mut kimage) {}

unsafe extern "C" fn machine_crash_nonpanic_core(_unused: *mut core::ffi::c_void) {
    let mut regs: pt_regs = core::mem::zeroed();

    local_fiq_disable();

    crash_setup_regs(&mut regs, get_irq_regs());
    printk!(KERN_DEBUG, "CPU %u will stop doing anything useful since another CPU has crashed\n", smp_processor_id());
    crash_save_cpu(&mut regs, smp_processor_id());
    flush_cache_all();

    set_cpu_online(smp_processor_id(), false);
    atomic_dec(&mut waiting_for_crash_ipi);

    loop {
        cpu_relax();
        wfe();
    }
}

static mut cpu_stop_csd: per_cpu<call_single_data_t> =
    CSD_INIT!(machine_crash_nonpanic_core, core::ptr::null_mut());

#[no_mangle]
pub unsafe extern "C" fn crash_smp_send_stop() {
    static mut cpus_stopped: core::ffi::c_int = 0;
    let mut msecs: core::ffi::c_ulong;
    let mut csd: *mut call_single_data_t;
    let this_cpu: core::ffi::c_int = raw_smp_processor_id();

    if cpus_stopped != 0 {
        return;
    }

    atomic_set(&mut waiting_for_crash_ipi, num_online_cpus() - 1);
    for_each_online_cpu!(cpu, {
        if cpu == this_cpu {
            continue;
        }

        csd = &mut per_cpu!(cpu_stop_csd, cpu);
        smp_call_function_single_async(cpu, csd);
    });

    msecs = 1000; /* Wait at most a second for the other cpus to stop */
    while atomic_read(&waiting_for_crash_ipi) > 0 && msecs != 0 {
        mdelay(1);
        msecs -= 1;
    }
    if atomic_read(&waiting_for_crash_ipi) > 0 {
        pr_warn!("Non-crashing CPUs did not react to IPI\n");
    }

    cpus_stopped = 1;
}

#[no_mangle]
pub unsafe extern "C" fn machine_crash_shutdown(regs: *mut pt_regs) {
    local_irq_disable();
    crash_smp_send_stop();

    crash_save_cpu(regs, smp_processor_id());
    machine_kexec_mask_interrupts();

    pr_info!("Loading crashdump kernel...\n");
}

#[no_mangle]
pub unsafe extern "C" fn machine_kexec(image: *mut kimage) {
    let mut page_list: core::ffi::c_ulong;
    let mut reboot_entry_phys: core::ffi::c_ulong;
    let data: *mut kexec_relocate_data;
    let reboot_entry: Option<unsafe extern "C" fn()>;
    let reboot_code_buffer: *mut core::ffi::c_void;

    /*
     * This can only happen if machine_shutdown() failed to disable some
     * CPU, and that can only happen if the checks in
     * machine_kexec_prepare() were not correct. If this fails, we can't
     * reliably kexec anyway, so BUG_ON is appropriate.
     */
    BUG_ON!(num_online_cpus() > 1);

    page_list = (*image).head & PAGE_MASK;

    reboot_code_buffer = page_address((*image).control_code_page);

    /* copy our kernel relocation code to the control code page */
    reboot_entry = fncpy(
        reboot_code_buffer,
        relocate_new_kernel as *const core::ffi::c_void,
        relocate_new_kernel_size,
    );

    data = reboot_code_buffer.add(relocate_new_kernel_size as usize)
        as *mut kexec_relocate_data;
    (*data).kexec_start_address = (*image).start;
    (*data).kexec_indirection_page = page_list;
    (*data).kexec_mach_type = machine_arch_type;
    (*data).kexec_r2 = (*image).arch.kernel_r2;

    /* get the identity mapping physical address for the reboot code */
    reboot_entry_phys = virt_to_idmap(reboot_entry);

    pr_info!("Bye!\n");

    soft_restart(reboot_entry_phys);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
