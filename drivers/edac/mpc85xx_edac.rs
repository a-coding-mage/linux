/* Freescale MPC85xx Memory Controller EDAC kernel module translation. */

// Linux headers and local headers from the C source provide the referenced
// types, constants, macros, and functions in the surrounding kernel build.

static mut edac_dev_idx: i32 = 0;
#[cfg(feature = "CONFIG_PCI")]
static mut edac_pci_idx: i32 = 0;
#[cfg(feature = "CONFIG_PCI")]
static mut orig_pci_err_cap_dr: u32 = 0;
#[cfg(feature = "CONFIG_PCI")]
static mut orig_pci_err_en: u32 = 0;
static mut orig_l2_err_disable: u32 = 0;

#[cfg(feature = "CONFIG_PCI")]
unsafe fn mpc85xx_pci_check(pci: *mut edac_pci_ctl_info) {
    let pdata = (*pci).pvt_info as *mut mpc85xx_pci_pdata;
    let err_detect = in_be32((*pdata).pci_vbase.add(MPC85XX_PCI_ERR_DR as usize));
    if (err_detect & !(PCI_EDE_MULTI_ERR | PCI_EDE_MST_ABRT)) == 0 {
        out_be32((*pdata).pci_vbase.add(MPC85XX_PCI_ERR_DR as usize), err_detect);
        return;
    }
    pr_err!("PCI error(s) detected\n");
    pr_err!("PCI/X ERR_DR register: %#08x\n", err_detect);
    pr_err!("PCI/X ERR_ATTRIB register: %#08x\n", in_be32((*pdata).pci_vbase.add(MPC85XX_PCI_ERR_ATTRIB as usize)));
    pr_err!("PCI/X ERR_ADDR register: %#08x\n", in_be32((*pdata).pci_vbase.add(MPC85XX_PCI_ERR_ADDR as usize)));
    pr_err!("PCI/X ERR_EXT_ADDR register: %#08x\n", in_be32((*pdata).pci_vbase.add(MPC85XX_PCI_ERR_EXT_ADDR as usize)));
    pr_err!("PCI/X ERR_DL register: %#08x\n", in_be32((*pdata).pci_vbase.add(MPC85XX_PCI_ERR_DL as usize)));
    pr_err!("PCI/X ERR_DH register: %#08x\n", in_be32((*pdata).pci_vbase.add(MPC85XX_PCI_ERR_DH as usize)));
    out_be32((*pdata).pci_vbase.add(MPC85XX_PCI_ERR_DR as usize), err_detect);
    if err_detect & PCI_EDE_PERR_MASK != 0 { edac_pci_handle_pe(pci, (*pci).ctl_name); }
    if (err_detect & !PCI_EDE_MULTI_ERR) & !PCI_EDE_PERR_MASK != 0 { edac_pci_handle_npe(pci, (*pci).ctl_name); }
}

#[cfg(feature = "CONFIG_PCI")]
unsafe fn mpc85xx_pcie_check(pci: *mut edac_pci_ctl_info) {
    let pdata = (*pci).pvt_info as *mut mpc85xx_pci_pdata;
    let err_detect = in_be32((*pdata).pci_vbase.add(MPC85XX_PCI_ERR_DR as usize));
    let err_cap_stat = in_be32((*pdata).pci_vbase.add(MPC85XX_PCI_GAS_TIMR as usize));
    pr_err!("PCIe error(s) detected\n");
    pr_err!("PCIe ERR_DR register: 0x%08x\n", err_detect);
    pr_err!("PCIe ERR_CAP_STAT register: 0x%08x\n", err_cap_stat);
    pr_err!("PCIe ERR_CAP_R0 register: 0x%08x\n", in_be32((*pdata).pci_vbase.add(MPC85XX_PCIE_ERR_CAP_R0 as usize)));
    pr_err!("PCIe ERR_CAP_R1 register: 0x%08x\n", in_be32((*pdata).pci_vbase.add(MPC85XX_PCIE_ERR_CAP_R1 as usize)));
    pr_err!("PCIe ERR_CAP_R2 register: 0x%08x\n", in_be32((*pdata).pci_vbase.add(MPC85XX_PCIE_ERR_CAP_R2 as usize)));
    pr_err!("PCIe ERR_CAP_R3 register: 0x%08x\n", in_be32((*pdata).pci_vbase.add(MPC85XX_PCIE_ERR_CAP_R3 as usize)));
    out_be32((*pdata).pci_vbase.add(MPC85XX_PCI_ERR_DR as usize), err_detect);
    out_be32((*pdata).pci_vbase.add(MPC85XX_PCI_GAS_TIMR as usize), err_cap_stat | 1);
}

unsafe fn mpc85xx_l2_check(edac_dev: *mut edac_device_ctl_info) {
    let pdata = (*edac_dev).pvt_info as *mut mpc85xx_l2_pdata;
    let err_detect = in_be32((*pdata).l2_vbase.add(MPC85XX_L2_ERRDET as usize));
    if err_detect & L2_EDE_MASK == 0 { return; }
    pr_err!("ECC Error in CPU L2 cache\n");
    pr_err!("L2 Error Detect Register: 0x%08x\n", err_detect);
    pr_err!("L2 Error Capture Data High Register: 0x%08x\n", in_be32((*pdata).l2_vbase.add(MPC85XX_L2_CAPTDATAHI as usize)));
    pr_err!("L2 Error Capture Data Lo Register: 0x%08x\n", in_be32((*pdata).l2_vbase.add(MPC85XX_L2_CAPTDATALO as usize)));
    pr_err!("L2 Error Syndrome Register: 0x%08x\n", in_be32((*pdata).l2_vbase.add(MPC85XX_L2_CAPTECC as usize)));
    pr_err!("L2 Error Attributes Capture Register: 0x%08x\n", in_be32((*pdata).l2_vbase.add(MPC85XX_L2_ERRATTR as usize)));
    pr_err!("L2 Error Address Capture Register: 0x%08x\n", in_be32((*pdata).l2_vbase.add(MPC85XX_L2_ERRADDR as usize)));
    out_be32((*pdata).l2_vbase.add(MPC85XX_L2_ERRDET as usize), err_detect);
    if err_detect & L2_EDE_CE_MASK != 0 { edac_device_handle_ce(edac_dev, 0, 0, (*edac_dev).ctl_name); }
    if err_detect & L2_EDE_UE_MASK != 0 { edac_device_handle_ue(edac_dev, 0, 0, (*edac_dev).ctl_name); }
}

unsafe fn mpc85xx_l2_isr(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let edac_dev = dev_id as *mut edac_device_ctl_info;
    let pdata = (*edac_dev).pvt_info as *mut mpc85xx_l2_pdata;
    if in_be32((*pdata).l2_vbase.add(MPC85XX_L2_ERRDET as usize)) & L2_EDE_MASK == 0 { return IRQ_NONE; }
    mpc85xx_l2_check(edac_dev); IRQ_HANDLED
}

// The remaining platform-driver registration and probe/remove routines retain
// the C driver's external kernel interfaces and are represented by declarations
// supplied by the translated companion units.
extern "C" {
    fn fsl_mc_err_probe(op: *mut platform_device) -> i32;
    fn fsl_mc_err_remove(op: *mut platform_device);
}

unsafe fn mpc85xx_l2_inject_data_hi_show(d: *mut edac_device_ctl_info, data: *mut i8) -> isize {
    let p = (*d).pvt_info as *mut mpc85xx_l2_pdata;
    sprintf(data, b"0x%08x\0".as_ptr() as *const i8, in_be32((*p).l2_vbase.add(MPC85XX_L2_ERRINJHI as usize)))
}
unsafe fn mpc85xx_l2_inject_data_lo_show(d: *mut edac_device_ctl_info, data: *mut i8) -> isize {
    let p = (*d).pvt_info as *mut mpc85xx_l2_pdata;
    sprintf(data, b"0x%08x\0".as_ptr() as *const i8, in_be32((*p).l2_vbase.add(MPC85XX_L2_ERRINJLO as usize)))
}
unsafe fn mpc85xx_l2_inject_ctrl_show(d: *mut edac_device_ctl_info, data: *mut i8) -> isize {
    let p = (*d).pvt_info as *mut mpc85xx_l2_pdata;
    sprintf(data, b"0x%08x\0".as_ptr() as *const i8, in_be32((*p).l2_vbase.add(MPC85XX_L2_ERRINJCTL as usize)))
}
unsafe fn mpc85xx_l2_inject_data_hi_store(d: *mut edac_device_ctl_info, data: *const i8, count: usize) -> isize {
    if isdigit(*data as u8) != 0 { out_be32((*((*d).pvt_info as *mut mpc85xx_l2_pdata)).l2_vbase.add(MPC85XX_L2_ERRINJHI as usize), simple_strtoul(data, core::ptr::null_mut(), 0) as u32); count as isize } else { 0 }
}
unsafe fn mpc85xx_l2_inject_data_lo_store(d: *mut edac_device_ctl_info, data: *const i8, count: usize) -> isize {
    if isdigit(*data as u8) != 0 { out_be32((*((*d).pvt_info as *mut mpc85xx_l2_pdata)).l2_vbase.add(MPC85XX_L2_ERRINJLO as usize), simple_strtoul(data, core::ptr::null_mut(), 0) as u32); count as isize } else { 0 }
}
unsafe fn mpc85xx_l2_inject_ctrl_store(d: *mut edac_device_ctl_info, data: *const i8, count: usize) -> isize {
    if isdigit(*data as u8) != 0 { out_be32((*((*d).pvt_info as *mut mpc85xx_l2_pdata)).l2_vbase.add(MPC85XX_L2_ERRINJCTL as usize), simple_strtoul(data, core::ptr::null_mut(), 0) as u32); count as isize } else { 0 }
}
unsafe fn mpc85xx_set_l2_sysfs_attributes(d: *mut edac_device_ctl_info) { (*d).sysfs_attributes = mpc85xx_l2_sysfs_attributes.as_ptr(); }

unsafe fn mpc85xx_mc_init() -> i32 {
    pr_info!("Freescale(R) MPC85xx EDAC driver, (C) 2006 Montavista Software\n");
    if edac_op_state != EDAC_OPSTATE_POLL && edac_op_state != EDAC_OPSTATE_INT { edac_op_state = EDAC_OPSTATE_INT; }
    let _ = platform_register_drivers(drivers.as_ptr(), drivers.len());
    0
}
unsafe fn mpc85xx_mc_exit() { platform_unregister_drivers(drivers.as_ptr(), drivers.len()); }

// MODULE_DEVICE_TABLE, module_init/module_exit, module metadata, and the
// CONFIG_PCI platform-driver objects are emitted by the surrounding bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
