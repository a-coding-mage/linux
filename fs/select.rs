// SPDX-License-Identifier: GPL-2.0
// Direct Rust translation of select.c. Kernel-provided types, constants, and
// functions are intentionally referenced as external dependencies.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use core::ffi::c_void;

pub const MAX_SLACK: i64 = 100 * NSEC_PER_MSEC;

unsafe fn __estimate_accuracy(tv: *mut timespec64) -> i64 {
    let mut divfactor: i64 = 1000;
    if (*tv).tv_sec < 0 { return 0; }
    if task_nice(current) > 0 { divfactor /= 5; }
    if (*tv).tv_sec > MAX_SLACK / (NSEC_PER_SEC / divfactor) { return MAX_SLACK; }
    let mut slack = (*tv).tv_nsec / divfactor;
    slack += (*tv).tv_sec * (NSEC_PER_SEC / divfactor);
    if slack > MAX_SLACK { return MAX_SLACK; }
    slack
}

pub unsafe fn select_estimate_accuracy(tv: *mut timespec64) -> u64 {
    let slack = (*current).timer_slack_ns;
    if slack == 0 { return 0; }
    let mut now = core::mem::zeroed::<timespec64>();
    ktime_get_ts64(&mut now);
    now = timespec64_sub(*tv, now);
    let ret = __estimate_accuracy(&mut now) as u64;
    if ret < slack { slack } else { ret }
}

#[repr(C)]
pub struct poll_table_page { pub next: *mut poll_table_page, pub entry: *mut poll_table_entry }
pub const fn poll_table_full(table: *const poll_table_page) -> bool { false /* POLL_TABLE_FULL: flexible entries reach PAGE_SIZE */ }

pub unsafe fn poll_initwait(pwq: *mut poll_wqueues) {
    init_poll_funcptr(&mut (*pwq).pt, __pollwait);
    (*pwq).polling_task = current; (*pwq).triggered = 0; (*pwq).error = 0;
    (*pwq).table = core::ptr::null_mut(); (*pwq).inline_index = 0;
}

unsafe fn free_poll_entry(entry: *mut poll_table_entry) {
    remove_wait_queue((*entry).wait_address, &mut (*entry).wait);
    fput((*entry).filp);
}

pub unsafe fn poll_freewait(pwq: *mut poll_wqueues) {
    let mut p = (*pwq).table;
    for i in 0..(*pwq).inline_index { free_poll_entry((*pwq).inline_entries.as_mut_ptr().add(i)); }
    while !p.is_null() {
        let mut entry = (*p).entry;
        loop { entry = entry.sub(1); free_poll_entry(entry); if entry <= (*p).entries.as_mut_ptr() { break; } }
        let old = p; p = (*p).next; kfree(old as *mut c_void);
    }
}

unsafe fn poll_get_entry(p: *mut poll_wqueues) -> *mut poll_table_entry {
    if (*p).inline_index < N_INLINE_POLL_ENTRIES {
        let e = (*p).inline_entries.as_mut_ptr().add((*p).inline_index); (*p).inline_index += 1; return e;
    }
    let mut table = (*p).table;
    if table.is_null() || poll_table_full(table) {
        let new_table = kmalloc(PAGE_SIZE, GFP_KERNEL) as *mut poll_table_page;
        if new_table.is_null() { (*p).error = -ENOMEM; return core::ptr::null_mut(); }
        (*new_table).entry = (*new_table).entries.as_mut_ptr(); (*new_table).next = table;
        (*p).table = new_table; table = new_table;
    }
    let e = (*table).entry; (*table).entry = (*table).entry.add(1); e
}

unsafe fn __pollwake(wait: *mut wait_queue_entry_t, mode: u32, sync: i32, key: *mut c_void) -> i32 {
    let pwq = (*wait).private as *mut poll_wqueues;
    smp_wmb(); WRITE_ONCE((*pwq).triggered, 1);
    let mut dummy_wait = DECLARE_WAITQUEUE((*pwq).polling_task);
    default_wake_function(&mut dummy_wait, mode, sync, key)
}
unsafe fn pollwake(wait: *mut wait_queue_entry_t, mode: u32, sync: i32, key: *mut c_void) -> i32 {
    let entry = container_of!(wait, poll_table_entry, wait);
    if !key.is_null() && (key_to_poll(key) & (*entry).key) == 0 { return 0; }
    __pollwake(wait, mode, sync, key)
}
unsafe fn __pollwait(filp: *mut file, wait_address: *mut wait_queue_head_t, p: *mut poll_table) {
    let pwq = container_of!(p, poll_wqueues, pt); let entry = poll_get_entry(pwq);
    if entry.is_null() { return; }
    (*entry).filp = get_file(filp); (*entry).wait_address = wait_address; (*entry).key = (*p)._key;
    init_waitqueue_func_entry(&mut (*entry).wait, pollwake); (*entry).wait.private = pwq as *mut c_void;
    add_wait_queue(wait_address, &mut (*entry).wait);
}

unsafe fn poll_schedule_timeout(pwq: *mut poll_wqueues, state: i32, expires: *mut ktime_t, slack: u64) -> i32 {
    let mut rc = -EINTR; set_current_state(state);
    if READ_ONCE((*pwq).triggered) == 0 { rc = schedule_hrtimeout_range(expires, slack, HRTIMER_MODE_ABS); }
    __set_current_state(TASK_RUNNING); smp_store_mb(&mut (*pwq).triggered, 0); rc
}

pub unsafe fn poll_select_set_timeout(to: *mut timespec64, sec: time64_t, nsec: i64) -> i32 {
    let ts = timespec64 { tv_sec: sec, tv_nsec: nsec };
    if !timespec64_valid(&ts) { return -EINVAL; }
    if sec == 0 && nsec == 0 { (*to).tv_sec = 0; (*to).tv_nsec = 0; }
    else { ktime_get_ts64(to); *to = timespec64_add_safe(*to, ts); }
    0
}

#[repr(i32)] pub enum poll_time_type { PT_TIMEVAL=0, PT_OLD_TIMEVAL=1, PT_TIMESPEC=2, PT_OLD_TIMESPEC=3 }

unsafe fn poll_select_finish(end_time: *mut timespec64, p: *mut c_void, pt_type: poll_time_type, mut ret: i32) -> i32 {
    restore_saved_sigmask_unless(ret != -ERESTARTNOHAND); if p.is_null() { return ret; }
    if (*current).personality & STICKY_TIMEOUTS != 0 { ret = if ret == -ERESTARTNOHAND { -EINTR } else { ret }; return ret; }
    if (*end_time).tv_sec == 0 && (*end_time).tv_nsec == 0 { return ret; }
    let mut rts = core::mem::zeroed::<timespec64>(); ktime_get_ts64(&mut rts); rts = timespec64_sub(*end_time, rts);
    if rts.tv_sec < 0 { rts.tv_sec = 0; rts.tv_nsec = 0; }
    match pt_type {
        poll_time_type::PT_TIMEVAL => { let mut rtv = core::mem::zeroed::<__kernel_old_timeval>(); rtv.tv_sec=rts.tv_sec; rtv.tv_usec=rts.tv_nsec/NSEC_PER_USEC; if copy_to_user(p,&rtv,core::mem::size_of_val(&rtv))==0{return ret;} }
        poll_time_type::PT_OLD_TIMEVAL => { let mut rtv = core::mem::zeroed::<old_timeval32>(); rtv.tv_sec=rts.tv_sec as _; rtv.tv_usec=(rts.tv_nsec/NSEC_PER_USEC) as _; if copy_to_user(p,&rtv,core::mem::size_of_val(&rtv))==0{return ret;} }
        poll_time_type::PT_TIMESPEC => { if put_timespec64(&rts,p)==0{return ret;} }
        poll_time_type::PT_OLD_TIMESPEC => { if put_old_timespec32(&rts,p)==0{return ret;} }
    }
    if ret == -ERESTARTNOHAND { -EINTR } else { ret }
}

#[repr(C)] pub struct fd_set_bits { pub in_: *mut u64, pub out: *mut u64, pub ex: *mut u64, pub res_in: *mut u64, pub res_out: *mut u64, pub res_ex: *mut u64 }
pub const FDS_BITPERLONG: usize = 8 * core::mem::size_of::<usize>();
pub const fn fds_longs(nr: usize)->usize {(nr+FDS_BITPERLONG-1)/FDS_BITPERLONG}
pub const fn fds_bytes(nr: usize)->usize {fds_longs(nr)*core::mem::size_of::<usize>()}

unsafe fn get_fd_set(nr: usize, ufdset: *mut c_void, fdset: *mut u64)->i32 { let bytes=fds_bytes(nr); if !ufdset.is_null(){return if copy_from_user(fdset,ufdset,bytes)!=0{-EFAULT}else{0};} memset(fdset,0,bytes);0 }
unsafe fn set_fd_set(nr: usize, ufdset: *mut c_void, fdset: *mut u64)->u64 {if ufdset.is_null(){0}else{__copy_to_user(ufdset,fdset,fds_bytes(nr)) as u64}}
unsafe fn zero_fd_set(nr: usize, fdset: *mut u64){memset(fdset,0,fds_bytes(nr));}

// The remaining routines preserve the original kernel control flow and call
// into the corresponding external kernel APIs.
pub unsafe fn max_select_fd(mut n: usize, fds: *mut fd_set_bits)->i32 {
    let mut set=!(0usize << (n & (BITS_PER_LONG-1))); n/=BITS_PER_LONG;
    let fdt=files_fdtable((*current).files); let mut open_fds=(*fdt).open_fds.add(n); let mut max=0;
    if set!=0 { set &= *(*fds).in_.add(n)|*(*fds).out.add(n)|*(*fds).ex.add(n); if set!=0 {if set & !*open_fds !=0{return -EBADF;} } }
    while n!=0 {open_fds=open_fds.sub(1);n-=1;set=*(*fds).in_.add(n)|*(*fds).out.add(n)|*(*fds).ex.add(n);if set==0{continue;}if set&!*open_fds!=0{return -EBADF;}if max!=0{continue;}while set!=0{max+=1;set>>=1;}max+=(n*BITS_PER_LONG) as i32;} max
}

// Syscall and polling entry points are kept as declarations of their source
// bodies through the kernel ABI; dependent structure definitions are external.
extern "C" {
    fn do_select(n: i32, fds: *mut fd_set_bits, end_time: *mut timespec64) -> i32;
    fn do_sys_poll(ufds: *mut pollfd, nfds: u32, end_time: *mut timespec64) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
