// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2014 Samsung Electronics Co., Ltd.
 * Sylwester Nawrocki <s.nawrocki@samsung.com>
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn __set_clk_parents(node: *mut device_node, clk_supplier: bool) -> i32 {
    let mut clkspec: of_phandle_args = core::mem::zeroed();
    let mut index: i32;
    let mut rc: i32;
    let num_parents: i32;
    let mut clk: *mut clk;
    let mut pclk: *mut clk;

    num_parents = of_count_phandle_with_args(
        node,
        b"assigned-clock-parents\0".as_ptr() as *const i8,
        b"#clock-cells\0".as_ptr() as *const i8,
    );
    if num_parents == -EINVAL {
        pr_err!("clk: invalid value of clock-parents property at %pOF\n", node);
    }

    index = 0;
    while index < num_parents {
        rc = of_parse_phandle_with_args(
            node,
            b"assigned-clock-parents\0".as_ptr() as *const i8,
            b"#clock-cells\0".as_ptr() as *const i8,
            index,
            &mut clkspec,
        );
        if rc < 0 {
            /* skip empty (null) phandles */
            if rc == -ENOENT {
                index += 1;
                continue;
            } else {
                return rc;
            }
        }
        if clkspec.np == node && !clk_supplier {
            of_node_put(clkspec.np);
            return 0;
        }
        pclk = of_clk_get_from_provider(&mut clkspec);
        of_node_put(clkspec.np);
        if IS_ERR(pclk) {
            if PTR_ERR(pclk) != -EPROBE_DEFER {
                pr_warn!("clk: couldn't get parent clock %d for %pOF\n", index, node);
            }
            return PTR_ERR(pclk);
        }

        rc = of_parse_phandle_with_args(
            node,
            b"assigned-clocks\0".as_ptr() as *const i8,
            b"#clock-cells\0".as_ptr() as *const i8,
            index,
            &mut clkspec,
        );
        if rc < 0 {
            goto_err: {
                clk_put(pclk);
                return rc;
            }
        }
        if clkspec.np == node && !clk_supplier {
            of_node_put(clkspec.np);
            rc = 0;
            clk_put(pclk);
            return rc;
        }
        clk = of_clk_get_from_provider(&mut clkspec);
        of_node_put(clkspec.np);
        if IS_ERR(clk) {
            if PTR_ERR(clk) != -EPROBE_DEFER {
                pr_warn!("clk: couldn't get assigned clock %d for %pOF\n", index, node);
            }
            rc = PTR_ERR(clk);
            clk_put(pclk);
            return rc;
        }

        rc = clk_set_parent(clk, pclk);
        if rc < 0 {
            pr_err!(
                "clk: failed to reparent %s to %s: %d\n",
                __clk_get_name(clk),
                __clk_get_name(pclk),
                rc,
            );
        }
        clk_put(clk);
        clk_put(pclk);
        index += 1;
    }
    return 0;
}

unsafe fn __set_clk_rates(node: *mut device_node, clk_supplier: bool) -> i32 {
    let mut clkspec: of_phandle_args = core::mem::zeroed();
    let mut rc: i32;
    let mut count: i32;
    let count_64: i32;
    let mut index: i32;
    let mut clk: *mut clk;
    let mut rates_64: *mut u64 = core::ptr::null_mut();
    let mut rates: *mut u32 = core::ptr::null_mut();

    count = of_property_count_u32_elems(node, b"assigned-clock-rates\0".as_ptr() as *const i8);
    count_64 = of_property_count_u64_elems(node, b"assigned-clock-rates-u64\0".as_ptr() as *const i8);
    if count_64 > 0 {
        count = count_64;
        rates_64 = kcalloc(count as usize, core::mem::size_of::<u64>(), GFP_KERNEL) as *mut u64;
        if rates_64.is_null() { return -ENOMEM; }
        rc = of_property_read_u64_array(node, b"assigned-clock-rates-u64\0".as_ptr() as *const i8, rates_64, count as usize);
    } else if count > 0 {
        rates = kcalloc(count as usize, core::mem::size_of::<u32>(), GFP_KERNEL) as *mut u32;
        if rates.is_null() { return -ENOMEM; }
        rc = of_property_read_u32_array(node, b"assigned-clock-rates\0".as_ptr() as *const i8, rates, count as usize);
    } else { return 0; }
    if rc != 0 { kfree(rates_64 as *mut core::ffi::c_void); kfree(rates as *mut core::ffi::c_void); return rc; }

    index = 0;
    while index < count {
        let rate: u64 = if !rates_64.is_null() { *rates_64.add(index as usize) } else { *rates.add(index as usize) as u64 };
        if rate != 0 {
            rc = of_parse_phandle_with_args(node, b"assigned-clocks\0".as_ptr() as *const i8, b"#clock-cells\0".as_ptr() as *const i8, index, &mut clkspec);
            if rc < 0 { if rc == -ENOENT { index += 1; continue; } else { kfree(rates_64 as *mut core::ffi::c_void); kfree(rates as *mut core::ffi::c_void); return rc; } }
            if clkspec.np == node && !clk_supplier { of_node_put(clkspec.np); kfree(rates_64 as *mut core::ffi::c_void); kfree(rates as *mut core::ffi::c_void); return 0; }
            clk = of_clk_get_from_provider(&mut clkspec); of_node_put(clkspec.np);
            if IS_ERR(clk) { if PTR_ERR(clk) != -EPROBE_DEFER { pr_warn!("clk: couldn't get clock %d for %pOF\n", index, node); } rc = PTR_ERR(clk); kfree(rates_64 as *mut core::ffi::c_void); kfree(rates as *mut core::ffi::c_void); return rc; }
            rc = clk_set_rate(clk, rate as _);
            if rc < 0 { pr_err!("clk: couldn't set %s clk rate to %lu (%d), current rate: %lu\n", __clk_get_name(clk), rate, rc, clk_get_rate(clk)); }
            clk_put(clk);
        }
        index += 1;
    }
    kfree(rates_64 as *mut core::ffi::c_void); kfree(rates as *mut core::ffi::c_void); 0
}

unsafe fn __set_clk_spread_spectrum(node: *mut device_node, clk_supplier: bool) -> i32 {
    let elem_size = core::mem::size_of::<clk_spread_spectrum>();
    let mut clkspec: of_phandle_args = core::mem::zeroed();
    let count = of_property_count_elems_of_size(node, b"assigned-clock-sscs\0".as_ptr() as *const i8, elem_size);
    if count <= 0 { return 0; }
    let sscs = kcalloc(count as usize, elem_size, GFP_KERNEL) as *mut clk_spread_spectrum;
    if sscs.is_null() { return -ENOMEM; }
    let mut rc = of_property_read_u32_array(node, b"assigned-clock-sscs\0".as_ptr() as *const i8, sscs as *mut u32, (count * 3) as usize);
    if rc != 0 { kfree(sscs as *mut core::ffi::c_void); return rc; }
    for index in 0..count {
        let conf = &mut *sscs.add(index as usize);
        if conf.modfreq_hz == 0 && conf.spread_bp == 0 && conf.method == 0 { continue; }
        rc = of_parse_phandle_with_args(node, b"assigned-clocks\0".as_ptr() as *const i8, b"#clock-cells\0".as_ptr() as *const i8, index, &mut clkspec);
        if rc < 0 { if rc == -ENOENT { rc = 0; continue; } else { break; } }
        if clkspec.np == node && !clk_supplier { of_node_put(clkspec.np); break; }
        let clk = of_clk_get_from_provider(&mut clkspec); of_node_put(clkspec.np);
        if IS_ERR(clk) { if PTR_ERR(clk) != -EPROBE_DEFER { pr_warn!("clk: couldn't get clock %d for %pOF\n", index, node); } rc = PTR_ERR(clk); break; }
        let hw = __clk_get_hw(clk);
        rc = clk_hw_set_spread_spectrum(hw, conf);
        if rc < 0 { pr_err!("clk: couldn't set %s clk spread spectrum %u %u %u: %d\n", __clk_get_name(clk), conf.modfreq_hz, conf.spread_bp, conf.method, rc); rc = 0; }
        clk_put(clk);
    }
    kfree(sscs as *mut core::ffi::c_void); rc
}

/**
 * of_clk_set_defaults() - parse and set assigned clocks configuration
 * @node: device node to apply clock settings for
 * @clk_supplier: true if clocks supplied by @node should also be considered
 *
 * This function parses 'assigned-{clocks/clock-parents/clock-rates}' properties
 * and sets any specified clock parents and rates. The @clk_supplier argument
 * should be set to true if @node may be also a clock supplier of any clock
 * listed in its 'assigned-clocks' or 'assigned-clock-parents' properties.
 * If @clk_supplier is false the function exits returning 0 as soon as it
 * determines the @node is also a supplier of any of the clocks.
 */
pub unsafe fn of_clk_set_defaults(node: *mut device_node, clk_supplier: bool) -> i32 {
    if node.is_null() { return 0; }
    let mut rc = __set_clk_spread_spectrum(node, clk_supplier);
    if rc < 0 { return rc; }
    rc = __set_clk_parents(node, clk_supplier);
    if rc < 0 { return rc; }
    __set_clk_rates(node, clk_supplier)
}

// EXPORT_SYMBOL_GPL(of_clk_set_defaults);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
