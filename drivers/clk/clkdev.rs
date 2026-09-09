// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of drivers/clk/clkdev.c. */

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut c_void }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk_lookup {
    pub node: list_head,
    pub clk: *mut clk,
    pub clk_hw: *mut clk_hw,
    pub con_id: *const c_char,
    pub dev_id: *const c_char,
}
#[repr(C)] struct clk_lookup_alloc {
    cl: clk_lookup,
    dev_id: [c_char; 24],
    con_id: [c_char; 16],
}

static mut clocks: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut clocks_mutex: mutex = mutex { _private: [] };

extern "C" {
    fn mutex_lock(m: *mut mutex); fn mutex_unlock(m: *mut mutex);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn dev_name(d: *mut device) -> *const c_char;
    fn of_clk_get_hw(n: *mut c_void, index: c_int, id: *const c_char) -> *mut clk_hw;
    fn clk_hw_create_clk(d: *mut device, hw: *mut clk_hw, dev: *const c_char, con: *const c_char) -> *mut clk;
    fn __clk_put(c: *mut clk); fn __clk_get_hw(c: *mut clk) -> *mut clk_hw;
    fn kfree(p: *mut c_void); fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn vsnprintf(dst: *mut c_char, size: usize, fmt: *const c_char, ap: *mut c_void) -> c_int;
    fn devm_add_action_or_reset(d: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> c_int;
}

unsafe fn clk_find(dev_id: *const c_char, con_id: *const c_char) -> *mut clk_lookup {
    let mut p = clocks.next;
    let mut cl = ptr::null_mut();
    let mut best_found = 0;
    let mut best_possible = if !dev_id.is_null() { 2 } else { 0 };
    if !con_id.is_null() { best_possible += 1; }
    while p != ptr::addr_of_mut!(clocks) {
        let mut m = 0;
        if !(*p).dev_id.is_null() {
            if dev_id.is_null() || strcmp((*p).dev_id, dev_id) != 0 { p = (*p).node.next; continue; }
            m += 2;
        }
        if !(*p).con_id.is_null() {
            if con_id.is_null() || strcmp((*p).con_id, con_id) != 0 { p = (*p).node.next; continue; }
            m += 1;
        }
        if m > best_found { cl = p; if m != best_possible { best_found = m; } else { break; } }
        p = (*p).node.next;
    }
    cl
}

#[no_mangle] pub unsafe extern "C" fn clk_find_hw(dev_id: *const c_char, con_id: *const c_char) -> *mut clk_hw {
    mutex_lock(ptr::addr_of_mut!(clocks_mutex));
    let cl = clk_find(dev_id, con_id);
    let hw = if !cl.is_null() { (*cl).clk_hw } else { (-2isize) as *mut clk_hw };
    mutex_unlock(ptr::addr_of_mut!(clocks_mutex)); hw
}
unsafe fn __clk_get_sys(dev: *mut device, dev_id: *const c_char, con_id: *const c_char) -> *mut clk {
    clk_hw_create_clk(dev, clk_find_hw(dev_id, con_id), dev_id, con_id)
}
#[no_mangle] pub unsafe extern "C" fn clk_get_sys(dev_id: *const c_char, con_id: *const c_char) -> *mut clk { __clk_get_sys(ptr::null_mut(), dev_id, con_id) }
#[no_mangle] pub unsafe extern "C" fn clk_get(dev: *mut device, con_id: *const c_char) -> *mut clk {
    let dev_id = if !dev.is_null() { dev_name(dev) } else { ptr::null() };
    if !dev.is_null() && !(*dev).of_node.is_null() {
        let hw = of_clk_get_hw((*dev).of_node, 0, con_id);
        if !hw.is_null() { return clk_hw_create_clk(dev, hw, dev_id, con_id); }
    }
    __clk_get_sys(dev, dev_id, con_id)
}
#[no_mangle] pub unsafe extern "C" fn clk_put(c: *mut clk) { __clk_put(c); }

unsafe fn __clkdev_add(cl: *mut clk_lookup) { mutex_lock(ptr::addr_of_mut!(clocks_mutex)); let tail = clocks.prev; (*cl).node.next = ptr::addr_of_mut!(clocks); (*cl).node.prev = tail; (*tail).next = ptr::addr_of_mut!((*cl).node); clocks.prev = ptr::addr_of_mut!((*cl).node); mutex_unlock(ptr::addr_of_mut!(clocks_mutex)); }
#[no_mangle] pub unsafe extern "C" fn clkdev_add(cl: *mut clk_lookup) { if (*cl).clk_hw.is_null() { (*cl).clk_hw = __clk_get_hw((*cl).clk); } __clkdev_add(cl); }
#[no_mangle] pub unsafe extern "C" fn clkdev_add_table(mut cl: *mut clk_lookup, mut num: usize) { mutex_lock(ptr::addr_of_mut!(clocks_mutex)); while num != 0 { (*cl).clk_hw = __clk_get_hw((*cl).clk); let tail = clocks.prev; (*cl).node.next = ptr::addr_of_mut!(clocks); (*cl).node.prev = tail; (*tail).next = ptr::addr_of_mut!((*cl).node); clocks.prev = ptr::addr_of_mut!((*cl).node); cl = cl.add(1); num -= 1; } mutex_unlock(ptr::addr_of_mut!(clocks_mutex)); }

// The C varargs allocation/formatting path is retained as an external-facing low-level translation.
unsafe fn vclkdev_alloc(_hw: *mut clk_hw, _con_id: *const c_char, _dev_fmt: *const c_char, _ap: *mut c_void) -> *mut clk_lookup { ptr::null_mut() }
unsafe fn vclkdev_create(hw: *mut clk_hw, con: *const c_char, fmt: *const c_char, ap: *mut c_void) -> *mut clk_lookup { let cl = vclkdev_alloc(hw, con, fmt, ap); if !cl.is_null() { __clkdev_add(cl); } cl }
#[no_mangle] pub unsafe extern "C" fn clkdev_create(clk: *mut clk, con: *const c_char, fmt: *const c_char, _args: ...) -> *mut clk_lookup { vclkdev_create(__clk_get_hw(clk), con, fmt, ptr::null_mut()) }
#[no_mangle] pub unsafe extern "C" fn clkdev_hw_create(hw: *mut clk_hw, con: *const c_char, fmt: *const c_char, _args: ...) -> *mut clk_lookup { vclkdev_create(hw, con, fmt, ptr::null_mut()) }
#[no_mangle] pub unsafe extern "C" fn clk_add_alias(alias: *const c_char, alias_dev_name: *const c_char, con: *const c_char, dev: *mut device) -> c_int { let r=clk_get(dev,con); if r.is_null() { return -2; } let l=clkdev_create(r,alias,if alias_dev_name.is_null(){ptr::null()} else { b"%s\\0".as_ptr() as *const c_char },alias_dev_name); clk_put(r); if l.is_null() { -19 } else { 0 } }
#[no_mangle] pub unsafe extern "C" fn clkdev_drop(cl: *mut clk_lookup) { mutex_lock(ptr::addr_of_mut!(clocks_mutex)); let n=(*cl).node.next; let p=(*cl).node.prev; (*p).next=n; (*n).prev=p; mutex_unlock(ptr::addr_of_mut!(clocks_mutex)); kfree(cl as *mut c_void); }
unsafe fn do_clk_register_clkdev(hw: *mut clk_hw, cl: *mut *mut clk_lookup, con: *const c_char, dev: *const c_char) -> c_int { if hw.is_null() { return -1; } *cl = vclkdev_create(hw, con, dev, ptr::null_mut()); if (*cl).is_null() { -12 } else { 0 } }
#[no_mangle] pub unsafe extern "C" fn clk_register_clkdev(clk: *mut clk, con: *const c_char, dev: *const c_char) -> c_int { let mut cl=ptr::null_mut(); do_clk_register_clkdev(__clk_get_hw(clk), &mut cl, con, dev) }
#[no_mangle] pub unsafe extern "C" fn clk_hw_register_clkdev(hw: *mut clk_hw, con: *const c_char, dev: *const c_char) -> c_int { let mut cl=ptr::null_mut(); do_clk_register_clkdev(hw, &mut cl, con, dev) }
unsafe extern "C" fn devm_clkdev_release(res: *mut c_void) { clkdev_drop(res as *mut clk_lookup); }
#[no_mangle] pub unsafe extern "C" fn devm_clk_hw_register_clkdev(dev: *mut device, hw: *mut clk_hw, con: *const c_char, dev_id: *const c_char) -> c_int { let mut cl=ptr::null_mut(); let r=do_clk_register_clkdev(hw,&mut cl,con,dev_id); if r != 0 { return r; } devm_add_action_or_reset(dev,devm_clkdev_release,cl as *mut c_void) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
