// SPDX-License-Identifier: GPL-2.0-or-later
/*
** SMP Support
**
** Copyright (C) 1999 Walt Drummond <drummond@valinux.com>
** Copyright (C) 1999 David Mosberger-Tang <davidm@hpl.hp.com>
** Copyright (C) 2001,2004 Grant Grundler <grundler@parisc-linux.org>
**
** Lots of stuff stolen from arch/alpha/kernel/smp.c
** ...and then parisc stole from arch/ia64/kernel/smp.c. Thanks David! :^)
**
** Thanks to John Curry and Ullas Ponnadi. I learned a lot from their work.
** -grant (1/12/2001)
*/

// C headers provide the kernel types, constants, and external symbols used below.

#[allow(non_camel_case_types)]
type irqreturn_t = i32;
type c_int = i32;
type c_uint = u32;
type c_ulong = usize;

#[repr(C)] pub struct task_struct { pub active_mm: *mut mm_struct, pub mm: *mut mm_struct }
#[repr(C)] pub struct mm_struct;
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct cpumask;
#[repr(C)] pub struct irq_desc { pub kstat_irqs: *mut irqstat }
#[repr(C)] pub struct irqstat;
#[repr(C)] pub struct cpuinfo_parisc { pub pending_ipi: c_ulong, pub hpa: c_ulong }

const IPI_NOP: c_uint = 0;
const IPI_RESCHEDULE: c_uint = 1;
const IPI_CALL_FUNC: c_uint = 2;
const IPI_CPU_START: c_uint = 3;
const IPI_CPU_STOP: c_uint = 4;
const IPI_CPU_TEST: c_uint = 5;
// CONFIG_KGDB adds IPI_ENTER_KGDB = 6.
const CPU_IRQ_BASE: c_ulong = 0; // supplied by the architecture headers
const IPI_IRQ: c_ulong = 0;
const TIMER_IRQ: c_ulong = 0;
const NO_PROC_ID: c_int = -1;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;

extern "C" {
    static mut smp_init_current_idle_task: *const task_struct;
    static mut cpu_now_booting: c_int;
    static mut current: *mut task_struct;
    static mut init_mm: mm_struct;
    static mut nr_cpu_ids: c_uint;
    static mut setup_max_cpus: c_uint;
    static mut time_keeper_id: c_uint;
    static mut cpu_online_mask: cpumask;
    static mut PAGE0: Page0;
    fn per_cpu_cpu_data(cpu: c_int) -> *mut cpuinfo_parisc;
    fn per_cpu_ipi_lock(cpu: c_int) -> *mut spinlock_t;
    fn smp_processor_id() -> c_int;
    fn raw_smp_processor_id() -> c_int;
    fn cpu_online(cpu: c_int) -> bool;
    fn set_cpu_online(cpu: c_int, online: bool);
    fn num_online_cpus() -> c_uint;
    fn init_per_cpu(cpu: c_int); fn disable_sr_hashing(); fn mb();
    fn machine_halt() -> !; fn notify_cpu_starting(cpu: c_int);
    fn mmgrab(mm: *mut mm_struct); fn enter_lazy_tlb(mm: *mut mm_struct, task: *mut task_struct);
    fn init_IRQ(); fn parisc_clockevent_init(); fn flush_cache_all_local();
    fn flush_tlb_all_local(arg: *mut core::ffi::c_void); fn local_irq_enable();
    fn local_irq_disable(); fn cpu_startup_entry(state: c_int) -> !; fn panic(s: *const u8) -> !;
    fn cpu_startup_entry_dummy(); fn gsc_writel(value: c_ulong, hpa: c_ulong);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn generic_smp_call_function_interrupt(); fn scheduler_ipi();
    fn inc_irq_stat(stat: c_ulong); fn get_irq_regs() -> *mut core::ffi::c_void;
    fn kgdb_nmicallback(cpu: c_int, regs: *mut core::ffi::c_void);
    fn printk(fmt: *const u8, ...); fn pr_info(fmt: *const u8, ...);
    fn preempt_disable(); fn preempt_enable(); fn barrier(); fn udelay(us: c_ulong);
    fn irq_to_desc(i: c_int) -> *mut irq_desc; fn spin_lock_init(lock: *mut spinlock_t);
    fn init_cpu_present(mask: *const cpumask); fn cpumask_of(cpu: c_uint) -> *const cpumask;
    fn remove_cpu_topology(cpu: c_uint); fn disable_percpu_irq(irq: c_ulong);
    fn irq_migrate_all_off_this_cpu(); fn mdelay(ms: c_ulong); fn set_eiem(v: c_ulong);
    fn mtctl(v: c_ulong, reg: c_ulong); fn mfctl(reg: c_ulong) -> c_ulong;
    fn pdc_cpu_rendezvous(); fn pdc_cpu_rendezvous_lock(); fn pdc_cpu_rendezvous_unlock();
    fn set_current_state(state: c_int); fn schedule_timeout(ticks: c_ulong) -> c_ulong;
}

#[repr(C)] struct Page0 { mem_pdc_hi: u32, mem_pdc: u32 }

unsafe fn halt_processor() -> ! {
    set_cpu_online(smp_processor_id(), false); local_irq_disable(); pdc_cpu_rendezvous();
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn ipi_interrupt(_irq: c_int, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let this_cpu = smp_processor_id();
    loop {
        let p = &mut *per_cpu_cpu_data(this_cpu); let lock = per_cpu_ipi_lock(this_cpu);
        let mut flags = 0; spin_lock_irqsave(lock, &mut flags); let mut ops = p.pending_ipi;
        p.pending_ipi = 0; spin_unlock_irqrestore(lock, flags); mb(); if ops == 0 { break; }
        while ops != 0 {
            let which = (!ops).trailing_zeros(); ops &= !(1usize << which);
            match which {
                IPI_NOP => {},
                IPI_RESCHEDULE => { inc_irq_stat(0); scheduler_ipi(); },
                IPI_CALL_FUNC => { inc_irq_stat(0); generic_smp_call_function_interrupt(); },
                IPI_CPU_START => {}, IPI_CPU_STOP => halt_processor(), IPI_CPU_TEST => {},
                _ => { printk(b"Unknown IPI num\0".as_ptr()); return IRQ_NONE; }
            }
            if ops != 0 { local_irq_enable(); local_irq_disable(); }
        }
    }
    IRQ_HANDLED
}

unsafe fn ipi_send(cpu: c_int, op: c_uint) {
    let p = &mut *per_cpu_cpu_data(cpu); let lock = per_cpu_ipi_lock(cpu); let mut flags = 0;
    spin_lock_irqsave(lock, &mut flags); p.pending_ipi |= 1usize << op; gsc_writel(IPI_IRQ - CPU_IRQ_BASE, p.hpa); spin_unlock_irqrestore(lock, flags);
}
unsafe fn send_IPI_mask(mask: *const cpumask, op: c_uint) { let _ = mask; for cpu in 0..nr_cpu_ids { if cpu_online(cpu as c_int) { ipi_send(cpu as c_int, op); } } }
unsafe fn send_IPI_single(dest_cpu: c_int, op: c_uint) { if dest_cpu == NO_PROC_ID { panic!(b"BUG_ON\0".as_ptr()) } ipi_send(dest_cpu, op); }
unsafe fn send_IPI_allbutself(op: c_uint) { preempt_disable(); for i in 0..nr_cpu_ids { if i as c_int != smp_processor_id() && cpu_online(i as c_int) { send_IPI_single(i as c_int, op); } } preempt_enable(); }

#[no_mangle] pub unsafe extern "C" fn smp_send_stop() { send_IPI_allbutself(IPI_CPU_STOP); }
#[no_mangle] pub unsafe extern "C" fn arch_smp_send_reschedule(cpu: c_int) { send_IPI_single(cpu, IPI_RESCHEDULE); }
#[no_mangle] pub unsafe extern "C" fn smp_send_all_nop() { send_IPI_allbutself(IPI_NOP); }
#[no_mangle] pub unsafe extern "C" fn arch_send_call_function_ipi_mask(mask: *const cpumask) { send_IPI_mask(mask, IPI_CALL_FUNC); }
#[no_mangle] pub unsafe extern "C" fn arch_send_call_function_single_ipi(cpu: c_int) { send_IPI_single(cpu, IPI_CALL_FUNC); }

unsafe fn smp_cpu_init(cpunum: c_int) {
    init_per_cpu(cpunum); disable_sr_hashing(); mb();
    if cpu_online(cpunum) { machine_halt(); }
    notify_cpu_starting(cpunum); set_cpu_online(cpunum, true); mmgrab(&mut init_mm);
    (*current).active_mm = &mut init_mm; enter_lazy_tlb(&mut init_mm, current); init_IRQ(); parisc_clockevent_init();
}

#[no_mangle] pub unsafe extern "C" fn smp_callin(pdce_proc: c_ulong) {
    let slave_id = cpu_now_booting; let _ = pdce_proc;
    smp_cpu_init(slave_id); flush_cache_all_local(); flush_tlb_all_local(core::ptr::null_mut()); local_irq_enable(); cpu_startup_entry(0);
}

unsafe fn smp_boot_one_cpu(cpuid: c_int, idle: *mut task_struct) -> c_int {
    let p = &*per_cpu_cpu_data(cpuid); let mut timeout: c_ulong = 0;
    while cpu_now_booting != 0 {} cpu_now_booting = cpuid; smp_init_current_idle_task = idle; mb();
    printk(b"Releasing cpu\0".as_ptr()); gsc_writel(TIMER_IRQ - CPU_IRQ_BASE, p.hpa); mb();
    while timeout < 10000 { if cpu_online(cpuid) { cpu_now_booting = 0; return 0; } udelay(100); barrier(); timeout += 1; }
    printk(b"SMP: CPU stuck.\0".as_ptr()); -1
}

#[no_mangle] pub unsafe extern "C" fn smp_prepare_boot_cpu() { pr_info(b"SMP: bootstrap CPU ID is 0\n\0".as_ptr()); }
#[no_mangle] pub unsafe extern "C" fn smp_prepare_cpus(_max_cpus: c_uint) { for cpu in 0..nr_cpu_ids { spin_lock_init(per_cpu_ipi_lock(cpu as c_int)); } init_cpu_present(cpumask_of(0)); }
#[no_mangle] pub unsafe extern "C" fn smp_cpus_done(_cpu_max: c_uint) {}
#[no_mangle] pub unsafe extern "C" fn __cpu_up(cpu: c_uint, tidle: *mut task_struct) -> c_int { if cpu_online(cpu as c_int) { return 0; } if num_online_cpus() < nr_cpu_ids && num_online_cpus() < setup_max_cpus && smp_boot_one_cpu(cpu as c_int, tidle) != 0 { return -5; } if cpu_online(cpu as c_int) { 0 } else { -5 } }
#[no_mangle] pub unsafe extern "C" fn __cpu_disable() -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn __cpu_die(_cpu: c_uint) { pdc_cpu_rendezvous_lock(); }
#[no_mangle] pub unsafe extern "C" fn arch_cpuhp_cleanup_dead_cpu(cpu: c_uint) { pr_info(b"CPU%u: is shutting down\n\0".as_ptr(), cpu); pdc_cpu_rendezvous_unlock(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
