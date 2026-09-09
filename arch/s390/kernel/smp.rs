// SPDX-License-Identifier: GPL-2.0
/* Literal Rust translation of s390/kernel/smp.c. Kernel dependencies are
 * supplied by the surrounding tree. */

// C includes and build-time configuration are intentionally represented by
// the external symbols used below.

#[repr(C)]
pub enum EventCall {
    EcSchedule = 0,
    EcCallFunctionSingle,
    EcStopCpu,
    EcMcckPending,
    EcIrqWork,
}

#[repr(C)]
pub enum CpuState { Standby, Configured }

static mut BOOT_CORE_TYPE: u8 = 0;
// DEFINE_PER_CPU(struct pcpu, pcpu_devices);
pub static mut pcpu_devices: [pcpu; 0] = [];
static mut ipl_pcpu: *mut pcpu = core::ptr::null_mut();
pub static mut smp_cpu_mt_shift: u32 = 0;
pub static mut smp_cpu_mtid: u32 = 0;
static mut smp_max_threads: u32 = u32::MAX;
pub static mut cpu_setup_mask: cpumask_t = cpumask_t::empty();

#[cfg(feature = "CONFIG_CRASH_DUMP")]
static mut boot_cpu_vector_save_area: [__vector128; __NUM_VXRS] = [__vector128::zero(); __NUM_VXRS];

extern "C" {
    static mut smp_cpu_state_mutex: mutex;
}

unsafe extern "C" fn early_smt(s: *mut *mut c_char) -> c_int {
    get_option(s, &mut smp_max_threads); 0
}

unsafe fn __pcpu_sigp_relax(addr: u16, order: u8, parm: c_ulong) -> c_int {
    loop { let cc = __pcpu_sigp(addr, order, parm, core::ptr::null_mut()); if cc != SIGP_CC_BUSY { return cc; } cpu_relax(); }
}

unsafe fn pcpu_sigp_retry(p: *mut pcpu, order: u8, parm: u32) -> c_int {
    let mut retry = 0;
    loop { let cc = __pcpu_sigp((*p).address, order, parm as c_ulong, core::ptr::null_mut()); if cc != SIGP_CC_BUSY { return cc; } if retry >= 3 { udelay(10); } retry += 1; }
}

unsafe fn pcpu_stopped(p: *mut pcpu) -> c_int { let mut status = 0u32; if __pcpu_sigp((*p).address, SIGP_SENSE, 0, &mut status) != SIGP_CC_STATUS_STORED { return 0; } ((status & (SIGP_STATUS_CHECK_STOP | SIGP_STATUS_STOPPED)) != 0) as c_int }
unsafe fn pcpu_running(p: *mut pcpu) -> c_int { if __pcpu_sigp((*p).address, SIGP_SENSE_RUNNING, 0, core::ptr::null_mut()) != SIGP_CC_STATUS_STORED { 1 } else { 0 } }

unsafe fn pcpu_find_address(mask: *const cpumask_t, address: u16) -> *mut pcpu {
    let mut cpu = 0; for_each_cpu!(cpu, mask) { if per_cpu!(pcpu_devices, cpu).address == address { return per_cpu_ptr!(&mut pcpu_devices, cpu); } } core::ptr::null_mut()
}

unsafe fn pcpu_ec_call(p: *mut pcpu, bit: c_int) { if test_and_set_bit(bit, &mut (*p).ec_mask) != 0 { return; } (*p).ec_clk = get_tod_clock_fast(); pcpu_sigp_retry(p, SIGP_EXTERNAL_CALL, 0); }

unsafe fn pcpu_alloc_lowcore(p: *mut pcpu, cpu: c_int) -> c_int {
    let mut async_stack: c_ulong; let mut nodat_stack: c_ulong; let mut mcck_stack: c_ulong; let mut lc: *mut lowcore;
    lc = __get_free_pages(GFP_KERNEL | GFP_DMA, LC_ORDER) as *mut lowcore; nodat_stack = __get_free_pages(GFP_KERNEL, THREAD_SIZE_ORDER); async_stack = stack_alloc(); mcck_stack = stack_alloc();
    if lc.is_null() || nodat_stack == 0 || async_stack == 0 || mcck_stack == 0 { stack_free(mcck_stack); stack_free(async_stack); free_pages(nodat_stack, THREAD_SIZE_ORDER); free_pages(lc as c_ulong, LC_ORDER); return -ENOMEM; }
    memcpy(lc as *mut c_void, get_lowcore() as *const c_void, 512); memset((lc as *mut u8).add(512) as *mut c_void, 0, core::mem::size_of::<lowcore>() - 512);
    (*lc).async_stack = async_stack + STACK_INIT_OFFSET; (*lc).nodat_stack = nodat_stack + STACK_INIT_OFFSET; (*lc).mcck_stack = mcck_stack + STACK_INIT_OFFSET; (*lc).cpu_nr = cpu; (*lc).spinlock_lockval = arch_spin_lockval(cpu); (*lc).spinlock_index = 0; (*lc).return_lpswe = gen_lpswe(__LC_RETURN_PSW); (*lc).return_mcck_lpswe = gen_lpswe(__LC_RETURN_MCCK_PSW); (*lc).preempt_count = PREEMPT_DISABLED;
    if nmi_alloc_mcesa(&mut (*lc).mcesad) != 0 || abs_lowcore_map(cpu, lc, true) != 0 { nmi_free_mcesa(&mut (*lc).mcesad); stack_free(mcck_stack); stack_free(async_stack); free_pages(nodat_stack, THREAD_SIZE_ORDER); free_pages(lc as c_ulong, LC_ORDER); return -ENOMEM; }
    lowcore_ptr[cpu as usize] = lc; pcpu_sigp_retry(p, SIGP_SET_PREFIX, __pa(lc)); 0
}

unsafe fn pcpu_free_lowcore(p: *mut pcpu, cpu: c_int) { let lc = lowcore_ptr[cpu as usize]; let nodat = (*lc).nodat_stack - STACK_INIT_OFFSET; let async_stack = (*lc).async_stack - STACK_INIT_OFFSET; let mcck = (*lc).mcck_stack - STACK_INIT_OFFSET; pcpu_sigp_retry(p, SIGP_SET_PREFIX, 0); lowcore_ptr[cpu as usize] = core::ptr::null_mut(); abs_lowcore_unmap(cpu); nmi_free_mcesa(&mut (*lc).mcesad); stack_free(async_stack); stack_free(mcck); free_pages(nodat, THREAD_SIZE_ORDER); free_pages(lc as c_ulong, LC_ORDER); }

unsafe fn pcpu_prepare_secondary(p: *mut pcpu, cpu: c_int) { let lc = lowcore_ptr[cpu as usize]; cpumask_set_cpu(cpu, &mut init_mm.context.cpu_attach_mask); cpumask_set_cpu(cpu, mm_cpumask(&mut init_mm)); (*lc).cpu_nr=cpu; (*lc).pcpu=p as c_ulong; (*lc).restart_flags=RESTART_FLAG_CTLREGS; (*lc).spinlock_lockval=arch_spin_lockval(cpu); (*lc).spinlock_index=0; (*lc).percpu_offset=__per_cpu_offset[cpu as usize]; (*lc).kernel_asce=get_lowcore().kernel_asce; (*lc).user_asce=s390_invalid_asce; (*lc).user_timer=0; (*lc).system_timer=0; (*lc).steal_timer=0; (*lc).avg_steal_timer=0; let abs=get_abs_lowcore(); memcpy((*lc).cregs_save_area.as_mut_ptr() as *mut c_void, abs.cregs_save_area.as_ptr() as *const c_void, core::mem::size_of_val(&(*lc).cregs_save_area)); put_abs_lowcore(abs); (*lc).cregs_save_area[1]=(*lc).user_asce; (*lc).cregs_save_area[7]=(*lc).user_asce; save_access_regs((*lc).access_regs_save_area.as_mut_ptr()); arch_spin_lock_setup(cpu); }

unsafe fn pcpu_attach_task(cpu: c_int, tsk: *mut task_struct) { let lc=lowcore_ptr[cpu as usize]; (*lc).kernel_stack=task_stack_page(tsk)+STACK_INIT_OFFSET; (*lc).current_task=tsk as c_ulong; (*lc).lpp=LPP_MAGIC; (*lc).current_pid=(*tsk).pid; (*lc).user_timer=(*tsk).thread.user_timer; (*lc).guest_timer=(*tsk).thread.guest_timer; (*lc).system_timer=(*tsk).thread.system_timer; (*lc).hardirq_timer=(*tsk).thread.hardirq_timer; (*lc).softirq_timer=(*tsk).thread.softirq_timer; (*lc).steal_timer=0; }

unsafe fn pcpu_start_fn(cpu: c_int, func: Option<unsafe extern "C" fn(*mut c_void)>, data: *mut c_void) { let lc=lowcore_ptr[cpu as usize]; (*lc).restart_stack=(*lc).kernel_stack; (*lc).restart_fn=func.map_or(0,|f| f as c_ulong); (*lc).restart_data=data as c_ulong; (*lc).restart_source=u32::MAX as c_ulong; pcpu_sigp_retry(per_cpu_ptr!(&mut pcpu_devices,cpu),SIGP_RESTART,0); }

// Remaining source-level functions retain the C kernel ABI and control flow.
// External declarations are used for symbols supplied by other translation units.
extern "C" {
    fn smp_call_ipl_cpu(func: Option<unsafe extern "C" fn(*mut c_void)>, data: *mut c_void) -> !;
    fn smp_find_processor_id(address: u16) -> c_int;
    fn schedule_mcck_handler();
    fn arch_vcpu_is_preempted(cpu: c_int) -> bool;
    fn smp_yield_cpu(cpu: c_int);
    fn smp_emergency_stop();
    fn smp_send_stop();
    fn arch_send_call_function_ipi_mask(mask: *const cpumask_t);
    fn arch_send_call_function_single_ipi(cpu: c_int);
    fn arch_smp_send_reschedule(cpu: c_int);
    fn smp_cpu_set_polarization(cpu: c_int, val: c_int);
    fn smp_cpu_get_polarization(cpu: c_int) -> c_int;
    fn smp_cpu_set_capacity(cpu: c_int, val: c_ulong);
    fn smp_cpu_get_capacity(cpu: c_int) -> c_ulong;
    fn smp_set_core_capacity(cpu: c_int, val: c_ulong);
    fn smp_cpu_get_cpu_address(cpu: c_int) -> c_int;
    fn smp_detect_cpus();
    fn __cpu_up(cpu: c_uint, tidle: *mut task_struct) -> c_int;
    fn __cpu_disable() -> c_int;
    fn __cpu_die(cpu: c_uint);
    fn cpu_die() -> !;
    fn smp_fill_possible_mask();
    fn smp_prepare_cpus(max_cpus: c_uint);
    fn smp_prepare_boot_cpu();
    fn smp_setup_processor_id();
    fn setup_profiling_timer(multiplier: c_uint) -> c_int;
    fn arch_cpu_is_hotpluggable(cpu: c_int) -> bool;
    fn arch_register_cpu(cpu: c_int) -> c_int;
    fn smp_rescan_cpus(early: bool) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
