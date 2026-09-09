// SPDX-License-Identifier: GPL-2.0-only
/*
 * Board-level suspend/resume support.
 *
 * Copyright (C) 2014-2015 Marvell
 *
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 */

// Linux dependencies supplied by the surrounding kernel translation.

const ARMADA_PIC_NR_GPIOS: usize = 3;

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GpioDesc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct OfPhandleArgs {
    pub np: *mut DeviceNode,
    pub args: [u32; 16],
}

static mut GPIO_CTRL: *mut core::ffi::c_void = core::ptr::null_mut();
static mut PIC_GPIOS: [*mut GpioDesc; ARMADA_PIC_NR_GPIOS] = [core::ptr::null_mut(); ARMADA_PIC_NR_GPIOS];
static mut PIC_RAW_GPIOS: [i32; ARMADA_PIC_NR_GPIOS] = [0; ARMADA_PIC_NR_GPIOS];

extern "C" {
    fn of_machine_is_compatible(compat: *const core::ffi::c_char) -> bool;
    fn of_find_node_by_name(from: *mut DeviceNode, name: *const core::ffi::c_char) -> *mut DeviceNode;
    fn kasprintf(flags: u32, fmt: *const core::ffi::c_char, ...) -> *mut core::ffi::c_char;
    fn fwnode_gpiod_get_index(fwnode: *mut core::ffi::c_void, con_id: *const core::ffi::c_char, index: usize, flags: u32, label: *const core::ffi::c_char) -> *mut GpioDesc;
    fn ptr_err_or_zero<T>(ptr: *mut T) -> i32;
    fn kfree(ptr: *mut core::ffi::c_char);
    fn of_fwnode_handle(node: *mut DeviceNode) -> *mut core::ffi::c_void;
    fn of_parse_phandle_with_fixed_args(node: *mut DeviceNode, list: *const core::ffi::c_char, cells: usize, index: usize, args: *mut OfPhandleArgs) -> i32;
    fn gpiod_put(desc: *mut GpioDesc);
    fn of_node_put(node: *mut DeviceNode);
    fn of_iomap(node: *mut DeviceNode, index: usize) -> *mut core::ffi::c_void;
    fn mvebu_pm_suspend_init(enter: unsafe extern "C" fn(*mut core::ffi::c_void, u32));
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn mdelay(milliseconds: u32);
}

unsafe extern "C" fn mvebu_armada_pm_enter(sdram_reg: *mut core::ffi::c_void, mut srcmd: u32) {
    let mut reg: u32;
    let mut ackcmd: u32;
    let mut i: usize;

    /* Put 001 as value on the GPIOs */
    reg = readl(GPIO_CTRL);
    for i in 0..ARMADA_PIC_NR_GPIOS {
        reg &= !(1u32.wrapping_shl(PIC_RAW_GPIOS[i] as u32));
    }
    reg |= 1u32.wrapping_shl(PIC_RAW_GPIOS[0] as u32);
    writel(reg, GPIO_CTRL);

    /* Prepare writing 111 to the GPIOs */
    ackcmd = readl(GPIO_CTRL);
    for i in 0..ARMADA_PIC_NR_GPIOS {
        ackcmd |= 1u32.wrapping_shl(PIC_RAW_GPIOS[i] as u32);
    }

    srcmd = srcmd.to_le();
    ackcmd = ackcmd.to_le();

    /*
     * Wait a while, the PIC needs quite a bit of time between the
     * two GPIO commands.
     */
    mdelay(3000);

    // ARM inline assembly from the original source: enter SDRAM self-refresh,
    // wait 100 cycles, acknowledge the GPIO command, then trap the processor.
    core::arch::asm!(
        ".balign 32",
        "str {srcmd}, [{sdram_reg}]",
        "mov r1, #50",
        "1: subs r1, r1, #1",
        "bne 1b",
        "str {ackcmd}, [{gpio_ctrl}]",
        "b .",
        srcmd = in(reg) srcmd,
        sdram_reg = in(reg) sdram_reg,
        ackcmd = in(reg) ackcmd,
        gpio_ctrl = in(reg) GPIO_CTRL,
        out("r1") _,
        options(noreturn)
    );
}

unsafe extern "C" fn mvebu_armada_pm_init() -> i32 {
    let mut np: *mut DeviceNode;
    let mut gpio_ctrl_np: *mut DeviceNode = core::ptr::null_mut();
    let mut ret: i32 = 0;
    let mut i: usize;

    if !of_machine_is_compatible(b"marvell,axp-gp\0".as_ptr() as *const _) {
        return -19;
    }

    np = of_find_node_by_name(core::ptr::null_mut(), b"pm_pic\0".as_ptr() as *const _);
    if np.is_null() {
        return -19;
    }

    for i in 0..ARMADA_PIC_NR_GPIOS {
        let name = kasprintf(0, b"pic-pin%d\0".as_ptr() as *const _, i as i32);
        if name.is_null() {
            ret = -12;
            break;
        }

        PIC_GPIOS[i] = fwnode_gpiod_get_index(of_fwnode_handle(np), b"ctrl\0".as_ptr() as *const _, i, 1, name);
        ret = ptr_err_or_zero(PIC_GPIOS[i]);
        if ret != 0 {
            kfree(name);
            break;
        }

        let mut args = OfPhandleArgs { np: core::ptr::null_mut(), args: [0; 16] };
        ret = of_parse_phandle_with_fixed_args(np, b"ctrl-gpios\0".as_ptr() as *const _, 2, i, &mut args);
        if ret < 0 {
            gpiod_put(PIC_GPIOS[i]);
            kfree(name);
            break;
        }

        if !gpio_ctrl_np.is_null() {
            of_node_put(gpio_ctrl_np);
        }
        gpio_ctrl_np = args.np;
        PIC_RAW_GPIOS[i] = args.args[0] as i32;
    }

    if ret == 0 {
        GPIO_CTRL = of_iomap(gpio_ctrl_np, 0);
        if GPIO_CTRL.is_null() {
            ret = -12;
        } else {
            mvebu_pm_suspend_init(mvebu_armada_pm_enter);
        }
    }

    of_node_put(np);
    of_node_put(gpio_ctrl_np);
    ret
}

/*
 * Registering the mvebu_board_pm_enter callback must be done before
 * the platform_suspend_ops will be registered. In the same time we
 * also need to have the gpio devices registered. That's why we use a
 * device_initcall_sync which is called after all the device_initcall
 * (used by the gpio device) but before the late_initcall (used to
 * register the platform_suspend_ops)
 */
// device_initcall_sync(mvebu_armada_pm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
