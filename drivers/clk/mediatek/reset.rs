// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 */

// External kernel declarations and constants are supplied by dependent files.

#[inline]
unsafe fn to_mtk_clk_rst_data(
    rcdev: *mut reset_controller_dev,
) -> *mut mtk_clk_rst_data {
    container_of!(rcdev, mtk_clk_rst_data, rcdev)
}

unsafe fn mtk_reset_update(
    rcdev: *mut reset_controller_dev,
    id: libc::c_ulong,
    deassert: bool,
) -> libc::c_int {
    let data = to_mtk_clk_rst_data(rcdev);
    let val: libc::c_uint = if deassert { 0 } else { !0 };

    regmap_update_bits(
        (*data).regmap,
        (*(*data).desc).rst_bank_ofs[(id / RST_NR_PER_BANK as libc::c_ulong) as usize],
        1u32 << (id % RST_NR_PER_BANK as libc::c_ulong),
        val,
    )
}

unsafe fn mtk_reset_assert(
    rcdev: *mut reset_controller_dev,
    id: libc::c_ulong,
) -> libc::c_int {
    mtk_reset_update(rcdev, id, false)
}

unsafe fn mtk_reset_deassert(
    rcdev: *mut reset_controller_dev,
    id: libc::c_ulong,
) -> libc::c_int {
    mtk_reset_update(rcdev, id, true)
}

unsafe fn mtk_reset(
    rcdev: *mut reset_controller_dev,
    id: libc::c_ulong,
) -> libc::c_int {
    let ret = mtk_reset_assert(rcdev, id);
    if ret != 0 {
        return ret;
    }

    mtk_reset_deassert(rcdev, id)
}

unsafe fn mtk_reset_update_set_clr(
    rcdev: *mut reset_controller_dev,
    id: libc::c_ulong,
    deassert: bool,
) -> libc::c_int {
    let data = to_mtk_clk_rst_data(rcdev);
    let deassert_ofs: libc::c_uint = if deassert { 0x4 } else { 0 };

    regmap_write(
        (*data).regmap,
        (*(*data).desc).rst_bank_ofs[(id / RST_NR_PER_BANK as libc::c_ulong) as usize]
            .wrapping_add(deassert_ofs),
        1u32 << (id % RST_NR_PER_BANK as libc::c_ulong),
    )
}

unsafe fn mtk_reset_assert_set_clr(
    rcdev: *mut reset_controller_dev,
    id: libc::c_ulong,
) -> libc::c_int {
    mtk_reset_update_set_clr(rcdev, id, false)
}

unsafe fn mtk_reset_deassert_set_clr(
    rcdev: *mut reset_controller_dev,
    id: libc::c_ulong,
) -> libc::c_int {
    mtk_reset_update_set_clr(rcdev, id, true)
}

unsafe fn mtk_reset_set_clr(
    rcdev: *mut reset_controller_dev,
    id: libc::c_ulong,
) -> libc::c_int {
    let ret = mtk_reset_assert_set_clr(rcdev, id);
    if ret != 0 {
        return ret;
    }
    mtk_reset_deassert_set_clr(rcdev, id)
}

static MTK_RESET_OPS: reset_control_ops = reset_control_ops {
    assert: Some(mtk_reset_assert),
    deassert: Some(mtk_reset_deassert),
    reset: Some(mtk_reset),
};

static MTK_RESET_OPS_SET_CLR: reset_control_ops = reset_control_ops {
    assert: Some(mtk_reset_assert_set_clr),
    deassert: Some(mtk_reset_deassert_set_clr),
    reset: Some(mtk_reset_set_clr),
};

unsafe fn reset_xlate(
    rcdev: *mut reset_controller_dev,
    reset_spec: *const of_phandle_args,
) -> libc::c_int {
    let data = to_mtk_clk_rst_data(rcdev);
    let arg = (*reset_spec).args[0];

    if arg >= (*rcdev).nr_resets || arg >= (*(*data).desc).rst_idx_map_nr {
        return -22;
    }

    (*(*data).desc).rst_idx_map[arg as usize]
}

pub unsafe fn mtk_register_reset_controller_with_dev(
    dev: *mut device,
    desc: *const mtk_clk_rst_desc,
) -> libc::c_int {
    let np = (*dev).of_node;
    let mut regmap: *mut regmap = core::ptr::null_mut();
    let mut rcops: *const reset_control_ops = core::ptr::null();
    let data: *mut mtk_clk_rst_data;
    let ret: libc::c_int;

    if desc.is_null() {
        dev_err!(dev, "mtk clock reset desc is NULL\\n");
        return -22;
    }

    match (*desc).version {
        MTK_RST_SIMPLE => rcops = &MTK_RESET_OPS,
        MTK_RST_SET_CLR => rcops = &MTK_RESET_OPS_SET_CLR,
        _ => {
            dev_err!(dev, "Unknown reset version %d\\n", (*desc).version);
            return -22;
        }
    }

    regmap = device_node_to_regmap(np);
    if IS_ERR!(regmap) {
        dev_err!(dev, "Cannot find regmap %pe\\n", regmap);
        return PTR_ERR!(regmap);
    }

    data = devm_kzalloc!(dev, core::mem::size_of::<mtk_clk_rst_data>(), GFP_KERNEL);
    if data.is_null() {
        return -12;
    }

    (*data).desc = desc;
    (*data).regmap = regmap;
    (*data).rcdev.owner = THIS_MODULE;
    (*data).rcdev.ops = rcops;
    (*data).rcdev.of_node = np;
    (*data).rcdev.dev = dev;

    if (*(*data).desc).rst_idx_map_nr > 0 {
        (*data).rcdev.of_reset_n_cells = 1;
        (*data).rcdev.nr_resets = (*desc).rst_idx_map_nr;
        (*data).rcdev.of_xlate = Some(reset_xlate);
    } else {
        (*data).rcdev.nr_resets = (*desc).rst_bank_nr * RST_NR_PER_BANK;
    }

    ret = devm_reset_controller_register!(dev, &mut (*data).rcdev);
    if ret != 0 {
        dev_err!(dev, "could not register reset controller: %d\\n", ret);
        return ret;
    }

    0
}

EXPORT_SYMBOL_GPL!(mtk_register_reset_controller_with_dev);

MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
