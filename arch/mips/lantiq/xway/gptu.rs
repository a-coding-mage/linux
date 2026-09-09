// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *  Copyright (C) 2012 John Crispin <john@phrozen.org>
 *  Copyright (C) 2012 Lantiq GmbH
 */

// External Linux, Lantiq, and clock declarations are supplied by other files.

const GPTU_MAGIC: u32 = 0x59;
const GPTU_CLC: usize = 0x00;
const GPTU_ID: usize = 0x08;
const GPTU_IRNEN: usize = 0xf4;
const GPTU_IRCR: usize = 0xf8;
const GPTU_IRNCR: usize = 0xfc;

#[inline]
const fn gptu_shift(x: usize) -> usize { if x % 2 != 0 { 4 } else { 0 } }
#[inline]
const fn gptu_base(x: usize) -> usize { ((x >> 1) * 0x20) + 0x10 }
#[inline]
const fn gptu_con(x: usize) -> usize { gptu_base(x) + gptu_shift(x) }
#[inline]
const fn gptu_run(x: usize) -> usize { gptu_base(x) + gptu_shift(x) + 0x08 }
#[inline]
const fn gptu_rld(x: usize) -> usize { gptu_base(x) + gptu_shift(x) + 0x10 }
#[inline]
const fn gptu_cnt(x: usize) -> usize { gptu_base(x) + gptu_shift(x) + 0x18 }

const CON_CNT: u32 = 1 << 2;
const CON_EDGE_ANY: u32 = (1 << 7) | (1 << 6);
const CON_SYNC: u32 = 1 << 8;
const CON_CLK_INT: u32 = 1 << 10;
const RUN_SEN: u32 = 1 << 0;
const RUN_RL: u32 = 1 << 2;
const CLC_RMC: u32 = 1 << 8;
const CLC_SUSPEND: u32 = 1 << 4;
const CLC_DISABLE: u32 = 1 << 0;

#[repr(usize)]
enum GptuTimer { Timer1a = 0, Timer1b, Timer2a, Timer2b, Timer3a, Timer3b }

static mut gptu_membase: *mut core::ffi::c_void = core::ptr::null_mut();
static mut irqres: [struct_resource; 6] = [struct_resource { _opaque: 0 }; 6];

unsafe fn gptu_w32(x: u32, y: usize) { ltq_w32(x, gptu_membase, y); }
unsafe fn gptu_r32(x: usize) -> u32 { ltq_r32(gptu_membase, x) }

unsafe extern "C" {
    fn ltq_w32(x: u32, base: *mut core::ffi::c_void, offset: usize);
    fn ltq_r32(base: *mut core::ffi::c_void, offset: usize) -> u32;
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32, flags: u32, name: *const u8, dev: *mut core::ffi::c_void) -> i32;
    fn free_irq(irq: i32, dev: *mut core::ffi::c_void);
}

unsafe extern "C" fn timer_irq_handler(irq: i32, _priv: *mut core::ffi::c_void) -> i32 {
    let timer = irq - irqres[0].start;
    gptu_w32(1u32 << timer, GPTU_IRNCR);
    1
}

unsafe fn gptu_hwinit() {
    gptu_w32(0x00, GPTU_IRNEN);
    gptu_w32(0xff, GPTU_IRNCR);
    gptu_w32(CLC_RMC | CLC_SUSPEND, GPTU_CLC);
}

unsafe fn gptu_hwexit() {
    gptu_w32(0x00, GPTU_IRNEN);
    gptu_w32(0xff, GPTU_IRNCR);
    gptu_w32(CLC_DISABLE, GPTU_CLC);
}

unsafe fn gptu_enable(clk: *mut struct_clk) -> i32 {
    let ret = request_irq(irqres[(*clk).bits].start, timer_irq_handler, IRQF_TIMER, b"gtpu\0".as_ptr(), core::ptr::null_mut());
    if ret != 0 { return ret; }
    gptu_w32(CON_CNT | CON_EDGE_ANY | CON_SYNC | CON_CLK_INT, gptu_con((*clk).bits));
    gptu_w32(1, gptu_rld((*clk).bits));
    gptu_w32(gptu_r32(GPTU_IRNEN) | (1 << (*clk).bits), GPTU_IRNEN);
    gptu_w32(RUN_SEN | RUN_RL, gptu_run((*clk).bits));
    0
}

unsafe fn gptu_disable(clk: *mut struct_clk) {
    gptu_w32(0, gptu_run((*clk).bits));
    gptu_w32(0, gptu_con((*clk).bits));
    gptu_w32(0, gptu_rld((*clk).bits));
    gptu_w32(gptu_r32(GPTU_IRNEN) & !(1 << (*clk).bits), GPTU_IRNEN);
    free_irq(irqres[(*clk).bits].start, core::ptr::null_mut());
}

// The remaining platform-driver registration declarations and initialization
// are supplied by the surrounding kernel translation.

unsafe fn clkdev_add_gptu(dev: *mut struct_device, con: *const u8, timer: usize) {
    let clk = kzalloc_obj::<struct_clk>();
    if clk.is_null() { return; }
    (*clk).cl.dev_id = dev_name(dev);
    (*clk).cl.con_id = con;
    (*clk).cl.clk = clk;
    (*clk).enable = Some(gptu_enable);
    (*clk).disable = Some(gptu_disable);
    (*clk).bits = timer;
    clkdev_add(&mut (*clk).cl);
}

unsafe fn gptu_probe(pdev: *mut struct_platform_device) -> i32 {
    let mut clk: *mut struct_clk;
    if of_irq_to_resource_table((*pdev).dev.of_node, irqres.as_mut_ptr(), 6) != 6 {
        return -22;
    }
    gptu_membase = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if is_err(gptu_membase) { return ptr_err(gptu_membase); }
    clk = clk_get(&mut (*pdev).dev, core::ptr::null());
    if is_err(clk) { return -2; }
    clk_enable(clk);
    gptu_hwinit();
    if ((gptu_r32(GPTU_ID) >> 8) & 0xff) != GPTU_MAGIC {
        gptu_hwexit();
        clk_disable(clk);
        clk_put(clk);
        return -119;
    }
    clkdev_add_gptu(&mut (*pdev).dev, b"timer1a\0".as_ptr(), GptuTimer::Timer1a as usize);
    clkdev_add_gptu(&mut (*pdev).dev, b"timer1b\0".as_ptr(), GptuTimer::Timer1b as usize);
    clkdev_add_gptu(&mut (*pdev).dev, b"timer2a\0".as_ptr(), GptuTimer::Timer2a as usize);
    clkdev_add_gptu(&mut (*pdev).dev, b"timer2b\0".as_ptr(), GptuTimer::Timer2b as usize);
    clkdev_add_gptu(&mut (*pdev).dev, b"timer3a\0".as_ptr(), GptuTimer::Timer3a as usize);
    clkdev_add_gptu(&mut (*pdev).dev, b"timer3b\0".as_ptr(), GptuTimer::Timer3b as usize);
    0
}

#[no_mangle]
unsafe extern "C" fn gptu_init() -> i32 { platform_driver_register(&mut dma_driver) }

static mut gptu_match: [struct_of_device_id; 2] = [
    struct_of_device_id { compatible: b"lantiq,gptu-xway\0".as_ptr() },
    struct_of_device_id { compatible: core::ptr::null() },
];

static mut dma_driver: struct_platform_driver = struct_platform_driver {
    probe: Some(gptu_probe),
    driver: struct_device_driver { name: b"gptu-xway\0".as_ptr(), of_match_table: unsafe { gptu_match.as_ptr() } },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
