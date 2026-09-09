// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux/UML environment:
// kmsg_dump.h, spinlock.h, console.h, string.h, shared/init.h,
// shared/kern.h, and os.h.

static mut KMSG_DUMPER_STDOUT_ITER: kmsg_dump_iter = kmsg_dump_iter::default();
static KMSG_DUMPER_STDOUT_LOCK: spinlock_t = DEFINE_SPINLOCK!();
static mut KMSG_DUMPER_STDOUT_LINE: [c_char; 1024] = [0; 1024];

unsafe extern "C" {
    fn console_srcu_read_lock() -> c_int;
    fn console_srcu_read_unlock(cookie: c_int);
    fn console_srcu_read_flags(con: *mut console) -> c_uint;
    fn console_is_usable(con: *mut console, flags: c_uint, use_preferred: bool) -> bool;
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
    fn spin_trylock_irqsave(lock: *const spinlock_t, flags: *mut c_ulong) -> bool;
    fn spin_unlock_irqrestore(lock: *const spinlock_t, flags: c_ulong);
    fn kmsg_dump_rewind(iter: *mut kmsg_dump_iter);
    fn kmsg_dump_get_line(
        iter: *mut kmsg_dump_iter,
        syslog: bool,
        line: *mut c_char,
        size: usize,
        len: *mut usize,
    ) -> bool;
    fn printf(format: *const c_char, ...);
    fn kmsg_dump_register(dumper: *mut kmsg_dumper) -> c_int;
}

#[allow(non_camel_case_types)]
type c_char = i8;
#[allow(non_camel_case_types)]
type c_int = i32;
#[allow(non_camel_case_types)]
type c_uint = u32;
#[allow(non_camel_case_types)]
type c_ulong = usize;

// These types and macros are provided by the included kernel headers.
#[allow(non_camel_case_types)]
struct kmsg_dump_iter;
#[allow(non_camel_case_types)]
struct spinlock_t;
#[allow(non_camel_case_types)]
struct console {
    name: *const c_char,
}
#[allow(non_camel_case_types)]
struct kmsg_dump_detail;
#[allow(non_camel_case_types)]
struct kmsg_dumper {
    dump: Option<unsafe extern "C" fn(*mut kmsg_dumper, *mut kmsg_dump_detail)>,
}

unsafe fn kmsg_dumper_stdout(
    _dumper: *mut kmsg_dumper,
    _detail: *mut kmsg_dump_detail,
) {
    let mut con: *mut console = core::ptr::null_mut();
    let mut flags: c_ulong;
    let mut len: usize = 0;
    let mut cookie: c_int;

    /*
     * If no consoles are available to output crash information, dump
     * the kmsg buffer to stdout.
     */

    cookie = console_srcu_read_lock();
    // Equivalent of for_each_console_srcu(con).
    for_each_console_srcu!(con, {
        /*
         * The ttynull console and disabled consoles are ignored
         * since they cannot output. All other consoles are
         * expected to output the crash information.
         */
        if strcmp((*con).name, b"ttynull\0".as_ptr() as *const c_char) != 0
            && console_is_usable(con, console_srcu_read_flags(con), true)
        {
            break;
        }
    });
    console_srcu_read_unlock(cookie);
    if !con.is_null() {
        return;
    }

    if !spin_trylock_irqsave(&KMSG_DUMPER_STDOUT_LOCK, &mut flags) {
        return;
    }

    kmsg_dump_rewind(&mut KMSG_DUMPER_STDOUT_ITER);

    printf(b"kmsg_dump:\n\0".as_ptr() as *const c_char);
    while kmsg_dump_get_line(
        &mut KMSG_DUMPER_STDOUT_ITER,
        true,
        KMSG_DUMPER_STDOUT_LINE.as_mut_ptr(),
        core::mem::size_of_val(&KMSG_DUMPER_STDOUT_LINE),
        &mut len,
    ) {
        KMSG_DUMPER_STDOUT_LINE[len] = 0;
        printf(b"%s\0".as_ptr() as *const c_char, KMSG_DUMPER_STDOUT_LINE.as_ptr());
    }

    spin_unlock_irqrestore(&KMSG_DUMPER_STDOUT_LOCK, flags);
}

static mut KMSG_DUMPER: kmsg_dumper = kmsg_dumper {
    dump: Some(kmsg_dumper_stdout),
};

unsafe extern "C" fn kmsg_dumper_stdout_init() -> c_int {
    kmsg_dump_register(&mut KMSG_DUMPER)
}

// __uml_postsetup(kmsg_dumper_stdout_init);
uml_postsetup!(kmsg_dumper_stdout_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
