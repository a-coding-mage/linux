// SPDX-License-Identifier: GPL-2.0
/* Sparc SS1000/SC2000 SMP support.
 *
 * Copyright (C) 1998 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 *
 * Based on sun4m's smp.c, which is:
 * Copyright (C) 1996 David S. Miller (davem@caip.rutgers.edu)
 */

// Linux and architecture dependencies supplied by the surrounding kernel.

const IRQ_CROSS_CALL: i32 = 15;

static mut smp_processors_ready: core::ffi::c_int = 0;
static mut smp_highest_cpu: i32 = 0;

#[inline]
unsafe fn sun4d_swap(ptr: *mut usize, mut val: usize) -> usize {
    core::ptr::swap(ptr, &mut val);
    val
}

extern "C" {
    fn smp4d_ipi_init();
}

static mut cpu_leds: [u8; 32] = [0; 32];

#[inline]
unsafe fn show_leds(mut cpuid: i32) {
    cpuid &= 0x1e;
    let value = ((cpu_leds[cpuid as usize] << 4) | cpu_leds[(cpuid + 1) as usize]) as u8;
    core::ptr::write_volatile((ECSR_BASE(cpuid) | BB_LEDS) as *mut u8, value);
}

pub unsafe extern "C" fn sun4d_cpu_pre_starting(_arg: *mut core::ffi::c_void) {
    let cpuid = hard_smp_processor_id();
    cpu_leds[cpuid as usize] = 0x6;
    show_leds(cpuid);
    cc_set_imsk((cc_get_imsk() & !0x8000) | 0x4000);
}

pub unsafe extern "C" fn sun4d_cpu_pre_online(_arg: *mut core::ffi::c_void) {
    let flags: usize;
    let cpuid = hard_smp_processor_id();
    sun4d_swap((&mut cpu_callin_map[cpuid as usize]) as *mut _ as *mut usize, 1);
    (*local_ops).cache_all();
    (*local_ops).tlb_all();
    while (current_set[cpuid as usize] as usize) < PAGE_OFFSET { core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst); }
    while (*current_set[cpuid as usize]).cpu != cpuid { core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst); }
    core::arch::asm!("ld [{0}], %%g6", in(reg) &current_set[cpuid as usize], options(nostack));
    cpu_leds[cpuid as usize] = 0x9;
    show_leds(cpuid);
    mmgrab(&mut init_mm);
    (*current).active_mm = &mut init_mm;
    (*local_ops).cache_all();
    (*local_ops).tlb_all();
    while !cpumask_test_cpu(cpuid, &smp_commenced_mask) { core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst); }
    spin_lock_irqsave(&sun4d_imsk_lock, &flags);
    cc_set_imsk(cc_get_imsk() & !0x4000);
    spin_unlock_irqrestore(&sun4d_imsk_lock, flags);
}

pub unsafe extern "C" fn smp4d_boot_cpus() {
    smp4d_ipi_init();
    if boot_cpu_id != 0 { current_set[0] = core::ptr::null_mut(); }
    (*local_ops).cache_all();
}

pub unsafe extern "C" fn smp4d_boot_one_cpu(i: i32, idle: *mut task_struct) -> i32 {
    let entry: *mut usize = &mut sun4d_cpu_startup;
    let mut timeout: i32;
    let mut cpu_node: i32 = 0;
    cpu_find_by_instance(i, &mut cpu_node, core::ptr::null_mut());
    current_set[i as usize] = task_thread_info(idle);
    smp_penguin_ctable.which_io = 0;
    smp_penguin_ctable.phys_addr = srmmu_ctx_table_phys as u32;
    smp_penguin_ctable.reg_size = 0;
    printk(KERN_INFO, "Starting CPU %d at %p\n", i, entry);
    (*local_ops).cache_all();
    prom_startcpu(cpu_node, &mut smp_penguin_ctable, 0, entry as *mut u8);
    printk(KERN_INFO, "prom_startcpu returned :)\n");
    timeout = 0;
    while timeout < 10000 {
        if cpu_callin_map[i as usize] != 0 { break; }
        udelay(200);
        timeout += 1;
    }
    if cpu_callin_map[i as usize] == 0 { printk(KERN_ERR, "Processor %d is stuck.\n", i); return -19; }
    (*local_ops).cache_all();
    0
}

pub unsafe extern "C" fn smp4d_smp_done() {
    let mut first = 0;
    let mut prev: *mut i32 = &mut first;
    let mut i = 0;
    for_each_online_cpu!(i) {
        *prev = i;
        prev = &mut cpu_data(i).next;
    }
    *prev = first;
    (*local_ops).cache_all();
    smp_processors_ready = 1;
    sun4d_distribute_irqs();
}

#[repr(C)]
struct sun4d_ipi_work { single: i32, msk: i32, resched: i32 }
static mut sun4d_ipi_work: [sun4d_ipi_work; NR_CPUS] = [sun4d_ipi_work { single: 0, msk: 0, resched: 0 }; NR_CPUS];

unsafe fn smp4d_ipi_init() {
    printk(KERN_INFO, "smp4d: setup IPI at IRQ %d\n", SUN4D_IPI_IRQ);
    for cpu in 0..NR_CPUS { sun4d_ipi_work[cpu] = sun4d_ipi_work { single: 0, msk: 0, resched: 0 }; }
}

pub unsafe extern "C" fn sun4d_ipi_interrupt() {
    let work = &mut sun4d_ipi_work[hard_smp_processor_id() as usize];
    if work.single != 0 { work.single = 0; smp_call_function_single_interrupt(); }
    if work.msk != 0 { work.msk = 0; smp_call_function_interrupt(); }
    if work.resched != 0 { work.resched = 0; smp_resched_interrupt(); }
}

#[inline]
const fn IGEN_MESSAGE(bcast: i32, devid: i32, sid: i32, levels: i32) -> i32 { (bcast << 31) | (devid << 23) | (sid << 15) | levels }
unsafe fn sun4d_send_ipi(cpu: i32, level: i32) { cc_set_igen(IGEN_MESSAGE(0, cpu << 3, 6 + ((level >> 1) & 7), 1 << (level - 1))); }
unsafe fn sun4d_ipi_single(cpu: i32) { sun4d_ipi_work[cpu as usize].single = 1; sun4d_send_ipi(cpu, SUN4D_IPI_IRQ); }
unsafe fn sun4d_ipi_mask_one(cpu: i32) { sun4d_ipi_work[cpu as usize].msk = 1; sun4d_send_ipi(cpu, SUN4D_IPI_IRQ); }
unsafe fn sun4d_ipi_resched(cpu: i32) { sun4d_ipi_work[cpu as usize].resched = 1; sun4d_send_ipi(cpu, SUN4D_IPI_IRQ); }

#[repr(C, align(8))]
struct smp_funcall { func: Option<unsafe extern "C" fn(usize, usize, usize, usize, usize)>, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, processors_in: [u8; NR_CPUS], processors_out: [u8; NR_CPUS] }
static mut ccall_info: smp_funcall = smp_funcall { func: None, arg1: 0, arg2: 0, arg3: 0, arg4: 0, arg5: 0, processors_in: [0; NR_CPUS], processors_out: [0; NR_CPUS] };

unsafe fn sun4d_cross_call(_func: *mut core::ffi::c_void, mut mask: cpumask_t, arg1: usize, arg2: usize, arg3: usize, arg4: usize) {
    if smp_processors_ready != 0 {
        let high = smp_highest_cpu;
        let flags: usize;
        spin_lock_irqsave(&cross_call_lock, &flags);
        ccall_info.arg1 = arg1; ccall_info.arg2 = arg2; ccall_info.arg3 = arg3; ccall_info.arg4 = arg4; ccall_info.arg5 = 0;
        cpumask_clear_cpu(smp_processor_id(), &mut mask); cpumask_and(&mut mask, cpu_online_mask, &mask);
        for i in 0..=high { if cpumask_test_cpu(i, &mask) { ccall_info.processors_in[i as usize] = 0; ccall_info.processors_out[i as usize] = 0; sun4d_send_ipi(i, IRQ_CROSS_CALL); } }
        for i in 0..=high { if cpumask_test_cpu(i, &mask) { while ccall_info.processors_in[i as usize] == 0 { core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst); } } }
        for i in 0..=high { if cpumask_test_cpu(i, &mask) { while ccall_info.processors_out[i as usize] == 0 { core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst); } } }
        spin_unlock_irqrestore(&cross_call_lock, flags);
    }
}

pub unsafe extern "C" fn smp4d_cross_call_irq() {
    let i = hard_smp_processor_id(); ccall_info.processors_in[i as usize] = 1;
    if let Some(func) = ccall_info.func { func(ccall_info.arg1, ccall_info.arg2, ccall_info.arg3, ccall_info.arg4, ccall_info.arg5); }
    ccall_info.processors_out[i as usize] = 1;
}

pub unsafe extern "C" fn smp4d_percpu_timer_interrupt(regs: *mut pt_regs) {
    static mut cpu_tick: [i32; NR_CPUS] = [0; NR_CPUS];
    static led_mask: [i8; 6] = [0xe, 0xd, 0xb, 0x7, 0xb, 0xd];
    let cpu = hard_smp_processor_id(); let old_regs = set_irq_regs(regs);
    bw_get_prof_limit(cpu); bw_clear_intr_mask(0, 1); cpu_tick[cpu as usize] += 1;
    if (cpu_tick[cpu as usize] & 15) == 0 { if cpu_tick[cpu as usize] == 0x60 { cpu_tick[cpu as usize] = 0; } cpu_leds[cpu as usize] = led_mask[(cpu_tick[cpu as usize] >> 4) as usize] as u8; show_leds(cpu); }
    let ce = &mut sparc32_clockevent[cpu as usize]; irq_enter(); (ce.event_handler)(ce); irq_exit(); set_irq_regs(old_regs);
}

pub unsafe extern "C" fn sun4d_init_smp() {
    t_nmi[1] = t_nmi[1] + (linux_trap_ipi15_sun4d - linux_trap_ipi15_sun4m);
    sparc32_ipi_ops = &sun4d_ipi_ops;
    for i in 0..NR_CPUS { ccall_info.processors_in[i] = 1; ccall_info.processors_out[i] = 1; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
