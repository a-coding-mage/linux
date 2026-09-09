// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2014 Broadcom Corporation

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct iproc_asiu;

#[repr(C)]
pub struct iproc_asiu_clk {
    pub hw: clk_hw,
    pub name: *const core::ffi::c_char,
    pub asiu: *mut iproc_asiu,
    pub rate: usize,
    pub div: iproc_asiu_div,
    pub gate: iproc_asiu_gate,
}

#[repr(C)]
pub struct iproc_asiu {
    pub div_base: *mut core::ffi::c_void,
    pub gate_base: *mut core::ffi::c_void,
    pub clks: [iproc_asiu_clk; 0],
}

extern "C" {
    pub fn readl(addr: *mut core::ffi::c_void) -> u32;
    pub fn writel(value: u32, addr: *mut core::ffi::c_void);
    pub fn pr_debug(fmt: *const core::ffi::c_char, ...);
    pub fn of_iomap(node: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    pub fn of_property_read_string_index(
        node: *mut device_node,
        property: *const core::ffi::c_char,
        index: u32,
        output: *mut *const core::ffi::c_char,
    ) -> i32;
    pub fn of_clk_get_parent_name(node: *mut device_node, index: i32) -> *const core::ffi::c_char;
    pub fn clk_hw_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> i32;
    pub fn clk_hw_unregister(hw: *mut clk_hw);
    pub fn of_clk_add_hw_provider(
        node: *mut device_node,
        get: unsafe extern "C" fn(*mut device_node, *const clk_hw_onecell_data) -> *mut clk_hw,
        data: *mut clk_hw_onecell_data,
    ) -> i32;
    pub fn iounmap(addr: *mut core::ffi::c_void);
    pub fn kfree(ptr: *mut core::ffi::c_void);
    pub fn kzalloc_flex<T>(count: usize) -> *mut T;
}

#[repr(C)]
pub struct clk_hw {
    pub init: *mut clk_init_data,
}
#[repr(C)]
pub struct clk_init_data {
    pub name: *const core::ffi::c_char,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub parent_names: *const *const core::ffi::c_char,
    pub num_parents: u8,
}
#[repr(C)]
pub struct clk_ops {
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize, usize) -> i32>,
}
#[repr(C)]
pub struct clk_rate_request { pub rate: usize, pub best_parent_rate: usize }
#[repr(C)]
pub struct clk_hw_onecell_data { pub num: u32, pub hws: [*mut clk_hw; 0] }
pub struct device_node;
pub struct iproc_asiu_div { pub offset: usize, pub en_shift: u32, pub high_shift: u32, pub high_width: u32, pub low_shift: u32, pub low_width: u32 }
pub struct iproc_asiu_gate { pub offset: usize, pub en_shift: u32 }

const IPROC_CLK_INVALID_OFFSET: usize = usize::MAX;

unsafe fn to_asiu_clk(hw: *mut clk_hw) -> *mut iproc_asiu_clk {
    (hw as *mut u8).sub(core::mem::offset_of!(iproc_asiu_clk, hw)) as *mut iproc_asiu_clk
}
unsafe fn bit_mask(width: u32) -> u32 { if width == 32 { u32::MAX } else { (1u32 << width) - 1 } }

pub unsafe extern "C" fn iproc_asiu_clk_enable(hw: *mut clk_hw) -> i32 {
    let clk = &mut *to_asiu_clk(hw); let asiu = &mut *clk.asiu;
    if clk.gate.offset == IPROC_CLK_INVALID_OFFSET { return 0; }
    let addr = (asiu.gate_base as *mut u8).add(clk.gate.offset) as *mut core::ffi::c_void;
    let val = readl(addr) | (1u32 << clk.gate.en_shift); writel(val, addr); 0
}
pub unsafe extern "C" fn iproc_asiu_clk_disable(hw: *mut clk_hw) {
    let clk = &mut *to_asiu_clk(hw); let asiu = &mut *clk.asiu;
    if clk.gate.offset == IPROC_CLK_INVALID_OFFSET { return; }
    let addr = (asiu.gate_base as *mut u8).add(clk.gate.offset) as *mut core::ffi::c_void;
    writel(readl(addr) & !(1u32 << clk.gate.en_shift), addr);
}

pub unsafe extern "C" fn iproc_asiu_clk_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let clk = &mut *to_asiu_clk(hw); let asiu = &mut *clk.asiu;
    if parent_rate == 0 { clk.rate = 0; return 0; }
    let addr = (asiu.div_base as *mut u8).add(clk.div.offset) as *mut core::ffi::c_void;
    let val = readl(addr);
    if val & (1u32 << clk.div.en_shift) == 0 { clk.rate = parent_rate; return parent_rate; }
    let div_h = ((val >> clk.div.high_shift) & bit_mask(clk.div.high_width)) + 1;
    let div_l = ((val >> clk.div.low_shift) & bit_mask(clk.div.low_width)) + 1;
    clk.rate = parent_rate / ((div_h + div_l) as usize); clk.rate
}

pub unsafe extern "C" fn iproc_asiu_clk_determine_rate(_: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let req = &mut *req; if req.rate == 0 || req.best_parent_rate == 0 { return -22; }
    if req.rate == req.best_parent_rate { return 0; }
    let div = (req.best_parent_rate + req.rate / 2) / req.rate;
    if div < 2 { req.rate = req.best_parent_rate; } else { req.rate = req.best_parent_rate / div; } 0
}

pub unsafe extern "C" fn iproc_asiu_clk_set_rate(hw: *mut clk_hw, rate: usize, parent_rate: usize) -> i32 {
    let clk = &mut *to_asiu_clk(hw); let asiu = &mut *clk.asiu;
    if rate == 0 || parent_rate == 0 { return -22; }
    let addr = (asiu.div_base as *mut u8).add(clk.div.offset) as *mut core::ffi::c_void;
    let mut val = readl(addr);
    if rate == parent_rate { writel(val & !(1u32 << clk.div.en_shift), addr); return 0; }
    let div = (parent_rate + rate / 2) / rate; if div < 2 { return -22; }
    let div_h = (div >> 1) - 1; let div_l = (div >> 1) - 1;
    val |= 1u32 << clk.div.en_shift;
    val &= !(bit_mask(clk.div.high_width) << clk.div.high_shift); val |= (div_h as u32) << clk.div.high_shift;
    val &= !(bit_mask(clk.div.low_width) << clk.div.low_shift); val |= (div_l as u32) << clk.div.low_shift;
    writel(val, addr); 0
}

pub static iproc_asiu_ops: clk_ops = clk_ops { enable: Some(iproc_asiu_clk_enable), disable: Some(iproc_asiu_clk_disable), recalc_rate: Some(iproc_asiu_clk_recalc_rate), determine_rate: Some(iproc_asiu_clk_determine_rate), set_rate: Some(iproc_asiu_clk_set_rate) };

// The setup routine's allocation/provider interfaces depend on the surrounding kernel bindings.
pub unsafe extern "C" fn iproc_asiu_setup(node: *mut device_node, div: *const iproc_asiu_div, gate: *const iproc_asiu_gate, num_clks: u32) {
    if gate.is_null() || div.is_null() { return; }
    let asiu = kzalloc_flex::<iproc_asiu>(num_clks as usize); if asiu.is_null() { return; }
    let clk_data = kzalloc_flex::<clk_hw_onecell_data>(num_clks as usize);
    if clk_data.is_null() { kfree(asiu as *mut _); return; }
    (*clk_data).num = num_clks;
    (*asiu).div_base = of_iomap(node, 0); if (*asiu).div_base.is_null() { kfree(clk_data as *mut _); kfree(asiu as *mut _); return; }
    (*asiu).gate_base = of_iomap(node, 1); if (*asiu).gate_base.is_null() { iounmap((*asiu).div_base); kfree(clk_data as *mut _); kfree(asiu as *mut _); return; }
    let mut i = 0u32;
    while i < num_clks {
        let mut clk_name: *const core::ffi::c_char = core::ptr::null();
        if of_property_read_string_index(node, b"clock-output-names\0".as_ptr() as _, i, &mut clk_name) != 0 { break; }
        let clk = &mut *(((*asiu).clks.as_mut_ptr()).add(i as usize));
        clk.name = clk_name; clk.asiu = asiu; clk.div = core::ptr::read(div.add(i as usize)); clk.gate = core::ptr::read(gate.add(i as usize));
        let parent_name = of_clk_get_parent_name(node, 0);
        let mut init = clk_init_data { name: clk_name, ops: &iproc_asiu_ops, flags: 0, parent_names: if parent_name.is_null() { core::ptr::null() } else { &parent_name }, num_parents: if parent_name.is_null() { 0 } else { 1 } };
        clk.hw.init = &mut init;
        if clk_hw_register(core::ptr::null_mut(), &mut clk.hw) != 0 { break; }
        // Flexible-array storage for hws is supplied by kzalloc_flex in the kernel binding.
        i += 1;
    }
    if i != num_clks { while i > 0 { i -= 1; /* clk_hw_unregister(clk_data->hws[i]) */ } iounmap((*asiu).gate_base); iounmap((*asiu).div_base); kfree(clk_data as *mut _); kfree(asiu as *mut _); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
