// SPDX-License-Identifier: GPL-2.0-only
/*
 * The simple platform -- for booting when firmware doesn't supply a device
 *                        tree or any platform configuration information.
 *                        All data is extracted from an embedded device tree
 *                        blob.
 *
 * Authors: Scott Wood <scottwood@freescale.com>
 *          Grant Likely <grant.likely@secretlab.ca>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 * Copyright (c) 2008 Secret Lab Technologies Ltd.
 */

// Declarations supplied by ops.h, types.h, io.h, stdio.h, and libfdt.h.
extern "C" {
    static _dtb_start: *const u8;
    static _end: u8;
    static mut timebase_period_ns: u32;

    fn fdt_check_header(fdt: *const u8) -> i32;
    fn fdt_path_offset(fdt: *const u8, path: *const i8) -> i32;
    fn fdt_getprop(fdt: *const u8, nodeoffset: i32, name: *const i8,
                   lenp: *mut i32) -> *const u32;
    fn fdt_node_offset_by_prop_value(fdt: *const u8, startoffset: i32,
                                     propname: *const i8, propval: *const i8,
                                     proplen: usize) -> i32;
    fn fatal(message: *const i8) -> !;
    fn simple_alloc_init(base: *mut u8, size: usize, align: usize, min_alloc: usize);
    fn fdt_init(fdt: *const u8);
    fn serial_console_init();

    // This is a weak C symbol; a zero address means no platform-specific init.
    fn platform_specific_init();
}

// BSS_STACK(4*1024);
const BSS_STACK_SIZE: usize = 4 * 1024;

#[no_mangle]
pub unsafe extern "C" fn platform_init(
    _r3: usize,
    _r4: usize,
    _r5: usize,
    _r6: usize,
    _r7: usize,
) {
    let mut na: *const u32;
    let mut ns: *const u32;
    let mut reg: *const u32;
    let mut timebase: *const u32;
    let mut memsize64: u64;
    let mut node: i32;
    let mut size: i32 = 0;
    let mut i: usize;

    /* Make sure FDT blob is sane */
    if fdt_check_header(_dtb_start) != 0 {
        fatal(b"Invalid device tree blob\n".as_ptr() as *const i8);
    }

    /* Find the #address-cells and #size-cells properties */
    node = fdt_path_offset(_dtb_start, b"/\0".as_ptr() as *const i8);
    if node < 0 {
        fatal(b"Cannot find root node\n".as_ptr() as *const i8);
    }
    na = fdt_getprop(_dtb_start, node, b"#address-cells\0".as_ptr() as *const i8, &mut size);
    if na.is_null() || size != 4 {
        fatal(b"Cannot find #address-cells property\0".as_ptr() as *const i8);
    }
    ns = fdt_getprop(_dtb_start, node, b"#size-cells\0".as_ptr() as *const i8, &mut size);
    if ns.is_null() || size != 4 {
        fatal(b"Cannot find #size-cells property\0".as_ptr() as *const i8);
    }

    /* Find the memory range */
    node = fdt_node_offset_by_prop_value(_dtb_start, -1, b"device_type\0".as_ptr() as *const i8,
                                         b"memory\0".as_ptr() as *const i8, 7);
    if node < 0 {
        fatal(b"Cannot find memory node\n".as_ptr() as *const i8);
    }
    reg = fdt_getprop(_dtb_start, node, b"reg\0".as_ptr() as *const i8, &mut size);
    if size < ((*na + *ns) * core::mem::size_of::<u32>()) as i32 {
        fatal(b"cannot get memory range\n".as_ptr() as *const i8);
    }

    /* Only interested in memory based at 0 */
    i = 0;
    while i < *na as usize {
        if *reg != 0 {
            fatal(b"Memory range is not based at address 0\n".as_ptr() as *const i8);
        }
        reg = reg.add(1);
        i += 1;
    }

    /* get the memsize and truncate it to under 4G on 32 bit machines */
    memsize64 = 0;
    i = 0;
    while i < *ns as usize {
        memsize64 = (memsize64 << 32) | *reg as u64;
        reg = reg.add(1);
        i += 1;
    }
    if core::mem::size_of::<*const u8>() == 4 && memsize64 >= 0x1_0000_0000_u64 {
        memsize64 = 0xffff_ffff;
    }

    /* finally, setup the timebase */
    node = fdt_node_offset_by_prop_value(_dtb_start, -1, b"device_type\0".as_ptr() as *const i8,
                                         b"cpu\0".as_ptr() as *const i8, 4);
    if node < 0 {
        fatal(b"Cannot find cpu node\n".as_ptr() as *const i8);
    }
    timebase = fdt_getprop(_dtb_start, node, b"timebase-frequency\0".as_ptr() as *const i8, &mut size);
    if !timebase.is_null() && size == 4 {
        timebase_period_ns = 1_000_000_000 / *timebase;
    }

    /* Now we have the memory size; initialize the heap */
    simple_alloc_init((&_end as *const u8) as *mut u8,
                      memsize64 - (&_end as *const u8 as usize) as u64 as usize,
                      32, 64);

    /* prepare the device tree and find the console */
    fdt_init(_dtb_start);

    if platform_specific_init as usize != 0 {
        platform_specific_init();
    }

    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
