// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

// Linux kernel dependencies and "clk-krait.h" are supplied by the surrounding
// translation unit.

use core::ffi::{c_char, c_int, c_ulong, c_void};

const CPU0_MUX: usize = 0;
const CPU1_MUX: usize = CPU0_MUX + 1;
const CPU2_MUX: usize = CPU1_MUX + 1;
const CPU3_MUX: usize = CPU2_MUX + 1;
const L2_MUX: usize = CPU3_MUX + 1;
const CLKS_MAX: usize = L2_MUX + 1;

static mut SEC_MUX_MAP: [u32; 2] = [2, 0];
static mut PRI_MUX_MAP: [u32; 3] = [1, 2, 0];

unsafe fn krait_notifier_cb(nb: *mut notifier_block, event: c_ulong, _data: *mut c_void) -> c_int {
    let mut ret: c_int = 0;
    let mux = container_of!(nb, krait_mux_clk, clk_nb);
    if event == PRE_RATE_CHANGE {
        (*mux).old_index = krait_mux_clk_ops.get_parent(&mut (*mux).hw);
        ret = krait_mux_clk_ops.set_parent(&mut (*mux).hw, (*mux).safe_sel);
        (*mux).reparent = false;
    } else if event == POST_RATE_CHANGE {
        if !(*mux).reparent {
            ret = krait_mux_clk_ops.set_parent(&mut (*mux).hw, (*mux).old_index);
        }
    }
    notifier_from_errno(ret)
}

unsafe fn krait_notifier_register(dev: *mut device, clk: *mut clk,
                                  mux: *mut krait_mux_clk) -> c_int {
    let mut ret: c_int = 0;
    (*mux).clk_nb.notifier_call = Some(krait_notifier_cb);
    ret = devm_clk_notifier_register(dev, clk, &mut (*mux).clk_nb);
    if ret != 0 {
        dev_err!(dev, "failed to register clock notifier: %d\n", ret);
    }
    ret
}

unsafe fn krait_add_div(dev: *mut device, id: c_int, s: *const c_char,
                        offset: u32) -> *mut clk_hw {
    let div = devm_kzalloc::<krait_div2_clk>(dev, core::mem::size_of::<krait_div2_clk>(), GFP_KERNEL);
    if div.is_null() { return ERR_PTR(-ENOMEM); }
    (*div).width = 2;
    (*div).shift = 6;
    (*div).lpl = id >= 0;
    (*div).offset = offset;
    (*div).hw.init = &mut clk_init_data { num_parents: 1, ops: &krait_div2_clk_ops,
        flags: CLK_SET_RATE_PARENT, name: kasprintf!(GFP_KERNEL, "hfpll%s_div", s),
        parent_data: core::ptr::null_mut() };
    let mut parent_name = kasprintf!(GFP_KERNEL, "hfpll%s", s);
    if parent_name.is_null() { return ERR_PTR(-ENOMEM); }
    let mut p_data = clk_parent_data { fw_name: parent_name, name: parent_name, hw: core::ptr::null_mut() };
    (*div).hw.init.parent_data = &mut p_data;
    let ret = devm_clk_hw_register(dev, &mut (*div).hw);
    if ret != 0 { kfree(parent_name); return ERR_PTR(ret); }
    if id < 0 {
        for_each_online_cpu!(cpu, { clk_prepare_enable((*div).hw.clk); });
    } else { clk_prepare_enable((*div).hw.clk); }
    kfree(parent_name);
    &mut (*div).hw
}

unsafe fn krait_add_sec_mux(dev: *mut device, id: c_int, s: *const c_char,
                            offset: u32, unique_aux: bool) -> *mut clk_hw {
    let mux = devm_kzalloc::<krait_mux_clk>(dev, core::mem::size_of::<krait_mux_clk>(), GFP_KERNEL);
    if mux.is_null() { return ERR_PTR(-ENOMEM); }
    (*mux).offset = offset; (*mux).lpl = id >= 0; (*mux).mask = 0x3; (*mux).shift = 2;
    (*mux).parent_map = SEC_MUX_MAP.as_mut_ptr(); (*mux).safe_sel = 0;
    if of_machine_is_compatible(c"qcom,ipq8064") || of_machine_is_compatible(c"qcom,apq8064") {
        (*mux).disable_sec_src_gating = true;
    }
    let name = kasprintf!(GFP_KERNEL, "krait%s_sec_mux", s);
    if name.is_null() { return ERR_PTR(-ENOMEM); }
    let parent_name = if unique_aux { kasprintf!(GFP_KERNEL, "acpu%s_aux", s) } else { c"apu_aux".as_ptr() as *mut c_char };
    if parent_name.is_null() { kfree(name); return ERR_PTR(-ENOMEM); }
    let ret = devm_clk_hw_register(dev, &mut (*mux).hw);
    if ret != 0 { if unique_aux { kfree(parent_name); } kfree(name); return ERR_PTR(ret); }
    let ret = krait_notifier_register(dev, (*mux).hw.clk, mux);
    if ret != 0 { if unique_aux { kfree(parent_name); } kfree(name); return ERR_PTR(ret); }
    if id < 0 { for_each_online_cpu!(cpu, { clk_prepare_enable((*mux).hw.clk); }); }
    else { clk_prepare_enable((*mux).hw.clk); }
    if unique_aux { kfree(parent_name); } kfree(name);
    &mut (*mux).hw
}

unsafe fn krait_add_pri_mux(dev: *mut device, hfpll_div: *mut clk_hw, sec_mux: *mut clk_hw,
                            id: c_int, s: *const c_char, offset: u32) -> *mut clk_hw {
    let mux = devm_kzalloc::<krait_mux_clk>(dev, core::mem::size_of::<krait_mux_clk>(), GFP_KERNEL);
    if mux.is_null() { return ERR_PTR(-ENOMEM); }
    (*mux).mask = 0x3; (*mux).shift = 0; (*mux).offset = offset; (*mux).lpl = id >= 0;
    (*mux).parent_map = PRI_MUX_MAP.as_mut_ptr(); (*mux).safe_sel = 2;
    let name = kasprintf!(GFP_KERNEL, "krait%s_pri_mux", s);
    if name.is_null() { return ERR_PTR(-ENOMEM); }
    let hfpll_name = kasprintf!(GFP_KERNEL, "hfpll%s", s);
    if hfpll_name.is_null() { kfree(name); return ERR_PTR(-ENOMEM); }
    let ret = devm_clk_hw_register(dev, &mut (*mux).hw);
    if ret != 0 { kfree(hfpll_name); kfree(name); return ERR_PTR(ret); }
    let ret = krait_notifier_register(dev, (*mux).hw.clk, mux);
    if ret != 0 { kfree(hfpll_name); kfree(name); return ERR_PTR(ret); }
    kfree(hfpll_name); kfree(name); &mut (*mux).hw
}

unsafe fn krait_add_clks(dev: *mut device, id: c_int, unique_aux: bool) -> *mut clk_hw {
    let (offset, s, p) = if id >= 0 { (0x4501 + 0x1000 * id as u32, kasprintf!(GFP_KERNEL, "%d", id), true) }
        else { (0x500, c"_l2".as_ptr() as *mut c_char, false) };
    if s.is_null() { return ERR_PTR(-ENOMEM); }
    let div = krait_add_div(dev, id, s, offset); if IS_ERR(div) { if p { kfree(s); } return div; }
    let sec = krait_add_sec_mux(dev, id, s, offset, unique_aux); if IS_ERR(sec) { if p { kfree(s); } return sec; }
    let pri = krait_add_pri_mux(dev, div, sec, id, s, offset); if p { kfree(s); } pri
}

unsafe fn krait_of_get(clkspec: *mut of_phandle_args, data: *mut c_void) -> *mut clk {
    let idx = (*clkspec).args[0] as usize;
    let clks = data as *mut *mut clk;
    if idx >= CLKS_MAX { pr_err!("krait_of_get: invalid clock index %d\n", idx); return ERR_PTR(-EINVAL); }
    if !(*clks.add(idx)).is_null() { *clks.add(idx) } else { ERR_PTR(-ENODEV) }
}

static KRAIT_CC_MATCH_TABLE: [of_device_id; 3] = [
    of_device_id { compatible: c"qcom,krait-cc-v1".as_ptr(), data: 1 as *const c_void },
    of_device_id { compatible: c"qcom,krait-cc-v2".as_ptr(), data: core::ptr::null() },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe fn krait_cc_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev;
    let unique_aux = !device_get_match_data(dev).is_null();
    let mut clk = clk_register_fixed_rate(dev, c"qsb".as_ptr(), core::ptr::null(), 0, 1);
    if IS_ERR(clk) { return PTR_ERR(clk); }
    if !unique_aux { clk = clk_register_fixed_factor(dev, c"acpu_aux".as_ptr(), c"gpll0_vote".as_ptr(), 0, 1, 2); if IS_ERR(clk) { return PTR_ERR(clk); } }
    let clks = devm_kcalloc::<*mut clk>(dev, CLKS_MAX, core::mem::size_of::<*mut clk>(), GFP_KERNEL);
    if clks.is_null() { return -ENOMEM; }
    for_each_possible_cpu!(cpu, { let mux = krait_add_clks(dev, cpu, unique_aux); if IS_ERR(mux) { return PTR_ERR(mux); } *clks.add(cpu as usize) = (*mux).clk; });
    let l2 = krait_add_clks(dev, -1, unique_aux); if IS_ERR(l2) { return PTR_ERR(l2); } *clks.add(L2_MUX) = (*l2).clk;
    for_each_online_cpu!(cpu, { clk_prepare_enable(*clks.add(L2_MUX)); WARN!(clk_prepare_enable(*clks.add(cpu as usize)) != 0, "Unable to turn on CPU%d clock", cpu); });
    let aux_rate: c_ulong = 384000000; let mut cur_rate = clk_get_rate(*clks.add(L2_MUX));
    if cur_rate < aux_rate { pr_info!("L2 @ Undefined rate. Forcing new rate.\n"); cur_rate = aux_rate; }
    clk_set_rate(*clks.add(L2_MUX), aux_rate); clk_set_rate(*clks.add(L2_MUX), 2); clk_set_rate(*clks.add(L2_MUX), cur_rate);
    pr_info!("L2 @ %lu KHz\n", clk_get_rate(*clks.add(L2_MUX)) / 1000);
    for_each_possible_cpu!(cpu, { let c = *clks.add(cpu as usize); cur_rate = clk_get_rate(c); if cur_rate < aux_rate { pr_info!("CPU%d @ Undefined rate. Forcing new rate.\n", cpu); cur_rate = aux_rate; } clk_set_rate(c, aux_rate); clk_set_rate(c, 2); clk_set_rate(c, cur_rate); pr_info!("CPU%d @ %lu KHz\n", cpu, clk_get_rate(c) / 1000); });
    of_clk_add_provider((*dev).of_node, krait_of_get, clks as *mut c_void); 0
}

static mut KRAIT_CC_DRIVER: platform_driver = platform_driver { probe: Some(krait_cc_probe), driver: driver { name: c"krait-cc".as_ptr(), of_match_table: KRAIT_CC_MATCH_TABLE.as_ptr() } };

module_platform_driver!(KRAIT_CC_DRIVER);
module_description!("Krait CPU Clock Driver");
module_license!("GPL v2");
module_alias!("platform:krait-cc");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
