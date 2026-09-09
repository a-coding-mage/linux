/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// linux/compiler.h, linux/ktime.h, linux/wait.h, linux/string.h,
// linux/fs.h, linux/uaccess.h, uapi/linux/poll.h, uapi/linux/eventpoll.h

/* ~832 bytes of stack space used max in sys_select/sys_poll before allocating
   additional memory. */
pub const MAX_STACK_ALLOC: usize = 832;
pub const FRONTEND_STACK_ALLOC: usize = 256;
pub const SELECT_STACK_ALLOC: usize = FRONTEND_STACK_ALLOC;
pub const POLL_STACK_ALLOC: usize = FRONTEND_STACK_ALLOC;
pub const WQUEUES_STACK_ALLOC: usize = MAX_STACK_ALLOC - FRONTEND_STACK_ALLOC;
pub const N_INLINE_POLL_ENTRIES: usize =
    WQUEUES_STACK_ALLOC / core::mem::size_of::<poll_table_entry>();

pub const DEFAULT_POLLMASK: __poll_t =
    EPOLLIN | EPOLLOUT | EPOLLRDNORM | EPOLLWRNORM;

pub struct poll_table_struct;

/*
 * structures and helpers for f_op->poll implementations
 */
pub type poll_queue_proc = unsafe extern "C" fn(
    *mut file,
    *mut wait_queue_head_t,
    *mut poll_table_struct,
);

/*
 * Do not touch the structure directly, use the access function
 * poll_requested_events() instead.
 */
#[repr(C)]
pub struct poll_table {
    pub _qproc: poll_queue_proc,
    pub _key: __poll_t,
}

pub unsafe fn poll_wait(
    filp: *mut file,
    wait_address: *mut wait_queue_head_t,
    p: *mut poll_table,
) {
    if !p.is_null() && !(*p)._qproc as *const () .is_null() {
        ((*p)._qproc)(filp, wait_address, p.cast::<poll_table_struct>());
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
pub unsafe fn poll_requested_events(p: *const poll_table) -> __poll_t {
    if p.is_null() { !(__poll_t::MAX) } else { (*p)._key }
}

pub unsafe fn init_poll_funcptr(pt: *mut poll_table, qproc: poll_queue_proc) {
    (*pt)._qproc = qproc;
    (*pt)._key = !(__poll_t::MAX); /* all events enabled */
}

pub unsafe fn file_can_poll(file: *mut file) -> bool {
    !(*file).f_op.poll.is_null()
}

pub unsafe fn vfs_poll(file: *mut file, pt: *mut poll_table_struct) -> __poll_t {
    if !(*file).f_op.poll.is_null() {
        (*file).f_op.poll(file, pt)
    } else {
        DEFAULT_POLLMASK
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

unsafe extern "C" {
    pub fn poll_initwait(pwq: *mut poll_wqueues);
    pub fn poll_freewait(pwq: *mut poll_wqueues);
    pub fn select_estimate_accuracy(tv: *mut timespec64) -> u64;
}

pub const MAX_INT64_SECONDS: s64 = ((((!0u64) >> 1) as s64 / HZ) - 1);

unsafe extern "C" {
    pub fn core_sys_select(
        n: core::ffi::c_int,
        inp: *mut fd_set,
        outp: *mut fd_set,
        exp: *mut fd_set,
        end_time: *mut timespec64,
    ) -> core::ffi::c_int;

    pub fn poll_select_set_timeout(
        to: *mut timespec64,
        sec: time64_t,
        nsec: core::ffi::c_long,
    ) -> core::ffi::c_int;
}

pub unsafe fn mangle_poll(val: __poll_t) -> u16 {
    let v = val as u16;
    ((v & (EPOLLIN as u16)) * (POLLIN as u16 / EPOLLIN as u16)) |
        ((v & (EPOLLOUT as u16)) * (POLLOUT as u16 / EPOLLOUT as u16)) |
        ((v & (EPOLLPRI as u16)) * (POLLPRI as u16 / EPOLLPRI as u16)) |
        ((v & (EPOLLERR as u16)) * (POLLERR as u16 / EPOLLERR as u16)) |
        ((v & (EPOLLNVAL as u16)) * (POLLNVAL as u16 / EPOLLNVAL as u16)) |
        ((v & (EPOLLRDNORM as u16)) * (POLLRDNORM as u16 / EPOLLRDNORM as u16)) |
        ((v & (EPOLLRDBAND as u16)) * (POLLRDBAND as u16 / EPOLLRDBAND as u16)) |
        ((v & (EPOLLWRNORM as u16)) * (POLLWRNORM as u16 / EPOLLWRNORM as u16)) |
        ((v & (EPOLLWRBAND as u16)) * (POLLWRBAND as u16 / EPOLLWRBAND as u16)) |
        ((v & (EPOLLHUP as u16)) * (POLLHUP as u16 / EPOLLHUP as u16)) |
        ((v & (EPOLLRDHUP as u16)) * (POLLRDHUP as u16 / EPOLLRDHUP as u16)) |
        ((v & (EPOLLMSG as u16)) * (POLLMSG as u16 / EPOLLMSG as u16))
}

pub unsafe fn demangle_poll(val: u16) -> __poll_t {
    ((val & POLLIN as u16) / (POLLIN as u16 / EPOLLIN as u16)) as __poll_t |
        ((val & POLLOUT as u16) / (POLLOUT as u16 / EPOLLOUT as u16)) as __poll_t |
        ((val & POLLPRI as u16) / (POLLPRI as u16 / EPOLLPRI as u16)) as __poll_t |
        ((val & POLLERR as u16) / (POLLERR as u16 / EPOLLERR as u16)) as __poll_t |
        ((val & POLLNVAL as u16) / (POLLNVAL as u16 / EPOLLNVAL as u16)) as __poll_t |
        ((val & POLLRDNORM as u16) / (POLLRDNORM as u16 / EPOLLRDNORM as u16)) as __poll_t |
        ((val & POLLRDBAND as u16) / (POLLRDBAND as u16 / EPOLLRDBAND as u16)) as __poll_t |
        ((val & POLLWRNORM as u16) / (POLLWRNORM as u16 / EPOLLWRNORM as u16)) as __poll_t |
        ((val & POLLWRBAND as u16) / (POLLWRBAND as u16 / EPOLLWRBAND as u16)) as __poll_t |
        ((val & POLLHUP as u16) / (POLLHUP as u16 / EPOLLHUP as u16)) as __poll_t |
        ((val & POLLRDHUP as u16) / (POLLRDHUP as u16 / EPOLLRDHUP as u16)) as __poll_t |
        ((val & POLLMSG as u16) / (POLLMSG as u16 / EPOLLMSG as u16)) as __poll_t
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
