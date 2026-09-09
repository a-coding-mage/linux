// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016 Chen-Yu Tsai. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel/CCU translation.

static CLK_PARENT_HOSC: [ClkParentData; 1] = [ClkParentData { fw_name: "hosc" }];

static CLK_PARENT_BUS: [ClkParentData; 1] = [ClkParentData { fw_name: "bus" }];

static BUS_HCI0_CLK: SunxiCcuGateData =
    sunxi_ccu_gate_data!("bus-hci0", CLK_PARENT_BUS, 0x0, BIT!(1), 0);
static USB_OHCI0_CLK: SunxiCcuGateData =
    sunxi_ccu_gate_data!("usb-ohci0", CLK_PARENT_HOSC, 0x0, BIT!(2), 0);
static BUS_HCI1_CLK: SunxiCcuGateData =
    sunxi_ccu_gate_data!("bus-hci1", CLK_PARENT_BUS, 0x0, BIT!(3), 0);
static BUS_HCI2_CLK: SunxiCcuGateData =
    sunxi_ccu_gate_data!("bus-hci2", CLK_PARENT_BUS, 0x0, BIT!(5), 0);
static USB_OHCI2_CLK: SunxiCcuGateData =
    sunxi_ccu_gate_data!("usb-ohci2", CLK_PARENT_HOSC, 0x0, BIT!(6), 0);

static USB0_PHY_CLK: SunxiCcuGateData =
    sunxi_ccu_gate_data!("usb0-phy", CLK_PARENT_HOSC, 0x4, BIT!(1), 0);
static USB1_HSIC_CLK: SunxiCcuGateData =
    sunxi_ccu_gate_data!("usb1-hsic", CLK_PARENT_HOSC, 0x4, BIT!(2), 0);
static USB1_PHY_CLK: SunxiCcuGateData =
    sunxi_ccu_gate_data!("usb1-phy", CLK_PARENT_HOSC, 0x4, BIT!(3), 0);
static USB2_HSIC_CLK: SunxiCcuGateData =
    sunxi_ccu_gate_data!("usb2-hsic", CLK_PARENT_HOSC, 0x4, BIT!(4), 0);
static USB2_PHY_CLK: SunxiCcuGateData =
    sunxi_ccu_gate_data!("usb2-phy", CLK_PARENT_HOSC, 0x4, BIT!(5), 0);
static USB_HSIC_CLK: SunxiCcuGateData =
    sunxi_ccu_gate_data!("usb-hsic", CLK_PARENT_HOSC, 0x4, BIT!(10), 0);

static mut SUN9I_A80_USB_CLKS: [&'static mut CcuCommon; 11] = [
    &mut BUS_HCI0_CLK.common,
    &mut USB_OHCI0_CLK.common,
    &mut BUS_HCI1_CLK.common,
    &mut BUS_HCI2_CLK.common,
    &mut USB_OHCI2_CLK.common,
    &mut USB0_PHY_CLK.common,
    &mut USB1_HSIC_CLK.common,
    &mut USB1_PHY_CLK.common,
    &mut USB2_HSIC_CLK.common,
    &mut USB2_PHY_CLK.common,
    &mut USB_HSIC_CLK.common,
];

static mut SUN9I_A80_USB_HW_CLKS: ClkHwOnecellData = ClkHwOnecellData {
    hws: [
        [CLK_BUS_HCI0] = &BUS_HCI0_CLK.common.hw,
        [CLK_USB_OHCI0] = &USB_OHCI0_CLK.common.hw,
        [CLK_BUS_HCI1] = &BUS_HCI1_CLK.common.hw,
        [CLK_BUS_HCI2] = &BUS_HCI2_CLK.common.hw,
        [CLK_USB_OHCI2] = &USB_OHCI2_CLK.common.hw,
        [CLK_USB0_PHY] = &USB0_PHY_CLK.common.hw,
        [CLK_USB1_HSIC] = &USB1_HSIC_CLK.common.hw,
        [CLK_USB1_PHY] = &USB1_PHY_CLK.common.hw,
        [CLK_USB2_HSIC] = &USB2_HSIC_CLK.common.hw,
        [CLK_USB2_PHY] = &USB2_PHY_CLK.common.hw,
        [CLK_USB_HSIC] = &USB_HSIC_CLK.common.hw,
    ],
    num: CLK_NUMBER,
};

static SUN9I_A80_USB_RESETS: [CcuResetMap; 9] = [
    [RST_USB0_HCI] = CcuResetMap { reg: 0x0, bit: BIT!(17) },
    [RST_USB1_HCI] = CcuResetMap { reg: 0x0, bit: BIT!(18) },
    [RST_USB2_HCI] = CcuResetMap { reg: 0x0, bit: BIT!(19) },
    [RST_USB0_PHY] = CcuResetMap { reg: 0x4, bit: BIT!(17) },
    [RST_USB1_HSIC] = CcuResetMap { reg: 0x4, bit: BIT!(18) },
    [RST_USB1_PHY] = CcuResetMap { reg: 0x4, bit: BIT!(19) },
    [RST_USB2_HSIC] = CcuResetMap { reg: 0x4, bit: BIT!(20) },
    [RST_USB2_PHY] = CcuResetMap { reg: 0x4, bit: BIT!(21) },
];

static SUN9I_A80_USB_CLK_DESC: SunxiCcuDesc = SunxiCcuDesc {
    ccu_clks: unsafe { &SUN9I_A80_USB_CLKS },
    num_ccu_clks: ARRAY_SIZE!(SUN9I_A80_USB_CLKS),
    hw_clks: unsafe { &SUN9I_A80_USB_HW_CLKS },
    resets: &SUN9I_A80_USB_RESETS,
    num_resets: ARRAY_SIZE!(SUN9I_A80_USB_RESETS),
};

unsafe fn sun9i_a80_usb_clk_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut bus_clk: *mut Clk;
    let reg: *mut core::ffi::c_void;
    let mut ret: i32;

    reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR!(reg) {
        return PTR_ERR!(reg);
    }

    bus_clk = devm_clk_get(&mut (*pdev).dev, "bus");
    if IS_ERR!(bus_clk) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR!(bus_clk), "Couldn't get bus clk\n");
    }

    // The bus clock needs to be enabled for us to access the registers.
    ret = clk_prepare_enable(bus_clk);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, "Couldn't enable bus clk: %d\n", ret);
        return ret;
    }

    ret = devm_sunxi_ccu_probe(&mut (*pdev).dev, reg, &SUN9I_A80_USB_CLK_DESC);
    if ret != 0 {
        clk_disable_unprepare(bus_clk);
        return ret;
    }

    0
}

static SUN9I_A80_USB_CLK_IDS: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "allwinner,sun9i-a80-usb-clks" },
    OfDeviceId { compatible: core::ptr::null() },
];

static mut SUN9I_A80_USB_CLK_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(sun9i_a80_usb_clk_probe),
    driver: Driver {
        name: "sun9i-a80-usb-clks",
        of_match_table: &SUN9I_A80_USB_CLK_IDS,
    },
};

module_device_table!(of, SUN9I_A80_USB_CLK_IDS);
module_platform_driver!(SUN9I_A80_USB_CLK_DRIVER);

module_import_ns!("SUNXI_CCU");
module_description!("Support for the Allwinner A80 USB CCU");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
