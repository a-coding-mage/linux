// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2011 Sascha Hauer, Pengutronix <s.hauer@pengutronix.de>
 */

/*
 * DOC: basic fixed multiplier and divider clock that cannot gate
 *
 * Traits of this clock:
 * prepare - clk_prepare only ensures that parents are prepared
 * enable - clk_enable only ensures that parents are enabled
 * rate - rate is fixed.  clk->rate = parent->rate / div * mult
 * parent - fixed parent.  No clk_set_parent support
 */

unsafe fn clk_factor_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: c_ulong,
) -> c_ulong {
    let fix: *mut clk_fixed_factor = to_clk_fixed_factor(hw);
    let mut rate: c_ulonglong = (parent_rate as c_ulonglong)
        .wrapping_mul((*fix).mult as c_ulonglong);

    rate /= (*fix).div as c_ulonglong;
    rate as c_ulong
}

unsafe fn clk_factor_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let fix: *mut clk_fixed_factor = to_clk_fixed_factor(hw);

    if clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT != 0 {
        let parent_hw: *mut clk_hw = clk_hw_get_parent(hw);
        let best_parent: c_ulong;

        if parent_hw.is_null() {
            return -EINVAL;
        }

        best_parent = ((*req).rate / (*fix).mult as c_ulong)
            .wrapping_mul((*fix).div as c_ulong);
        (*req).best_parent_rate = clk_hw_round_rate(parent_hw, best_parent);
    }

    (*req).rate = ((*req).best_parent_rate / (*fix).div as c_ulong)
        .wrapping_mul((*fix).mult as c_ulong);

    0
}

unsafe fn clk_factor_set_rate(
    _hw: *mut clk_hw,
    _rate: c_ulong,
    _parent_rate: c_ulong,
) -> c_int {
    /*
     * We must report success but we can do so unconditionally because
     * clk_factor_determine_rate returns values that ensure this call is a
     * nop.
     */

    0
}

unsafe fn clk_factor_recalc_accuracy(
    hw: *mut clk_hw,
    parent_accuracy: c_ulong,
) -> c_ulong {
    let fix: *mut clk_fixed_factor = to_clk_fixed_factor(hw);

    if (*fix).flags & CLK_FIXED_FACTOR_FIXED_ACCURACY != 0 {
        return (*fix).acc;
    }

    parent_accuracy
}

#[no_mangle]
pub static clk_fixed_factor_ops: clk_ops = clk_ops {
    determine_rate: Some(clk_factor_determine_rate),
    set_rate: Some(clk_factor_set_rate),
    recalc_rate: Some(clk_factor_recalc_rate),
    recalc_accuracy: Some(clk_factor_recalc_accuracy),
};

unsafe fn devm_clk_hw_register_fixed_factor_release(
    _dev: *mut device,
    res: *mut c_void,
) {
    let fix: *mut clk_fixed_factor = res as *mut clk_fixed_factor;

    /*
     * We can not use clk_hw_unregister_fixed_factor, since it will kfree()
     * the hw, resulting in double free. Just unregister the hw and let
     * devres code kfree() it.
     */
    clk_hw_unregister(&mut (*fix).hw);
}

unsafe fn __clk_hw_register_fixed_factor(
    dev: *mut device,
    np: *mut device_node,
    name: *const c_char,
    parent_name: *const c_char,
    parent_hw: *const clk_hw,
    pdata: *const clk_parent_data,
    flags: c_ulong,
    mult: c_uint,
    div: c_uint,
    acc: c_ulong,
    fixflags: c_uint,
    devm: bool,
) -> *mut clk_hw {
    let fix: *mut clk_fixed_factor;
    let mut init: clk_init_data = core::mem::zeroed();
    let mut hw: *mut clk_hw;
    let ret: c_int;

    /* You can't use devm without a dev */
    if devm && dev.is_null() {
        return ERR_PTR(-EINVAL);
    }

    if devm {
        fix = devres_alloc(
            Some(devm_clk_hw_register_fixed_factor_release),
            core::mem::size_of::<clk_fixed_factor>(),
            GFP_KERNEL,
        ) as *mut clk_fixed_factor;
    } else {
        fix = kmalloc_obj::<clk_fixed_factor>();
    }
    if fix.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    /* struct clk_fixed_factor assignments */
    (*fix).mult = mult;
    (*fix).div = div;
    (*fix).hw.init = &mut init;
    (*fix).acc = acc;
    (*fix).flags = fixflags;

    init.name = name;
    init.ops = &clk_fixed_factor_ops;
    init.flags = flags;
    init.parent_names = if !parent_name.is_null() { &parent_name } else { core::ptr::null() };
    init.parent_hws = if !parent_hw.is_null() { &parent_hw } else { core::ptr::null() };
    init.parent_data = pdata;
    init.num_parents = if !parent_name.is_null() || !parent_hw.is_null() || !pdata.is_null() { 1 } else { 0 };

    hw = &mut (*fix).hw;
    if !dev.is_null() {
        ret = clk_hw_register(dev, hw);
    } else {
        ret = of_clk_hw_register(np, hw);
    }
    if ret != 0 {
        if devm {
            devres_free(fix as *mut c_void);
        } else {
            kfree(fix as *mut c_void);
        }
        hw = ERR_PTR(ret);
    } else if devm {
        devres_add(dev, fix as *mut c_void);
    }

    hw
}

unsafe fn devm_clk_hw_register_fixed_factor_index(
    dev: *mut device,
    name: *const c_char,
    index: c_uint,
    flags: c_ulong,
    mult: c_uint,
    div: c_uint,
) -> *mut clk_hw {
    let pdata = clk_parent_data { index, ..core::mem::zeroed() };

    __clk_hw_register_fixed_factor(dev, core::ptr::null_mut(), name, core::ptr::null(), core::ptr::null(), &pdata, flags, mult, div, 0, 0, true)
}

unsafe fn clk_hw_register_fixed_factor_fwname(
    dev: *mut device,
    np: *mut device_node,
    name: *const c_char,
    fw_name: *const c_char,
    flags: c_ulong,
    mult: c_uint,
    div: c_uint,
) -> *mut clk_hw {
    let pdata = clk_parent_data { index: -1, fw_name };

    __clk_hw_register_fixed_factor(dev, np, name, core::ptr::null(), core::ptr::null(), &pdata, flags, mult, div, 0, 0, false)
}

unsafe fn clk_hw_register_fixed_factor_with_accuracy_fwname(
    dev: *mut device,
    np: *mut device_node,
    name: *const c_char,
    fw_name: *const c_char,
    flags: c_ulong,
    mult: c_uint,
    div: c_uint,
    acc: c_ulong,
) -> *mut clk_hw {
    let pdata = clk_parent_data { index: -1, fw_name };

    __clk_hw_register_fixed_factor(dev, np, name, core::ptr::null(), core::ptr::null(), &pdata, flags, mult, div, acc, CLK_FIXED_FACTOR_FIXED_ACCURACY, false)
}

unsafe fn clk_hw_register_fixed_factor_index(
    dev: *mut device,
    name: *const c_char,
    index: c_uint,
    flags: c_ulong,
    mult: c_uint,
    div: c_uint,
) -> *mut clk_hw {
    let pdata = clk_parent_data { index, ..core::mem::zeroed() };

    __clk_hw_register_fixed_factor(dev, core::ptr::null_mut(), name, core::ptr::null(), core::ptr::null(), &pdata, flags, mult, div, 0, 0, false)
}

unsafe fn clk_register_fixed_factor(
    dev: *mut device,
    name: *const c_char,
    parent_name: *const c_char,
    flags: c_ulong,
    mult: c_uint,
    div: c_uint,
) -> *mut clk {
    let hw = clk_hw_register_fixed_factor(dev, name, parent_name, flags, mult, div);
    if IS_ERR(hw) {
        return ERR_CAST(hw);
    }
    (*hw).clk
}

unsafe fn clk_unregister_fixed_factor(clk: *mut clk) {
    let hw: *mut clk_hw = __clk_get_hw(clk);
    if hw.is_null() {
        return;
    }

    clk_unregister(clk);
    kfree(to_clk_fixed_factor(hw) as *mut c_void);
}

unsafe fn clk_hw_unregister_fixed_factor(hw: *mut clk_hw) {
    let fix: *mut clk_fixed_factor = to_clk_fixed_factor(hw);

    clk_hw_unregister(hw);
    kfree(fix as *mut c_void);
}

unsafe fn devm_clk_hw_register_fixed_factor_fwname(
    dev: *mut device,
    np: *mut device_node,
    name: *const c_char,
    fw_name: *const c_char,
    flags: c_ulong,
    mult: c_uint,
    div: c_uint,
) -> *mut clk_hw {
    let pdata = clk_parent_data { index: -1, fw_name };

    __clk_hw_register_fixed_factor(dev, np, name, core::ptr::null(), core::ptr::null(), &pdata, flags, mult, div, 0, 0, true)
}

unsafe fn devm_clk_hw_register_fixed_factor_with_accuracy_fwname(
    dev: *mut device,
    np: *mut device_node,
    name: *const c_char,
    fw_name: *const c_char,
    flags: c_ulong,
    mult: c_uint,
    div: c_uint,
    acc: c_ulong,
) -> *mut clk_hw {
    let pdata = clk_parent_data { index: -1, fw_name };

    __clk_hw_register_fixed_factor(dev, np, name, core::ptr::null(), core::ptr::null(), &pdata, flags, mult, div, acc, CLK_FIXED_FACTOR_FIXED_ACCURACY, true)
}

// #ifdef CONFIG_OF
unsafe fn _of_fixed_factor_clk_setup(node: *mut device_node) -> *mut clk_hw {
    let hw: *mut clk_hw;
    let clk_name = (*node).name;
    let pdata = clk_parent_data { index: 0, ..core::mem::zeroed() };
    let mut div: u32 = 0;
    let mut mult: u32 = 0;
    let ret: c_int;

    if of_property_read_u32(node, b"clock-div\0".as_ptr() as *const c_char, &mut div) != 0 {
        pr_err!("%s Fixed factor clock <%pOFn> must have a clock-div property\n", cstr!("_of_fixed_factor_clk_setup"), node);
        return ERR_PTR(-EIO);
    }

    if of_property_read_u32(node, b"clock-mult\0".as_ptr() as *const c_char, &mut mult) != 0 {
        pr_err!("%s Fixed factor clock <%pOFn> must have a clock-mult property\n", cstr!("_of_fixed_factor_clk_setup"), node);
        return ERR_PTR(-EIO);
    }

    of_property_read_string(node, b"clock-output-names\0".as_ptr() as *const c_char, &clk_name);

    hw = __clk_hw_register_fixed_factor(core::ptr::null_mut(), node, clk_name, core::ptr::null(), core::ptr::null(), &pdata, 0, mult, div, 0, 0, false);
    if IS_ERR(hw) {
        /*
         * Clear OF_POPULATED flag so that clock registration can be
         * attempted again from probe function.
         */
        of_node_clear_flag(node, OF_POPULATED);
        return ERR_CAST(hw);
    }

    ret = of_clk_add_hw_provider(node, of_clk_hw_simple_get, hw);
    if ret != 0 {
        clk_hw_unregister_fixed_factor(hw);
        return ERR_PTR(ret);
    }

    hw
}

/**
 * of_fixed_factor_clk_setup() - Setup function for simple fixed factor clock
 * @node: device node for the clock
 */
unsafe fn of_fixed_factor_clk_setup(node: *mut device_node) {
    _of_fixed_factor_clk_setup(node);
}

// CLK_OF_DECLARE(fixed_factor_clk, "fixed-factor-clock", of_fixed_factor_clk_setup);

unsafe fn of_fixed_factor_clk_remove(pdev: *mut platform_device) {
    let clk: *mut clk_hw = platform_get_drvdata(pdev);

    of_clk_del_provider((*pdev).dev.of_node);
    clk_hw_unregister_fixed_factor(clk);
}

unsafe fn of_fixed_factor_clk_probe(pdev: *mut platform_device) -> c_int {
    let clk: *mut clk_hw;

    /*
     * This function is not executed when of_fixed_factor_clk_setup
     * succeeded.
     */
    clk = _of_fixed_factor_clk_setup((*pdev).dev.of_node);
    if IS_ERR(clk) {
        return PTR_ERR(clk);
    }

    platform_set_drvdata(pdev, clk as *mut c_void);

    0
}

static mut of_fixed_factor_clk_ids: [of_device_id; 2] = [
    of_device_id { compatible: b"fixed-factor-clock\0".as_ptr() as *const c_char, ..unsafe { core::mem::zeroed() } },
    unsafe { core::mem::zeroed() },
];

// MODULE_DEVICE_TABLE(of, of_fixed_factor_clk_ids);

static mut of_fixed_factor_clk_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"of_fixed_factor_clk\0".as_ptr() as *const c_char,
        of_match_table: of_fixed_factor_clk_ids.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(of_fixed_factor_clk_probe),
    remove: Some(of_fixed_factor_clk_remove),
    ..unsafe { core::mem::zeroed() }
};

// builtin_platform_driver(of_fixed_factor_clk_driver);
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
