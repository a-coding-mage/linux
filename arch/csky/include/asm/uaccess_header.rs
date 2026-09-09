/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard and include directives are intentionally omitted. */

/* __put_user_fn */
unsafe extern "C" {
    pub fn __put_user_bad() -> core::ffi::c_int;
}

/* The following macros preserve the C inline-assembly operations and their
 * exception-table intent.  The C source targets the C-SKY assembler. */
macro_rules! __put_user_asm_b {
    ($x:expr, $ptr:expr, $err:expr) => {{
        let mut errcode: i32;
        core::arch::asm!(
            "1: stb {x}, ({ptr},0)", "br 3f", "2: mov {err}, {ec}", "br 3f",
            ".section __ex_table, \"a\"", ".align 2", ".long 1b,2b", ".previous", "3:",
            x = inout(reg) $x, ptr = inout(reg) $ptr, err = inout(reg) $err,
            ec = inout(reg) (-EFAULT), options(raw)
        );
        let _ = &mut errcode;
    }};
}

macro_rules! __put_user_asm_h {
    ($x:expr, $ptr:expr, $err:expr) => {{
        let mut errcode: i32;
        core::arch::asm!(
            "1: sth {x}, ({ptr},0)", "br 3f", "2: mov {err}, {ec}", "br 3f",
            ".section __ex_table, \"a\"", ".align 2", ".long 1b,2b", ".previous", "3:",
            x = inout(reg) $x, ptr = inout(reg) $ptr, err = inout(reg) $err,
            ec = inout(reg) (-EFAULT), options(raw)
        );
        let _ = &mut errcode;
    }};
}

macro_rules! __put_user_asm_w {
    ($x:expr, $ptr:expr, $err:expr) => {{
        let mut errcode: i32;
        core::arch::asm!(
            "1: stw {x}, ({ptr},0)", "br 3f", "2: mov {err}, {ec}", "br 3f",
            ".section __ex_table,\"a\"", ".align 2", ".long 1b, 2b", ".previous", "3:",
            x = inout(reg) $x, ptr = inout(reg) $ptr, err = inout(reg) $err,
            ec = inout(reg) (-EFAULT), options(raw)
        );
        let _ = &mut errcode;
    }};
}

macro_rules! __put_user_asm_64 {
    ($x:expr, $ptr:expr, $err:expr) => {{
        let mut tmp: i32 = 0;
        let mut errcode: i32;
        core::arch::asm!(
            "ldw {tmp}, ({x}, 0)", "1: stw {tmp}, ({ptr}, 0)",
            "ldw {tmp}, ({x}, 4)", "2: stw {tmp}, ({ptr}, 4)", "br 4f",
            "3: mov {err}, {ec}", "br 4f", ".section __ex_table, \"a\"",
            ".align 2", ".long 1b, 3b", ".long 2b, 3b", ".previous", "4:",
            x = inout(reg) $x, ptr = inout(reg) $ptr, err = inout(reg) $err,
            tmp = inout(reg) tmp, ec = inout(reg) (-EFAULT), options(raw)
        );
        let _ = &mut errcode;
    }};
}

pub unsafe fn __put_user_fn(size: usize, ptr: *mut core::ffi::c_void, x: *mut core::ffi::c_void) -> i32 {
    let mut retval: i32 = 0;
    let mut tmp: u32;
    match size {
        1 => { tmp = *(x as *const u8) as u32; __put_user_asm_b!(tmp, ptr, retval); }
        2 => { tmp = *(x as *const u16) as u32; __put_user_asm_h!(tmp, ptr, retval); }
        4 => { tmp = *(x as *const u32); __put_user_asm_w!(tmp, ptr, retval); }
        8 => { __put_user_asm_64!(x, ptr as *mut u64, retval); }
        _ => {}
    }
    retval
}

/* __put_user_fn is also exposed under its identical C macro name. */

unsafe extern "C" {
    pub fn __get_user_bad() -> core::ffi::c_int;
}

macro_rules! __get_user_asm_common {
    ($x:expr, $ptr:expr, $ins:tt, $err:expr) => {{
        let mut errcode: i32;
        core::arch::asm!(concat!("1: ", $ins, " {x}, ({ptr}, 0)"), "br 3f",
            "2: mov {err}, {ec}", "movi {x}, 0", "br 3f",
            ".section __ex_table,\"a\"", ".align 2", ".long 1b, 2b", ".previous", "3:",
            x = inout(reg) $x, ptr = in(reg) $ptr, err = inout(reg) $err,
            ec = inout(reg) (-EFAULT), options(raw));
        let _ = &mut errcode;
    }};
}

macro_rules! __get_user_asm_64 {
    ($x:expr, $ptr:expr, $err:expr) => {{
        let mut tmp: i32 = 0;
        let mut errcode: i32;
        core::arch::asm!(
            "1: ldw {tmp}, ({ptr}, 0)", "stw {tmp}, ({x}, 0)",
            "2: ldw {tmp}, ({ptr}, 4)", "stw {tmp}, ({x}, 4)", "br 4f",
            "3: mov {err}, {ec}", "br 4f", ".section __ex_table, \"a\"",
            ".align 2", ".long 1b, 3b", ".long 2b, 3b", ".previous", "4:",
            x = inout(reg) $x, ptr = in(reg) $ptr, err = inout(reg) $err,
            tmp = inout(reg) tmp, ec = inout(reg) (-EFAULT), options(raw));
        let _ = &mut errcode;
    }};
}

pub unsafe fn __get_user_fn(size: usize, ptr: *const core::ffi::c_void, x: *mut core::ffi::c_void) -> i32 {
    let mut retval: i32 = 0;
    let mut tmp: u32 = 0;
    match size {
        1 => { __get_user_asm_common!(tmp, ptr, "ldb", retval); *(x as *mut u8) = tmp as u8; }
        2 => { __get_user_asm_common!(tmp, ptr, "ldh", retval); *(x as *mut u16) = tmp as u16; }
        4 => { __get_user_asm_common!(tmp, ptr, "ldw", retval); *(x as *mut u32) = tmp; }
        8 => { __get_user_asm_64!(x, ptr, retval); }
        _ => {}
    }
    retval
}

pub unsafe extern "C" fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: c_ulong) -> c_ulong;
pub unsafe extern "C" fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: c_ulong) -> c_ulong;
pub unsafe extern "C" fn __clear_user(to: *mut core::ffi::c_void, n: c_ulong) -> c_ulong;

/* <asm-generic/uaccess.h> supplies the generic uaccess declarations. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
