// SPDX-License-Identifier: GPL-2.0
/*
 * Test interpreter for the binfmt_misc_bpf selftest. A bpf-backed 'B' handler
 * routes a matched binary here; printing this marker proves the program's
 * chosen interpreter actually ran.
 */

unsafe extern "C" {
    fn write(fd: i32, buf: *const core::ffi::c_void, count: usize) -> isize;
}

fn main() {
    let argc = std::env::args_os().count() as i32;
    let argv: *mut *mut u8 = core::ptr::null_mut();

    let _ = argc;
    let _ = argv;
    unsafe {
        write(
            1,
            b"BPF_INTERP_RAN\n".as_ptr() as *const core::ffi::c_void,
            15,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
