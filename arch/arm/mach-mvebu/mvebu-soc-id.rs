// SPDX-License-Identifier: GPL-2.0-only
/*
 * ID and revision information for mvebu SoCs
 *
 * Copyright (C) 2014 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 *
 * All the mvebu SoCs have information related to their variant and
 * revision that can be read from the PCI control register. This is
 * done before the PCI initialization to avoid any conflict. Once the
 * ID and revision are retrieved, the mapping is freed.
 */

// C dependencies supplied by the surrounding kernel translation unit.

const PCIE_DEV_ID_OFF: usize = 0x0;
const PCIE_DEV_REV_OFF: usize = 0x8;

const SOC_ID_MASK: u32 = 0xFFFF0000;
const SOC_REV_MASK: u32 = 0xFF;

static mut soc_dev_id: u32 = 0;
static mut soc_rev: u32 = 0;
static mut is_id_valid: bool = false;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_device_attribute {
    pub family: *mut u8,
    pub machine: *mut u8,
    pub firmware: *mut u8,
    pub revision: *mut u8,
    pub soc_id: *mut u8,
}

#[repr(C)]
pub struct soc_device {
    _private: [u8; 0],
}

extern "C" {
    fn of_find_matching_node(from: *mut device_node, table: *const u8) -> *mut device_node;
    fn of_get_next_child(node: *mut device_node, prev: *mut device_node) -> *mut device_node;
    fn of_clk_get_by_name(node: *mut device_node, name: *const u8) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_put(clk: *mut clk);
    fn of_iomap(node: *mut device_node, index: i32) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn of_device_is_available(node: *mut device_node) -> bool;
    fn of_node_put(node: *mut device_node);
    fn mvebu_system_controller_get_soc_id(dev: *mut u32, rev: *mut u32) -> i32;
    fn kzalloc_obj<T>() -> *mut T;
    fn kasprintf(flags: u32, fmt: *const u8, ...) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn soc_device_register(attr: *mut soc_device_attribute) -> *mut soc_device;
    fn is_err<T>(ptr: *mut T) -> bool;
    fn readl(addr: *mut u8) -> u32;
}

const GFP_KERNEL: u32 = 0;
const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;

#[no_mangle]
pub unsafe extern "C" fn mvebu_get_soc_id(dev: *mut u32, rev: *mut u32) -> i32 {
    if is_id_valid {
        *dev = soc_dev_id;
        *rev = soc_rev;
        0
    } else {
        -ENODEV
    }
}

unsafe fn get_soc_id_by_pci() -> i32 {
    let np = of_find_matching_node(core::ptr::null_mut(), core::ptr::null());
    if np.is_null() {
        return 0;
    }

    let child = of_get_next_child(np, core::ptr::null_mut());
    if child.is_null() {
        of_node_put(np);
        return -ENOMEM;
    }

    let clk = of_clk_get_by_name(child, core::ptr::null());
    if clk.is_null() {
        of_node_put(child);
        of_node_put(np);
        return -ENOMEM;
    }

    let mut ret = clk_prepare_enable(clk);
    if ret != 0 {
        of_node_put(child);
        of_node_put(np);
        return ret;
    }

    let pci_base = of_iomap(child, 0);
    if pci_base.is_null() {
        ret = -ENOMEM;
    } else {
        soc_dev_id = readl(pci_base.add(PCIE_DEV_ID_OFF)) >> 16;
        soc_rev = readl(pci_base.add(PCIE_DEV_REV_OFF)) & SOC_REV_MASK;
        is_id_valid = true;
        iounmap(pci_base);
    }

    // If the PCIe unit is enabled, retain its clock reference when PCI support is enabled.
    if !of_device_is_available(child) {
        clk_disable_unprepare(clk);
        clk_put(clk);
    }

    of_node_put(child);
    of_node_put(np);
    ret
}

unsafe fn mvebu_soc_id_init() -> i32 {
    /*
     * First try to get the ID and the revision by the system
     * register and use PCI registers only if it is not possible
     */
    if mvebu_system_controller_get_soc_id(&mut soc_dev_id, &mut soc_rev) == 0 {
        is_id_valid = true;
        return 0;
    }

    get_soc_id_by_pci()
}

// early_initcall(mvebu_soc_id_init);

unsafe fn mvebu_soc_device() -> i32 {
    // Also protects against running on non-mvebu systems.
    if !is_id_valid {
        return 0;
    }

    let soc_dev_attr = kzalloc_obj::<soc_device_attribute>();
    if soc_dev_attr.is_null() {
        return -ENOMEM;
    }

    (*soc_dev_attr).family = kasprintf(GFP_KERNEL, b"Marvell\0".as_ptr());
    (*soc_dev_attr).revision = kasprintf(GFP_KERNEL, b"%X\0".as_ptr(), soc_rev);
    (*soc_dev_attr).soc_id = kasprintf(GFP_KERNEL, b"%X\0".as_ptr(), soc_dev_id);

    let soc_dev = soc_device_register(soc_dev_attr);
    if is_err(soc_dev) {
        kfree((*soc_dev_attr).family);
        kfree((*soc_dev_attr).revision);
        kfree((*soc_dev_attr).soc_id);
        kfree(soc_dev_attr as *mut u8);
    }

    0
}

// postcore_initcall(mvebu_soc_device);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
