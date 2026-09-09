// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2016 IBM Corporation.
 */

#[repr(C)]
pub struct opal {
    pub base: u64,
    pub entry: u64,
}

#[no_mangle]
pub static mut opal: opal = opal { base: 0, entry: 0 };

static mut opal_con_id: u32 = 0;

extern "C" {
    static mut platform_ops: platform_ops;

    fn opal_console_write(term_number: i64, length: *mut u64, buffer: *const u8) -> i64;
    fn opal_console_read(term_number: i64, length: *mut u64, buffer: *mut u8) -> i64;
    fn opal_console_write_buffer_space(term_number: u64, length: *mut u64) -> i64;
    fn opal_console_flush(term_number: u64) -> i64;
    fn opal_poll_events(outstanding_event_mask: *mut u64) -> i64;

    fn finddevice(name: *const u8) -> *mut core::ffi::c_void;
    fn getprop(
        node: *mut core::ffi::c_void,
        name: *const u8,
        value: *mut core::ffi::c_void,
        size: usize,
    ) -> i32;
    fn be64_to_cpu(value: u64) -> u64;
    fn be32_to_cpu(value: u32) -> u32;
    fn cpu_to_be64(value: u64) -> u64;
}

#[repr(C)]
pub struct platform_ops {
    pub kentry: Option<unsafe extern "C" fn(fdt_addr: usize, vmlinux_addr: *mut core::ffi::c_void)>,
}

#[repr(C)]
pub struct serial_console_data {
    pub open: Option<unsafe extern "C" fn() -> i32>,
    pub putc: Option<unsafe extern "C" fn(c: u8)>,
    pub close: Option<unsafe extern "C" fn()>,
}

extern "C" {
    fn opal_kentry(fdt_addr: usize, vmlinux_addr: *mut core::ffi::c_void);
}

unsafe extern "C" fn opal_con_open() -> i32 {
    /*
     * When OPAL loads the boot kernel it stashes the OPAL base and entry
     * address in r8 and r9 so the kernel can use the OPAL console
     * before unflattening the devicetree. While executing the wrapper will
     * probably trash r8 and r9 so this kentry hook restores them before
     * entering the decompressed kernel.
     */
    platform_ops.kentry = Some(opal_kentry);
    0
}

unsafe extern "C" fn opal_con_putc(c: u8) {
    let rc: i64;
    let mut olen: u64;
    let mut len: u64;

    loop {
        rc = opal_console_write_buffer_space(opal_con_id as u64, &mut olen);
        len = be64_to_cpu(olen);
        if rc != 0 {
            return;
        }
        opal_poll_events(core::ptr::null_mut());
        if len >= 1 {
            break;
        }
    }

    olen = cpu_to_be64(1);
    opal_console_write(opal_con_id as i64, &mut olen, &c);
}

unsafe extern "C" fn opal_con_close() {
    opal_console_flush(opal_con_id as u64);
}

unsafe fn opal_init() {
    let opal_node: *mut core::ffi::c_void;

    opal_node = finddevice(b"/ibm,opal\0".as_ptr());
    if opal_node.is_null() {
        return;
    }
    if getprop(
        opal_node,
        b"opal-base-address\0".as_ptr(),
        &mut opal.base as *mut u64 as *mut core::ffi::c_void,
        core::mem::size_of::<u64>(),
    ) < 0 {
        return;
    }
    opal.base = be64_to_cpu(opal.base);
    if getprop(
        opal_node,
        b"opal-entry-address\0".as_ptr(),
        &mut opal.entry as *mut u64 as *mut core::ffi::c_void,
        core::mem::size_of::<u64>(),
    ) < 0 {
        return;
    }
    opal.entry = be64_to_cpu(opal.entry);
}

#[no_mangle]
pub unsafe extern "C" fn opal_console_init(
    devp: *mut core::ffi::c_void,
    scdp: *mut serial_console_data,
) -> i32 {
    opal_init();

    if !devp.is_null() {
        let n = getprop(
            devp,
            b"reg\0".as_ptr(),
            &mut opal_con_id as *mut u32 as *mut core::ffi::c_void,
            core::mem::size_of::<u32>(),
        );
        if n != core::mem::size_of::<u32>() as i32 {
            return -1;
        }
        opal_con_id = be32_to_cpu(opal_con_id);
    } else {
        opal_con_id = 0;
    }

    (*scdp).open = Some(opal_con_open);
    (*scdp).putc = Some(opal_con_putc);
    (*scdp).close = Some(opal_con_close);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
