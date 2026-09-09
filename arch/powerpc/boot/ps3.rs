// SPDX-License-Identifier: GPL-2.0-only
/*
 *  PS3 bootwrapper support.
 *
 *  Copyright (C) 2007 Sony Computer Entertainment Inc.
 *  Copyright 2007 Sony Corp.
 */

// Dependencies supplied by the surrounding bootwrapper sources:
// types.h, elf.h, string.h, stdio.h, page.h, and ops.h.

extern "C" {
    fn lv1_panic(in_1: u64) -> i32;
    fn lv1_get_logical_partition_id(out_1: *mut u64) -> i32;
    fn lv1_get_logical_ppe_id(out_1: *mut u64) -> i32;
    fn lv1_get_repository_node_value(
        in_1: u64,
        in_2: u64,
        in_3: u64,
        in_4: u64,
        in_5: u64,
        out_1: *mut u64,
        out_2: *mut u64,
    ) -> i32;

    static mut _end: u8;
    static mut _dtb_start: u8;
    static mut _initrd_start: u8;
    static mut _initrd_end: u8;
    static mut __system_reset_kernel: u8;
}

// BSS_STACK(4096);
#[no_mangle]
static mut BSS_STACK: [u8; 4096] = [0; 4096];

/* A buffer that may be edited by tools operating on a zImage binary so as to
 * edit the command line passed to vmlinux (by setting /chosen/bootargs).
 * The buffer is put in its own section so that tools may locate it easier.
 */

// BOOT_COMMAND_LINE_SIZE is supplied by the bootwrapper environment.
#[link_section = "__builtin_cmdline"]
static mut cmdline: [u8; BOOT_COMMAND_LINE_SIZE] = [0; BOOT_COMMAND_LINE_SIZE];

unsafe fn prep_cmdline(chosen: *mut core::ffi::c_void) {
    if cmdline[0] == b'\0' {
        getprop(
            chosen,
            b"bootargs\0".as_ptr() as *const i8,
            cmdline.as_mut_ptr() as *mut i8,
            BOOT_COMMAND_LINE_SIZE - 1,
        );
    } else {
        setprop_str(
            chosen,
            b"bootargs\0".as_ptr() as *const i8,
            cmdline.as_mut_ptr() as *const i8,
        );
    }

    printf(b"cmdline: '%s'\n\0".as_ptr() as *const i8, cmdline.as_ptr());
}

unsafe extern "C" fn ps3_console_write(_buf: *const i8, _len: i32) {}

unsafe extern "C" fn ps3_exit() {
    printf(b"ps3_exit\n\0".as_ptr() as *const i8);

    /* lv1_panic will shutdown the lpar. */

    lv1_panic(0); /* zero = do not reboot */
    loop {}
}

unsafe fn ps3_repository_read_rm_size(rm_size: *mut u64) -> i32 {
    let mut result: i32;
    let mut lpar_id: u64 = 0;
    let mut ppe_id: u64 = 0;
    let mut v2: u64 = 0;

    result = lv1_get_logical_partition_id(&mut lpar_id);
    if result != 0 {
        return -1;
    }

    result = lv1_get_logical_ppe_id(&mut ppe_id);
    if result != 0 {
        return -1;
    }

    /*
     * n1: 0000000062690000 : ....bi..
     * n2: 7075000000000000 : pu......
     * n3: 0000000000000001 : ........
     * n4: 726d5f73697a6500 : rm_size.
     */

    result = lv1_get_repository_node_value(
        lpar_id,
        0x0000000062690000,
        0x7075000000000000,
        ppe_id,
        0x726d5f73697a6500,
        rm_size,
        &mut v2,
    );

    printf(
        b"%s:%d: ppe_id  %lu \n\0".as_ptr() as *const i8,
        b"ps3_repository_read_rm_size\0".as_ptr(),
        line!() as i32,
        ppe_id as libc::c_ulong,
    );
    printf(
        b"%s:%d: lpar_id %lu \n\0".as_ptr() as *const i8,
        b"ps3_repository_read_rm_size\0".as_ptr(),
        line!() as i32,
        lpar_id as libc::c_ulong,
    );
    printf(
        b"%s:%d: rm_size %llxh \n\0".as_ptr() as *const i8,
        b"ps3_repository_read_rm_size\0".as_ptr(),
        line!() as i32,
        *rm_size,
    );

    if result != 0 { -1 } else { 0 }
}

pub unsafe extern "C" fn ps3_copy_vectors() {
    memcpy(
        0x100 as *mut core::ffi::c_void,
        &__system_reset_kernel as *const u8 as *const core::ffi::c_void,
        512,
    );
    flush_cache(0x100 as *mut core::ffi::c_void, 512);
}

pub unsafe extern "C" fn platform_init() {
    let heapsize: u32 = 0x1000000u32.wrapping_sub(&_end as *const u8 as u32);
    let mut chosen: *mut core::ffi::c_void;
    let mut ft_addr: libc::c_ulong;
    let mut rm_size: u64 = 0;

    console_ops.write = Some(ps3_console_write);
    platform_ops.exit = Some(ps3_exit);

    printf(b"\n-- PS3 bootwrapper --\n\0".as_ptr() as *const i8);

    simple_alloc_init(&_end as *mut u8, heapsize, 32, 64);
    fdt_init(&_dtb_start as *mut u8);

    chosen = finddevice(b"/chosen\0".as_ptr() as *const i8);

    ps3_repository_read_rm_size(&mut rm_size);
    dt_fixup_memory(0, rm_size);

    if (&_initrd_end as *const u8 as usize) > (&_initrd_start as *const u8 as usize) {
        setprop_val(chosen, b"linux,initrd-start\0".as_ptr() as *const i8, &_initrd_start as *const u8 as u32);
        setprop_val(chosen, b"linux,initrd-end\0".as_ptr() as *const i8, &_initrd_end as *const u8 as u32);
    }

    prep_cmdline(chosen);
    ft_addr = dt_ops.finalize();
    ps3_copy_vectors();
    printf(b" flat tree at 0x%lx\n\r\0".as_ptr() as *const i8, ft_addr);
    (core::mem::transmute::<usize, kernel_entry_t>(0))(ft_addr, 0, core::ptr::null_mut());
    ps3_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
