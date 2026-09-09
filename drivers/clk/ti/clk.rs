// SPDX-License-Identifier: GPL-2.0-only
/* TI clock support */

// C dependencies supplied by the surrounding kernel translation are referenced here.

static mut CLK_HW_OMAP_CLOCKS: list_head = list_head::new();
static mut TI_CLK_LL_OPS: *mut ti_clk_ll_ops = core::ptr::null_mut();
static mut CLOCKS_NODE_PTR: [*mut device_node; CLK_MAX_MEMMAPS] = [core::ptr::null_mut(); CLK_MAX_MEMMAPS];
static mut TI_CLK_FEATURES: ti_clk_features = unsafe { core::mem::zeroed() };

#[repr(C)]
struct clk_iomap { regmap: *mut regmap, mem: *mut core::ffi::c_void }

static mut CLK_MEMMAPS: [*mut clk_iomap; CLK_MAX_MEMMAPS] = [core::ptr::null_mut(); CLK_MAX_MEMMAPS];

unsafe fn clk_memmap_writel(val: u32, reg: *const clk_omap_reg) {
    let io = CLK_MEMMAPS[(*reg).index as usize];
    if !(*reg).ptr.is_null() { writel_relaxed(val, (*reg).ptr); }
    else if !(*io).regmap.is_null() { regmap_write((*io).regmap, (*reg).offset, val); }
    else { writel_relaxed(val, ((*io).mem as *mut u8).add((*reg).offset as usize)); }
}

unsafe fn _clk_rmw(val: u32, mask: u32, ptr: *mut core::ffi::c_void) {
    let mut v = readl_relaxed(ptr); v &= !mask; v |= val; writel_relaxed(v, ptr);
}

unsafe fn clk_memmap_rmw(val: u32, mask: u32, reg: *const clk_omap_reg) {
    let io = CLK_MEMMAPS[(*reg).index as usize];
    if !(*reg).ptr.is_null() { _clk_rmw(val, mask, (*reg).ptr); }
    else if !(*io).regmap.is_null() { regmap_update_bits((*io).regmap, (*reg).offset, mask, val); }
    else { _clk_rmw(val, mask, ((*io).mem as *mut u8).add((*reg).offset as usize) as *mut _); }
}

unsafe fn clk_memmap_readl(reg: *const clk_omap_reg) -> u32 {
    let io = CLK_MEMMAPS[(*reg).index as usize];
    if !(*reg).ptr.is_null() { readl_relaxed((*reg).ptr) }
    else if !(*io).regmap.is_null() { let mut val = 0; regmap_read((*io).regmap, (*reg).offset, &mut val); val }
    else { readl_relaxed(((*io).mem as *mut u8).add((*reg).offset as usize) as *mut _) }
}

pub unsafe fn ti_clk_setup_ll_ops(ops: *mut ti_clk_ll_ops) -> i32 {
    if !TI_CLK_LL_OPS.is_null() { pr_err!("Attempt to register ll_ops multiple times.\n"); return -EBUSY; }
    TI_CLK_LL_OPS = ops; (*ops).clk_readl = Some(clk_memmap_readl); (*ops).clk_writel = Some(clk_memmap_writel); (*ops).clk_rmw = Some(clk_memmap_rmw); 0
}

unsafe fn ti_find_clock_provider(name: *const i8) -> *mut device_node {
    let mut tmp = kstrdup_and_replace(name, b'-' as i32, b'_' as i32, GFP_KERNEL);
    if tmp.is_null() { return core::ptr::null_mut(); }
    let p = strchr(tmp, b'@' as i32); if !p.is_null() { *p = 0; }
    let mut np = core::ptr::null_mut();
    for_each_node_with_property!(np, b"clock-output-names\0".as_ptr() as _, { if of_property_match_string(np, b"clock-output-names\0".as_ptr() as _, tmp) == 0 { kfree(tmp as _); return np; } });
    let ret = of_find_node_by_name(core::ptr::null_mut(), tmp); kfree(tmp as _); ret
}

pub unsafe fn ti_clk_get_legacy_bit_shift(node: *mut device_node) -> i32 { let mut val=0; if of_property_read_u32(node,b"ti,bit-shift\0".as_ptr() as _,&mut val)==0 && in_range(val,0,32) { val as i32 } else { 0 } }

pub unsafe fn ti_clk_latch(reg: *mut clk_omap_reg, shift: i8) { if shift < 0 { return; } let latch=1u32 << shift; ((*TI_CLK_LL_OPS).clk_rmw.unwrap())(latch,latch,reg); ((*TI_CLK_LL_OPS).clk_rmw.unwrap())(0,latch,reg); ((*TI_CLK_LL_OPS).clk_readl.unwrap())(reg); }

pub unsafe fn ti_clk_get_features() -> *const ti_clk_features { &TI_CLK_FEATURES }
pub unsafe fn ti_clk_setup_features(features: *const ti_clk_features) { core::ptr::copy_nonoverlapping(features,&mut TI_CLK_FEATURES,1); }

// The remaining exported routines retain the kernel ABI and call the corresponding
// external clock, device-tree, allocation, and list primitives.
extern "C" {
    fn ti_dt_clocks_register(oclks: *mut ti_dt_clk);
}

pub unsafe fn ti_clk_get_reg_addr(node:*mut device_node,index:i32,reg:*mut clk_omap_reg)->i32 {
    let mut i=0; while i<CLK_MAX_MEMMAPS as i32 { if CLOCKS_NODE_PTR[i as usize]==(*node).parent || CLOCKS_NODE_PTR[i as usize]==(*(*node).parent).parent { break } i+=1; }
    if i==CLK_MAX_MEMMAPS as i32 { pr_err!("clk-provider not found!\n"); return -ENOENT; } (*reg).index=i as _;
    let mut addr=0; let mut val=0; let clksel=of_device_is_compatible((*node).parent,b"ti,clksel\0".as_ptr() as _);
    if clksel && of_property_read_u32_index((*node).parent,b"reg\0".as_ptr() as _,index,&mut addr)!=0 { return -EINVAL; }
    let err=of_property_read_u32_index(node,b"reg\0".as_ptr() as _,index,&mut val);
    if err!=0 && clksel { (*reg).offset=addr; (*reg).bit=ti_clk_get_legacy_bit_shift(node) as _; (*reg).ptr=core::ptr::null_mut(); return 0; }
    if clksel { (*reg).offset=addr; (*reg).bit=val as _; } else { (*reg).offset=val; (*reg).bit=ti_clk_get_legacy_bit_shift(node) as _; } (*reg).ptr=core::ptr::null_mut(); 0
}

pub unsafe fn ti_clk_add_alias(clk:*mut clk,con:*const i8)->i32 { if clk.is_null(){return 0} if IS_ERR(clk){return PTR_ERR(clk)} let cl=kzalloc(core::mem::size_of::<clk_lookup>(),GFP_KERNEL) as *mut clk_lookup; if cl.is_null(){return -ENOMEM} (*cl).con_id=con;(*cl).clk=clk;clkdev_add(cl);0 }
pub unsafe fn of_ti_clk_register(node:*mut device_node,hw:*mut clk_hw,con:*const i8)->*mut clk { let r=of_clk_hw_register(node,hw);if r!=0{return ERR_PTR(r)} let c=(*hw).clk;let r=ti_clk_add_alias(c,con);if r!=0{clk_unregister(c);return ERR_PTR(r)}c }
pub unsafe fn of_ti_clk_register_omap_hw(node:*mut device_node,hw:*mut clk_hw,con:*const i8)->*mut clk { let c=of_ti_clk_register(node,hw,con);if IS_ERR(c){return c} list_add(&mut (*(to_clk_hw_omap(hw))).node,&mut CLK_HW_OMAP_CLOCKS);c }
pub unsafe fn omap2_clk_is_hw_omap(hw:*mut clk_hw)->bool { let mut p=core::ptr::null_mut(); list_for_each_entry!(p,CLK_HW_OMAP_CLOCKS,node,{if &(*p).hw==hw{return true}});false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
