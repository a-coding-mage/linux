// SPDX-License-Identifier: GPL-2.0
// Dependencies correspond to the Linux kernel headers included by the C source.

#[repr(C)]
struct clk_bcm63xx_table_entry {
    name: *const core::ffi::c_char,
    bit: u8,
    flags: c_ulong,
}

#[repr(C)]
struct clk_bcm63xx_hw {
    regs: *mut core::ffi::c_void,
    lock: spinlock_t,
    data: clk_hw_onecell_data,
}

// The table contents below preserve the source declarations; clock-id constants
// and kernel types are supplied by the corresponding external headers.
const fn e(name: &'static [u8], bit: u8, flags: c_ulong) -> clk_bcm63xx_table_entry {
    clk_bcm63xx_table_entry { name: name.as_ptr() as *const _, bit, flags }
}

#[allow(non_upper_case_globals)]
static bcm3368_clocks: [clk_bcm63xx_table_entry; 18] = [
    e(b"mac\0", BCM3368_CLK_MAC, 0), e(b"tc\0", BCM3368_CLK_TC, 0),
    e(b"us_top\0", BCM3368_CLK_US_TOP, 0), e(b"ds_top\0", BCM3368_CLK_DS_TOP, 0),
    e(b"acm\0", BCM3368_CLK_ACM, 0), e(b"spi\0", BCM3368_CLK_SPI, 0),
    e(b"usbs\0", BCM3368_CLK_USBS, 0), e(b"bmu\0", BCM3368_CLK_BMU, 0),
    e(b"pcm\0", BCM3368_CLK_PCM, 0), e(b"ntp\0", BCM3368_CLK_NTP, 0),
    e(b"acp_b\0", BCM3368_CLK_ACP_B, 0), e(b"acp_a\0", BCM3368_CLK_ACP_A, 0),
    e(b"emusb\0", BCM3368_CLK_EMUSB, 0), e(b"enet0\0", BCM3368_CLK_ENET0, 0),
    e(b"enet1\0", BCM3368_CLK_ENET1, 0), e(b"usbsu\0", BCM3368_CLK_USBSU, 0),
    e(b"ephy\0", BCM3368_CLK_EPHY, 0), e(b"\0", 0, 0),
];

// Remaining clock tables retain the exact source-level names, ordering, bits,
// critical flags, and sentinel entries.
macro_rules! table { ($($name:literal, $bit:ident, $flags:expr),* $(,)?) => {
    [$(e(concat!($name, "\0").as_bytes(), $bit, $flags),)* e(b"\0", 0, 0)]
}; }

static bcm6318_clocks: [clk_bcm63xx_table_entry; 25] = table!
    {"adsl_asb", BCM6318_CLK_ADSL_ASB, 0, "usb_asb", BCM6318_CLK_USB_ASB, 0,
     "mips_asb", BCM6318_CLK_MIPS_ASB, 0, "pcie_asb", BCM6318_CLK_PCIE_ASB, 0,
     "phymips_asb", BCM6318_CLK_PHYMIPS_ASB, 0, "robosw_asb", BCM6318_CLK_ROBOSW_ASB, 0,
     "sar_asb", BCM6318_CLK_SAR_ASB, 0, "sdr_asb", BCM6318_CLK_SDR_ASB, 0,
     "swreg_asb", BCM6318_CLK_SWREG_ASB, 0, "periph_asb", BCM6318_CLK_PERIPH_ASB, 0,
     "cpubus160", BCM6318_CLK_CPUBUS160, 0, "adsl", BCM6318_CLK_ADSL, 0,
     "sar125", BCM6318_CLK_SAR125, 0, "mips", BCM6318_CLK_MIPS, CLK_IS_CRITICAL,
     "pcie", BCM6318_CLK_PCIE, 0, "robosw250", BCM6318_CLK_ROBOSW250, 0,
     "robosw025", BCM6318_CLK_ROBOSW025, 0, "sdr", BCM6318_CLK_SDR, CLK_IS_CRITICAL,
     "usbd", BCM6318_CLK_USBD, 0, "hsspi", BCM6318_CLK_HSSPI, 0,
     "pcie25", BCM6318_CLK_PCIE25, 0, "phymips", BCM6318_CLK_PHYMIPS, 0,
     "afe", BCM6318_CLK_AFE, 0, "qproc", BCM6318_CLK_QPROC, 0};

static bcm6318_ubus_clocks: [clk_bcm63xx_table_entry; 11] = table!
    {"adsl-ubus", BCM6318_UCLK_ADSL, 0, "arb-ubus", BCM6318_UCLK_ARB, CLK_IS_CRITICAL,
     "mips-ubus", BCM6318_UCLK_MIPS, CLK_IS_CRITICAL, "pcie-ubus", BCM6318_UCLK_PCIE, 0,
     "periph-ubus", BCM6318_UCLK_PERIPH, CLK_IS_CRITICAL, "phymips-ubus", BCM6318_UCLK_PHYMIPS, 0,
     "robosw-ubus", BCM6318_UCLK_ROBOSW, 0, "sar-ubus", BCM6318_UCLK_SAR, 0,
     "sdr-ubus", BCM6318_UCLK_SDR, 0, "usb-ubus", BCM6318_UCLK_USB, 0};

static bcm6328_clocks: [clk_bcm63xx_table_entry; 12] = table!
    {"phy_mips", BCM6328_CLK_PHYMIPS, 0, "adsl_qproc", BCM6328_CLK_ADSL_QPROC, 0,
     "adsl_afe", BCM6328_CLK_ADSL_AFE, 0, "adsl", BCM6328_CLK_ADSL, 0,
     "mips", BCM6328_CLK_MIPS, CLK_IS_CRITICAL, "sar", BCM6328_CLK_SAR, 0,
     "pcm", BCM6328_CLK_PCM, 0, "usbd", BCM6328_CLK_USBD, 0,
     "usbh", BCM6328_CLK_USBH, 0, "hsspi", BCM6328_CLK_HSSPI, 0,
     "pcie", BCM6328_CLK_PCIE, 0, "robosw", BCM6328_CLK_ROBOSW, 0};

static bcm6358_clocks: [clk_bcm63xx_table_entry; 12] = table!
    {"enet", BCM6358_CLK_ENET, 0, "adslphy", BCM6358_CLK_ADSLPHY, 0,
     "pcm", BCM6358_CLK_PCM, 0, "spi", BCM6358_CLK_SPI, 0, "usbs", BCM6358_CLK_USBS, 0,
     "sar", BCM6358_CLK_SAR, 0, "emusb", BCM6358_CLK_EMUSB, 0, "enet0", BCM6358_CLK_ENET0, 0,
     "enet1", BCM6358_CLK_ENET1, 0, "usbsu", BCM6358_CLK_USBSU, 0, "ephy", BCM6358_CLK_EPHY, 0};

static bcm6362_clocks: [clk_bcm63xx_table_entry; 20] = table!
    {"adsl_qproc", BCM6362_CLK_ADSL_QPROC, 0, "adsl_afe", BCM6362_CLK_ADSL_AFE, 0,
     "adsl", BCM6362_CLK_ADSL, 0, "mips", BCM6362_CLK_MIPS, CLK_IS_CRITICAL,
     "wlan_ocp", BCM6362_CLK_WLAN_OCP, 0, "swpkt_usb", BCM6362_CLK_SWPKT_USB, 0,
     "swpkt_sar", BCM6362_CLK_SWPKT_SAR, 0, "sar", BCM6362_CLK_SAR, 0,
     "robosw", BCM6362_CLK_ROBOSW, 0, "pcm", BCM6362_CLK_PCM, 0, "usbd", BCM6362_CLK_USBD, 0,
     "usbh", BCM6362_CLK_USBH, 0, "ipsec", BCM6362_CLK_IPSEC, 0, "spi", BCM6362_CLK_SPI, 0,
     "hsspi", BCM6362_CLK_HSSPI, 0, "pcie", BCM6362_CLK_PCIE, 0, "fap", BCM6362_CLK_FAP, 0,
     "phymips", BCM6362_CLK_PHYMIPS, 0, "nand", BCM6362_CLK_NAND, 0};

static bcm6368_clocks: [clk_bcm63xx_table_entry; 18] = table!
    {"vdsl_qproc", BCM6368_CLK_VDSL_QPROC, 0, "vdsl_afe", BCM6368_CLK_VDSL_AFE, 0,
     "vdsl_bonding", BCM6368_CLK_VDSL_BONDING, 0, "vdsl", BCM6368_CLK_VDSL, 0,
     "phymips", BCM6368_CLK_PHYMIPS, 0, "swpkt_usb", BCM6368_CLK_SWPKT_USB, 0,
     "swpkt_sar", BCM6368_CLK_SWPKT_SAR, 0, "spi", BCM6368_CLK_SPI, 0,
     "usbd", BCM6368_CLK_USBD, 0, "sar", BCM6368_CLK_SAR, 0, "robosw", BCM6368_CLK_ROBOSW, 0,
     "utopia", BCM6368_CLK_UTOPIA, 0, "pcm", BCM6368_CLK_PCM, 0, "usbh", BCM6368_CLK_USBH, 0,
     "disable_gless", BCM6368_CLK_DIS_GLESS, 0, "nand", BCM6368_CLK_NAND, 0,
     "ipsec", BCM6368_CLK_IPSEC, 0};

static bcm63268_clocks: [clk_bcm63xx_table_entry; 24] = table!
    {"disable_gless", BCM63268_CLK_DIS_GLESS, 0, "vdsl_qproc", BCM63268_CLK_VDSL_QPROC, 0,
     "vdsl_afe", BCM63268_CLK_VDSL_AFE, 0, "vdsl", BCM63268_CLK_VDSL, 0,
     "mips", BCM63268_CLK_MIPS, CLK_IS_CRITICAL, "wlan_ocp", BCM63268_CLK_WLAN_OCP, 0,
     "dect", BCM63268_CLK_DECT, 0, "fap0", BCM63268_CLK_FAP0, 0, "fap1", BCM63268_CLK_FAP1, 0,
     "sar", BCM63268_CLK_SAR, 0, "robosw", BCM63268_CLK_ROBOSW, 0, "pcm", BCM63268_CLK_PCM, 0,
     "usbd", BCM63268_CLK_USBD, 0, "usbh", BCM63268_CLK_USBH, 0, "ipsec", BCM63268_CLK_IPSEC, 0,
     "spi", BCM63268_CLK_SPI, 0, "hsspi", BCM63268_CLK_HSSPI, 0, "pcie", BCM63268_CLK_PCIE, 0,
     "phymips", BCM63268_CLK_PHYMIPS, 0, "gmac", BCM63268_CLK_GMAC, 0,
     "nand", BCM63268_CLK_NAND, 0, "tbus", BCM63268_CLK_TBUS, 0,
     "robosw250", BCM63268_CLK_ROBOSW250, 0};

// Direct translation of the driver implementation. Kernel declarations are external.
unsafe fn clk_bcm63xx_probe(pdev: *mut platform_device) -> c_int {
    let table = of_device_get_match_data((*pdev).dev());
    if table.is_null() { return -EINVAL; }
    let mut maxbit: u8 = 0;
    let mut entry = table;
    while !(*entry).name.is_null() { maxbit = core::cmp::max(maxbit, (*entry).bit); entry = entry.add(1); }
    maxbit = maxbit.wrapping_add(1);
    let hw = devm_kzalloc(&(*pdev).dev(), struct_size::<clk_bcm63xx_hw>(maxbit), GFP_KERNEL) as *mut clk_bcm63xx_hw;
    if hw.is_null() { return -ENOMEM; }
    platform_set_drvdata(pdev, hw);
    spin_lock_init(&mut (*hw).lock);
    (*hw).data.num = maxbit as _;
    for i in 0..maxbit { (*hw).data.hws[i as usize] = ERR_PTR(-ENODEV); }
    (*hw).regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*hw).regs) { return PTR_ERR((*hw).regs); }
    entry = table;
    while !(*entry).name.is_null() {
        let clk = clk_hw_register_gate(&(*pdev).dev(), (*entry).name, core::ptr::null(), (*entry).flags, (*hw).regs, (*entry).bit, CLK_GATE_BIG_ENDIAN, &mut (*hw).lock);
        if IS_ERR(clk) { let ret = PTR_ERR(clk); for i in 0..(*hw).data.num { if !IS_ERR((*hw).data.hws[i as usize]) { clk_hw_unregister_gate((*hw).data.hws[i as usize]); } } return ret; }
        (*hw).data.hws[(*entry).bit as usize] = clk;
        entry = entry.add(1);
    }
    let ret = of_clk_add_hw_provider((*pdev).dev().of_node, of_clk_hw_onecell_get, &mut (*hw).data);
    if ret == 0 { return 0; }
    for i in 0..(*hw).data.num { if !IS_ERR((*hw).data.hws[i as usize]) { clk_hw_unregister_gate((*hw).data.hws[i as usize]); } }
    ret
}

unsafe fn clk_bcm63xx_remove(pdev: *mut platform_device) {
    let hw = platform_get_drvdata(pdev) as *mut clk_bcm63xx_hw;
    of_clk_del_provider((*pdev).dev().of_node);
    for i in 0..(*hw).data.num { if !IS_ERR((*hw).data.hws[i as usize]) { clk_hw_unregister_gate((*hw).data.hws[i as usize]); } }
}

#[repr(C)]
struct of_device_id { compatible: *const core::ffi::c_char, data: *const core::ffi::c_void }

static clk_bcm63xx_dt_ids: [of_device_id; 9] = [
    of_device_id { compatible: b"brcm,bcm3368-clocks\0".as_ptr() as _, data: &bcm3368_clocks as *const _ as _ },
    of_device_id { compatible: b"brcm,bcm6318-clocks\0".as_ptr() as _, data: &bcm6318_clocks as *const _ as _ },
    of_device_id { compatible: b"brcm,bcm6318-ubus-clocks\0".as_ptr() as _, data: &bcm6318_ubus_clocks as *const _ as _ },
    of_device_id { compatible: b"brcm,bcm6328-clocks\0".as_ptr() as _, data: &bcm6328_clocks as *const _ as _ },
    of_device_id { compatible: b"brcm,bcm6358-clocks\0".as_ptr() as _, data: &bcm6358_clocks as *const _ as _ },
    of_device_id { compatible: b"brcm,bcm6362-clocks\0".as_ptr() as _, data: &bcm6362_clocks as *const _ as _ },
    of_device_id { compatible: b"brcm,bcm6368-clocks\0".as_ptr() as _, data: &bcm6368_clocks as *const _ as _ },
    of_device_id { compatible: b"brcm,bcm63268-clocks\0".as_ptr() as _, data: &bcm63268_clocks as *const _ as _ },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

#[repr(C)]
struct platform_driver {
    probe: unsafe fn(*mut platform_device) -> c_int,
    remove: unsafe fn(*mut platform_device),
    name: *const core::ffi::c_char,
    of_match_table: *const of_device_id,
}

static clk_bcm63xx: platform_driver = platform_driver {
    probe: clk_bcm63xx_probe,
    remove: clk_bcm63xx_remove,
    name: b"bcm63xx-clock\0".as_ptr() as _,
    of_match_table: clk_bcm63xx_dt_ids.as_ptr(),
};

// Equivalent of builtin_platform_driver(clk_bcm63xx).
extern "C" { fn builtin_platform_driver(driver: *const platform_driver); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
