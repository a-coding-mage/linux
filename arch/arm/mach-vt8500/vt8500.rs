// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  arch/arm/mach-vt8500/vt8500.c
 *
 *  Copyright (C) 2012 Tony Prisk <linux@prisktech.co.nz>
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::{c_char, c_void};

const LEGACY_GPIO_BASE: usize = 0xD8110000;
const LEGACY_PMC_BASE: usize = 0xD8130000;

/* Registers in GPIO Controller */
const VT8500_GPIO_MUX_REG: usize = 0x200;

/* Registers in Power Management Controller */
const VT8500_HCR_REG: usize = 0x12;
const VT8500_PMSR_REG: usize = 0x60;

type IoMem = *mut u8;

#[repr(C)]
pub enum RebootMode {
    Unknown,
}

extern "C" {
    fn writel(value: u32, addr: *mut c_void);
    fn writew(value: u16, addr: *mut c_void);
    fn readl(addr: *const c_void) -> u32;
    fn ioremap(addr: usize, size: usize) -> IoMem;
    fn iounmap(addr: IoMem);
    fn of_find_compatible_node(from: *mut DeviceNode, type_: *const c_char,
                               compatible: *const c_char) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: i32) -> IoMem;
    fn of_node_put(node: *mut DeviceNode);
    fn pr_err(format: *const c_char, ...);
    fn local_irq_disable();
    fn register_platform_power_off(power_off: unsafe extern "C" fn());
    fn iotable_init(desc: *mut MapDesc, count: usize);
    fn __phys_to_pfn(value: usize) -> usize;
    fn __arm_mcr_p15_0_0_7_0_4(value: u32);
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct MapDesc {
    pub virtual_: usize,
    pub pfn: usize,
    pub length: usize,
    pub type_: usize,
}

const MT_DEVICE: usize = 0;
static mut pmc_base: IoMem = core::ptr::null_mut();

unsafe extern "C" fn vt8500_restart(_mode: RebootMode, _cmd: *const c_char) {
    if !pmc_base.is_null() {
        writel(1, pmc_base.add(VT8500_PMSR_REG) as *mut c_void);
    }
}

static mut vt8500_io_desc: [MapDesc; 1] = [MapDesc {
    virtual_: 0xf8000000,
    pfn: 0,
    length: 0x00390000, /* max of all chip variants */
    type_: MT_DEVICE,
}];

unsafe extern "C" fn vt8500_map_io() {
    vt8500_io_desc[0].pfn = __phys_to_pfn(0xd8000000);
    iotable_init(vt8500_io_desc.as_mut_ptr(), vt8500_io_desc.len());
}

unsafe extern "C" fn vt8500_power_off() {
    local_irq_disable();
    writew(5, pmc_base.add(VT8500_HCR_REG) as *mut c_void);
    __arm_mcr_p15_0_0_7_0_4(0);
}

unsafe extern "C" fn vt8500_init() {
    let mut np: *mut DeviceNode;

    // CONFIG_FB_VT8500 conditional section from the C source.
    #[cfg(CONFIG_FB_VT8500)]
    {
        let fb = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(),
                                         b"via,vt8500-fb\0".as_ptr() as *const c_char);
        if !fb.is_null() {
            np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(),
                                         b"via,vt8500-gpio\0".as_ptr() as *const c_char);
            let gpio_base = if !np.is_null() {
                let base = of_iomap(np, 0);
                if base.is_null() { pr_err(b"%s: of_iomap(gpio_mux) failed\n\0".as_ptr() as *const c_char); }
                of_node_put(np); base
            } else {
                let base = ioremap(LEGACY_GPIO_BASE, 0x1000);
                if base.is_null() { pr_err(b"%s: ioremap(legacy_gpio_mux) failed\n\0".as_ptr() as *const c_char); }
                base
            };
            if !gpio_base.is_null() {
                let addr = gpio_base.add(VT8500_GPIO_MUX_REG);
                writel(readl(addr as *const c_void) | 1, addr as *mut c_void);
                iounmap(gpio_base);
            } else { pr_err(b"%s: Could not remap GPIO mux\n\0".as_ptr() as *const c_char); }
            of_node_put(fb);
        }
    }

    // CONFIG_FB_WM8505 conditional section from the C source.
    #[cfg(CONFIG_FB_WM8505)]
    {
        let fb = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(),
                                         b"wm,wm8505-fb\0".as_ptr() as *const c_char);
        if !fb.is_null() {
            np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"wm,wm8505-gpio\0".as_ptr() as *const c_char);
            if np.is_null() { np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"wm,wm8650-gpio\0".as_ptr() as *const c_char); }
            let gpio_base = if !np.is_null() { let base = of_iomap(np, 0); if base.is_null() { pr_err(b"%s: of_iomap(gpio_mux) failed\n\0".as_ptr() as *const c_char); } of_node_put(np); base } else { let base = ioremap(LEGACY_GPIO_BASE, 0x1000); if base.is_null() { pr_err(b"%s: ioremap(legacy_gpio_mux) failed\n\0".as_ptr() as *const c_char); } base };
            if !gpio_base.is_null() { let addr = gpio_base.add(VT8500_GPIO_MUX_REG); writel(readl(addr as *const c_void) | 0x80000000, addr as *mut c_void); iounmap(gpio_base); } else { pr_err(b"%s: Could not remap GPIO mux\n\0".as_ptr() as *const c_char); }
            of_node_put(fb);
        }
    }

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"via,vt8500-pmc\0".as_ptr() as *const c_char);
    if !np.is_null() { pmc_base = of_iomap(np, 0); if pmc_base.is_null() { pr_err(b"%s:of_iomap(pmc) failed\n\0".as_ptr() as *const c_char); } of_node_put(np); } else { pmc_base = ioremap(LEGACY_PMC_BASE, 0x1000); if pmc_base.is_null() { pr_err(b"%s:ioremap(power_off) failed\n\0".as_ptr() as *const c_char); } }
    if !pmc_base.is_null() { register_platform_power_off(vt8500_power_off); } else { pr_err(b"%s: PMC Hibernation register could not be remapped, not enabling power off!\n\0".as_ptr() as *const c_char); }
}

pub static vt8500_dt_compat: &[Option<&[u8]>] = &[Some(b"via,vt8500\0"), Some(b"wm,wm8650\0"), Some(b"wm,wm8505\0"), Some(b"wm,wm8750\0"), Some(b"wm,wm8850\0"), None];

// DT_MACHINE_START(WMT_DT, "VIA/Wondermedia SoC (Device Tree Support)")
// .dt_compat = vt8500_dt_compat, .map_io = vt8500_map_io,
// .init_machine = vt8500_init, .restart = vt8500_restart, MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
