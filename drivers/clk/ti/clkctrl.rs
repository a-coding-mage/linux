// SPDX-License-Identifier: GPL-2.0-only
/* OMAP clkctrl clock support (translated from clkctrl.c). */

const NO_IDLEST: usize = 0;
const OMAP4_MODULEMODE_MASK: u32 = 0x3;
const MODULEMODE_HWCTRL: u32 = 0x1;
const MODULEMODE_SWCTRL: u32 = 0x2;
const OMAP4_IDLEST_MASK: u32 = 0x3 << 16;
const OMAP4_IDLEST_SHIFT: u32 = 16;
const OMAP4_STBYST_MASK: u32 = 1 << 18;
const OMAP4_STBYST_SHIFT: u32 = 18;
const CLKCTRL_IDLEST_FUNCTIONAL: u32 = 0x0;
const CLKCTRL_IDLEST_INTERFACE_IDLE: u32 = 0x2;
const CLKCTRL_IDLEST_DISABLED: u32 = 0x3;
const OMAP4_MAX_MODULE_READY_TIME: u32 = 2000;
const OMAP4_MAX_MODULE_DISABLE_TIME: u32 = 5000;

static mut _early_timeout: bool = true;

#[repr(C)]
struct omap_clkctrl_provider {
    base: *mut core::ffi::c_void,
    clocks: list_head,
    clkdm_name: *mut core::ffi::c_char,
}
#[repr(C)]
struct omap_clkctrl_clk {
    clk: *mut clk_hw,
    reg_offset: u16,
    bit_offset: i32,
    node: list_head,
}
#[repr(C)]
union omap4_timeout {
    cycles: u32,
    start: ktime_t,
}

static default_clkctrl_data: [omap_clkctrl_data; 1] = [omap_clkctrl_data { addr: 0 }];

unsafe fn _omap4_idlest(mut val: u32) -> u32 {
    val &= OMAP4_IDLEST_MASK;
    val >>= OMAP4_IDLEST_SHIFT;
    val
}
unsafe fn _omap4_is_idle(val: u32) -> bool { _omap4_idlest(val) == CLKCTRL_IDLEST_DISABLED }
unsafe fn _omap4_is_ready(val: u32) -> bool {
    let v = _omap4_idlest(val);
    v == CLKCTRL_IDLEST_FUNCTIONAL || v == CLKCTRL_IDLEST_INTERFACE_IDLE
}
unsafe fn _omap4_is_timeout(time: *mut omap4_timeout, timeout: u32) -> bool {
    if _early_timeout || timekeeping_suspended {
        let cycles = (*time).cycles;
        (*time).cycles = cycles.wrapping_add(1);
        if cycles < timeout { udelay(1 * 2); return false; }
    } else {
        if ktime_to_ns((*time).start) == 0 { (*time).start = ktime_get(); return false; }
        if ktime_us_delta(ktime_get(), (*time).start) < timeout as i64 { cpu_relax(); return false; }
    }
    true
}
unsafe extern "C" fn _omap4_disable_early_timeout() -> i32 { _early_timeout = false; 0 }

unsafe fn _omap4_clkctrl_clk_enable(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_hw_omap(hw); let mut timeout = omap4_timeout { cycles: 0 }; let mut val;
    if !(*clk).clkdm.is_null() {
        let ret = (*ti_clk_ll_ops).clkdm_clk_enable((*clk).clkdm, (*hw).clk);
        if ret != 0 { WARN(1, c"%s: could not enable %s's clockdomain %s: %d\n", c"_omap4_clkctrl_clk_enable", clk_hw_get_name(hw), (*clk).clkdm_name, ret); return ret; }
    }
    if (*clk).enable_bit == 0 { return 0; }
    val = (*ti_clk_ll_ops).clk_readl(&(*clk).enable_reg);
    val = (val & !OMAP4_MODULEMODE_MASK) | (*clk).enable_bit as u32;
    (*ti_clk_ll_ops).clk_writel(val, &(*clk).enable_reg);
    if test_bit(NO_IDLEST, &(*clk).flags) { return 0; }
    while !_omap4_is_ready((*ti_clk_ll_ops).clk_readl(&(*clk).enable_reg)) {
        if _omap4_is_timeout(&mut timeout, OMAP4_MAX_MODULE_READY_TIME) { pr_err(c"%s: failed to enable\n", clk_hw_get_name(hw)); return -EBUSY; }
    }
    0
}
unsafe fn _omap4_clkctrl_clk_disable(hw: *mut clk_hw) {
    let clk = to_clk_hw_omap(hw); let mut timeout = omap4_timeout { cycles: 0 };
    if (*clk).enable_bit != 0 {
        let mut val = (*ti_clk_ll_ops).clk_readl(&(*clk).enable_reg);
        val &= !OMAP4_MODULEMODE_MASK; (*ti_clk_ll_ops).clk_writel(val, &(*clk).enable_reg);
        if !test_bit(NO_IDLEST, &(*clk).flags) { while !_omap4_is_idle((*ti_clk_ll_ops).clk_readl(&(*clk).enable_reg)) { if _omap4_is_timeout(&mut timeout, OMAP4_MAX_MODULE_DISABLE_TIME) { pr_err(c"%s: failed to disable\n", clk_hw_get_name(hw)); break; } } }
    }
    if !(*clk).clkdm.is_null() { (*ti_clk_ll_ops).clkdm_clk_disable((*clk).clkdm, (*hw).clk); }
}
unsafe fn _omap4_clkctrl_clk_is_enabled(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_hw_omap(hw); if (*ti_clk_ll_ops).clk_readl(&(*clk).enable_reg) & (*clk).enable_bit as u32 != 0 { 1 } else { 0 }
}

static omap4_clkctrl_clk_ops: clk_ops = clk_ops { enable: Some(_omap4_clkctrl_clk_enable), disable: Some(_omap4_clkctrl_clk_disable), is_enabled: Some(_omap4_clkctrl_clk_is_enabled), init: Some(omap2_init_clk_clkdm) };

unsafe fn _ti_omap4_clkctrl_xlate(clkspec: *mut of_phandle_args, data: *mut core::ffi::c_void) -> *mut clk_hw {
    let provider = data as *mut omap_clkctrl_provider;
    if (*clkspec).args_count != 2 { return ERR_PTR(-EINVAL); }
    let mut iter = (*provider).clocks.next as *mut omap_clkctrl_clk;
    while !iter.is_null() {
        if (*iter).reg_offset as u32 == (*clkspec).args[0] && (*iter).bit_offset == (*clkspec).args[1] as i32 { return (*iter).clk; }
        iter = (*iter).node.next as *mut omap_clkctrl_clk;
    }
    ERR_PTR(-EINVAL)
}

unsafe fn clkctrl_get_clock_name(np: *mut device_node, clkctrl_name: *const core::ffi::c_char, offset: i32, index: i32, legacy: bool) -> *mut core::ffi::c_char {
    if !clkctrl_name.is_null() && !legacy { return kasprintf(GFP_KERNEL, c"%s-clkctrl:%04x:%d", clkctrl_name, offset, index); }
    if !clkctrl_name.is_null() { return kasprintf(GFP_KERNEL, c"%s_cm:clk:%04x:%d", clkctrl_name, offset, index); }
    if legacy { return kasprintf(GFP_KERNEL, c"%pOFn:clk:%04x:%d", (*np).parent, offset, index); }
    kasprintf(GFP_KERNEL, c"%pOFn:%04x:%d", np, offset, index)
}

unsafe fn _ti_clkctrl_clk_register(provider: *mut omap_clkctrl_provider, node: *mut device_node, clk_hw: *mut clk_hw, offset: u16, bit: u8, parents: *const *const core::ffi::c_char, num_parents: i32, ops: *const clk_ops, name: *const core::ffi::c_char) -> i32 {
    let mut init = clk_init_data::default(); let entry = kzalloc::<omap_clkctrl_clk>();
    if name.is_null() || entry.is_null() { if !name.is_null() { kfree(name); } return -ENOMEM; }
    (*clk_hw).init = &mut init; init.name = name; init.parent_names = parents; init.num_parents = num_parents; init.ops = ops;
    let clk = of_ti_clk_register(node, clk_hw, name); if IS_ERR_OR_NULL(clk) { kfree(name); kfree(entry); return -EINVAL; }
    (*entry).reg_offset = offset; (*entry).bit_offset = bit as i32; (*entry).clk = clk_hw; list_add(&mut (*entry).node, &mut (*provider).clocks); 0
}
unsafe fn _ti_clkctrl_setup_gate(provider: *mut omap_clkctrl_provider, node: *mut device_node, offset: u16, data: *const omap_clkctrl_bit_data, reg: *mut core::ffi::c_void, name: *const core::ffi::c_char) {
    let hw = kzalloc::<clk_hw_omap>(); if hw.is_null() { return; } (*hw).enable_bit = (*data).bit; (*hw).enable_reg.ptr = reg;
    if _ti_clkctrl_clk_register(provider, node, &mut (*hw).hw, offset, (*data).bit, (*data).parents, 1, &omap_gate_clk_ops, name) != 0 { kfree(hw); }
}
unsafe fn _ti_clkctrl_setup_mux(provider: *mut omap_clkctrl_provider, node: *mut device_node, offset: u16, data: *const omap_clkctrl_bit_data, reg: *mut core::ffi::c_void, name: *const core::ffi::c_char) {
    let mux = kzalloc::<clk_omap_mux>(); if mux.is_null() { return; } let mut n = 0; let mut p = (*data).parents; while !(*p).is_null() { n += 1; p = p.add(1); }
    (*mux).mask = n; if (*mux).flags & CLK_MUX_INDEX_ONE == 0 { (*mux).mask -= 1; } (*mux).mask = (1 << fls((*mux).mask)) - 1; (*mux).shift = (*data).bit; (*mux).reg.ptr = reg;
    if _ti_clkctrl_clk_register(provider, node, &mut (*mux).hw, offset, (*data).bit, (*data).parents, n, &ti_clk_mux_ops, name) != 0 { kfree(mux); }
}
unsafe fn _ti_clkctrl_setup_div(provider: *mut omap_clkctrl_provider, node: *mut device_node, offset: u16, data: *const omap_clkctrl_bit_data, reg: *mut core::ffi::c_void, name: *const core::ffi::c_char) {
    let div = kzalloc::<clk_omap_divider>(); if div.is_null() { return; } let d = (*data).data as *const omap_clkctrl_div_data; (*div).reg.ptr = reg; (*div).shift = (*data).bit; (*div).flags = (*d).flags;
    let flags = if (*div).flags & CLK_DIVIDER_POWER_OF_TWO != 0 { CLKF_INDEX_POWER_OF_TWO } else { 0 }; if ti_clk_parse_divider_data((*d).dividers as *mut i32, 0, (*d).max_div, flags, div) != 0 { kfree(div); return; }
    if _ti_clkctrl_clk_register(provider, node, &mut (*div).hw, offset, (*data).bit, (*data).parents, 1, &ti_clk_divider_ops, name) != 0 { kfree(div); }
}
unsafe fn _ti_clkctrl_setup_subclks(provider: *mut omap_clkctrl_provider, node: *mut device_node, data: *const omap_clkctrl_reg_data, reg: *mut core::ffi::c_void, name: *const core::ffi::c_char) {
    let mut bits = (*data).bit_data; if bits.is_null() { return; } while (*bits).bit != 0 { match (*bits).type_ { TI_CLK_GATE => _ti_clkctrl_setup_gate(provider,node,(*data).offset,bits,reg,name), TI_CLK_DIVIDER => _ti_clkctrl_setup_div(provider,node,(*data).offset,bits,reg,name), TI_CLK_MUX => _ti_clkctrl_setup_mux(provider,node,(*data).offset,bits,reg,name), _ => return } bits = bits.add(1); }
}
unsafe fn clkctrl_get_name(np: *mut device_node) -> *mut core::ffi::c_char {
    let mut output: *const core::ffi::c_char = core::ptr::null();
    if of_property_read_string_index(np, c"clock-output-names", 0, &mut output) == 0 {
        let mut len = strlen(output); if let Some(end) = strstr(output, c"_clkctrl") { len = end.offset_from(output) as usize; }
        return kstrndup(output, len, GFP_KERNEL);
    }
    core::ptr::null_mut()
}
unsafe fn _ti_omap4_clkctrl_setup(node: *mut device_node) {
    let mut res = resource::default(); of_address_to_resource(node, 0, &mut res);
    let addr = res.start as u32; let data = default_clkctrl_data.as_ptr();
    if addr != (*data).addr { pr_err(c"%pOF not found from clkctrl data.\n", node); return; }
    let provider = kzalloc::<omap_clkctrl_provider>(); if provider.is_null() { return; }
    (*provider).base = of_iomap(node, 0); INIT_LIST_HEAD(&mut (*provider).clocks);
    let name = clkctrl_get_name(node); (*provider).clkdm_name = name;
    let ret = of_clk_add_hw_provider(node, _ti_omap4_clkctrl_xlate, provider as *mut _);
    if ret == -EPROBE_DEFER { ti_clk_retry_init(node, provider as *mut _, _clkctrl_add_provider); }
}
unsafe fn _clkctrl_add_provider(data: *mut core::ffi::c_void, np: *mut device_node) { of_clk_add_hw_provider(np, _ti_omap4_clkctrl_xlate, data); }

pub unsafe fn ti_clk_is_in_standby(clk: *mut clk) -> bool {
    let hw = __clk_get_hw(clk); if !omap2_clk_is_hw_omap(hw) { return false; }
    let hwclk = to_clk_hw_omap(hw); (*ti_clk_ll_ops).clk_readl(&(*hwclk).enable_reg) & OMAP4_STBYST_MASK != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
