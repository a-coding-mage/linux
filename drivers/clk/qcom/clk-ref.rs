// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2026, Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// Dependencies supplied by the surrounding kernel/Rust bindings are intentionally
// left external, corresponding to the C includes.

const QCOM_CLK_REF_EN_MASK: u32 = 1 << 0;

#[repr(C)]
struct qcom_clk_ref_provider {
    num_refs: usize,
    refs: [qcom_clk_ref; 0],
}

#[inline]
unsafe fn to_qcom_clk_ref(hw: *mut clk_hw) -> *mut qcom_clk_ref {
    container_of!(hw, qcom_clk_ref, hw)
}

static qcom_clk_ref_parent_data: clk_parent_data = clk_parent_data { index: 0 };

unsafe fn qcom_clk_ref_prepare(hw: *mut clk_hw) -> i32 {
    let rclk = to_qcom_clk_ref(hw);
    let mut ret: i32;

    if (*rclk).desc.num_regulators == 0 {
        return 0;
    }

    ret = regulator_bulk_enable((*rclk).desc.num_regulators, (*rclk).regulators);
    if ret != 0 {
        pr_err!("Failed to enable regulators for %s: %d\n", clk_hw_get_name(hw), ret);
    }

    ret
}

unsafe fn qcom_clk_ref_unprepare(hw: *mut clk_hw) {
    let rclk = to_qcom_clk_ref(hw);

    if (*rclk).desc.num_regulators != 0 {
        regulator_bulk_disable((*rclk).desc.num_regulators, (*rclk).regulators);
    }
}

unsafe fn qcom_clk_ref_enable(hw: *mut clk_hw) -> i32 {
    let rclk = to_qcom_clk_ref(hw);
    let ret = regmap_set_bits((*rclk).regmap, (*rclk).desc.offset, QCOM_CLK_REF_EN_MASK);
    if ret != 0 {
        return ret;
    }

    udelay(10);
    0
}

unsafe fn qcom_clk_ref_disable(hw: *mut clk_hw) {
    let rclk = to_qcom_clk_ref(hw);

    regmap_clear_bits((*rclk).regmap, (*rclk).desc.offset, QCOM_CLK_REF_EN_MASK);
    udelay(10);
}

unsafe fn qcom_clk_ref_is_enabled(hw: *mut clk_hw) -> i32 {
    let rclk = to_qcom_clk_ref(hw);
    let mut val: u32 = 0;

    let ret = regmap_read((*rclk).regmap, (*rclk).desc.offset, &mut val);
    if ret != 0 {
        return 0;
    }

    if val & QCOM_CLK_REF_EN_MASK != 0 { 1 } else { 0 }
}

static qcom_clk_ref_ops: clk_ops = clk_ops {
    prepare: Some(qcom_clk_ref_prepare),
    unprepare: Some(qcom_clk_ref_unprepare),
    enable: Some(qcom_clk_ref_enable),
    disable: Some(qcom_clk_ref_disable),
    is_enabled: Some(qcom_clk_ref_is_enabled),
};

unsafe fn qcom_clk_ref_register(
    dev: *mut device,
    regmap: *mut regmap,
    clk_refs: *mut qcom_clk_ref,
    descs: *const *const qcom_clk_ref_desc,
    num_clk_refs: usize,
) -> i32 {
    let mut init_data: clk_init_data = core::mem::zeroed();
    let mut clk_idx: usize = 0;
    let mut i: u32;

    while clk_idx < num_clk_refs {
        let clk_ref = clk_refs.add(clk_idx);
        let desc = *descs.add(clk_idx);

        // Skip unpopulated indices; the array is indexed by clock ID.
        if desc.is_null() {
            clk_idx += 1;
            continue;
        }

        if WARN_ON!((*desc).name.is_null()) {
            clk_idx += 1;
            continue;
        }

        (*clk_ref).regmap = regmap;
        (*clk_ref).desc = *desc;

        if (*clk_ref).desc.num_regulators != 0 {
            (*clk_ref).regulators = devm_kcalloc(
                dev,
                (*clk_ref).desc.num_regulators as usize,
                core::mem::size_of::<regulator_bulk_data>(),
                GFP_KERNEL,
            );
            if (*clk_ref).regulators.is_null() {
                return -ENOMEM;
            }

            i = 0;
            while i < (*clk_ref).desc.num_regulators {
                (*clk_ref).regulators.add(i as usize).supply =
                    *(*clk_ref).desc.regulator_names.add(i as usize);
                i += 1;
            }

            let ret = devm_regulator_bulk_get(
                dev,
                (*clk_ref).desc.num_regulators,
                (*clk_ref).regulators,
            );
            if ret != 0 {
                return dev_err_probe(dev, ret, c"Failed to get regulators for %s\n", (*clk_ref).desc.name);
            }
        }

        init_data.name = (*clk_ref).desc.name;
        init_data.parent_data = &qcom_clk_ref_parent_data;
        init_data.num_parents = 1;
        init_data.ops = &qcom_clk_ref_ops;
        (*clk_ref).hw.init = &init_data;

        let ret = devm_clk_hw_register(dev, &mut (*clk_ref).hw);
        if ret != 0 {
            return ret;
        }
        clk_idx += 1;
    }

    0
}

unsafe fn qcom_clk_ref_provider_get(
    clkspec: *mut of_phandle_args,
    data: *mut core::ffi::c_void,
) -> *mut clk_hw {
    let provider = data as *mut qcom_clk_ref_provider;
    let idx = (*clkspec).args[0];

    if idx >= (*provider).num_refs as u32 {
        return ERR_PTR!(-EINVAL);
    }
    if (*provider).refs.as_ptr().add(idx as usize).read().regmap.is_null() {
        return ERR_PTR!(-ENOENT);
    }
    &mut (*provider).refs.as_ptr().add(idx as usize).read().hw
}

pub unsafe fn qcom_clk_ref_probe(
    pdev: *mut platform_device,
    config: *const regmap_config,
    descs: *const *const qcom_clk_ref_desc,
    num_clk_refs: usize,
) -> i32 {
    let dev = &mut (*pdev).dev;
    let base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR!(base) {
        return PTR_ERR!(base);
    }

    let regmap = devm_regmap_init_mmio(dev, base, config);
    if IS_ERR!(regmap) {
        return PTR_ERR!(regmap);
    }

    let provider = devm_kzalloc(
        dev,
        core::mem::size_of::<qcom_clk_ref_provider>() +
            core::mem::size_of::<qcom_clk_ref>() * num_clk_refs,
        GFP_KERNEL,
    ) as *mut qcom_clk_ref_provider;
    if provider.is_null() {
        return -ENOMEM;
    }
    (*provider).num_refs = num_clk_refs;

    let ret = qcom_clk_ref_register(
        dev,
        regmap,
        (*provider).refs.as_mut_ptr(),
        descs,
        (*provider).num_refs,
    );
    if ret != 0 {
        return ret;
    }

    devm_of_clk_add_hw_provider(dev, qcom_clk_ref_provider_get, provider as *mut core::ffi::c_void)
}

EXPORT_SYMBOL_GPL!(qcom_clk_ref_probe);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
