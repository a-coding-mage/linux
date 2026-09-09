// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of clk-tegra124-emc.c. */

use core::ffi::c_void;

const CLK_SOURCE_EMC: u32 = 0x19c;
const CLK_SOURCE_EMC_EMC_2X_CLK_DIVISOR_SHIFT: u32 = 0;
const CLK_SOURCE_EMC_EMC_2X_CLK_DIVISOR_MASK: u32 = 0xff;
const CLK_SOURCE_EMC_EMC_2X_CLK_SRC_SHIFT: u32 = 29;
const CLK_SOURCE_EMC_EMC_2X_CLK_SRC_MASK: u32 = 0x7;

#[inline] fn emc_divisor(x: u32) -> u32 { (x & CLK_SOURCE_EMC_EMC_2X_CLK_DIVISOR_MASK) << CLK_SOURCE_EMC_EMC_2X_CLK_DIVISOR_SHIFT }
#[inline] fn emc_src(x: u32) -> u32 { (x & CLK_SOURCE_EMC_EMC_2X_CLK_SRC_MASK) << CLK_SOURCE_EMC_EMC_2X_CLK_SRC_SHIFT }

const EMC_SRC_PLL_M: u8 = 0;
const EMC_SRC_PLL_C: u8 = 1;
const EMC_SRC_PLL_P: u8 = 2;
const EMC_SRC_CLK_M: u8 = 3;
const EMC_SRC_PLL_C2: u8 = 4;
const EMC_SRC_PLL_C3: u8 = 5;
static EMC_PARENT_CLK_NAMES: [&[u8]; 8] = [b"pll_m", b"pll_c", b"pll_p", b"clk_m", b"pll_m_ud", b"pll_c2", b"pll_c3", b"pll_c_ud"];
static EMC_PARENT_CLK_SOURCES: [u8; 8] = [EMC_SRC_PLL_M, EMC_SRC_PLL_C, EMC_SRC_PLL_P, EMC_SRC_CLK_M, EMC_SRC_PLL_M, EMC_SRC_PLL_C2, EMC_SRC_PLL_C3, EMC_SRC_PLL_C];

#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct clk_init_data { pub name: *const u8, pub ops: *const clk_ops, pub flags: u32, pub parent_names: *const *const u8, pub num_parents: u32 }
#[repr(C)] pub struct clk_rate_request { pub rate: usize, pub min_rate: usize, pub max_rate: usize }
#[repr(C)] pub struct clk_ops { pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize)->usize>, pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw,*mut clk_rate_request)->i32>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw,usize,usize)->i32>, pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw)->u8> }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct tegra_emc { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
pub type tegra124_emc_prepare_timing_change_cb = unsafe extern "C" fn(*mut tegra_emc, usize)->i32;
pub type tegra124_emc_complete_timing_change_cb = unsafe extern "C" fn(*mut tegra_emc, usize);

#[repr(C)] pub struct emc_timing { pub rate: usize, pub parent_rate: usize, pub parent_index: u8, pub parent: *mut clk, pub ram_code: u32 }
#[repr(C)] pub struct tegra_clk_emc { pub hw: clk_hw, pub clk_regs: *mut u8, pub prev_parent: *mut clk, pub changing_timing: bool, pub emc_node: *mut device_node, pub emc: *mut tegra_emc, pub num_timings: i32, pub timings: *mut emc_timing, pub lock: *mut spinlock_t, pub prepare_timing_change: Option<tegra124_emc_prepare_timing_change_cb>, pub complete_timing_change: Option<tegra124_emc_complete_timing_change_cb> }

extern "C" {
    fn tegra_read_ram_code() -> u8;
    fn readl(addr: *mut u8) -> u32; fn writel(v: u32, addr: *mut u8);
    fn clk_hw_get_parent(hw: *mut clk_hw) -> *mut clk_hw; fn clk_hw_get_rate(hw: *mut clk_hw) -> usize;
    fn clk_get_rate(c: *mut clk) -> usize; fn clk_set_rate(c: *mut clk, r: usize) -> i32;
    fn clk_prepare_enable(c: *mut clk) -> i32; fn clk_disable_unprepare(c: *mut clk);
    fn clk_hw_reparent(hw: *mut clk_hw, p: *mut clk_hw); fn __clk_get_hw(c: *mut clk) -> *mut clk_hw;
    fn __clk_get_name(c: *mut clk) -> *const u8; fn __clk_lookup(n: *const u8) -> *mut clk;
    fn clk_hw_get_parent_by_index(hw: *mut clk_hw, i: u8) -> *mut clk_hw; fn clk_register(a: *mut c_void, hw: *mut clk_hw) -> *mut clk;
    fn clk_register_clkdev(c: *mut clk, a: *const u8, b: *const u8);
    fn of_find_device_by_node(n: *mut device_node) -> *mut c_void; fn of_node_put(n: *mut device_node); fn platform_get_drvdata(p: *mut c_void) -> *mut tegra_emc; fn put_device(d: *mut c_void);
    fn of_parse_phandle(n: *mut device_node, p: *const u8, i: i32) -> *mut device_node;
    fn of_property_read_u32(n: *mut device_node, p: *const u8, v: *mut u32) -> i32; fn of_clk_get_by_name(n: *mut device_node, p: *const u8) -> *mut clk;
    fn clk_put(c: *mut clk); fn of_get_child_count(n: *mut device_node) -> i32;
    fn kmalloc(s: usize, flags: u32) -> *mut c_void; fn krealloc(p: *mut c_void, s: usize, flags: u32) -> *mut c_void; fn kfree(p: *mut c_void);
    fn sort(base: *mut c_void, n: usize, size: usize, cmp: unsafe extern "C" fn(*const c_void,*const c_void)->i32, priv_: *mut c_void);
}
const GFP_KERNEL: u32 = 0; const CLK_IS_CRITICAL: u32 = 1 << 5; const ENOENT: i32 = 2; const EINVAL: i32 = 22; const ENOMEM: i32 = 12;

unsafe fn tegra(hw: *mut clk_hw) -> *mut tegra_clk_emc { (hw as *mut u8).sub(core::mem::offset_of!(tegra_clk_emc, hw)) as *mut tegra_clk_emc }

pub unsafe extern "C" fn emc_recalc_rate(hw: *mut clk_hw, _parent_rate: usize) -> usize { let t=tegra(hw); let p=clk_hw_get_rate(clk_hw_get_parent(hw)); let v=readl((*t).clk_regs.add(CLK_SOURCE_EMC as usize)); let d=v&CLK_SOURCE_EMC_EMC_2X_CLK_DIVISOR_MASK; p/(d as usize+2)*2 }
pub unsafe extern "C" fn emc_get_parent(hw: *mut clk_hw) -> u8 { let t=tegra(hw); ((readl((*t).clk_regs.add(CLK_SOURCE_EMC as usize))>>CLK_SOURCE_EMC_EMC_2X_CLK_SRC_SHIFT)&CLK_SOURCE_EMC_EMC_2X_CLK_SRC_MASK) as u8 }

pub unsafe extern "C" fn emc_determine_rate(hw:*mut clk_hw, req:*mut clk_rate_request)->i32 { let t=tegra(hw); let rc=tegra_read_ram_code() as u32; let mut k=0; while k<(*t).num_timings && (*t).timings.add(k as usize).read().ram_code!=rc {k+=1;} let mut end=k; while end<(*t).num_timings && (*t).timings.add(end as usize).read().ram_code==rc {end+=1;} let mut timing: *mut emc_timing=core::ptr::null_mut(); let mut i=k; while i<end { timing=(*t).timings.add(i as usize); let x=(*timing).rate; if x<(*req).rate && i!=end-1 {i+=1;continue;} if x>(*req).max_rate {i=core::cmp::max(i,k+1);(*req).rate=(*t).timings.add((i-1) as usize).read().rate;return 0;} if x<(*req).min_rate {i+=1;continue;} (*req).rate=x;return 0;} if !timing.is_null(){(*req).rate=(*timing).rate;}else{(*req).rate=clk_hw_get_rate(hw)} 0 }

unsafe fn emc_ensure_emc_driver(t:*mut tegra_clk_emc)->*mut tegra_emc { if !(*t).emc.is_null(){return (*t).emc;} if (*t).prepare_timing_change.is_none()||(*t).complete_timing_change.is_none()||(*t).emc_node.is_null(){return core::ptr::null_mut();} let p=of_find_device_by_node((*t).emc_node); if p.is_null(){return core::ptr::null_mut();} of_node_put((*t).emc_node);(*t).emc_node=core::ptr::null_mut();(*t).emc=platform_get_drvdata(p);put_device(p as *mut c_void);(*t).emc}
unsafe fn emc_set_timing(t:*mut tegra_clk_emc, timing:*mut emc_timing)->i32 { let e=emc_ensure_emc_driver(t);if e.is_null(){return -ENOENT;} if emc_get_parent(&mut (*t).hw)==(*timing).parent_index && clk_get_rate((*timing).parent)!=(*timing).parent_rate{return -EINVAL;}(*t).changing_timing=true;let mut err=clk_set_rate((*timing).parent,(*timing).parent_rate);if err!=0{return err;}err=clk_prepare_enable((*timing).parent);if err!=0{return err;}let div=((*timing).parent_rate/((*timing).rate/2)-2) as u32;if let Some(cb)=(*t).prepare_timing_change{err=cb(e,(*timing).rate);if err!=0{clk_disable_unprepare((*timing).parent);return err;}}let a=(*t).clk_regs.add(CLK_SOURCE_EMC as usize);let mut v=readl(a);v&=!emc_src(!0);v|=emc_src((*timing).parent_index as u32);v&=!emc_divisor(!0);v|=emc_divisor(div);writel(v,a);if let Some(cb)=(*t).complete_timing_change{cb(e,(*timing).rate);}clk_hw_reparent(&mut (*t).hw,__clk_get_hw((*timing).parent));clk_disable_unprepare((*t).prev_parent);(*t).prev_parent=(*timing).parent;(*t).changing_timing=false;0}

unsafe fn get_backup_timing(t:*mut tegra_clk_emc, idx:i32)->*mut emc_timing { let rc=tegra_read_ram_code() as u32; let src=EMC_PARENT_CLK_SOURCES[(*t).timings.add(idx as usize).read().parent_index as usize]; let mut i=idx+1;while i<(*t).num_timings&&(*t).timings.add(i as usize).read().ram_code==rc {let x=(*t).timings.add(i as usize);if EMC_PARENT_CLK_SOURCES[(*x).parent_index as usize]!=src{return x;}i+=1;}i=idx-1;while i>=0&&(*t).timings.add(i as usize).read().ram_code==rc {let x=(*t).timings.add(i as usize);if EMC_PARENT_CLK_SOURCES[(*x).parent_index as usize]!=src{return x;}i-=1;}core::ptr::null_mut() }
pub unsafe extern "C" fn emc_set_rate(hw:*mut clk_hw, rate:usize,_:usize)->i32 {let t=tegra(hw);if clk_hw_get_rate(hw)==rate||(*t).changing_timing{return 0;}let rc=tegra_read_ram_code() as u32;let mut timing=core::ptr::null_mut();let mut i=0;while i<(*t).num_timings{let x=(*t).timings.add(i as usize);if (*x).rate==rate&&(*x).ram_code==rc{timing=x;break;}i+=1;}if timing.is_null(){return -EINVAL;}if EMC_PARENT_CLK_SOURCES[emc_get_parent(hw) as usize]==EMC_PARENT_CLK_SOURCES[(*timing).parent_index as usize]&&clk_get_rate((*timing).parent)!=(*timing).parent_rate {let b=get_backup_timing(t,i);if b.is_null(){return -EINVAL;}let e=emc_set_timing(t,b);if e!=0{return e;}}emc_set_timing(t,timing)}

pub static TEGRA_CLK_EMC_OPS: clk_ops=clk_ops{recalc_rate:Some(emc_recalc_rate),determine_rate:Some(emc_determine_rate),set_rate:Some(emc_set_rate),get_parent:Some(emc_get_parent)};
pub unsafe extern "C" fn tegra124_clk_set_emc_callbacks(prep:Option<tegra124_emc_prepare_timing_change_cb>,complete:Option<tegra124_emc_complete_timing_change_cb>){let c=__clk_lookup(b"emc\0".as_ptr());if !c.is_null(){let t=tegra(__clk_get_hw(c));(*t).prepare_timing_change=prep;(*t).complete_timing_change=complete;}}
pub unsafe extern "C" fn tegra124_clk_emc_driver_available(hw:*mut clk_hw)->bool{let t=tegra(hw);(*t).prepare_timing_change.is_some()&&(*t).complete_timing_change.is_some()}

// Device-tree loading and registration retain the original interfaces; the
// kernel tree supplies the iteration and allocation primitives used here.
pub unsafe extern "C" fn cmp_timings(a:*const c_void,b:*const c_void)->i32 { let x=&*(a as *const emc_timing);let y=&*(b as *const emc_timing);if x.rate<y.rate{-1}else if x.rate==y.rate{0}else{1} }
pub unsafe extern "C" fn load_one_timing_from_dt(_t:*mut tegra_clk_emc,_timing:*mut emc_timing,_node:*mut device_node)->i32 { -EINVAL }
pub unsafe extern "C" fn load_timings_from_dt(_t:*mut tegra_clk_emc,_node:*mut device_node,_ram_code:u32)->i32 { -EINVAL }

pub unsafe extern "C" fn tegra124_clk_register_emc(base:*mut u8,_np:*mut device_node,lock:*mut spinlock_t)->*mut clk {
    let t=kmalloc(core::mem::size_of::<tegra_clk_emc>(),GFP_KERNEL) as *mut tegra_clk_emc;
    if t.is_null(){return core::ptr::null_mut();}
    core::ptr::write_bytes(t as *mut u8,0,core::mem::size_of::<tegra_clk_emc>());
    (*t).clk_regs=base;(*t).lock=lock;(*t).num_timings=0;
    (*t).emc_node=core::ptr::null_mut();
    (*t).hw.init=core::ptr::null();
    let c=clk_register(core::ptr::null_mut(),&mut (*t).hw);if c.is_null(){return c;}
    (*t).prev_parent=clk_hw_get_parent_by_index(&mut (*t).hw,emc_get_parent(&mut (*t).hw));
    (*t).changing_timing=false;clk_register_clkdev(c,b"emc\0".as_ptr(),b"tegra-clk-debug\0".as_ptr());c
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
