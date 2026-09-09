// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018, The Linux Foundation. All rights reserved.
 */

// C dependencies supplied by the surrounding kernel clock framework.

use core::ffi::c_void;

enum Parent {
    P_XO,
    P_GPLL0,
    P_APSS_PLL_EARLY,
}

static PARENTS_APCS_ALIAS0_CLK_SRC: [clk_parent_data; 3] = [
    clk_parent_data { fw_name: "xo" },
    clk_parent_data { fw_name: "gpll0" },
    clk_parent_data { fw_name: "pll" },
];

static PARENTS_APCS_ALIAS0_CLK_SRC_MAP: [parent_map; 3] = [
    parent_map { src: P_XO as u32, cfg: 0 },
    parent_map { src: P_GPLL0 as u32, cfg: 4 },
    parent_map { src: P_APSS_PLL_EARLY as u32, cfg: 5 },
];

static mut APCS_ALIAS0_CLK_SRC: clk_rcg2 = clk_rcg2 {
    cmd_rcgr: 0x0050,
    hid_width: 5,
    parent_map: PARENTS_APCS_ALIAS0_CLK_SRC_MAP.as_ptr(),
    clkr: clk_regmap {
        hw: clk_hw {
            init: &clk_init_data {
                name: "apcs_alias0_clk_src",
                parent_data: PARENTS_APCS_ALIAS0_CLK_SRC.as_ptr(),
                num_parents: PARENTS_APCS_ALIAS0_CLK_SRC.len(),
                ops: &clk_rcg2_mux_closest_ops,
                flags: CLK_SET_RATE_PARENT,
            },
        },
    },
};

static mut APCS_ALIAS0_CORE_CLK: clk_branch = clk_branch {
    halt_reg: 0x0058,
    clkr: clk_regmap {
        enable_reg: 0x0058,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "apcs_alias0_core_clk",
                parent_hws: &APCS_ALIAS0_CORE_PARENT_HW,
                num_parents: 1,
                flags: CLK_SET_RATE_PARENT | CLK_IS_CRITICAL,
                ops: &clk_branch2_ops,
            },
        },
    },
};

static mut APCS_ALIAS0_CORE_PARENT_HW: [*const clk_hw; 1] = [core::ptr::null()];

static APSS_IPQ6018_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0x1000,
    fast_io: true,
};

static mut APSS_IPQ6018_CLKS: [*mut clk_regmap; 2] = [
    &raw mut APCS_ALIAS0_CLK_SRC.clkr,
    &raw mut APCS_ALIAS0_CORE_CLK.clkr,
];

static APSS_IPQ6018_DESC: qcom_cc_desc = qcom_cc_desc {
    config: &APSS_IPQ6018_REGMAP_CONFIG,
    clks: APSS_IPQ6018_CLKS.as_ptr(),
    num_clks: APSS_IPQ6018_CLKS.len(),
};

unsafe extern "C" {
    static clk_rcg2_mux_closest_ops: clk_ops;
    static clk_branch2_ops: clk_ops;
}

unsafe extern "C" fn cpu_clk_notifier_fn(
    _nb: *mut notifier_block,
    action: c_ulong,
    _data: *mut c_void,
) -> c_int {
    let index: u8;
    if action == PRE_RATE_CHANGE {
        index = P_GPLL0 as u8;
    } else if action == POST_RATE_CHANGE || action == ABORT_RATE_CHANGE {
        index = P_APSS_PLL_EARLY as u8;
    } else {
        return NOTIFY_OK;
    }

    let hw = &raw mut APCS_ALIAS0_CLK_SRC.clkr.hw;
    let err = ((*(*hw).init).ops).set_parent(hw, index);
    notifier_from_errno(err)
}

unsafe extern "C" fn apss_ipq6018_probe(pdev: *mut platform_device) -> c_int {
    let hw = &raw mut APCS_ALIAS0_CLK_SRC.clkr.hw;
    let mut cpu_clk_notifier: *mut notifier_block;
    let regmap: *mut regmap;
    let mut soc_id: u32 = 0;
    let mut ret = qcom_smem_get_soc_id(&mut soc_id);
    if ret != 0 { return ret; }

    regmap = dev_get_regmap((*pdev).dev.parent, core::ptr::null());
    if regmap.is_null() { return -ENODEV; }
    ret = qcom_cc_really_probe(&mut (*pdev).dev, &APSS_IPQ6018_DESC, regmap);
    if ret != 0 { return ret; }

    match soc_id {
        QCOM_ID_IPQ5332 | QCOM_ID_IPQ5322 | QCOM_ID_IPQ5300 => {
            cpu_clk_notifier = devm_kzalloc(&mut (*pdev).dev,
                core::mem::size_of::<notifier_block>(), GFP_KERNEL);
            if cpu_clk_notifier.is_null() { return -ENOMEM; }
            (*cpu_clk_notifier).notifier_call = Some(cpu_clk_notifier_fn);
            ret = devm_clk_notifier_register(&mut (*pdev).dev, (*hw).clk,
                cpu_clk_notifier);
            if ret != 0 { return ret; }
        }
        _ => {}
    }
    0
}

static mut APSS_IPQ6018_DRIVER: platform_driver = platform_driver {
    probe: Some(apss_ipq6018_probe),
    driver: driver { name: "qcom,apss-ipq6018-clk" },
};

module_platform_driver!(APSS_IPQ6018_DRIVER);

module_description!("QCOM APSS IPQ 6018 CLK Driver");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
