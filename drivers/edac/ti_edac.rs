// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017 Texas Instruments Incorporated - https://www.ti.com/
 *
 * Texas Instruments DDR3 ECC error correction and detection driver
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms and conditions of the GNU General Public License,
 * version 2, as published by the Free Software Foundation.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const EMIF_SDRAM_CONFIG: u16 = 0x008;
const EMIF_IRQ_STATUS: u16 = 0x0ac;
const EMIF_IRQ_ENABLE_SET: u16 = 0x0b4;
const EMIF_ECC_CTRL: u16 = 0x110;
const EMIF_1B_ECC_ERR_CNT: u16 = 0x130;
const EMIF_1B_ECC_ERR_THRSH: u16 = 0x134;
const EMIF_1B_ECC_ERR_ADDR_LOG: u16 = 0x13c;
const EMIF_2B_ECC_ERR_ADDR_LOG: u16 = 0x140;

const SDRAM_TYPE_SHIFT: u32 = 29;
const SDRAM_TYPE_MASK: u32 = genmask(31, 29);
const SDRAM_TYPE_DDR3: u32 = 3 << SDRAM_TYPE_SHIFT;
const SDRAM_TYPE_DDR2: u32 = 2 << SDRAM_TYPE_SHIFT;
const SDRAM_NARROW_MODE_MASK: u32 = genmask(15, 14);
const SDRAM_K2_NARROW_MODE_SHIFT: u32 = 12;
const SDRAM_K2_NARROW_MODE_MASK: u32 = genmask(13, 12);
const SDRAM_ROWSIZE_SHIFT: u32 = 7;
const SDRAM_ROWSIZE_MASK: u32 = genmask(9, 7);
const SDRAM_IBANK_SHIFT: u32 = 4;
const SDRAM_IBANK_MASK: u32 = genmask(6, 4);
const SDRAM_K2_IBANK_SHIFT: u32 = 5;
const SDRAM_K2_IBANK_MASK: u32 = genmask(6, 5);
const SDRAM_K2_EBANK_SHIFT: u32 = 3;
const SDRAM_K2_EBANK_MASK: u32 = bit(SDRAM_K2_EBANK_SHIFT);
const SDRAM_PAGESIZE_SHIFT: u32 = 0;
const SDRAM_PAGESIZE_MASK: u32 = genmask(2, 0);
const SDRAM_K2_PAGESIZE_SHIFT: u32 = 0;
const SDRAM_K2_PAGESIZE_MASK: u32 = genmask(1, 0);
const EMIF_1B_ECC_ERR_THRSH_SHIFT: u32 = 24;
const EMIF_1B_ECC_ERR: u32 = bit(5);
const EMIF_2B_ECC_ERR: u32 = bit(4);
const EMIF_WR_ECC_ERR: u32 = bit(3);
const EMIF_SYS_ERR: u32 = bit(0);
const ECC_ENABLED: u32 = bit(31) | bit(28);
const EDAC_MOD_NAME: &str = "ti-emif-edac";

enum { EMIF_TYPE_DRA7, EMIF_TYPE_K2 }

#[repr(C)]
struct ti_edac {
    reg: *mut core::ffi::c_void,
}

unsafe fn ti_edac_readl(edac: *mut ti_edac, offset: u16) -> u32 {
    readl_relaxed((*edac).reg.add(offset as usize))
}

unsafe fn ti_edac_writel(edac: *mut ti_edac, val: u32, offset: u16) {
    writel_relaxed(val, (*edac).reg.add(offset as usize));
}

unsafe fn ti_edac_isr(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let mci = data as *mut mem_ctl_info;
    let edac = (*mci).pvt_info as *mut ti_edac;
    let irq_status = ti_edac_readl(edac, EMIF_IRQ_STATUS);
    if irq_status & EMIF_1B_ECC_ERR != 0 {
        let err_addr = ti_edac_readl(edac, EMIF_1B_ECC_ERR_ADDR_LOG);
        let err_count = ti_edac_readl(edac, EMIF_1B_ECC_ERR_CNT);
        ti_edac_writel(edac, err_count, EMIF_1B_ECC_ERR_CNT);
        edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, err_count,
            err_addr >> PAGE_SHIFT, err_addr & !PAGE_MASK, -1, 0, 0, 0,
            (*mci).ctl_name, "1B");
    }
    if irq_status & EMIF_2B_ECC_ERR != 0 {
        let err_addr = ti_edac_readl(edac, EMIF_2B_ECC_ERR_ADDR_LOG);
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1,
            err_addr >> PAGE_SHIFT, err_addr & !PAGE_MASK, -1, 0, 0, 0,
            (*mci).ctl_name, "2B");
    }
    if irq_status & EMIF_WR_ECC_ERR != 0 {
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1,
            0, 0, -1, 0, 0, 0, (*mci).ctl_name, "WR");
    }
    ti_edac_writel(edac, irq_status, EMIF_IRQ_STATUS);
    IRQ_HANDLED
}

unsafe fn ti_edac_setup_dimm(mci: *mut mem_ctl_info, type_: u32) {
    let dimm = edac_get_dimm(mci, 0, 0, 0);
    let edac = (*mci).pvt_info as *mut ti_edac;
    let mut val = ti_edac_readl(edac, EMIF_SDRAM_CONFIG);
    let mut bits: i32;
    if type_ == EMIF_TYPE_DRA7 {
        bits = ((val & SDRAM_PAGESIZE_MASK) >> SDRAM_PAGESIZE_SHIFT) as i32 + 8;
        bits += ((val & SDRAM_ROWSIZE_MASK) >> SDRAM_ROWSIZE_SHIFT) as i32 + 9;
        bits += ((val & SDRAM_IBANK_MASK) >> SDRAM_IBANK_SHIFT) as i32;
        if val & SDRAM_NARROW_MODE_MASK != 0 { bits += 1; (*dimm).dtype = DEV_X16; }
        else { bits += 2; (*dimm).dtype = DEV_X32; }
    } else {
        bits = 16;
        bits += ((val & SDRAM_K2_PAGESIZE_MASK) >> SDRAM_K2_PAGESIZE_SHIFT) as i32 + 8;
        bits += ((val & SDRAM_K2_IBANK_MASK) >> SDRAM_K2_IBANK_SHIFT) as i32;
        bits += ((val & SDRAM_K2_EBANK_MASK) >> SDRAM_K2_EBANK_SHIFT) as i32;
        val = (val & SDRAM_K2_NARROW_MODE_MASK) >> SDRAM_K2_NARROW_MODE_SHIFT;
        match val { 0 => { bits += 3; (*dimm).dtype = DEV_X64; }, 1 => { bits += 2; (*dimm).dtype = DEV_X32; }, 2 => { bits += 1; (*dimm).dtype = DEV_X16; }, _ => {} }
    }
    let memsize = 1u32 << bits;
    (*dimm).nr_pages = memsize >> PAGE_SHIFT;
    (*dimm).grain = 4;
    (*dimm).mtype = if val & SDRAM_TYPE_MASK == SDRAM_TYPE_DDR2 { MEM_DDR2 } else { MEM_DDR3 };
    val = ti_edac_readl(edac, EMIF_ECC_CTRL);
    (*dimm).edac_mode = if val & ECC_ENABLED != 0 { EDAC_SECDED } else { EDAC_NONE };
}

static ti_edac_of_match: [of_device_id; 3] = [
    of_device_id { compatible: "ti,emif-keystone", data: EMIF_TYPE_K2 as *mut _ },
    of_device_id { compatible: "ti,emif-dra7xx", data: EMIF_TYPE_DRA7 as *mut _ },
    of_device_id::default(),
];

unsafe fn _emif_get_id(node: *mut device_node) -> i32 {
    let mut my_id = 0;
    let addrp = of_get_address(node, 0, core::ptr::null_mut(), core::ptr::null_mut());
    let my_addr = of_translate_address(node, addrp) as u32;
    for_each_matching_node!(np, ti_edac_of_match, {
        if np == node { continue; }
        let addrp = of_get_address(np, 0, core::ptr::null_mut(), core::ptr::null_mut());
        let addr = of_translate_address(np, addrp) as u32;
        edac_printk(KERN_INFO, EDAC_MOD_NAME, "addr=%x, my_addr=%x\n", addr, my_addr);
        if addr < my_addr { my_id += 1; }
    });
    my_id
}

unsafe fn ti_edac_probe(pdev: *mut platform_device) -> i32 {
    let mut error_irq = 0;
    let mut ret = -ENODEV;
    let dev = &mut (*pdev).dev;
    let id = of_match_device(ti_edac_of_match.as_ptr(), dev);
    if id.is_null() { return -ENODEV; }
    let res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    let reg = devm_ioremap_resource(dev, res);
    if IS_ERR(reg) { return PTR_ERR(reg); }
    let mut layers = [edac_mc_layer { type_: EDAC_MC_LAYER_ALL_MEM, size: 1 }];
    let emif_id = _emif_get_id((*pdev).dev.of_node);
    if emif_id < 0 { return -EINVAL; }
    let mci = edac_mc_alloc(emif_id, 1, layers.as_mut_ptr(), core::mem::size_of::<ti_edac>());
    if mci.is_null() { return -ENOMEM; }
    (*mci).pdev = dev;
    let edac = (*mci).pvt_info as *mut ti_edac;
    (*edac).reg = reg;
    platform_set_drvdata(pdev, mci as *mut _);
    (*mci).mtype_cap = MEM_FLAG_DDR3 | MEM_FLAG_DDR2;
    (*mci).edac_ctl_cap = EDAC_FLAG_SECDED | EDAC_FLAG_NONE;
    (*mci).mod_name = EDAC_MOD_NAME;
    (*mci).ctl_name = (*id).compatible;
    (*mci).dev_name = dev_name(dev);
    ti_edac_setup_dimm(mci, (*id).data as u32);
    error_irq = platform_get_irq(pdev, 0);
    if error_irq < 0 { ret = error_irq; edac_mc_free(mci); return ret; }
    ret = devm_request_irq(dev, error_irq, ti_edac_isr, 0, "emif-edac-irq", mci as *mut _);
    if ret != 0 { edac_printk(KERN_ERR, EDAC_MOD_NAME, "request_irq fail for EMIF EDAC irq\n"); edac_mc_free(mci); return ret; }
    ret = edac_mc_add_mc(mci);
    if ret != 0 { edac_printk(KERN_ERR, EDAC_MOD_NAME, "Failed to register mci: %d.\n", ret); edac_mc_free(mci); return ret; }
    ti_edac_writel(edac, 1 << EMIF_1B_ECC_ERR_THRSH_SHIFT, EMIF_1B_ECC_ERR_THRSH);
    ti_edac_writel(edac, EMIF_1B_ECC_ERR | EMIF_2B_ECC_ERR | EMIF_WR_ECC_ERR, EMIF_IRQ_ENABLE_SET);
    0
}

unsafe fn ti_edac_remove(pdev: *mut platform_device) {
    let mci = platform_get_drvdata(pdev) as *mut mem_ctl_info;
    edac_mc_del_mc(&mut (*pdev).dev);
    edac_mc_free(mci);
}

static mut ti_edac_driver: platform_driver = platform_driver {
    probe: Some(ti_edac_probe), remove: Some(ti_edac_remove),
    driver: driver { name: EDAC_MOD_NAME, of_match_table: ti_edac_of_match.as_ptr() },
};

module_platform_driver!(ti_edac_driver);
const MODULE_AUTHOR: &str = "Texas Instruments Inc.";
const MODULE_DESCRIPTION: &str = "EDAC Driver for Texas Instruments DDR3 MC";
const MODULE_LICENSE: &str = "GPL v2";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
