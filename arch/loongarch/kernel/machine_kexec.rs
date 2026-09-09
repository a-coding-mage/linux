// SPDX-License-Identifier: GPL-2.0-only
/*
 * machine_kexec.c for kexec
 *
 * Copyright (C) 2022 Loongson Technology Corporation Limited
 */

const KEXEC_CONTROL_CODE: usize = TO_CACHE(0x100000);
const KEXEC_CMDLINE_ADDR: usize = TO_CACHE(0x108000);

static mut reboot_code_buffer: c_ulong = 0;
static mut cpus_in_crash: cpumask_t = CPU_MASK_NONE;

#[cfg(CONFIG_SMP)]
static mut relocated_kexec_smp_wait: Option<unsafe extern "C" fn(*mut c_void)> = None;
#[cfg(CONFIG_SMP)]
static mut kexec_ready_to_reboot: atomic_t = ATOMIC_INIT(0);

static mut efi_boot: c_ulong = 0;
static mut cmdline_ptr: c_ulong = 0;
static mut systable_ptr: c_ulong = 0;
static mut start_addr: c_ulong = 0;
static mut first_ind_entry: c_ulong = 0;

pub unsafe fn machine_kexec_prepare(kimage: *mut kimage) -> c_int {
    let mut head = [0i8; 8];
    let bootloader = b"kexec\0".as_ptr() as *mut c_char;
    let cmdline_ptr_local = KEXEC_CMDLINE_ADDR as *mut c_void;

    (*kimage).arch.efi_boot = fw_arg0;
    (*kimage).arch.systable_ptr = fw_arg2;

    if (*kimage).file_mode == 1 {
        /*
         * kimage->cmdline_buf will be released in kexec_file_load, so copy
         * to the KEXEC_CMDLINE_ADDR safe area.
         */
        memcpy(
            KEXEC_CMDLINE_ADDR as *mut c_void,
            (*kimage).arch.cmdline_ptr as *const c_void,
            strlen((*kimage).arch.cmdline_ptr as *const c_char) + 1,
        );
        (*kimage).arch.cmdline_ptr = KEXEC_CMDLINE_ADDR as c_ulong;
    } else {
        /* Find the command line */
        for i in 0..(*kimage).nr_segments {
            let segment = (*kimage).segment.add(i as usize);
            if copy_from_user(
                head.as_mut_ptr() as *mut c_void,
                (*segment).buf,
                strlen(bootloader),
            ) != 0 {
                continue;
            }
            if strncmp(bootloader, head.as_ptr(), strlen(bootloader)) == 0 {
                if copy_from_user(cmdline_ptr_local, (*segment).buf, COMMAND_LINE_SIZE) == 0 {
                    (*kimage).arch.cmdline_ptr = cmdline_ptr_local as c_ulong;
                }
                break;
            }
        }

        if (*kimage).arch.cmdline_ptr == 0 {
            pr_err!("Command line not included in the provided image\n");
            return -EINVAL;
        }
    }

    /* kexec/kdump need a safe page to save reboot_code_buffer */
    (*kimage).control_code_page = virt_to_page(KEXEC_CONTROL_CODE as *mut c_void);

    reboot_code_buffer = page_address((*kimage).control_code_page) as c_ulong;
    memcpy(
        reboot_code_buffer as *mut c_void,
        relocate_new_kernel as *const c_void,
        relocate_new_kernel_size,
    );

    #[cfg(CONFIG_SMP)]
    {
        /* All secondary cpus now may jump to kexec_smp_wait cycle */
        relocated_kexec_smp_wait = Some(core::mem::transmute(
            reboot_code_buffer.wrapping_add(
                (kexec_smp_wait as usize).wrapping_sub(relocate_new_kernel as usize),
            ),
        ));
    }

    0
}

pub unsafe fn machine_kexec_cleanup(_kimage: *mut kimage) {}

pub unsafe fn kexec_reboot() {
    let mut do_kexec: Option<unsafe extern "C" fn(c_ulong, c_ulong, c_ulong, c_ulong, c_ulong)> = None;

    /*
     * We know we were online, and there will be no incoming IPIs at
     * this point. Mark online again before rebooting so that the crash
     * analysis tool will see us correctly.
     */
    set_cpu_online(smp_processor_id(), true);
    smp_mb__after_atomic();
    core::arch::asm!("ibar 0");

    #[cfg(CONFIG_SMP)]
    if smp_processor_id() > 0 {
        if let Some(wait) = relocated_kexec_smp_wait { wait(core::ptr::null_mut()); }
        BUG!();
    }

    do_kexec = Some(core::mem::transmute(reboot_code_buffer as usize));
    do_kexec.unwrap()(efi_boot, cmdline_ptr, systable_ptr, start_addr, first_ind_entry);
    BUG!();
}

#[cfg(CONFIG_SMP)]
unsafe fn kexec_shutdown_secondary(_regs: *mut c_void) {
    let cpu = smp_processor_id();
    if !cpu_online(cpu) { return; }
    set_cpu_online(cpu, false);
    local_irq_disable();
    while atomic_read(&kexec_ready_to_reboot) == 0 { cpu_relax(); }
    kexec_reboot();
}

#[cfg(CONFIG_SMP)]
unsafe fn crash_shutdown_secondary(passed_regs: *mut c_void) {
    let cpu = smp_processor_id();
    let mut regs = passed_regs as *mut pt_regs;
    if regs.is_null() { regs = get_irq_regs(); }
    if regs.is_null() { regs = task_pt_regs(current); }
    if !cpu_online(cpu) { return; }
    set_cpu_online(cpu, false);
    local_irq_disable();
    if !cpumask_test_cpu(cpu, &cpus_in_crash) { crash_save_cpu(regs, cpu); }
    cpumask_set_cpu(cpu, &mut cpus_in_crash);
    while atomic_read(&kexec_ready_to_reboot) == 0 { cpu_relax(); }
    kexec_reboot();
}

#[cfg(CONFIG_SMP)]
pub unsafe fn crash_smp_send_stop() {
    let ncpus: c_uint;
    let mut timeout: c_ulong;
    static mut cpus_stopped: c_int = 0;
    if cpus_stopped != 0 { return; }
    cpus_stopped = 1;
    ncpus = num_online_cpus() - 1;
    smp_call_function(crash_shutdown_secondary, core::ptr::null_mut(), 0);
    smp_wmb();
    timeout = MSEC_PER_SEC * 10;
    pr_emerg!("Sending IPI to other cpus...\n");
    while cpumask_weight(&cpus_in_crash) < ncpus && timeout != 0 {
        mdelay(1);
        cpu_relax();
        timeout -= 1;
    }
}

pub unsafe fn machine_shutdown() {
    #[cfg(CONFIG_SMP)]
    {
        for_each_possible_cpu!(cpu, {
            if !cpu_online(cpu) { cpu_device_up(get_cpu_device(cpu)); }
        });
        smp_call_function(kexec_shutdown_secondary, core::ptr::null_mut(), 0);
    }
}

pub unsafe fn machine_crash_shutdown(regs: *mut pt_regs) {
    local_irq_disable();
    let crashing_cpu = smp_processor_id();
    crash_save_cpu(regs, crashing_cpu);
    #[cfg(CONFIG_SMP)]
    crash_smp_send_stop();
    machine_kexec_mask_interrupts();
    cpumask_set_cpu(crashing_cpu, &mut cpus_in_crash);
    pr_info!("Starting crashdump kernel...\n");
}

pub unsafe fn machine_kexec(image: *mut kimage) {
    let mut ptr: *mut c_ulong;
    let mut entry: c_ulong;
    let internal = &mut (*image).arch;
    efi_boot = internal.efi_boot;
    cmdline_ptr = internal.cmdline_ptr;
    systable_ptr = internal.systable_ptr;
    start_addr = phys_to_virt((*image).start) as c_ulong;
    first_ind_entry = if (*image).type_ == KEXEC_TYPE_DEFAULT {
        phys_to_virt((*image).head & PAGE_MASK) as c_ulong
    } else { 0 };

    /* The generic kexec code builds a page list with physical addresses. */
    ptr = &mut (*image).head;
    loop {
        entry = *ptr;
        if entry == 0 || entry & IND_DONE != 0 { break; }
        if entry & IND_SOURCE != 0 || entry & IND_INDIRECTION != 0 || entry & IND_DESTINATION != 0 {
            *ptr = phys_to_virt(entry) as c_ulong;
        }
        ptr = if entry & IND_INDIRECTION != 0 {
            phys_to_virt(entry & PAGE_MASK) as *mut c_ulong
        } else { ptr.add(1) };
    }

    set_cpu_online(smp_processor_id(), false);
    local_irq_disable();
    machine_kexec_mask_interrupts();
    pr_notice!("EFI boot flag: 0x{:lx}\n", efi_boot);
    pr_notice!("Command line addr: 0x{:lx}\n", cmdline_ptr);
    pr_notice!("Command line string: %s\n", cmdline_ptr as *const c_char);
    pr_notice!("System table addr: 0x{:lx}\n", systable_ptr);
    pr_notice!("We will call new kernel at 0x{:lx}\n", start_addr);
    pr_notice!("Bye ...\n");
    flush_cache_all();
    #[cfg(CONFIG_SMP)]
    atomic_set(&mut kexec_ready_to_reboot, 1);
    kexec_reboot();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
