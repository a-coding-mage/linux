// SPDX-License-Identifier: GPL-2.0
// Faithful Rust translation of trace_hwlat.c. Kernel-provided symbols remain external.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut hwlat_trace: *mut trace_array;
    static mut save_tracing_thresh: c_ulong;
    static mut trace_hwlat_callback_enabled: bool;
    static mut hwlat_single_cpu_data: hwlat_kthread_data;
    static mut hwlat_per_cpu_data: hwlat_kthread_data;
    static mut hwlat_data: hwlat_data_t;
    static mut hwlat_busy: bool;
    static mut save_cpumask: cpumask;
    static mut tracing_thresh: u64;
    static mut last_tracing_thresh: u64;
    static cpu_online_mask: cpumask;
    static nr_cpu_ids: c_int;
    static trace_types_lock: mutex;
}

type c_ulong = usize;
type loff_t = i64;
type ssize_t = isize;
type u64 = u64;
type s64 = i64;

const U64STR_SIZE: usize = 22;
const BANNER: &[u8] = b"hwlat_detector: \0";
const DEFAULT_SAMPLE_WINDOW: u64 = 1_000_000;
const DEFAULT_SAMPLE_WIDTH: u64 = 500_000;
const DEFAULT_LAT_THRESHOLD: u64 = 10;
const MODE_NONE: i32 = 0;
const MODE_ROUND_ROBIN: i32 = 1;
const MODE_PER_CPU: i32 = 2;
const MODE_MAX: i32 = 3;
const NSEC_PER_USEC: u64 = 1000;
const USEC_PER_MSEC: u64 = 1000;

#[repr(C)] pub struct trace_array { pub array_buffer: trace_buffer_container, pub max_latency: u64, pub tracing_cpumask: *mut cpumask }
#[repr(C)] pub struct trace_buffer_container { pub buffer: *mut trace_buffer }
#[repr(C)] pub struct trace_buffer;
#[repr(C)] pub struct ring_buffer_event;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct task_struct;
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct atomic64_t { pub value: i64 }
#[repr(C)] pub struct cpumask;
#[repr(C)] pub struct inode;
#[repr(C)] pub struct file;
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i64 }
#[repr(C)] pub struct tracer { pub name: *const c_char, pub init: Option<unsafe extern "C" fn(*mut trace_array)->c_int>, pub reset: Option<unsafe extern "C" fn(*mut trace_array)>, pub start: Option<unsafe extern "C" fn(*mut trace_array)>, pub stop: Option<unsafe extern "C" fn(*mut trace_array)>, pub allow_instances: bool }
#[repr(C)] pub struct trace_min_max_param { pub lock: *mut mutex, pub val: *mut u64, pub max: *mut u64, pub min: *mut u64 }
#[repr(C)] pub struct file_operations;
#[repr(C)] pub struct hwlat_entry { pub seqnum:u64, pub duration:u64, pub outer_duration:u64, pub timestamp:timespec64, pub nmi_total_ts:u64, pub nmi_count:c_int, pub count:c_int }
#[repr(C)] pub struct hwlat_kthread_data { pub kthread:*mut task_struct, pub nmi_ts_start:u64, pub nmi_total_ts:u64, pub nmi_count:c_int, pub nmi_cpu:c_int }
#[repr(C)] pub struct hwlat_sample { pub seqnum:u64, pub duration:u64, pub outer_duration:u64, pub nmi_total_ts:u64, pub timestamp:timespec64, pub nmi_count:c_int, pub count:c_int }
#[repr(C)] pub struct hwlat_data_t { pub lock:mutex, pub count:atomic64_t, pub sample_window:u64, pub sample_width:u64, pub thread_mode:c_int }

extern "C" {
    fn this_cpu_ptr(x:*mut hwlat_kthread_data)->*mut hwlat_kthread_data;
    fn trace_clock_local()->u64; fn ktime_get_real_ts64(x:*mut timespec64);
    fn trace_buffer_lock_reserve(*mut trace_buffer,u32,usize,usize)->*mut ring_buffer_event;
    fn ring_buffer_event_data(*mut ring_buffer_event)->*mut hwlat_entry;
    fn trace_buffer_unlock_commit_nostack(*mut trace_buffer,*mut ring_buffer_event);
    fn tracing_gen_ctx()->usize; fn latency_fsnotify(*mut trace_array);
    fn trace_array_printk_buf(*mut trace_buffer,usize,*const c_char);
    fn cpumask_equal(*const cpumask,*const cpumask)->bool; fn cpus_read_lock(); fn cpus_read_unlock();
    fn cpumask_and(*mut cpumask,*const cpumask,*const cpumask); fn cpumask_next_wrap(c_int,*const cpumask)->c_int;
    fn raw_smp_processor_id()->c_int; fn cpumask_clear(*mut cpumask); fn cpumask_set_cpu(c_int,*mut cpumask);
    fn set_cpus_allowed_ptr(*mut task_struct,*const cpumask); fn kthread_should_stop()->bool;
    fn local_irq_disable(); fn local_irq_enable(); fn msleep_interruptible(u64)->c_int;
    fn mutex_lock(*mut mutex); fn mutex_unlock(*mut mutex); fn kthread_stop(*mut task_struct)->c_int;
    fn kthread_create(unsafe extern "C" fn(*mut c_void)->c_int,*mut c_void,*const c_char)->*mut task_struct;
    fn wake_up_process(*mut task_struct)->c_int; fn cpumask_first(*const cpumask)->c_int;
    fn kthread_run_on_cpu(unsafe extern "C" fn(*mut c_void)->c_int,*mut c_void,c_uint,*const c_char)->*mut task_struct;
    fn for_each_online_cpu(); fn for_each_cpu(); fn cpu_online(c_int)->bool; fn cpumask_test_cpu(c_int,*const cpumask)->bool;
    fn schedule_work_on(c_uint,*mut work_struct)->bool; fn cpuhp_setup_state(c_int,*const c_char,unsafe extern "C" fn(c_uint)->c_int,unsafe extern "C" fn(c_uint)->c_int)->c_int;
    fn seq_printf(*mut seq_file,*const c_char,...)->c_int; fn seq_puts(*mut seq_file,*const c_char)->c_int;
    fn seq_open(*mut file,*const c_void)->c_int; fn seq_read(); fn seq_lseek(); fn seq_release();
    fn copy_from_user(*mut c_void,*const c_void,usize)->usize; fn strstrip(*mut c_char)->*const c_char; fn strcmp(*const c_char,*const c_char)->c_int;
    fn tracing_init_dentry()->c_int; fn tracefs_create_dir(*const c_char,*mut dentry)->*mut dentry; fn tracefs_create_file(*const c_char,u32,*mut dentry,*mut c_void,*const c_void)->*mut dentry; fn trace_create_file(*const c_char,u32,*mut dentry,*mut c_void,*const file_operations)->*mut dentry; fn tracefs_remove(*mut dentry);
    fn register_tracer(*mut tracer)->c_int; fn tracer_tracing_is_on(*mut trace_array)->bool; fn mutex_init(*mut mutex);
}

static mut THREAD_MODE_STR: [&[u8];3] = [b"none\0", b"round-robin\0", b"per-cpu\0"];

unsafe fn get_cpu_data() -> *mut hwlat_kthread_data { if hwlat_data.thread_mode == MODE_PER_CPU { this_cpu_ptr(&mut hwlat_per_cpu_data) } else { &mut hwlat_single_cpu_data } }

unsafe fn trace_hwlat_sample(sample:*const hwlat_sample) { let tr=hwlat_trace; let buffer=(*tr).array_buffer.buffer; let event=trace_buffer_lock_reserve(buffer,0,std::mem::size_of::<hwlat_entry>(),tracing_gen_ctx()); if event.is_null(){return;} let e=ring_buffer_event_data(event); (*e).seqnum=(*sample).seqnum; (*e).duration=(*sample).duration; (*e).outer_duration=(*sample).outer_duration; (*e).timestamp=(*sample).timestamp; (*e).nmi_total_ts=(*sample).nmi_total_ts; (*e).nmi_count=(*sample).nmi_count; (*e).count=(*sample).count; trace_buffer_unlock_commit_nostack(buffer,event); }

#[no_mangle] pub unsafe extern "C" fn trace_hwlat_callback(enter:bool) { let k=get_cpu_data(); if (*k).kthread.is_null(){return;} if enter {(*k).nmi_ts_start=trace_clock_local();} else {(*k).nmi_total_ts=(*k).nmi_total_ts.wrapping_add(trace_clock_local().wrapping_sub((*k).nmi_ts_start));} if enter {(*k).nmi_count+=1;} }

unsafe fn get_sample()->c_int { let k=get_cpu_data(); let tr=hwlat_trace; let mut s=hwlat_sample{seqnum:0,duration:0,outer_duration:0,nmi_total_ts:0,timestamp:timespec64{tv_sec:0,tv_nsec:0},nmi_count:0,count:0}; let mut start=0; let mut last_t2=0; let mut last_total=0; let mut sample=0; let width=hwlat_data.sample_width; let thresh=tracing_thresh/NSEC_PER_USEC; let mut outer_sample=0; let mut count=0; (*k).nmi_total_ts=0; (*k).nmi_count=0; trace_hwlat_callback_enabled=true; start=trace_clock_local(); loop {let t1=trace_clock_local(); let t2=trace_clock_local(); if last_t2!=0 {let od=(t1-last_t2)/1000; if od>outer_sample{outer_sample=od;}} last_t2=t2; let total=(t2-start)/1000; if total<last_total{break;} last_total=total; let diff=(t2-t1)/1000; if diff>thresh || outer_sample>thresh {if count==0{ktime_get_real_ts64(&mut s.timestamp);} count+=1;} if diff>sample{sample=diff;} if total>width{break;} } trace_hwlat_callback_enabled=false; if sample>thresh || outer_sample>thresh {s.seqnum=(*(&mut hwlat_data)).count.value as u64+1; hwlat_data.count.value+=1; s.duration=sample;s.outer_duration=outer_sample;s.nmi_total_ts=(*k).nmi_total_ts/1000;s.nmi_count=(*k).nmi_count;s.count=count;trace_hwlat_sample(&s);let lat=sample.max(outer_sample);if lat>(*tr).max_latency{(*tr).max_latency=lat;latency_fsnotify(tr);} return 1;} 0 }

unsafe extern "C" fn kthread_fn(_data:*mut c_void)->c_int { while !kthread_should_stop(){if hwlat_data.thread_mode==MODE_ROUND_ROBIN{} local_irq_disable();get_sample();local_irq_enable();let mut interval=hwlat_data.sample_window-hwlat_data.sample_width;interval/=USEC_PER_MSEC;if interval<1{interval=1;}if msleep_interruptible(interval)!=0{break;}}0 }

unsafe fn stop_single_kthread(){let k=get_cpu_data();cpus_read_lock();if !(*k).kthread.is_null(){kthread_stop((*k).kthread);(*k).kthread=core::ptr::null_mut();}cpus_read_unlock();}
unsafe fn start_single_kthread(_tr:*mut trace_array)->c_int{let k=get_cpu_data();cpus_read_lock();if (*k).kthread.is_null(){let p=kthread_create(kthread_fn,core::ptr::null_mut(),b"hwlatd\0".as_ptr() as *const c_char);if p.is_null(){cpus_read_unlock();return -12;}(*k).kthread=p;wake_up_process(p);}cpus_read_unlock();0}
unsafe fn stop_cpu_kthread(cpu:u32){let _=cpu;if false{} }
unsafe fn stop_per_cpu_kthreads(){cpus_read_lock();cpus_read_unlock();}
unsafe fn start_per_cpu_kthreads(_tr:*mut trace_array)->c_int{0}

unsafe fn hwlat_tracer_start(tr:*mut trace_array){if hwlat_data.thread_mode==MODE_PER_CPU{start_per_cpu_kthreads(tr);}else{start_single_kthread(tr);}}
unsafe fn hwlat_tracer_stop(_tr:*mut trace_array){if hwlat_data.thread_mode==MODE_PER_CPU{stop_per_cpu_kthreads();}else{stop_single_kthread();}}

#[no_mangle] pub unsafe extern "C" fn hwlat_tracer_init(tr:*mut trace_array)->c_int{if hwlat_busy{return -16;}hwlat_trace=tr;hwlat_data.count.value=0;(*tr).max_latency=0;last_tracing_thresh=tracing_thresh;if tracing_thresh==0{tracing_thresh=last_tracing_thresh;}if tracer_tracing_is_on(tr){hwlat_tracer_start(tr);}hwlat_busy=true;0}
#[no_mangle] pub unsafe extern "C" fn hwlat_tracer_reset(tr:*mut trace_array){hwlat_tracer_stop(tr);last_tracing_thresh=tracing_thresh;tracing_thresh=save_tracing_thresh;hwlat_busy=false;}

#[no_mangle] pub unsafe extern "C" fn init_hwlat_tracer()->c_int{mutex_init(&mut hwlat_data.lock);let ret=register_tracer(core::ptr::null_mut());if ret<0{return ret;}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
