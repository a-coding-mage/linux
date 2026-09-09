/* Direct Rust translation of smp.c. */

use core::ffi::c_void;

extern "C" {
    static mut octeon_processor_boot: c_ulong;
    static mut octeon_processor_sp: c_ulong;
    static mut octeon_processor_gp: c_ulong;
    fn kernel_entry(arg1: c_ulong, ...);
}

type c_ulong = usize;
type irqreturn_t = i32;
type u64_ = u64;

pub static mut OCTEON_PROCESSOR_BOOT: c_ulong = 0xff;
pub static mut OCTEON_PROCESSOR_SP: c_ulong = 0;
pub static mut OCTEON_PROCESSOR_GP: c_ulong = 0;

#[cfg(feature = "CONFIG_RELOCATABLE")]
pub static mut octeon_processor_relocated_kernel_entry: c_ulong = 0;

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
pub static mut octeon_bootloader_entry_addr: u64 = 0;

extern "C" {
    fn scheduler_ipi();
    fn generic_smp_call_function_interrupt();
    fn cvmx_get_core_num() -> i32;
    fn cvmx_read_csr(addr: u64) -> u64;
    fn cvmx_write_csr(addr: u64, value: u64);
    fn cpu_logical_map(cpu: i32) -> i32;
    fn octeon_ciu3_mbox_send(cpu: i32, box_: i32);
}

static mut octeon_message_functions: [Option<unsafe extern "C" fn()>; 8] = [
    Some(scheduler_ipi), Some(generic_smp_call_function_interrupt),
    Some(octeon_icache_flush), None, None, None, None, None,
];

unsafe extern "C" fn octeon_icache_flush() {
    core::arch::asm!("synci 0($0)");
}

unsafe extern "C" fn mailbox_interrupt(_irq: i32, _dev_id: *mut c_void) -> irqreturn_t {
    let mbox_clrx = CVMX_CIU_MBOX_CLRX(cvmx_get_core_num());
    let mut action = cvmx_read_csr(mbox_clrx);
    if OCTEON_IS_MODEL(OCTEON_CN68XX) { action &= 0xff; } else { action &= 0xffff; }
    cvmx_write_csr(mbox_clrx, action);
    let mut i = 0usize;
    while i < octeon_message_functions.len() && action != 0 {
        if action & 1 != 0 {
            if let Some(f) = octeon_message_functions[i] { f(); }
        }
        action >>= 1;
        i += 1;
    }
    IRQ_HANDLED
}

pub unsafe extern "C" fn octeon_send_ipi_single(cpu: i32, action: u32) {
    let coreid = cpu_logical_map(cpu);
    cvmx_write_csr(CVMX_CIU_MBOX_SETX(coreid), action as u64);
}

unsafe fn octeon_send_ipi_mask(mask: *const cpumask, action: u32) {
    for_each_cpu(|i| octeon_send_ipi_single(i, action), mask);
}

unsafe fn octeon_smp_hotplug_setup() {
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    {
        if !setup_max_cpus { return; }
        let labi = PHYS_TO_XKSEG_CACHED(LABI_ADDR_IN_BOOTLOADER) as *mut linux_app_boot_info;
        if (*labi).labi_signature != LABI_SIGNATURE { return; }
        octeon_bootloader_entry_addr = (*labi).InitTLBStart_addr;
    }
}

unsafe fn octeon_smp_setup() {
    let coreid = cvmx_get_core_num();
    let sysinfo = cvmx_sysinfo_get();
    for id in 0..NR_CPUS { set_cpu_possible(id, id == 0); set_cpu_present(id, id == 0); }
    __cpu_number_map[coreid as usize] = 0;
    __cpu_logical_map[0] = coreid;
    let mut cpus = 1;
    for id in 0..NR_CPUS {
        if id != coreid && cvmx_coremask_is_core_set(&(*sysinfo).core_mask, id) {
            set_cpu_possible(cpus, true); set_cpu_present(cpus, true);
            __cpu_number_map[id as usize] = cpus; __cpu_logical_map[cpus as usize] = id; cpus += 1;
        }
    }
    octeon_smp_hotplug_setup();
}

unsafe fn octeon_boot_secondary(cpu: i32, idle: *mut task_struct) -> i32 {
    octeon_processor_sp = __KSTK_TOS(idle) as c_ulong;
    octeon_processor_gp = task_thread_info(idle) as c_ulong;
    octeon_processor_boot = cpu_logical_map(cpu) as c_ulong; mb();
    let mut count = 10000;
    while octeon_processor_sp != 0 && count != 0 { udelay(1); count -= 1; }
    if count == 0 { return -ETIMEDOUT; } 0
}

unsafe fn octeon_init_secondary() { let sr = set_c0_status(ST0_BEV); write_c0_ebase(ebase as u32); write_c0_status(sr); octeon_check_cpu_bist(); octeon_init_cvmcount(); octeon_irq_setup_secondary(); }

unsafe fn octeon_prepare_cpus(_max_cpus: u32) {
    cvmx_write_csr(CVMX_CIU_MBOX_CLRX(cvmx_get_core_num()), 0xffff);
    if request_irq(OCTEON_IRQ_MBOX0, mailbox_interrupt, IRQF_PERCPU | IRQF_NO_THREAD, "SMP-IPI", mailbox_interrupt) != 0 { panic!("Cannot request_irq(OCTEON_IRQ_MBOX0)"); }
}

unsafe fn octeon_smp_finish() { octeon_user_io_init(); write_c0_compare(read_c0_count() + mips_hpt_frequency / HZ); local_irq_enable(); }

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn octeon_cpu_disable() -> i32 { let cpu = smp_processor_id(); if octeon_bootloader_entry_addr == 0 { return -ENOTSUPP; } set_cpu_online(cpu, false); calculate_cpu_foreign_map(); octeon_fixup_irqs(); __flush_cache_all(); local_flush_tlb_all(); 0 }

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn octeon_cpu_die(cpu: u32) {
    let coreid = cpu_logical_map(cpu as i32);
    while per_cpu_cpu_state(cpu) != CPU_DEAD { cpu_relax(); }
    let mask = 1u32 << coreid;
    let block_desc = cvmx_bootmem_find_named_block(LINUX_APP_BOOT_BLOCK_NAME);
    let new_mask;
    if block_desc.is_null() {
        let labi = PHYS_TO_XKSEG_CACHED(LABI_ADDR_IN_BOOTLOADER) as *mut linux_app_boot_info;
        (*labi).avail_coremask |= mask; new_mask = (*labi).avail_coremask;
    } else {
        let p = PHYS_TO_XKSEG_CACHED((*block_desc).base_addr + AVAIL_COREMASK_OFFSET_IN_LINUX_APP_BOOT_BLOCK) as *mut u32;
        *p |= mask; new_mask = *p;
    }
    mb(); cvmx_write_csr(CVMX_CIU_PP_RST, 1u64 << coreid); cvmx_write_csr(CVMX_CIU_PP_RST, 0);
    let _ = new_mask;
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
pub unsafe fn play_dead() -> ! {
    let cpu = cpu_number_map(cvmx_get_core_num()); idle_task_exit(); cpuhp_ap_report_dead();
    octeon_processor_boot = 0xff; set_per_cpu_cpu_state(cpu as u32, CPU_DEAD); mb();
    loop { core::hint::spin_loop(); }
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn start_after_reset() { kernel_entry(0, 0, 0); }

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn octeon_update_boot_vector(cpu: u32) -> i32 {
    let coreid = cpu_logical_map(cpu as i32);
    let block_desc = cvmx_bootmem_find_named_block(LINUX_APP_BOOT_BLOCK_NAME);
    let avail_coremask;
    if block_desc.is_null() {
        let labi = PHYS_TO_XKSEG_CACHED(LABI_ADDR_IN_BOOTLOADER) as *mut linux_app_boot_info;
        avail_coremask = (*labi).avail_coremask; (*labi).avail_coremask &= !(1u32 << coreid);
    } else { avail_coremask = *(PHYS_TO_XKSEG_CACHED((*block_desc).base_addr + AVAIL_COREMASK_OFFSET_IN_LINUX_APP_BOOT_BLOCK) as *const u32); }
    if avail_coremask & (1u32 << coreid) == 0 { cvmx_write_csr(CVMX_CIU_PP_RST, 1u64 << coreid); cvmx_write_csr(CVMX_CIU_PP_RST, 0); }
    let boot_vect = PHYS_TO_XKSEG_CACHED(BOOTLOADER_BOOT_VECTOR) as *mut boot_init_vector;
    (*boot_vect.add(coreid as usize)).app_start_func_addr = start_after_reset as usize as u32;
    (*boot_vect.add(coreid as usize)).code_addr = octeon_bootloader_entry_addr as u32; mb();
    cvmx_write_csr(CVMX_CIU_NMI, ((1u32 << coreid) & avail_coremask) as u64); 0
}

unsafe fn octeon_78xx_reched_interrupt(_: i32, _: *mut c_void) -> irqreturn_t { scheduler_ipi(); IRQ_HANDLED }
unsafe fn octeon_78xx_call_function_interrupt(_: i32, _: *mut c_void) -> irqreturn_t { generic_smp_call_function_interrupt(); IRQ_HANDLED }
unsafe fn octeon_78xx_icache_flush_interrupt(_: i32, _: *mut c_void) -> irqreturn_t { octeon_icache_flush(); IRQ_HANDLED }

unsafe fn octeon_78xx_prepare_cpus(_max_cpus: u32) {
    if request_irq(OCTEON_IRQ_MBOX0, octeon_78xx_reched_interrupt, IRQF_PERCPU | IRQF_NO_THREAD, "Scheduler", octeon_78xx_reched_interrupt) != 0 { panic!("Cannot request_irq for SchedulerIPI"); }
    if request_irq(OCTEON_IRQ_MBOX0 + 1, octeon_78xx_call_function_interrupt, IRQF_PERCPU | IRQF_NO_THREAD, "SMP-Call", octeon_78xx_call_function_interrupt) != 0 { panic!("Cannot request_irq for SMP-Call"); }
    if request_irq(OCTEON_IRQ_MBOX0 + 2, octeon_78xx_icache_flush_interrupt, IRQF_PERCPU | IRQF_NO_THREAD, "ICache-Flush", octeon_78xx_icache_flush_interrupt) != 0 { panic!("Cannot request_irq for ICache-Flush"); }
}

unsafe fn octeon_78xx_send_ipi_single(cpu: i32, mut action: u32) { for i in 0..8 { if action & 1 != 0 { octeon_ciu3_mbox_send(cpu, i); } action >>= 1; } }
unsafe fn octeon_78xx_send_ipi_mask(mask: *const cpumask, action: u32) { for_each_cpu(|cpu| octeon_78xx_send_ipi_single(cpu, action), mask); }

pub unsafe extern "C" fn octeon_setup_smp() {
    let ops = if octeon_has_feature(OCTEON_FEATURE_CIU3) { &octeon_78xx_smp_ops } else { &octeon_smp_ops };
    register_smp_ops(ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
