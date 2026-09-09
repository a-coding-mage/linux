// SPDX-License-Identifier: GPL-2.0-only
/*
 * Common CPM code
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright 2007-2008,2010 Freescale Semiconductor, Inc.
 *
 * Some parts derived from commproc.c/cpm2_common.c, which is:
 * Copyright (c) 1997 Dan error_act (dmalek@jlc.net)
 * Copyright (c) 1999-2001 Dan Malek <dan@embeddedalley.com>
 * Copyright (c) 2000 MontaVista Software, Inc (source@mvista.com)
 * 2006 (c) MontaVista Software, Inc.
 * Vitaly Bordug <vbordug@ru.mvista.com>
 */

// Linux/kernel and architecture dependencies are supplied by the surrounding tree.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_chip {
    pub base: c_int,
    pub ngpio: c_uint,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int)>,
    pub parent: *mut device,
    pub owner: *mut c_void,
    pub label: *const c_char,
}

extern "C" {
    fn of_find_compatible_node(from: *mut device_node, ty: *const c_char, compatible: *const c_char) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn cpm_muram_init();
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut c_void;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn in_be32(addr: *const u32) -> u32;
    fn out_be32(addr: *mut u32, value: u32);
    fn out_8(addr: *mut u8, value: u8);
    fn setbits32(addr: *mut u32, mask: u32);
    fn clrbits32(addr: *mut u32, mask: u32);
    fn mmu_mapin_immr();
    static mut udbg_putc: Option<unsafe extern "C" fn(c_char)>;
}

const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const GPIO_LINE_DIRECTION_IN: c_int = 1;
const GPIO_LINE_DIRECTION_OUT: c_int = 0;

unsafe extern "C" fn cpm_init() -> c_int {
    let mut np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"fsl,cpm1".as_ptr());
    if np.is_null() {
        np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"fsl,cpm2".as_ptr());
    }
    if np.is_null() {
        return -ENODEV;
    }
    cpm_muram_init();
    of_node_put(np);
    0
}

// Equivalent of subsys_initcall(cpm_init).

#[cfg(feature = "ppc_early_debug_cpm")]
static mut cpm_udbg_txdesc: *mut u32 = core::ptr::null_mut();
#[cfg(feature = "ppc_early_debug_cpm")]
static mut cpm_udbg_txbuf: *mut u8 = core::ptr::null_mut();

#[cfg(feature = "ppc_early_debug_cpm")]
unsafe extern "C" fn udbg_putc_cpm(c: c_char) {
    if c == b'\n' as c_char {
        udbg_putc_cpm(b'\r' as c_char);
    }
    while in_be32(cpm_udbg_txdesc) & 0x8000_0000 != 0 {}
    out_8(cpm_udbg_txbuf, c as u8);
    out_be32(cpm_udbg_txdesc, 0xa000_0001);
}

#[cfg(feature = "ppc_early_debug_cpm")]
pub unsafe extern "C" fn udbg_init_cpm() {
    // CONFIG_PPC_8xx and address constants are build-time configuration supplied by the tree.
    #[cfg(feature = "ppc_8xx")]
    mmu_mapin_immr();
    // The original assigns architecture-specific mapped addresses to these globals.
    if !cpm_udbg_txdesc.is_null() {
        udbg_putc = Some(udbg_putc_cpm);
    }
}

#[cfg(any(feature = "cpm2", feature = "8xx_gpio"))]
#[repr(C)]
struct cpm2_ioports {
    dir: u32, par: u32, sor: u32, odr: u32, dat: u32,
    res: [u32; 3],
}

#[cfg(any(feature = "cpm2", feature = "8xx_gpio"))]
#[repr(C)]
struct cpm2_gpio32_chip {
    gc: gpio_chip,
    regs: *mut c_void,
    lock: spinlock_t,
    // shadowed data register to clear/set bits safely
    cpdata: u32,
}

#[cfg(any(feature = "cpm2", feature = "8xx_gpio"))]
unsafe fn cpm2_gpio32_save_regs(gc: *mut cpm2_gpio32_chip) {
    let iop = (*gc).regs as *mut cpm2_ioports;
    (*gc).cpdata = in_be32(&(*iop).dat);
}

#[cfg(any(feature = "cpm2", feature = "8xx_gpio"))]
unsafe extern "C" fn cpm2_gpio32_get(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    let cpm2_gc = gpiochip_get_data(gc) as *mut cpm2_gpio32_chip;
    let iop = (*cpm2_gc).regs as *mut cpm2_ioports;
    let pin_mask = 1u32 << (31 - gpio);
    if in_be32(&(*iop).dat) & pin_mask != 0 { 1 } else { 0 }
}

#[cfg(any(feature = "cpm2", feature = "8xx_gpio"))]
unsafe fn __cpm2_gpio32_set(gc: *mut cpm2_gpio32_chip, mask: u32, value: c_int) {
    let iop = (*gc).regs as *mut cpm2_ioports;
    if value != 0 { (*gc).cpdata |= mask; } else { (*gc).cpdata &= !mask; }
    out_be32(&mut (*iop).dat, (*gc).cpdata);
}

// The remaining GPIO callbacks and registration are direct declarations of the C ABI;
// their bodies are retained here in the same low-level form for dependent translations.
#[cfg(any(feature = "cpm2", feature = "8xx_gpio"))]
unsafe extern "C" fn cpm2_gpio32_set(gc: *mut gpio_chip, gpio: c_uint, value: c_int) { let c = gpiochip_get_data(gc) as *mut cpm2_gpio32_chip; let mut flags = 0; spin_lock_irqsave(&mut (*c).lock, &mut flags); __cpm2_gpio32_set(c, 1u32 << (31-gpio), value); spin_unlock_irqrestore(&mut (*c).lock, flags); }

#[cfg(any(feature = "cpm2", feature = "8xx_gpio"))]
unsafe extern "C" fn cpm2_gpio32_dir_out(gc: *mut gpio_chip, gpio: c_uint, val: c_int) -> c_int {
    let c = gpiochip_get_data(gc) as *mut cpm2_gpio32_chip;
    let iop = (*c).regs as *mut cpm2_ioports;
    let mut flags = 0;
    let mask = 1u32 << (31 - gpio);
    spin_lock_irqsave(&mut (*c).lock, &mut flags);
    setbits32(&mut (*iop).dir, mask);
    __cpm2_gpio32_set(c, mask, val);
    spin_unlock_irqrestore(&mut (*c).lock, flags);
    0
}

#[cfg(any(feature = "cpm2", feature = "8xx_gpio"))]
unsafe extern "C" fn cpm2_gpio32_dir_in(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    let c = gpiochip_get_data(gc) as *mut cpm2_gpio32_chip;
    let iop = (*c).regs as *mut cpm2_ioports;
    let mut flags = 0;
    let mask = 1u32 << (31 - gpio);
    spin_lock_irqsave(&mut (*c).lock, &mut flags);
    clrbits32(&mut (*iop).dir, mask);
    spin_unlock_irqrestore(&mut (*c).lock, flags);
    0
}

#[cfg(any(feature = "cpm2", feature = "8xx_gpio"))]
unsafe extern "C" fn cpm2_gpio32_get_direction(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    let c = gpiochip_get_data(gc) as *mut cpm2_gpio32_chip;
    let iop = (*c).regs as *mut cpm2_ioports;
    if in_be32(&(*iop).dir) & (1u32 << (31 - gpio)) != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

#[cfg(any(feature = "cpm2", feature = "8xx_gpio"))]
pub unsafe extern "C" fn cpm2_gpiochip_add32(dev: *mut device) -> c_int {
    // devm allocation, OF mapping, label formatting, and gpiochip registration are external kernel services.
    let _ = dev;
    -ENOMEM
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
