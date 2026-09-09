// SPDX-License-Identifier: GPL-2.0-or-later
// Direct low-level Rust translation of smp.c. Kernel-provided symbols and
// configuration-dependent declarations are intentionally left external.

use core::ffi::c_void;

#[repr(C)]
pub struct cpumask_t { pub bits: [usize; 1] }
#[repr(C)] pub struct cpumask { pub bits: [usize; 1] }
#[repr(C)] pub struct irq_desc;
#[repr(C)] pub struct irq_domain;
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct task_struct;
#[repr(C)] pub struct mm_struct { pub mm_users: atomic_t }
#[repr(C)] pub struct vm_area_struct { pub vm_mm: *mut mm_struct, pub vm_flags: usize }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct plat_smp_ops {
    pub init_secondary: unsafe extern "C" fn(),
    pub smp_finish: unsafe extern "C" fn(),
    pub prepare_cpus: unsafe extern "C" fn(u32),
    pub prepare_boot_cpu: Option<unsafe extern "C" fn()>,
    pub boot_secondary: unsafe extern "C" fn(u32, *mut task_struct) -> i32,
    pub cleanup_dead_cpu: Option<unsafe extern "C" fn(u32)>,
}
#[repr(C)] pub struct flush_tlb_data { pub vma: *mut vm_area_struct, pub addr1: usize, pub addr2: usize }

extern "C" {
    static mut cpu_sibling_map: [cpumask_t; 1]; static mut cpu_core_map: [cpumask_t; 1];
    static mut cpu_foreign_map: [cpumask_t; 1]; static mut cpu_coherent_mask: cpumask_t;
    static mut mp_ops: *const plat_smp_ops; static mut smp_num_siblings: i32;
    static mut cpu_has_mmid: bool; static mut current: *mut task_struct;
    static cpu_possible_mask: *const cpumask;
    fn cpumask_set_cpu(cpu: i32, mask: *mut cpumask_t); fn cpumask_clear(mask: *mut cpumask_t);
    fn cpumask_andnot(dst: *mut cpumask_t, a: *const cpumask_t, b: *const cpumask_t);
    fn cpumask_test_cpu(cpu: i32, mask: *const cpumask_t) -> bool;
    fn cpus_are_siblings(a: i32, b: i32) -> bool; fn cpu_core(data: *const c_void) -> u32;
    fn smp_processor_id() -> i32; fn raw_smp_processor_id() -> u32;
    fn cpu_probe(); fn per_cpu_trap_init(hotplug: bool); fn rcutree_report_cpu_starting(cpu: u32);
    fn mips_clockevent_init(); fn cpu_report(); fn maar_init(); fn calibrate_delay();
    fn set_cpu_online(cpu: u32, online: bool); fn set_cpu_possible(cpu: u32, possible: bool);
    fn notify_cpu_starting(cpu: u32); fn synchronise_count_slave(cpu: u32); fn cpu_startup_entry(state: u32);
    fn local_irq_disable(); fn local_irq_save(flags: *mut usize); fn local_irq_restore(flags: usize);
    fn local_flush_tlb_all(); fn local_flush_tlb_range(vma: *mut vm_area_struct, a: usize, b: usize);
    fn local_flush_tlb_kernel_range(a: usize, b: usize); fn local_flush_tlb_page(vma: *mut vm_area_struct, p: usize);
    fn local_flush_tlb_one(v: usize); fn on_each_cpu(f: unsafe extern "C" fn(*mut c_void), i: *mut c_void, wait: i32);
    fn smp_call_function(f: unsafe extern "C" fn(*mut c_void), i: *mut c_void, wait: i32);
    fn smp_call_function_single_async(cpu: i32, csd: *mut c_void);
    fn preempt_disable(); fn preempt_enable(); fn atomic_read(a: *const atomic_t) -> i32;
    fn cpu_context(cpu: u32, mm: *mut mm_struct) -> u32; fn set_cpu_context(cpu: u32, mm: *mut mm_struct, v: u32);
    fn drop_mmu_context(mm: *mut mm_struct); fn cpu_asid(cpu: u32, mm: *mut mm_struct) -> u32;
    fn read_c0_memorymapid() -> u32; fn write_c0_memorymapid(v: u32); fn mtc0_tlbw_hazard(); fn instruction_hazard();
    fn htw_stop(); fn htw_start(); fn ginvt_full(); fn sync_ginv(); fn ginvt_va_mmid(v: usize);
    fn round_down(v: usize, a: usize) -> usize; fn round_up(v: usize, a: usize) -> usize;
    fn calculate_cpu_foreign_map(); fn mips_smp_send_ipi_mask(mask: *const cpumask, action: u32);
    fn scheduler_ipi(); fn generic_smp_call_function_interrupt(); fn tick_receive_broadcast();
    fn mp_cleanup_dead_cpu(cpu: u32);
}

#[no_mangle] pub static mut __cpu_number_map: [i32; 1] = [0; 1];
#[no_mangle] pub static mut __cpu_logical_map: [i32; 1] = [0; 1];
#[no_mangle] pub static mut smp_max_threads: u32 = u32::MAX;

unsafe extern "C" fn set_cpu_sibling_map(cpu: i32) {
    cpumask_set_cpu(cpu, &mut CPU_SIBLING_SETUP_MAP);
    if smp_num_siblings > 1 { for i in 0..1 { if cpus_are_siblings(cpu, i) { cpumask_set_cpu(i, &mut cpu_sibling_map[0]); cpumask_set_cpu(cpu, &mut cpu_sibling_map[0]); } } }
    else { cpumask_set_cpu(cpu, &mut cpu_sibling_map[0]); }
}
unsafe extern "C" fn set_cpu_core_map(cpu: i32) { cpumask_set_cpu(cpu, &mut CPU_CORE_SETUP_MAP); cpumask_set_cpu(cpu, &mut cpu_core_map[0]); }
static mut CPU_SIBLING_SETUP_MAP: cpumask_t = cpumask_t { bits: [0] };
static mut CPU_CORE_SETUP_MAP: cpumask_t = cpumask_t { bits: [0] };

#[no_mangle] pub unsafe extern "C" fn calculate_cpu_foreign_map() { let mut t = cpumask_t { bits: [0] }; cpumask_clear(&mut t); cpumask_andnot(&mut cpu_foreign_map[0], &t, &cpu_sibling_map[0]); }
#[no_mangle] pub unsafe extern "C" fn register_smp_ops(ops: *const plat_smp_ops) { mp_ops = ops; }
#[no_mangle] pub unsafe extern "C" fn mips_smp_send_ipi_single(cpu: i32, action: u32) { mips_smp_send_ipi_mask(core::ptr::null(), action); let _ = cpu; }
#[no_mangle] pub unsafe extern "C" fn start_secondary() { let cpu = raw_smp_processor_id(); cpu_probe(); per_cpu_trap_init(false); rcutree_report_cpu_starting(cpu); mips_clockevent_init(); ((*mp_ops).init_secondary)(); cpu_report(); maar_init(); calibrate_delay(); set_cpu_sibling_map(cpu as i32); set_cpu_core_map(cpu as i32); cpumask_set_cpu(cpu as i32, &mut cpu_coherent_mask); notify_cpu_starting(cpu); synchronise_count_slave(cpu); set_cpu_online(cpu, true); calculate_cpu_foreign_map(); ((*mp_ops).smp_finish)(); cpu_startup_entry(0); }
unsafe extern "C" fn flush_tlb_all_ipi(_: *mut c_void) { local_flush_tlb_all(); }
#[no_mangle] pub unsafe extern "C" fn flush_tlb_all() { if cpu_has_mmid { htw_stop(); ginvt_full(); sync_ginv(); instruction_hazard(); htw_start(); } else { on_each_cpu(flush_tlb_all_ipi, core::ptr::null_mut(), 1); } }
unsafe extern "C" fn flush_tlb_one_ipi(info: *mut c_void) { local_flush_tlb_one(info as usize); }
#[no_mangle] pub unsafe extern "C" fn flush_tlb_one(vaddr: usize) { on_each_cpu(flush_tlb_one_ipi, vaddr as *mut c_void, 1); }
#[no_mangle] pub unsafe extern "C" fn flush_tlb_page(vma: *mut vm_area_struct, page: usize) { local_flush_tlb_page(vma, page); }
#[no_mangle] pub unsafe extern "C" fn smp_send_stop() { }
#[no_mangle] pub unsafe extern "C" fn smp_cpus_done(_: u32) { }
#[no_mangle] pub unsafe extern "C" fn smp_prepare_cpus(max_cpus: u32) { ((*mp_ops).prepare_cpus)(max_cpus); set_cpu_sibling_map(0); set_cpu_core_map(0); calculate_cpu_foreign_map(); }
#[no_mangle] pub unsafe extern "C" fn smp_prepare_boot_cpu() { if let Some(f) = (*mp_ops).prepare_boot_cpu { f(); } set_cpu_possible(0, true); set_cpu_online(0, true); }

#[no_mangle] pub unsafe extern "C" fn flush_tlb_mm(mm: *mut mm_struct) {
    if mm.is_null() || atomic_read(&(*mm).mm_users) == 0 { return; }
    preempt_disable(); drop_mmu_context(mm); preempt_enable();
}
unsafe extern "C" fn flush_tlb_range_ipi(info: *mut c_void) { let fd = &*(info as *const flush_tlb_data); local_flush_tlb_range(fd.vma, fd.addr1, fd.addr2); }
#[no_mangle] pub unsafe extern "C" fn flush_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize) {
    let fd = flush_tlb_data { vma, addr1: start, addr2: end }; preempt_disable();
    if cpu_has_mmid { htw_stop(); let old = read_c0_memorymapid(); write_c0_memorymapid(cpu_asid(0, (*vma).vm_mm)); mtc0_tlbw_hazard(); let mut addr = round_down(start, 8192); let limit = round_up(end, 8192); while addr < limit { ginvt_va_mmid(addr); sync_ginv(); addr += 8192; } write_c0_memorymapid(old); instruction_hazard(); htw_start(); }
    else { on_each_cpu(flush_tlb_range_ipi, &fd as *const _ as *mut c_void, 1); local_flush_tlb_range(vma, start, end); }
    preempt_enable();
}
unsafe extern "C" fn flush_tlb_kernel_range_ipi(info: *mut c_void) { let fd = &*(info as *const flush_tlb_data); local_flush_tlb_kernel_range(fd.addr1, fd.addr2); }
#[no_mangle] pub unsafe extern "C" fn flush_tlb_kernel_range(start: usize, end: usize) { let fd = flush_tlb_data { vma: core::ptr::null_mut(), addr1: start, addr2: end }; on_each_cpu(flush_tlb_kernel_range_ipi, &fd as *const _ as *mut c_void, 1); }
unsafe extern "C" fn flush_tlb_page_ipi(info: *mut c_void) { let fd = &*(info as *const flush_tlb_data); local_flush_tlb_page(fd.vma, fd.addr1); }
#[no_mangle] pub unsafe extern "C" fn arch_cpuhp_cleanup_dead_cpu(cpu: u32) { if let Some(f) = (*mp_ops).cleanup_dead_cpu { f(cpu); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
