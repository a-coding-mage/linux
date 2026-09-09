// SPDX-License-Identifier: GPL-2.0-or-later
/* Platform level USB initialization for FS USB OTG controller on omap1 */

// Kernel dependencies and configuration symbols are supplied by the surrounding tree.

const INT_USB_IRQ_GEN: u32 = IH2_BASE + 20;
const INT_USB_IRQ_NISO: u32 = IH2_BASE + 30;
const INT_USB_IRQ_ISO: u32 = IH2_BASE + 29;
const INT_USB_IRQ_HGEN: u32 = INT_USB_HHC_1;
const INT_USB_IRQ_OTG: u32 = IH2_BASE + 8;

#[cfg(CONFIG_ARCH_OMAP_OTG)]
unsafe fn omap_otg_init(config: *mut omap_usb_config) {
    let mut syscon: u32;
    let mut alt_pingroup: i32 = 0;
    let mut w: u16;

    syscon = omap_readl(OTG_SYSCON_1) & 0xffff;
    if syscon & OTG_RESET_DONE == 0 { pr_debug!("USB resets not complete?\n"); }
    if (*config).pins[0] > 2 { alt_pingroup = 1; }
    syscon |= ((*config).usb0_init)((*config).pins[0], is_usb0_device(config));
    syscon |= ((*config).usb1_init)((*config).pins[1]);
    syscon |= ((*config).usb2_init)((*config).pins[2], alt_pingroup as u32);
    pr_debug!("OTG_SYSCON_1 = %08x\n", omap_readl(OTG_SYSCON_1));
    omap_writel(syscon, OTG_SYSCON_1);
    syscon = (*config).hmc_mode;
    syscon |= USBX_SYNCHRO | (4 << 16);
    if (*config).otg != 0 { syscon |= OTG_EN; }
    omap_writel(syscon, OTG_SYSCON_2);
    printk!("USB: hmc %d", (*config).hmc_mode);
    if alt_pingroup == 0 { pr_cont!(", usb2 alt %d wires", (*config).pins[2]); }
    else if (*config).pins[0] != 0 { pr_cont!(", usb0 %d wires%s", (*config).pins[0], if is_usb0_device(config) { " (dev)" } else { "" }); }
    if (*config).pins[1] != 0 { pr_cont!(", usb1 %d wires", (*config).pins[1]); }
    if alt_pingroup == 0 && (*config).pins[2] != 0 { pr_cont!(", usb2 %d wires", (*config).pins[2]); }
    if (*config).otg != 0 { pr_cont!(", Mini-AB on usb%d", (*config).otg - 1); }
    pr_cont!("\n");
    w = omap_readw(ULPD_SOFT_REQ) & !SOFT_USB_CLK_REQ; omap_writew(w, ULPD_SOFT_REQ);
    w = omap_readw(ULPD_CLOCK_CTRL); w &= !USB_MCLK_EN; w |= DIS_USB_PVCI_CLK; omap_writew(w, ULPD_CLOCK_CTRL);
    syscon = omap_readl(OTG_SYSCON_1) | HST_IDLE_EN | DEV_IDLE_EN | OTG_IDLE_EN;
    if (*config).otg != 0 || (*config).register_dev { syscon &= !DEV_IDLE_EN; (*(*config).udc_device).dev.platform_data = config; let status = platform_device_register((*config).udc_device); if status != 0 { pr_debug!("can't register UDC device, %d\n", status); } }
    if (*config).otg != 0 || (*config).register_host { syscon &= !HST_IDLE_EN; (*(*config).ohci_device).dev.platform_data = config; let status = platform_device_register((*config).ohci_device); if status != 0 { pr_debug!("can't register OHCI device, %d\n", status); } }
    if (*config).otg != 0 { syscon &= !OTG_IDLE_EN; (*(*config).otg_device).dev.platform_data = config; let status = platform_device_register((*config).otg_device); if status != 0 { pr_debug!("can't register OTG device, %d\n", status); } }
    pr_debug!("OTG_SYSCON_1 = %08x\n", omap_readl(OTG_SYSCON_1)); omap_writel(syscon, OTG_SYSCON_1);
}
#[cfg(not(CONFIG_ARCH_OMAP_OTG))]
unsafe fn omap_otg_init(_config: *mut omap_usb_config) {}

unsafe fn omap1_usb0_init(nwires: u32, is_device: u32) -> u32 {
    let mut syscon1 = 0;
    if nwires == 0 { if !cpu_is_omap15xx() { let mut l = omap_readl(USB_TRANSCEIVER_CTRL); l &= !(3 << 1); omap_writel(l, USB_TRANSCEIVER_CTRL); } return 0; }
    if is_device != 0 { omap_cfg_reg(W4_USB_PUEN); }
    if nwires == 2 { if cpu_is_omap15xx() { return 0; } let mut l = omap_readl(USB_TRANSCEIVER_CTRL); l &= !(7 << 4); if is_device == 0 { l |= 3 << 1; } omap_writel(l, USB_TRANSCEIVER_CTRL); return 3 << 16; }
    if cpu_is_omap15xx() { printk!(KERN_ERR "no usb0 alt pin config on 15xx\n"); return 0; }
    omap_cfg_reg(V6_USB0_TXD); omap_cfg_reg(W9_USB0_TXEN); omap_cfg_reg(W5_USB0_SE0); if nwires != 3 { omap_cfg_reg(Y5_USB0_RCV); }
    if nwires != 6 { let mut l = omap_readl(USB_TRANSCEIVER_CTRL); l &= !CONF_USB2_UNI_R; omap_writel(l, USB_TRANSCEIVER_CTRL); }
    match nwires { 3 => syscon1 = 2, 4 => syscon1 = 1, 6 => { syscon1 = 3; omap_cfg_reg(AA9_USB0_VP); omap_cfg_reg(R9_USB0_VM); let mut l = omap_readl(USB_TRANSCEIVER_CTRL); l |= CONF_USB2_UNI_R; omap_writel(l, USB_TRANSCEIVER_CTRL); }, _ => printk!(KERN_ERR "illegal usb%d %d-wire transceiver\n", 0, nwires) }
    syscon1 << 16
}

unsafe fn omap1_usb1_init(nwires: u32) -> u32 {
    let mut syscon1 = 0;
    if !cpu_is_omap15xx() && nwires != 6 { let mut l = omap_readl(USB_TRANSCEIVER_CTRL); l &= !CONF_USB1_UNI_R; omap_writel(l, USB_TRANSCEIVER_CTRL); }
    if nwires == 0 { return 0; }
    omap_cfg_reg(USB1_TXD); omap_cfg_reg(USB1_TXEN); if nwires != 3 { omap_cfg_reg(USB1_RCV); }
    if cpu_is_omap15xx() { omap_cfg_reg(USB1_SEO); omap_cfg_reg(USB1_SPEED); } else if cpu_is_omap1610() || cpu_is_omap5912() { omap_cfg_reg(W13_1610_USB1_SE0); omap_cfg_reg(R13_1610_USB1_SPEED); } else if cpu_is_omap1710() { omap_cfg_reg(R13_1710_USB1_SE0); } else { pr_debug!("usb%d cpu unrecognized\n", 1); return 0; }
    match nwires { 3 => syscon1 = 2, 4 => syscon1 = 1, 6 => { syscon1 = 3; omap_cfg_reg(USB1_VP); omap_cfg_reg(USB1_VM); if !cpu_is_omap15xx() { let mut l = omap_readl(USB_TRANSCEIVER_CTRL); l |= CONF_USB1_UNI_R; omap_writel(l, USB_TRANSCEIVER_CTRL); } }, _ => printk!(KERN_ERR "illegal usb%d %d-wire transceiver\n", 1, nwires) }
    syscon1 << 20
}

unsafe fn omap1_usb2_init(nwires: u32, alt_pingroup: u32) -> u32 {
    let mut syscon1 = 0; if alt_pingroup != 0 || nwires == 0 { return 0; }
    if !cpu_is_omap15xx() && nwires != 6 { let mut l = omap_readl(USB_TRANSCEIVER_CTRL); l &= !CONF_USB2_UNI_R; omap_writel(l, USB_TRANSCEIVER_CTRL); }
    if cpu_is_omap15xx() { omap_cfg_reg(USB2_TXD); omap_cfg_reg(USB2_TXEN); omap_cfg_reg(USB2_SEO); if nwires != 3 { omap_cfg_reg(USB2_RCV); } } else if cpu_is_omap16xx() { omap_cfg_reg(V6_USB2_TXD); omap_cfg_reg(W9_USB2_TXEN); omap_cfg_reg(W5_USB2_SE0); if nwires != 3 { omap_cfg_reg(Y5_USB2_RCV); } } else { pr_debug!("usb%d cpu unrecognized\n", 1); return 0; }
    match nwires { 3 => syscon1 = 2, 4 => syscon1 = 1, 6 => { syscon1 = 3; if cpu_is_omap15xx() { omap_cfg_reg(USB2_VP); omap_cfg_reg(USB2_VM); } else { omap_cfg_reg(AA9_USB2_VP); omap_cfg_reg(R9_USB2_VM); let mut l = omap_readl(USB_TRANSCEIVER_CTRL); l |= CONF_USB2_UNI_R; omap_writel(l, USB_TRANSCEIVER_CTRL); } }, _ => printk!(KERN_ERR "illegal usb%d %d-wire transceiver\n", 2, nwires) }
    syscon1 << 24
}

#[cfg(CONFIG_ARCH_OMAP15XX)]
const OMAP1510_LB_MEMSIZE: u32 = 32;
#[cfg(CONFIG_ARCH_OMAP15XX)]
const OMAP1510_LB_OFFSET: u32 = 0x30000000;
#[cfg(CONFIG_ARCH_OMAP15XX)]
const OMAP1510_LB_CLOCK_DIV: u32 = 0xfffec10c;
#[cfg(CONFIG_ARCH_OMAP15XX)]
const OMAP1510_LB_MMU_CTL: u32 = 0xfffec208;
#[cfg(CONFIG_ARCH_OMAP15XX)]
const OMAP1510_LB_MMU_LCK: u32 = 0xfffec224;
#[cfg(CONFIG_ARCH_OMAP15XX)]
const OMAP1510_LB_MMU_LD_TLB: u32 = 0xfffec228;
#[cfg(CONFIG_ARCH_OMAP15XX)]
const OMAP1510_LB_MMU_CAM_H: u32 = 0xfffec22c;
#[cfg(CONFIG_ARCH_OMAP15XX)]
const OMAP1510_LB_MMU_CAM_L: u32 = 0xfffec230;
#[cfg(CONFIG_ARCH_OMAP15XX)]
const OMAP1510_LB_MMU_RAM_H: u32 = 0xfffec234;
#[cfg(CONFIG_ARCH_OMAP15XX)]
const OMAP1510_LB_MMU_RAM_L: u32 = 0xfffec238;

#[cfg(CONFIG_ARCH_OMAP15XX)]
unsafe fn omap_1510_local_bus_power(on: i32) -> i32 {
    if on != 0 { omap_writel(3, OMAP1510_LB_MMU_CTL); udelay(200); } else { omap_writel(0, OMAP1510_LB_MMU_CTL); } 0
}
#[cfg(CONFIG_ARCH_OMAP15XX)]
unsafe fn omap_1510_local_bus_init() -> i32 {
    omap_writel((omap_readl(OMAP1510_LB_CLOCK_DIV) & 0xfffffff8) | 4, OMAP1510_LB_CLOCK_DIV);
    for tlb in 0..OMAP1510_LB_MEMSIZE { let lbaddr = tlb * 0x00100000 + OMAP1510_LB_OFFSET; let physaddr = tlb * 0x00100000 + PHYS_OFFSET;
        omap_writel((lbaddr & 0x0fffffff) >> 22, OMAP1510_LB_MMU_CAM_H); omap_writel(((lbaddr & 0x003ffc00) >> 6) | 0xc, OMAP1510_LB_MMU_CAM_L); omap_writel(physaddr >> 16, OMAP1510_LB_MMU_RAM_H); omap_writel((physaddr & 0x0000fc00) | 0x300, OMAP1510_LB_MMU_RAM_L); omap_writel(tlb << 4, OMAP1510_LB_MMU_LCK); omap_writel(1, OMAP1510_LB_MMU_LD_TLB); }
    omap_writel(omap_readl(OMAP1510_LB_MMU_CTL) | 8, OMAP1510_LB_MMU_CTL); udelay(200); 0
}
#[cfg(CONFIG_ARCH_OMAP15XX)]
unsafe fn omap_1510_local_bus_reset() { omap_1510_local_bus_power(1); omap_1510_local_bus_init(); }
#[cfg(not(CONFIG_ARCH_OMAP15XX))]
unsafe fn omap_1510_usb_init(_config: *mut omap_usb_config) {}
#[cfg(CONFIG_ARCH_OMAP15XX)]
unsafe fn omap_1510_usb_init(config: *mut omap_usb_config) {
    ((*config).usb0_init)((*config).pins[0], is_usb0_device(config)); ((*config).usb1_init)((*config).pins[1]); ((*config).usb2_init)((*config).pins[2], 0);
    let mut val = omap_readl(MOD_CONF_CTRL_0) & !(0x3f << 1); val |= (*config).hmc_mode << 1; omap_writel(val, MOD_CONF_CTRL_0);
    printk!("USB: hmc %d", (*config).hmc_mode); if (*config).pins[0] != 0 { pr_cont!(", usb0 %d wires%s", (*config).pins[0], if is_usb0_device(config) { " (dev)" } else { "" }); } if (*config).pins[1] != 0 { pr_cont!(", usb1 %d wires", (*config).pins[1]); } if (*config).pins[2] != 0 { pr_cont!(", usb2 %d wires", (*config).pins[2]); } pr_cont!("\n");
    let mut w = omap_readw(ULPD_APLL_CTRL) & !APLL_NDPLL_SWITCH; omap_writew(w, ULPD_APLL_CTRL); w = omap_readw(ULPD_DPLL_CTRL) | DPLL_IOB | DPLL_PLL_ENABLE; omap_writew(w, ULPD_DPLL_CTRL); w = omap_readw(ULPD_SOFT_REQ) | SOFT_UDC_REQ | SOFT_DPLL_REQ; omap_writew(w, ULPD_SOFT_REQ); while omap_readw(ULPD_DPLL_CTRL) & DPLL_LOCK == 0 { cpu_relax(); }
    if (*config).register_host { (*config).lb_reset = Some(omap_1510_local_bus_reset); }
}

pub unsafe fn omap1_usb_init(_pdata: *mut omap_usb_config) {
    let pdata = kmemdup(_pdata as *const _, core::mem::size_of::<omap_usb_config>(), GFP_KERNEL);
    if pdata.is_null() { return; }
    (*pdata).usb0_init = Some(omap1_usb0_init); (*pdata).usb1_init = Some(omap1_usb1_init); (*pdata).usb2_init = Some(omap1_usb2_init);
    udc_device_init(pdata); ohci_device_init(pdata); otg_device_init(pdata);
    if cpu_is_omap16xx() { omap_otg_init(pdata); } else if cpu_is_omap15xx() { omap_1510_usb_init(pdata); } else { printk!(KERN_ERR "USB: No init for your chip yet\n"); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
