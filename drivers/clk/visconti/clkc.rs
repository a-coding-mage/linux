// SPDX-License-Identifier: GPL-2.0-only
/*
 * Toshiba Visconti clock controller
 *
 * Copyright (c) 2021 TOSHIBA CORPORATION
 * Copyright (c) 2021 Toshiba Electronic Devices & Storage Corporation
 *
 * Nobuhiro Iwamatsu <nobuhiro1.iwamatsu@toshiba.co.jp>
 */

// Dependencies supplied by the Linux clock, device, IO, OF, regmap, slab,
// string, and local clock-controller interfaces are intentionally external.

#[inline]
unsafe fn to_visconti_clk_gate(hw: *mut clk_hw) -> *mut visconti_clk_gate {
    container_of(hw, visconti_clk_gate, hw)
}

unsafe fn visconti_gate_clk_is_enabled(hw: *mut clk_hw) -> i32 {
    let gate = to_visconti_clk_gate(hw);
    let clk: u32 = 1u32.wrapping_shl((*gate).ck_idx as u32);
    let mut val: u32 = 0;

    regmap_read((*gate).regmap, (*gate).ckon_offset, &mut val);
    if val & clk != 0 { 1 } else { 0 }
}

unsafe fn visconti_gate_clk_disable(hw: *mut clk_hw) {
    let gate = to_visconti_clk_gate(hw);
    let clk: u32 = 1u32.wrapping_shl((*gate).ck_idx as u32);
    let mut flags: c_ulong = 0;

    spin_lock_irqsave((*gate).lock, &mut flags);

    if visconti_gate_clk_is_enabled(hw) == 0 {
        spin_unlock_irqrestore((*gate).lock, flags);
        return;
    }

    regmap_update_bits((*gate).regmap, (*gate).ckoff_offset, clk, clk);
    spin_unlock_irqrestore((*gate).lock, flags);
}

unsafe fn visconti_gate_clk_enable(hw: *mut clk_hw) -> i32 {
    let gate = to_visconti_clk_gate(hw);
    let clk: u32 = 1u32.wrapping_shl((*gate).ck_idx as u32);
    let mut flags: c_ulong = 0;

    spin_lock_irqsave((*gate).lock, &mut flags);
    regmap_update_bits((*gate).regmap, (*gate).ckon_offset, clk, clk);
    spin_unlock_irqrestore((*gate).lock, flags);

    0
}

static visconti_clk_gate_ops: clk_ops = clk_ops {
    enable: Some(visconti_gate_clk_enable),
    disable: Some(visconti_gate_clk_disable),
    is_enabled: Some(visconti_gate_clk_is_enabled),
};

unsafe fn visconti_clk_register_gate(
    dev: *mut device,
    name: *const c_char,
    parent_name: *const c_char,
    regmap: *mut regmap,
    clks: *const visconti_clk_gate_table,
    rson_offset: u32,
    rsoff_offset: u32,
    rs_idx: u8,
    lock: *mut spinlock_t,
) -> *mut clk_hw {
    let mut init: clk_init_data = core::mem::zeroed();
    let gate: *mut visconti_clk_gate;
    let pdata: *mut clk_parent_data;
    let mut hw: *mut clk_hw;
    let mut ret: i32;

    pdata = devm_kzalloc(dev, core::mem::size_of::<clk_parent_data>(), GFP_KERNEL);
    if pdata.is_null() { return ERR_PTR(-ENOMEM); }

    (*pdata).name = parent_name;
    (*pdata).fw_name = parent_name;

    gate = devm_kzalloc(dev, core::mem::size_of::<visconti_clk_gate>(), GFP_KERNEL);
    if gate.is_null() { return ERR_PTR(-ENOMEM); }

    init.name = name;
    init.ops = &visconti_clk_gate_ops;
    init.flags = (*clks).flags;
    init.parent_data = pdata;
    init.num_parents = 1;

    (*gate).regmap = regmap;
    (*gate).ckon_offset = (*clks).ckon_offset;
    (*gate).ckoff_offset = (*clks).ckoff_offset;
    (*gate).ck_idx = (*clks).ck_idx;
    (*gate).rson_offset = rson_offset;
    (*gate).rsoff_offset = rsoff_offset;
    (*gate).rs_idx = rs_idx;
    (*gate).lock = lock;
    (*gate).hw.init = &init;

    hw = &mut (*gate).hw;
    ret = devm_clk_hw_register(dev, hw);
    if ret != 0 { hw = ERR_PTR(ret); }

    hw
}

unsafe fn visconti_clk_register_gates(
    ctx: *mut visconti_clk_provider,
    clks: *const visconti_clk_gate_table,
    num_gate: i32,
    reset: *const visconti_reset_data,
    lock: *mut spinlock_t,
) -> i32 {
    let dev = (*ctx).dev;
    let mut i = 0;

    while i < num_gate {
        let clk = clks.add(i as usize);
        let parent_div_name = (*(*clk).parent_data).name;
        let pdata: *mut clk_parent_data;
        let (rson_offset, rsoff_offset, rs_idx): (u32, u32, u8);
        let gate_clk: *mut clk_hw;
        let div_clk: *mut clk_hw;
        let dev_name: *mut c_char;

        pdata = devm_kzalloc(dev, core::mem::size_of::<clk_parent_data>(), GFP_KERNEL);
        if pdata.is_null() { return -ENOMEM; }

        dev_name = devm_kasprintf(dev, GFP_KERNEL, b"%s_div\0".as_ptr() as *const c_char, (*clk).name);
        if dev_name.is_null() { return -ENOMEM; }

        if (*clk).rs_id != NO_RESET {
            let rs = reset.add((*clk).rs_id as usize);
            rson_offset = (*rs).rson_offset;
            rsoff_offset = (*rs).rsoff_offset;
            rs_idx = (*rs).rs_idx;
        } else {
            rson_offset = u32::MAX;
            rsoff_offset = u32::MAX;
            rs_idx = u8::MAX;
        }

        div_clk = devm_clk_hw_register_fixed_factor(dev, dev_name, parent_div_name, 0, 1, (*clk).div);
        if IS_ERR(div_clk) { return PTR_ERR(div_clk); }

        gate_clk = visconti_clk_register_gate(dev, (*clk).name, dev_name, (*ctx).regmap, clk,
                                              rson_offset, rsoff_offset, rs_idx, lock);
        if IS_ERR(gate_clk) {
            dev_err(dev, b"visconti_clk_register_gates: failed to register clock %s\n\0".as_ptr() as *const c_char, (*clk).name);
            return PTR_ERR(gate_clk);
        }

        *(*ctx).clk_data.hws.add((*clk).id as usize) = gate_clk;
        i += 1;
    }

    0
}

unsafe fn visconti_init_clk(
    dev: *mut device,
    regmap: *mut regmap,
    nr_clks: c_ulong,
) -> *mut visconti_clk_provider {
    let ctx = devm_kzalloc(dev, struct_size_clk_provider(nr_clks), GFP_KERNEL);
    if ctx.is_null() { return ERR_PTR(-ENOMEM); }

    for i in 0..nr_clks {
        *(*ctx).clk_data.hws.add(i as usize) = ERR_PTR(-ENOENT);
    }
    (*ctx).clk_data.num = nr_clks;
    (*ctx).dev = dev;
    (*ctx).regmap = regmap;

    ctx
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
