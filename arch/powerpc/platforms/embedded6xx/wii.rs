// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/platforms/embedded6xx/wii.c
 *
 * Nintendo Wii board-specific support
 * Copyright (C) 2008-2009 The GameCube Linux Team
 * Copyright (C) 2008,2009 Albert Herranz
 */

// Kernel headers and board-specific headers provide the external declarations
// used below. Build-time kernel configuration is intentionally left to the
// surrounding translation unit.

const DRV_MODULE_NAME: &str = "wii";

/* control block */
const HW_CTRL_COMPATIBLE: &str = "nintendo,hollywood-control";
const HW_CTRL_RESETS: usize = 0x94;
const HW_CTRL_RESETS_SYS: u32 = 1 << 0;

/* gpio */
const HW_GPIO_COMPATIBLE: &str = "nintendo,hollywood-gpio";

const fn hw_gpio_base(idx: usize) -> usize { idx * 0x20 }
const fn hw_gpio_out(idx: usize) -> usize { hw_gpio_base(idx) }
const fn hw_gpio_dir(idx: usize) -> usize { hw_gpio_base(idx) + 4 }
const HW_GPIO_OWNER: usize = hw_gpio_base(1) + 0x1c;

const HW_GPIO_SHUTDOWN: u32 = 1 << 1;
const HW_GPIO_SLOT_LED: u32 = 1 << 5;
const HW_GPIO_SENSOR_BAR: u32 = 1 << 8;

#[repr(C)]
pub struct DeviceNode { _private: [u8; 0] }

#[repr(C)]
pub struct Resource {
    pub start: usize,
    _rest: [u8; 0],
}

extern "C" {
    fn local_irq_disable();
    fn cpu_relax();
    fn pr_err(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn of_find_compatible_node(from: *mut DeviceNode, ty: *const u8, compatible: *const u8) -> *mut DeviceNode;
    fn of_address_to_resource(np: *mut DeviceNode, index: i32, res: *mut Resource) -> i32;
    fn ioremap(addr: usize, size: usize) -> *mut u8;
    fn of_node_put(np: *mut DeviceNode);
    fn flipper_pic_probe();
    fn hlwd_pic_probe();
    fn flipper_pic_get_irq() -> i32;
    fn hlwd_quiesce();
    fn flipper_quiesce();
    fn ug_udbg_init();
    fn udbg_progress(message: *const u8, value: u32);
    fn of_platform_populate(a: *mut DeviceNode, b: *const OfDeviceId, c: *mut core::ffi::c_void, d: *mut core::ffi::c_void);
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const u8,
}

#[repr(C)]
pub struct PpcMd {
    pub restart: Option<unsafe extern "C" fn(*mut u8) -> !>,
}

extern "C" {
    static mut ppc_md: PpcMd;
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
}

static mut hw_ctrl: *mut u8 = core::ptr::null_mut();
static mut hw_gpio: *mut u8 = core::ptr::null_mut();

unsafe fn clrbits32(addr: *mut u8, bits: u32) {
    let p = addr as *mut u32;
    core::ptr::write_volatile(p, core::ptr::read_volatile(p) & !bits);
}

unsafe fn setbits32(addr: *mut u8, bits: u32) {
    let p = addr as *mut u32;
    core::ptr::write_volatile(p, core::ptr::read_volatile(p) | bits);
}

unsafe fn wii_spin() -> ! {
    local_irq_disable();
    loop { cpu_relax(); }
}

unsafe fn wii_ioremap_hw_regs(name: *mut u8, compatible: *const u8) -> *mut u8 {
    let mut hw_regs = core::ptr::null_mut();
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), compatible);
    if np.is_null() {
        pr_err(b"wii: no compatible node found for %s\0".as_ptr(), compatible);
        return hw_regs;
    }
    let mut res = Resource { start: 0, _rest: [] };
    let error = of_address_to_resource(np, 0, &mut res);
    if error != 0 {
        pr_err(b"wii: no valid reg found\0".as_ptr());
        of_node_put(np);
        return hw_regs;
    }
    hw_regs = ioremap(res.start, 0);
    if !hw_regs.is_null() {
        pr_info(b"wii: %s at 0x%p mapped to 0x%p\n\0".as_ptr(), name, &res.start, hw_regs);
    }
    of_node_put(np);
    hw_regs
}

unsafe fn wii_setup_arch() {
    hw_ctrl = wii_ioremap_hw_regs(b"hw_ctrl\0".as_ptr() as *mut u8, HW_CTRL_COMPATIBLE.as_ptr());
    hw_gpio = wii_ioremap_hw_regs(b"hw_gpio\0".as_ptr() as *mut u8, HW_GPIO_COMPATIBLE.as_ptr());
    if !hw_gpio.is_null() {
        /* turn off the front blue led and IR light */
        clrbits32(hw_gpio.add(hw_gpio_out(0)), HW_GPIO_SLOT_LED | HW_GPIO_SENSOR_BAR);
    }
}

unsafe extern "C" fn wii_restart(_cmd: *mut u8) -> ! {
    local_irq_disable();
    if !hw_ctrl.is_null() {
        /* clear the system reset pin to cause a reset */
        clrbits32(hw_ctrl.add(HW_CTRL_RESETS), HW_CTRL_RESETS_SYS);
    }
    wii_spin()
}

unsafe extern "C" fn wii_power_off() {
    local_irq_disable();
    if !hw_gpio.is_null() {
        /* set the owner of the shutdown pin to ARM */
        clrbits32(hw_gpio.add(HW_GPIO_OWNER), HW_GPIO_SHUTDOWN);
        /* make sure that the poweroff GPIO is configured as output */
        setbits32(hw_gpio.add(hw_gpio_dir(1)), HW_GPIO_SHUTDOWN);
        /* drive the poweroff GPIO high */
        setbits32(hw_gpio.add(hw_gpio_out(1)), HW_GPIO_SHUTDOWN);
    }
    wii_spin();
}

unsafe extern "C" fn wii_halt() -> ! {
    if let Some(restart) = ppc_md.restart { restart(core::ptr::null_mut()); }
    wii_spin()
}

unsafe extern "C" fn wii_pic_probe() { flipper_pic_probe(); hlwd_pic_probe(); }

unsafe extern "C" fn wii_probe() -> i32 {
    pm_power_off = Some(wii_power_off);
    ug_udbg_init();
    1
}

unsafe extern "C" fn wii_shutdown() { hlwd_quiesce(); flipper_quiesce(); }

static wii_of_bus: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"nintendo,hollywood\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe extern "C" fn wii_device_probe() -> i32 {
    of_platform_populate(core::ptr::null_mut(), wii_of_bus.as_ptr(), core::ptr::null_mut(), core::ptr::null_mut());
    0
}

// machine_device_initcall(wii, wii_device_probe);
// define_machine(wii) {
//     .name = "wii", .compatible = "nintendo,wii", .probe = wii_probe,
//     .setup_arch = wii_setup_arch, .restart = wii_restart, .halt = wii_halt,
//     .init_IRQ = wii_pic_probe, .get_irq = flipper_pic_get_irq,
//     .progress = udbg_progress, .machine_shutdown = wii_shutdown,
// };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
