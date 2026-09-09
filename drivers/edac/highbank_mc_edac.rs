// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2011-2012 Calxeda, Inc.
 */

// Linux kernel dependencies supplied by other translation units.

const HB_DDR_ECC_ERR_BASE: i32 = 0x128;
const MW_DDR_ECC_ERR_BASE: i32 = 0x1b4;
const HB_DDR_ECC_OPT: i32 = 0x00;
const HB_DDR_ECC_U_ERR_ADDR: i32 = 0x08;
const HB_DDR_ECC_U_ERR_STAT: i32 = 0x0c;
const HB_DDR_ECC_U_ERR_DATAL: i32 = 0x10;
const HB_DDR_ECC_U_ERR_DATAH: i32 = 0x14;
const HB_DDR_ECC_C_ERR_ADDR: i32 = 0x18;
const HB_DDR_ECC_C_ERR_STAT: i32 = 0x1c;
const HB_DDR_ECC_C_ERR_DATAL: i32 = 0x20;
const HB_DDR_ECC_C_ERR_DATAH: i32 = 0x24;
const HB_DDR_ECC_OPT_MODE_MASK: u32 = 0x3;
const HB_DDR_ECC_OPT_FWC: u32 = 0x100;
const HB_DDR_ECC_OPT_XOR_SHIFT: u32 = 16;
const HB_DDR_ECC_INT_BASE: i32 = 0x180;
const MW_DDR_ECC_INT_BASE: i32 = 0x218;
const HB_DDR_ECC_INT_STATUS: i32 = 0x00;
const HB_DDR_ECC_INT_ACK: i32 = 0x04;
const HB_DDR_ECC_INT_STAT_CE: u32 = 0x8;
const HB_DDR_ECC_INT_STAT_DOUBLE_CE: u32 = 0x10;
const HB_DDR_ECC_INT_STAT_UE: u32 = 0x20;
const HB_DDR_ECC_INT_STAT_DOUBLE_UE: u32 = 0x40;

#[repr(C)]
struct hb_mc_drvdata {
    mc_err_base: *mut core::ffi::c_void,
    mc_int_base: *mut core::ffi::c_void,
}

unsafe fn highbank_mc_err_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let mci = dev_id as *mut mem_ctl_info;
    let drvdata = (*mci).pvt_info as *mut hb_mc_drvdata;
    let mut status: u32;
    let mut err_addr: u32;

    // Read the interrupt status register
    status = readl((*drvdata).mc_int_base.byte_add(HB_DDR_ECC_INT_STATUS as usize));
    if status & HB_DDR_ECC_INT_STAT_UE != 0 {
        err_addr = readl((*drvdata).mc_err_base.byte_add(HB_DDR_ECC_U_ERR_ADDR as usize));
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1,
            err_addr >> PAGE_SHIFT, err_addr & !PAGE_MASK, 0, 0, 0, -1,
            (*mci).ctl_name, "");
    }
    if status & HB_DDR_ECC_INT_STAT_CE != 0 {
        let mut syndrome = readl((*drvdata).mc_err_base.byte_add(HB_DDR_ECC_C_ERR_STAT as usize));
        syndrome = (syndrome >> 8) & 0xff;
        err_addr = readl((*drvdata).mc_err_base.byte_add(HB_DDR_ECC_C_ERR_ADDR as usize));
        edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1,
            err_addr >> PAGE_SHIFT, err_addr & !PAGE_MASK, syndrome, 0, 0, -1,
            (*mci).ctl_name, "");
    }
    // clear the error, clears the interrupt
    writel(status, (*drvdata).mc_int_base.byte_add(HB_DDR_ECC_INT_ACK as usize));
    IRQ_HANDLED
}

unsafe fn highbank_mc_err_inject(mci: *mut mem_ctl_info, synd: u8) {
    let pdata = (*mci).pvt_info as *mut hb_mc_drvdata;
    let mut reg = readl((*pdata).mc_err_base.byte_add(HB_DDR_ECC_OPT as usize));
    reg &= HB_DDR_ECC_OPT_MODE_MASK;
    reg |= ((synd as u32) << HB_DDR_ECC_OPT_XOR_SHIFT) | HB_DDR_ECC_OPT_FWC;
    writel(reg, (*pdata).mc_err_base.byte_add(HB_DDR_ECC_OPT as usize));
}

#[repr(C)]
struct hb_mc_settings { err_offset: i32, int_offset: i32 }

static mut hb_settings: hb_mc_settings = hb_mc_settings {
    err_offset: HB_DDR_ECC_ERR_BASE, int_offset: HB_DDR_ECC_INT_BASE,
};
static mut mw_settings: hb_mc_settings = hb_mc_settings {
    err_offset: MW_DDR_ECC_ERR_BASE, int_offset: MW_DDR_ECC_INT_BASE,
};

// Equivalent of the device-tree match table.
static hb_ddr_ctrl_of_match: [of_device_id; 3] = [
    of_device_id { compatible: "calxeda,hb-ddr-ctrl", data: unsafe { &mut hb_settings as *mut _ as *mut _ } },
    of_device_id { compatible: "calxeda,ecx-2000-ddr-ctrl", data: unsafe { &mut mw_settings as *mut _ as *mut _ } },
    of_device_id::default(),
];

unsafe fn highbank_mc_probe(pdev: *mut platform_device) -> i32 {
    let id = of_match_device(&hb_ddr_ctrl_of_match, &mut (*pdev).dev);
    if id.is_null() { return -ENODEV; }
    let mut layers: [edac_mc_layer; 2] = [core::mem::zeroed(), core::mem::zeroed()];
    layers[0].type_ = EDAC_MC_LAYER_CHIP_SELECT; layers[0].size = 1; layers[0].is_virt_csrow = true;
    layers[1].type_ = EDAC_MC_LAYER_CHANNEL; layers[1].size = 1; layers[1].is_virt_csrow = false;
    let mci = edac_mc_alloc(0, layers.len(), layers.as_mut_ptr(), core::mem::size_of::<hb_mc_drvdata>());
    if mci.is_null() { return -ENOMEM; }
    (*mci).pdev = &mut (*pdev).dev;
    let drvdata = (*mci).pvt_info as *mut hb_mc_drvdata;
    platform_set_drvdata(pdev, mci);
    if !devres_open_group(&mut (*pdev).dev, core::ptr::null_mut(), GFP_KERNEL) { edac_mc_free(mci); return -ENOMEM; }
    let r = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if r.is_null() { dev_err(&(*pdev).dev, "Unable to get mem resource\n"); goto_err(&mut (*pdev).dev, mci, -ENODEV); }
    let base = devm_ioremap(&mut (*pdev).dev, (*r).start, resource_size(r));
    if base.is_null() { dev_err(&(*pdev).dev, "Unable to map regs\n"); goto_err(&mut (*pdev).dev, mci, -ENOMEM); }
    let settings = (*id).data as *mut hb_mc_settings;
    (*drvdata).mc_err_base = base.byte_add((*settings).err_offset as usize);
    (*drvdata).mc_int_base = base.byte_add((*settings).int_offset as usize);
    if { let control = readl((*drvdata).mc_err_base) & 0x3; control == 0 || control == 0x2 } { dev_err(&(*pdev).dev, "No ECC present, or ECC disabled\n"); goto_err(&mut (*pdev).dev, mci, -ENODEV); }
    (*mci).mtype_cap = MEM_FLAG_DDR3; (*mci).edac_ctl_cap = EDAC_FLAG_NONE | EDAC_FLAG_SECDED; (*mci).edac_cap = EDAC_FLAG_SECDED;
    (*mci).mod_name = (*pdev).dev.driver.name; (*mci).ctl_name = (*id).compatible; (*mci).dev_name = dev_name(&(*pdev).dev); (*mci).scrub_mode = SCRUB_SW_SRC;
    // Only a single 4GB DIMM is supported
    let dimm = *(*mci).dimms; (*dimm).nr_pages = (!0usize >> PAGE_SHIFT) + 1; (*dimm).grain = 8; (*dimm).dtype = DEV_X8; (*dimm).mtype = MEM_DDR3; (*dimm).edac_mode = EDAC_SECDED;
    let res = edac_mc_add_mc_with_groups(mci, highbank_dev_groups); if res < 0 { goto_err(&mut (*pdev).dev, mci, res); }
    let irq = platform_get_irq(pdev, 0); let res = devm_request_irq(&mut (*pdev).dev, irq, highbank_mc_err_handler, 0, dev_name(&(*pdev).dev), mci); if res < 0 { goto_err(&mut (*pdev).dev, mci, res); }
    devres_close_group(&mut (*pdev).dev, core::ptr::null_mut()); 0
}

unsafe fn highbank_mc_remove(pdev: *mut platform_device) { let mci = platform_get_drvdata(pdev); edac_mc_del_mc(&mut (*pdev).dev); edac_mc_free(mci); }

// module_platform_driver(highbank_mc_edac_driver);
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Calxeda, Inc.");
// MODULE_DESCRIPTION("EDAC Driver for Calxeda Highbank");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
