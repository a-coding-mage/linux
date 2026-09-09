// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PowerNV nvram code.
 *
 * Copyright 2011 IBM Corp.
 */

// DEBUG

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn opal_read_nvram(buf: u64, count: usize, offset: i32) -> i64;
    fn opal_write_nvram(buf: u64, count: usize, offset: i32) -> i64;
    fn opal_poll_events(arg: *mut core::ffi::c_void);
    fn in_interrupt() -> bool;
    fn irqs_disabled() -> bool;
    fn mdelay(ms: u64);
    fn msleep(ms: u64);
    fn nvram_scan_partitions();
    fn nvram_init_oops_partition(partition: i32);
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const core::ffi::c_char,
        compatible: *const core::ffi::c_char,
    ) -> *mut device_node;
    fn of_get_property(
        node: *mut device_node,
        name: *const core::ffi::c_char,
        length: *mut i32,
    ) -> *const u32;
    fn of_node_put(node: *mut device_node);
    fn pr_info(format: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    static mut ppc_md: MachineDesc;
}

#[repr(C)]
pub struct MachineDesc {
    pub nvram_read: Option<unsafe extern "C" fn(*mut i8, usize, *mut i64) -> isize>,
    pub nvram_write: Option<unsafe extern "C" fn(*mut i8, usize, *mut i64) -> isize>,
    pub nvram_size: Option<unsafe extern "C" fn() -> isize>,
}

const OPAL_SUCCESS: i64 = 0;
const OPAL_BUSY: i64 = -2;
const OPAL_BUSY_EVENT: i64 = -12;
const OPAL_BUSY_DELAY_MS: u64 = 10;
const EIO: isize = 5;

static mut NVRAM_SIZE: u32 = 0;

unsafe extern "C" fn opal_nvram_size() -> isize {
    NVRAM_SIZE as isize
}

unsafe extern "C" fn opal_nvram_read(buf: *mut i8, count: usize, index: *mut i64) -> isize {
    let rc: i64;
    let mut off: i32;

    if *index >= NVRAM_SIZE as i64 {
        return 0;
    }
    off = *index as i32;
    let mut count = count;
    if (off as usize + count) > NVRAM_SIZE as usize {
        count = NVRAM_SIZE as usize - off as usize;
    }
    rc = opal_read_nvram(buf as usize as u64, count, off);
    if rc != OPAL_SUCCESS {
        return -EIO;
    }
    *index += count as i64;
    count as isize
}

/*
 * This can be called in the panic path with interrupts off, so use
 * mdelay in that case.
 */
unsafe extern "C" fn opal_nvram_write(buf: *mut i8, count: usize, index: *mut i64) -> isize {
    let mut rc: i64 = OPAL_BUSY;
    let mut off: i32;

    if *index >= NVRAM_SIZE as i64 {
        return 0;
    }
    off = *index as i32;
    let mut count = count;
    if (off as usize + count) > NVRAM_SIZE as usize {
        count = NVRAM_SIZE as usize - off as usize;
    }

    while rc == OPAL_BUSY || rc == OPAL_BUSY_EVENT {
        rc = opal_write_nvram(buf as usize as u64, count, off);
        if rc == OPAL_BUSY_EVENT {
            if in_interrupt() || irqs_disabled() {
                mdelay(OPAL_BUSY_DELAY_MS);
            } else {
                msleep(OPAL_BUSY_DELAY_MS);
            }
            opal_poll_events(core::ptr::null_mut());
        } else if rc == OPAL_BUSY {
            if in_interrupt() || irqs_disabled() {
                mdelay(OPAL_BUSY_DELAY_MS);
            } else {
                msleep(OPAL_BUSY_DELAY_MS);
            }
        }
    }

    if rc != 0 {
        return -EIO;
    }

    *index += count as i64;
    count as isize
}

unsafe extern "C" fn opal_nvram_init_log_partitions() -> i32 {
    /* Scan nvram for partitions */
    nvram_scan_partitions();
    nvram_init_oops_partition(0);
    0
}

// machine_arch_initcall(powernv, opal_nvram_init_log_partitions);

pub unsafe extern "C" fn opal_nvram_init() {
    let np: *mut device_node;
    let nbytes_p: *const u32;

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"ibm,opal-nvram\0".as_ptr() as *const core::ffi::c_char,
    );
    if np.is_null() {
        return;
    }

    nbytes_p = of_get_property(
        np,
        b"#bytes\0".as_ptr() as *const core::ffi::c_char,
        core::ptr::null_mut(),
    );
    if nbytes_p.is_null() {
        of_node_put(np);
        return;
    }
    NVRAM_SIZE = u32::from_be(*nbytes_p);

    pr_info(b"OPAL nvram setup, %u bytes\n\0".as_ptr() as *const core::ffi::c_char, NVRAM_SIZE);
    of_node_put(np);

    ppc_md.nvram_read = Some(opal_nvram_read);
    ppc_md.nvram_write = Some(opal_nvram_write);
    ppc_md.nvram_size = Some(opal_nvram_size);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
