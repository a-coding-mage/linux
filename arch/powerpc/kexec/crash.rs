// SPDX-License-Identifier: GPL-2.0-only
/* Architecture specific (PPC64) functions for kexec based crash dumps. */

// Dependencies supplied by the surrounding kernel translation.

const PRIMARY_TIMEOUT: u32 = 500;
const SECONDARY_TIMEOUT: i32 = 1000;
const IPI_TIMEOUT: u32 = 10000;
const REAL_MODE_TIMEOUT: u32 = 10000;
const CRASH_HANDLER_MAX: usize = 3;

static mut TIME_TO_DUMP: i32 = 0;
static mut IS_VIA_SYSTEM_RESET: i32 = 0;
pub static mut crash_wake_offline: i32 = 0;

// The following items use kernel types and symbols supplied by other translation units.
extern "C" {
    static mut crash_shutdown_handles: [crash_shutdown_t; CRASH_HANDLER_MAX];
    static mut crash_handlers_lock: spinlock_t;
    static mut crash_shutdown_buf: [c_ulong; JMP_BUF_LEN];
    static mut crash_shutdown_cpu: i32;
    static mut cpus_in_crash: atomic_t;
}

unsafe fn handle_fault(_regs: *mut pt_regs) -> i32 {
    if crash_shutdown_cpu == smp_processor_id() {
        longjmp(crash_shutdown_buf.as_mut_ptr(), 1);
    }
    0
}

#[cfg(feature = "CONFIG_SMP")]
pub unsafe extern "C" fn crash_ipi_callback(regs: *mut pt_regs) {
    static mut CPUS_STATE_SAVED: cpumask_t = CPU_MASK_NONE;
    let cpu = smp_processor_id();
    hard_irq_disable();
    if !cpumask_test_cpu(cpu, &raw mut CPUS_STATE_SAVED) {
        crash_save_cpu(regs, cpu);
        cpumask_set_cpu(cpu, &raw mut CPUS_STATE_SAVED);
    }
    atomic_inc(&raw mut cpus_in_crash);
    smp_mb__after_atomic();
    while TIME_TO_DUMP == 0 { cpu_relax(); }
    if let Some(f) = ppc_md.kexec_cpu_down { f(1, 1); }
    kexec_smp_wait();
}

#[cfg(feature = "CONFIG_SMP")]
unsafe fn crash_kexec_prepare_cpus() {
    let mut ncpus: c_uint = num_online_cpus() - 1;
    let mut tries: i32 = 0;
    printk(KERN_EMERG, c"Sending IPI to other CPUs\0".as_ptr());
    if crash_wake_offline != 0 { ncpus = num_present_cpus() - 1; }
    if IS_VIA_SYSTEM_RESET != 0 { mdelay(PRIMARY_TIMEOUT); }
    else { crash_send_ipi(crash_ipi_callback); }
    smp_wmb();
    'again: loop {
        let mut msecs = IPI_TIMEOUT;
        while atomic_read(&raw mut cpus_in_crash) < ncpus && { msecs -= 1; msecs > 0 } { mdelay(1); }
        if atomic_read(&raw mut cpus_in_crash) >= ncpus { printk(KERN_EMERG, c"IPI complete\0".as_ptr()); return; }
        printk(KERN_EMERG, c"ERROR: cpu(s) not responding\0".as_ptr());
        if panic_timeout > 0 || tries > 0 { return; }
        let old_handler = __debugger;
        __debugger = Some(handle_fault);
        crash_shutdown_cpu = smp_processor_id();
        if setjmp(crash_shutdown_buf.as_mut_ptr()) == 0 {
            printk(KERN_EMERG, c"Activate system reset (dumprestart) to stop other cpu(s)\0".as_ptr());
            atomic_set(&raw mut cpus_in_crash, 0); smp_mb();
            while atomic_read(&raw mut cpus_in_crash) < ncpus { cpu_relax(); }
        }
        crash_shutdown_cpu = -1; __debugger = old_handler; tries += 1;
        continue 'again;
    }
}

#[cfg(feature = "CONFIG_SMP")]
pub unsafe extern "C" fn crash_kexec_secondary(regs: *mut pt_regs) {
    let mut flags = 0; let mut msecs = SECONDARY_TIMEOUT;
    local_irq_save(&mut flags);
    while crashing_cpu < 0 {
        msecs -= 1;
        if msecs < 0 { local_irq_restore(flags); return; }
        mdelay(1);
    }
    crash_ipi_callback(regs);
}

#[cfg(not(feature = "CONFIG_SMP"))]
unsafe fn crash_kexec_prepare_cpus() { smp_release_cpus(); }
#[cfg(not(feature = "CONFIG_SMP"))]
pub unsafe extern "C" fn crash_kexec_secondary(_regs: *mut pt_regs) {}

#[cfg(all(feature = "CONFIG_SMP", feature = "CONFIG_PPC64"))]
unsafe fn crash_kexec_wait_realmode(cpu: i32) {
    let mut msecs = REAL_MODE_TIMEOUT;
    for i in 0..nr_cpu_ids {
        if i == cpu { continue; }
        while (*paca_ptrs[i as usize]).kexec_state < KEXEC_STATE_REAL_MODE {
            barrier();
            if !cpu_possible(i) || !cpu_online(i) || msecs == 0 { break; }
            msecs -= 1; mdelay(1);
        }
    }
    mb();
}
#[cfg(not(all(feature = "CONFIG_SMP", feature = "CONFIG_PPC64")))]
unsafe fn crash_kexec_wait_realmode(_cpu: i32) {}

pub unsafe extern "C" fn crash_kexec_prepare() {
    printk_deferred_enter(); hard_irq_disable();
    crashing_cpu = smp_processor_id(); crash_kexec_prepare_cpus();
}

pub unsafe extern "C" fn crash_shutdown_register(handler: crash_shutdown_t) -> i32 {
    let mut rc = 1; spin_lock(&raw mut crash_handlers_lock);
    for i in 0..CRASH_HANDLER_MAX { if crash_shutdown_handles[i].is_none() { crash_shutdown_handles[i] = handler; rc = 0; break; } }
    spin_unlock(&raw mut crash_handlers_lock); rc
}

pub unsafe extern "C" fn crash_shutdown_unregister(handler: crash_shutdown_t) -> i32 {
    spin_lock(&raw mut crash_handlers_lock);
    let mut i = 0; while i < CRASH_HANDLER_MAX && crash_shutdown_handles[i] != handler { i += 1; }
    if i == CRASH_HANDLER_MAX { spin_unlock(&raw mut crash_handlers_lock); return 1; }
    while i + 1 < CRASH_HANDLER_MAX { crash_shutdown_handles[i] = crash_shutdown_handles[i + 1]; i += 1; }
    crash_shutdown_handles[i] = None; spin_unlock(&raw mut crash_handlers_lock); 0
}

pub unsafe extern "C" fn default_machine_crash_shutdown(regs: *mut pt_regs) {
    if TRAP(regs) == INTERRUPT_SYSTEM_RESET { IS_VIA_SYSTEM_RESET = 1; }
    if IS_ENABLED_CONFIG_SMP() { crash_smp_send_stop(); } else { crash_kexec_prepare(); }
    crash_save_cpu(regs, crashing_cpu); TIME_TO_DUMP = 1; crash_kexec_wait_realmode(crashing_cpu);
    machine_kexec_mask_interrupts();
    let old_handler = __debugger_fault_handler; __debugger_fault_handler = Some(handle_fault); crash_shutdown_cpu = smp_processor_id();
    for i in 0..CRASH_HANDLER_MAX { if crash_shutdown_handles[i].is_none() { break; } if setjmp(crash_shutdown_buf.as_mut_ptr()) == 0 { asm!("sync; isync"); (crash_shutdown_handles[i].unwrap())(); asm!("sync; isync"); } }
    crash_shutdown_cpu = -1; __debugger_fault_handler = old_handler;
    if let Some(f) = ppc_md.kexec_cpu_down { f(1, 0); }
}

#[cfg(feature = "CONFIG_CRASH_DUMP")]
pub unsafe extern "C" fn sync_backup_region_phdr(image: *mut kimage, ehdr: *mut Elf64_Ehdr, phdr_to_kimage: bool) {
    let mut phdr = (ehdr.add(1)) as *mut Elf64_Phdr;
    for _ in 0..(*ehdr).e_phnum {
        if (*phdr).p_paddr == BACKUP_SRC_START {
            if phdr_to_kimage { (*image).arch.backup_start = (*phdr).p_offset; }
            else { (*phdr).p_offset = (*image).arch.backup_start; }
            kexec_dprintk(c"Backup region offset updated\n\0".as_ptr(), (*image).arch.backup_start);
            return;
        }
        phdr = phdr.add(1);
    }
}

#[cfg(feature = "CONFIG_CRASH_HOTPLUG")]
pub unsafe extern "C" fn machine_kexec_post_load(image: *mut kimage) -> i32 {
    if (*image).type_ != KEXEC_TYPE_CRASH || (*image).file_mode { return 0; }
    for i in 0..(*image).nr_segments {
        let mem = (*image).segment[i].mem; let ptr = __va(mem);
        if !ptr.is_null() && memcmp(ptr, ELFMAG.as_ptr(), SELFMAG) == 0 { sync_backup_region_phdr(image, ptr as *mut Elf64_Ehdr, true); }
    }
    0
}

#[cfg(feature = "CONFIG_CRASH_HOTPLUG")]
unsafe fn get_fdt_index(image: *mut kimage) -> i32 {
    for i in 0..(*image).nr_segments {
        let ptr = __va((*image).segment[i].mem);
        if !ptr.is_null() && fdt_magic(ptr) == FDT_MAGIC { return i as i32; }
    }
    -1
}

#[cfg(feature = "CONFIG_CRASH_HOTPLUG")]
unsafe fn update_crash_fdt(image: *mut kimage) {
    let index = get_fdt_index(image); if index < 0 { pr_err(c"Unable to locate FDT segment.\0".as_ptr()); return; }
    let fdt = __va((*image).segment[index as usize].mem); xchg(&raw mut kexec_crash_image, core::ptr::null_mut());
    if update_cpus_node(fdt) != 0 { pr_err(c"Failed to update crash FDT\0".as_ptr()); }
    xchg(&raw mut kexec_crash_image, image);
}

#[cfg(feature = "CONFIG_CRASH_HOTPLUG")]
unsafe fn update_crash_elfcorehdr(image: *mut kimage, mn: *mut memory_notify) {
    let segment = &mut (*image).segment[(*image).elfcorehdr_index];
    let mut cmem: *mut crash_mem = core::ptr::null_mut();
    let mut elfbuf: *mut c_void = core::ptr::null_mut(); let mut elfsz = 0usize;
    if get_crash_memory_ranges(&mut cmem) != 0 { pr_err(c"Failed to get crash mem range\0".as_ptr()); return; }
    if (*image).hp_action == KEXEC_CRASH_HP_REMOVE_MEMORY {
        let base = PFN_PHYS((*mn).start_pfn); let end = base + (*mn).nr_pages * PAGE_SIZE - 1;
        if arch_crash_exclude_mem_range(&mut cmem, base, end) != 0 { kvfree(cmem as *mut c_void); return; }
    }
    if crash_prepare_elf64_headers(cmem, false, &mut elfbuf, &mut elfsz) != 0 || elfsz > segment.memsz { kvfree(cmem as *mut c_void); kvfree(elfbuf); return; }
    sync_backup_region_phdr(image, elfbuf as *mut Elf64_Ehdr, false);
    let ptr = __va(segment.mem);
    if !ptr.is_null() { xchg(&raw mut kexec_crash_image, core::ptr::null_mut()); memcpy(ptr, elfbuf, elfsz); xchg(&raw mut kexec_crash_image, image); }
    kvfree(cmem as *mut c_void); kvfree(elfbuf);
}

#[cfg(feature = "CONFIG_CRASH_HOTPLUG")]
pub unsafe extern "C" fn arch_crash_hotplug_support(image: *mut kimage, kexec_flags: c_ulong) -> i32 {
    if (*image).file_mode { return 1; }
    (kexec_flags & KEXEC_CRASH_HOTPLUG_SUPPORT) as i32
}

#[cfg(feature = "CONFIG_CRASH_HOTPLUG")]
pub unsafe extern "C" fn arch_crash_handle_hotplug_event(image: *mut kimage, arg: *mut c_void) {
    match (*image).hp_action {
        KEXEC_CRASH_HP_REMOVE_CPU => return,
        KEXEC_CRASH_HP_ADD_CPU => update_crash_fdt(image),
        KEXEC_CRASH_HP_REMOVE_MEMORY | KEXEC_CRASH_HP_ADD_MEMORY => {
            update_crash_elfcorehdr(image, arg as *mut memory_notify); return;
        }
        _ => pr_warn_once(c"Unknown hotplug action\n\0".as_ptr()),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
