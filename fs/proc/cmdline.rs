// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel and internal.h are declared here
// as external symbols.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proc_dir_entry {
    pub size: u64,
}

unsafe extern "C" {
    static saved_command_line: *const c_char;
    static saved_command_line_len: usize;

    fn seq_puts(m: *mut seq_file, s: *const c_char);
    fn seq_putc(m: *mut seq_file, c: c_int);
    fn proc_create_single(
        name: *const c_char,
        mode: u16,
        parent: *mut proc_dir_entry,
        show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int,
    ) -> *mut proc_dir_entry;
    fn pde_make_permanent(pde: *mut proc_dir_entry);
}

unsafe extern "C" fn cmdline_proc_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    unsafe {
        seq_puts(m, saved_command_line);
        seq_putc(m, b'\n' as c_int);
    }
    0
}

unsafe extern "C" fn proc_cmdline_init() -> c_int {
    let pde: *mut proc_dir_entry;

    unsafe {
        static CMDLINE: &[u8] = b"cmdline\0";
        pde = proc_create_single(
            CMDLINE.as_ptr() as *const c_char,
            0,
            core::ptr::null_mut(),
            cmdline_proc_show,
        );
        pde_make_permanent(pde);
        (*pde).size = saved_command_line_len as u64 + 1;
    }
    0
}

// fs_initcall(proc_cmdline_init): register proc_cmdline_init for the
// filesystem initialization phase using the kernel's build-time mechanism.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
