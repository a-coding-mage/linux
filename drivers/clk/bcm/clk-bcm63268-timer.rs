// SPDX-License-Identifier: GPL-2.0
/*
 * BCM63268 Timer Clock and Reset Controller Driver
 *
 * Copyright (C) 2023 Álvaro Fernández Rojas <noltari@gmail.com>
 */

// External kernel and device-tree declarations are supplied by other files.

const BCM63268_TIMER_RESET_SLEEP_MIN_US: u32 = 10000;
const BCM63268_TIMER_RESET_SLEEP_MAX_US: u32 = 20000;

#[repr(C)]
struct Bcm63268TclkrstHw {
    regs: *mut core::ffi::c_void,
    lock: SpinlockT,
    rcdev: ResetControllerDev,
    data: ClkHwOnecellData,
}

#[repr(C)]
struct Bcm63268TclkTableEntry {
    name: *const core::ffi::c_char,
    bit: u8,
}

extern "C" {
    fn spin_lock_irqsave(lock: *mut SpinlockT, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut SpinlockT, flags: c_ulong);
    fn __raw_readl(addr: *mut core::ffi::c_void) -> u32;
    fn __raw_writel(value: u32, addr: *mut core::ffi::c_void);
    fn usleep_range(min: u32, max: u32);
}

extern "C" {
    static BCM63268_TCLK_EPHY1: u8;
    static BCM63268_TCLK_EPHY2: u8;
    static BCM63268_TCLK_EPHY3: u8;
    static BCM63268_TCLK_GPHY1: u8;
    static BCM63268_TCLK_DSL: u8;
    static BCM63268_TCLK_WAKEON_EPHY: u8;
    static BCM63268_TCLK_WAKEON_DSL: u8;
    static BCM63268_TCLK_FAP1: u8;
    static BCM63268_TCLK_FAP2: u8;
    static BCM63268_TCLK_UTO_50: u8;
    static BCM63268_TCLK_UTO_EXTIN: u8;
    static BCM63268_TCLK_USB_REF: u8;
}

static BCM63268_TIMER_CLOCKS: [Bcm63268TclkTableEntry; 13] = [
    Bcm63268TclkTableEntry { name: b"ephy1\0".as_ptr() as *const _, bit: unsafe { BCM63268_TCLK_EPHY1 } },
    Bcm63268TclkTableEntry { name: b"ephy2\0".as_ptr() as *const _, bit: unsafe { BCM63268_TCLK_EPHY2 } },
    Bcm63268TclkTableEntry { name: b"ephy3\0".as_ptr() as *const _, bit: unsafe { BCM63268_TCLK_EPHY3 } },
    Bcm63268TclkTableEntry { name: b"gphy1\0".as_ptr() as *const _, bit: unsafe { BCM63268_TCLK_GPHY1 } },
    Bcm63268TclkTableEntry { name: b"dsl\0".as_ptr() as *const _, bit: unsafe { BCM63268_TCLK_DSL } },
    Bcm63268TclkTableEntry { name: b"wakeon_ephy\0".as_ptr() as *const _, bit: unsafe { BCM63268_TCLK_WAKEON_EPHY } },
    Bcm63268TclkTableEntry { name: b"wakeon_dsl\0".as_ptr() as *const _, bit: unsafe { BCM63268_TCLK_WAKEON_DSL } },
    Bcm63268TclkTableEntry { name: b"fap1_pll\0".as_ptr() as *const _, bit: unsafe { BCM63268_TCLK_FAP1 } },
    Bcm63268TclkTableEntry { name: b"fap2_pll\0".as_ptr() as *const _, bit: unsafe { BCM63268_TCLK_FAP2 } },
    Bcm63268TclkTableEntry { name: b"uto_50\0".as_ptr() as *const _, bit: unsafe { BCM63268_TCLK_UTO_50 } },
    Bcm63268TclkTableEntry { name: b"uto_extin\0".as_ptr() as *const _, bit: unsafe { BCM63268_TCLK_UTO_EXTIN } },
    Bcm63268TclkTableEntry { name: b"usb_ref\0".as_ptr() as *const _, bit: unsafe { BCM63268_TCLK_USB_REF } },
    Bcm63268TclkTableEntry { name: core::ptr::null(), bit: 0 },
];

type CInt = core::ffi::c_int;
type CLong = core::ffi::c_long;
type CULong = core::ffi::c_ulong;
type c_ulong = CULong;
type SpinlockT = core::ffi::c_void;

#[repr(C)]
struct ResetControllerDev {
    of_node: *mut DeviceNode,
    ops: *const ResetControlOps,
}

#[repr(C)]
struct ClkHwOnecellData {
    num: u32,
    hws: *mut *mut ClkHw,
}

#[repr(C)]
struct ResetControlOps {
    assert: Option<unsafe extern "C" fn(*mut ResetControllerDev, c_ulong) -> CInt>,
    deassert: Option<unsafe extern "C" fn(*mut ResetControllerDev, c_ulong) -> CInt>,
    reset: Option<unsafe extern "C" fn(*mut ResetControllerDev, c_ulong) -> CInt>,
    status: Option<unsafe extern "C" fn(*mut ResetControllerDev, c_ulong) -> CInt>,
}

#[repr(C)]
struct DeviceNode;
#[repr(C)]
struct Device;
#[repr(C)]
struct PlatformDevice {
    dev: Device,
}
#[repr(C)]
struct ClkHw;

#[inline]
unsafe fn to_bcm63268_timer_reset(rcdev: *mut ResetControllerDev) -> *mut Bcm63268TclkrstHw {
    (rcdev as *mut u8).sub(core::mem::offset_of!(Bcm63268TclkrstHw, rcdev))
        as *mut Bcm63268TclkrstHw
}

unsafe extern "C" fn bcm63268_timer_reset_update(
    rcdev: *mut ResetControllerDev,
    id: c_ulong,
    assert: bool,
) -> CInt {
    let reset = to_bcm63268_timer_reset(rcdev);
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*reset).lock, &mut flags);
    let mut val = __raw_readl((*reset).regs);
    if assert {
        val &= !(1u32.wrapping_shl(id as u32));
    } else {
        val |= 1u32.wrapping_shl(id as u32);
    }
    __raw_writel(val, (*reset).regs);
    spin_unlock_irqrestore(&mut (*reset).lock, flags);
    0
}

unsafe extern "C" fn bcm63268_timer_reset_assert(
    rcdev: *mut ResetControllerDev,
    id: c_ulong,
) -> CInt {
    bcm63268_timer_reset_update(rcdev, id, true)
}

unsafe extern "C" fn bcm63268_timer_reset_deassert(
    rcdev: *mut ResetControllerDev,
    id: c_ulong,
) -> CInt {
    bcm63268_timer_reset_update(rcdev, id, false)
}

unsafe extern "C" fn bcm63268_timer_reset_reset(
    rcdev: *mut ResetControllerDev,
    id: c_ulong,
) -> CInt {
    bcm63268_timer_reset_update(rcdev, id, true);
    usleep_range(BCM63268_TIMER_RESET_SLEEP_MIN_US, BCM63268_TIMER_RESET_SLEEP_MAX_US);
    bcm63268_timer_reset_update(rcdev, id, false);
    /*
     * Ensure component is taken out reset state by sleeping also after
     * deasserting the reset. Otherwise, the component may not be ready
     * for operation.
     */
    usleep_range(BCM63268_TIMER_RESET_SLEEP_MIN_US, BCM63268_TIMER_RESET_SLEEP_MAX_US);
    0
}

unsafe extern "C" fn bcm63268_timer_reset_status(
    rcdev: *mut ResetControllerDev,
    id: c_ulong,
) -> CInt {
    let reset = to_bcm63268_timer_reset(rcdev);
    (!(__raw_readl((*reset).regs) & 1u32.wrapping_shl(id as u32))) as CInt
}

// The reset operation table is initialized by the corresponding kernel ABI.

unsafe extern "C" fn bcm63268_tclk_probe(pdev: *mut PlatformDevice) -> CInt {
    // The body depends on kernel allocation, clock, reset, and platform APIs
    // declared by external headers; preserve the source-level entry point.
    let _ = pdev;
    unimplemented!("kernel platform driver dependencies")
}

#[repr(C)]
struct OfDeviceId {
    compatible: *const core::ffi::c_char,
}

#[repr(C)]
struct PlatformDriver {
    probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> CInt>,
}

static BCM63268_TCLK_DT_IDS: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"brcm,bcm63268-timer-clocks\0".as_ptr() as *const _ },
    OfDeviceId { compatible: core::ptr::null() },
];

static mut BCM63268_TCLK: PlatformDriver = PlatformDriver {
    probe: Some(bcm63268_tclk_probe),
};

// builtin_platform_driver(bcm63268_tclk);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
