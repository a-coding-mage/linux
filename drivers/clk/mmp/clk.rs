// SPDX-License-Identifier: GPL-2.0
// C dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub unsafe fn mmp_clk_init(np: *mut device_node, unit: *mut mmp_clk_unit, nr_clks: i32) {
    let clk_table: *mut *mut clk = kzalloc_objs::<*mut clk>(nr_clks);
    if clk_table.is_null() {
        return;
    }

    (*unit).clk_table = clk_table;
    (*unit).nr_clks = nr_clks;
    (*unit).clk_data.clks = clk_table;
    (*unit).clk_data.clk_num = nr_clks;
    of_clk_add_provider(np, of_clk_src_onecell_get, &mut (*unit).clk_data);
}

pub unsafe fn mmp_register_fixed_rate_clks(
    unit: *mut mmp_clk_unit,
    clks: *mut mmp_param_fixed_rate_clk,
    size: i32,
) {
    let mut i = 0;
    while i < size {
        let p = &*clks.add(i as usize);
        let clk = clk_register_fixed_rate(
            core::ptr::null_mut(), p.name, p.parent_name, p.flags, p.fixed_rate,
        );
        if IS_ERR(clk) {
            pr_err("%s: failed to register clock %s\n", "mmp_register_fixed_rate_clks", p.name);
            i += 1;
            continue;
        }
        if p.id != 0 {
            (*unit).clk_table.add(p.id as usize).write(clk);
        }
        i += 1;
    }
}

pub unsafe fn mmp_register_fixed_factor_clks(
    unit: *mut mmp_clk_unit,
    clks: *mut mmp_param_fixed_factor_clk,
    size: i32,
) {
    let mut i = 0;
    while i < size {
        let p = &*clks.add(i as usize);
        let clk = clk_register_fixed_factor(
            core::ptr::null_mut(), p.name, p.parent_name, p.flags, p.mult, p.div,
        );
        if IS_ERR(clk) {
            pr_err("%s: failed to register clock %s\n", "mmp_register_fixed_factor_clks", p.name);
            i += 1;
            continue;
        }
        if p.id != 0 {
            (*unit).clk_table.add(p.id as usize).write(clk);
        }
        i += 1;
    }
}

pub unsafe fn mmp_register_general_gate_clks(
    unit: *mut mmp_clk_unit, clks: *mut mmp_param_general_gate_clk,
    base: *mut core::ffi::c_void, size: i32,
) {
    let mut i = 0;
    while i < size {
        let p = &*clks.add(i as usize);
        let clk = clk_register_gate(core::ptr::null_mut(), p.name, p.parent_name, p.flags,
            (base as *mut u8).add(p.offset as usize) as *mut _, p.bit_idx, p.gate_flags, p.lock);
        if IS_ERR(clk) { pr_err("%s: failed to register clock %s\n", "mmp_register_general_gate_clks", p.name); i += 1; continue; }
        if p.id != 0 { (*unit).clk_table.add(p.id as usize).write(clk); }
        i += 1;
    }
}

pub unsafe fn mmp_register_gate_clks(unit: *mut mmp_clk_unit, clks: *mut mmp_param_gate_clk,
    base: *mut core::ffi::c_void, size: i32) {
    let mut i = 0;
    while i < size { let p = &*clks.add(i as usize);
        let clk = mmp_clk_register_gate(core::ptr::null_mut(), p.name, p.parent_name, p.flags,
            (base as *mut u8).add(p.offset as usize) as *mut _, p.mask, p.val_enable,
            p.val_disable, p.gate_flags, p.lock);
        if IS_ERR(clk) { pr_err("%s: failed to register clock %s\n", "mmp_register_gate_clks", p.name); i += 1; continue; }
        if p.id != 0 { (*unit).clk_table.add(p.id as usize).write(clk); } i += 1;
    }
}

pub unsafe fn mmp_register_mux_clks(unit: *mut mmp_clk_unit, clks: *mut mmp_param_mux_clk,
    base: *mut core::ffi::c_void, size: i32) {
    let mut i = 0;
    while i < size { let p = &*clks.add(i as usize);
        let clk = clk_register_mux(core::ptr::null_mut(), p.name, p.parent_name, p.num_parents,
            p.flags, (base as *mut u8).add(p.offset as usize) as *mut _, p.shift, p.width,
            p.mux_flags, p.lock);
        if IS_ERR(clk) { pr_err("%s: failed to register clock %s\n", "mmp_register_mux_clks", p.name); i += 1; continue; }
        if p.id != 0 { (*unit).clk_table.add(p.id as usize).write(clk); } i += 1;
    }
}

pub unsafe fn mmp_register_div_clks(unit: *mut mmp_clk_unit, clks: *mut mmp_param_div_clk,
    base: *mut core::ffi::c_void, size: i32) {
    let mut i = 0;
    while i < size { let p = &*clks.add(i as usize);
        let clk = clk_register_divider(core::ptr::null_mut(), p.name, p.parent_name, p.flags,
            (base as *mut u8).add(p.offset as usize) as *mut _, p.shift, p.width, p.div_flags, p.lock);
        if IS_ERR(clk) { pr_err("%s: failed to register clock %s\n", "mmp_register_div_clks", p.name); i += 1; continue; }
        if p.id != 0 { (*unit).clk_table.add(p.id as usize).write(clk); } i += 1;
    }
}

pub unsafe fn mmp_clk_add(unit: *mut mmp_clk_unit, id: u32, clk: *mut clk) {
    if IS_ERR_OR_NULL(clk) { pr_err("CLK %d has invalid pointer %p\n", id, clk); return; }
    if id >= (*unit).nr_clks as u32 { pr_err("CLK %d is invalid\n", id); return; }
    (*unit).clk_table.add(id as usize).write(clk);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
