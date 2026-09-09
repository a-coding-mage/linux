// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 FORTH-ICS/CARV
 *  Nick Kossifidis <mick@ics.forth.gr>
 */

/* Kernel dependencies are supplied by the surrounding translation unit. */

use core::ffi::c_void;

/*
 * machine_kexec_prepare - Initialize kexec
 *
 * This function is called from do_kexec_load, when the user has
 * provided us with an image to be loaded. Its goal is to validate
 * the image and prepare the control code buffer as needed.
 * Note that kimage_alloc_init has already been called and the
 * control buffer has already been allocated.
 */
pub unsafe extern "C" fn machine_kexec_prepare(image: *mut kimage) -> i32 {
    let internal = &mut (*image).arch;
    let mut fdt: fdt_header = core::mem::zeroed();
    let mut control_code_buffer: *mut c_void = core::ptr::null_mut();
    let mut control_code_buffer_sz: u32 = 0;
    let mut i: u32 = 0;

    /* Find the Flattened Device Tree and save its physical address */
    while i < (*image).nr_segments {
        let segment = &(*image).segment.add(i as usize);
        if (*segment).memsz <= core::mem::size_of::<fdt_header>() {
            i += 1;
            continue;
        }

        if (*segment).buf.is_null() {
            i += 1;
            continue;
        }

        if (*image).file_mode {
            core::ptr::copy_nonoverlapping(
                (*segment).buf as *const u8,
                &mut fdt as *mut fdt_header as *mut u8,
                core::mem::size_of::<fdt_header>(),
            );
        } else if copy_from_user(
            &mut fdt as *mut fdt_header as *mut c_void,
            (*segment).buf,
            core::mem::size_of::<fdt_header>(),
        ) != 0 {
            i += 1;
            continue;
        }

        if fdt_check_header(&fdt) != 0 {
            i += 1;
            continue;
        }

        (*internal).fdt_addr = (*segment).mem as usize;
        break;
    }

    if (*internal).fdt_addr == 0 {
        pr_err!("Device tree not included in the provided image\n");
        return -EINVAL;
    }

    /* Copy the assembler code for relocation to the control page */
    if (*image).type_ != KEXEC_TYPE_CRASH {
        control_code_buffer = page_address((*image).control_code_page);
        control_code_buffer_sz = page_size((*image).control_code_page);

        if riscv_kexec_relocate_size > control_code_buffer_sz as usize {
            pr_err!("Relocation code doesn't fit within a control page\n");
            return -EINVAL;
        }

        core::ptr::copy_nonoverlapping(
            riscv_kexec_relocate,
            control_code_buffer as *mut u8,
            riscv_kexec_relocate_size,
        );

        /* Mark the control page executable */
        set_memory_x(control_code_buffer as usize, 1);
    }

    0
}

/*
 * machine_kexec_cleanup - Cleanup any leftovers from
 *                           machine_kexec_prepare
 */
pub unsafe extern "C" fn machine_kexec_cleanup(_image: *mut kimage) {}

/* Prepare for a kexec reboot. */
pub unsafe extern "C" fn machine_shutdown() {
    /* No more interrupts on this hart until we are back up. */
    local_irq_disable();

    // CONFIG_HOTPLUG_CPU conditional from the original implementation.
    #[cfg(CONFIG_HOTPLUG_CPU)]
    smp_shutdown_nonboot_cpus(smp_processor_id());
}

/* Prepare to kexec after a kernel crash. */
pub unsafe extern "C" fn machine_crash_shutdown(regs: *mut pt_regs) {
    local_irq_disable();

    /* shutdown non-crashing cpus */
    crash_smp_send_stop();

    crash_save_cpu(regs, smp_processor_id());
    machine_kexec_mask_interrupts();

    pr_info!("Starting crashdump kernel...\n");
}

/* Jump to the loaded kimage. */
pub unsafe extern "C" fn machine_kexec(image: *mut kimage) -> ! {
    let internal = &mut (*image).arch;
    let jump_addr = (*image).start as usize;
    let first_ind_entry = &(*image).head as *const _ as usize;
    let this_cpu_id = __smp_processor_id();
    let this_hart_id = cpuid_to_hartid_map(this_cpu_id);
    let fdt_addr = (*internal).fdt_addr;
    let control_code_buffer = page_address((*image).control_code_page);
    let mut kexec_method: Option<unsafe extern "C" fn(usize, usize, usize, usize, usize)> = None;

    // CONFIG_SMP conditional from the original implementation.
    #[cfg(CONFIG_SMP)]
    warn!(smp_crash_stop_failed(), "Some CPUs may be stale, kdump will be unreliable.\n");

    if (*image).type_ != KEXEC_TYPE_CRASH {
        kexec_method = Some(core::mem::transmute(control_code_buffer));
    } else {
        kexec_method = Some(riscv_kexec_norelocate);
    }

    pr_notice!("Will call new kernel at %08lx from hart id %lx\n", jump_addr, this_hart_id);
    pr_notice!("FDT image at %08lx\n", fdt_addr);

    /* Make sure the relocation code is visible to the hart */
    local_flush_icache_all();

    /* Jump to the relocation code */
    pr_notice!("Bye...\n");
    (kexec_method.unwrap())(first_ind_entry, jump_addr, fdt_addr, this_hart_id, kernel_map.va_pa_offset);
    core::hint::unreachable_unchecked();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
