/*
 * Copyright (C) 2014 Stefan Kristiansson <stefan.kristiansson@saunalahti.fi>
 * Copyright (C) 2017 Stafford Horne <shorne@gmail.com>
 *
 * Based on arm64 and arc implementations
 * Copyright (C) 2013 ARM Ltd.
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2.  This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

use core::ffi::c_void;

// C headers and build-time kernel configuration are supplied by other files.

#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct thread_info { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { pub pgd: *mut c_void, _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { pub vm_mm: *mut mm_struct, _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }

type CTaskFn = unsafe extern "C" fn(*mut c_void);

extern "C" {
    static mut current_pgd: [*mut c_void; 0];
    static mut init_mm: mm_struct;
    static mut current: *mut task_struct;
    static mut nr_cpu_ids: u32;
    static mut cpu_online_mask: *const cpumask;
    static mut secondary_release: usize;
    static mut secondary_thread_info: *mut thread_info;
    fn WARN_ON_ONCE(condition: bool) -> bool;
    fn WARN(condition: bool, fmt: *const u8, ...);
    fn enable_percpu_irq(irq: u32, flags: u32);
    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn cpumask_of(cpu: u32) -> *const cpumask;
    fn smp_cross_call(mask: *const cpumask, msg: u32);
    fn for_each_of_cpu_node(node: *mut *mut device_node);
    fn of_get_cpu_hwid(node: *mut device_node, index: u32) -> u32;
    fn set_cpu_possible(cpu: u32, possible: bool);
    fn set_cpu_present(cpu: u32, present: bool);
    fn task_thread_info(task: *mut task_struct) -> *mut thread_info;
    fn wait_for_completion_timeout(completion: *mut c_void, timeout: u64) -> u64;
    fn msecs_to_jiffies(ms: u32) -> u64;
    fn pr_warn(fmt: *const u8, ...);
    fn pr_crit(fmt: *const u8, ...);
    fn smp_processor_id() -> u32;
    fn mmgrab(mm: *mut mm_struct);
    fn cpumask_set_cpu(cpu: u32, mask: *mut cpumask);
    fn mm_cpumask(mm: *mut mm_struct) -> *mut cpumask;
    fn pr_info(fmt: *const u8, ...);
    fn setup_cpuinfo();
    fn openrisc_clockevent_init();
    fn notify_cpu_starting(cpu: u32);
    fn complete(completion: *mut c_void);
    fn synchronise_count_master(cpu: u32);
    fn synchronise_count_slave(cpu: u32);
    fn set_cpu_online(cpu: u32, online: bool);
    fn local_irq_enable();
    fn cpu_startup_entry(state: u32);
    fn scheduler_ipi();
    fn generic_smp_call_function_interrupt();
    fn generic_smp_call_function_single_interrupt();
    fn local_irq_disable();
    fn mfspr(reg: u32) -> u32;
    fn mtspr(reg: u32, value: u32);
    fn smp_call_function(func: CTaskFn, info: *mut c_void, wait: u32);
    fn get_cpu() -> u32;
    fn put_cpu();
    fn cpumask_empty(mask: *const cpumask) -> bool;
    fn cpumask_any_but(mask: *const cpumask, cpu: u32) -> u32;
    fn local_flush_tlb_all();
    fn local_flush_tlb_mm(mm: *mut mm_struct);
    fn on_each_cpu_mask(mask: *const cpumask, func: CTaskFn, info: *mut c_void, wait: u32);
    fn local_flush_tlb_page(vma: *mut vm_area_struct, addr: usize);
    fn local_flush_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize);
    fn on_each_cpu(func: CTaskFn, info: *mut c_void, wait: u32);
    fn local_icache_page_inv(page: *mut page);
    fn local_icache_all_inv();
    fn num_online_cpus() -> u32;
    fn mb();
}

const IPI_WAKEUP: u32 = 0;
const IPI_RESCHEDULE: u32 = 1;
const IPI_CALL_FUNC: u32 = 2;
const IPI_CALL_FUNC_SINGLE: u32 = 3;
const SPR_UPR: u32 = 0;
const SPR_PMR: u32 = 0;
const SPR_UPR_PMP: u32 = 0;
const SPR_PMR_DME: u32 = 0;
const CPUHP_AP_ONLINE_IDLE: u32 = 0;
const PAGE_SIZE: usize = 4096;
const EIO: i32 = 5;

static mut ipi_irq: u32 = 0;
static mut smp_cross_call_ptr: Option<unsafe extern "C" fn(*const cpumask, u32)> = None;
static mut secondary_release_global: usize = usize::MAX;
static mut secondary_thread_info_global: *mut thread_info = core::ptr::null_mut();
static mut boot_lock: u8 = 0;
static mut cpu_running: u8 = 0;

unsafe fn or1k_ipi_enable() {
    if WARN_ON_ONCE(ipi_irq == 0) { return; }
    enable_percpu_irq(ipi_irq, 0);
}

unsafe fn boot_secondary(cpu: u32, _idle: *mut task_struct) {
    spin_lock(&mut boot_lock as *mut _ as *mut c_void);
    secondary_release_global = cpu as usize;
    if let Some(call) = smp_cross_call_ptr { call(cpumask_of(cpu), IPI_WAKEUP); }
    spin_unlock(&mut boot_lock as *mut _ as *mut c_void);
}

pub unsafe extern "C" fn smp_init_cpus() {
    let mut cpu: *mut device_node = core::ptr::null_mut();
    for_each_of_cpu_node(&mut cpu);
    while !cpu.is_null() {
        let cpu_id = of_get_cpu_hwid(cpu, 0);
        if cpu_id < 0 { set_cpu_possible(cpu_id, true); }
        for_each_of_cpu_node(&mut cpu);
    }
}

pub unsafe extern "C" fn smp_prepare_cpus(max_cpus: u32) {
    for cpu in 0..max_cpus { set_cpu_present(cpu, true); }
}

pub unsafe extern "C" fn smp_cpus_done(_max_cpus: u32) {}

pub unsafe extern "C" fn __cpu_up(cpu: u32, idle: *mut task_struct) -> i32 {
    if smp_cross_call_ptr.is_none() { pr_warn(b"CPU%u: failed to start, IPI controller missing\0".as_ptr(), cpu); return -EIO; }
    secondary_thread_info_global = task_thread_info(idle);
    boot_secondary(cpu, idle);
    if wait_for_completion_timeout(&mut cpu_running as *mut _ as *mut c_void, msecs_to_jiffies(1000)) == 0 {
        pr_crit(b"CPU%u: failed to start\n\0".as_ptr(), cpu); return -EIO;
    }
    synchronise_count_master(cpu); 0
}

pub unsafe extern "C" fn secondary_start_kernel() {
    let mm = &mut init_mm as *mut mm_struct;
    let cpu = smp_processor_id();
    mmgrab(mm); (*current).active_mm = mm; cpumask_set_cpu(cpu, mm_cpumask(mm));
    pr_info(b"CPU%u: Booted secondary processor\n\0".as_ptr(), cpu);
    setup_cpuinfo(); openrisc_clockevent_init(); notify_cpu_starting(cpu);
    complete(&mut cpu_running as *mut _ as *mut c_void); synchronise_count_slave(cpu);
    or1k_ipi_enable(); set_cpu_online(cpu, true); local_irq_enable(); cpu_startup_entry(CPUHP_AP_ONLINE_IDLE);
}

pub unsafe extern "C" fn handle_IPI(ipi_msg: u32) {
    let cpu = smp_processor_id();
    match ipi_msg { IPI_WAKEUP => {}, IPI_RESCHEDULE => scheduler_ipi(), IPI_CALL_FUNC => generic_smp_call_function_interrupt(), IPI_CALL_FUNC_SINGLE => generic_smp_call_function_single_interrupt(), _ => WARN(true, b"CPU%u: Unknown IPI message 0x%x\n\0".as_ptr(), cpu, ipi_msg) }
}

pub unsafe extern "C" fn arch_smp_send_reschedule(cpu: i32) { if let Some(call) = smp_cross_call_ptr { call(cpumask_of(cpu as u32), IPI_RESCHEDULE); } }

unsafe extern "C" fn stop_this_cpu(_dummy: *mut c_void) { set_cpu_online(smp_processor_id(), false); local_irq_disable(); if mfspr(SPR_UPR) & SPR_UPR_PMP != 0 { mtspr(SPR_PMR, mfspr(SPR_PMR) | SPR_PMR_DME); } loop {} }
pub unsafe extern "C" fn smp_send_stop() { smp_call_function(stop_this_cpu, core::ptr::null_mut(), 0); }

pub unsafe extern "C" fn set_smp_cross_call(fn_: Option<unsafe extern "C" fn(*const cpumask, u32)>, irq: u32) { if WARN_ON_ONCE(ipi_irq != 0) { return; } smp_cross_call_ptr = fn_; ipi_irq = irq; or1k_ipi_enable(); }
pub unsafe extern "C" fn arch_send_call_function_single_ipi(cpu: i32) { if let Some(call) = smp_cross_call_ptr { call(cpumask_of(cpu as u32), IPI_CALL_FUNC_SINGLE); } }
pub unsafe extern "C" fn arch_send_call_function_ipi_mask(mask: *const cpumask) { if let Some(call) = smp_cross_call_ptr { call(mask, IPI_CALL_FUNC); } }

#[repr(C)] pub struct flush_tlb_data { pub addr1: usize, pub addr2: usize }
unsafe extern "C" fn ipi_flush_tlb_all(_ignored: *mut c_void) { local_flush_tlb_all(); }
unsafe extern "C" fn ipi_flush_tlb_mm(info: *mut c_void) { local_flush_tlb_mm(info as *mut mm_struct); }
unsafe fn smp_flush_tlb_mm(cmask: *mut cpumask, mm: *mut mm_struct) { if cpumask_empty(cmask) { return; } let cpu = get_cpu(); if cpumask_any_but(cmask, cpu) >= nr_cpu_ids { local_flush_tlb_mm(mm); } else { on_each_cpu_mask(cmask, ipi_flush_tlb_mm, mm as *mut c_void, 1); } put_cpu(); }
unsafe extern "C" fn ipi_flush_tlb_page(info: *mut c_void) { local_flush_tlb_page(core::ptr::null_mut(), (*(info as *mut flush_tlb_data)).addr1); }
unsafe extern "C" fn ipi_flush_tlb_range(info: *mut c_void) { let fd = &*(info as *mut flush_tlb_data); local_flush_tlb_range(core::ptr::null_mut(), fd.addr1, fd.addr2); }
unsafe fn smp_flush_tlb_range(cmask: *const cpumask, start: usize, end: usize) { if cpumask_empty(cmask) { return; } let cpu = get_cpu(); if cpumask_any_but(cmask, cpu) >= nr_cpu_ids { if end.wrapping_sub(start) <= PAGE_SIZE { local_flush_tlb_page(core::ptr::null_mut(), start); } else { local_flush_tlb_range(core::ptr::null_mut(), start, end); } } else { let mut fd = flush_tlb_data { addr1: start, addr2: end }; if end.wrapping_sub(start) <= PAGE_SIZE { on_each_cpu_mask(cmask, ipi_flush_tlb_page, &mut fd as *mut _ as *mut c_void, 1); } else { on_each_cpu_mask(cmask, ipi_flush_tlb_range, &mut fd as *mut _ as *mut c_void, 1); } } put_cpu(); }
pub unsafe extern "C" fn flush_tlb_all() { on_each_cpu(ipi_flush_tlb_all, core::ptr::null_mut(), 1); }
pub unsafe extern "C" fn flush_tlb_mm(mm: *mut mm_struct) { smp_flush_tlb_mm(mm_cpumask(mm), mm); }
pub unsafe extern "C" fn flush_tlb_page(vma: *mut vm_area_struct, uaddr: usize) { smp_flush_tlb_range(mm_cpumask((*vma).vm_mm), uaddr, uaddr.wrapping_add(PAGE_SIZE)); }
pub unsafe extern "C" fn flush_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize) { let cmask = if !vma.is_null() { mm_cpumask((*vma).vm_mm) } else { cpu_online_mask as *mut cpumask }; smp_flush_tlb_range(cmask, start, end); }
unsafe extern "C" fn ipi_icache_page_inv(arg: *mut c_void) { local_icache_page_inv(arg as *mut page); }
pub unsafe extern "C" fn smp_icache_page_inv(page: *mut page) { on_each_cpu(ipi_icache_page_inv, page as *mut c_void, 1); }
unsafe extern "C" fn ipi_icache_all_inv(_arg: *mut c_void) { local_icache_all_inv(); }
pub unsafe extern "C" fn smp_icache_all_inv() { if num_online_cpus() < 2 { local_icache_all_inv(); return; } mb(); on_each_cpu(ipi_icache_all_inv, core::ptr::null_mut(), 1); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
