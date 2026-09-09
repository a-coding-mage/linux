// SPDX-License-Identifier: GPL-2.0-only
/* SCI Clock driver for keystone based devices */
// Kernel dependencies are supplied by the surrounding Rust kernel bindings.

const SCI_CLK_SSC_ENABLE: u8 = 1 << 0;
const SCI_CLK_ALLOW_FREQ_CHANGE: u8 = 1 << 1;
const SCI_CLK_INPUT_TERMINATION: u8 = 1 << 2;

#[repr(C)]
struct SciClkProvider {
    sci: *const TiSciHandle,
    ops: *const TiSciClkOps,
    dev: *mut Device,
    clocks: *mut *mut SciClk,
    num_clocks: i32,
}

#[repr(C)]
struct SciClk {
    hw: ClkHw,
    dev_id: u16,
    clk_id: u32,
    num_parents: u32,
    provider: *mut SciClkProvider,
    flags: u8,
    node: ListHead,
    cached_req: usize,
    cached_res: usize,
    parent_id: i32,
    rate: usize,
}

unsafe fn to_sci_clk(hw: *mut ClkHw) -> *mut SciClk {
    (hw as *mut u8).sub(core::mem::offset_of!(SciClk, hw)) as *mut SciClk
}

unsafe fn sci_clk_prepare(hw: *mut ClkHw) -> i32 {
    let clk = to_sci_clk(hw);
    let p = &*(*clk).provider;
    let enable_ssc = (*clk).flags & SCI_CLK_SSC_ENABLE != 0;
    let allow_freq_change = (*clk).flags & SCI_CLK_ALLOW_FREQ_CHANGE != 0;
    let input_termination = (*clk).flags & SCI_CLK_INPUT_TERMINATION != 0;
    ((*p.ops).get_clock)(p.sci, (*clk).dev_id, (*clk).clk_id,
                         enable_ssc, allow_freq_change, input_termination)
}

unsafe fn sci_clk_unprepare(hw: *mut ClkHw) {
    let clk = to_sci_clk(hw);
    let p = &*(*clk).provider;
    let ret = ((*p.ops).put_clock)(p.sci, (*clk).dev_id, (*clk).clk_id);
    if ret != 0 { dev_err(p.dev, "unprepare failed for dev=%d, clk=%d, ret=%d\n", (*clk).dev_id, (*clk).clk_id, ret); }
}

unsafe fn sci_clk_is_prepared(hw: *mut ClkHw) -> i32 {
    let clk = to_sci_clk(hw);
    let p = &*(*clk).provider;
    let mut req_state = false;
    let mut current_state = false;
    let ret = ((*p.ops).is_on)(p.sci, (*clk).dev_id, (*clk).clk_id, &mut req_state, &mut current_state);
    if ret != 0 { dev_err(p.dev, "is_prepared failed for dev=%d, clk=%d, ret=%d\n", (*clk).dev_id, (*clk).clk_id, ret); return 0; }
    req_state as i32
}

unsafe fn sci_clk_recalc_rate(hw: *mut ClkHw, _parent_rate: usize) -> usize {
    let clk = to_sci_clk(hw);
    let p = &*(*clk).provider;
    let mut freq = 0u64;
    let ret = ((*p.ops).get_freq)(p.sci, (*clk).dev_id, (*clk).clk_id, &mut freq);
    if ret != 0 { dev_err(p.dev, "recalc-rate failed for dev=%d, clk=%d, ret=%d\n", (*clk).dev_id, (*clk).clk_id, ret); return 0; }
    (*clk).rate = freq as usize;
    freq as usize
}

unsafe fn sci_clk_determine_rate(hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 {
    let clk = to_sci_clk(hw); let p = &*(*clk).provider;
    if (*clk).cached_req != 0 && (*clk).cached_req == (*req).rate { (*req).rate = (*clk).cached_res; return 0; }
    let mut new_rate = 0u64;
    let ret = ((*p.ops).get_best_match_freq)(p.sci, (*clk).dev_id, (*clk).clk_id, (*req).min_rate, (*req).rate, (*req).max_rate, &mut new_rate);
    if ret != 0 { dev_err(p.dev, "determine-rate failed for dev=%d, clk=%d, ret=%d\n", (*clk).dev_id, (*clk).clk_id, ret); return ret; }
    (*clk).cached_req = (*req).rate; (*clk).cached_res = new_rate as usize; (*req).rate = new_rate as usize; 0
}

unsafe fn sci_clk_set_rate(hw: *mut ClkHw, rate: usize, _parent_rate: usize) -> i32 {
    let clk = to_sci_clk(hw); let p = &*(*clk).provider;
    let ret = ((*p.ops).set_freq)(p.sci, (*clk).dev_id, (*clk).clk_id, rate / 10 * 9, rate, rate / 10 * 11);
    if ret == 0 { (*clk).rate = rate; } ret
}

unsafe fn sci_clk_get_parent(hw: *mut ClkHw) -> u8 {
    let clk = to_sci_clk(hw); let p = &*(*clk).provider; let mut parent_id = 0u32;
    let ret = ((*p.ops).get_parent)(p.sci, (*clk).dev_id, (*clk).clk_id, &mut parent_id as *mut _ as *mut core::ffi::c_void);
    if ret != 0 { dev_err(p.dev, "get-parent failed for dev=%d, clk=%d, ret=%d\n", (*clk).dev_id, (*clk).clk_id, ret); (*clk).parent_id = ret; return 0; }
    (*clk).parent_id = parent_id as i32 - (*clk).clk_id as i32 - 1; (*clk).parent_id as u8
}

unsafe fn sci_clk_set_parent(hw: *mut ClkHw, index: u8) -> i32 {
    let clk = to_sci_clk(hw); let p = &*(*clk).provider; (*clk).cached_req = 0;
    let ret = ((*p.ops).set_parent)(p.sci, (*clk).dev_id, (*clk).clk_id, index as u32 + 1 + (*clk).clk_id);
    if ret == 0 { (*clk).parent_id = index as i32; } ret
}

unsafe fn sci_clk_restore_context(hw: *mut ClkHw) {
    let clk = to_sci_clk(hw);
    if (*clk).num_parents > 1 && (*clk).parent_id >= 0 { sci_clk_set_parent(hw, (*clk).parent_id as u8); }
    if (*clk).rate != 0 { sci_clk_set_rate(hw, (*clk).rate, 0); }
}

// The following declarations mirror the remaining kernel-facing driver items.
// Their concrete types and allocation/device-tree primitives are external dependencies.
unsafe fn _sci_clk_build(provider: *mut SciClkProvider, sci_clk: *mut SciClk) -> i32 {
    let name = kasprintf("clk:%d:%d", (*sci_clk).dev_id, (*sci_clk).clk_id);
    if name.is_null() { return -12; }
    if (*sci_clk).num_parents < 2 { (*sci_clk).num_parents = 0; }
    let mut parent_names: *mut *mut i8 = core::ptr::null_mut();
    if (*sci_clk).num_parents != 0 {
        parent_names = kcalloc((*sci_clk).num_parents as usize);
        if parent_names.is_null() { kfree(name); return -12; }
        for i in 0..(*sci_clk).num_parents as usize {
            *parent_names.add(i) = kasprintf("clk:%d:%d", (*sci_clk).dev_id, (*sci_clk).clk_id + 1 + i as u32);
            if (*parent_names.add(i)).is_null() { kfree(parent_names); kfree(name); return -12; }
        }
    }
    let init = ClkInitData { name, parent_names, ops: &SCI_CLK_OPS, num_parents: (*sci_clk).num_parents, flags: CLK_GET_RATE_NOCACHE };
    (*sci_clk).hw.init = &init;
    let ret = devm_clk_hw_register((*provider).dev, &mut (*sci_clk).hw);
    if ret != 0 { dev_err((*provider).dev, "failed clk register with %d\n", ret); }
    if !parent_names.is_null() { kfree(parent_names); } kfree(name); ret
}

unsafe fn ti_sci_init_clocks(p: *mut SciClkProvider) -> i32 {
    for i in 0..(*p).num_clocks as usize { let ret = _sci_clk_build(p, *(*p).clocks.add(i)); if ret != 0 { return ret; } } 0
}

// CONFIG_TI_SCI_CLK_PROBE_FROM_FW selects the firmware scan implementation;
// otherwise the device-tree scan implementation is used. The full kernel
// device-tree/list plumbing remains represented by these external declarations.
unsafe fn ti_sci_clk_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev;
    let np = (*dev).of_node;
    let handle = devm_ti_sci_get_handle(dev);
    if is_err(handle) { return ptr_err(handle); }
    let provider = devm_kzalloc::<SciClkProvider>(dev);
    if provider.is_null() { return -12; }
    (*provider).sci = handle; (*provider).ops = &(*handle).ops.clk_ops; (*provider).dev = dev;
    let ret = ti_sci_scan_clocks(provider);
    if ret != 0 { dev_err(dev, "scan clocks failed: %d\n", ret); return ret; }
    let ret = ti_sci_init_clocks(provider);
    if ret != 0 { pr_err("ti-sci-init-clocks failed.\n"); return ret; }
    of_clk_add_hw_provider(np, sci_clk_get, provider)
}

unsafe fn ti_sci_clk_remove(pdev: *mut PlatformDevice) { of_clk_del_provider((*pdev).dev.of_node); }

// External kernel types/functions referenced above.
extern "C" {
    static SCI_CLK_OPS: ClkOps;
    fn ti_sci_scan_clocks(p: *mut SciClkProvider) -> i32;
    fn sci_clk_get(spec: *mut OfPhandleArgs, data: *mut core::ffi::c_void) -> *mut ClkHw;
    fn dev_err(dev: *mut Device, fmt: *const i8, ...);
    fn devm_ti_sci_get_handle(dev: *mut Device) -> *const TiSciHandle;
    fn devm_kzalloc<T>(dev: *mut Device) -> *mut T;
    fn kasprintf(fmt: *const i8, ...) -> *mut i8;
    fn kcalloc(n: usize) -> *mut *mut i8;
    fn kfree(p: *mut core::ffi::c_void);
    fn devm_clk_hw_register(dev: *mut Device, hw: *mut ClkHw) -> i32;
    fn is_err<T>(p: *const T) -> bool;
    fn ptr_err<T>(p: *const T) -> i32;
    fn pr_err(fmt: *const i8, ...);
    fn of_clk_add_hw_provider(np: *mut DeviceNode, get: unsafe fn(*mut OfPhandleArgs, *mut core::ffi::c_void) -> *mut ClkHw, data: *mut SciClkProvider) -> i32;
    fn of_clk_del_provider(np: *mut DeviceNode);
}

#[allow(non_camel_case_types)] type u16_ = u16;
#[repr(C)] struct ClkHw { init: *const ClkInitData }
#[repr(C)] struct ClkInitData { name: *mut i8, parent_names: *mut *mut i8, ops: *const ClkOps, num_parents: u32, flags: u32 }
#[repr(C)] struct ClkOps;
#[repr(C)] struct ClkRateRequest { min_rate: usize, rate: usize, max_rate: usize }
#[repr(C)] struct ListHead { next: *mut ListHead, prev: *mut ListHead }
#[repr(C)] struct Device { of_node: *mut DeviceNode }
#[repr(C)] struct DeviceNode;
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct OfPhandleArgs;
#[repr(C)] struct TiSciHandle { ops: TiSciOps }
#[repr(C)] struct TiSciOps { clk_ops: TiSciClkOps }
#[repr(C)] struct TiSciClkOps {
    get_clock: unsafe extern "C" fn(*const TiSciHandle,u16,u32,bool,bool,bool)->i32,
    put_clock: unsafe extern "C" fn(*const TiSciHandle,u16,u32)->i32,
    is_on: unsafe extern "C" fn(*const TiSciHandle,u16,u32,*mut bool,*mut bool)->i32,
    get_freq: unsafe extern "C" fn(*const TiSciHandle,u16,u32,*mut u64)->i32,
    get_best_match_freq: unsafe extern "C" fn(*const TiSciHandle,u16,u32,usize,usize,usize,*mut u64)->i32,
    set_freq: unsafe extern "C" fn(*const TiSciHandle,u16,u32,usize,usize,usize)->i32,
    get_parent: unsafe extern "C" fn(*const TiSciHandle,u16,u32,*mut core::ffi::c_void)->i32,
    set_parent: unsafe extern "C" fn(*const TiSciHandle,u16,u32,u32)->i32,
}
const CLK_GET_RATE_NOCACHE: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
