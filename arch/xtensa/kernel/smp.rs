/*
 * Xtensa SMP support functions.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 - 2013 Tensilica Inc.
 *
 * Chris Zankel <chris@zankel.net>
 * Joe Taylor <joe@tensilica.com>
 * Pete Delaney <piet@tensilica.com
 */

// C headers and build-provided symbols are supplied by the surrounding kernel.

#[cfg(CONFIG_SMP)]
// XCHAL_HAVE_S32C1I == 0 is a build-time error in the C implementation.
const _: () = assert!(XCHAL_HAVE_S32C1I != 0);

extern "C" {
    fn system_invalidate_dcache_range(start: usize, size: usize);
    fn system_flush_invalidate_dcache_range(start: usize, size: usize);
}

const IPI_IRQ: u32 = 0;

unsafe extern "C" fn ipi_interrupt(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;

#[no_mangle]
pub unsafe extern "C" fn ipi_init() {
    let irq = irq_create_mapping(core::ptr::null_mut(), IPI_IRQ);
    if request_irq(irq, Some(ipi_interrupt), IRQF_PERCPU, b"ipi\0".as_ptr() as *const i8, core::ptr::null_mut()) != 0 {
        pr_err!("Failed to request irq %u (ipi)\n", irq);
    }
}

#[inline]
unsafe fn get_core_count() -> u32 {
    /* Bits 18..21 of SYSCFGID contain the core count minus 1. */
    let syscfgid = get_er(SYSCFGID);
    ((syscfgid >> 18) & 0xf) + 1
}

#[inline]
unsafe fn get_core_id() -> i32 {
    /* Bits 0...18 of SYSCFGID contain the core id  */
    let core_id = get_er(SYSCFGID);
    (core_id & 0x3fff) as i32
}

#[no_mangle]
pub unsafe extern "C" fn smp_prepare_cpus(_max_cpus: u32) {
    for_each_possible_cpu!(i, { set_cpu_present(i, true); });
}

#[no_mangle]
pub unsafe extern "C" fn smp_init_cpus() {
    let mut i: u32;
    let mut ncpus = get_core_count();
    let core_id = get_core_id();
    pr_info!("%s: Core Count = %d\n", __func__, ncpus);
    pr_info!("%s: Core Id = %d\n", __func__, core_id);
    if ncpus > NR_CPUS {
        ncpus = NR_CPUS;
        pr_info!("%s: limiting core count by %d\n", __func__, ncpus);
    }
    i = 0;
    while i < ncpus { set_cpu_possible(i, true); i += 1; }
}

#[no_mangle]
pub unsafe extern "C" fn smp_prepare_boot_cpu() {
    let cpu = smp_processor_id();
    BUG_ON!(cpu != 0);
    cpu_asid_cache(cpu) = ASID_USER_FIRST;
}

#[no_mangle]
pub unsafe extern "C" fn smp_cpus_done(_max_cpus: u32) {}

static mut boot_secondary_processors: i32 = 1; /* Set with xt-gdb via .xt-gdb */
static mut cpu_running: completion = DECLARE_COMPLETION!();

#[no_mangle]
pub unsafe extern "C" fn secondary_start_kernel() {
    let mm = &mut init_mm;
    let cpu = smp_processor_id();
    init_mmu();
    #[cfg(CONFIG_DEBUG_MISC)]
    {
        if boot_secondary_processors == 0 {
            pr_debug!("%s: boot_secondary_processors:%d; Hanging cpu:%d\n", __func__, boot_secondary_processors, cpu);
            loop { core::arch::asm!("waiti " /* LOCKLEVEL */); }
        }
        pr_debug!("%s: boot_secondary_processors:%d; Booting cpu:%d\n", __func__, boot_secondary_processors, cpu);
    }
    /* Init EXCSAVE1 */
    secondary_trap_init();
    /* All kernel threads share the same mm context. */
    mmget(mm);
    mmgrab(mm);
    (*current).active_mm = mm;
    cpumask_set_cpu(cpu, mm_cpumask(mm));
    enter_lazy_tlb(mm, current);
    trace_hardirqs_off();
    calibrate_delay();
    notify_cpu_starting(cpu);
    secondary_init_irq();
    local_timer_setup(cpu);
    set_cpu_online(cpu, true);
    local_irq_enable();
    complete(&mut cpu_running);
    cpu_startup_entry(CPUHP_AP_ONLINE_IDLE);
}

unsafe fn mx_cpu_start(p: *mut core::ffi::c_void) {
    let cpu = p as usize as u32;
    let run_stall_mask = get_er(MPSCORE);
    set_er(run_stall_mask & !(1u32 << cpu), MPSCORE);
    pr_debug!("%s: cpu: %d, run_stall_mask: %lx ---> %lx\n", __func__, cpu, run_stall_mask, get_er(MPSCORE));
}

unsafe fn mx_cpu_stop(p: *mut core::ffi::c_void) {
    let cpu = p as usize as u32;
    let run_stall_mask = get_er(MPSCORE);
    set_er(run_stall_mask | (1u32 << cpu), MPSCORE);
    pr_debug!("%s: cpu: %d, run_stall_mask: %lx ---> %lx\n", __func__, cpu, run_stall_mask, get_er(MPSCORE));
}

#[cfg(CONFIG_HOTPLUG_CPU)]
#[repr(align(64))]
static mut cpu_start_id: usize = 0;
static mut cpu_start_ccount: usize = 0;

unsafe fn boot_secondary(cpu: u32, _ts: *mut task_struct) -> i32 {
    let timeout = jiffies + msecs_to_jiffies(1000);
    let mut ccount;
    #[cfg(CONFIG_HOTPLUG_CPU)]
    { WRITE_ONCE!(cpu_start_id, cpu as usize); mb!(); system_flush_invalidate_dcache_range((&cpu_start_id as *const _ as usize), core::mem::size_of::<usize>()); }
    smp_call_function_single(0, Some(mx_cpu_start), cpu as usize as *mut _, 1);
    let mut i = 0;
    while i < 2 {
        loop { ccount = get_ccount() as usize; if ccount != 0 { break; } }
        WRITE_ONCE!(cpu_start_ccount, ccount);
        loop { mb!(); ccount = READ_ONCE!(cpu_start_ccount); if !(ccount != 0 && time_before(jiffies, timeout)) { break; } }
        if ccount != 0 { smp_call_function_single(0, Some(mx_cpu_stop), cpu as usize as *mut _, 1); WRITE_ONCE!(cpu_start_ccount, 0); return -EIO; }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn __cpu_up(cpu: u32, idle: *mut task_struct) -> i32 {
    let mut ret = 0;
    if cpu_asid_cache(cpu) == 0 { cpu_asid_cache(cpu) = ASID_USER_FIRST; }
    start_info.stack = task_pt_regs(idle) as usize;
    wmb!();
    pr_debug!("%s: Calling wakeup_secondary(cpu:%d, idle:%p, sp: %08lx)\n", __func__, cpu, idle, start_info.stack);
    init_completion(&mut cpu_running);
    ret = boot_secondary(cpu, idle);
    if ret == 0 { wait_for_completion_timeout(&mut cpu_running, msecs_to_jiffies(1000)); if !cpu_online(cpu) { ret = -EIO; } }
    if ret != 0 { pr_err!("CPU %u failed to boot\n", cpu); }
    ret
}

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe extern "C" fn __cpu_disable() -> i32 {
    let cpu = smp_processor_id();
    set_cpu_online(cpu, false);
    #[cfg(XTENSA_HAVE_COPROCESSORS)]
    local_coprocessors_flush_release_all();
    migrate_irqs();
    local_flush_cache_all(); local_flush_tlb_all(); invalidate_page_directory(); clear_tasks_mm_cpumask(cpu);
    0
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn platform_cpu_kill(cpu: u32) { smp_call_function_single(0, Some(mx_cpu_stop), cpu as usize as *mut _, true); }

#[cfg(CONFIG_HOTPLUG_CPU)]
#[no_mangle]
pub unsafe extern "C" fn __cpu_die(cpu: u32) {
    let timeout = jiffies + msecs_to_jiffies(1000);
    while time_before(jiffies, timeout) { system_invalidate_dcache_range((&cpu_start_id as *const _ as usize), core::mem::size_of::<usize>()); mb!(); if READ_ONCE!(cpu_start_id) == (-(cpu as i32)) as usize { platform_cpu_kill(cpu); return; } }
    pr_err!("CPU%u: unable to kill\n", cpu);
}

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe extern "C" fn arch_cpu_idle_dead() -> ! { cpu_die(); }

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe extern "C" fn cpu_die() -> ! {
    idle_task_exit(); local_irq_disable();
    core::arch::asm!("movi a2, cpu_restart", "jx a2");
    BUG!();
}

#[repr(C)]
enum ipi_msg_type { IPI_RESCHEDULE = 0, IPI_CALL_FUNC, IPI_CPU_STOP, IPI_MAX }

#[repr(C)]
struct ipi_text_entry { short_text: *const i8, long_text: *const i8 }
static ipi_text: [ipi_text_entry; 3] = [
    ipi_text_entry { short_text: b"RES\0".as_ptr() as _, long_text: b"Rescheduling interrupts\0".as_ptr() as _ },
    ipi_text_entry { short_text: b"CAL\0".as_ptr() as _, long_text: b"Function call interrupts\0".as_ptr() as _ },
    ipi_text_entry { short_text: b"DIE\0".as_ptr() as _, long_text: b"CPU shutdown interrupts\0".as_ptr() as _ },
];

#[repr(C)] struct ipi_data { ipi_count: [usize; IPI_MAX as usize] }
static mut ipi_data: PerCpu<ipi_data> = DEFINE_PER_CPU!();

unsafe fn send_ipi_message(callmask: *const cpumask, msg_id: ipi_msg_type) {
    let mut mask = 0usize;
    for_each_cpu!(index, callmask, { mask |= 1usize << index; });
    set_er(mask, MIPISET(msg_id as u32));
}

pub unsafe extern "C" fn arch_send_call_function_ipi_mask(mask: *const cpumask) { send_ipi_message(mask, ipi_msg_type::IPI_CALL_FUNC); }
pub unsafe extern "C" fn arch_send_call_function_single_ipi(cpu: i32) { send_ipi_message(cpumask_of(cpu), ipi_msg_type::IPI_CALL_FUNC); }
pub unsafe extern "C" fn arch_smp_send_reschedule(cpu: i32) { send_ipi_message(cpumask_of(cpu), ipi_msg_type::IPI_RESCHEDULE); }
pub unsafe extern "C" fn smp_send_stop() { let mut targets: cpumask = core::mem::zeroed(); cpumask_copy(&mut targets, cpu_online_mask); cpumask_clear_cpu(smp_processor_id(), &mut targets); send_ipi_message(&targets, ipi_msg_type::IPI_CPU_STOP); }
unsafe fn ipi_cpu_stop(cpu: u32) { set_cpu_online(cpu, false); machine_halt(); }

#[no_mangle]
pub unsafe extern "C" fn ipi_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let cpu = smp_processor_id();
    let ipi = per_cpu_ptr(&mut ipi_data, cpu);
    loop {
        let msg = get_er(MIPICAUSE(cpu)); set_er(msg, MIPICAUSE(cpu)); if msg == 0 { break; }
        if msg & (1 << ipi_msg_type::IPI_CALL_FUNC as u32) != 0 { (*ipi).ipi_count[IPI_CALL_FUNC as usize] += 1; generic_smp_call_function_interrupt(); }
        if msg & (1 << ipi_msg_type::IPI_RESCHEDULE as u32) != 0 { (*ipi).ipi_count[IPI_RESCHEDULE as usize] += 1; scheduler_ipi(); }
        if msg & (1 << ipi_msg_type::IPI_CPU_STOP as u32) != 0 { (*ipi).ipi_count[IPI_CPU_STOP as usize] += 1; ipi_cpu_stop(cpu); }
    }
    IRQ_HANDLED
}

pub unsafe extern "C" fn show_ipi_list(p: *mut seq_file, prec: i32) {
    let mut i = 0;
    while i < IPI_MAX as u32 { seq_printf(p, b"%*s:\0".as_ptr() as _, prec, ipi_text[i as usize].short_text); for_each_online_cpu!(cpu, { seq_printf(p, b" %10lu\0".as_ptr() as _, per_cpu_ptr(&mut ipi_data, cpu).as_ref().unwrap().ipi_count[i as usize]); }); seq_printf(p, b"   %s\n\0".as_ptr() as _, ipi_text[i as usize].long_text); i += 1; }
}

#[repr(C)] struct flush_data { vma: *mut vm_area_struct, addr1: usize, addr2: usize }
unsafe fn ipi_flush_tlb_all(_arg: *mut core::ffi::c_void) { local_flush_tlb_all(); }
pub unsafe extern "C" fn flush_tlb_all() { on_each_cpu(Some(ipi_flush_tlb_all), core::ptr::null_mut(), 1); }
unsafe fn ipi_flush_tlb_mm(arg: *mut core::ffi::c_void) { local_flush_tlb_mm(arg); }
pub unsafe extern "C" fn flush_tlb_mm(mm: *mut mm_struct) { on_each_cpu(Some(ipi_flush_tlb_mm), mm as *mut _, 1); }
unsafe fn ipi_flush_tlb_page(arg: *mut core::ffi::c_void) { let fd = &*(arg as *const flush_data); local_flush_tlb_page(fd.vma, fd.addr1); }
pub unsafe extern "C" fn flush_tlb_page(vma: *mut vm_area_struct, addr: usize) { let fd = flush_data { vma, addr1: addr, addr2: 0 }; on_each_cpu(Some(ipi_flush_tlb_page), &fd as *const _ as *mut _, 1); }
unsafe fn ipi_flush_tlb_range(arg: *mut core::ffi::c_void) { let fd = &*(arg as *const flush_data); local_flush_tlb_range(fd.vma, fd.addr1, fd.addr2); }
pub unsafe extern "C" fn flush_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize) { let fd = flush_data { vma, addr1: start, addr2: end }; on_each_cpu(Some(ipi_flush_tlb_range), &fd as *const _ as *mut _, 1); }
unsafe fn ipi_flush_tlb_kernel_range(arg: *mut core::ffi::c_void) { let fd = &*(arg as *const flush_data); local_flush_tlb_kernel_range(fd.addr1, fd.addr2); }
pub unsafe extern "C" fn flush_tlb_kernel_range(start: usize, end: usize) { let fd = flush_data { vma: core::ptr::null_mut(), addr1: start, addr2: end }; on_each_cpu(Some(ipi_flush_tlb_kernel_range), &fd as *const _ as *mut _, 1); }

unsafe fn ipi_flush_cache_all(_arg: *mut core::ffi::c_void) { local_flush_cache_all(); }
pub unsafe extern "C" fn flush_cache_all() { on_each_cpu(Some(ipi_flush_cache_all), core::ptr::null_mut(), 1); }
unsafe fn ipi_flush_cache_page(arg: *mut core::ffi::c_void) { let fd = &*(arg as *const flush_data); local_flush_cache_page(fd.vma, fd.addr1, fd.addr2); }
pub unsafe extern "C" fn flush_cache_page(vma: *mut vm_area_struct, address: usize, pfn: usize) { let fd = flush_data { vma, addr1: address, addr2: pfn }; on_each_cpu(Some(ipi_flush_cache_page), &fd as *const _ as *mut _, 1); }
unsafe fn ipi_flush_cache_range(arg: *mut core::ffi::c_void) { let fd = &*(arg as *const flush_data); local_flush_cache_range(fd.vma, fd.addr1, fd.addr2); }
pub unsafe extern "C" fn flush_cache_range(vma: *mut vm_area_struct, start: usize, end: usize) { let fd = flush_data { vma, addr1: start, addr2: end }; on_each_cpu(Some(ipi_flush_cache_range), &fd as *const _ as *mut _, 1); }
unsafe fn ipi_flush_icache_range(arg: *mut core::ffi::c_void) { let fd = &*(arg as *const flush_data); local_flush_icache_range(fd.addr1, fd.addr2); }
pub unsafe extern "C" fn flush_icache_range(start: usize, end: usize) { let fd = flush_data { vma: core::ptr::null_mut(), addr1: start, addr2: end }; on_each_cpu(Some(ipi_flush_icache_range), &fd as *const _ as *mut _, 1); }
// EXPORT_SYMBOL(flush_icache_range);

unsafe fn ipi_invalidate_dcache_range(arg: *mut core::ffi::c_void) { let fd = &*(arg as *const flush_data); __invalidate_dcache_range(fd.addr1, fd.addr2); }
unsafe fn system_invalidate_dcache_range_impl(start: usize, size: usize) { let fd = flush_data { vma: core::ptr::null_mut(), addr1: start, addr2: size }; on_each_cpu(Some(ipi_invalidate_dcache_range), &fd as *const _ as *mut _, 1); }
unsafe fn ipi_flush_invalidate_dcache_range(arg: *mut core::ffi::c_void) { let fd = &*(arg as *const flush_data); __flush_invalidate_dcache_range(fd.addr1, fd.addr2); }
unsafe fn system_flush_invalidate_dcache_range_impl(start: usize, size: usize) { let fd = flush_data { vma: core::ptr::null_mut(), addr1: start, addr2: size }; on_each_cpu(Some(ipi_flush_invalidate_dcache_range), &fd as *const _ as *mut _, 1); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
