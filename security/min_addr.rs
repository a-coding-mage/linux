// SPDX-License-Identifier: GPL-2.0
// Dependencies from:
// <linux/init.h>
// <linux/mm.h>
// <linux/security.h>
// <linux/sysctl.h>
// <linux/minmax.h>

use core::ffi::{c_char, c_int, c_void};

type SizeT = usize;
type LoffT = i64;

#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: SizeT,
    pub mode: u16,
    pub proc_handler: Option<
        unsafe extern "C" fn(
            table: *const ctl_table,
            write: c_int,
            buffer: *mut c_void,
            lenp: *mut SizeT,
            ppos: *mut LoffT,
        ) -> c_int,
    >,
}

unsafe extern "C" {
    static CAP_SYS_RAWIO: c_int;
    static EPERM: c_int;

    fn capable(cap: c_int) -> bool;
    fn proc_doulongvec_minmax(
        table: *const ctl_table,
        write: c_int,
        buffer: *mut c_void,
        lenp: *mut SizeT,
        ppos: *mut LoffT,
    ) -> c_int;
    fn register_sysctl_init(path: *const c_char, table: *const ctl_table);
    fn umax(a: u64, b: u64) -> u64;
}

/* amount of vm to protect from userspace access by both DAC and the LSM*/
#[no_mangle]
pub static mut mmap_min_addr: u64 = 0;
/* amount of vm to protect from userspace using CAP_SYS_RAWIO (DAC) */
#[no_mangle]
pub static mut dac_mmap_min_addr: u64 = CONFIG_DEFAULT_MMAP_MIN_ADDR;
/* amount of vm to protect from userspace using the LSM = CONFIG_LSM_MMAP_MIN_ADDR */

/*
 * Update mmap_min_addr = max(dac_mmap_min_addr, CONFIG_LSM_MMAP_MIN_ADDR)
 */
unsafe fn update_mmap_min_addr() {
    // C conditional: #ifdef CONFIG_LSM_MMAP_MIN_ADDR
    #[cfg(CONFIG_LSM_MMAP_MIN_ADDR)]
    {
        mmap_min_addr = umax(dac_mmap_min_addr, CONFIG_LSM_MMAP_MIN_ADDR);
    }

    // C conditional: #else
    #[cfg(not(CONFIG_LSM_MMAP_MIN_ADDR))]
    {
        mmap_min_addr = dac_mmap_min_addr;
    }
}

/*
 * sysctl handler which just sets dac_mmap_min_addr = the new value and then
 * calls update_mmap_min_addr() so non MAP_FIXED hints get rounded properly
 */
#[no_mangle]
pub unsafe extern "C" fn mmap_min_addr_handler(
    table: *const ctl_table,
    write: c_int,
    buffer: *mut c_void,
    lenp: *mut SizeT,
    ppos: *mut LoffT,
) -> c_int {
    let ret: c_int;

    if write != 0 && !capable(CAP_SYS_RAWIO) {
        return -EPERM;
    }

    ret = proc_doulongvec_minmax(table, write, buffer, lenp, ppos);

    update_mmap_min_addr();

    ret
}

static mut min_addr_sysctl_table: [ctl_table; 1] = [ctl_table {
    procname: c"mmap_min_addr".as_ptr(),
    data: &raw mut dac_mmap_min_addr as *mut c_void,
    maxlen: core::mem::size_of::<u64>(),
    mode: 0o644,
    proc_handler: Some(mmap_min_addr_handler),
}];

unsafe fn mmap_min_addr_init() -> c_int {
    register_sysctl_init(c"vm".as_ptr(), &raw const min_addr_sysctl_table as *const ctl_table);
    update_mmap_min_addr();

    0
}

// C initcall registration: pure_initcall(mmap_min_addr_init);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
