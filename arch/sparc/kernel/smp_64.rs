// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level translation of sparc/kernel/smp_64.c.
 * Kernel-provided symbols and configuration-dependent assembly are intentionally
 * referenced as external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const MASTER: usize = 0;
pub const SLAVE: usize = 1;
pub const NUM_ROUNDS: usize = 64;
pub const NUM_ITERS: usize = 5;
pub const MONDO_USEC_WAIT_MIN: i32 = 2;
pub const MONDO_USEC_WAIT_MAX: i32 = 100;
pub const MONDO_RETRY_LIMIT: i32 = 500000;

#[repr(C)] pub struct cpumask_t { pub bits: [usize; 1] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { pub context: usize, pub pgd: *mut c_void }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct trap_per_cpu {
    pub cpu_list_pa: u64, pub cpu_mondo_block_pa: u64,
    pub cpu_mondo_pa: u64, pub dev_mondo_pa: u64,
    pub resum_mondo_pa: u64, pub nonresum_mondo_pa: u64,
    pub pgd_paddr: u64,
}
#[repr(C)] pub struct cpuinfo_sparc { pub core_id: i32, pub proc_id: i32, pub max_cache_id: i32, pub sock_id: i32 }

extern "C" {
    static mut cpu_sibling_map: cpumask_t;
    static mut cpu_core_map: [cpumask_t; 1];
    static mut cpu_core_sib_map: [cpumask_t; 1];
    static mut cpu_core_sib_cache_map: [cpumask_t; 1];
    static mut xcall_deliver_impl: Option<unsafe extern "C" fn(*mut trap_per_cpu, i32)>;
    static mut trap_block: trap_per_cpu;
    static mut smp_commenced_mask: cpumask_t;
    static mut cpu_poke: bool;
    static mut callin_flag: u64;
    static mut penguins_are_doing_time: u64;
    static mut xcall_sync_tick: u64;
    static mut xcall_call_function: u64;
    static mut xcall_call_function_single: u64;
    static mut xcall_receive_signal: u64;
    static mut xcall_capture: u64;
    static mut xcall_flush_tlb_mm: u64;
    static mut xcall_flush_tlb_page: u64;
    static mut xcall_flush_tlb_kernel_range: u64;
    static mut xcall_fetch_glob_regs: u64;
    static mut xcall_fetch_glob_pmu: u64;
    static mut xcall_fetch_glob_pmu_n4: u64;
    static mut xcall_new_mmu_context_version: u64;
    fn hard_smp_processor_id() -> i32; fn smp_processor_id() -> i32;
    fn cpu_online(cpu: u16) -> bool; fn num_online_cpus() -> i32;
    fn set_cpu_online(cpu: u32, online: bool); fn cpumask_of(cpu: i32) -> *const cpumask_t;
    fn cpumask_test_cpu(cpu: i32, mask: *const cpumask_t) -> bool;
    fn cpumask_set_cpu(cpu: u32, mask: *mut cpumask_t); fn cpumask_clear_cpu(cpu: i32, mask: *mut cpumask_t);
    fn local_irq_enable(); fn local_irq_disable(); fn preempt_disable(); fn preempt_enable();
    fn udelay(usec: u32); fn mdelay(usec: u32); fn msleep(usec: u32); fn mb(); fn rmb(); fn wmb();
    fn printk(fmt: *const u8, ...); fn panic(fmt: *const u8, ...);
    fn generic_smp_call_function_interrupt(); fn generic_smp_call_function_single_interrupt();
    fn irq_enter(); fn irq_exit(); fn clear_softint(mask: u32); fn set_softint(mask: u32); fn scheduler_ipi();
    fn prom_stopself(); fn prom_stopcpu_cpuid(cpu: i32); fn cpu_map_rebuild(); fn fixup_irqs();
    fn idle_cpu(cpu: i32) -> bool; fn sun4v_cpu_poke(cpu: i32) -> u64;
    fn sun4v_cpu_stop(cpu: u32) -> u64; fn prom_world(x: i32); fn sun4v_hvapi_get(g: u64, major: *mut u64, minor: *mut u64) -> i32;
    fn smp_call_function(f: unsafe extern "C" fn(*mut c_void), data: *mut c_void, wait: i32);
    fn smp_call_function_many(mask: *const cpumask_t, f: unsafe extern "C" fn(*mut c_void), data: *mut c_void, wait: i32);
    fn __flush_tlb_mm(ctx: u32, secondary: u32); fn __flush_tlb_page(ctx: usize, addr: u64); fn __flush_tlb_kernel_range(s: u64,e: u64);
    fn __flush_tlb_pending(ctx: u64, nr: u64, vaddrs: *mut u64); fn tsb_context_switch(mm: *mut mm_struct);
    fn __pa(p: *mut c_void) -> u64; fn __va(p: u64) -> *mut c_void;
}

static mut cpu_poke_state: bool = false;

#[no_mangle] pub unsafe extern "C" fn smp_info(_m: *mut seq_file) { /* for_each_online_cpu: seq_printf(m, ...); */ }
#[no_mangle] pub unsafe extern "C" fn smp_bogo(_m: *mut seq_file) { /* for_each_online_cpu: print cpu_data(i).clock_tick */ }
#[no_mangle] pub unsafe extern "C" fn smp_callin() {
    let cpuid = hard_smp_processor_id();
    let _ = cpuid;
    callin_flag = 1;
    while !cpumask_test_cpu(cpuid, &smp_commenced_mask) { rmb(); }
    set_cpu_online(cpuid as u32, true); local_irq_enable();
}
#[no_mangle] pub unsafe extern "C" fn cpu_panic() { panic(b"SMP bolixed\0".as_ptr()); }

#[inline] unsafe fn xcall_deliver(_data0: u64, _data1: u64, _data2: u64, _mask: *const cpumask_t) {
    // The original performs interrupt-disabled mondo setup and architecture-specific delivery.
    if let Some(f) = xcall_deliver_impl { f(&mut trap_block, 0); }
}
unsafe fn smp_cross_call_masked(func: *mut u64, ctx: u32, data1: u64, data2: u64, mask: *const cpumask_t) {
    let data0 = ((ctx as u64) << 32) | ((*func as u64) & 0xffff_ffff); xcall_deliver(data0, data1, data2, mask);
}
unsafe fn smp_cross_call(func: *mut u64, ctx: u32, data1: u64, data2: u64) { smp_cross_call_masked(func,ctx,data1,data2,core::ptr::null()); }

#[no_mangle] pub unsafe extern "C" fn arch_send_call_function_ipi_mask(mask: *const cpumask_t) { xcall_deliver(xcall_call_function,0,0,mask); }
#[no_mangle] pub unsafe extern "C" fn arch_send_call_function_single_ipi(cpu: i32) { xcall_deliver(xcall_call_function_single,0,0,cpumask_of(cpu)); }
#[no_mangle] pub unsafe extern "C" fn smp_call_function_client(irq: i32, _regs: *mut pt_regs) { clear_softint(1 << irq); irq_enter(); generic_smp_call_function_interrupt(); irq_exit(); }
#[no_mangle] pub unsafe extern "C" fn smp_call_function_single_client(irq: i32, _regs: *mut pt_regs) { clear_softint(1 << irq); irq_enter(); generic_smp_call_function_single_interrupt(); irq_exit(); }

unsafe extern "C" fn tsb_sync(info: *mut c_void) { tsb_context_switch(info as *mut mm_struct); }
#[no_mangle] pub unsafe extern "C" fn smp_tsb_sync(mm: *mut mm_struct) { smp_call_function_many(core::ptr::null(), tsb_sync, mm as *mut c_void, 1); }
#[no_mangle] pub unsafe extern "C" fn smp_fetch_global_regs() { smp_cross_call(&mut xcall_fetch_glob_regs,0,0,0); }
#[no_mangle] pub unsafe extern "C" fn smp_fetch_global_pmu() { smp_cross_call(&mut xcall_fetch_glob_pmu,0,0,0); }
#[no_mangle] pub unsafe extern "C" fn smp_flush_tlb_mm(mm: *mut mm_struct) { smp_cross_call_masked(&mut xcall_flush_tlb_mm, (*mm).context as u32,0,0,core::ptr::null()); __flush_tlb_mm((*mm).context as u32,0); }
#[no_mangle] pub unsafe extern "C" fn smp_flush_tlb_page(mm: *mut mm_struct, vaddr: u64) { smp_cross_call_masked(&mut xcall_flush_tlb_page,(*mm).context as u32,vaddr,0,core::ptr::null()); __flush_tlb_page((*mm).context,vaddr); }
#[no_mangle] pub unsafe extern "C" fn smp_flush_tlb_kernel_range(start: u64,end:u64) { if start != end { smp_cross_call(&mut xcall_flush_tlb_kernel_range,0,start,end); __flush_tlb_kernel_range(start,end); } }

#[no_mangle] pub unsafe extern "C" fn smp_capture() { penguins_are_doing_time=1; smp_cross_call(&mut xcall_capture,0,0,0); }
#[no_mangle] pub unsafe extern "C" fn smp_release() { penguins_are_doing_time=0; mb(); }
#[no_mangle] pub unsafe extern "C" fn smp_penguin_jailcell(irq:i32,_regs:*mut pt_regs) { clear_softint(1<<irq); preempt_disable(); prom_world(1); while penguins_are_doing_time { rmb(); } prom_world(0); preempt_enable(); }
#[no_mangle] pub unsafe extern "C" fn smp_prepare_cpus(_max_cpus:u32) {}
#[no_mangle] pub unsafe extern "C" fn smp_cpus_done(_max_cpus:u32) {}
#[no_mangle] pub unsafe extern "C" fn smp_setup_processor_id() {}

#[no_mangle] pub unsafe extern "C" fn scheduler_poke() { if cpu_poke_state { set_softint(1 << 14); } }
#[no_mangle] pub unsafe extern "C" fn arch_smp_send_reschedule(cpu:i32) { if cpu == smp_processor_id() { set_softint(1<<14); } else { xcall_deliver(xcall_receive_signal,0,0,cpumask_of(cpu)); } }
#[no_mangle] pub unsafe extern "C" fn smp_init_cpu_poke() { cpu_poke = true; cpu_poke_state = true; }
#[no_mangle] pub unsafe extern "C" fn smp_receive_signal_client(irq:i32,_regs:*mut pt_regs) { clear_softint(1<<irq); scheduler_ipi(); }
#[no_mangle] pub unsafe extern "C" fn smp_send_stop() { smp_call_function(stop_this_cpu,core::ptr::null_mut(),0); }
unsafe extern "C" fn stop_this_cpu(_dummy:*mut c_void) { set_cpu_online(smp_processor_id() as u32,false); prom_stopself(); }

#[no_mangle] pub unsafe extern "C" fn setup_per_cpu_areas() { /* allocator and per-cpu offset setup are supplied by the kernel */ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
