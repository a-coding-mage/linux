// SPDX-License-Identifier: GPL-2.0
// Translated from trace_snapshot.c. Kernel includes and external symbols are
// supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_void};

const RING_BUFFER_ALL_CPUS: c_int = -1;
const UINT_MAX: u32 = u32::MAX;

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strstr(a: *const c_char, b: *const c_char) -> *mut c_char;
    fn snprintf(dst: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...);
    fn kstrtoul_from_user(p: *const c_char, n: usize, base: c_int, v: *mut usize) -> c_int;
}

#[repr(C)] pub struct trace_array { pub allocated_snapshot: bool, pub mapped: u32, pub stop_count: c_int, pub snapshot: u32, pub range_addr_start: usize, pub current_trace: *mut c_void, pub array_buffer: array_buffer, pub snapshot_buffer: array_buffer, pub max_lock: usize, pub snapshot_trigger_lock: usize, pub cond_snapshot: *mut cond_snapshot, pub max_latency: u64 }
#[repr(C)] pub struct array_buffer { pub buffer: *mut c_void, pub data: *mut c_void, pub cpu: c_int, pub time_start: u64 }
#[repr(C)] pub struct cond_snapshot { pub cond_data: *mut c_void, pub update: Option<unsafe extern "C" fn(*mut trace_array, *mut c_void) -> bool> }
#[repr(C)] pub struct trace_iterator { pub tr: *mut trace_array, pub array_buffer: *mut array_buffer, pub cpu_file: c_int, pub snapshot: bool }
#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct inode { pub i_private: *mut c_void }
#[repr(C)] pub struct file { pub f_mode: usize, pub private_data: *mut c_void }
#[repr(C)] pub struct task_struct { pub comm: [c_char; 16], pub pid: c_int, pub static_prio: c_int, pub policy: c_int, pub rt_priority: c_int }
#[repr(C)] pub struct ftrace_probe_ops { pub func: Option<unsafe extern "C" fn(usize,usize,*mut trace_array,*mut ftrace_probe_ops,*mut c_void)>, pub print: Option<unsafe extern "C" fn(*mut seq_file,usize,*mut ftrace_probe_ops,*mut c_void)->c_int>, pub init: Option<unsafe extern "C" fn(*mut ftrace_probe_ops,*mut trace_array,usize,*mut c_void,*mut *mut c_void)->c_int>, pub free: Option<unsafe extern "C" fn(*mut ftrace_probe_ops,*mut trace_array,usize,*mut c_void)> }
#[repr(C)] pub struct ftrace_func_command { pub name: *const c_char, pub func: Option<unsafe extern "C" fn(*mut trace_array,*mut c_void,*mut c_char,*mut c_char,*mut c_char,c_int)->c_int> }

extern "C" {
    static mut allocate_snapshot: bool;
    static mut snapshot_at_boot: bool;
    static mut boot_snapshot_info: [c_char; 4096];
    static mut boot_snapshot_index: c_int;
    fn trace_set_ring_buffer_expanded(*mut c_void); fn in_nmi()->bool; fn trace_array_puts(*mut trace_array,*const c_char);
    fn tracer_tracing_off(*mut trace_array); fn tracer_uses_snapshot(*mut c_void)->bool; fn local_irq_save(*mut usize); fn local_irq_restore(usize);
    fn update_max_tr(*mut trace_array,*mut task_struct,c_int,*mut c_void); fn current_task()->*mut task_struct; fn smp_processor_id()->c_int;
    fn arch_spin_lock(*mut usize); fn arch_spin_unlock(*mut usize); fn local_irq_disable(); fn local_irq_enable();
    fn ring_buffer_resize(*mut c_void,usize,c_int)->c_int; fn ring_buffer_subbuf_order_get(*mut c_void)->c_int; fn ring_buffer_subbuf_order_set(*mut c_void,c_int)->c_int;
    fn per_cpu_entries(*mut c_void,c_int)->*mut usize; fn tracing_reset_online_cpus(*mut array_buffer); fn tracing_reset_cpu(*mut array_buffer,c_int);
    fn lockdep_assert_held(*mut usize); fn spin_lock(*mut usize); fn spin_unlock(*mut usize); fn tracing_alloc_snapshot_instance(*mut trace_array)->c_int;
    fn tracing_snapshot(); fn tracing_snapshot_instance(*mut trace_array); fn tracing_disarm_snapshot(*mut trace_array); fn tracing_arm_snapshot_locked(*mut trace_array)->c_int;
    fn ring_buffer_record_is_set_on(*mut c_void)->bool; fn ring_buffer_record_on(*mut c_void); fn ring_buffer_record_off(*mut c_void); fn ring_buffer_wake_waiters(*mut c_void,c_int);
    fn ring_buffer_swap_cpu(*mut c_void,*mut c_void,c_int)->c_int; fn trace_array_printk_buf(*mut c_void,usize,*const c_char,...);
    fn trace_set_buffer_entries(*mut array_buffer,usize); fn allocate_trace_buffer(*mut trace_array,*mut array_buffer,usize)->c_int;
    fn trace_array_put(*mut trace_array); fn synchronize_rcu();
}

static mut BOOT_SNAPSHOT_INFO: [c_char; 4096] = [0;4096];

pub unsafe extern "C" fn tracing_snapshot_instance_cond(tr: *mut trace_array, cond_data: *mut c_void) {
    if in_nmi() { trace_array_puts(tr, c"*** SNAPSHOT CALLED FROM NMI CONTEXT ***\n".as_ptr()); trace_array_puts(tr,c"*** snapshot is being ignored        ***\n".as_ptr()); return; }
    if !(*tr).allocated_snapshot { trace_array_puts(tr,c"*** SNAPSHOT NOT ALLOCATED ***\n".as_ptr()); trace_array_puts(tr,c"*** stopping trace here!   ***\n".as_ptr()); tracer_tracing_off(tr); return; }
    if (*tr).mapped != 0 { trace_array_puts(tr,c"*** BUFFER MEMORY MAPPED ***\n".as_ptr()); trace_array_puts(tr,c"*** Can not use snapshot (sorry) ***\n".as_ptr()); return; }
    if tracer_uses_snapshot((*tr).current_trace) { trace_array_puts(tr,c"*** LATENCY TRACER ACTIVE ***\n".as_ptr()); trace_array_puts(tr,c"*** Can not use snapshot (sorry) ***\n".as_ptr()); return; }
    let mut flags=0; local_irq_save(&mut flags); update_max_tr(tr,current_task(),smp_processor_id(),cond_data); local_irq_restore(flags);
}
pub unsafe extern "C" fn tracing_snapshot_cond(tr:*mut trace_array, d:*mut c_void){ tracing_snapshot_instance_cond(tr,d); }

pub unsafe extern "C" fn tracing_cond_snapshot_data(tr:*mut trace_array)->*mut c_void { let mut d=core::ptr::null_mut(); local_irq_disable(); arch_spin_lock(&mut (*tr).max_lock); if !(*tr).cond_snapshot.is_null(){d=(*(*tr).cond_snapshot).cond_data;} arch_spin_unlock(&mut (*tr).max_lock); local_irq_enable(); d }

pub unsafe extern "C" fn resize_buffer_duplicate_size(trace_buf:*mut array_buffer,size_buf:*mut array_buffer,cpu_id:c_int)->c_int { let mut ret=0; let cpus=if cpu_id==RING_BUFFER_ALL_CPUS {0..256} else {cpu_id..cpu_id+1}; for cpu in cpus { let n=*per_cpu_entries((*size_buf).data,cpu); ret=ring_buffer_resize((*trace_buf).buffer,n,cpu); if ret<0{break;} *per_cpu_entries((*trace_buf).data,cpu)=n; } ret }

pub unsafe extern "C" fn tracing_alloc_snapshot_instance_rs(tr:*mut trace_array)->c_int { if !(*tr).allocated_snapshot { let o=ring_buffer_subbuf_order_get((*tr).array_buffer.buffer); let r=ring_buffer_subbuf_order_set((*tr).snapshot_buffer.buffer,o); if r<0{return r;} let r=resize_buffer_duplicate_size(&mut (*tr).snapshot_buffer,&mut (*tr).array_buffer,RING_BUFFER_ALL_CPUS); if r<0{return r;} (*tr).allocated_snapshot=true;} 0 }
pub unsafe extern "C" fn free_snapshot(tr:*mut trace_array){ring_buffer_subbuf_order_set((*tr).snapshot_buffer.buffer,0);ring_buffer_resize((*tr).snapshot_buffer.buffer,1,RING_BUFFER_ALL_CPUS);trace_set_buffer_entries(&mut (*tr).snapshot_buffer,1);tracing_reset_online_cpus(&mut (*tr).snapshot_buffer);(*tr).allocated_snapshot=false;}

pub unsafe extern "C" fn tracing_arm_snapshot_locked_rs(tr:*mut trace_array)->c_int { spin_lock(&mut (*tr).snapshot_trigger_lock); if (*tr).snapshot==UINT_MAX || (*tr).mapped!=0 {spin_unlock(&mut (*tr).snapshot_trigger_lock);return -16;} (*tr).snapshot+=1;spin_unlock(&mut (*tr).snapshot_trigger_lock);let r=tracing_alloc_snapshot_instance_rs(tr);if r!=0{spin_lock(&mut (*tr).snapshot_trigger_lock);(*tr).snapshot-=1;spin_unlock(&mut (*tr).snapshot_trigger_lock);}r }
pub unsafe extern "C" fn tracing_disarm_snapshot_rs(tr:*mut trace_array){spin_lock(&mut (*tr).snapshot_trigger_lock);if (*tr).snapshot!=0{(*tr).snapshot-=1;}spin_unlock(&mut (*tr).snapshot_trigger_lock);}

pub unsafe extern "C" fn trace_allocate_snapshot(tr:*mut trace_array,size:usize)->c_int {if (*tr).range_addr_start!=0{return 0;}let r=allocate_trace_buffer(tr,&mut (*tr).snapshot_buffer,if allocate_snapshot{size}else{1});if r<0{return -12;}(*tr).allocated_snapshot=allocate_snapshot;allocate_snapshot=false;0}
pub unsafe extern "C" fn do_allocate_snapshot(_name:*const c_char){allocate_snapshot=true;}
pub unsafe extern "C" fn ftrace_boot_snapshot(){if !snapshot_at_boot{return;}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
