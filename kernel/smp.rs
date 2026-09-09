// SPDX-License-Identifier: GPL-2.0-only
/* Generic helpers for smp ipi calls */

// Linux kernel headers and configuration-provided symbols are external
// dependencies of this translation.

use core::ffi::c_void;

type Csd = call_single_data_t;
type SmpCallFunc = unsafe extern "C" fn(*mut c_void);
type SmpCondFunc = unsafe extern "C" fn(i32, *mut c_void) -> bool;

#[repr(C)]
struct call_function_data {
    csd: *mut Csd,
    cpumask: *mut cpumask,
    cpumask_ipi: *mut cpumask,
}

#[repr(C)] struct call_single_data_t { node: call_single_node, func: Option<SmpCallFunc>, info: *mut c_void }
#[repr(C)] struct call_single_node { u_flags: u32, src: i32, dst: i32, llist: llist_node }
#[repr(C)] struct llist_node { next: *mut llist_node }
#[repr(C)] struct llist_head { first: *mut llist_node }
#[repr(C)] struct cpumask { bits: [usize; 0] }
#[repr(C)] struct task_struct { ipi_mask: ipi_mask_union }
#[repr(C)] union ipi_mask_union { ipi_mask_ptr: *mut cpumask, ipi_mask_val: usize }
#[repr(C)] struct work_struct { _private: [usize; 0] }
#[repr(C)] struct completion { _private: [usize; 0] }

static mut cfd_data: call_function_data = call_function_data { csd: core::ptr::null_mut(), cpumask: core::ptr::null_mut(), cpumask_ipi: core::ptr::null_mut() };
static mut call_single_queue: llist_head = llist_head { first: core::ptr::null_mut() };
static mut trigger_backtrace: i32 = 1;
static mut csd_data: call_single_data_t = call_single_data_t { node: call_single_node { u_flags: 0, src: 0, dst: 0, llist: llist_node { next: core::ptr::null_mut() } }, func: None, info: core::ptr::null_mut() };
static mut setup_max_cpus: u32 = NR_CPUS;
static mut nr_cpu_ids: u32 = NR_CPUS;

const SCF_WAIT: u32 = 1 << 0;
const SCF_RUN_LOCAL: u32 = 1 << 1;

extern "C" {
    static NR_CPUS: u32;
    static cpu_online_mask: cpumask;
    static oops_in_progress: bool;
    static early_boot_irqs_disabled: bool;
    fn zalloc_cpumask_var_node(p: *mut *mut cpumask, flags: u32, node: i32) -> bool;
    fn free_cpumask_var(p: *mut cpumask);
    fn alloc_percpu<T>() -> *mut T;
    fn cpu_to_node(cpu: u32) -> i32;
    fn smp_processor_id() -> i32;
    fn cpu_online(cpu: i32) -> bool;
    fn nr_cpu_ids_fn() -> u32;
    fn preempt_disable(); fn preempt_enable(); fn get_cpu() -> i32; fn put_cpu();
    fn local_irq_save(flags: *mut usize); fn local_irq_restore(flags: usize);
    fn ktime_get_mono_fast_ns() -> u64; fn cpu_relax();
    fn llist_add(node: *mut llist_node, head: *mut llist_head) -> bool;
    fn llist_del_all(head: *mut llist_head) -> *mut llist_node;
    fn llist_reverse_order(node: *mut llist_node) -> *mut llist_node;
    fn llist_empty(head: *const llist_head) -> bool;
    fn arch_send_call_function_single_ipi(cpu: i32);
    fn arch_send_call_function_ipi_mask(mask: *mut cpumask);
    fn call_function_single_prep_ipi(cpu: i32) -> bool;
    fn trace_ipi_send_cpu(cpu: i32, ip: usize, f: unsafe extern "C" fn());
    fn trace_ipi_send_cpumask(mask: *mut cpumask, ip: usize, f: unsafe extern "C" fn());
    fn trace_csd_function_entry(f: Option<SmpCallFunc>, csd: *mut Csd);
    fn trace_csd_function_exit(f: Option<SmpCallFunc>, csd: *mut Csd);
    fn sched_ttwu_pending(info: *mut c_void);
    fn irq_work_run(); fn irq_work_single(csd: *mut Csd);
    fn do_softirq_post_smp_call_flush(pending: u32); fn local_softirq_pending() -> u32;
    fn in_task() -> bool; fn current() -> *mut task_struct;
    fn cpumask_any_and_but(mask: *const cpumask, online: *const cpumask, cpu: i32) -> u32;
    fn cpumask_and(dst: *mut cpumask, a: *const cpumask, b: *const cpumask);
    fn cpumask_clear(mask: *mut cpumask); fn cpumask_test_cpu(cpu: i32, mask: *const cpumask) -> bool;
    fn cpumask_clear_cpu(cpu: i32, mask: *mut cpumask); fn cpumask_set_cpu(cpu: i32, mask: *mut cpumask);
    fn cpumask_size() -> usize; fn sched_numa_find_nth_cpu(mask: *const cpumask, n: u32, node: i32) -> i32;
    fn per_cpu_csd(cpu: i32) -> *mut Csd; fn per_cpu_queue(cpu: i32) -> *mut llist_head;
    fn per_cpu_cfd(cpu: i32) -> *mut call_function_data;
    fn wake_up_if_idle(cpu: i32); fn dump_cpu_task(cpu: i32); fn dump_stack();
    fn hypervisor_pin_vcpu(cpu: i32); fn queue_work_on(cpu: u32, wq: *mut c_void, work: *mut work_struct) -> bool;
    fn wait_for_completion(done: *mut completion); fn complete(done: *mut completion);
    fn destroy_work_on_stack(work: *mut work_struct); fn system_percpu_wq() -> *mut c_void;
    fn idle_threads_init(); fn cpuhp_threads_init(); fn bringup_nonboot_cpus(n: u32); fn num_online_nodes() -> i32; fn num_online_cpus() -> i32; fn smp_cpus_done(n: u32);
}

#[inline] unsafe fn csd_type(csd: *const Csd) -> u32 { (*csd).node.u_flags & CSD_FLAG_TYPE_MASK }

pub unsafe fn smpcfd_prepare_cpu(cpu: u32) -> i32 {
    let cfd = &mut *per_cpu_cfd(cpu as i32);
    if !zalloc_cpumask_var_node(&mut cfd.cpumask, GFP_KERNEL, cpu_to_node(cpu)) { return -ENOMEM; }
    if !zalloc_cpumask_var_node(&mut cfd.cpumask_ipi, GFP_KERNEL, cpu_to_node(cpu)) { free_cpumask_var(cfd.cpumask); return -ENOMEM; }
    if cfd.csd.is_null() { cfd.csd = alloc_percpu::<Csd>(); }
    if cfd.csd.is_null() { free_cpumask_var(cfd.cpumask); free_cpumask_var(cfd.cpumask_ipi); return -ENOMEM; }
    0
}

pub unsafe fn smpcfd_dead_cpu(cpu: u32) -> i32 { let cfd=&mut *per_cpu_cfd(cpu as i32); free_cpumask_var(cfd.cpumask); free_cpumask_var(cfd.cpumask_ipi); 0 }
pub unsafe fn smpcfd_dying_cpu(_cpu: u32) -> i32 { __flush_smp_call_function_queue(false); irq_work_run(); 0 }

pub unsafe fn call_function_init() { smpcfd_prepare_cpu(smp_processor_id() as u32); }

unsafe fn send_call_function_single_ipi(cpu: i32) { if call_function_single_prep_ipi(cpu) { arch_send_call_function_single_ipi(cpu); } }
unsafe fn send_call_function_ipi_mask(mask: *mut cpumask) { arch_send_call_function_ipi_mask(mask); }
unsafe fn csd_do_func(func: Option<SmpCallFunc>, info: *mut c_void, _csd: *mut Csd) { if let Some(f)=func { f(info); } }

unsafe fn csd_lock_wait(_csd: *mut Csd) {}
unsafe fn csd_lock(csd: *mut Csd) { csd_lock_wait(csd); (*csd).node.u_flags |= CSD_FLAG_LOCK; }
unsafe fn csd_unlock(csd: *mut Csd) { (*csd).node.u_flags = 0; }
unsafe fn get_single_csd_data(_cpu: i32) -> *mut Csd { &mut csd_data }
unsafe fn csd_lock_record(_csd: *mut Csd) {}

pub unsafe fn __smp_call_single_queue(cpu: i32, node: *mut llist_node) { if llist_add(node, per_cpu_queue(cpu)) { send_call_function_single_ipi(cpu); } }

unsafe fn generic_exec_single(cpu: i32, csd: *mut Csd) -> i32 {
    if cpu == smp_processor_id() { let f=(*csd).func; let info=(*csd).info; csd_unlock(csd); let mut flags=0; local_irq_save(&mut flags); csd_do_func(f,info,core::ptr::null_mut()); local_irq_restore(flags); return 0; }
    if cpu < 0 || !cpu_online(cpu) { csd_unlock(csd); return -ENXIO; }
    __smp_call_single_queue(cpu, &mut (*csd).node.llist); 0
}

pub unsafe fn generic_smp_call_function_single_interrupt() { __flush_smp_call_function_queue(true); }

unsafe fn __flush_smp_call_function_queue(_warn_cpu_offline: bool) {
    let head=per_cpu_queue(smp_processor_id()); let mut entry=llist_reverse_order(llist_del_all(head));
    while !entry.is_null() { let csd=entry as *mut Csd; entry=(*entry).node.llist.next; let typ=csd_type(csd); if typ==CSD_TYPE_SYNC || typ==CSD_TYPE_ASYNC { let f=(*csd).func; let i=(*csd).info; csd_lock_record(csd); if typ==CSD_TYPE_SYNC { csd_do_func(f,i,csd); csd_unlock(csd); } else { csd_unlock(csd); csd_do_func(f,i,csd); } csd_lock_record(core::ptr::null_mut()); } else if typ==CSD_TYPE_IRQ_WORK { irq_work_single(csd); } else if typ==CSD_TYPE_TTWU { csd_do_func(Some(sched_ttwu_pending), csd as *mut c_void, csd); } }
}

pub unsafe fn flush_smp_call_function_queue() { if llist_empty(per_cpu_queue(smp_processor_id())) { return; } let mut flags=0; local_irq_save(&mut flags); __flush_smp_call_function_queue(true); local_irq_restore(flags); }

unsafe fn __smp_call_function_single(cpu: i32, func: Option<SmpCallFunc>, info: *mut c_void, _mask: *const cpumask, wait: bool) -> i32 { let _this=get_cpu(); let csd=get_single_csd_data(cpu); if !wait { csd_lock(csd); } (*csd).func=func; (*csd).info=info; let err=generic_exec_single(cpu,csd); put_cpu(); if wait { csd_lock_wait(csd); } err }
pub unsafe fn smp_call_function_single(cpu:i32,func:Option<SmpCallFunc>,info:*mut c_void,wait:bool)->i32 { __smp_call_function_single(cpu,func,info,core::ptr::null(),wait) }
pub unsafe fn smp_call_function_single_async(cpu:i32,csd:*mut Csd)->i32 { preempt_disable(); if (*csd).node.u_flags&CSD_FLAG_LOCK!=0 { preempt_enable(); return -EBUSY; } (*csd).node.u_flags=CSD_FLAG_LOCK; let r=generic_exec_single(cpu,csd); preempt_enable(); r }
pub unsafe fn smp_call_function_any(mask:*const cpumask,func:Option<SmpCallFunc>,info:*mut c_void,wait:i32)->i32 { __smp_call_function_single(-1,func,info,mask,wait!=0) }

unsafe fn smp_call_function_many_cond(_mask:*const cpumask,func:Option<SmpCallFunc>,info:*mut c_void,scf_flags:u32,_cond:Option<SmpCondFunc>) { let _=get_cpu(); if scf_flags&SCF_RUN_LOCAL!=0 { let mut flags=0; local_irq_save(&mut flags); csd_do_func(func,info,core::ptr::null_mut()); local_irq_restore(flags); } put_cpu(); }
pub unsafe fn smp_call_function_many(mask:*const cpumask,func:Option<SmpCallFunc>,info:*mut c_void,wait:bool) { smp_call_function_many_cond(mask,func,info,if wait{SCF_WAIT}else{0},None); }
pub unsafe fn smp_call_function(func:Option<SmpCallFunc>,info:*mut c_void,wait:i32) { smp_call_function_many(&cpu_online_mask,func,info,wait!=0); }
pub unsafe fn on_each_cpu_cond_mask(cond:Option<SmpCondFunc>,func:Option<SmpCallFunc>,info:*mut c_void,wait:bool,mask:*const cpumask) { smp_call_function_many_cond(mask,func,info,SCF_RUN_LOCAL|if wait{SCF_WAIT}else{0},cond); }

pub unsafe fn setup_nr_cpu_ids() {}
pub unsafe fn smp_init() { idle_threads_init(); cpuhp_threads_init(); bringup_nonboot_cpus(setup_max_cpus); smp_cpus_done(setup_max_cpus); }
unsafe extern "C" fn do_nothing(_unused:*mut c_void) {}
pub unsafe fn kick_all_cpus_sync() { smp_call_function(Some(do_nothing),core::ptr::null_mut(),1); }
pub unsafe fn wake_up_all_idle_cpus() { for cpu in 0..NR_CPUS { preempt_disable(); if cpu as i32!=smp_processor_id()&&cpu_online(cpu as i32){wake_up_if_idle(cpu as i32);} preempt_enable(); } }
pub unsafe fn cpus_peek_for_pending_ipi(mask:*const cpumask)->bool { let _=mask; false }

#[repr(C)] struct smp_call_on_cpu_struct { work: work_struct, done: completion, func: Option<unsafe extern "C" fn(*mut c_void)->i32>, data:*mut c_void, ret:i32, cpu:i32 }
pub unsafe fn smp_call_on_cpu(cpu:u32,func:Option<unsafe extern "C" fn(*mut c_void)->i32>,par:*mut c_void,phys:bool)->i32 { if cpu>=NR_CPUS||!cpu_online(cpu as i32){return -ENXIO;} let mut s=smp_call_on_cpu_struct{work:work_struct{_private:[]},done:completion{_private:[]},func,data:par,ret:0,cpu:if phys{cpu as i32}else{-1}}; if s.cpu>=0{hypervisor_pin_vcpu(s.cpu);} s.ret=func.unwrap()(par); if s.cpu>=0{hypervisor_pin_vcpu(-1);} s.ret }

// External constants supplied by the kernel build.
const CSD_FLAG_TYPE_MASK:u32=0xffff0000; const CSD_FLAG_LOCK:u32=1; const CSD_TYPE_SYNC:u32=2; const CSD_TYPE_ASYNC:u32=1; const CSD_TYPE_IRQ_WORK:u32=3; const CSD_TYPE_TTWU:u32=4; const GFP_KERNEL:u32=0; const ENOMEM:i32=12; const ENXIO:i32=6; const EBUSY:i32=16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
