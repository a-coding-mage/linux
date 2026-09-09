// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

// Translated from the Linux kernel implementation. Kernel types, functions,
// constants, and module-registration macros are supplied by external headers.

#[repr(C)]
pub struct clk_parent_data {
    pub fw_name: *const core::ffi::c_char,
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

static AUX_PARENTS: [clk_parent_data; 2] = [
    clk_parent_data {
        fw_name: b"pll8_vote\0".as_ptr() as *const core::ffi::c_char,
        name: b"pll8_vote\0".as_ptr() as *const core::ffi::c_char,
    },
    clk_parent_data {
        fw_name: b"pxo\0".as_ptr() as *const core::ffi::c_char,
        name: b"pxo_board\0".as_ptr() as *const core::ffi::c_char,
    },
];

static AUX_PARENT_MAP: [u32; 2] = [3, 0];

static KPSS_XCC_MATCH_TABLE: [of_device_id; 3] = [
    of_device_id {
        compatible: b"qcom,kpss-acc-v1\0".as_ptr() as *const core::ffi::c_char,
        data: 1usize as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: b"qcom,kpss-gcc\0".as_ptr() as *const core::ffi::c_char,
        data: core::ptr::null(),
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, kpss_xcc_match_table);

unsafe fn kpss_xcc_driver_probe(pdev: *mut platform_device) -> i32 {
    let dev: *mut device = &mut (*pdev).dev;
    let mut base: *mut core::ffi::c_void;
    let mut hw: *mut clk_hw;
    let mut name: *const core::ffi::c_char;

    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    if !device_get_match_data(&*pdev).is_null() {
        if of_property_read_string_index(
            (*dev).of_node,
            b"clock-output-names\0".as_ptr() as *const core::ffi::c_char,
            0,
            &mut name,
        ) != 0
        {
            return -ENODEV;
        }
        base = (base as *mut u8).add(0x14) as *mut core::ffi::c_void;
    } else {
        name = b"acpu_l2_aux\0".as_ptr() as *const core::ffi::c_char;
        base = (base as *mut u8).add(0x28) as *mut core::ffi::c_void;
    }

    hw = devm_clk_hw_register_mux_parent_data_table(
        dev,
        name,
        AUX_PARENTS.as_ptr(),
        AUX_PARENTS.len(),
        0,
        base,
        0,
        0x3,
        0,
        AUX_PARENT_MAP.as_ptr(),
        core::ptr::null_mut(),
    );
    if IS_ERR(hw) {
        return PTR_ERR(hw);
    }

    of_clk_add_hw_provider((*dev).of_node, of_clk_hw_simple_get, hw)
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe fn(*mut platform_device) -> i32>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

static mut KPSS_XCC_DRIVER: platform_driver = platform_driver {
    probe: Some(kpss_xcc_driver_probe),
    driver: driver {
        name: b"kpss-xcc\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: KPSS_XCC_MATCH_TABLE.as_ptr(),
    },
};

// module_platform_driver(kpss_xcc_driver);
// MODULE_DESCRIPTION("Krait Processor Sub System (KPSS) Clock Driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:kpss-xcc");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
