// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2013 John Crispin <john@phrozen.org>
 */

// Linux kernel dependencies supplied by other translation units.

const REG_ILL_ACC_ADDR: u32 = 0x10;
const REG_ILL_ACC_TYPE: u32 = 0x14;

const ILL_INT_STATUS: u32 = 1u32 << 31;
const ILL_ACC_WRITE: u32 = 1u32 << 30;
const ILL_ACC_LEN_M: u32 = 0xff;
const ILL_ACC_OFF_M: u32 = 0xf;
const ILL_ACC_OFF_S: u32 = 16;
const ILL_ACC_ID_M: u32 = 0x7;
const ILL_ACC_ID_S: u32 = 8;

const DRV_NAME: &str = "ill_acc";

static ILL_ACC_IDS: [&str; 8] = [
    "cpu", "dma", "ppe", "pdma rx", "pdma tx", "pci/e", "wmac", "usb",
];

unsafe extern "C" {
    fn rt_memc_r32(reg: u32) -> u32;
    fn rt_memc_w32(value: u32, reg: u32);
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn of_machine_is_compatible(compat: *const core::ffi::c_char) -> bool;
    fn of_find_compatible_node(
        from: *mut device_node,
        ty: *const core::ffi::c_char,
        compat: *const core::ffi::c_char,
    ) -> *mut device_node;
    fn of_find_device_by_node(np: *mut device_node) -> *mut platform_device;
    fn irq_of_parse_and_map(np: *mut device_node, index: u32) -> i32;
    fn of_node_put(np: *mut device_node);
    fn put_device(dev: *mut device);
    fn request_irq(
        irq: i32,
        handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
        flags: u32,
        name: *const core::ffi::c_char,
        dev: *mut core::ffi::c_void,
    ) -> i32;
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

type irqreturn_t = i32;
const IRQ_HANDLED: irqreturn_t = 1;

unsafe extern "C" fn ill_acc_irq_handler(_irq: i32, priv_: *mut core::ffi::c_void) -> irqreturn_t {
    let dev = priv_ as *mut device;
    let addr = rt_memc_r32(REG_ILL_ACC_ADDR);
    let type_ = rt_memc_r32(REG_ILL_ACC_TYPE);

    let access = if (type_ & ILL_ACC_WRITE) != 0 { "write" } else { "read" };
    let id = ILL_ACC_IDS[((type_ >> ILL_ACC_ID_S) & ILL_ACC_ID_M) as usize];
    let offset = (type_ >> ILL_ACC_OFF_S) & ILL_ACC_OFF_M;
    let len = type_ & ILL_ACC_LEN_M;

    // Corresponds to dev_err(dev, "illegal %s access from %s - addr:0x%08x offset:%d len:%d\n", ...).
    let _ = (dev, access, id, addr, offset, len);

    rt_memc_w32(ILL_INT_STATUS, REG_ILL_ACC_TYPE);

    IRQ_HANDLED
}

unsafe extern "C" fn ill_acc_of_setup() -> i32 {
    let mut pdev: *mut platform_device;
    let np: *mut device_node;
    let irq: i32;

    /* somehow this driver breaks on RT5350 */
    if of_machine_is_compatible(c"ralink,rt5350-soc".as_ptr()) {
        return -22;
    }

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"ralink,rt3050-memc".as_ptr());
    if np.is_null() {
        return -22;
    }

    pdev = of_find_device_by_node(np);
    if pdev.is_null() {
        pr_err(c"%pOFn: failed to lookup pdev\n".as_ptr());
        of_node_put(np);
        return -22;
    }

    irq = irq_of_parse_and_map(np, 0);
    of_node_put(np);
    if irq == 0 {
        // dev_err(&pdev->dev, "failed to get irq\n");
        put_device(&mut (*pdev).dev);
        return -22;
    }

    if request_irq(irq, ill_acc_irq_handler, 0, c"ill_acc".as_ptr(), &mut (*pdev).dev as *mut device as *mut core::ffi::c_void) != 0 {
        // dev_err(&pdev->dev, "failed to request irq\n");
        put_device(&mut (*pdev).dev);
        return -22;
    }

    rt_memc_w32(ILL_INT_STATUS, REG_ILL_ACC_TYPE);

    // dev_info(&pdev->dev, "irq registered\n");
    0
}

// arch_initcall(ill_acc_of_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
