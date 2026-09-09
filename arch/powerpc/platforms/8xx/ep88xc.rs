/*
 * Platform setup for the Embedded Planet EP88xC board
 *
 * Author: Scott Wood <scottwood@freescale.com>
 * Copyright 2007 Freescale Semiconductor, Inc.
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty of
 * any kind, whether express or implied.
 */

#[repr(C)]
struct CpmPin {
    port: i32,
    pin: i32,
    flags: i32,
}

static mut EP88XC_PINS: [CpmPin; 46] = [
    CpmPin { port: 1, pin: 24, flags: CPM_PIN_INPUT },
    CpmPin { port: 1, pin: 25, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },
    CpmPin { port: 0, pin: 12, flags: CPM_PIN_INPUT },
    CpmPin { port: 0, pin: 13, flags: CPM_PIN_INPUT },
    CpmPin { port: 2, pin: 8, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_GPIO },
    CpmPin { port: 2, pin: 9, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_GPIO },
    CpmPin { port: 2, pin: 14, flags: CPM_PIN_INPUT },
    CpmPin { port: 0, pin: 0, flags: CPM_PIN_INPUT }, CpmPin { port: 0, pin: 1, flags: CPM_PIN_INPUT },
    CpmPin { port: 0, pin: 2, flags: CPM_PIN_INPUT }, CpmPin { port: 0, pin: 3, flags: CPM_PIN_INPUT },
    CpmPin { port: 0, pin: 4, flags: CPM_PIN_OUTPUT }, CpmPin { port: 0, pin: 10, flags: CPM_PIN_OUTPUT },
    CpmPin { port: 0, pin: 11, flags: CPM_PIN_OUTPUT }, CpmPin { port: 1, pin: 19, flags: CPM_PIN_INPUT },
    CpmPin { port: 1, pin: 31, flags: CPM_PIN_INPUT }, CpmPin { port: 2, pin: 12, flags: CPM_PIN_INPUT },
    CpmPin { port: 2, pin: 13, flags: CPM_PIN_INPUT }, CpmPin { port: 3, pin: 8, flags: CPM_PIN_INPUT },
    CpmPin { port: 4, pin: 30, flags: CPM_PIN_OUTPUT }, CpmPin { port: 4, pin: 31, flags: CPM_PIN_OUTPUT },
    CpmPin { port: 4, pin: 14, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY }, CpmPin { port: 4, pin: 15, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    CpmPin { port: 4, pin: 16, flags: CPM_PIN_OUTPUT }, CpmPin { port: 4, pin: 17, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    CpmPin { port: 4, pin: 18, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY }, CpmPin { port: 4, pin: 19, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    CpmPin { port: 4, pin: 20, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY }, CpmPin { port: 4, pin: 21, flags: CPM_PIN_OUTPUT },
    CpmPin { port: 4, pin: 22, flags: CPM_PIN_OUTPUT }, CpmPin { port: 4, pin: 23, flags: CPM_PIN_OUTPUT },
    CpmPin { port: 4, pin: 24, flags: CPM_PIN_OUTPUT }, CpmPin { port: 4, pin: 25, flags: CPM_PIN_OUTPUT },
    CpmPin { port: 4, pin: 26, flags: CPM_PIN_OUTPUT }, CpmPin { port: 4, pin: 27, flags: CPM_PIN_OUTPUT },
    CpmPin { port: 4, pin: 28, flags: CPM_PIN_OUTPUT }, CpmPin { port: 4, pin: 29, flags: CPM_PIN_OUTPUT },
    CpmPin { port: 0, pin: 6, flags: CPM_PIN_INPUT }, CpmPin { port: 0, pin: 14, flags: CPM_PIN_INPUT },
    CpmPin { port: 0, pin: 15, flags: CPM_PIN_INPUT }, CpmPin { port: 2, pin: 6, flags: CPM_PIN_OUTPUT },
    CpmPin { port: 2, pin: 7, flags: CPM_PIN_OUTPUT }, CpmPin { port: 2, pin: 10, flags: CPM_PIN_INPUT },
    CpmPin { port: 2, pin: 11, flags: CPM_PIN_INPUT }, CpmPin { port: 1, pin: 26, flags: CPM_PIN_INPUT },
    CpmPin { port: 1, pin: 27, flags: CPM_PIN_INPUT },
];

unsafe fn init_ioports() {
    for pin in EP88XC_PINS.iter_mut() {
        cpm1_set_pin(pin.port, pin.pin, pin.flags);
    }
    cpm1_clk_setup(CPM_CLK_SMC1, CPM_BRG1, CPM_CLK_RTX);
    cpm1_clk_setup(CPM_CLK_SCC1, CPM_CLK2, CPM_CLK_TX);
    cpm1_clk_setup(CPM_CLK_SCC1, CPM_CLK2, CPM_CLK_RX);
    cpm1_clk_setup(CPM_CLK_SCC2, CPM_BRG2, CPM_CLK_TX);
    cpm1_clk_setup(CPM_CLK_SCC2, CPM_BRG2, CPM_CLK_RX);
}

static mut EP88XC_BCSR: *mut u8 = core::ptr::null_mut();

const BCSR7_SCC2_ENABLE: u8 = 0x10;
const BCSR8_PHY1_ENABLE: u8 = 0x80;
const BCSR8_PHY1_POWER: u8 = 0x40;
const BCSR8_PHY2_ENABLE: u8 = 0x20;
const BCSR8_PHY2_POWER: u8 = 0x10;
const BCSR9_USB_ENABLE: u8 = 0x80;
const BCSR9_USB_POWER: u8 = 0x40;
const BCSR9_USB_HOST: u8 = 0x20;
const BCSR9_USB_FULL_SPEED_TARGET: u8 = 0x10;

unsafe fn ep88xc_setup_arch() {
    cpm_reset();
    init_ioports();
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,ep88xc-bcsr\0".as_ptr() as *const i8);
    if np.is_null() { printk("Could not find fsl,ep88xc-bcsr node\n\0".as_ptr() as *const i8); return; }
    EP88XC_BCSR = of_iomap(np, 0);
    of_node_put(np);
    if EP88XC_BCSR.is_null() { printk("Could not remap BCSR\n\0".as_ptr() as *const i8); return; }
    setbits8(EP88XC_BCSR.add(7), BCSR7_SCC2_ENABLE);
    setbits8(EP88XC_BCSR.add(8), BCSR8_PHY1_ENABLE | BCSR8_PHY1_POWER | BCSR8_PHY2_ENABLE | BCSR8_PHY2_POWER);
}

#[repr(C)]
struct OfDeviceId { name: *const i8 }
static OF_BUS_IDS: [OfDeviceId; 4] = [
    OfDeviceId { name: "soc\0".as_ptr() as *const i8 },
    OfDeviceId { name: "cpm\0".as_ptr() as *const i8 },
    OfDeviceId { name: "localbus\0".as_ptr() as *const i8 },
    OfDeviceId { name: core::ptr::null() },
];

unsafe fn declare_of_platform_devices() -> i32 {
    of_platform_bus_probe(core::ptr::null_mut(), OF_BUS_IDS.as_ptr(), core::ptr::null_mut());
    0
}

#[repr(C)]
struct MachineDesc {
    name: *const i8,
    compatible: *const i8,
    setup_arch: unsafe fn(),
    init_irq: unsafe extern "C" fn(),
    get_irq: unsafe extern "C" fn() -> i32,
    restart: unsafe extern "C" fn(),
    calibrate_decr: unsafe extern "C" fn(),
    progress: unsafe extern "C" fn(*const i8, u32),
}

static EP88XC_MACHINE: MachineDesc = MachineDesc {
    name: "Embedded Planet EP88xC\0".as_ptr() as *const i8,
    compatible: "fsl,ep88xc\0".as_ptr() as *const i8,
    setup_arch: ep88xc_setup_arch,
    init_irq: mpc8xx_pic_init,
    get_irq: mpc8xx_get_irq,
    restart: mpc8xx_restart,
    calibrate_decr: mpc8xx_calibrate_decr,
    progress: udbg_progress,
};

extern "C" {
    fn cpm_reset(); fn cpm1_set_pin(port: i32, pin: i32, flags: i32);
    fn cpm1_clk_setup(a: i32, b: i32, c: i32); fn printk(s: *const i8);
    fn of_find_compatible_node(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void, c: *const i8) -> *mut core::ffi::c_void;
    fn of_iomap(n: *mut core::ffi::c_void, index: i32) -> *mut u8; fn of_node_put(n: *mut core::ffi::c_void);
    fn setbits8(p: *mut u8, v: u8); fn of_platform_bus_probe(a: *mut core::ffi::c_void, b: *const OfDeviceId, c: *mut core::ffi::c_void);
    fn mpc8xx_pic_init(); fn mpc8xx_get_irq() -> i32; fn mpc8xx_restart();
    fn mpc8xx_calibrate_decr(); fn udbg_progress(s: *const i8, v: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
