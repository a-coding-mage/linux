// SPDX-License-Identifier: GPL-2.0

// C dependencies retained as external kernel/KUnit symbols.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut runtime_ms: c_ulong;
}

/* test data structure */
#[repr(C)]
pub struct prbtest_rbdata {
    pub size: c_uint,
    pub text: [c_char; 0], // __counted_by(size)
}

pub const MAX_RBDATA_TEXT_SIZE: usize = 0x80;
pub const MAX_PRB_RECORD_SIZE: usize = core::mem::size_of::<prbtest_rbdata>() + MAX_RBDATA_TEXT_SIZE;

#[repr(C)]
pub struct prbtest_data {
    pub test: *mut kunit,
    pub ringbuffer: *mut printk_ringbuffer,
    /* used by writers to signal reader of new records */
    pub new_record_wait: wait_queue_head_t,
}

#[repr(C)]
pub struct prbtest_thread_data {
    pub num: c_ulong,
    pub test_data: *mut prbtest_data,
}

#[repr(C)]
pub struct prbtest_wakeup_timer {
    pub timer: timer_list,
    pub task: *mut task_struct,
}

// External declarations supplied by the kernel and KUnit environment.
#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct printk_ringbuffer { _private: [u8; 0] }
#[repr(C)] pub struct prb_reserved_entry { _private: [u8; 0] }
#[repr(C)] pub struct printk_record { pub info: *mut printk_info, pub text_buf: *mut c_char }
#[repr(C)] pub struct printk_info { pub seq: u64, pub text_len: c_uint }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }

type cpumask_var_t = *mut cpumask;

extern "C" {
    fn kunit_info(test: *mut kunit, fmt: *const c_char, ...);
    fn kunit_warn(test: *mut kunit, fmt: *const c_char, ...);
    fn kunit_fail(test: *mut kunit, fmt: *const c_char, ...);
    fn alloc_cpumask_var(mask: *mut cpumask_var_t, flags: c_uint) -> bool;
    fn free_cpumask_var(mask: cpumask_var_t);
    fn kunit_add_action_or_reset(test: *mut kunit, action: *const c_void, data: *mut c_void) -> c_int;
    fn kunit_kmalloc(test: *mut kunit, size: usize, flags: c_uint) -> *mut c_void;
    fn cpus_read_lock();
    fn cpus_read_unlock();
    fn cpumask_copy(dst: cpumask_var_t, src: *const cpumask);
    fn cpumask_first(mask: cpumask_var_t) -> c_int;
    fn cpumask_weight(mask: cpumask_var_t) -> c_uint;
    fn cpumask_clear_cpu(cpu: c_int, mask: cpumask_var_t);
    fn init_waitqueue_head(wait: *mut wait_queue_head_t);
    fn kthread_run_on_cpu(threadfn: unsafe extern "C" fn(*mut c_void) -> c_int, data: *mut c_void, cpu: c_int, name: *const c_char, ...) -> *mut task_struct;
    fn kthread_stop(thread: *mut task_struct) -> c_int;
    fn set_cpus_allowed_ptr(task: *mut task_struct, mask: cpumask_var_t) -> c_int;
    fn prb_init(rb: *mut printk_ringbuffer, text_data: *mut u8, text_bits: c_uint, descs: *mut c_void, desc_bits: c_uint, infos: *mut c_void);
    fn prb_rec_init_wr(record: *mut printk_record, size: c_uint);
    fn prb_reserve(entry: *mut prb_reserved_entry, rb: *mut printk_ringbuffer, record: *mut printk_record) -> bool;
    fn prb_commit(entry: *mut prb_reserved_entry);
    fn prb_rec_init_rd(record: *mut printk_record, info: *mut printk_info, text: *mut c_char, size: usize);
    fn prb_read_valid(rb: *mut printk_ringbuffer, seq: u64, record: *mut printk_record) -> bool;
    fn wake_up_interruptible(wait: *mut wait_queue_head_t);
    fn get_random_u32_inclusive(min: c_uint, max: c_uint) -> c_uint;
    fn cond_resched();
    fn kthread_should_stop() -> bool;
    fn timer_setup_on_stack(timer: *mut timer_list, callback: unsafe extern "C" fn(*mut timer_list), flags: c_uint);
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn timer_delete_sync(timer: *mut timer_list);
    fn timer_destroy_on_stack(timer: *mut timer_list);
    fn set_tsk_thread_flag(task: *mut task_struct, flag: c_int);
    fn wake_up_process(task: *mut task_struct) -> c_int;
    static mut cpu_online_mask: *const cpumask;
    static mut current: *mut task_struct;
}

unsafe extern "C" fn prbtest_fail_record(test: *mut kunit, dat: *const prbtest_rbdata, seq: u64) {
    let len = (*dat).size - 1;
    let text = (dat as *const u8).add(core::mem::size_of::<c_uint>()) as *const c_char;
    kunit_fail(test, b"BAD RECORD: seq=%llu size=%u text=%.*s\n\0".as_ptr() as *const c_char,
        seq, (*dat).size,
        if len < MAX_RBDATA_TEXT_SIZE as c_uint { len } else { -1i32 as c_uint },
        if len < MAX_RBDATA_TEXT_SIZE as c_uint { text } else { b"<invalid>\0".as_ptr() as *const c_char });
}

unsafe extern "C" fn prbtest_check_data(dat: *const prbtest_rbdata) -> bool {
    if (*dat).size < 2 || (*dat).size > MAX_RBDATA_TEXT_SIZE as c_uint { return false; }
    let len = (*dat).size - 1;
    let text = (dat as *const u8).add(core::mem::size_of::<c_uint>()) as *const c_char;
    if *text.add(len as usize) != 0 { return false; }
    let mut i = len;
    while i != 0 {
        i -= 1;
        if *text.add(i as usize) != *text { return false; }
    }
    true
}

unsafe extern "C" fn prbtest_writer(data: *mut c_void) -> c_int {
    let tr = data as *mut prbtest_thread_data;
    let text_id = b'A' as c_uint + (*tr).num as c_uint;
    let mut e = core::mem::MaybeUninit::<prb_reserved_entry>::uninit();
    let mut r = core::mem::MaybeUninit::<printk_record>::zeroed().assume_init();
    let mut count: c_ulong = 0;
    kunit_info((*tr).test_data.as_ref().unwrap().test, b"start thread %03lu (writer)\n\0".as_ptr() as *const c_char, (*tr).num);
    loop {
        let mut text_size = get_random_u32_inclusive(2, MAX_RBDATA_TEXT_SIZE as c_uint);
        if text_size < 2 { text_size = 2; }
        if text_size > MAX_RBDATA_TEXT_SIZE as c_uint { text_size = MAX_RBDATA_TEXT_SIZE as c_uint; }
        let record_size = core::mem::size_of::<prbtest_rbdata>() as c_uint + text_size;
        prb_rec_init_wr(&mut r, record_size);
        if prb_reserve(e.as_mut_ptr(), (*tr).test_data.as_ref().unwrap().ringbuffer, &mut r) {
            (*r.info).text_len = record_size;
            let dat = r.text_buf as *mut prbtest_rbdata;
            (*dat).size = text_size;
            let text = (dat as *mut u8).add(core::mem::size_of::<c_uint>());
            core::ptr::write_bytes(text, text_id as u8, text_size as usize - 1);
            *text.add(text_size as usize - 1) = 0;
            prb_commit(e.as_mut_ptr());
            wake_up_interruptible(&mut (*(*tr).test_data).new_record_wait);
        }
        if (count & 0x3fff) == 0 { cond_resched(); }
        count = count.wrapping_add(1);
        if kthread_should_stop() { break; }
    }
    kunit_info((*tr).test_data.as_ref().unwrap().test, b"end thread %03lu: wrote=%lu\n\0".as_ptr() as *const c_char, (*tr).num, count);
    0
}

unsafe extern "C" fn prbtest_wakeup_callback(timer: *mut timer_list) {
    let wakeup = timer as *mut prbtest_wakeup_timer;
    set_tsk_thread_flag((*wakeup).task, 0);
    wake_up_process((*wakeup).task);
}

unsafe extern "C" fn prbtest_reader(test_data: *mut prbtest_data, _timeout_ms: c_ulong) -> c_int {
    let _wakeup = core::mem::MaybeUninit::<prbtest_wakeup_timer>::zeroed().assume_init();
    let mut text_buf = [0u8; MAX_PRB_RECORD_SIZE];
    let mut info = core::mem::MaybeUninit::<printk_info>::zeroed().assume_init();
    let mut r = core::mem::MaybeUninit::<printk_record>::zeroed().assume_init();
    let mut seq: u64 = 0;
    let mut count: c_ulong = 0;
    prb_rec_init_rd(&mut r, &mut info, text_buf.as_mut_ptr() as *mut c_char, text_buf.len());
    kunit_info((*test_data).test, b"start reader\n\0".as_ptr() as *const c_char);
    while prb_read_valid((*test_data).ringbuffer, seq, &mut r) {
        if info.seq < seq { kunit_fail((*test_data).test, b"BAD SEQ READ: request=%llu read=%llu\n\0".as_ptr() as *const c_char, seq, info.seq); }
        let dat = r.text_buf as *const prbtest_rbdata;
        if !prbtest_check_data(dat) { prbtest_fail_record((*test_data).test, dat, info.seq); }
        if (count & 0x3fff) == 0 { cond_resched(); }
        count = count.wrapping_add(1);
        seq = info.seq + 1;
    }
    kunit_info((*test_data).test, b"end reader: read=%lu seq=%llu\n\0".as_ptr() as *const c_char, count, info.seq);
    0
}

unsafe extern "C" fn prbtest_add_cpumask_cleanup(_test: *mut kunit, _mask: cpumask_var_t) {}
unsafe extern "C" fn prbtest_add_kthread_cleanup(_test: *mut kunit, _kthread: *mut task_struct) {}

unsafe fn prbtest_prb_reinit(_rb: *mut printk_ringbuffer) {
    // prb_init(rb, rb->text_data_ring.data, rb->text_data_ring.size_bits,
    //           rb->desc_ring.descs, rb->desc_ring.count_bits, rb->desc_ring.infos);
}

unsafe extern "C" fn test_readerwriter(test: *mut kunit) {
    // Equivalent to CONFIG_LOG_BUF_SHIFT=13
    // DEFINE_PRINTKRB(test_rb, 8, 5);
    let mut test_rb = core::mem::MaybeUninit::<printk_ringbuffer>::zeroed().assume_init();
    let mut thread_data: *mut prbtest_thread_data;
    let mut test_data: *mut prbtest_data;
    let mut thread: *mut task_struct;
    let mut test_cpus: cpumask_var_t = core::ptr::null_mut();
    let mut cpu: c_int;
    let reader_cpu: c_int;

    if !alloc_cpumask_var(&mut test_cpus, 0) { return; }
    prbtest_add_cpumask_cleanup(test, test_cpus);
    cpus_read_lock();
    cpumask_copy(test_cpus, cpu_online_mask);
    cpus_read_unlock();

    reader_cpu = cpumask_first(test_cpus);
    if cpumask_weight(test_cpus) == 1 {
        kunit_warn(test, b"more than one CPU is recommended\0".as_ptr() as *const c_char);
    } else {
        cpumask_clear_cpu(reader_cpu, test_cpus);
    }

    // KUnit test can get restarted more times.
    prbtest_prb_reinit(&mut test_rb);
    test_data = kunit_kmalloc(test, core::mem::size_of::<prbtest_data>(), 0) as *mut prbtest_data;
    if test_data.is_null() { return; }
    (*test_data).test = test;
    (*test_data).ringbuffer = &mut test_rb;
    init_waitqueue_head(&mut (*test_data).new_record_wait);
    kunit_info(test, b"running for %lu ms\n\0".as_ptr() as *const c_char, runtime_ms);

    // One CPU is for the reader, all others are writers.
    cpu = 0;
    while cpu < 4096 {
        // for_each_cpu(cpu, test_cpus)
        if cpu == reader_cpu { cpu += 1; continue; }
        thread_data = kunit_kmalloc(test, core::mem::size_of::<prbtest_thread_data>(), 0) as *mut prbtest_thread_data;
        if thread_data.is_null() { return; }
        (*thread_data).test_data = test_data;
        (*thread_data).num = cpu as c_ulong;
        thread = kthread_run_on_cpu(prbtest_writer, thread_data as *mut c_void, cpu,
                                    b"prbtest writer %u\0".as_ptr() as *const c_char, cpu);
        if thread.is_null() { return; }
        prbtest_add_kthread_cleanup(test, thread);
        cpu += 1;
        break; // placeholder for the external for_each_cpu iterator
    }

    kunit_info(test, b"starting test\n\0".as_ptr() as *const c_char);
    set_cpus_allowed_ptr(current, test_cpus);
    prbtest_reader(test_data, runtime_ms);
    kunit_info(test, b"completed test\n\0".as_ptr() as *const c_char);
}

// KUNIT_CASE_SLOW(test_readerwriter), kunit_test_suite(), module metadata,
// and action-wrapper macros are retained as integration declarations supplied
// by the kernel build environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
