// SPDX-License-Identifier: GPL-2.0
//
// Kernel dependencies supplied by the surrounding translation unit:
// linux/init.h, linux/kernel_stat.h, linux/proc_fs.h, linux/seq_file.h,
// and internal.h.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct SeqFile {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ProcDirEntry {
    _private: [u8; 0],
}

extern "C" {
    static softirq_to_name: *const *const c_char;

    fn seq_puts(p: *mut SeqFile, s: *const c_char);
    fn seq_printf(p: *mut SeqFile, format: *const c_char, ...);
    fn seq_putc(p: *mut SeqFile, c: c_int);
    fn seq_put_decimal_ull_width(
        p: *mut SeqFile,
        delimiter: *const c_char,
        value: u64,
        width: c_int,
    );
    fn kstat_softirqs_cpu(softirq: c_int, cpu: c_int) -> u64;
    fn proc_create_single(
        name: *const c_char,
        mode: c_int,
        parent: *mut ProcDirEntry,
        show: unsafe extern "C" fn(*mut SeqFile, *mut c_void) -> c_int,
    ) -> *mut ProcDirEntry;
    fn pde_make_permanent(pde: *mut ProcDirEntry);
}

/*
 * /proc/softirqs  ... display the number of softirqs
 */
unsafe extern "C" fn show_softirqs(p: *mut SeqFile, _v: *mut c_void) -> c_int {
    let mut i: c_int;
    let mut j: c_int;

    seq_puts(p, c"                    ".as_ptr());
    for_each_possible_cpu!(cpu, {
        seq_printf(p, c"CPU%-8d".as_ptr(), cpu);
    });
    seq_putc(p, b'\n' as c_int);

    i = 0;
    while (i < NR_SOFTIRQS as c_int) {
        seq_printf(p, c"%12s:".as_ptr(), *softirq_to_name.add(i as usize));
        for_each_possible_cpu!(cpu, {
            seq_put_decimal_ull_width(
                p,
                c" ".as_ptr(),
                kstat_softirqs_cpu(i, cpu),
                10,
            );
        });
        seq_putc(p, b'\n' as c_int);
        i += 1;
    }
    0
}

unsafe extern "C" fn proc_softirqs_init() -> c_int {
    let pde: *mut ProcDirEntry;

    pde = proc_create_single(c"softirqs".as_ptr(), 0, core::ptr::null_mut(), show_softirqs);
    pde_make_permanent(pde);
    0
}

// fs_initcall(proc_softirqs_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
