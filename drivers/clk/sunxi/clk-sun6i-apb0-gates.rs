// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 Free Electrons
 *
 * Author: Boris BREZILLON <boris.brezillon@free-electrons.com>
 *
 * Allwinner A31 APB0 clock gates driver
 */

// Translated from the Linux kernel implementation. Kernel headers and symbols
// referenced below are supplied by the surrounding Rust translation unit.

const SUN6I_APB0_GATES_MAX_SIZE: usize = 32;

#[repr(C)]
struct gates_data {
    mask: [u32; 1],
}

static SUN6I_A31_APB0_GATES: gates_data = gates_data { mask: [0x7F] };

static SUN8I_A23_APB0_GATES: gates_data = gates_data { mask: [0x5D] };

#[repr(C)]
struct of_device_id {
    compatible: *const core::ffi::c_char,
    data: *const core::ffi::c_void,
}

static SUN6I_A31_APB0_GATES_CLK_DT_IDS: [of_device_id; 3] = [
    of_device_id {
        compatible: b"allwinner,sun6i-a31-apb0-gates-clk\0".as_ptr() as *const core::ffi::c_char,
        data: &SUN6I_A31_APB0_GATES as *const gates_data as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: b"allwinner,sun8i-a23-apb0-gates-clk\0".as_ptr() as *const core::ffi::c_char,
        data: &SUN8I_A23_APB0_GATES as *const gates_data as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

unsafe fn sun6i_a31_apb0_gates_clk_probe(pdev: *mut platform_device) -> i32 {
    let np = (*pdev).dev.of_node;
    let mut clk_data: *mut clk_onecell_data;
    let data: *const gates_data;
    let clk_parent: *const core::ffi::c_char;
    let mut clk_name: *const core::ffi::c_char;
    let reg: *mut core::ffi::c_void;
    let ngates: i32;
    let mut i: i32;
    let mut j: i32 = 0;

    if np.is_null() {
        return -ENODEV;
    }

    data = of_device_get_match_data(&(*pdev).dev) as *const gates_data;
    if data.is_null() {
        return -ENODEV;
    }

    reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(reg) {
        return PTR_ERR(reg);
    }

    clk_parent = of_clk_get_parent_name(np, 0);
    if clk_parent.is_null() {
        return -EINVAL;
    }

    clk_data = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<clk_onecell_data>(),
        GFP_KERNEL,
    ) as *mut clk_onecell_data;
    if clk_data.is_null() {
        return -ENOMEM;
    }

    /* Worst-case size approximation and memory allocation */
    ngates = find_last_bit((*data).mask.as_ptr(), SUN6I_APB0_GATES_MAX_SIZE) as i32;
    (*clk_data).clks = devm_kcalloc(
        &mut (*pdev).dev,
        (ngates + 1) as usize,
        core::mem::size_of::<*mut clk>(),
        GFP_KERNEL,
    ) as *mut *mut clk;
    if (*clk_data).clks.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < SUN6I_APB0_GATES_MAX_SIZE as i32 {
        if ((*data).mask[(i as usize) / 32] & (1u32 << ((i as usize) % 32))) != 0 {
            of_property_read_string_index(np, b"clock-output-names\0".as_ptr() as *const core::ffi::c_char, j, &mut clk_name);

            *(*clk_data).clks.add(i as usize) = clk_register_gate(
                &mut (*pdev).dev,
                clk_name,
                clk_parent,
                0,
                reg,
                i,
                0,
                core::ptr::null_mut(),
            );
            WARN_ON(IS_ERR(*(*clk_data).clks.add(i as usize)));

            j += 1;
        }
        i += 1;
    }

    (*clk_data).clk_num = (ngates + 1) as u32;

    of_clk_add_provider(np, of_clk_src_onecell_get, clk_data)
}

static mut SUN6I_A31_APB0_GATES_CLK_DRIVER: platform_driver = platform_driver {
    driver: driver {
        name: b"sun6i-a31-apb0-gates-clk\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: SUN6I_A31_APB0_GATES_CLK_DT_IDS.as_ptr(),
    },
    probe: Some(sun6i_a31_apb0_gates_clk_probe),
};

// builtin_platform_driver(sun6i_a31_apb0_gates_clk_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
