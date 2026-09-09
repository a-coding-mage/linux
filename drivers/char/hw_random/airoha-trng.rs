// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2024 Christian Marangi */

// Linux kernel dependencies are supplied by the surrounding Rust kernel
// environment.

const TRNG_IP_RDY: usize = 0x800;
const CNT_TRANS: u32 = 0xff00;
const SAMPLE_RDY: u32 = 1 << 0;
const TRNG_NS_SEK_AND_DAT_EN: usize = 0x804;
const RNG_EN: u32 = 1 << 31;
const RAW_DATA_EN: u32 = 1 << 16;
const TRNG_HEALTH_TEST_SW_RST: usize = 0x808;
const SW_RST: u32 = 1 << 0;
const TRNG_INTR_EN: usize = 0x818;
const INTR_MASK: u32 = 1 << 16;
const CONTINUOUS_HEALTH_INITR_EN: u32 = 1 << 2;
const SW_STARTUP_INITR_EN: u32 = 1 << 1;
const RST_STARTUP_INITR_EN: u32 = 1 << 0;
// Notice that Health Test are done only out of Reset and with RNG_EN
const TRNG_HEALTH_TEST_STATUS: usize = 0x824;
const CONTINUOUS_HEALTH_AP_TEST_FAIL: u32 = 1 << 23;
const CONTINUOUS_HEALTH_RC_TEST_FAIL: u32 = 1 << 22;
const SW_STARTUP_TEST_DONE: u32 = 1 << 21;
const SW_STARTUP_AP_TEST_FAIL: u32 = 1 << 20;
const SW_STARTUP_RC_TEST_FAIL: u32 = 1 << 19;
const RST_STARTUP_TEST_DONE: u32 = 1 << 18;
const RST_STARTUP_AP_TEST_FAIL: u32 = 1 << 17;
const RST_STARTUP_RC_TEST_FAIL: u32 = 1 << 16;
const RAW_DATA_VALID: u32 = 1 << 7;
const TRNG_RAW_DATA_OUT: usize = 0x828;
const TRNG_CNT_TRANS_VALID: u32 = 0x80;
const BUSY_LOOP_SLEEP: u64 = 10;
const BUSY_LOOP_TIMEOUT: u64 = BUSY_LOOP_SLEEP * 10000;

#[repr(C)]
struct AirohaTrng {
    base: *mut core::ffi::c_void,
    rng: Hwrng,
    dev: *mut Device,
    rng_op_done: Completion,
}

#[repr(C)]
struct Hwrng {
    name: *const core::ffi::c_char,
    init: Option<unsafe extern "C" fn(*mut Hwrng) -> i32>,
    cleanup: Option<unsafe extern "C" fn(*mut Hwrng)>,
    read: Option<unsafe extern "C" fn(*mut Hwrng, *mut core::ffi::c_void, usize, bool) -> isize>,
    quality: u32,
}

#[repr(C)] struct Device { _private: [u8; 0] }
#[repr(C)] struct Completion { _private: [u8; 0] }
#[repr(C)] struct PlatformDevice { dev: Device, name: *const core::ffi::c_char }
#[repr(C)] struct OfDeviceId { compatible: *const core::ffi::c_char }
#[repr(C)] struct PlatformDriver { _private: [u8; 0] }

extern "C" {
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn wait_for_completion_timeout(done: *mut Completion, timeout: u64) -> i32;
    fn complete(done: *mut Completion);
    fn init_completion(done: *mut Completion);
    fn readl_poll_timeout(addr: *mut u8, value: *mut u32, condition: bool, delay: u32, timeout: u32) -> i32;
    fn dev_err(dev: *mut Device, fmt: *const core::ffi::c_char, ...);
    fn container_of<T, U>(ptr: *mut T, member: *const U) -> *mut AirohaTrng;
    fn platform_get_irq(pdev: *mut PlatformDevice, index: u32) -> i32;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut AirohaTrng;
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: u32) -> *mut core::ffi::c_void;
    fn devm_request_irq(dev: *mut Device, irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32, flags: u32, name: *const core::ffi::c_char, data: *mut core::ffi::c_void) -> i32;
    fn devm_hwrng_register(dev: *mut Device, rng: *mut Hwrng) -> i32;
}

unsafe fn airoha_trng_irq_mask(trng: *mut AirohaTrng) -> i32 {
    let addr = (*trng).base.cast::<u8>().add(TRNG_INTR_EN);
    let val = readl(addr) | INTR_MASK;
    writel(val, addr);
    0
}

unsafe fn airoha_trng_irq_unmask(trng: *mut AirohaTrng) -> i32 {
    let addr = (*trng).base.cast::<u8>().add(TRNG_INTR_EN);
    let val = readl(addr) & !INTR_MASK;
    writel(val, addr);
    0
}

unsafe extern "C" fn airoha_trng_init(rng: *mut Hwrng) -> i32 {
    let trng = container_of(rng, core::ptr::null());
    let addr = (*trng).base.cast::<u8>().add(TRNG_NS_SEK_AND_DAT_EN);
    writel(readl(addr) | RNG_EN, addr);
    airoha_trng_irq_unmask(trng);
    writel(0, (*trng).base.cast::<u8>().add(TRNG_HEALTH_TEST_SW_RST));
    let ret = wait_for_completion_timeout(&mut (*trng).rng_op_done, BUSY_LOOP_TIMEOUT);
    if ret <= 0 { dev_err((*trng).dev, b"Timeout waiting for Health Check\0".as_ptr() as _,); airoha_trng_irq_mask(trng); return -19; }
    let val = readl((*trng).base.cast::<u8>().add(TRNG_HEALTH_TEST_STATUS));
    if val & (RST_STARTUP_AP_TEST_FAIL | RST_STARTUP_RC_TEST_FAIL) != 0 { dev_err((*trng).dev, b"Health Check fail: %s test fail\n\0".as_ptr() as _,); return -19; }
    let mut status = 0;
    let ret = readl_poll_timeout((*trng).base.cast::<u8>().add(TRNG_IP_RDY), &mut status, status & SAMPLE_RDY != 0, 10, 1000);
    if ret < 0 { dev_err((*trng).dev, b"Timeout waiting for IP ready\0".as_ptr() as _); return -19; }
    let ret = readl_poll_timeout((*trng).base.cast::<u8>().add(TRNG_IP_RDY), &mut status, ((status & CNT_TRANS) >> 8) == TRNG_CNT_TRANS_VALID, 10, 1000);
    if ret < 0 { dev_err((*trng).dev, b"Timeout waiting for IP ready\0".as_ptr() as _); return -19; }
    0
}

unsafe extern "C" fn airoha_trng_cleanup(rng: *mut Hwrng) {
    let trng = container_of(rng, core::ptr::null());
    let addr = (*trng).base.cast::<u8>().add(TRNG_NS_SEK_AND_DAT_EN);
    writel(readl(addr) & !RNG_EN, addr);
    writel(SW_RST, (*trng).base.cast::<u8>().add(TRNG_HEALTH_TEST_SW_RST));
}

unsafe extern "C" fn airoha_trng_read(rng: *mut Hwrng, buf: *mut core::ffi::c_void, _max: usize, _wait: bool) -> isize {
    let trng = container_of(rng, core::ptr::null());
    let mut status = 0;
    let ret = readl_poll_timeout((*trng).base.cast::<u8>().add(TRNG_HEALTH_TEST_STATUS), &mut status, status & RAW_DATA_VALID != 0, 10, 1000);
    if ret < 0 { dev_err((*trng).dev, b"Timeout waiting for TRNG RAW Data valid\n\0".as_ptr() as _); return ret as isize; }
    *(buf as *mut u32) = readl((*trng).base.cast::<u8>().add(TRNG_RAW_DATA_OUT));
    4
}

unsafe extern "C" fn airoha_trng_irq(_irq: i32, priv_: *mut core::ffi::c_void) -> i32 {
    let trng = priv_ as *mut AirohaTrng;
    airoha_trng_irq_mask(trng);
    // Just complete the task, we will read the value later
    complete(&mut (*trng).rng_op_done);
    1 // IRQ_HANDLED
}

unsafe extern "C" fn airoha_trng_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev as *mut Device;
    let trng = devm_kzalloc(dev, core::mem::size_of::<AirohaTrng>(), 0);
    if trng.is_null() { return -12; }
    (*trng).base = devm_platform_ioremap_resource(pdev, 0);
    if (*trng).base as isize == -1 { return -1; }
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    airoha_trng_irq_mask(trng);
    let ret = devm_request_irq(dev, irq, airoha_trng_irq, 0, (*pdev).name, trng.cast());
    if ret != 0 { return ret; }
    init_completion(&mut (*trng).rng_op_done);
    let addr = (*trng).base.cast::<u8>().add(TRNG_INTR_EN);
    writel(readl(addr) | RST_STARTUP_INITR_EN, addr);
    let addr = (*trng).base.cast::<u8>().add(TRNG_NS_SEK_AND_DAT_EN);
    writel(readl(addr) | RAW_DATA_EN, addr);
    writel(SW_RST, (*trng).base.cast::<u8>().add(TRNG_HEALTH_TEST_SW_RST));
    (*trng).dev = dev;
    (*trng).rng.name = (*pdev).name;
    (*trng).rng.init = Some(airoha_trng_init);
    (*trng).rng.cleanup = Some(airoha_trng_cleanup);
    (*trng).rng.read = Some(airoha_trng_read);
    (*trng).rng.quality = 900;
    let ret = devm_hwrng_register(dev, &mut (*trng).rng);
    if ret != 0 { dev_err(dev, b"failed to register rng device: %d\n\0".as_ptr() as _, ret); return ret; }
    0
}

static AIROHA_TRNG_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"airoha,en7581-trng\0".as_ptr() as _ },
    OfDeviceId { compatible: core::ptr::null() },
];

// MODULE_DEVICE_TABLE(of, airoha_trng_of_match);
// module_platform_driver(airoha_trng_driver);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Christian Marangi <ansuelsmth@gmail.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
