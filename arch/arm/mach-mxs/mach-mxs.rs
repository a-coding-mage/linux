// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 Freescale Semiconductor, Inc.
 * Copyright 2012 Linaro Ltd.
 */

// Kernel headers and "pm.h" provide the external types, functions, constants,
// and macros referenced below.

const MXS_DIGCTL_SAIF_CLKMUX_DIRECT: u32 = 0x0;
const MXS_DIGCTL_SAIF_CLKMUX_CROSSINPUT: u32 = 0x1;
const MXS_DIGCTL_SAIF_CLKMUX_EXTMSTR0: u32 = 0x2;
const MXS_DIGCTL_SAIF_CLKMUX_EXTMSTR1: u32 = 0x3;

const HW_DIGCTL_CHIPID: usize = 0x310;
const HW_DIGCTL_CHIPID_MASK: u32 = 0xffff << 16;
const HW_DIGCTL_REV_MASK: u32 = 0xff;
const HW_DIGCTL_CHIPID_MX23: u32 = 0x3780 << 16;
const HW_DIGCTL_CHIPID_MX28: u32 = 0x2800 << 16;

const MXS_CHIP_REVISION_1_0: u32 = 0x10;
const MXS_CHIP_REVISION_1_1: u32 = 0x11;
const MXS_CHIP_REVISION_1_2: u32 = 0x12;
const MXS_CHIP_REVISION_1_3: u32 = 0x13;
const MXS_CHIP_REVISION_1_4: u32 = 0x14;
const MXS_CHIP_REV_UNKNOWN: u32 = 0xff;

const MXS_SET_ADDR: usize = 0x4;
const MXS_CLR_ADDR: usize = 0x8;
const MXS_TOG_ADDR: usize = 0xc;
const HW_OCOTP_OPS2: usize = 19;
const HW_OCOTP_OPS3: usize = 20;
const OCOTP_WORD_OFFSET: usize = 0x20;
const OCOTP_WORD_COUNT: usize = 0x20;
const BM_OCOTP_CTRL_BUSY: u32 = 1 << 8;
const BM_OCOTP_CTRL_ERROR: u32 = 1 << 9;
const BM_OCOTP_CTRL_RD_BANK_OPEN: u32 = 1 << 12;

static mut chipid: u32 = 0;
static mut socid: u32 = 0;
static mut reset_addr: *mut core::ffi::c_void = core::ptr::null_mut();
static mut ocotp_words: [u32; OCOTP_WORD_COUNT] = [0; OCOTP_WORD_COUNT];
static mut ocotp_once: i32 = 0;

#[inline]
unsafe fn __mxs_setl(mask: u32, reg: *mut core::ffi::c_void) {
    __raw_writel(mask, (reg as *mut u8).add(MXS_SET_ADDR) as *mut core::ffi::c_void);
}
#[inline]
unsafe fn __mxs_clrl(mask: u32, reg: *mut core::ffi::c_void) {
    __raw_writel(mask, (reg as *mut u8).add(MXS_CLR_ADDR) as *mut core::ffi::c_void);
}
#[inline]
unsafe fn __mxs_togl(mask: u32, reg: *mut core::ffi::c_void) {
    __raw_writel(mask, (reg as *mut u8).add(MXS_TOG_ADDR) as *mut core::ffi::c_void);
}

static mut ocotp_mutex: core::ffi::c_void = core::ffi::c_void {};

unsafe fn mxs_get_ocotp() -> *const u32 {
    let np: *mut core::ffi::c_void;
    let ocotp_base: *mut core::ffi::c_void;
    let mut timeout: i32 = 0x400;
    if ocotp_once != 0 { return ocotp_words.as_ptr(); }
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"fsl,ocotp\0".as_ptr() as *const i8);
    ocotp_base = of_iomap(np, 0);
    WARN_ON(ocotp_base.is_null());
    mutex_lock(&raw mut ocotp_mutex);
    __mxs_clrl(BM_OCOTP_CTRL_ERROR, ocotp_base);
    while (__raw_readl(ocotp_base) & (BM_OCOTP_CTRL_BUSY | BM_OCOTP_CTRL_ERROR)) != 0 && { timeout -= 1; timeout != 0 } { cpu_relax(); }
    if timeout == 0 { mutex_unlock(&raw mut ocotp_mutex); pr_err("%s: timeout in reading OCOTP\n", "mxs_get_ocotp"); return core::ptr::null(); }
    __mxs_setl(BM_OCOTP_CTRL_RD_BANK_OPEN, ocotp_base);
    udelay(1);
    timeout = 0x400;
    while (__raw_readl(ocotp_base) & BM_OCOTP_CTRL_BUSY) != 0 && { timeout -= 1; timeout != 0 } { cpu_relax(); }
    if timeout == 0 { mutex_unlock(&raw mut ocotp_mutex); pr_err("%s: timeout in reading OCOTP\n", "mxs_get_ocotp"); return core::ptr::null(); }
    for i in 0..OCOTP_WORD_COUNT { ocotp_words[i] = __raw_readl((ocotp_base as *mut u8).add(OCOTP_WORD_OFFSET + i * 0x10) as *mut core::ffi::c_void); }
    __mxs_clrl(BM_OCOTP_CTRL_RD_BANK_OPEN, ocotp_base);
    ocotp_once = 1;
    mutex_unlock(&raw mut ocotp_mutex);
    ocotp_words.as_ptr()
}

#[repr(C)]
enum mac_oui { OUI_FSL, OUI_DENX, OUI_CRYSTALFONTZ, OUI_I2SE, OUI_ARMADEUS }

unsafe fn update_fec_mac_prop(oui: mac_oui) {
    let mut from: *mut core::ffi::c_void = core::ptr::null_mut();
    let ocotp = mxs_get_ocotp();
    for i in 0..2 {
        let np = of_find_compatible_node(from, core::ptr::null(), b"fsl,imx28-fec\0".as_ptr() as *const i8);
        if np.is_null() { return; } from = np;
        if of_property_present(np, b"local-mac-address\0".as_ptr() as *const i8) { continue; }
        let newmac = kzalloc(0, GFP_KERNEL);
        if newmac.is_null() { return; }
        let macaddr = (newmac as *mut u8).add(1);
        let oui_bytes = match oui { mac_oui::OUI_FSL => [0x00,0x04,0x9f], mac_oui::OUI_DENX => [0xc0,0xe5,0x4e], mac_oui::OUI_CRYSTALFONTZ => [0x58,0xb9,0xe1], mac_oui::OUI_I2SE => [0x00,0x01,0x87], mac_oui::OUI_ARMADEUS => [0x00,0x1e,0xac] };
        for j in 0..3 { *macaddr.add(j) = oui_bytes[j]; }
        let val = *ocotp.add(i); *macaddr.add(3) = ((val >> 16) & 0xff) as u8; *macaddr.add(4) = ((val >> 8) & 0xff) as u8; *macaddr.add(5) = (val & 0xff) as u8;
        of_update_property(np, newmac);
    }
}

unsafe fn enable_clk_enet_out() { let clk = clk_get_sys(b"enet_out\0".as_ptr() as *const i8, core::ptr::null()); if !IS_ERR(clk) { clk_prepare_enable(clk); } }
unsafe fn imx28_evk_init() { update_fec_mac_prop(mac_oui::OUI_FSL); mxs_saif_clkmux_select(MXS_DIGCTL_SAIF_CLKMUX_EXTMSTR0); }
unsafe fn imx28_apf28_init() { update_fec_mac_prop(mac_oui::OUI_ARMADEUS); }
unsafe fn apx4devkit_phy_fixup(phy: *mut phy_device) -> i32 { (*phy).dev_flags |= MICREL_PHY_50MHZ_CLK; 0 }
unsafe fn apx4devkit_init() { enable_clk_enet_out(); if IS_BUILTIN(CONFIG_PHYLIB) { phy_register_fixup_for_uid(PHY_ID_KSZ8051, MICREL_PHY_ID_MASK, apx4devkit_phy_fixup); } }
unsafe fn crystalfontz_init() { update_fec_mac_prop(mac_oui::OUI_CRYSTALFONTZ); }
unsafe fn duckbill_init() { update_fec_mac_prop(mac_oui::OUI_I2SE); }
unsafe fn m28cu3_init() { update_fec_mac_prop(mac_oui::OUI_DENX); }

unsafe fn mxs_get_soc_id() -> *const i8 { let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"fsl,imx23-digctl\0".as_ptr() as *const i8); let base = of_iomap(np, 0); chipid = readl((base as *mut u8).add(HW_DIGCTL_CHIPID) as *mut core::ffi::c_void); socid = chipid & HW_DIGCTL_CHIPID_MASK; iounmap(base); of_node_put(np); match socid { HW_DIGCTL_CHIPID_MX23 => b"i.MX23\0".as_ptr() as *const i8, HW_DIGCTL_CHIPID_MX28 => b"i.MX28\0".as_ptr() as *const i8, _ => b"Unknown\0".as_ptr() as *const i8 } }
unsafe fn mxs_get_cpu_rev() -> u32 { let rev = chipid & HW_DIGCTL_REV_MASK; match (socid, rev) { (HW_DIGCTL_CHIPID_MX23,0) => MXS_CHIP_REVISION_1_0, (HW_DIGCTL_CHIPID_MX23,1) => MXS_CHIP_REVISION_1_1, (HW_DIGCTL_CHIPID_MX23,2) => MXS_CHIP_REVISION_1_2, (HW_DIGCTL_CHIPID_MX23,3) => MXS_CHIP_REVISION_1_3, (HW_DIGCTL_CHIPID_MX23,4) => MXS_CHIP_REVISION_1_4, (HW_DIGCTL_CHIPID_MX28,0) => MXS_CHIP_REVISION_1_1, (HW_DIGCTL_CHIPID_MX28,1) => MXS_CHIP_REVISION_1_2, _ => MXS_CHIP_REV_UNKNOWN } }
unsafe fn mxs_get_revision() -> *const i8 { kasprintf(GFP_KERNEL, b"%d.%d\0".as_ptr() as *const i8, (mxs_get_cpu_rev() >> 4) & 0xf, mxs_get_cpu_rev() & 0xf) }

const MX23_CLKCTRL_RESET_OFFSET: usize = 0x120;
const MX28_CLKCTRL_RESET_OFFSET: usize = 0x1e0;
unsafe fn mxs_restart_init() -> i32 { let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"fsl,imx23-clkctrl\0".as_ptr() as *const i8); let np = if np.is_null() { of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"fsl,imx28-clkctrl\0".as_ptr() as *const i8) } else { np }; reset_addr = of_iomap(np, 0); if reset_addr.is_null() { return -ENODEV; } reset_addr = (reset_addr as *mut u8).add(if of_device_is_compatible(np, b"fsl,imx23-clkctrl\0".as_ptr() as *const i8) { MX23_CLKCTRL_RESET_OFFSET } else { MX28_CLKCTRL_RESET_OFFSET }) as *mut core::ffi::c_void; of_node_put(np); 0 }
unsafe fn eukrea_mbmx283lc_init() { mxs_saif_clkmux_select(MXS_DIGCTL_SAIF_CLKMUX_EXTMSTR0); }

unsafe fn mxs_machine_init() {
    let root: *mut core::ffi::c_void;
    let parent: *mut core::ffi::c_void;
    let ocotp = mxs_get_ocotp();
    let soc_dev_attr = kzalloc_obj();
    if soc_dev_attr.is_null() { return; }
    root = of_find_node_by_path(b"/\0".as_ptr() as *const i8);
    if of_property_read_string(root, b"model\0".as_ptr() as *const i8, &mut (*soc_dev_attr).machine) != 0 { kfree(soc_dev_attr); return; }
    (*soc_dev_attr).family = b"Freescale MXS Family\0".as_ptr() as *const i8;
    (*soc_dev_attr).soc_id = mxs_get_soc_id();
    (*soc_dev_attr).revision = mxs_get_revision();
    let mut soc_uid: u64 = 0;
    if socid == HW_DIGCTL_CHIPID_MX23 {
        soc_uid = *ocotp.add(HW_OCOTP_OPS3) as u64;
        system_serial_low = *ocotp.add(HW_OCOTP_OPS3);
    } else if socid == HW_DIGCTL_CHIPID_MX28 {
        system_serial_high = *ocotp.add(HW_OCOTP_OPS2);
        soc_uid = (system_serial_high as u64) << 32;
        system_serial_low = *ocotp.add(HW_OCOTP_OPS3);
        soc_uid |= system_serial_low as u64;
    }
    if soc_uid != 0 { (*soc_dev_attr).serial_number = kasprintf(GFP_KERNEL, b"%016llX\0".as_ptr() as *const i8, soc_uid); }
    let soc_dev = soc_device_register(soc_dev_attr);
    if IS_ERR(soc_dev) { kfree((*soc_dev_attr).serial_number); kfree((*soc_dev_attr).revision); kfree(soc_dev_attr); return; }
    parent = soc_device_to_device(soc_dev);
    if of_machine_is_compatible(b"fsl,imx28-evk\0".as_ptr() as *const i8) { imx28_evk_init(); }
    if of_machine_is_compatible(b"armadeus,imx28-apf28\0".as_ptr() as *const i8) { imx28_apf28_init(); }
    else if of_machine_is_compatible(b"bluegiga,apx4devkit\0".as_ptr() as *const i8) { apx4devkit_init(); }
    else if of_machine_is_compatible(b"crystalfontz,cfa10036\0".as_ptr() as *const i8) { crystalfontz_init(); }
    else if of_machine_is_compatible(b"eukrea,mbmx283lc\0".as_ptr() as *const i8) { eukrea_mbmx283lc_init(); }
    else if of_machine_is_compatible(b"i2se,duckbill\0".as_ptr() as *const i8) || of_machine_is_compatible(b"i2se,duckbill-2\0".as_ptr() as *const i8) { duckbill_init(); }
    else if of_machine_is_compatible(b"msr,m28cu3\0".as_ptr() as *const i8) { m28cu3_init(); }
    of_platform_default_populate(core::ptr::null_mut(), core::ptr::null_mut(), parent);
    mxs_restart_init();
}

const MXS_CLKCTRL_RESET_CHIP: u32 = 1 << 1;
unsafe fn mxs_restart(_mode: reboot_mode, _cmd: *const i8) { if !reset_addr.is_null() { __mxs_setl(MXS_CLKCTRL_RESET_CHIP, reset_addr); pr_err("Failed to assert the chip reset\n"); mdelay(50); } soft_restart(0); }

// DT_MACHINE_START(MXS, "Freescale MXS (Device Tree)")
// .init_machine = mxs_machine_init, .init_late = mxs_pm_init,
// .dt_compat = ["fsl,imx28", "fsl,imx23"], .restart = mxs_restart

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
