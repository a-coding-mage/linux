// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell Armada CP110 System Controller
 *
 * Copyright (C) 2016 Marvell
 *
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 */

/*
 * CP110 has 6 core clocks:
 *
 *  - PLL0 (1 Ghz)
 *    - PPv2 core (1/3 PLL0)
 *    - x2 Core (1/2 PLL0)
 *      - Core (1/2 x2 Core)
 *    - SDIO (2/5 PLL0)
 *
 *  - NAND clock, which is either equal to SDIO clock or 2/5 PLL0
 *
 * CP110 has 32 gateable clocks, for the various peripherals in the IP.
 */

// Dependencies supplied by the surrounding kernel translation unit.

const CP110_PM_CLOCK_GATING_REG: u32 = 0x220;
const CP110_NAND_FLASH_CLK_CTRL_REG: u32 = 0x700;
const NF_CLOCK_SEL_400_MASK: u32 = 1 << 0;

#[repr(u32)]
enum Cp110ClkType {
    CP110_CLK_TYPE_CORE,
    CP110_CLK_TYPE_GATABLE,
}

const CP110_MAX_CORE_CLOCKS: usize = 6;
const CP110_MAX_GATABLE_CLOCKS: usize = 32;
const CP110_CLK_NUM: usize = CP110_MAX_CORE_CLOCKS + CP110_MAX_GATABLE_CLOCKS;

const CP110_CORE_PLL0: usize = 0;
const CP110_CORE_PPV2: usize = 1;
const CP110_CORE_X2CORE: usize = 2;
const CP110_CORE_CORE: usize = 3;
const CP110_CORE_NAND: usize = 4;
const CP110_CORE_SDIO: usize = 5;

const CP110_GATE_AUDIO: usize = 0;
const CP110_GATE_COMM_UNIT: usize = 1;
const CP110_GATE_NAND: usize = 2;
const CP110_GATE_PPV2: usize = 3;
const CP110_GATE_SDIO: usize = 4;
const CP110_GATE_MG: usize = 5;
const CP110_GATE_MG_CORE: usize = 6;
const CP110_GATE_XOR1: usize = 7;
const CP110_GATE_XOR0: usize = 8;
const CP110_GATE_GOP_DP: usize = 9;
const CP110_GATE_PCIE_X1_0: usize = 11;
const CP110_GATE_PCIE_X1_1: usize = 12;
const CP110_GATE_PCIE_X4: usize = 13;
const CP110_GATE_PCIE_XOR: usize = 14;
const CP110_GATE_SATA: usize = 15;
const CP110_GATE_SATA_USB: usize = 16;
const CP110_GATE_MAIN: usize = 17;
const CP110_GATE_SDMMC_GOP: usize = 18;
const CP110_GATE_SLOW_IO: usize = 21;
const CP110_GATE_USB3H0: usize = 22;
const CP110_GATE_USB3H1: usize = 23;
const CP110_GATE_USB3DEV: usize = 24;
const CP110_GATE_EIP150: usize = 25;
const CP110_GATE_EIP197: usize = 26;

static GATE_BASE_NAMES: [&str; 27] = [
    "audio", "communit", "nand", "ppv2", "sdio", "mg-domain", "mg-core",
    "xor1", "xor0", "gop-dp", "", "pcie_x10", "pcie_x11", "pcie_x4",
    "pcie-xor", "sata", "sata-usb", "main", "sd-mmc-gop", "", "", "slow-io",
    "usb3h0", "usb3h1", "usb3dev", "eip150", "eip197",
];

unsafe fn gate_flags(bit_idx: u8) -> c_ulong {
    match bit_idx as usize {
        CP110_GATE_PCIE_X1_0 | CP110_GATE_PCIE_X1_1 | CP110_GATE_PCIE_X4 => {
            // Prevent stopping this clock until after a driver has taken ownership.
            CLK_IGNORE_UNUSED
        }
        _ => 0,
    }
}

#[repr(C)]
struct Cp110GateClk {
    hw: clk_hw,
    regmap: *mut regmap,
    bit_idx: u8,
}

unsafe fn cp110_gate_enable(hw: *mut clk_hw) -> c_int {
    let gate = container_of!(hw, Cp110GateClk, hw);
    regmap_update_bits((*gate).regmap, CP110_PM_CLOCK_GATING_REG,
                       1u32 << (*gate).bit_idx, 1u32 << (*gate).bit_idx);
    0
}

unsafe fn cp110_gate_disable(hw: *mut clk_hw) {
    let gate = container_of!(hw, Cp110GateClk, hw);
    regmap_update_bits((*gate).regmap, CP110_PM_CLOCK_GATING_REG,
                       1u32 << (*gate).bit_idx, 0);
}

unsafe fn cp110_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    let gate = container_of!(hw, Cp110GateClk, hw);
    let mut val: u32 = 0;
    regmap_read((*gate).regmap, CP110_PM_CLOCK_GATING_REG, &mut val);
    (val & (1u32 << (*gate).bit_idx)) as c_int
}

static CP110_GATE_OPS: clk_ops = clk_ops {
    enable: Some(cp110_gate_enable),
    disable: Some(cp110_gate_disable),
    is_enabled: Some(cp110_gate_is_enabled),
};

unsafe fn cp110_register_gate(name: *const c_char, parent_name: *const c_char,
                              regmap: *mut regmap, bit_idx: u8) -> *mut clk_hw {
    let gate = kzalloc::<Cp110GateClk>();
    if gate.is_null() { return ERR_PTR(-ENOMEM); }
    let mut init: clk_init_data = core::mem::zeroed();
    init.name = name;
    init.ops = &CP110_GATE_OPS;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    init.flags = gate_flags(bit_idx);
    (*gate).regmap = regmap;
    (*gate).bit_idx = bit_idx;
    (*gate).hw.init = &init;
    let mut hw = &mut (*gate).hw as *mut clk_hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 { kfree(gate as *mut c_void); hw = ERR_PTR(ret); }
    hw
}

unsafe fn cp110_unregister_gate(hw: *mut clk_hw) {
    clk_hw_unregister(hw);
    kfree(container_of!(hw, Cp110GateClk, hw) as *mut c_void);
}

unsafe fn cp110_of_clk_get(clkspec: *mut of_phandle_args, data: *mut c_void) -> *mut clk_hw {
    let clk_data = data as *mut clk_hw_onecell_data;
    let typ = (*clkspec).args[0];
    let idx = (*clkspec).args[1] as usize;
    if typ == CP110_CLK_TYPE_CORE as u32 {
        if idx >= CP110_MAX_CORE_CLOCKS { return ERR_PTR(-EINVAL); }
        return (*clk_data).hws[idx];
    } else if typ == CP110_CLK_TYPE_GATABLE as u32 {
        if idx >= CP110_MAX_GATABLE_CLOCKS { return ERR_PTR(-EINVAL); }
        return (*clk_data).hws[CP110_MAX_CORE_CLOCKS + idx];
    }
    ERR_PTR(-EINVAL)
}

// The remaining platform-probe implementation follows the C source's kernel APIs and
// cleanup labels directly; external kernel types and functions are intentionally unresolved.
unsafe fn cp110_syscon_common_probe(pdev: *mut platform_device, syscon_node: *mut device_node) -> c_int {
    let regmap = syscon_node_to_regmap(syscon_node);
    if IS_ERR(regmap) { return PTR_ERR(regmap); }
    let np = (*(*pdev).dev.of_node);
    let mut nand_clk_ctrl: u32 = 0;
    let mut ret = regmap_read(regmap, CP110_NAND_FLASH_CLK_CTRL_REG, &mut nand_clk_ctrl);
    if ret != 0 { return ret; }
    let cp110_clk_data = devm_kzalloc(&mut (*pdev).dev, struct_size::<clk_hw_onecell_data>(CP110_CLK_NUM), GFP_KERNEL) as *mut clk_hw_onecell_data;
    if cp110_clk_data.is_null() { return -ENOMEM; }
    (*cp110_clk_data).num = CP110_CLK_NUM;
    let cp110_clks = (*cp110_clk_data).hws;
    let pll0_name = ap_cp_unique_name(&mut (*pdev).dev, syscon_node, b"pll0\0".as_ptr() as *const c_char);
    let mut hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), pll0_name, core::ptr::null(), 0, 1000 * 1000 * 1000);
    if IS_ERR(hw) { return PTR_ERR(hw); }
    (*cp110_clks.add(CP110_CORE_PLL0)) = hw;
    let ppv2_name = ap_cp_unique_name(&mut (*pdev).dev, syscon_node, b"ppv2-core\0".as_ptr() as *const c_char);
    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), ppv2_name, pll0_name, 0, 1, 3);
    if IS_ERR(hw) { ret = PTR_ERR(hw); goto_fail!(ret, cp110_clks); }
    (*cp110_clks.add(CP110_CORE_PPV2)) = hw;
    let x2core_name = ap_cp_unique_name(&mut (*pdev).dev, syscon_node, b"x2core\0".as_ptr() as *const c_char);
    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), x2core_name, pll0_name, 0, 1, 2);
    if IS_ERR(hw) { ret = PTR_ERR(hw); goto_fail!(ret, cp110_clks); }
    (*cp110_clks.add(CP110_CORE_X2CORE)) = hw;
    let core_name = ap_cp_unique_name(&mut (*pdev).dev, syscon_node, b"core\0".as_ptr() as *const c_char);
    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), core_name, x2core_name, 0, 1, 2);
    if IS_ERR(hw) { ret = PTR_ERR(hw); goto_fail!(ret, cp110_clks); }
    (*cp110_clks.add(CP110_CORE_CORE)) = hw;
    let nand_name = ap_cp_unique_name(&mut (*pdev).dev, syscon_node, b"nand-core\0".as_ptr() as *const c_char);
    hw = if nand_clk_ctrl & NF_CLOCK_SEL_400_MASK != 0 { clk_hw_register_fixed_factor(core::ptr::null_mut(), nand_name, pll0_name, 0, 2, 5) } else { clk_hw_register_fixed_factor(core::ptr::null_mut(), nand_name, core_name, 0, 1, 1) };
    if IS_ERR(hw) { ret = PTR_ERR(hw); goto_fail!(ret, cp110_clks); }
    (*cp110_clks.add(CP110_CORE_NAND)) = hw;
    let sdio_name = ap_cp_unique_name(&mut (*pdev).dev, syscon_node, b"sdio-core\0".as_ptr() as *const c_char);
    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), sdio_name, pll0_name, 0, 2, 5);
    if IS_ERR(hw) { ret = PTR_ERR(hw); goto_fail!(ret, cp110_clks); }
    (*cp110_clks.add(CP110_CORE_SDIO)) = hw;
    let mut gate_name = [core::ptr::null_mut(); 27];
    for i in 0..GATE_BASE_NAMES.len() { gate_name[i] = ap_cp_unique_name(&mut (*pdev).dev, syscon_node, GATE_BASE_NAMES[i].as_ptr() as *const c_char); }
    for i in 0..GATE_BASE_NAMES.len() {
        if gate_name[i].is_null() { continue; }
        let parent = match i { CP110_GATE_NAND => nand_name, CP110_GATE_MG | CP110_GATE_GOP_DP | CP110_GATE_PPV2 => ppv2_name, CP110_GATE_SDIO => sdio_name, CP110_GATE_MAIN | CP110_GATE_PCIE_XOR | CP110_GATE_PCIE_X4 | CP110_GATE_EIP150 | CP110_GATE_EIP197 => x2core_name, _ => core_name };
        hw = cp110_register_gate(gate_name[i], parent, regmap, i as u8);
        if IS_ERR(hw) { ret = PTR_ERR(hw); goto_fail!(ret, cp110_clks); }
        (*cp110_clks.add(CP110_MAX_CORE_CLOCKS + i)) = hw;
    }
    ret = of_clk_add_hw_provider(np, cp110_of_clk_get, cp110_clk_data as *mut c_void);
    if ret != 0 { goto_fail!(ret, cp110_clks); }
    platform_set_drvdata(pdev, cp110_clks as *mut c_void);
    0
}

unsafe fn cp110_syscon_legacy_clk_probe(pdev: *mut platform_device) -> c_int {
    dev_warn(&mut (*pdev).dev, b"Using legacy device tree binding\n\0".as_ptr() as *const c_char);
    dev_warn(&mut (*pdev).dev, b"Update your device tree:\n\0".as_ptr() as *const c_char);
    dev_warn(&mut (*pdev).dev, b"This binding won't be supported in future kernels\n\0".as_ptr() as *const c_char);
    cp110_syscon_common_probe(pdev, (*pdev).dev.of_node)
}

unsafe fn cp110_clk_probe(pdev: *mut platform_device) -> c_int {
    cp110_syscon_common_probe(pdev, (*(*pdev).dev.of_node).parent)
}

static CP110_SYSCON_LEGACY_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"marvell,cp110-system-controller0\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];

static mut CP110_SYSCON_LEGACY_DRIVER: platform_driver = platform_driver {
    probe: Some(cp110_syscon_legacy_clk_probe),
    driver: driver {
        name: b"marvell-cp110-system-controller0\0".as_ptr() as *const c_char,
        of_match_table: CP110_SYSCON_LEGACY_OF_MATCH.as_ptr(),
        suppress_bind_attrs: true,
    },
};

builtin_platform_driver!(CP110_SYSCON_LEGACY_DRIVER);

static CP110_CLOCK_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"marvell,cp110-clock\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];

static mut CP110_CLOCK_DRIVER: platform_driver = platform_driver {
    probe: Some(cp110_clk_probe),
    driver: driver {
        name: b"marvell-cp110-clock\0".as_ptr() as *const c_char,
        of_match_table: CP110_CLOCK_OF_MATCH.as_ptr(),
        suppress_bind_attrs: true,
    },
};

builtin_platform_driver!(CP110_CLOCK_DRIVER);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
