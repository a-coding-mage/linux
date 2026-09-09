// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the surrounding kernel translation unit:
// linux/string.h, linux/err.h, linux/slab.h, linux/of.h, asm/prom.h,
// and "of_helpers.h".

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct property {
    _private: [u8; 0],
}

pub type __be32 = u32;

#[repr(C)]
pub struct of_drc_info {
    pub drc_type: *mut c_char,
    pub drc_name_prefix: *mut c_char,
    pub drc_index_start: u32,
    pub drc_name_suffix_start: u32,
    pub num_sequential_elems: u32,
    pub sequential_inc: u32,
    pub drc_power_domain: u32,
    pub last_drc_index: u32,
}

extern "C" {
    fn kbasename(path: *const c_char) -> *const c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> i32;
    fn kstrndup(s: *const c_char, len: usize, flags: u32) -> *mut c_char;
    fn of_find_node_by_path(path: *const c_char) -> *mut device_node;
    fn kfree(ptr: *mut c_void);
    fn of_prop_next_string(prop: *mut *mut property, cur: *const c_char) -> *const c_char;
    fn of_prop_next_u32(
        prop: *mut *mut property,
        cur: *const __be32,
        val: *mut u32,
    ) -> *const __be32;
    fn be32_to_cpu(value: __be32) -> u32;
    fn err_ptr(error: isize) -> *mut device_node;
}

const GFP_KERNEL: u32 = 0;
const EINVAL: isize = 22;
const ENOMEM: isize = 12;

/// pseries_of_derive_parent - basically like dirname(1)
/// @path:  the full_name of a node to be added to the tree
///
/// Returns the node which should be the parent of the node
/// described by path.  E.g., for path = "/foo/bar", returns
/// the node with full_name = "/foo".
#[no_mangle]
pub unsafe extern "C" fn pseries_of_derive_parent(path: *const c_char) -> *mut device_node {
    let parent: *mut device_node;
    let mut parent_path: *mut c_char = b"/\0".as_ptr() as *mut c_char;

    // We do not want the trailing '/' character
    let tail = kbasename(path).offset(-1);

    // reject if path is "/"
    if strcmp(path, b"/\0".as_ptr() as *const c_char) == 0 {
        return err_ptr(-EINVAL);
    }

    if tail > path {
        parent_path = kstrndup(path, tail.offset_from(path) as usize, GFP_KERNEL);
        if parent_path.is_null() {
            return err_ptr(-ENOMEM);
        }
    }
    parent = of_find_node_by_path(parent_path);
    if strcmp(parent_path, b"/\0".as_ptr() as *const c_char) != 0 {
        kfree(parent_path as *mut c_void);
    }
    if !parent.is_null() {
        parent
    } else {
        err_ptr(-EINVAL)
    }
}

// Helper Routines to convert between drc_index to cpu numbers

#[no_mangle]
pub unsafe extern "C" fn of_read_drc_info_cell(
    prop: *mut *mut property,
    curval: *mut *const __be32,
    data: *mut of_drc_info,
) -> i32 {
    let mut p = *curval as *const c_char;
    let mut p2: *const __be32;

    if data.is_null() {
        return -EINVAL as i32;
    }

    // Get drc-type:encode-string
    (*data).drc_type = p as *mut c_char;
    p = of_prop_next_string(prop, p);
    if p.is_null() {
        return -EINVAL as i32;
    }

    // Get drc-name-prefix:encode-string
    (*data).drc_name_prefix = p as *mut c_char;
    p = of_prop_next_string(prop, p);
    if p.is_null() {
        return -EINVAL as i32;
    }

    // Get drc-index-start:encode-int
    p2 = p as *const __be32;
    (*data).drc_index_start = be32_to_cpu(*p2);

    // Get drc-name-suffix-start:encode-int
    p2 = of_prop_next_u32(prop, p2, &mut (*data).drc_name_suffix_start);
    if p2.is_null() {
        return -EINVAL as i32;
    }

    // Get number-sequential-elements:encode-int
    p2 = of_prop_next_u32(prop, p2, &mut (*data).num_sequential_elems);
    if p2.is_null() {
        return -EINVAL as i32;
    }

    // Get sequential-increment:encode-int
    p2 = of_prop_next_u32(prop, p2, &mut (*data).sequential_inc);
    if p2.is_null() {
        return -EINVAL as i32;
    }

    // Get drc-power-domain:encode-int
    p2 = of_prop_next_u32(prop, p2, &mut (*data).drc_power_domain);
    if p2.is_null() {
        return -EINVAL as i32;
    }

    // Should now know end of current entry
    *curval = p2.add(1) as *const __be32;
    (*data).last_drc_index = (*data).drc_index_start
        + ((*data).num_sequential_elems - 1) * (*data).sequential_inc;

    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
