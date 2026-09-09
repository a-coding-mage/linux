// SPDX-License-Identifier: GPL-2.0-only
/*
 * Freescale Memory Controller kernel module
 *
 * Support Power-based SoCs including MPC85xx, MPC86xx, MPC83xx and
 * ARM-based Layerscape SoCs including LS2xxx and LS1021A. Originally
 * split out from mpc85xx_edac EDAC driver.
 *
 * Parts Copyrighted (c) 2013 by Freescale Semiconductor, Inc.
 *
 * Author: Dave Jiang <djiang@mvista.com>
 *
 * 2006-2007 (c) MontaVista Software, Inc.
 */

// C dependencies supplied by the surrounding kernel translation.

const EDAC_MOD_STR: &str = "fsl_ddr_edac";
static mut edac_mc_idx: i32 = 0;

#[inline]
unsafe fn ddr_reg_addr(pdata: *mut fsl_mc_pdata, off: u32) -> *mut core::ffi::c_void {
    if (*pdata).flag == TYPE_IMX9 && off >= FSL_MC_DATA_ERR_INJECT_HI && off <= FSL_MC_ERR_SBE {
        return (*pdata).inject_vbase.add((off - FSL_MC_DATA_ERR_INJECT_HI + IMX9_MC_DATA_ERR_INJECT_OFF) as usize);
    }
    if (*pdata).flag == TYPE_IMX9 && off >= IMX9_MC_ERR_EN {
        return (*pdata).inject_vbase.add((off - IMX9_MC_ERR_EN) as usize);
    }
    (*pdata).mc_vbase.add(off as usize)
}

#[inline]
unsafe fn ddr_in32(pdata: *mut fsl_mc_pdata, off: u32) -> u32 {
    let addr = ddr_reg_addr(pdata, off);
    if (*pdata).little_endian { ioread32(addr) } else { ioread32be(addr) }
}

#[inline]
unsafe fn ddr_out32(pdata: *mut fsl_mc_pdata, off: u32, value: u32) {
    let addr = ddr_reg_addr(pdata, off);
    if (*pdata).little_endian { iowrite32(value, addr); } else { iowrite32be(value, addr); }
}

#[cfg(CONFIG_EDAC_DEBUG)]
unsafe fn fsl_mc_inject_data_hi_show(dev: *mut device, _mattr: *mut device_attribute, data: *mut i8) -> isize {
    let mci = to_mci(dev); let pdata = (*mci).pvt_info;
    sprintf(data, "0x%08x", ddr_in32(pdata, FSL_MC_DATA_ERR_INJECT_HI))
}
#[cfg(CONFIG_EDAC_DEBUG)]
unsafe fn fsl_mc_inject_data_lo_show(dev: *mut device, _mattr: *mut device_attribute, data: *mut i8) -> isize {
    let mci = to_mci(dev); let pdata = (*mci).pvt_info;
    sprintf(data, "0x%08x", ddr_in32(pdata, FSL_MC_DATA_ERR_INJECT_LO))
}
#[cfg(CONFIG_EDAC_DEBUG)]
unsafe fn fsl_mc_inject_ctrl_show(dev: *mut device, _mattr: *mut device_attribute, data: *mut i8) -> isize {
    let mci = to_mci(dev); let pdata = (*mci).pvt_info;
    sprintf(data, "0x%08x", ddr_in32(pdata, FSL_MC_ECC_ERR_INJECT))
}
#[cfg(CONFIG_EDAC_DEBUG)]
unsafe fn inject_store(dev: *mut device, data: *const i8, count: usize, reg: u32) -> isize {
    let mci = to_mci(dev); let pdata = (*mci).pvt_info; let mut val: u64 = 0;
    if isdigit(*data) { let rc = kstrtoul(data, 0, &mut val); if rc != 0 { return rc as isize; } ddr_out32(pdata, reg, val as u32); return count as isize; }
    0
}
#[cfg(CONFIG_EDAC_DEBUG)]
unsafe fn fsl_mc_inject_data_hi_store(d: *mut device, _: *mut device_attribute, s: *const i8, n: usize) -> isize { inject_store(d, s, n, FSL_MC_DATA_ERR_INJECT_HI) }
#[cfg(CONFIG_EDAC_DEBUG)]
unsafe fn fsl_mc_inject_data_lo_store(d: *mut device, _: *mut device_attribute, s: *const i8, n: usize) -> isize { inject_store(d, s, n, FSL_MC_DATA_ERR_INJECT_LO) }
#[cfg(CONFIG_EDAC_DEBUG)]
unsafe fn fsl_mc_inject_ctrl_store(d: *mut device, _: *mut device_attribute, s: *const i8, n: usize) -> isize { inject_store(d, s, n, FSL_MC_ECC_ERR_INJECT) }

static mut ecc_table: [u32; 16] = [
    0xf00fe11e, 0xc33c0ff7, 0x00ff00ff, 0x00fff0ff,
    0x0f0f0f0f, 0x0f0fff00, 0x11113333, 0x7777000f,
    0x22224444, 0x8888222f, 0x44448888, 0xffff4441,
    0x8888ffff, 0x11118882, 0xffff1111, 0x22221114,
];

unsafe fn calculate_ecc(high: u32, low: u32) -> u8 {
    let mut ecc = 0u8;
    for i in 0..8 { let mh = ecc_table[i * 2]; let ml = ecc_table[i * 2 + 1]; let mut cnt = 0u32;
        for j in 0..32 { if ((mh >> j) & 1) != 0 { cnt ^= (high >> j) & 1; } if ((ml >> j) & 1) != 0 { cnt ^= (low >> j) & 1; } }
        ecc |= (cnt << i) as u8;
    } ecc
}
unsafe fn syndrome_from_bit(bit: u32) -> u8 { let mut syndrome = 0u8; let mut i = if bit < 32 { 1 } else { 0 }; while i < 16 { syndrome |= (((ecc_table[i] >> (bit % 32)) & 1) << (i / 2)) as u8; i += 2; } syndrome }
unsafe fn sbe_ecc_decode(h: u32, l: u32, e: u32, db: *mut i32, eb: *mut i32) {
    *db = -1; *eb = -1; let syndrome = calculate_ecc(h, l) ^ e as u8;
    for i in 0..64 { if syndrome == syndrome_from_bit(i) { *db = i as i32; return; } }
    for i in 0..8 { if ((syndrome >> i) & 1) != 0 { *eb = i; return; } }
}

unsafe fn fsl_mc_check(mci: *mut mem_ctl_info) {
    let pdata = (*mci).pvt_info; let mut err_detect = ddr_in32(pdata, FSL_MC_ERR_DETECT); if err_detect == 0 { return; }
    fsl_mc_printk(mci, KERN_ERR, "Err Detect Register: %#8.8x\n", err_detect);
    if (err_detect & (DDR_EDE_SBE | DDR_EDE_MBE)) == 0 { ddr_out32(pdata, FSL_MC_ERR_DETECT, err_detect); return; }
    let mut syndrome = ddr_in32(pdata, FSL_MC_CAPTURE_ECC); let bus_width = if (ddr_in32(pdata, FSL_MC_DDR_SDRAM_CFG) & DSC_DBW_MASK) != 0 { 32 } else { 64 };
    syndrome &= if bus_width == 64 { 0xff } else { 0xffff };
    let err_addr = ((ddr_in32(pdata, FSL_MC_CAPTURE_EXT_ADDRESS) as u64) << 32) | ddr_in32(pdata, FSL_MC_CAPTURE_ADDRESS) as u64; let pfn = err_addr >> PAGE_SHIFT;
    let mut row_index = 0; while row_index < (*mci).nr_csrows { let csrow = *(*mci).csrows.add(row_index as usize); if pfn >= (*csrow).first_page && pfn <= (*csrow).last_page { break; } row_index += 1; }
    let cap_high = ddr_in32(pdata, FSL_MC_CAPTURE_DATA_HI); let cap_low = ddr_in32(pdata, FSL_MC_CAPTURE_DATA_LO);
    if (err_detect & DDR_EDE_SBE) != 0 && bus_width == 64 { let mut cap = ((cap_high as u64) << 32) | cap_low as u64; let mut s = syndrome; let mut db = -1; let mut eb = -1; sbe_ecc_decode(cap_high, cap_low, syndrome, &mut db, &mut eb); if db >= 0 { fsl_mc_printk(mci, KERN_ERR, "Faulty Data bit: %d\n", db); cap ^= 1u64 << db; } if eb >= 0 { fsl_mc_printk(mci, KERN_ERR, "Faulty ECC bit: %d\n", eb); s ^= 1 << eb; } fsl_mc_printk(mci, KERN_ERR, "Expected Data / ECC:\t%#8.8x_%08x / %#2.2x\n", (cap >> 32) as u32, cap as u32, s); }
    fsl_mc_printk(mci, KERN_ERR, "Captured Data / ECC:\t%#8.8x_%08x / %#2.2x\n", cap_high, cap_low, syndrome); fsl_mc_printk(mci, KERN_ERR, "Err addr: %#8.8llx\n", err_addr); fsl_mc_printk(mci, KERN_ERR, "PFN: %#8.8x\n", pfn);
    if row_index == (*mci).nr_csrows { fsl_mc_printk(mci, KERN_ERR, "PFN out of range!\n"); }
    if (err_detect & DDR_EDE_SBE) != 0 { edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, pfn, err_addr & !PAGE_MASK, syndrome, row_index, 0, -1, (*mci).ctl_name, ""); }
    if (err_detect & DDR_EDE_MBE) != 0 { edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, pfn, err_addr & !PAGE_MASK, syndrome, row_index, 0, -1, (*mci).ctl_name, ""); }
    ddr_out32(pdata, FSL_MC_ERR_DETECT, err_detect);
}

unsafe fn fsl_mc_isr(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t { let mci = dev_id as *mut mem_ctl_info; if ddr_in32((*mci).pvt_info, FSL_MC_ERR_DETECT) == 0 { return IRQ_NONE; } fsl_mc_check(mci); IRQ_HANDLED }

unsafe fn fsl_ddr_init_csrows(mci: *mut mem_ctl_info) {
    let pdata = (*mci).pvt_info; let sdram_ctl = ddr_in32(pdata, FSL_MC_DDR_SDRAM_CFG); let sdtype = sdram_ctl & DSC_SDTYPE_MASK;
    let mtype = match (sdram_ctl & DSC_RD_EN != 0, sdtype) { (true, 0x02000000) => MEM_RDDR, (true, 0x03000000) => MEM_RDDR2, (true, 0x07000000) => MEM_RDDR3, (true, 0x05000000) => MEM_RDDR4, (false, 0x02000000) => MEM_DDR, (false, 0x03000000) => MEM_DDR2, (false, 0x07000000) => MEM_DDR3, (false, 0x05000000) => MEM_DDR4, (false, 0x04000000) => MEM_LPDDR4, _ => MEM_UNKNOWN };
    for index in 0..(*mci).nr_csrows { let csrow = *(*mci).csrows.add(index as usize); let dimm = (*(*csrow).channels).dimm; let b = ddr_in32(pdata, FSL_MC_CS_BNDS_0 + index as u32 * FSL_MC_CS_BNDS_OFS); let mut start = (b & 0xffff0000) >> 16; let mut end = b & 0xffff; if start == end { continue; } start <<= 24 - PAGE_SHIFT; end = (end << (24 - PAGE_SHIFT)) | ((1 << (24 - PAGE_SHIFT)) - 1); (*csrow).first_page = start as u64; (*csrow).last_page = end as u64; (*dimm).nr_pages = (end + 1 - start) as u64; (*dimm).grain = 8; (*dimm).mtype = mtype; (*dimm).dtype = if (*pdata).flag == TYPE_IMX9 { DEV_X16 } else if sdram_ctl & DSC_X32_EN != 0 { DEV_X32 } else { DEV_UNKNOWN }; (*dimm).edac_mode = EDAC_SECDED; }
}

pub unsafe fn fsl_mc_err_probe(op: *mut platform_device) -> i32 { let mut mci; let mut layers = [edac_mc_layer { r#type: EDAC_MC_LAYER_CHIP_SELECT, size: 4, is_virt_csrow: true }, edac_mc_layer { r#type: EDAC_MC_LAYER_CHANNEL, size: 1, is_virt_csrow: false }]; if !devres_open_group(&mut (*op).dev, fsl_mc_err_probe as *const _, GFP_KERNEL) { return -ENOMEM; } mci = edac_mc_alloc(edac_mc_idx, 2, layers.as_mut_ptr(), core::mem::size_of::<fsl_mc_pdata>()); if mci.is_null() { devres_release_group(&mut (*op).dev, fsl_mc_err_probe as *const _); return -ENOMEM; } let pdata = (*mci).pvt_info; (*pdata).name = "fsl_mc_err"; (*mci).pdev = &mut (*op).dev; (*pdata).edac_idx = edac_mc_idx; edac_mc_idx += 1; dev_set_drvdata((*mci).pdev, mci as *mut _); (*mci).ctl_name = (*pdata).name; (*mci).dev_name = (*pdata).name; (*pdata).flag = device_get_match_data(&(*op).dev) as usize; (*pdata).little_endian = of_property_read_bool((*op).dev.of_node, "little-endian"); let mut r = resource::default(); let mut res = of_address_to_resource((*op).dev.of_node, 0, &mut r); if res != 0 { pr_err!("%s: Unable to get resource for MC err regs\n", "fsl_mc_err_probe"); goto_err(mci, op, res); } (*pdata).mc_vbase = devm_ioremap(&mut (*op).dev, r.start, resource_size(&r)); if (*pdata).mc_vbase.is_null() { res = -ENOMEM; goto_err(mci, op, res); } let (sdram_ctl, ecc_en_mask) = if (*pdata).flag == TYPE_IMX9 { (ddr_in32(pdata, IMX9_MC_ERR_EN), ERR_ECC_EN | ERR_INLINE_ECC) } else { (ddr_in32(pdata, FSL_MC_DDR_SDRAM_CFG), DSC_ECC_EN) }; if sdram_ctl & ecc_en_mask != ecc_en_mask { res = -ENODEV; goto_err(mci, op, res); } (*mci).mtype_cap = MEM_FLAG_DDR | MEM_FLAG_RDDR | MEM_FLAG_DDR2 | MEM_FLAG_RDDR2 | MEM_FLAG_DDR3 | MEM_FLAG_RDDR3 | MEM_FLAG_DDR4 | MEM_FLAG_RDDR4 | MEM_FLAG_LPDDR4; (*mci).edac_ctl_cap = EDAC_FLAG_NONE | EDAC_FLAG_SECDED; (*mci).edac_cap = EDAC_FLAG_SECDED; (*mci).mod_name = EDAC_MOD_STR; if edac_op_state == EDAC_OPSTATE_POLL { (*mci).edac_check = Some(fsl_mc_check); } (*mci).ctl_page_to_phys = None; (*mci).scrub_mode = SCRUB_SW_SRC; fsl_ddr_init_csrows(mci); (*pdata).orig_ddr_err_disable = ddr_in32(pdata, FSL_MC_ERR_DISABLE); ddr_out32(pdata, FSL_MC_ERR_DISABLE, 0); ddr_out32(pdata, FSL_MC_ERR_DETECT, !0); res = edac_mc_add_mc_with_groups(mci, fsl_ddr_dev_groups); if res != 0 { edac_mc_free(mci); return res; } 0 }

pub unsafe fn fsl_mc_err_remove(op: *mut platform_device) { let mci = dev_get_drvdata(&mut (*op).dev) as *mut mem_ctl_info; let pdata = (*mci).pvt_info; if edac_op_state == EDAC_OPSTATE_INT { ddr_out32(pdata, FSL_MC_ERR_INT_EN, 0); } ddr_out32(pdata, FSL_MC_ERR_DISABLE, (*pdata).orig_ddr_err_disable); ddr_out32(pdata, FSL_MC_ERR_SBE, (*pdata).orig_ddr_err_sbe); edac_mc_del_mc(&mut (*op).dev); edac_mc_free(mci); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
