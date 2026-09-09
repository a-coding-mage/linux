// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/smp.c
 *
 * SMP support for the SuperH processors.
 *
 * Copyright (C) 2002 - 2010 Paul Mundt
 * Copyright (C) 2006 - 2007 Akio Idehara
 */

// Linux and SuperH header dependencies are supplied by the surrounding build.

extern "C" {
    static mut __cpu_number_map: [i32; NR_CPUS];
    static mut __cpu_logical_map: [i32; NR_CPUS];
    static mut mp_ops: *mut plat_smp_ops;
    static mut cpu_state: PerCpu<i32>;
    static mut cpu_data: *mut sh_cpuinfo;
    static mut boot_cpu_data: sh_cpuinfo;
    static mut loops_per_jiffy: u64;
    static mut init_mm: mm_struct;
    static mut current: *mut task_struct;
    static mut system_state: i32;
    static mut jiffies: c_ulong;

    fn printk(fmt: *const c_char, ...);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn smp_processor_id() -> c_uint;
    fn init_new_context(task: *mut task_struct, mm: *mut mm_struct);
    fn current_thread_info() -> *mut thread_info;
    fn init_cpu_present(mask: *const cpumask);
    fn set_cpu_online(cpu: c_uint, online: bool);
    fn set_cpu_possible(cpu: c_uint, possible: bool);
    fn smp_rmb();
    fn msleep(ms: c_uint);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn idle_task_exit();
    fn irq_ctx_exit(cpu: c_uint);
    fn mb();
    fn local_irq_disable();
    fn migrate_irqs();
    fn flush_cache_all();
    fn local_flush_tlb_all();
    fn clear_tasks_mm_cpumask(cpu: c_uint);
    fn BUG() -> !;
    fn enable_mmu();
    fn mmgrab(mm: *mut mm_struct);
    fn mmget(mm: *mut mm_struct);
    fn enter_lazy_tlb(mm: *mut mm_struct, task: *mut task_struct);
    fn per_cpu_trap_init();
    fn notify_cpu_starting(cpu: c_uint);
    fn local_irq_enable();
    fn calibrate_delay();
    fn cpu_startup_entry(state: c_int);
    fn flush_icache_range(start: c_ulong, end: c_ulong);
    fn wmb();
    fn time_before(a: c_ulong, b: c_ulong) -> bool;
    fn cpu_online(cpu: c_uint) -> bool;
    fn udelay(us: c_uint);
    fn barrier();
    fn num_online_cpus() -> c_int;
    fn smp_call_function(func: unsafe extern "C" fn(*mut c_void), info: *mut c_void, wait: c_int);
    fn stop_this_cpu(info: *mut c_void);
    fn for_each_online_cpu_body(cpu: *mut c_int);
    fn for_each_cpu_body(cpu: *mut c_int, mask: *const cpumask);
    fn irq_enter();
    fn tick_receive_broadcast();
    fn irq_exit();
    fn generic_smp_call_function_interrupt();
    fn scheduler_ipi();
    fn generic_smp_call_function_single_interrupt();
    fn preempt_disable();
    fn preempt_enable();
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn local_flush_tlb_mm(mm: *mut mm_struct);
    fn local_flush_tlb_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong);
    fn local_flush_tlb_kernel_range(start: c_ulong, end: c_ulong);
    fn local_flush_tlb_page(vma: *mut vm_area_struct, page: c_ulong);
    fn local_flush_tlb_one(asid: c_ulong, vaddr: c_ulong);
    fn on_each_cpu(func: unsafe extern "C" fn(*mut c_void), info: *mut c_void, wait: c_int);
}

type c_int = i32;
type c_uint = u32;
type c_ulong = usize;
type c_char = i8;
type c_void = core::ffi::c_void;

#[repr(C)] pub struct plat_smp_ops { pub prepare_cpus: Option<unsafe extern "C" fn(c_uint)>, pub cpu_disable: Option<unsafe extern "C" fn(c_uint) -> c_int>, pub start_cpu: Option<unsafe extern "C" fn(c_uint, c_ulong)>, pub send_ipi: Option<unsafe extern "C" fn(c_int, c_int)> }
#[repr(C)] pub struct sh_cpuinfo { pub loops_per_jiffy: u64 }
#[repr(C)] pub struct mm_struct { pub mm_users: atomic_t }
#[repr(C)] pub struct task_struct { pub active_mm: *mut mm_struct, pub mm: *mut mm_struct, pub stack: *mut c_void, pub thread: thread_struct }
#[repr(C)] pub struct thread_struct { pub sp: c_ulong }
#[repr(C)] pub struct thread_info { pub cpu: c_uint }
#[repr(C)] pub struct vm_area_struct { pub vm_mm: *mut mm_struct }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct PerCpu<T>(pub T);
#[repr(C)] pub struct StackStart { pub sp: c_ulong, pub bss_start: c_ulong, pub bss_end: c_ulong, pub start_kernel_fn: *mut c_void, pub cpu_init_fn: *mut c_void, pub thread_info: *mut c_void }

const EPERM: c_int = 1;
const ENOSYS: c_int = 38;
const ENOENT: c_int = 2;
const CPU_DEAD: i32 = 0;
const CPU_ONLINE: i32 = 1;
const CPU_UP_PREPARE: i32 = 2;
const SMP_MSG_RESCHEDULE: c_int = 0;
const SMP_MSG_FUNCTION: c_int = 1;
const SMP_MSG_FUNCTION_SINGLE: c_int = 2;
const SMP_MSG_TIMER: c_int = 3;

static mut stack_start: StackStart = StackStart { sp: 0, bss_start: 0, bss_end: 0, start_kernel_fn: core::ptr::null_mut(), cpu_init_fn: core::ptr::null_mut(), thread_info: core::ptr::null_mut() };

pub unsafe extern "C" fn register_smp_ops(ops: *mut plat_smp_ops) { if !mp_ops.is_null() { printk(b"Overriding previously set SMP ops\n\0".as_ptr() as *const c_char); } mp_ops = ops; }

unsafe fn smp_store_cpu_info(cpu: c_uint) { let c = cpu_data.add(cpu as usize); memcpy(c as *mut c_void, &boot_cpu_data as *const _ as *const c_void, core::mem::size_of::<sh_cpuinfo>()); (*c).loops_per_jiffy = loops_per_jiffy; }

pub unsafe extern "C" fn smp_prepare_cpus(max_cpus: c_uint) { let cpu = smp_processor_id(); init_new_context(current, &mut init_mm); (*current_thread_info()).cpu = cpu; ((*mp_ops).prepare_cpus.unwrap())(max_cpus); /* !CONFIG_HOTPLUG_CPU: initialize cpu present mask. */ }
pub unsafe extern "C" fn smp_prepare_boot_cpu() { let cpu = smp_processor_id(); __cpu_number_map[0] = cpu as i32; __cpu_logical_map[0] = cpu as i32; set_cpu_online(cpu, true); set_cpu_possible(cpu, true); /* per_cpu(cpu_state, cpu) = CPU_ONLINE; */ }

pub unsafe extern "C" fn native_cpu_disable(cpu: c_uint) -> c_int { if cpu == 0 { -EPERM } else { 0 } }
pub unsafe extern "C" fn native_cpu_die(_cpu: c_uint) { BUG(); }
pub unsafe extern "C" fn native_play_dead() { BUG(); }

unsafe extern "C" fn start_secondary() { let cpu = smp_processor_id(); let mm = &mut init_mm; enable_mmu(); mmgrab(mm); mmget(mm); (*current).active_mm = mm; enter_lazy_tlb(mm, current); local_flush_tlb_all(); per_cpu_trap_init(); notify_cpu_starting(cpu); local_irq_enable(); calibrate_delay(); smp_store_cpu_info(cpu); set_cpu_online(cpu, true); cpu_startup_entry(0); }

pub unsafe extern "C" fn __cpu_up(cpu: c_uint, tsk: *mut task_struct) -> c_int { /* per_cpu(cpu_state, cpu) = CPU_UP_PREPARE; */ stack_start.sp = (*tsk).thread.sp; stack_start.thread_info = (*tsk).stack; stack_start.bss_start = 0; stack_start.start_kernel_fn = start_secondary as *mut c_void; flush_icache_range(&stack_start as *const _ as c_ulong, &stack_start as *const _ as c_ulong + core::mem::size_of::<StackStart>()); wmb(); ((*mp_ops).start_cpu.unwrap())(cpu, _stext as c_ulong); let timeout = jiffies + HZ; while time_before(jiffies, timeout) { if cpu_online(cpu) { return 0; } udelay(10); barrier(); } if cpu_online(cpu) { 0 } else { -ENOENT } }

pub unsafe extern "C" fn arch_smp_send_reschedule(cpu: c_int) { ((*mp_ops).send_ipi.unwrap())(cpu, SMP_MSG_RESCHEDULE); }
pub unsafe extern "C" fn arch_send_call_function_single_ipi(cpu: c_int) { ((*mp_ops).send_ipi.unwrap())(cpu, SMP_MSG_FUNCTION_SINGLE); }

pub unsafe extern "C" fn smp_message_recv(msg: c_uint) { match msg as c_int { SMP_MSG_FUNCTION => generic_smp_call_function_interrupt(), SMP_MSG_RESCHEDULE => scheduler_ipi(), SMP_MSG_FUNCTION_SINGLE => generic_smp_call_function_single_interrupt(), SMP_MSG_TIMER => ipi_timer(), _ => printk(b"unknown IPI\n\0".as_ptr() as *const c_char) } }

#[cfg(feature = "CONFIG_MMU")]
#[repr(C)] pub struct flush_tlb_data { pub vma: *mut vm_area_struct, pub addr1: c_ulong, pub addr2: c_ulong }
#[cfg(feature = "CONFIG_MMU")]
pub unsafe extern "C" fn flush_tlb_all() { on_each_cpu(flush_tlb_all_ipi, core::ptr::null_mut(), 1); }
#[cfg(feature = "CONFIG_MMU")]
unsafe extern "C" fn flush_tlb_all_ipi(_info: *mut c_void) { local_flush_tlb_all(); }

#[cfg(feature = "CONFIG_MMU")]
unsafe extern "C" fn flush_tlb_mm_ipi(mm: *mut c_void) { local_flush_tlb_mm(mm as *mut mm_struct); }
#[cfg(feature = "CONFIG_MMU")]
pub unsafe extern "C" fn flush_tlb_mm(mm: *mut mm_struct) { preempt_disable(); if atomic_read(&(*mm).mm_users) != 1 || (*current).mm != mm { smp_call_function(flush_tlb_mm_ipi, mm as *mut c_void, 1); } local_flush_tlb_mm(mm); preempt_enable(); }

#[cfg(feature = "CONFIG_MMU")]
unsafe extern "C" fn flush_tlb_range_ipi(info: *mut c_void) { let fd = &*(info as *mut flush_tlb_data); local_flush_tlb_range(fd.vma, fd.addr1, fd.addr2); }
#[cfg(feature = "CONFIG_MMU")]
pub unsafe extern "C" fn flush_tlb_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong) { let mut fd = flush_tlb_data { vma, addr1: start, addr2: end }; preempt_disable(); smp_call_function(flush_tlb_range_ipi, &mut fd as *mut _ as *mut c_void, 1); local_flush_tlb_range(vma, start, end); preempt_enable(); }

#[cfg(feature = "CONFIG_MMU")]
unsafe extern "C" fn flush_tlb_kernel_range_ipi(info: *mut c_void) { let fd = &*(info as *mut flush_tlb_data); local_flush_tlb_kernel_range(fd.addr1, fd.addr2); }
#[cfg(feature = "CONFIG_MMU")]
pub unsafe extern "C" fn flush_tlb_kernel_range(start: c_ulong, end: c_ulong) { let mut fd = flush_tlb_data { vma: core::ptr::null_mut(), addr1: start, addr2: end }; on_each_cpu(flush_tlb_kernel_range_ipi, &mut fd as *mut _ as *mut c_void, 1); }

#[cfg(feature = "CONFIG_MMU")]
unsafe extern "C" fn flush_tlb_page_ipi(info: *mut c_void) { let fd = &*(info as *mut flush_tlb_data); local_flush_tlb_page(fd.vma, fd.addr1); }
#[cfg(feature = "CONFIG_MMU")]
pub unsafe extern "C" fn flush_tlb_page(vma: *mut vm_area_struct, page: c_ulong) { let mut fd = flush_tlb_data { vma, addr1: page, addr2: 0 }; preempt_disable(); smp_call_function(flush_tlb_page_ipi, &mut fd as *mut _ as *mut c_void, 1); local_flush_tlb_page(vma, page); preempt_enable(); }

#[cfg(feature = "CONFIG_MMU")]
unsafe extern "C" fn flush_tlb_one_ipi(info: *mut c_void) { let fd = &*(info as *mut flush_tlb_data); local_flush_tlb_one(fd.addr1, fd.addr2); }
#[cfg(feature = "CONFIG_MMU")]
pub unsafe extern "C" fn flush_tlb_one(asid: c_ulong, vaddr: c_ulong) { let mut fd = flush_tlb_data { vma: core::ptr::null_mut(), addr1: asid, addr2: vaddr }; smp_call_function(flush_tlb_one_ipi, &mut fd as *mut _ as *mut c_void, 1); local_flush_tlb_one(asid, vaddr); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
