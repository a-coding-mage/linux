// SPDX-License-Identifier: GPL-2.0
/*
 * AMD Versal True Random Number Generator driver
 * Copyright (c) 2024 - 2025 Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel bindings:
// crypto/sha2.h, linux/bitfield.h, linux/clk.h, linux/delay.h,
// linux/firmware/xlnx-zynqmp.h, linux/hw_random.h, linux/io.h,
// linux/iopoll.h, linux/kernel.h, linux/module.h, linux/platform_device.h

const TRNG_STATUS_OFFSET: usize = 0x4;
const TRNG_CTRL_OFFSET: usize = 0x8;
const TRNG_EXT_SEED_OFFSET: usize = 0x40;
const TRNG_PER_STRNG_OFFSET: usize = 0x80;
const TRNG_CORE_OUTPUT_OFFSET: usize = 0xC0;
const TRNG_RESET_OFFSET: usize = 0xD0;
const TRNG_OSC_EN_OFFSET: usize = 0xD4;

const TRNG_RESET_VAL_MASK: u32 = 1 << 0;
const TRNG_OSC_EN_VAL_MASK: u32 = 1 << 0;
const TRNG_CTRL_PRNGSRST_MASK: u32 = 1 << 0;
const TRNG_CTRL_EUMODE_MASK: u32 = 1 << 8;
const TRNG_CTRL_TRSSEN_MASK: u32 = 1 << 2;
const TRNG_CTRL_PRNGSTART_MASK: u32 = 1 << 5;
const TRNG_CTRL_PRNGXS_MASK: u32 = 1 << 3;
const TRNG_CTRL_PRNGMODE_MASK: u32 = 1 << 7;
const TRNG_STATUS_DONE_MASK: u32 = 1 << 0;
const TRNG_STATUS_QCNT_MASK: u32 = ((1 << 3) - 1) << 9;
const TRNG_STATUS_QCNT_16_BYTES: u32 = 0x800;

const TRNG_SEED_LEN_BYTES: usize = 48;
const TRNG_SEC_STRENGTH_SHIFT: usize = 5;
const TRNG_SEC_STRENGTH_BYTES: usize = 1 << TRNG_SEC_STRENGTH_SHIFT;
const TRNG_BYTES_PER_REG: usize = 4;
const TRNG_RESET_DELAY: u32 = 10;
const TRNG_NUM_INIT_REGS: usize = 12;
const TRNG_READ_4_WORD: usize = 4;
const TRNG_DATA_READ_DELAY: u32 = 8000;

#[repr(C)]
pub struct xilinx_rng {
    pub rng_base: *mut core::ffi::c_void,
    pub dev: *mut device,
    pub trng: hwrng,
}

unsafe extern "C" {
    type device;
    type hwrng;
    type platform_device;

    fn ioread32(addr: *mut core::ffi::c_void) -> u32;
    fn iowrite32(value: u32, addr: *mut core::ffi::c_void);
    fn udelay(usecs: u32);
    fn readl_poll_timeout(addr: *mut core::ffi::c_void, val: *mut u32, condition: bool,
                           delay_us: u32, timeout_us: u32) -> i32;
    fn hmac_sha512_usingrawkey(key: *const u8, key_len: usize, data: *const u8,
                               data_len: usize, out: *mut u8);
    fn memzero_explicit(ptr: *mut u8, len: usize);
    fn hwrng_register(rng: *mut hwrng) -> i32;
    fn hwrng_unregister(rng: *mut hwrng);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32)
        -> *mut core::ffi::c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut core::ffi::c_void;
}

unsafe fn xtrng_readwrite32(addr: *mut core::ffi::c_void, mask: u32, value: u8) {
    let val = ioread32(addr);
    iowrite32((val & !mask) | (mask & value as u32), addr);
}

unsafe fn xtrng_trng_reset(addr: *mut core::ffi::c_void) {
    xtrng_readwrite32(addr.add(TRNG_RESET_OFFSET), TRNG_RESET_VAL_MASK, TRNG_RESET_VAL_MASK as u8);
    udelay(TRNG_RESET_DELAY);
    xtrng_readwrite32(addr.add(TRNG_RESET_OFFSET), TRNG_RESET_VAL_MASK, 0);
}

unsafe fn xtrng_hold_reset(addr: *mut core::ffi::c_void) {
    xtrng_readwrite32(addr.add(TRNG_CTRL_OFFSET), TRNG_CTRL_PRNGSRST_MASK,
                      TRNG_CTRL_PRNGSRST_MASK as u8);
    iowrite32(TRNG_RESET_VAL_MASK, addr.add(TRNG_RESET_OFFSET));
    udelay(TRNG_RESET_DELAY);
}

unsafe fn xtrng_softreset(rng: *mut xilinx_rng) {
    xtrng_readwrite32((*rng).rng_base.add(TRNG_CTRL_OFFSET), TRNG_CTRL_PRNGSRST_MASK,
                      TRNG_CTRL_PRNGSRST_MASK as u8);
    udelay(TRNG_RESET_DELAY);
    xtrng_readwrite32((*rng).rng_base.add(TRNG_CTRL_OFFSET), TRNG_CTRL_PRNGSRST_MASK, 0);
}

unsafe fn xtrng_readblock32(rng_base: *mut core::ffi::c_void, buf: *mut u32,
                            blocks32: i32, wait: bool) -> i32 {
    let mut read = 0i32;
    let mut timeout = 1u32;
    let mut val = 0u32;
    if wait { timeout = TRNG_DATA_READ_DELAY; }
    for _ in 0..(blocks32 * 2) {
        let ret = readl_poll_timeout(rng_base.add(TRNG_STATUS_OFFSET), &mut val,
            (val & TRNG_STATUS_QCNT_MASK) == TRNG_STATUS_QCNT_16_BYTES,
            if wait { 1 } else { 0 }, timeout);
        if ret != 0 {
            if read == 0 { return ret; }
            break;
        }
        for idx in 0..TRNG_READ_4_WORD {
            *buf.add(read as usize + idx) = ioread32(rng_base.add(TRNG_CORE_OUTPUT_OFFSET)).to_be();
        }
        read += TRNG_READ_4_WORD as i32;
    }
    read * 4
}

unsafe fn xtrng_collect_random_data(rng: *mut xilinx_rng, rand_gen_buf: *mut u8,
                                    no_of_random_bytes: i32, wait: bool) -> i32 {
    let mut randbuf = [0u8; TRNG_SEC_STRENGTH_BYTES];
    let byteleft = no_of_random_bytes & (TRNG_SEC_STRENGTH_BYTES as i32 - 1);
    let blocks = no_of_random_bytes >> TRNG_SEC_STRENGTH_SHIFT;
    let full_blocks_bytes = blocks * TRNG_SEC_STRENGTH_BYTES as i32;
    let mut count = 0;
    xtrng_readwrite32((*rng).rng_base.add(TRNG_CTRL_OFFSET), TRNG_CTRL_PRNGSTART_MASK,
                      TRNG_CTRL_PRNGSTART_MASK as u8);
    if blocks != 0 {
        let ret = xtrng_readblock32((*rng).rng_base, rand_gen_buf as *mut u32, blocks, wait);
        if ret <= 0 { count = ret; return xtrng_collect_stop(rng, count); }
        count += ret;
        if ret < full_blocks_bytes { return xtrng_collect_stop(rng, count); }
    }
    if byteleft != 0 {
        let ret = xtrng_readblock32((*rng).rng_base, randbuf.as_mut_ptr() as *mut u32, 1, wait);
        if ret < 0 { if count == 0 { count = ret; } return xtrng_collect_stop(rng, count); }
        if ret == 0 { return xtrng_collect_stop(rng, count); }
        let copy = core::cmp::min(ret, no_of_random_bytes - count) as usize;
        core::ptr::copy_nonoverlapping(randbuf.as_ptr(), rand_gen_buf.add(count as usize), copy);
        count += copy as i32;
    }
    xtrng_collect_stop(rng, count)
}

unsafe fn xtrng_collect_stop(rng: *mut xilinx_rng, count: i32) -> i32 {
    xtrng_readwrite32((*rng).rng_base.add(TRNG_CTRL_OFFSET),
                      TRNG_CTRL_PRNGMODE_MASK | TRNG_CTRL_PRNGSTART_MASK, 0);
    count
}

unsafe fn xtrng_write_multiple_registers(base_addr: *mut core::ffi::c_void,
                                         values: *const u32, n: usize) {
    for i in 0..n {
        iowrite32((*values.add(i)).to_be(), base_addr.add((n - 1 - i) * TRNG_BYTES_PER_REG));
    }
}

unsafe fn xtrng_enable_entropy(rng: *mut xilinx_rng) {
    iowrite32(TRNG_OSC_EN_VAL_MASK, (*rng).rng_base.add(TRNG_OSC_EN_OFFSET));
    xtrng_softreset(rng);
    iowrite32(TRNG_CTRL_EUMODE_MASK | TRNG_CTRL_TRSSEN_MASK, (*rng).rng_base.add(TRNG_CTRL_OFFSET));
}

unsafe fn xtrng_reseed_internal(rng: *mut xilinx_rng) -> i32 {
    let default_salt = [0u8; 64];
    let mut entropy = [0u8; 64];
    let mut val = 0u32;
    xtrng_enable_entropy(rng);
    let ret = xtrng_collect_random_data(rng, entropy.as_mut_ptr(), TRNG_SEED_LEN_BYTES as i32, true);
    if ret != TRNG_SEED_LEN_BYTES as i32 { return -22; }
    hmac_sha512_usingrawkey(default_salt.as_ptr(), default_salt.len(), entropy.as_ptr(),
                            TRNG_SEED_LEN_BYTES, entropy.as_mut_ptr());
    xtrng_write_multiple_registers((*rng).rng_base.add(TRNG_EXT_SEED_OFFSET),
                                   entropy.as_ptr() as *const u32, TRNG_NUM_INIT_REGS);
    memzero_explicit(entropy.as_mut_ptr(), entropy.len());
    iowrite32(TRNG_CTRL_PRNGXS_MASK, (*rng).rng_base.add(TRNG_CTRL_OFFSET));
    xtrng_readwrite32((*rng).rng_base.add(TRNG_CTRL_OFFSET), TRNG_CTRL_PRNGSTART_MASK,
                      TRNG_CTRL_PRNGSTART_MASK as u8);
    let ret = readl_poll_timeout((*rng).rng_base.add(TRNG_STATUS_OFFSET), &mut val,
        (val & TRNG_STATUS_DONE_MASK) == TRNG_STATUS_DONE_MASK, 1, 15000);
    if ret != 0 { return ret; }
    xtrng_readwrite32((*rng).rng_base.add(TRNG_CTRL_OFFSET), TRNG_CTRL_PRNGSTART_MASK, 0);
    0
}

unsafe fn xtrng_random_bytes_generate(rng: *mut xilinx_rng, rand_buf_ptr: *mut u8,
                                      rand_buf_size: u32, wait: i32) -> i32 {
    xtrng_readwrite32((*rng).rng_base.add(TRNG_CTRL_OFFSET),
        TRNG_CTRL_PRNGMODE_MASK | TRNG_CTRL_PRNGXS_MASK,
        (TRNG_CTRL_PRNGMODE_MASK | TRNG_CTRL_PRNGXS_MASK) as u8);
    let nbytes = xtrng_collect_random_data(rng, rand_buf_ptr, rand_buf_size as i32, wait != 0);
    let ret = xtrng_reseed_internal(rng);
    if ret != 0 { return ret; }
    nbytes
}

unsafe fn xtrng_hwrng_trng_read(hwrng_ptr: *mut hwrng, data: *mut u8,
                                max: usize, wait: bool) -> i32 {
    let rng = ((hwrng_ptr as *mut u8).sub(core::mem::offset_of!(xilinx_rng, trng)))
        as *mut xilinx_rng;
    let mut buf = [0u8; TRNG_SEC_STRENGTH_BYTES];
    let mut i = 0usize;
    while i < max {
        let ret = xtrng_random_bytes_generate(rng, buf.as_mut_ptr(),
                                              TRNG_SEC_STRENGTH_BYTES as u32, wait as i32);
        if ret < 0 {
            if i == 0 { return ret; }
            break;
        }
        let copy = core::cmp::min(ret as usize, max - i);
        core::ptr::copy_nonoverlapping(buf.as_ptr(), data.add(i), copy);
        i += copy;
    }
    i as i32
}

unsafe fn xtrng_hwrng_register(trng: *mut hwrng) -> i32 {
    // trng->name = "Xilinx Versal Crypto Engine TRNG";
    // trng->read = xtrng_hwrng_trng_read;
    let ret = hwrng_register(trng);
    if ret != 0 {
        // pr_err("Fail to register the TRNG\n");
    }
    ret
}

unsafe fn xtrng_hwrng_unregister(trng: *mut hwrng) {
    hwrng_unregister(trng);
}

unsafe fn xtrng_probe(pdev: *mut platform_device) -> i32 {
    let rng = devm_kzalloc(pdev as *mut device, core::mem::size_of::<xilinx_rng>(), 0)
        as *mut xilinx_rng;
    if rng.is_null() { return -12; }
    (*rng).dev = pdev as *mut device;
    (*rng).rng_base = devm_platform_ioremap_resource(pdev, 0);
    if (*rng).rng_base as isize == -1 {
        // dev_err(&pdev->dev, "Failed to map resource %pe\n", rng->rng_base);
        return -1;
    }
    xtrng_trng_reset((*rng).rng_base);
    let ret = xtrng_reseed_internal(rng);
    if ret != 0 { return ret; }
    let ret = xtrng_hwrng_register(&mut (*rng).trng);
    if ret != 0 { return ret; }
    platform_set_drvdata(pdev, rng as *mut core::ffi::c_void);
    0
}

unsafe fn xtrng_remove(pdev: *mut platform_device) {
    let rng = platform_get_drvdata(pdev) as *mut xilinx_rng;
    let zero = [0u32; TRNG_NUM_INIT_REGS];
    xtrng_hwrng_unregister(&mut (*rng).trng);
    xtrng_write_multiple_registers((*rng).rng_base.add(TRNG_EXT_SEED_OFFSET),
                                   zero.as_ptr(), TRNG_NUM_INIT_REGS);
    xtrng_write_multiple_registers((*rng).rng_base.add(TRNG_PER_STRNG_OFFSET),
                                   zero.as_ptr(), TRNG_NUM_INIT_REGS);
    xtrng_hold_reset((*rng).rng_base);
}

#[repr(C)]
struct of_device_id {
    compatible: *const core::ffi::c_char,
}

#[repr(C)]
struct platform_driver {
    driver: driver,
    probe: Option<unsafe fn(*mut platform_device) -> i32>,
    remove: Option<unsafe fn(*mut platform_device)>,
}

#[repr(C)]
struct driver {
    name: *const core::ffi::c_char,
    of_match_table: *const of_device_id,
}

static XTRNG_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"xlnx,versal-trng\0".as_ptr() as *const _ },
    of_device_id { compatible: core::ptr::null() },
];

static mut XTRNG_DRIVER: platform_driver = platform_driver {
    driver: driver {
        name: b"xlnx,versal-trng\0".as_ptr() as *const _,
        of_match_table: XTRNG_OF_MATCH.as_ptr(),
    },
    probe: Some(xtrng_probe),
    remove: Some(xtrng_remove),
};

// MODULE_DEVICE_TABLE(of, xtrng_of_match);
// module_platform_driver(xtrng_driver);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Harsh Jain <h.jain@amd.com>");
// MODULE_AUTHOR("Mounika Botcha <mounika.botcha@amd.com>");
// MODULE_DESCRIPTION("True Random Number Generator Driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
