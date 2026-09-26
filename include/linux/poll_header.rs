/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies: linux/compiler.h, linux/ktime.h, linux/wait.h, linux/string.h,
// linux/fs.h, linux/uaccess.h, uapi/linux/poll.h, uapi/linux/eventpoll.h.


/* ~832 bytes of stack space used max in sys_select/sys_poll before allocating
   additional memory. */
pub const MAX_STACK_ALLOC: usize = 832;
pub const FRONTEND_STACK_ALLOC: usize = 256;
pub const SELECT_STACK_ALLOC: usize = FRONTEND_STACK_ALLOC;
pub const POLL_STACK_ALLOC: usize = FRONTEND_STACK_ALLOC;
pub const WQUEUES_STACK_ALLOC: usize = MAX_STACK_ALLOC - FRONTEND_STACK_ALLOC;
pub const N_INLINE_POLL_ENTRIES: usize = WQUEUES_STACK_ALLOC / core::mem::size_of::<poll_table_entry>();

pub const DEFAULT_POLLMASK: __poll_t = EPOLLIN | EPOLLOUT | EPOLLRDNORM | EPOLLWRNORM;

/*
 * structures and helpers for f_op->poll implementations
 */
pub type poll_queue_proc =
    Option<unsafe extern "C" fn(*mut file, *mut wait_queue_head_t, *mut poll_table_struct)>;

/*
 * Do not touch the structure directly, use the access function
 * poll_requested_events() instead.
 */
#[repr(C)]
pub struct poll_table_struct {
    pub _qproc: poll_queue_proc,
    pub _key: __poll_t,
}
pub type poll_table = poll_table_struct;

#[inline]
pub unsafe fn poll_wait(filp: *mut file, wait_address: *mut wait_queue_head_t, p: *mut poll_table) {
    if p.is_null() {
        return;
    }
    if let Some(qproc) = (*p)._qproc {
        qproc(filp, wait_address, p);
        /*
         * This memory barrier is paired in the wq_has_sleeper().
         * See the comment above prepare_to_wait(), we need to
         * ensure that subsequent tests in this thread can't be
         * reordered with __add_wait_queue() in _qproc() paths.
         */
        smp_mb();
    }
}

/*
 * Return the set of events that the application wants to poll for.
 * This is useful for drivers that need to know whether a DMA transfer has
 * to be started implicitly on poll(). You typically only want to do that
 * if the application is actually polling for POLLIN and/or POLLOUT.
 */
#[inline]
pub unsafe fn poll_requested_events(p: *const poll_table) -> __poll_t {
    if p.is_null() { !0 } else { (*p)._key }
}

#[inline]
pub unsafe fn init_poll_funcptr(pt: *mut poll_table, qproc: poll_queue_proc) {
    (*pt)._qproc = qproc;
    (*pt)._key = !0; /* all events enabled */
}

#[inline]
pub unsafe fn file_can_poll(file: *mut file) -> bool {
    (*(*file).f_op).poll.is_some()
}

#[inline]
pub unsafe fn vfs_poll(file: *mut file, pt: *mut poll_table_struct) -> __poll_t {
    match (*(*file).f_op).poll {
        Some(poll) => poll(file, pt),
        None => DEFAULT_POLLMASK,
    }
}

#[repr(C)]
pub struct poll_table_entry {
    pub filp: *mut file,
    pub key: __poll_t,
    pub wait: wait_queue_entry_t,
    pub wait_address: *mut wait_queue_head_t,
}

/*
 * Structures and helpers for select/poll syscall
 */
#[repr(C)]
pub struct poll_wqueues {
    pub pt: poll_table,
    pub table: *mut poll_table_page,
    pub polling_task: *mut task_struct,
    pub triggered: core::ffi::c_int,
    pub error: core::ffi::c_int,
    pub inline_index: core::ffi::c_int,
    pub inline_entries: [poll_table_entry; N_INLINE_POLL_ENTRIES],
}

extern "C" {
    pub fn poll_initwait(pwq: *mut poll_wqueues);
    pub fn poll_freewait(pwq: *mut poll_wqueues);
    pub fn select_estimate_accuracy(tv: *mut timespec64) -> u64;
}

pub const MAX_INT64_SECONDS: s64 = ((!0u64 >> 1) as s64 / HZ as s64) - 1;

extern "C" {
    pub fn core_sys_select(
        n: core::ffi::c_int,
        inp: *mut fd_set,
        outp: *mut fd_set,
        exp: *mut fd_set,
        end_time: *mut timespec64,
    ) -> core::ffi::c_int;

    pub fn poll_select_set_timeout(to: *mut timespec64, sec: time64_t, nsec: core::ffi::c_long) -> core::ffi::c_int;
}

/// `__MAP(v, from, to)`: move the single-bit flag `from` in `v` to `to`.
#[inline(always)]
const fn __MAP(v: u16, from: u16, to: u16) -> u16 {
    if from < to { (v & from) * (to / from) } else { (v & from) / (from / to) }
}

#[inline]
pub fn mangle_poll(val: __poll_t) -> u16 {
    let v = val as u16;
    macro_rules! M {
        ($x:ident) => {
            ::kernel::macros::paste!(__MAP(v, [<EPOLL $x>] as u16, [<POLL $x>] as u16))
        };
    }
    M!(IN) | M!(OUT) | M!(PRI) | M!(ERR) | M!(NVAL) | M!(RDNORM) | M!(RDBAND)
        | M!(WRNORM) | M!(WRBAND) | M!(HUP) | M!(RDHUP) | M!(MSG)
}

#[inline]
pub fn demangle_poll(val: u16) -> __poll_t {
    macro_rules! M {
        ($x:ident) => {
            ::kernel::macros::paste!(__MAP(val, [<POLL $x>] as u16, [<EPOLL $x>] as u16) as __poll_t)
        };
    }
    M!(IN) | M!(OUT) | M!(PRI) | M!(ERR) | M!(NVAL) | M!(RDNORM) | M!(RDBAND)
        | M!(WRNORM) | M!(WRBAND) | M!(HUP) | M!(RDHUP) | M!(MSG)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
