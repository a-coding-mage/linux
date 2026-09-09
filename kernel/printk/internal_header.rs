/* SPDX-License-Identifier: GPL-2.0-or-later */
/* internal.h - printk internal definitions */

// C dependencies supplied by other translation units:
// linux/console.h, linux/types.h, linux/sysctl.h

#[cfg(all(CONFIG_PRINTK, CONFIG_SYSCTL))]
extern "C" {
    pub fn printk_sysctl_init();
    pub fn devkmsg_sysctl_set_loglvl(
        table: *const ctl_table,
        write: c_int,
        buffer: *mut c_void,
        lenp: *mut usize,
        ppos: *mut loff_t,
    ) -> c_int;
}

#[cfg(not(all(CONFIG_PRINTK, CONFIG_SYSCTL)))]
#[inline]
pub unsafe fn printk_sysctl_init() {}

// The C con_printk() macro expands printk() with the supplied format and
// console metadata; callers should preserve that expansion at call sites.

#[cfg(CONFIG_PREEMPT_RT)]
#[inline]
pub const fn force_legacy_kthread() -> bool { true }
#[cfg(not(CONFIG_PREEMPT_RT))]
#[inline]
pub const fn force_legacy_kthread() -> bool { false }

#[cfg(CONFIG_PRINTK)]
pub const PRINTK_PREFIX_MAX: usize = if cfg!(CONFIG_PRINTK_CALLER) { 48 } else { 32 };
#[cfg(not(CONFIG_PRINTK))]
pub const PRINTK_PREFIX_MAX: usize = 0;
#[cfg(CONFIG_PRINTK)]
pub const PRINTK_MESSAGE_MAX: usize = 2048;
#[cfg(not(CONFIG_PRINTK))]
pub const PRINTK_MESSAGE_MAX: usize = 0;
#[cfg(CONFIG_PRINTK)]
pub const PRINTKRB_RECORD_MAX: usize = 1024;
#[cfg(not(CONFIG_PRINTK))]
pub const PRINTKRB_RECORD_MAX: usize = 0;

#[repr(C)]
pub enum printk_info_flags {
    LOG_FORCE_CON = 1,
    LOG_NEWLINE = 2,
    LOG_CONT = 8,
}

#[repr(C)] pub struct printk_ringbuffer { _private: [u8; 0] }
#[repr(C)] pub struct dev_printk_info { _private: [u8; 0] }

extern "C" {
    pub static mut prb: *mut printk_ringbuffer;
    pub static mut printk_kthreads_running: bool;
    pub static mut printk_kthreads_ready: bool;
    pub static mut debug_non_panic_cpus: bool;
}

extern "C" {
    pub fn vprintk_store(facility: c_int, level: c_int, dev_info: *const dev_printk_info,
                         fmt: *const c_char, args: va_list) -> c_int;
    pub fn vprintk_default(fmt: *const c_char, args: va_list) -> c_int;
    pub fn __printk_safe_enter();
    pub fn __printk_safe_exit();
    pub fn printk_percpu_data_ready() -> bool;
    pub fn defer_console_output();
    pub fn is_printk_legacy_deferred() -> bool;
    pub fn is_printk_force_console() -> bool;
    pub fn printk_parse_prefix(text: *const c_char, level: *mut c_int,
                               flags: *mut printk_info_flags) -> u16;
    pub fn console_lock_spinning_enable();
    pub fn console_lock_spinning_disable_and_check(cookie: c_int) -> c_int;
    pub fn nbcon_seq_read(con: *mut console) -> u64;
    pub fn nbcon_seq_force(con: *mut console, seq: u64);
    pub fn nbcon_alloc(con: *mut console) -> bool;
    pub fn nbcon_free(con: *mut console);
    pub fn nbcon_get_default_prio() -> nbcon_prio;
    pub fn nbcon_atomic_flush_pending();
    pub fn nbcon_legacy_emit_next_record(con: *mut console, handover: *mut bool,
                                         cookie: c_int, use_atomic: bool) -> bool;
    pub fn nbcon_kthread_create(con: *mut console) -> bool;
    pub fn nbcon_kthread_stop(con: *mut console);
    pub fn nbcon_kthreads_wake();
}

#[inline]
pub unsafe fn nbcon_kthread_wake(con: *mut console) {
    rcuwait_wake_up(&mut (*con).rcuwait);
}

#[cfg(not(CONFIG_PRINTK))]
pub static mut printk_kthreads_running: bool = false;
#[cfg(not(CONFIG_PRINTK))]
pub static mut printk_kthreads_ready: bool = false;
#[cfg(not(CONFIG_PRINTK))]
#[inline] pub unsafe fn printk_percpu_data_ready() -> bool { false }
#[cfg(not(CONFIG_PRINTK))]
#[inline] pub unsafe fn defer_console_output() {}
#[cfg(not(CONFIG_PRINTK))]
#[inline] pub unsafe fn is_printk_legacy_deferred() -> bool { false }
#[cfg(not(CONFIG_PRINTK))]
#[inline] pub unsafe fn nbcon_seq_read(_: *mut console) -> u64 { 0 }
#[cfg(not(CONFIG_PRINTK))]
#[inline] pub unsafe fn nbcon_seq_force(_: *mut console, _: u64) {}
#[cfg(not(CONFIG_PRINTK))]
#[inline] pub unsafe fn nbcon_alloc(_: *mut console) -> bool { false }
#[cfg(not(CONFIG_PRINTK))]
#[inline] pub unsafe fn nbcon_free(_: *mut console) {}
#[cfg(not(CONFIG_PRINTK))]
#[inline] pub unsafe fn nbcon_get_default_prio() -> nbcon_prio { NBCON_PRIO_NONE }
#[cfg(not(CONFIG_PRINTK))]
#[inline] pub unsafe fn nbcon_atomic_flush_pending() {}
#[cfg(not(CONFIG_PRINTK))]
#[inline] pub unsafe fn nbcon_legacy_emit_next_record(_: *mut console, _: *mut bool, _: c_int, _: bool) -> bool { false }
#[cfg(not(CONFIG_PRINTK))]
#[inline] pub unsafe fn nbcon_kthread_wake(_: *mut console) {}
#[cfg(not(CONFIG_PRINTK))]
#[inline] pub unsafe fn nbcon_kthreads_wake() {}

extern "C" {
    pub static mut have_boot_console: bool;
    pub static mut have_nbcon_console: bool;
    pub static mut have_legacy_console: bool;
    pub static mut legacy_allow_panic_sync: bool;
    pub static mut console_irqwork_blocked: bool;
}

#[repr(C)]
pub struct console_flush_type {
    pub nbcon_atomic: bool,
    pub nbcon_offload: bool,
    pub legacy_direct: bool,
    pub legacy_offload: bool,
}

#[inline]
pub unsafe fn printk_get_console_flush_type(ft: *mut console_flush_type) {
    *ft = console_flush_type { nbcon_atomic: false, nbcon_offload: false,
        legacy_direct: false, legacy_offload: false };
    match nbcon_get_default_prio() {
        NBCON_PRIO_NORMAL | NBCON_PRIO_EMERGENCY | NBCON_PRIO_PANIC => {
            if have_nbcon_console && !have_boot_console { (*ft).nbcon_atomic = true; }
            if have_legacy_console || have_boot_console {
                if !is_printk_legacy_deferred() { (*ft).legacy_direct = true; }
                else if !console_irqwork_blocked { (*ft).legacy_offload = true; }
            }
        }
        _ => WARN_ON_ONCE(1),
    }
}

extern "C" { pub static mut printk_shared_pbufs: printk_buffers; }
#[repr(C)] pub struct printk_buffers {
    pub outbuf: [c_char; PRINTK_MESSAGE_MAX],
    pub scratchbuf: [c_char; PRINTKRB_RECORD_MAX],
}
#[repr(C)] pub struct printk_message {
    pub pbufs: *mut printk_buffers,
    pub outbuf_len: c_uint,
    pub seq: u64,
    pub dropped: c_ulong,
    #[cfg(CONFIG_PRINTK_EXECUTION_CTX)] pub cpu: c_int,
    #[cfg(CONFIG_PRINTK_EXECUTION_CTX)] pub pid: pid_t,
    #[cfg(CONFIG_PRINTK_EXECUTION_CTX)] pub comm: [c_char; TASK_COMM_LEN],
}

extern "C" {
    pub fn printk_get_next_message(pmsg: *mut printk_message, seq: u64,
                                   is_extended: bool, may_supress: bool) -> bool;
    #[cfg(CONFIG_PRINTK)] pub fn console_prepend_dropped(pmsg: *mut printk_message, dropped: c_ulong);
    #[cfg(CONFIG_PRINTK)] pub fn console_prepend_replay(pmsg: *mut printk_message);
    pub fn is_printk_cpu_sync_owner() -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
