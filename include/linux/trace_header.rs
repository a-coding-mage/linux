/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: _LINUX_TRACE_H */

pub const TRACE_EXPORT_FUNCTION: u32 = 1u32 << 0;
pub const TRACE_EXPORT_EVENT: u32 = 1u32 << 1;
pub const TRACE_EXPORT_MARKER: u32 = 1u32 << 2;

/*
 * The trace export - an export of Ftrace output. The trace_export
 * can process traces and export them to a registered destination as
 * an addition to the current only output of Ftrace - i.e. ring buffer.
 *
 * If you want traces to be sent to some other place rather than ring
 * buffer only, just need to register a new trace_export and implement
 * its own .write() function for writing traces to the storage.
 *
 * next		- pointer to the next trace_export
 * write	- copy traces which have been delt with ->commit() to
 *		  the destination
 * flags	- which ftrace to be exported
 */
#[repr(C)]
pub struct trace_export {
    /* __rcu */
    pub next: *mut trace_export,
    pub write: Option<unsafe extern "C" fn(*mut trace_export, *const core::ffi::c_void, u32)>,
    pub flags: i32,
}

#[repr(C)]
pub struct trace_array {
    _private: [u8; 0],
}

/* CONFIG_TRACING declarations. */
extern "C" {
    pub fn register_ftrace_export(export: *mut trace_export) -> i32;
    pub fn unregister_ftrace_export(export: *mut trace_export) -> i32;

    /**
     * trace_array_puts - write a constant string into the trace buffer.
     * @tr:    The trace array to write to.
     * @str:   The constant string to write
     */
    pub fn __trace_array_puts(
        tr: *mut trace_array,
        ip: libc::c_ulong,
        str_: *const libc::c_char,
        size: libc::c_int,
    ) -> libc::c_int;

    pub fn trace_printk_init_buffers();
    pub fn trace_array_printk(
        tr: *mut trace_array,
        ip: libc::c_ulong,
        fmt: *const libc::c_char,
        ...,
    ) -> libc::c_int;
    pub fn trace_array_init_printk(tr: *mut trace_array) -> libc::c_int;
    pub fn trace_array_put(tr: *mut trace_array);
    pub fn trace_array_get_by_name(
        name: *const libc::c_char,
        systems: *const libc::c_char,
    ) -> *mut trace_array;
    pub fn trace_array_destroy(tr: *mut trace_array) -> libc::c_int;

    /* For osnoise tracer */
    pub fn osnoise_arch_register() -> libc::c_int;
    pub fn osnoise_arch_unregister();
    pub fn osnoise_trace_irq_entry(id: libc::c_int);
    pub fn osnoise_trace_irq_exit(id: libc::c_int, desc: *const libc::c_char);
}

/* The following declarations correspond to the !CONFIG_TRACING branch. */
#[cfg(not(feature = "CONFIG_TRACING"))]
pub unsafe fn register_ftrace_export_disabled(_export: *mut trace_export) -> libc::c_int {
    -libc::EINVAL
}

#[cfg(not(feature = "CONFIG_TRACING"))]
pub unsafe fn unregister_ftrace_export_disabled(_export: *mut trace_export) -> libc::c_int {
    0
}

#[cfg(not(feature = "CONFIG_TRACING"))]
pub unsafe fn trace_printk_init_buffers_disabled() {}

#[cfg(not(feature = "CONFIG_TRACING"))]
pub unsafe fn trace_array_printk_disabled(
    _tr: *mut trace_array,
    _ip: libc::c_ulong,
    _fmt: *const libc::c_char,
    _args: ...,
) -> libc::c_int {
    0
}

#[cfg(not(feature = "CONFIG_TRACING"))]
pub unsafe fn trace_array_init_printk_disabled(_tr: *mut trace_array) -> libc::c_int {
    -libc::EINVAL
}

#[cfg(not(feature = "CONFIG_TRACING"))]
pub unsafe fn trace_array_put_disabled(_tr: *mut trace_array) {}

#[cfg(not(feature = "CONFIG_TRACING"))]
pub unsafe fn trace_array_get_by_name_disabled(
    _name: *const libc::c_char,
    _systems: *const libc::c_char,
) -> *mut trace_array {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_TRACING"))]
pub unsafe fn trace_array_destroy_disabled(_tr: *mut trace_array) -> libc::c_int {
    0
}

/* Equivalent of the C trace_array_puts macro; strlen and _THIS_IP_ are external dependencies. */
#[macro_export]
macro_rules! trace_array_puts {
    ($tr:expr, $str:expr) => {{
        if !$str.is_null() {
            unsafe {
                $crate::__trace_array_puts(
                    $tr,
                    _THIS_IP_,
                    $str,
                    libc::strlen($str) as libc::c_int,
                )
            }
        } else {
            -1
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
