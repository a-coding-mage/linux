// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2011-2013 Freescale Semiconductor, Inc.
 * Copyright 2011 Linaro Ltd.
 */

// Kernel headers and local headers from the C translation unit provide the
// external types, constants, macros, and functions referenced below.

unsafe extern "C" {
    fn phy_write(phydev: *mut phy_device, regnum: u32, val: u16) -> i32;
    fn of_machine_is_compatible(compat: *const core::ffi::c_char) -> bool;
    fn pci_read_config_dword(dev: *mut pci_dev, where_: u32, val: *mut u32) -> i32;
    fn pci_write_config_dword(dev: *mut pci_dev, where_: u32, val: u32) -> i32;
    fn msleep(msecs: u32);
    fn phy_register_fixup_for_uid(uid: u32, mask: u32, fixup: unsafe extern "C" fn(*mut phy_device) -> i32) -> i32;
    fn of_find_compatible_node(from: *mut device_node, ty: *const core::ffi::c_char, compatible: *const core::ffi::c_char) -> *mut device_node;
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn of_clk_get_by_name(np: *mut device_node, name: *const core::ffi::c_char) -> *mut clk;
    fn of_clk_get(np: *mut device_node, index: u32) -> *mut clk;
    fn clk_get_sys(dev_id: *const core::ffi::c_char, con_id: *const core::ffi::c_char) -> *mut clk;
    fn clk_is_match(clk1: *mut clk, clk2: *mut clk) -> bool;
    fn syscon_regmap_lookup_by_compatible(compatible: *const core::ffi::c_char) -> *mut regmap;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn clk_put(clk: *mut clk);
    fn of_node_put(np: *mut device_node);
    fn cpu_is_imx6q() -> bool;
    fn cpu_is_imx6dl() -> bool;
    fn imx_get_soc_revision() -> u32;
    fn imx_print_silicon_rev(name: *const core::ffi::c_char, revision: u32);
    fn imx_anatop_init();
    fn imx6q_pm_init();
    fn imx6dl_pm_init();
    fn of_platform_default_populate(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void, c: *mut core::ffi::c_void) -> i32;
    fn imx6q_cpuidle_init();
    fn platform_device_register_simple(name: *const core::ffi::c_char, id: i32, data: *mut core::ffi::c_void, size: u32) -> *mut platform_device;
    fn debug_ll_io_init();
    fn imx_scu_map_io();
    fn imx_gpc_check_dt();
    fn imx_init_revision_from_anatop();
    fn imx_init_l2cache();
    fn imx_src_init();
    fn irqchip_init();
    fn imx6_pm_ccm_init(compatible: *const core::ffi::c_char);
}

#[repr(C)]
struct phy_device {
    _private: [u8; 0],
}
#[repr(C)]
struct pci_dev {
    devfn: u8,
    _private: [u8; 0],
}
#[repr(C)]
struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
struct clk {
    _private: [u8; 0],
}
#[repr(C)]
struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
struct platform_device {
    _private: [u8; 0],
}

/* For imx6q sabrelite board: set KSZ9021RN RGMII pad skew */
unsafe extern "C" fn ksz9021rn_phy_fixup(phydev: *mut phy_device) -> i32 {
    if IS_BUILTIN(CONFIG_PHYLIB) {
        /* min rx data delay */
        phy_write(phydev, MICREL_KSZ9021_EXTREG_CTRL, 0x8000 | MICREL_KSZ9021_RGMII_RX_DATA_PAD_SCEW);
        phy_write(phydev, MICREL_KSZ9021_EXTREG_DATA_WRITE, 0x0000);

        /* max rx/tx clock delay, min rx/tx control delay */
        phy_write(phydev, MICREL_KSZ9021_EXTREG_CTRL, 0x8000 | MICREL_KSZ9021_RGMII_CLK_CTRL_PAD_SCEW);
        phy_write(phydev, MICREL_KSZ9021_EXTREG_DATA_WRITE, 0xf0f0);
        phy_write(phydev, MICREL_KSZ9021_EXTREG_CTRL, MICREL_KSZ9021_RGMII_CLK_CTRL_PAD_SCEW);
    }
    0
}

/*
 * fixup for PLX PEX8909 bridge to configure GPIO1-7 as output High
 * as they are used for slots1-7 PERST#
 */
unsafe extern "C" fn ventana_pciesw_early_fixup(dev: *mut pci_dev) {
    let mut dw: u32 = 0;

    if !of_machine_is_compatible(c"gw,ventana".as_ptr()) { return; }
    if (*dev).devfn != 0 { return; }

    pci_read_config_dword(dev, 0x62c, &mut dw);
    dw |= 0xaaa8; // GPIO1-7 outputs
    pci_write_config_dword(dev, 0x62c, dw);
    pci_read_config_dword(dev, 0x644, &mut dw);
    dw |= 0xfe; // GPIO1-7 output high
    pci_write_config_dword(dev, 0x644, dw);
    msleep(100);
}

// DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_PLX, 0x8609, ventana_pciesw_early_fixup);
// DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_PLX, 0x8606, ventana_pciesw_early_fixup);
// DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_PLX, 0x8604, ventana_pciesw_early_fixup);

unsafe fn imx6q_enet_phy_init() {
    if IS_BUILTIN(CONFIG_PHYLIB) {
        phy_register_fixup_for_uid(PHY_ID_KSZ9021, MICREL_PHY_ID_MASK, ksz9021rn_phy_fixup);
    }
}

unsafe fn imx6q_1588_init() {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"fsl,imx6q-fec".as_ptr());
    if np.is_null() {
        pr_warn(c"%s: failed to find fec node\n".as_ptr(), c"imx6q_1588_init".as_ptr());
        return;
    }
    let fec_enet_ref = of_clk_get_by_name(np, c"enet_clk_ref".as_ptr());
    if !IS_ERR(fec_enet_ref) { of_node_put(np); return; }
    let ptp_clk = of_clk_get(np, 2);
    if IS_ERR(ptp_clk) {
        pr_warn(c"%s: failed to get ptp clock\n".as_ptr(), c"imx6q_1588_init".as_ptr());
        of_node_put(np); return;
    }
    let enet_ref = clk_get_sys(core::ptr::null(), c"enet_ref".as_ptr());
    if IS_ERR(enet_ref) {
        pr_warn(c"%s: failed to get enet clock\n".as_ptr(), c"imx6q_1588_init".as_ptr());
        clk_put(ptp_clk); of_node_put(np); return;
    }
    let clksel = if clk_is_match(ptp_clk, enet_ref) { IMX6Q_GPR1_ENET_CLK_SEL_ANATOP } else { IMX6Q_GPR1_ENET_CLK_SEL_PAD };
    let gpr = syscon_regmap_lookup_by_compatible(c"fsl,imx6q-iomuxc-gpr".as_ptr());
    if !IS_ERR(gpr) { regmap_update_bits(gpr, IOMUXC_GPR1, IMX6Q_GPR1_ENET_CLK_SEL_MASK, clksel); }
    else { pr_err(c"failed to find fsl,imx6q-iomuxc-gpr regmap\n".as_ptr()); }
    clk_put(enet_ref); clk_put(ptp_clk); of_node_put(np);
}

unsafe fn imx6q_axi_init() {
    let gpr = syscon_regmap_lookup_by_compatible(c"fsl,imx6q-iomuxc-gpr".as_ptr());
    if !IS_ERR(gpr) {
        let mask = IMX6Q_GPR4_VPU_WR_CACHE_SEL | IMX6Q_GPR4_VPU_RD_CACHE_SEL | IMX6Q_GPR4_VPU_P_WR_CACHE_VAL | IMX6Q_GPR4_VPU_P_RD_CACHE_VAL_MASK | IMX6Q_GPR4_IPU_WR_CACHE_CTL | IMX6Q_GPR4_IPU_RD_CACHE_CTL;
        regmap_update_bits(gpr, IOMUXC_GPR4, mask, mask);
        regmap_update_bits(gpr, IOMUXC_GPR6, IMX6Q_GPR6_IPU1_ID00_RD_QOS_MASK | IMX6Q_GPR6_IPU1_ID01_RD_QOS_MASK, (0xf << 16) | (0x7 << 20));
        regmap_update_bits(gpr, IOMUXC_GPR7, IMX6Q_GPR7_IPU2_ID00_RD_QOS_MASK | IMX6Q_GPR7_IPU2_ID01_RD_QOS_MASK, (0xf << 16) | (0x7 << 20));
    } else { pr_warn(c"failed to find fsl,imx6q-iomuxc-gpr regmap\n".as_ptr()); }
}

unsafe fn imx6q_init_machine() {
    if cpu_is_imx6q() && imx_get_soc_revision() >= IMX_CHIP_REVISION_2_0 { imx_print_silicon_rev(c"i.MX6QP".as_ptr(), imx_get_soc_revision() - 0x10); }
    else { imx_print_silicon_rev(if cpu_is_imx6dl() { c"i.MX6DL".as_ptr() } else { c"i.MX6Q".as_ptr() }, imx_get_soc_revision()); }
    imx6q_enet_phy_init();
    of_platform_default_populate(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    imx_anatop_init();
    if cpu_is_imx6q() { imx6q_pm_init(); } else { imx6dl_pm_init(); }
    imx6q_1588_init();
    imx6q_axi_init();
}

unsafe fn imx6q_init_late() {
    if (cpu_is_imx6q() && imx_get_soc_revision() > IMX_CHIP_REVISION_1_1) || (cpu_is_imx6dl() && imx_get_soc_revision() > IMX_CHIP_REVISION_1_0) { imx6q_cpuidle_init(); }
    if IS_ENABLED(CONFIG_ARM_IMX6Q_CPUFREQ) { platform_device_register_simple(c"imx6q-cpufreq".as_ptr(), -1, core::ptr::null_mut(), 0); }
}

unsafe fn imx6q_map_io() { debug_ll_io_init(); imx_scu_map_io(); }
unsafe fn imx6q_init_irq() { imx_gpc_check_dt(); imx_init_revision_from_anatop(); imx_init_l2cache(); imx_src_init(); irqchip_init(); imx6_pm_ccm_init(c"fsl,imx6q-ccm".as_ptr()); }

static IMX6Q_DT_COMPAT: [Option<&'static core::ffi::CStr>; 4] = [Some(c"fsl,imx6dl"), Some(c"fsl,imx6q"), Some(c"fsl,imx6qp"), None];

// DT_MACHINE_START(IMX6Q, "Freescale i.MX6 Quad/DualLite (Device Tree)")
// .l2c_aux_val = 0, .l2c_aux_mask = ~0, .smp = smp_ops(imx_smp_ops),
// .map_io = imx6q_map_io, .init_irq = imx6q_init_irq,
// .init_machine = imx6q_init_machine, .init_late = imx6q_init_late,
// .dt_compat = imx6q_dt_compat, MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
