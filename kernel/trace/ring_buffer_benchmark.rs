// SPDX-License-Identifier: GPL-2.0
/*
 * ring buffer tester and benchmark
 *
 * Copyright (C) 2009 Steven Rostedt <srostedt@redhat.com>
 */
// Kernel dependencies are supplied by the surrounding build.

#[repr(C)]
struct rb_page {
    ts: u64,
    commit: local_t,
    data: [std::ffi::c_char; 4080],
}

/* run time and sleep time in seconds */
const RUN_TIME: u64 = 10;
const SLEEP_TIME: i32 = 10;

/* number of events for writer to wake up the reader */
static mut wakeup_interval: i32 = 100;

static mut reader_finish: i32 = 0;
static mut read_start: completion = DECLARE_COMPLETION!();
static mut read_done: completion = DECLARE_COMPLETION!();

static mut buffer: *mut trace_buffer = std::ptr::null_mut();
static mut producer: *mut task_struct = std::ptr::null_mut();
static mut consumer: *mut task_struct = std::ptr::null_mut();
static mut read: c_ulong = 0;

static mut disable_reader: c_uint = 0;
module_param!(disable_reader, uint, 0644);
module_param_desc!(disable_reader, "only run producer");

static mut write_iteration: c_uint = 50;
module_param!(write_iteration, uint, 0644);
module_param_desc!(write_iteration, "# of writes between timestamp readings");

static mut producer_nice: i32 = MAX_NICE;
static mut consumer_nice: i32 = MAX_NICE;

static mut producer_fifo: i32 = 0;
static mut consumer_fifo: i32 = 0;

module_param!(producer_nice, int, 0644);
module_param_desc!(producer_nice, "nice prio for producer");
module_param!(consumer_nice, int, 0644);
module_param_desc!(consumer_nice, "nice prio for consumer");
module_param!(producer_fifo, int, 0644);
module_param_desc!(producer_fifo, "use fifo for producer: 0 - disabled, 1 - low prio, 2 - fifo");
module_param!(consumer_fifo, int, 0644);
module_param_desc!(consumer_fifo, "use fifo for consumer: 0 - disabled, 1 - low prio, 2 - fifo");

static mut read_events: i32 = 0;
static mut test_error: i32 = 0;

macro_rules! TEST_ERROR {
    () => {{
        unsafe {
            if test_error == 0 {
                test_error = 1;
                WARN_ON!(1);
            }
        }
    }};
}

#[repr(C)]
enum event_status { EVENT_FOUND, EVENT_DROPPED }

unsafe fn break_test() -> bool {
    test_error != 0 || kthread_should_stop()
}

unsafe fn read_event(cpu: i32) -> event_status {
    let mut ts: u64 = 0;
    let event = ring_buffer_consume(buffer, cpu, &mut ts, std::ptr::null_mut());
    if event.is_null() { return event_status::EVENT_DROPPED; }
    let entry = ring_buffer_event_data(event) as *mut i32;
    if *entry != cpu { TEST_ERROR!(); return event_status::EVENT_DROPPED; }
    read += 1;
    event_status::EVENT_FOUND
}

unsafe fn read_page(cpu: i32) -> event_status {
    let bpage = ring_buffer_alloc_read_page(buffer, cpu);
    if IS_ERR!(bpage) { return event_status::EVENT_DROPPED; }
    let page_size = ring_buffer_subbuf_size_get(buffer);
    let ret = ring_buffer_read_page(buffer, bpage, page_size, cpu, 1);
    if ret >= 0 {
        let rpage = ring_buffer_read_page_data(bpage) as *mut rb_page;
        let commit = local_read(&(*rpage).commit) & 0xfffff;
        let mut i = 0;
        let mut inc: i32;
        while i < commit && test_error == 0 {
            if i >= page_size - std::mem::offset_of!(rb_page, data) { TEST_ERROR!(); break; }
            inc = -1;
            let event = (&mut (*rpage).data[i as usize]) as *mut _ as *mut ring_buffer_event;
            match (*event).type_len {
                RINGBUF_TYPE_PADDING => {
                    if (*event).time_delta == 0 { TEST_ERROR!(); }
                    inc = (*event).array[0] as i32 + 4;
                }
                RINGBUF_TYPE_TIME_EXTEND => inc = 8,
                0 => {
                    let entry = ring_buffer_event_data(event) as *mut i32;
                    if *entry != cpu { TEST_ERROR!(); break; }
                    read += 1;
                    if (*event).array[0] == 0 { TEST_ERROR!(); break; }
                    inc = (*event).array[0] as i32 + 4;
                }
                _ => {
                    let entry = ring_buffer_event_data(event) as *mut i32;
                    if *entry != cpu { TEST_ERROR!(); break; }
                    read += 1;
                    inc = ((*event).type_len as i32 + 1) * 4;
                }
            }
            if test_error != 0 { break; }
            if inc <= 0 { TEST_ERROR!(); break; }
            i += inc;
        }
    }
    ring_buffer_free_read_page(buffer, cpu, bpage);
    if ret < 0 { event_status::EVENT_DROPPED } else { event_status::EVENT_FOUND }
}

unsafe fn ring_buffer_consumer() {
    read_events ^= 1;
    read = 0;
    while READ_ONCE!(reader_finish) == 0 {
        let mut found = 1;
        while found != 0 && test_error == 0 {
            found = 0;
            for_each_online_cpu!(cpu, {
                let stat = if read_events != 0 { read_event(cpu) } else { read_page(cpu) };
                if test_error != 0 { break; }
                if matches!(stat, event_status::EVENT_FOUND) { found = 1; }
            });
        }
        set_current_state!(TASK_INTERRUPTIBLE);
        if reader_finish != 0 { break; }
        schedule!();
    }
    __set_current_state!(TASK_RUNNING);
    reader_finish = 0;
    complete!(&mut read_done);
}

unsafe fn ring_buffer_producer() {
    let start_time = ktime_get();
    let mut end_time;
    let timeout = ktime_add_ns(start_time, RUN_TIME * NSEC_PER_SEC);
    let mut missed: c_ulonglong = 0;
    let mut hit: c_ulonglong = 0;
    let mut cnt = 0;
    trace_printk!("Starting ring buffer hammer\n");
    loop {
        for _ in 0..write_iteration {
            let event = ring_buffer_lock_reserve(buffer, 10);
            if event.is_null() { missed += 1; } else {
                hit += 1;
                *(ring_buffer_event_data(event) as *mut i32) = smp_processor_id();
                ring_buffer_unlock_commit(buffer);
            }
        }
        end_time = ktime_get();
        cnt += 1;
        if !consumer.is_null() && cnt % wakeup_interval == 0 { wake_up_process!(consumer); }
        // #ifndef CONFIG_PREEMPTION: preserve the conditional intent from the C source.
        if cnt % wakeup_interval != 0 { cond_resched!(); }
        if !(ktime_before(end_time, timeout) && !break_test()) { break; }
    }
    trace_printk!("End ring buffer hammer\n");
    if !consumer.is_null() {
        init_completion!(&mut read_start); init_completion!(&mut read_done); smp_wmb!();
        reader_finish = 1; wake_up_process!(consumer); wait_for_completion!(&mut read_done);
    }
    let mut time = ktime_us_delta(end_time, start_time);
    let entries = ring_buffer_entries(buffer); let overruns = ring_buffer_overruns(buffer);
    if test_error != 0 { trace_printk!("ERROR!\n"); }
    if disable_reader == 0 { trace_printk!("Running Consumer\n"); }
    trace_printk!("Time:     %lld (usecs)\n", time); trace_printk!("Overruns: %lld\n", overruns);
    if disable_reader != 0 { trace_printk!("Read:     (reader disabled)\n"); }
    else { trace_printk!("Read:     %ld  (by %s)\n", read, if read_events != 0 { "events" } else { "pages" }); }
    trace_printk!("Entries:  %lld\n", entries); trace_printk!("Total:    %lld\n", entries + overruns + read);
    trace_printk!("Missed:   %ld\n", missed); trace_printk!("Hit:      %ld\n", hit);
    time = do_div!(time, USEC_PER_MSEC);
    if time != 0 { hit /= time as c_ulonglong; } else { trace_printk!("TIME IS ZERO??\n"); }
    trace_printk!("Entries per millisec: %ld\n", hit);
    if hit != 0 { trace_printk!("%ld ns per entry\n", NSEC_PER_MSEC / hit); }
    if missed != 0 {
        if time != 0 { missed /= time as c_ulonglong; }
        trace_printk!("Total iterations per millisec: %ld\n", hit + missed);
        if hit + missed == 0 { trace_printk!("hit + missed overflowed and totalled zero!\n"); hit -= 1; }
        trace_printk!("%ld ns per entry\n", NSEC_PER_MSEC / (hit + missed));
    }
}

unsafe fn wait_to_die() { set_current_state!(TASK_INTERRUPTIBLE); while !kthread_should_stop() { schedule!(); set_current_state!(TASK_INTERRUPTIBLE); } __set_current_state!(TASK_RUNNING); }

unsafe extern "C" fn ring_buffer_consumer_thread(_arg: *mut std::ffi::c_void) -> i32 {
    while !break_test() { complete!(&mut read_start); ring_buffer_consumer(); set_current_state!(TASK_INTERRUPTIBLE); if break_test() { break; } schedule!(); }
    __set_current_state!(TASK_RUNNING); if !kthread_should_stop() { wait_to_die(); } 0
}

unsafe extern "C" fn ring_buffer_producer_thread(_arg: *mut std::ffi::c_void) -> i32 {
    while !break_test() { ring_buffer_reset(buffer); if !consumer.is_null() { wake_up_process!(consumer); wait_for_completion!(&mut read_start); } ring_buffer_producer(); if break_test() { break; } trace_printk!("Sleeping for 10 secs\n"); set_current_state!(TASK_INTERRUPTIBLE); if break_test() { break; } schedule_timeout!(HZ * SLEEP_TIME); }
    __set_current_state!(TASK_RUNNING); if !kthread_should_stop() { wait_to_die(); } 0
}

unsafe extern "C" fn ring_buffer_benchmark_init() -> i32 {
    buffer = ring_buffer_alloc(1000000, RB_FL_OVERWRITE); if buffer.is_null() { return -ENOMEM; }
    if disable_reader == 0 { consumer = kthread_create(ring_buffer_consumer_thread, std::ptr::null_mut(), "rb_consumer"); if IS_ERR!(consumer) { ring_buffer_free(buffer); return PTR_ERR!(consumer); } }
    producer = kthread_run(ring_buffer_producer_thread, std::ptr::null_mut(), "rb_producer"); if IS_ERR!(producer) { if !consumer.is_null() { kthread_stop(consumer); } ring_buffer_free(buffer); return PTR_ERR!(producer); }
    if disable_reader == 0 { if consumer_fifo >= 2 { sched_set_fifo(consumer); } else if consumer_fifo == 1 { sched_set_fifo_low(consumer); } else { set_user_nice(consumer, consumer_nice); } }
    if producer_fifo >= 2 { sched_set_fifo(producer); } else if producer_fifo == 1 { sched_set_fifo_low(producer); } else { set_user_nice(producer, producer_nice); }
    0
}

unsafe extern "C" fn ring_buffer_benchmark_exit() { kthread_stop(producer); if !consumer.is_null() { kthread_stop(consumer); } ring_buffer_free(buffer); }

module_init!(ring_buffer_benchmark_init);
module_exit!(ring_buffer_benchmark_exit);
module_author!("Steven Rostedt");
module_description!("ring_buffer_benchmark");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
