// SPDX-License-Identifier: GPL-2.0
/*
 * FPGA Freeze Bridge Controller
 *
 *  Copyright (C) 2016 Altera Corporation. All rights reserved.
 */

const FREEZE_CSR_STATUS_OFFSET: usize = 0;
const FREEZE_CSR_CTRL_OFFSET: usize = 4;
const FREEZE_CSR_ILLEGAL_REQ_OFFSET: usize = 8;
const FREEZE_CSR_REG_VERSION: usize = 12;

const FREEZE_CSR_SUPPORTED_VERSION: u32 = 2;
const FREEZE_CSR_OFFICIAL_VERSION: u32 = 0xad000003;

const FREEZE_CSR_STATUS_FREEZE_REQ_DONE: u32 = 1 << 0;
const FREEZE_CSR_STATUS_UNFREEZE_REQ_DONE: u32 = 1 << 1;

const FREEZE_CSR_CTRL_FREEZE_REQ: u32 = 1 << 0;
const FREEZE_CSR_CTRL_RESET_REQ: u32 = 1 << 1;
const FREEZE_CSR_CTRL_UNFREEZE_REQ: u32 = 1 << 2;

const FREEZE_BRIDGE_NAME: *const u8 = b"freeze\0".as_ptr();

#[repr(C)]
struct altera_freeze_br_data {
    dev: *mut device,
    base_addr: *mut core::ffi::c_void,
    enable: bool,
}

#[repr(C)]
struct device;
#[repr(C)]
struct device_node;
#[repr(C)]
struct platform_device {
    dev: device,
}
#[repr(C)]
struct fpga_image_info {
    enable_timeout_us: u32,
    disable_timeout_us: u32,
}
#[repr(C)]
struct fpga_bridge {
    priv_: *mut altera_freeze_br_data,
    info: *mut fpga_image_info,
}
#[repr(C)]
struct fpga_bridge_ops {
    enable_set: Option<unsafe extern "C" fn(*mut fpga_bridge, bool) -> i32>,
    enable_show: Option<unsafe extern "C" fn(*mut fpga_bridge) -> i32>,
}
#[repr(C)]
struct of_device_id {
    compatible: *const u8,
}

extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn udelay(usecs: u32);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: u32,
    ) -> *mut core::ffi::c_void;
    fn devm_kzalloc(
        dev: *mut device,
        size: usize,
        flags: u32,
    ) -> *mut core::ffi::c_void;
    fn fpga_bridge_register(
        dev: *mut device,
        name: *const u8,
        ops: *const fpga_bridge_ops,
        priv_: *mut altera_freeze_br_data,
    ) -> *mut fpga_bridge;
    fn fpga_bridge_unregister(bridge: *mut fpga_bridge);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut fpga_bridge);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut fpga_bridge;
}

const ETIMEDOUT: i32 = 110;
const EINVAL: i32 = 22;
const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;
const GFP_KERNEL: u32 = 0x00d0;

unsafe fn altera_freeze_br_req_ack(
    priv_: *mut altera_freeze_br_data,
    mut timeout: u32,
    req_ack: u32,
) -> i32 {
    let dev = (*priv_).dev;
    let csr_illegal_req_addr = (*priv_).base_addr.add(FREEZE_CSR_ILLEGAL_REQ_OFFSET);
    let mut ret = -ETIMEDOUT;

    loop {
        let mut illegal = readl(csr_illegal_req_addr);
        if illegal != 0 {
            dev_err(dev, b"illegal request detected 0x%x\0".as_ptr(), illegal);
            writel(1, csr_illegal_req_addr);
            illegal = readl(csr_illegal_req_addr);
            if illegal != 0 {
                dev_err(dev, b"illegal request not cleared 0x%x\0".as_ptr(), illegal);
            }
            ret = -EINVAL;
            break;
        }

        let mut status = readl((*priv_).base_addr.add(FREEZE_CSR_STATUS_OFFSET));
        dev_dbg(dev, b"%s %x %x\n\0".as_ptr(), b"altera_freeze_br_req_ack\0".as_ptr(), status, req_ack);
        status &= req_ack;
        if status != 0 {
            let ctrl = readl((*priv_).base_addr.add(FREEZE_CSR_CTRL_OFFSET));
            dev_dbg(dev, b"%s request %x acknowledged %x %x\n\0".as_ptr(), b"altera_freeze_br_req_ack\0".as_ptr(), req_ack, status, ctrl);
            ret = 0;
            break;
        }

        udelay(1);
        let old_timeout = timeout;
        timeout = timeout.wrapping_sub(1);
        if old_timeout == 0 {
            break;
        }
    }

    if ret == -ETIMEDOUT {
        dev_err(dev, b"%s timeout waiting for 0x%x\n\0".as_ptr(), b"altera_freeze_br_req_ack\0".as_ptr(), req_ack);
    }
    ret
}

unsafe fn altera_freeze_br_do_freeze(priv_: *mut altera_freeze_br_data, timeout: u32) -> i32 {
    let dev = (*priv_).dev;
    let csr_ctrl_addr = (*priv_).base_addr.add(FREEZE_CSR_CTRL_OFFSET);
    let status = readl((*priv_).base_addr.add(FREEZE_CSR_STATUS_OFFSET));
    dev_dbg(dev, b"%s %d %d\n\0".as_ptr(), b"altera_freeze_br_do_freeze\0".as_ptr(), status, readl(csr_ctrl_addr));

    if status & FREEZE_CSR_STATUS_FREEZE_REQ_DONE != 0 {
        dev_dbg(dev, b"%s bridge already disabled %d\n\0".as_ptr(), b"altera_freeze_br_do_freeze\0".as_ptr(), status);
        return 0;
    } else if status & FREEZE_CSR_STATUS_UNFREEZE_REQ_DONE == 0 {
        dev_err(dev, b"%s bridge not enabled %d\n\0".as_ptr(), b"altera_freeze_br_do_freeze\0".as_ptr(), status);
        return -EINVAL;
    }

    writel(FREEZE_CSR_CTRL_FREEZE_REQ, csr_ctrl_addr);
    let ret = altera_freeze_br_req_ack(priv_, timeout, FREEZE_CSR_STATUS_FREEZE_REQ_DONE);
    if ret != 0 { writel(0, csr_ctrl_addr); } else { writel(FREEZE_CSR_CTRL_RESET_REQ, csr_ctrl_addr); }
    ret
}

unsafe fn altera_freeze_br_do_unfreeze(priv_: *mut altera_freeze_br_data, timeout: u32) -> i32 {
    let dev = (*priv_).dev;
    let csr_ctrl_addr = (*priv_).base_addr.add(FREEZE_CSR_CTRL_OFFSET);
    writel(0, csr_ctrl_addr);
    let status = readl((*priv_).base_addr.add(FREEZE_CSR_STATUS_OFFSET));
    dev_dbg(dev, b"%s %d %d\n\0".as_ptr(), b"altera_freeze_br_do_unfreeze\0".as_ptr(), status, readl(csr_ctrl_addr));
    if status & FREEZE_CSR_STATUS_UNFREEZE_REQ_DONE != 0 {
        dev_dbg(dev, b"%s bridge already enabled %d\n\0".as_ptr(), b"altera_freeze_br_do_unfreeze\0".as_ptr(), status);
        return 0;
    } else if status & FREEZE_CSR_STATUS_FREEZE_REQ_DONE == 0 {
        dev_err(dev, b"%s bridge not frozen %d\0".as_ptr(), b"altera_freeze_br_do_unfreeze\0".as_ptr(), status);
        return -EINVAL;
    }
    writel(FREEZE_CSR_CTRL_UNFREEZE_REQ, csr_ctrl_addr);
    let ret = altera_freeze_br_req_ack(priv_, timeout, FREEZE_CSR_STATUS_UNFREEZE_REQ_DONE);
    let status = readl((*priv_).base_addr.add(FREEZE_CSR_STATUS_OFFSET));
    dev_dbg(dev, b"%s %d %d\n\0".as_ptr(), b"altera_freeze_br_do_unfreeze\0".as_ptr(), status, readl(csr_ctrl_addr));
    writel(0, csr_ctrl_addr);
    ret
}

unsafe extern "C" fn altera_freeze_br_enable_set(bridge: *mut fpga_bridge, enable: bool) -> i32 {
    let priv_ = (*bridge).priv_;
    let info = (*bridge).info;
    let mut timeout = 0;
    let ret = if enable {
        if !info.is_null() { timeout = (*info).enable_timeout_us; }
        altera_freeze_br_do_unfreeze(priv_, timeout)
    } else {
        if !info.is_null() { timeout = (*info).disable_timeout_us; }
        altera_freeze_br_do_freeze(priv_, timeout)
    };
    if ret == 0 { (*priv_).enable = enable; }
    ret
}

unsafe extern "C" fn altera_freeze_br_enable_show(bridge: *mut fpga_bridge) -> i32 {
    (*(*bridge).priv_).enable as i32
}

static altera_freeze_br_br_ops: fpga_bridge_ops = fpga_bridge_ops {
    enable_set: Some(altera_freeze_br_enable_set),
    enable_show: Some(altera_freeze_br_enable_show),
};

static altera_freeze_br_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"altr,freeze-bridge-controller\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

unsafe extern "C" fn altera_freeze_br_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let np = core::ptr::null_mut::<device_node>();
    if np.is_null() { return -ENODEV; }
    let base_addr = devm_platform_ioremap_resource(pdev, 0);
    if base_addr.is_null() { return -EINVAL; }
    let revision = readl(base_addr.add(FREEZE_CSR_REG_VERSION));
    if revision != FREEZE_CSR_SUPPORTED_VERSION && revision != FREEZE_CSR_OFFICIAL_VERSION {
        dev_err(dev, b"%s unexpected revision 0x%x != 0x%x != 0x%x\n\0".as_ptr(), b"altera_freeze_br_probe\0".as_ptr(), revision, FREEZE_CSR_SUPPORTED_VERSION, FREEZE_CSR_OFFICIAL_VERSION);
        return -EINVAL;
    }
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<altera_freeze_br_data>(), GFP_KERNEL) as *mut altera_freeze_br_data;
    if priv_.is_null() { return -ENOMEM; }
    (*priv_).dev = dev;
    let status = readl(base_addr.add(FREEZE_CSR_STATUS_OFFSET));
    if status & FREEZE_CSR_STATUS_UNFREEZE_REQ_DONE != 0 { (*priv_).enable = true; }
    (*priv_).base_addr = base_addr;
    let br = fpga_bridge_register(dev, FREEZE_BRIDGE_NAME, &altera_freeze_br_br_ops, priv_);
    if br.is_null() { return -EINVAL; }
    platform_set_drvdata(pdev, br);
    0
}

unsafe extern "C" fn altera_freeze_br_remove(pdev: *mut platform_device) {
    fpga_bridge_unregister(platform_get_drvdata(pdev));
}

// module_platform_driver(altera_freeze_br_driver);
// MODULE_DESCRIPTION("Altera Freeze Bridge");
// MODULE_AUTHOR("Alan Tull <atull@opensource.altera.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
