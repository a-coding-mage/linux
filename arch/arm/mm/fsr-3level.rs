// SPDX-License-Identifier: GPL-2.0

// External declarations and constants are supplied by the surrounding kernel
// translation unit.
extern "C" {
    fn do_bad();
    fn do_translation_fault();
    fn do_page_fault();
}

#[repr(C)]
pub struct fsr_info {
    pub fn_: unsafe extern "C" fn(),
    pub sig: i32,
    pub code: i32,
    pub name: *const u8,
}

// The following names correspond to the C signal and bus-error constants.
extern "C" {
    static SIGBUS: i32;
    static SIGSEGV: i32;
    static SEGV_MAPERR: i32;
    static SEGV_ACCERR: i32;
    static BUS_ADRALN: i32;
}

#[allow(non_upper_case_globals)]
pub static mut fsr_info_table: [fsr_info; 64] = [
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 0\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 1\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 2\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 3\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"reserved translation fault\0".as_ptr() },
    fsr_info { fn_: do_translation_fault, sig: unsafe { SIGSEGV }, code: unsafe { SEGV_MAPERR }, name: b"level 1 translation fault\0".as_ptr() },
    fsr_info { fn_: do_translation_fault, sig: unsafe { SIGSEGV }, code: unsafe { SEGV_MAPERR }, name: b"level 2 translation fault\0".as_ptr() },
    fsr_info { fn_: do_page_fault, sig: unsafe { SIGSEGV }, code: unsafe { SEGV_MAPERR }, name: b"level 3 translation fault\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"reserved access flag fault\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGSEGV }, code: unsafe { SEGV_ACCERR }, name: b"level 1 access flag fault\0".as_ptr() },
    fsr_info { fn_: do_page_fault, sig: unsafe { SIGSEGV }, code: unsafe { SEGV_ACCERR }, name: b"level 2 access flag fault\0".as_ptr() },
    fsr_info { fn_: do_page_fault, sig: unsafe { SIGSEGV }, code: unsafe { SEGV_ACCERR }, name: b"level 3 access flag fault\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"reserved permission fault\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGSEGV }, code: unsafe { SEGV_ACCERR }, name: b"level 1 permission fault\0".as_ptr() },
    fsr_info { fn_: do_page_fault, sig: unsafe { SIGSEGV }, code: unsafe { SEGV_ACCERR }, name: b"level 2 permission fault\0".as_ptr() },
    fsr_info { fn_: do_page_fault, sig: unsafe { SIGSEGV }, code: unsafe { SEGV_ACCERR }, name: b"level 3 permission fault\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"synchronous external abort\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"asynchronous external abort\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 18\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 19\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"synchronous abort (translation table walk)\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"synchronous abort (translation table walk)\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"synchronous abort (translation table walk)\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"synchronous abort (translation table walk)\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"synchronous parity error\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"asynchronous parity error\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 26\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 27\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"synchronous parity error (translation table walk\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"synchronous parity error (translation table walk\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"synchronous parity error (translation table walk\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"synchronous parity error (translation table walk\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 32\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: unsafe { BUS_ADRALN }, name: b"alignment fault\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"debug event\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 35\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 36\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 37\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 38\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 39\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 40\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 41\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 42\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 43\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 44\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 45\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 46\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 47\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 48\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 49\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 50\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 51\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"implementation fault (lockdown abort)\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 53\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 54\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 55\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 56\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 57\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"implementation fault (coprocessor abort)\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 59\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 60\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 61\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 62\0".as_ptr() },
    fsr_info { fn_: do_bad, sig: unsafe { SIGBUS }, code: 0, name: b"unknown 63\0".as_ptr() },
];

// #define ifsr_info fsr_info
pub use fsr_info_table as ifsr_info;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
