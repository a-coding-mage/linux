// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux kernel and chan_user.h are intentionally
// left as external Rust items.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct console {
    _private: [u8; 0],
}

extern "C" {
    fn generic_write(fd: c_int, string: *const c_char, len: c_uint, arg: *mut c_void);
    fn register_console(console: *mut console);
    fn unregister_console(console: *mut console);
    fn simple_strtoul(string: *const c_char, endp: *mut *mut c_char, base: c_uint) -> c_uint;
}

// CON_PRINTBUFFER is supplied by linux/console.h.
const CON_PRINTBUFFER: c_uint = 0x00000001;

/* ----------------------------------------------------------------------------- */
/* trivial console driver -- simply dump everything to stderr                    */

/*
 * Don't register by default -- as this registers very early in the
 * boot process it becomes the default console.
 *
 * Initialized at init time.
 */
static mut use_stderr_console: c_int = 0;

unsafe extern "C" fn stderr_console_write(
    _console: *mut console,
    string: *const c_char,
    len: c_uint,
) {
    generic_write(2 /* stderr */, string, len, core::ptr::null_mut());
}

// The remaining fields and the callback ABI correspond to struct console from
// linux/console.h; they are supplied by the surrounding kernel translation.
#[repr(C)]
struct stderr_console_type {
    name: *const c_char,
    write: Option<unsafe extern "C" fn(*mut console, *const c_char, c_uint)>,
    flags: c_uint,
}

static mut stderr_console: stderr_console_type = stderr_console_type {
    name: b"stderr\0".as_ptr() as *const c_char,
    write: Some(stderr_console_write),
    flags: CON_PRINTBUFFER,
};

unsafe extern "C" fn stderr_console_init() -> c_int {
    if use_stderr_console != 0 {
        register_console(&raw mut stderr_console as *mut stderr_console_type as *mut console);
    }
    0
}

// console_initcall(stderr_console_init);

unsafe extern "C" fn stderr_setup(mut str_: *mut c_char) -> c_int {
    if str_.is_null() {
        return 0;
    }
    use_stderr_console = simple_strtoul(
        str_ as *const c_char,
        &mut str_,
        0,
    ) as c_int;
    1
}

// __setup("stderr=", stderr_setup);

/* The previous behavior of not unregistering led to /dev/console being
 * impossible to open.  My FC5 filesystem started having init die, and the
 * system panicing because of this.  Unregistering causes the real
 * console to become the default console, and /dev/console can then be
 * opened.  Making this an initcall makes this happen late enough that
 * there is no added value in dumping everything to stderr, and the
 * normal console is good enough to show you all available output.
 */
unsafe extern "C" fn unregister_stderr() -> c_int {
    unregister_console(&raw mut stderr_console as *mut stderr_console_type as *mut console);

    0
}

// __initcall(unregister_stderr);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
