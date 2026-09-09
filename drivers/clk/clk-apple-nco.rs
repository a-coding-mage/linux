// SPDX-License-Identifier: GPL-2.0-only OR MIT
/*
 * Driver for an SoC block (Numerically Controlled Oscillator)
 * found on t8103 (M1) and other Apple chips
 *
 * Copyright (C) The Asahi Linux Contributors
 */

// Linux kernel dependencies supplied by the surrounding translation.

const NCO_CHANNEL_STRIDE: usize = 0x4000;
const NCO_CHANNEL_REGSIZE: usize = 20;

const REG_CTRL: usize = 0;
const CTRL_ENABLE: u32 = 1u32 << 31;
const REG_DIV: usize = 4;
const DIV_FINE: u32 = 0x3;
const DIV_COARSE: u32 = 0x1ffc;
const REG_INC1: usize = 8;
const REG_INC2: usize = 12;
const REG_ACCINIT: usize = 16;

/*
 * Theory of operation (postulated)
 *
 * The REG_DIV register indirectly expresses a base integer divisor, roughly
 * corresponding to twice the desired ratio of input to output clock. This
 * base divisor is adjusted on a cycle-by-cycle basis based on the state of a
 * 32-bit phase accumulator to achieve a desired precise clock ratio over the
 * long term.
 *
 * Specifically an output clock cycle is produced after (REG_DIV divisor)/2
 * or (REG_DIV divisor + 1)/2 input cycles, the latter taking effect when top
 * bit of the 32-bit accumulator is set. The accumulator is incremented each
 * produced output cycle, by the value from either REG_INC1 or REG_INC2, which
 * of the two is selected depending again on the accumulator's current top bit.
 *
 * Because the NCO hardware implements counting of input clock cycles in part
 * in a Galois linear-feedback shift register, the higher bits of divisor
 * are programmed into REG_DIV by picking an appropriate LFSR state. See
 * applnco_compute_tables/applnco_div_translate for details on this.
 */

const LFSR_POLY: u32 = 0xa01;
const LFSR_INIT: u32 = 0x7ff;
const LFSR_LEN: usize = 11;
const LFSR_PERIOD: usize = (1usize << LFSR_LEN) - 1;
const LFSR_TBLSIZE: usize = 1usize << LFSR_LEN;

/* The minimal attainable coarse divisor (first value in table) */
const COARSE_DIV_OFFSET: usize = 2;

#[repr(C)]
pub struct applnco_tables {
    pub fwd: [u16; LFSR_TBLSIZE],
    pub inv: [u16; LFSR_TBLSIZE],
}

#[repr(C)]
pub struct applnco_channel {
    pub base: *mut u8,
    pub tbl: *mut applnco_tables,
    pub hw: clk_hw,
    pub lock: spinlock_t,
}

#[allow(non_camel_case_types)]
pub struct clk_hw {
    pub init: *mut clk_init_data,
}
#[allow(non_camel_case_types)]
pub struct spinlock_t;
#[allow(non_camel_case_types)]
pub struct clk_init_data {
    pub name: *const u8,
    pub ops: *const clk_ops,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u32,
    pub flags: u32,
}
#[allow(non_camel_case_types)]
pub struct clk_parent_data { pub index: u32 }
#[allow(non_camel_case_types)]
pub struct clk_rate_request { pub rate: usize, pub best_parent_rate: usize }
#[allow(non_camel_case_types)]
pub struct clk_ops {
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize, usize) -> i32>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
}

unsafe fn to_applnco_channel(hw: *mut clk_hw) -> *mut applnco_channel {
    (hw as *mut u8).sub(core::mem::offset_of!(applnco_channel, hw)) as *mut applnco_channel
}

unsafe extern "C" fn applnco_enable_nolock(hw: *mut clk_hw) {
    let chan = to_applnco_channel(hw);
    let val = readl_relaxed((*chan).base.add(REG_CTRL));
    writel_relaxed(val | CTRL_ENABLE, (*chan).base.add(REG_CTRL));
}

unsafe extern "C" fn applnco_disable_nolock(hw: *mut clk_hw) {
    let chan = to_applnco_channel(hw);
    let val = readl_relaxed((*chan).base.add(REG_CTRL));
    writel_relaxed(val & !CTRL_ENABLE, (*chan).base.add(REG_CTRL));
}

unsafe extern "C" fn applnco_is_enabled(hw: *mut clk_hw) -> i32 {
    let chan = to_applnco_channel(hw);
    (readl_relaxed((*chan).base.add(REG_CTRL)) & CTRL_ENABLE != 0) as i32
}

unsafe extern "C" fn applnco_compute_tables(tbl: *mut applnco_tables) {
    let mut state = LFSR_INIT;
    for i in (1..=LFSR_PERIOD).rev() {
        state = if state & 1 != 0 { (state >> 1) ^ (LFSR_POLY >> 1) } else { state >> 1 };
        (*tbl).fwd[i] = state as u16;
        (*tbl).inv[state as usize] = i as u16;
    }
    (*tbl).fwd[0] = 0;
    (*tbl).inv[0] = 0;
}

unsafe fn applnco_div_out_of_range(div: usize) -> bool {
    let coarse = div / 4;
    coarse < COARSE_DIV_OFFSET || coarse >= COARSE_DIV_OFFSET + LFSR_TBLSIZE
}

unsafe fn applnco_div_translate(tbl: *mut applnco_tables, div: usize) -> u32 {
    let coarse = div / 4;
    if applnco_div_out_of_range(div) { return 0; }
    (((*tbl).fwd[coarse - COARSE_DIV_OFFSET] as u32) << 2) | (div % 4) as u32
}

unsafe fn applnco_div_translate_inv(tbl: *mut applnco_tables, regval: u32) -> usize {
    let coarse = (*tbl).inv[((regval & DIV_COARSE) >> 2) as usize] as usize + COARSE_DIV_OFFSET;
    coarse * 4 + (regval & DIV_FINE) as usize
}

unsafe extern "C" fn applnco_set_rate(hw: *mut clk_hw, rate: usize, parent_rate: usize) -> i32 {
    let chan = to_applnco_channel(hw);
    let div = 2 * parent_rate / rate;
    let inc1 = 2 * parent_rate - div * rate;
    let inc2 = inc1.wrapping_sub(rate);
    if applnco_div_out_of_range(div) { return -22; }
    let div = applnco_div_translate((*chan).tbl, div);
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*chan).lock, &mut flags);
    let was_enabled = applnco_is_enabled(hw) != 0;
    applnco_disable_nolock(hw);
    writel_relaxed(div, (*chan).base.add(REG_DIV));
    writel_relaxed(inc1 as u32, (*chan).base.add(REG_INC1));
    writel_relaxed(inc2 as u32, (*chan).base.add(REG_INC2));
    writel_relaxed(1u32 << 31, (*chan).base.add(REG_ACCINIT));
    if was_enabled { applnco_enable_nolock(hw); }
    spin_unlock_irqrestore(&mut (*chan).lock, flags);
    0
}

unsafe extern "C" fn applnco_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let chan = to_applnco_channel(hw);
    let div = applnco_div_translate_inv((*chan).tbl, readl_relaxed((*chan).base.add(REG_DIV)));
    let inc1 = readl_relaxed((*chan).base.add(REG_INC1));
    let inc2 = readl_relaxed((*chan).base.add(REG_INC2));
    if inc1 >= (1u32 << 31) || inc2 < (1u32 << 31) || (inc1 == 0 && inc2 == 0) { return 0; }
    let incbase = inc1 - inc2;
    (((parent_rate as u128) * 2 * incbase as u128) / (div as u128 * incbase as u128 + inc1 as u128)) as usize
}

unsafe extern "C" fn applnco_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let lo = (*req).best_parent_rate / (COARSE_DIV_OFFSET + LFSR_TBLSIZE) + 1;
    let hi = (*req).best_parent_rate / COARSE_DIV_OFFSET;
    (*req).rate = (*req).rate.clamp(lo, hi);
    0
}

unsafe extern "C" fn applnco_enable(hw: *mut clk_hw) -> i32 {
    let chan = to_applnco_channel(hw); let mut flags = 0usize;
    spin_lock_irqsave(&mut (*chan).lock, &mut flags); applnco_enable_nolock(hw); spin_unlock_irqrestore(&mut (*chan).lock, flags); 0
}

unsafe extern "C" fn applnco_disable(hw: *mut clk_hw) {
    let chan = to_applnco_channel(hw); let mut flags = 0usize;
    spin_lock_irqsave(&mut (*chan).lock, &mut flags); applnco_disable_nolock(hw); spin_unlock_irqrestore(&mut (*chan).lock, flags);
}

static applnco_ops: clk_ops = clk_ops { set_rate: Some(applnco_set_rate), recalc_rate: Some(applnco_recalc_rate), determine_rate: Some(applnco_determine_rate), enable: Some(applnco_enable), disable: Some(applnco_disable), is_enabled: Some(applnco_is_enabled) };

#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { pub name: *const u8 }
#[repr(C)] pub struct resource { pub start: usize, pub end: usize }
#[repr(C)] pub struct clk_hw_onecell_data { pub num: u32, pub hws: [*mut clk_hw; 0] }
#[repr(C)] pub struct of_device_id { pub compatible: *const u8 }
#[repr(C)] pub struct platform_driver { pub driver: driver, pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32> }
#[repr(C)] pub struct driver { pub name: *const u8, pub of_match_table: *const of_device_id }

unsafe extern "C" fn applnco_probe(pdev: *mut platform_device) -> i32 {
    let np = (*pdev).dev.of_node;
    let mut pdata = clk_parent_data { index: 0 };
    let mut init = clk_init_data { name: core::ptr::null(), ops: core::ptr::null(), parent_data: core::ptr::null(), num_parents: 0, flags: 0 };
    let mut onecell_data: *mut clk_hw_onecell_data;
    let mut base: *mut u8;
    let mut res: *mut resource = core::ptr::null_mut();
    let mut tbl: *mut applnco_tables;
    let nchannels: usize;
    let ret: i32;

    base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if is_err(base) { return ptr_err(base); }
    if resource_size(res) < NCO_CHANNEL_REGSIZE { return -22; }
    nchannels = (resource_size(res) - NCO_CHANNEL_REGSIZE) / NCO_CHANNEL_STRIDE + 1;
    onecell_data = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<clk_hw_onecell_data>() + nchannels * core::mem::size_of::<*mut clk_hw>()) as *mut clk_hw_onecell_data;
    if onecell_data.is_null() { return -12; }
    (*onecell_data).num = nchannels as u32;
    tbl = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<applnco_tables>()) as *mut applnco_tables;
    if tbl.is_null() { return -12; }
    applnco_compute_tables(tbl);
    for i in 0..nchannels {
        let chan = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<applnco_channel>()) as *mut applnco_channel;
        if chan.is_null() { return -12; }
        (*chan).base = base.add(NCO_CHANNEL_STRIDE * i); (*chan).tbl = tbl; spin_lock_init(&mut (*chan).lock);
        init.name = devm_kasprintf(&mut (*pdev).dev, (*np).name, i as i32);
        if init.name.is_null() { return -12; }
        init.ops = &applnco_ops; init.parent_data = &pdata; init.num_parents = 1; init.flags = 0; (*chan).hw.init = &mut init;
        ret = devm_clk_hw_register(&mut (*pdev).dev, &mut (*chan).hw); if ret != 0 { return ret; }
        (*onecell_data).hws[i] = &mut (*chan).hw;
    }
    devm_of_clk_add_hw_provider(&mut (*pdev).dev, onecell_data)
}

static applnco_ids: [of_device_id; 3] = [
    of_device_id { compatible: b"apple,t8103-nco\0".as_ptr() },
    of_device_id { compatible: b"apple,nco\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

static applnco_driver: platform_driver = platform_driver { driver: driver { name: b"apple-nco\0".as_ptr(), of_match_table: applnco_ids.as_ptr() }, probe: Some(applnco_probe) };

// MODULE_DEVICE_TABLE(of, applnco_ids); module_platform_driver(applnco_driver);
// MODULE_AUTHOR("Martin Povišer <povik+lin@cutebit.org>");
// MODULE_DESCRIPTION("Clock driver for NCO blocks on Apple SoCs");
// MODULE_LICENSE("GPL");

extern "C" {
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: i32, res: *mut *mut resource) -> *mut u8;
    fn is_err(ptr: *mut u8) -> bool;
    fn ptr_err(ptr: *mut u8) -> i32;
    fn resource_size(res: *mut resource) -> usize;
    fn devm_kzalloc(dev: *mut device, size: usize) -> *mut u8;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn devm_kasprintf(dev: *mut device, name: *const u8, index: i32) -> *const u8;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> i32;
    fn devm_of_clk_add_hw_provider(dev: *mut device, data: *mut clk_hw_onecell_data) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
