// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * drivers/clk/at91/sckc.c
 *
 *  Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
 */

// Kernel headers and DT bindings are supplied by the surrounding translation.

const SLOW_CLOCK_FREQ: u32 = 32768;
const SLOWCK_SW_CYCLES: u32 = 5;
const SLOWCK_SW_TIME_USEC: u32 = (SLOWCK_SW_CYCLES * USEC_PER_SEC) / SLOW_CLOCK_FREQ;
const AT91_SCKC_CR: usize = 0x00;

#[repr(C)]
struct clk_slow_bits { cr_rcen: u32, cr_osc32en: u32, cr_osc32byp: u32, cr_oscsel: u32 }
#[repr(C)]
struct clk_slow_osc { hw: clk_hw, sckcr: *mut core::ffi::c_void, bits: *const clk_slow_bits, startup_usec: usize }
#[repr(C)]
struct clk_sama5d4_slow_osc { hw: clk_hw, sckcr: *mut core::ffi::c_void, bits: *const clk_slow_bits, startup_usec: usize, prepared: bool }
#[repr(C)]
struct clk_slow_rc_osc { hw: clk_hw, sckcr: *mut core::ffi::c_void, bits: *const clk_slow_bits, frequency: usize, accuracy: usize, startup_usec: usize }
#[repr(C)]
struct clk_sam9x5_slow { hw: clk_hw, sckcr: *mut core::ffi::c_void, bits: *const clk_slow_bits, parent: u8 }

extern "C" {
    static mut system_state: i32;
    fn readl(p: *mut core::ffi::c_void) -> u32;
    fn writel(v: u32, p: *mut core::ffi::c_void);
    fn udelay(v: usize); fn usleep_range(a: usize, b: usize);
    fn clk_hw_register(p: *mut core::ffi::c_void, h: *mut clk_hw) -> i32;
    fn clk_hw_unregister(h: *mut clk_hw);
    fn kfree(p: *mut core::ffi::c_void);
    fn of_iomap(n: *mut device_node, i: i32) -> *mut core::ffi::c_void;
    fn of_clk_get_parent_name(n: *mut device_node, i: i32) -> *const core::ffi::c_char;
    fn of_get_compatible_child(n: *mut device_node, s: *const core::ffi::c_char) -> *mut device_node;
    fn of_property_read_bool(n: *mut device_node, s: *const core::ffi::c_char) -> bool;
    fn of_clk_add_hw_provider(n: *mut device_node, f: *const core::ffi::c_void, h: *mut core::ffi::c_void) -> i32;
    fn clk_hw_register_fixed_rate_with_accuracy(a: *mut core::ffi::c_void, n: *const core::ffi::c_char, p: *const core::ffi::c_char, f: u64, r: usize, ac: usize) -> *mut clk_hw;
    fn clk_hw_register_fixed_rate_parent_hw(a: *mut core::ffi::c_void, n: *const core::ffi::c_char, p: *mut clk_hw, f: u64, r: usize) -> *mut clk_hw;
}

#[repr(C)] struct clk_hw { init: *const clk_init_data }
#[repr(C)] struct clk_init_data { name: *const core::ffi::c_char, ops: *const clk_ops, parent_data: *const clk_parent_data, parent_hws: *const *const clk_hw, parent_names: *const *const core::ffi::c_char, num_parents: usize, flags: u32 }
#[repr(C)] struct clk_parent_data { name: *const core::ffi::c_char, fw_name: *const core::ffi::c_char }
#[repr(C)] struct clk_ops { prepare: Option<unsafe extern "C" fn(*mut clk_hw)->i32>, unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>, is_prepared: Option<unsafe extern "C" fn(*mut clk_hw)->i32>, recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw,usize)->usize>, recalc_accuracy: Option<unsafe extern "C" fn(*mut clk_hw,usize)->usize>, set_parent: Option<unsafe extern "C" fn(*mut clk_hw,u8)->i32>, get_parent: Option<unsafe extern "C" fn(*mut clk_hw)->u8>, determine_rate: *const core::ffi::c_void }
#[repr(C)] struct device_node;
const SYSTEM_RUNNING: i32 = 1; const CLK_IGNORE_UNUSED: u32 = 1;
const EINVAL: i32 = 22; const ENOMEM: i32 = 12;
extern "C" { fn ERR_PTR(e: i32) -> *mut clk_hw; fn IS_ERR(p: *mut clk_hw) -> bool; }

unsafe fn slow_osc(hw: *mut clk_hw) -> *mut clk_slow_osc { hw as *mut clk_slow_osc }
unsafe fn sama5d4_osc(hw: *mut clk_hw) -> *mut clk_sama5d4_slow_osc { hw as *mut clk_sama5d4_slow_osc }
unsafe fn rc_osc(hw: *mut clk_hw) -> *mut clk_slow_rc_osc { hw as *mut clk_slow_rc_osc }
unsafe fn sam_slow(hw: *mut clk_hw) -> *mut clk_sam9x5_slow { hw as *mut clk_sam9x5_slow }

unsafe extern "C" fn clk_slow_osc_prepare(hw: *mut clk_hw) -> i32 { let o=slow_osc(hw); let s=(*o).sckcr; let t=readl(s); if t & ((*(*o).bits).cr_osc32byp|(*(*o).bits).cr_osc32en)!=0{return 0} writel(t|(*(*o).bits).cr_osc32en,s); if system_state<SYSTEM_RUNNING {udelay((*o).startup_usec)} else {usleep_range((*o).startup_usec,(*o).startup_usec+1)} 0 }
unsafe extern "C" fn clk_slow_osc_unprepare(hw:*mut clk_hw){let o=slow_osc(hw);let s=(*o).sckcr;let t=readl(s);if t&(*(*o).bits).cr_osc32byp!=0{return}writel(t&!(*(*o).bits).cr_osc32en,s)}
unsafe extern "C" fn clk_slow_osc_is_prepared(hw:*mut clk_hw)->i32{let o=slow_osc(hw);let t=readl((*o).sckcr);if t&(*(*o).bits).cr_osc32byp!=0{1}else{((t&(*(*o).bits).cr_osc32en)!=0)as i32}}
static SLOW_OSC_OPS: clk_ops=clk_ops{prepare:Some(clk_slow_osc_prepare),unprepare:Some(clk_slow_osc_unprepare),is_prepared:Some(clk_slow_osc_is_prepared),recalc_rate:None,recalc_accuracy:None,set_parent:None,get_parent:None,determine_rate:core::ptr::null()};

unsafe extern "C" fn clk_slow_rc_osc_recalc_rate(hw:*mut clk_hw,_:usize)->usize{(*rc_osc(hw)).frequency}
unsafe extern "C" fn clk_slow_rc_osc_recalc_accuracy(hw:*mut clk_hw,_:usize)->usize{(*rc_osc(hw)).accuracy}
unsafe extern "C" fn clk_slow_rc_osc_prepare(hw:*mut clk_hw)->i32{let o=rc_osc(hw);writel(readl((*o).sckcr)|(*(*o).bits).cr_rcen,(*o).sckcr);if system_state<SYSTEM_RUNNING{udelay((*o).startup_usec)}else{usleep_range((*o).startup_usec,(*o).startup_usec+1)}0}
unsafe extern "C" fn clk_slow_rc_osc_unprepare(hw:*mut clk_hw){let o=rc_osc(hw);writel(readl((*o).sckcr)&!(*(*o).bits).cr_rcen,(*o).sckcr)}
unsafe extern "C" fn clk_slow_rc_osc_is_prepared(hw:*mut clk_hw)->i32{let o=rc_osc(hw);((readl((*o).sckcr)&(*(*o).bits).cr_rcen)!=0)as i32}
unsafe extern "C" fn clk_sam9x5_slow_set_parent(hw:*mut clk_hw,index:u8)->i32{let o=sam_slow(hw);if index>1{return -EINVAL}let mut t=readl((*o).sckcr);if (index==0&&(t&(*(*o).bits).cr_oscsel)==0)||(index!=0&&(t&(*(*o).bits).cr_oscsel)!=0){return 0}if index!=0{t|=(*(*o).bits).cr_oscsel}else{t&=!(*(*o).bits).cr_oscsel}writel(t,(*o).sckcr);if system_state<SYSTEM_RUNNING{udelay(SLOWCK_SW_TIME_USEC as usize)}else{usleep_range(SLOWCK_SW_TIME_USEC as usize,SLOWCK_SW_TIME_USEC as usize+1)}0}
unsafe extern "C" fn clk_sam9x5_slow_get_parent(hw:*mut clk_hw)->u8{let o=sam_slow(hw);((readl((*o).sckcr)&(*(*o).bits).cr_oscsel)!=0)as u8}
unsafe extern "C" fn clk_sama5d4_slow_osc_prepare(hw:*mut clk_hw)->i32{let o=sama5d4_osc(hw);if (*o).prepared{return 0}if readl((*o).sckcr)&(*(*o).bits).cr_oscsel!=0{(*o).prepared=true;return 0}if system_state<SYSTEM_RUNNING{udelay((*o).startup_usec)}else{usleep_range((*o).startup_usec,(*o).startup_usec+1)}(*o).prepared=true;0}
unsafe extern "C" fn clk_sama5d4_slow_osc_is_prepared(hw:*mut clk_hw)->i32{(*sama5d4_osc(hw)).prepared as i32}

// The remaining registration and SoC setup routines retain the C driver's external
// kernel dependencies and are represented with their original control-flow shape.
#[allow(dead_code)]
unsafe fn at91_clk_register_slow_osc(sckcr:*mut core::ffi::c_void,name:*const core::ffi::c_char,parent_data:*const clk_parent_data,startup:usize,bypass:bool,bits:*const clk_slow_bits)->*mut clk_hw { if sckcr.is_null()||name.is_null()||parent_data.is_null(){return ERR_PTR(-EINVAL)} let o=Box::into_raw(Box::new(clk_slow_osc{hw:clk_hw{init:core::ptr::null()},sckcr,bits,startup_usec:startup})); if bypass {writel((readl(sckcr)&!(*bits).cr_osc32en)|(*bits).cr_osc32byp,sckcr)} let r=clk_hw_register(core::ptr::null_mut(),&mut (*o).hw); if r!=0{kfree(o as *mut _);ERR_PTR(r)}else{&mut (*o).hw} }
unsafe fn at91_clk_unregister_slow_osc(hw:*mut clk_hw){let o=slow_osc(hw);clk_hw_unregister(hw);kfree(o as *mut _)}

// Additional C-only provider-registration glue is intentionally kept as declarations;
// its implementation depends on the translated kernel clock framework.
extern "C" { fn at91sam9x5_sckc_register(np:*mut device_node, rc_osc_startup_us:u32, bits:*const clk_slow_bits); fn of_at91sam9x5_sckc_setup(np:*mut device_node); fn of_sama5d3_sckc_setup(np:*mut device_node); fn of_sam9x60_sckc_setup(np:*mut device_node); fn of_sama5d4_sckc_setup(np:*mut device_node); }

static AT91SAM9X5_BITS: clk_slow_bits=clk_slow_bits{cr_rcen:1,cr_osc32en:2,cr_osc32byp:4,cr_oscsel:8};
static AT91SAM9X60_BITS: clk_slow_bits=clk_slow_bits{cr_rcen:0,cr_osc32en:2,cr_osc32byp:4,cr_oscsel:1<<24};
static AT91SAMA5D4_BITS: clk_slow_bits=clk_slow_bits{cr_rcen:0,cr_osc32en:0,cr_osc32byp:0,cr_oscsel:8};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
