// SPDX-License-Identifier: GPL-2.0
/*
 * /proc/bootconfig - Extra boot configuration
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xbc_node {
    _private: [u8; 0],
}

extern "C" {
    static mut boot_command_line: *mut c_char;

    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snprintf(dst: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn seq_puts(m: *mut seq_file, s: *const c_char) -> c_int;
    fn cmdline_has_extra_options() -> bool;
    fn xbc_node_compose_key(node: *mut xbc_node, key: *mut c_char, size: usize) -> c_int;
    fn xbc_node_get_child(node: *mut xbc_node) -> *mut xbc_node;
    fn xbc_node_is_array(node: *mut xbc_node) -> bool;
    fn proc_create_single(
        name: *const c_char,
        mode: c_int,
        parent: *mut c_void,
        show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int,
    ) -> *mut c_void;
}

const GFP_KERNEL: c_int = 0;
const XBC_KEYLEN_MAX: usize = 256;

static mut saved_boot_config: *mut c_char = core::ptr::null_mut();

unsafe extern "C" fn boot_config_proc_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    if !saved_boot_config.is_null() {
        seq_puts(m, saved_boot_config);
    }
    0
}

/* Rest size of buffer */
#[inline]
unsafe fn rest(dst: *mut c_char, end: *mut c_char) -> usize {
    if (end as usize) > (dst as usize) {
        (end as usize) - (dst as usize)
    } else {
        0
    }
}

/* Return the needed total length if @size is 0 */
unsafe extern "C" fn copy_xbc_key_value_list(mut dst: *mut c_char, size: usize) -> c_int {
    let mut leaf: *mut xbc_node;
    let mut vnode: *mut xbc_node;
    let mut key: *mut c_char;
    let end = (dst as usize).wrapping_add(size) as *mut c_char;
    let mut ret: c_int = 0;

    key = kzalloc(XBC_KEYLEN_MAX, GFP_KERNEL) as *mut c_char;
    if key.is_null() {
        return -12;
    }

    // xbc_for_each_key_value(leaf, val)
    // The iterator is supplied by the bootconfig implementation.
    let mut val: *const c_char = core::ptr::null();
    while xbc_for_each_key_value_next(&mut leaf, &mut val) {
        ret = xbc_node_compose_key(leaf, key, XBC_KEYLEN_MAX);
        if ret < 0 {
            break;
        }
        ret = snprintf(dst, rest(dst, end), b"%s = \0".as_ptr() as *const c_char, key);
        if ret < 0 {
            break;
        }
        dst = (dst as usize).wrapping_add(ret as usize) as *mut c_char;
        vnode = xbc_node_get_child(leaf);
        if !vnode.is_null() {
            // xbc_array_for_each_value(vnode, val)
            let mut array_val = core::ptr::null();
            while xbc_array_for_each_value_next(vnode, &mut array_val) {
                let q: c_char = if !strchr(array_val, b'"' as c_int).is_null() {
                    b'\'' as c_char
                } else {
                    b'"' as c_char
                };
                ret = snprintf(
                    dst,
                    rest(dst, end),
                    b"%c%s%c%s\0".as_ptr() as *const c_char,
                    q,
                    array_val,
                    q,
                    if xbc_node_is_array(vnode) {
                        b", \0".as_ptr() as *const c_char
                    } else {
                        b"\n\0".as_ptr() as *const c_char
                    },
                );
                if ret < 0 {
                    break;
                }
                dst = (dst as usize).wrapping_add(ret as usize) as *mut c_char;
            }
        } else {
            ret = snprintf(dst, rest(dst, end), b"\"\"\n\0".as_ptr() as *const c_char);
            if ret < 0 {
                break;
            }
            dst = (dst as usize).wrapping_add(ret as usize) as *mut c_char;
        }
    }
    if cmdline_has_extra_options() && ret >= 0 && !boot_command_line.is_null() && *boot_command_line != 0 {
        ret = snprintf(
            dst,
            rest(dst, end),
            b"# Parameters from bootloader:\n# %s\n\0".as_ptr() as *const c_char,
            boot_command_line,
        );
        if ret > 0 {
            dst = (dst as usize).wrapping_add(ret as usize) as *mut c_char;
        }
    }
    kfree(key as *mut c_void);
    if ret < 0 {
        ret
    } else {
        (dst as usize).wrapping_sub((end as usize).wrapping_sub(size)) as c_int
    }
}

unsafe extern "C" {
    fn xbc_for_each_key_value_next(leaf: *mut *mut xbc_node, val: *mut *const c_char) -> bool;
    fn xbc_array_for_each_value_next(vnode: *mut xbc_node, val: *mut *const c_char) -> bool;
}

unsafe extern "C" fn proc_boot_config_init() -> c_int {
    let mut len = copy_xbc_key_value_list(core::ptr::null_mut(), 0);
    if len < 0 {
        return len;
    }
    if len > 0 {
        saved_boot_config = kzalloc((len + 1) as usize, GFP_KERNEL) as *mut c_char;
        if saved_boot_config.is_null() {
            return -12;
        }
        len = copy_xbc_key_value_list(saved_boot_config, (len + 1) as usize);
        if len < 0 {
            kfree(saved_boot_config as *mut c_void);
            return len;
        }
    }
    proc_create_single(b"bootconfig\0".as_ptr() as *const c_char, 0, core::ptr::null_mut(), boot_config_proc_show);
    0
}

// fs_initcall(proc_boot_config_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
