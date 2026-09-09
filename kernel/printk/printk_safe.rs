// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * printk_safe.c - Safe printk for printk-deadlock-prone contexts
 */

use core::ffi::{c_char, c_int, c_void};

// Declarations supplied by the surrounding kernel translation.
#[repr(C)]
pub struct atomic_t {
    pub counter: c_int,
}

pub type va_list = *mut c_void;

extern "C" {
    fn atomic_inc(v: *mut atomic_t);
    fn atomic_dec(v: *mut atomic_t);
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn force_legacy_kthread() -> bool;
    fn this_cpu_read_printk_context() -> c_int;
    fn this_cpu_inc_printk_context();
    fn this_cpu_dec_printk_context();
    fn cant_migrate();
    fn in_nmi() -> bool;
    fn is_printk_cpu_sync_owner() -> bool;
    fn vprintk_default(fmt: *const c_char, args: va_list) -> c_int;
    fn vkdb_printf(src: c_int, fmt: *const c_char, args: va_list) -> c_int;
}

// Context where printk messages are never suppressed
static mut force_con: atomic_t = atomic_t { counter: 0 };

#[no_mangle]
pub unsafe extern "C" fn printk_force_console_enter() {
    atomic_inc(&raw mut force_con);
}

#[no_mangle]
pub unsafe extern "C" fn printk_force_console_exit() {
    atomic_dec(&raw mut force_con);
}

#[no_mangle]
pub unsafe extern "C" fn is_printk_force_console() -> bool {
    atomic_read(&raw const force_con) != 0
}

static mut printk_context: c_int = 0;

// Can be preempted by NMI.
#[no_mangle]
pub unsafe extern "C" fn __printk_safe_enter() {
    this_cpu_inc_printk_context();
}

// Can be preempted by NMI.
#[no_mangle]
pub unsafe extern "C" fn __printk_safe_exit() {
    this_cpu_dec_printk_context();
}

#[no_mangle]
pub unsafe extern "C" fn __printk_deferred_enter() {
    cant_migrate();
    __printk_safe_enter();
}

#[no_mangle]
pub unsafe extern "C" fn __printk_deferred_exit() {
    cant_migrate();
    __printk_safe_exit();
}

#[no_mangle]
pub unsafe extern "C" fn is_printk_legacy_deferred() -> bool {
    /*
     * The per-CPU variable @printk_context can be read safely in any
     * context. CPU migration is always disabled when set.
     *
     * A context holding the printk_cpu_sync must not spin waiting for
     * another CPU. For legacy printing, it could be the console_lock
     * or the port lock.
     */
    force_legacy_kthread()
        || this_cpu_read_printk_context() != 0
        || in_nmi()
        || is_printk_cpu_sync_owner()
}

#[no_mangle]
pub unsafe extern "C" fn vprintk(fmt: *const c_char, args: va_list) -> c_int {
    // CONFIG_KGDB_KDB: allow printk() to pass to kdb but avoid recursion.
    // The conditional is retained as a build-time dependency boundary.
    #[cfg(feature = "CONFIG_KGDB_KDB")]
    {
        extern "C" {
            static mut kdb_trap_printk: bool;
            static mut kdb_printf_cpu: c_int;
        }
        const KDB_MSGSRC_PRINTK: c_int = 0;
        if kdb_trap_printk && kdb_printf_cpu < 0 {
            return vkdb_printf(KDB_MSGSRC_PRINTK, fmt, args);
        }
    }
    vprintk_default(fmt, args)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
