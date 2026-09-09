// SPDX-License-Identifier: GPL-2.0-only
/* OMAP1/OMAP7xx - specific DMA driver (translated from dma.c). */

// Linux headers and symbols referenced below are supplied by the surrounding kernel bindings.

use core::ffi::{c_char, c_int, c_uint, c_void};

type u16 = core::primitive::u16;
type u32 = core::primitive::u32;
type uint = c_uint;

#[repr(C)]
pub struct omap_dma_reg { pub offset: u32, pub stride: u32, pub type_: u32 }
#[repr(C)] pub struct resource { pub start: usize, pub end: usize, pub flags: u32, pub name: *const c_char }
#[repr(C)] pub struct platform_device;
#[repr(C)] pub struct device;
#[repr(C)] pub struct omap_dma_dev_attr { pub dev_caps: u32, pub lch_count: u32 }
#[repr(C)] pub struct omap_system_dma_plat_info {
    pub reg_map: *const omap_dma_reg, pub channel_stride: u32,
    pub show_dma_caps: Option<unsafe extern "C" fn()>,
    pub clear_lch_regs: Option<unsafe extern "C" fn(c_int)>,
    pub clear_dma: Option<unsafe extern "C" fn(c_int)>,
    pub dma_write: Option<unsafe extern "C" fn(u32, c_int, c_int)>,
    pub dma_read: Option<unsafe extern "C" fn(c_int, c_int) -> u32>,
    pub dma_attr: *mut omap_dma_dev_attr, pub errata: u32,
    pub slave_map: *const dma_slave_map, pub slavecnt: usize,
}
#[repr(C)] pub struct dma_slave_map { pub slave: *const c_char, pub name: *const c_char, pub param: u32 }
#[repr(C)] pub struct platform_device_info { pub name: *const c_char, pub id: c_int, pub dma_mask: u64, pub res: *mut resource, pub num_res: usize }

extern "C" {
    static mut dma_base: *mut c_void;
    fn cpu_is_omap15xx() -> bool;
    fn cpu_is_omap16xx() -> bool;
    fn ioremap(start: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn __raw_writew(val: u16, addr: *mut c_void);
    fn __raw_readw(addr: *mut c_void) -> u16;
    fn printk(fmt: *const c_char, ...);
    fn platform_device_alloc(name: *const c_char, id: c_int) -> *mut platform_device;
    fn platform_device_add_resources(pdev: *mut platform_device, r: *mut resource, n: usize) -> c_int;
    fn platform_device_add_data(pdev: *mut platform_device, data: *const c_void, size: usize) -> c_int;
    fn platform_device_add(pdev: *mut platform_device) -> c_int;
    fn platform_device_register_full(info: *const platform_device_info) -> *mut platform_device;
    fn platform_device_del(pdev: *mut platform_device);
    fn platform_device_put(pdev: *mut platform_device);
    fn kfree(p: *mut c_void);
}

const OMAP1_DMA_BASE: usize = 0xfffed800;
static mut enable_1510_mode: u32 = 0;

static mut res: [resource; 18] = [resource { start: OMAP1_DMA_BASE, end: OMAP1_DMA_BASE + 2048 - 1, flags: 0, name: core::ptr::null() }; 18];
static omap_dma_dev_info: platform_device_info = platform_device_info { name: b"omap-dma-engine\0".as_ptr() as *const c_char, id: -1, dma_mask: 0xffff_ffff, res: core::ptr::null_mut(), num_res: 1 };
static omap1xxx_sdma_map: [dma_slave_map; 16] = [dma_slave_map { slave: core::ptr::null(), name: core::ptr::null(), param: 0 }; 16];

// Register indices are provided by omap-dma.h; this table follows the C designated initializers.
static reg_map: [omap_dma_reg; 31] = [
    omap_dma_reg { offset: 0x0400, stride: 0, type_: 16 }, omap_dma_reg { offset: 0x0404, stride: 0, type_: 16 },
    omap_dma_reg { offset: 0x0408, stride: 0, type_: 16 }, omap_dma_reg { offset: 0x0442, stride: 0, type_: 16 },
    omap_dma_reg { offset: 0x0444, stride: 0, type_: 16 }, omap_dma_reg { offset: 0x0446, stride: 0, type_: 16 },
    omap_dma_reg { offset: 0x0448, stride: 0, type_: 16 }, omap_dma_reg { offset: 0x044a, stride: 0, type_: 16 },
    omap_dma_reg { offset: 0x044c, stride: 0, type_: 16 }, omap_dma_reg { offset: 0x044e, stride: 0, type_: 32 },
    omap_dma_reg { offset: 0x0452, stride: 0, type_: 32 }, omap_dma_reg { offset: 0x0456, stride: 0, type_: 16 },
    omap_dma_reg { offset: 0x0458, stride: 0, type_: 16 }, omap_dma_reg { offset: 0x045a, stride: 0, type_: 16 },
    omap_dma_reg { offset: 0x0460, stride: 0, type_: 16 }, omap_dma_reg { offset: 0x0480, stride: 0, type_: 16 },
    omap_dma_reg { offset: 0x0482, stride: 0, type_: 16 }, omap_dma_reg { offset: 0x04c0, stride: 0, type_: 16 },
    omap_dma_reg { offset: 0x0000, stride: 0x40, type_: 16 }, omap_dma_reg { offset: 0x0002, stride: 0x40, type_: 16 },
    omap_dma_reg { offset: 0x0004, stride: 0x40, type_: 16 }, omap_dma_reg { offset: 0x0006, stride: 0x40, type_: 16 },
    omap_dma_reg { offset: 0x0010, stride: 0x40, type_: 16 }, omap_dma_reg { offset: 0x0012, stride: 0x40, type_: 16 },
    omap_dma_reg { offset: 0x0014, stride: 0x40, type_: 16 }, omap_dma_reg { offset: 0x0016, stride: 0x40, type_: 16 },
    omap_dma_reg { offset: 0x0018, stride: 0x40, type_: 16 }, omap_dma_reg { offset: 0x0018, stride: 0x40, type_: 16 },
    omap_dma_reg { offset: 0x001a, stride: 0x40, type_: 16 }, omap_dma_reg { offset: 0x001c, stride: 0x40, type_: 16 },
    omap_dma_reg { offset: 0x001e, stride: 0x40, type_: 16 }, omap_dma_reg { offset: 0x0028, stride: 0x40, type_: 16 },
];

unsafe fn dma_write(val: u32, reg: usize, lch: usize) { let a = dma_base.add(reg_map[reg].offset as usize + reg_map[reg].stride as usize * lch); __raw_writew(val as u16, a); if reg_map[reg].type_ == 32 { __raw_writew((val >> 16) as u16, a.add(2)); } }
unsafe fn dma_read(reg: usize, lch: usize) -> u32 { let a = dma_base.add(reg_map[reg].offset as usize + reg_map[reg].stride as usize * lch); let mut v = __raw_readw(a) as u32; if reg_map[reg].type_ == 32 { v |= (__raw_readw(a.add(2)) as u32) << 16; } v }
unsafe fn omap1_clear_lch_regs(lch: c_int) { for i in 0..=29 { dma_write(0, i, lch as usize); } }
unsafe fn omap1_clear_dma(lch: c_int) { let mut l = dma_read(19, lch as usize); l &= !1; dma_write(l, 19, lch as usize); let _ = dma_read(21, lch as usize); }
unsafe fn omap1_show_dma_caps() { if enable_1510_mode != 0 { printk(b"DMA support for OMAP15xx initialized\n\0".as_ptr() as *const c_char); } else { printk(b"OMAP DMA hardware version %d\n\0".as_ptr() as *const c_char, dma_read(3, 0)); printk(b"DMA capabilities: %08x:%08x:%04x:%04x:%04x\n\0".as_ptr() as *const c_char, dma_read(9, 0), dma_read(10, 0), dma_read(11, 0), dma_read(12, 0), dma_read(13, 0)); let mut w = dma_read(1, 0) as u16; w |= 1 << 3; dma_write(w as u32, 1, 0); } }

unsafe fn configure_dma_errata() -> uint { if !cpu_is_omap15xx() { 1 } else { 0 } }

// The remaining platform registration routine is retained as an external-facing initialization hook.
#[no_mangle]
pub unsafe extern "C" fn omap1_system_dma_init() -> c_int {
    let pdev = platform_device_alloc(b"omap_dma_system\0".as_ptr() as *const c_char, 0);
    if pdev.is_null() { return -12; }
    dma_base = ioremap(res[0].start, res[0].end - res[0].start + 1);
    if dma_base.is_null() { platform_device_put(pdev); return -19; }
    let mut ret = platform_device_add_resources(pdev, res.as_mut_ptr(), 18);
    if ret != 0 { iounmap(dma_base); platform_device_put(pdev); return ret; }
    let mut d = omap_dma_dev_attr { dev_caps: 0, lch_count: 0 };
    if cpu_is_omap15xx() { d.dev_caps = 1; }
    enable_1510_mode = d.dev_caps & 1;
    if cpu_is_omap16xx() { d.dev_caps = 2; }
    d.dev_caps |= 0x7c;
    d.lch_count = if cpu_is_omap15xx() || (d.dev_caps & 1) != 0 { 9 } else { 16 };
    ret = platform_device_add_data(pdev, &d as *const _ as *const c_void, core::mem::size_of_val(&d));
    if ret != 0 { iounmap(dma_base); platform_device_put(pdev); return ret; }
    ret = platform_device_add(pdev);
    if ret != 0 { kfree(&mut d as *mut _ as *mut c_void); iounmap(dma_base); platform_device_put(pdev); return ret; }
    let dma_pdev = platform_device_register_full(&omap_dma_dev_info);
    if dma_pdev.is_null() { platform_device_del(pdev); iounmap(dma_base); platform_device_put(pdev); return -19; }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
