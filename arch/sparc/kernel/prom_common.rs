// SPDX-License-Identifier: GPL-2.0-or-later
/* prom_common.c: OF device tree support common code.
 *
 * Paul Mackerras	August 1996.
 * Copyright (C) 1996-2005 Paul Mackerras.
 *
 *  Adapted for 64bit PowerPC by Dave Engebretsen and Peter Bergner.
 *    {engebret|bergner}@us.ibm.com
 *
 *  Adapted for sparc by David S. Miller davem@davemloft.net
 */

// C dependencies: linux/kernel.h, linux/export.h, linux/errno.h,
// linux/mutex.h, linux/slab.h, linux/of.h, linux/of_pdt.h, asm/prom.h,
// asm/oplib.h, and "prom.h".

extern "C" {
    pub static mut of_console_device: *mut device_node;
    pub static mut of_console_path: *mut core::ffi::c_char;
    pub static mut of_console_options: *mut core::ffi::c_char;
    pub static mut of_set_property_mutex: mutex;
    pub static mut devtree_lock: raw_spinlock;
    pub static mut prom_early_allocated: u32;

    fn of_find_property(np: *mut device_node, name: *const core::ffi::c_char,
                        len: *mut i32) -> *mut property;
    fn kmemdup(src: *const core::ffi::c_void, len: usize, flags: u32)
        -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn strcasecmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
    fn prom_setprop(node: phandle, name: *const core::ffi::c_char,
                    val: *const core::ffi::c_void, len: i32) -> i32;
    fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
    fn strlen(s: *const core::ffi::c_char) -> usize;
    fn strscpy(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char,
               size: usize) -> isize;
    fn prom_nextprop(node: phandle, prev: *mut core::ffi::c_char,
                     buf: *mut core::ffi::c_char) -> *const core::ffi::c_char;
    fn prom_getproplen(node: phandle, name: *const core::ffi::c_char) -> i32;
    fn prom_getproperty(node: phandle, name: *const core::ffi::c_char,
                        val: *mut core::ffi::c_void, len: i32) -> i32;
    fn prom_getchild(node: phandle) -> phandle;
    fn prom_getsibling(node: phandle) -> phandle;
    fn of_pdt_build_devicetree(root: phandle, ops: *const of_pdt_ops);
    fn of_console_init();
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock, flags: usize);
}

#[repr(C)]
pub struct device_node { pub properties: *mut property, pub phandle: phandle }
#[repr(C)]
pub struct property {
    pub name: *const core::ffi::c_char,
    pub length: i32,
    pub value: *mut core::ffi::c_void,
    pub next: *mut property,
}
#[repr(C)]
pub struct mutex { _private: [u8; 0] }
#[repr(C)]
pub struct raw_spinlock { _private: [u8; 0] }
pub type phandle = u32;
#[repr(C)]
pub struct of_pdt_ops {
    pub nextprop: Option<unsafe extern "C" fn(*mut core::ffi::c_char, *const core::ffi::c_char) -> i32>,
    pub getproplen: Option<unsafe extern "C" fn(phandle, *const core::ffi::c_char) -> i32>,
    pub getproperty: Option<unsafe extern "C" fn(phandle, *const core::ffi::c_char, *mut core::ffi::c_void, i32) -> i32>,
    pub getchild: Option<unsafe extern "C" fn(phandle) -> phandle>,
    pub getsibling: Option<unsafe extern "C" fn(phandle) -> phandle>,
}

const ENOMEM: i32 = 12;
const ENODEV: i32 = 19;
const EINVAL: i32 = 22;

pub unsafe fn of_getintprop_default(np: *mut device_node, name: *const core::ffi::c_char, def: i32) -> i32 {
    let mut len = 0;
    let prop = of_find_property(np, name, &mut len);
    if prop.is_null() || len != 4 { return def; }
    *( (*prop).value as *const i32 )
}

pub unsafe fn of_set_property(dp: *mut device_node, name: *const core::ffi::c_char,
                              val: *mut core::ffi::c_void, len: i32) -> i32 {
    let new_val = kmemdup(val, len as usize, 0);
    if new_val.is_null() { return -ENOMEM; }
    let mut err = -ENODEV;
    mutex_lock(&mut of_set_property_mutex);
    let mut flags = 0usize;
    raw_spin_lock_irqsave(&mut devtree_lock, &mut flags);
    let mut prevp = &mut (*dp).properties as *mut *mut property;
    while !(*prevp).is_null() {
        let prop = *prevp;
        if strcasecmp((*prop).name, name) == 0 {
            let old_val = (*prop).value;
            let ret = prom_setprop((*dp).phandle, name, val, len);
            err = -EINVAL;
            if ret >= 0 {
                (*prop).value = new_val;
                (*prop).length = len;
                // OF_IS_DYNAMIC(prop)
                kfree(old_val);
                // OF_MARK_DYNAMIC(prop)
                err = 0;
            }
            break;
        }
        prevp = &mut (*(*prevp)).next;
    }
    raw_spin_unlock_irqrestore(&mut devtree_lock, flags);
    mutex_unlock(&mut of_set_property_mutex);
    /* XXX Upate procfs if necessary... */
    err
}

pub unsafe fn of_find_in_proplist(mut list: *const core::ffi::c_char,
                                  match_: *const core::ffi::c_char, mut len: i32) -> i32 {
    while len > 0 {
        if strcmp(list, match_) == 0 { return 1; }
        let l = strlen(list) + 1;
        list = list.add(l);
        len -= l as i32;
    }
    0
}

unsafe fn handle_nextprop_quirks(buf: *mut core::ffi::c_char, name: *const core::ffi::c_char) -> i32 {
    let name_len = if name.is_null() { 0 } else { strlen(name) };
    if name_len == 0 { return -1; }
    // CONFIG_SPARC32: strscpy(buf, name, name_len + 1)
    0
}

unsafe fn prom_common_nextprop(node: phandle, prev: *mut core::ffi::c_char,
                               buf: *mut core::ffi::c_char) -> i32 {
    *buf = 0;
    let name = prom_nextprop(node, prev, buf);
    handle_nextprop_quirks(buf, name)
}

static mut prom_sparc_ops: of_pdt_ops = of_pdt_ops {
    nextprop: Some(prom_common_nextprop),
    getproplen: Some(prom_getproplen),
    getproperty: Some(prom_getproperty),
    getchild: Some(prom_getchild),
    getsibling: Some(prom_getsibling),
};

pub unsafe fn prom_build_devicetree(prom_root_node: phandle) {
    of_pdt_build_devicetree(prom_root_node, &prom_sparc_ops);
    of_console_init();
    pr_info(b"PROM: Built device tree with %u bytes of memory.\0".as_ptr() as *const _, prom_early_allocated);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
