// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 Intel Corporation
 */

// Kernel dependencies supplied by the surrounding build.

/* a tiny module only meant to test
 *
 *   set/clear_bit
 *   get_count_order/long
 */

/* use an enum because that's the most common BITMAP usage */
#[repr(i32)]
enum BitopsFun {
    BITOPS_4 = 4,
    BITOPS_7 = 7,
    BITOPS_11 = 11,
    BITOPS_31 = 31,
    BITOPS_88 = 88,
    BITOPS_LAST = 255,
    BITOPS_LENGTH = 256,
}

static mut g_bitmap: [usize; 4] = [0; 4];

static mut order_comb: [[u32; 2]; 7] = [
    [0x00000003, 2],
    [0x00000004, 2],
    [0x00001fff, 13],
    [0x00002000, 13],
    [0x50000000, 31],
    [0x80000000, 31],
    [0x80003000, 32],
];

#[cfg(target_pointer_width = "64")]
static mut order_comb_long: [[usize; 2]; 7] = [
    [0x0000000300000000, 34],
    [0x0000000400000000, 34],
    [0x00001fff00000000, 45],
    [0x0000200000000000, 45],
    [0x5000000000000000, 63],
    [0x8000000000000000, 63],
    [0x8000300000000000, 64],
];

extern "C" {
    fn kmalloc_array(n: usize, size: usize, flags: u32) -> *mut usize;
    fn kfree(ptr: *mut usize);
    fn get_random_bytes(buf: *mut core::ffi::c_void, len: usize);
    fn ktime_get() -> i64;
    fn fns(value: usize, bit: u32) -> usize;
    fn set_bit(bit: usize, addr: *mut usize);
    fn clear_bit(bit: usize, addr: *mut usize);
    fn get_count_order(value: u32) -> u32;
    fn get_count_order_long(value: usize) -> u32;
    fn find_first_bit(addr: *const usize, size: usize) -> usize;
    fn barrier();
    fn pr_info(fmt: *const u8, ...);
    fn pr_warn(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
}

unsafe fn test_fns() -> i32 {
    static mut tmp: usize = 0;
    let mut buf: *mut usize = core::ptr::null_mut();
    let mut time: i64;

    buf = kmalloc_array(10000, core::mem::size_of::<usize>(), 0);
    if buf.is_null() {
        return -12;
    }

    get_random_bytes(
        buf.cast::<core::ffi::c_void>(),
        10000 * core::mem::size_of::<usize>(),
    );
    time = ktime_get();

    for n in 0..(usize::BITS as usize) {
        for i in 0..10000 {
            tmp = fns(*buf.add(i), n as u32);
        }
    }

    time = ktime_get() - time;
    pr_err(b"fns:  %18llu ns\0".as_ptr(), time as u64);
    kfree(buf);
    0
}

unsafe fn test_bitops_startup() -> i32 {
    let mut bit_set: i32;

    pr_info(b"Starting bitops test\n\0".as_ptr());
    set_bit(BitopsFun::BITOPS_4 as usize, g_bitmap.as_mut_ptr());
    set_bit(BitopsFun::BITOPS_7 as usize, g_bitmap.as_mut_ptr());
    set_bit(BitopsFun::BITOPS_11 as usize, g_bitmap.as_mut_ptr());
    set_bit(BitopsFun::BITOPS_31 as usize, g_bitmap.as_mut_ptr());
    set_bit(BitopsFun::BITOPS_88 as usize, g_bitmap.as_mut_ptr());

    for i in 0..order_comb.len() {
        if order_comb[i][1] != get_count_order(order_comb[i][0]) {
            pr_warn(b"get_count_order wrong for %x\n\0".as_ptr(), order_comb[i][0]);
        }
    }

    for i in 0..order_comb.len() {
        if order_comb[i][1] != get_count_order_long(order_comb[i][0] as usize) {
            pr_warn(b"get_count_order_long wrong for %x\n\0".as_ptr(), order_comb[i][0]);
        }
    }

    #[cfg(target_pointer_width = "64")]
    for i in 0..order_comb_long.len() {
        if order_comb_long[i][1] as u32 != get_count_order_long(order_comb_long[i][0]) {
            pr_warn(b"get_count_order_long wrong for %lx\n\0".as_ptr(), order_comb_long[i][0]);
        }
    }

    barrier();

    clear_bit(BitopsFun::BITOPS_4 as usize, g_bitmap.as_mut_ptr());
    clear_bit(BitopsFun::BITOPS_7 as usize, g_bitmap.as_mut_ptr());
    clear_bit(BitopsFun::BITOPS_11 as usize, g_bitmap.as_mut_ptr());
    clear_bit(BitopsFun::BITOPS_31 as usize, g_bitmap.as_mut_ptr());
    clear_bit(BitopsFun::BITOPS_88 as usize, g_bitmap.as_mut_ptr());

    bit_set = find_first_bit(g_bitmap.as_ptr(), BitopsFun::BITOPS_LAST as usize) as i32;
    if bit_set != BitopsFun::BITOPS_LAST as i32 {
        pr_err(b"ERROR: FOUND SET BIT %d\n\0".as_ptr(), bit_set);
    }

    test_fns();
    pr_info(b"Completed bitops test\n\0".as_ptr());
    0
}

unsafe fn test_bitops_unstartup() {}

// module_init(test_bitops_startup);
// module_exit(test_bitops_unstartup);
// MODULE_AUTHOR("Jesse Brandeburg <jesse.brandeburg@intel.com>, Wei Yang <richard.weiyang@gmail.com>");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Bit testing module");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
