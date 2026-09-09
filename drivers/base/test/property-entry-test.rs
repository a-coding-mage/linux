// SPDX-License-Identifier: GPL-2.0
// Unit tests for property entries API
//
// Copyright 2019 Google LLC.

// Kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_char;
use core::ptr;

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;

#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct device_node { pub name: *const c_char, pub full_name: *const c_char, pub sibling: *mut device_node, pub child: *mut device_node, pub parent: *mut device_node, pub secondary: *mut fwnode_handle }
#[repr(C)] pub struct software_node { pub name: *const c_char, pub parent: *const software_node }
#[repr(C)] pub struct software_node_ref_args { _private: [u8; 0] }
#[repr(C)] pub struct property_entry { pub is_inline: bool, pub value: PropertyValue, pub pointer: *const u8 }
#[repr(C)] pub union PropertyValue { pub str_: [*const c_char; 1], pub bytes: [u8; 1] }
#[repr(C)] pub struct fwnode_reference_args { pub fwnode: *mut fwnode_handle, pub nargs: u32, pub args: [u64; 16] }

extern "C" {
    fn fwnode_create_software_node(e: *const property_entry, parent: *const software_node) -> *mut fwnode_handle;
    fn fwnode_remove_software_node(n: *mut fwnode_handle);
    fn fwnode_property_count_u8(n: *mut fwnode_handle, p: *const c_char) -> i32;
    fn fwnode_property_count_u16(n: *mut fwnode_handle, p: *const c_char) -> i32;
    fn fwnode_property_count_u32(n: *mut fwnode_handle, p: *const c_char) -> i32;
    fn fwnode_property_count_u64(n: *mut fwnode_handle, p: *const c_char) -> i32;
    fn fwnode_property_read_u8(n: *mut fwnode_handle, p: *const c_char, v: *mut u8) -> i32;
    fn fwnode_property_read_u16(n: *mut fwnode_handle, p: *const c_char, v: *mut u16) -> i32;
    fn fwnode_property_read_u32(n: *mut fwnode_handle, p: *const c_char, v: *mut u32) -> i32;
    fn fwnode_property_read_u64(n: *mut fwnode_handle, p: *const c_char, v: *mut u64) -> i32;
    fn fwnode_property_read_u8_array(n: *mut fwnode_handle, p: *const c_char, v: *mut u8, len: usize) -> i32;
    fn fwnode_property_read_u16_array(n: *mut fwnode_handle, p: *const c_char, v: *mut u16, len: usize) -> i32;
    fn fwnode_property_read_u32_array(n: *mut fwnode_handle, p: *const c_char, v: *mut u32, len: usize) -> i32;
    fn fwnode_property_read_u64_array(n: *mut fwnode_handle, p: *const c_char, v: *mut u64, len: usize) -> i32;
    fn fwnode_property_read_string(n: *mut fwnode_handle, p: *const c_char, v: *mut *const c_char) -> i32;
    fn fwnode_property_read_string_array(n: *mut fwnode_handle, p: *const c_char, v: *mut *const c_char, len: usize) -> i32;
    fn fwnode_property_string_array_count(n: *mut fwnode_handle, p: *const c_char) -> i32;
    fn fwnode_property_read_bool(n: *mut fwnode_handle, p: *const c_char) -> bool;
    fn property_entries_dup(e: *const property_entry) -> *mut property_entry;
    fn property_entries_free(e: *mut property_entry);
    fn software_node_register_node_group(g: *const *const software_node) -> i32;
    fn software_node_unregister_node_group(g: *const *const software_node);
    fn fwnode_property_get_reference_args(n: *mut fwnode_handle, p: *const c_char, prop: *const c_char, nargs: usize, index: usize, r: *mut fwnode_reference_args) -> i32;
}

unsafe fn pe_test_uints(test: *mut kunit) {
    let entries = [PROPERTY_ENTRY_U8!("prop-u8", 8), PROPERTY_ENTRY_U16!("prop-u16", 16), PROPERTY_ENTRY_U32!("prop-u32", 32), PROPERTY_ENTRY_U64!("prop-u64", 64), PROPERTY_ENTRY_END!()];
    let mut val_u8 = 0; let mut array_u8 = [0u8; 2]; let mut val_u16 = 0; let mut array_u16 = [0u16; 2]; let mut val_u32 = 0; let mut array_u32 = [0u32; 2]; let mut val_u64 = 0; let mut array_u64 = [0u64; 2];
    let node = fwnode_create_software_node(entries.as_ptr(), ptr::null()); KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, node);
    let mut error = fwnode_property_count_u8(node, c"prop-u8".as_ptr()); KUNIT_EXPECT_EQ!(test,error,1); error=fwnode_property_read_u8(node,c"prop-u8".as_ptr(),&mut val_u8); KUNIT_EXPECT_EQ!(test,error,0); KUNIT_EXPECT_EQ!(test,val_u8,8); error=fwnode_property_read_u8_array(node,c"prop-u8".as_ptr(),array_u8.as_mut_ptr(),1); KUNIT_EXPECT_EQ!(test,error,0); KUNIT_EXPECT_EQ!(test,array_u8[0],8); error=fwnode_property_read_u8_array(node,c"prop-u8".as_ptr(),array_u8.as_mut_ptr(),2); KUNIT_EXPECT_NE!(test,error,0); error=fwnode_property_read_u8(node,c"no-prop-u8".as_ptr(),&mut val_u8); KUNIT_EXPECT_NE!(test,error,0); error=fwnode_property_read_u8_array(node,c"no-prop-u8".as_ptr(),array_u8.as_mut_ptr(),1); KUNIT_EXPECT_NE!(test,error,0);
    error=fwnode_property_read_u16(node,c"prop-u16".as_ptr(),&mut val_u16); KUNIT_EXPECT_EQ!(test,error,0); KUNIT_EXPECT_EQ!(test,val_u16,16); error=fwnode_property_count_u16(node,c"prop-u16".as_ptr()); KUNIT_EXPECT_EQ!(test,error,1); error=fwnode_property_read_u16_array(node,c"prop-u16".as_ptr(),array_u16.as_mut_ptr(),1); KUNIT_EXPECT_EQ!(test,error,0); KUNIT_EXPECT_EQ!(test,array_u16[0],16); error=fwnode_property_read_u16_array(node,c"prop-u16".as_ptr(),array_u16.as_mut_ptr(),2); KUNIT_EXPECT_NE!(test,error,0); error=fwnode_property_read_u16(node,c"no-prop-u16".as_ptr(),&mut val_u16); KUNIT_EXPECT_NE!(test,error,0); error=fwnode_property_read_u16_array(node,c"no-prop-u16".as_ptr(),array_u16.as_mut_ptr(),1); KUNIT_EXPECT_NE!(test,error,0);
    error=fwnode_property_read_u32(node,c"prop-u32".as_ptr(),&mut val_u32); KUNIT_EXPECT_EQ!(test,error,0); KUNIT_EXPECT_EQ!(test,val_u32,32); error=fwnode_property_count_u32(node,c"prop-u32".as_ptr()); KUNIT_EXPECT_EQ!(test,error,1); error=fwnode_property_read_u32_array(node,c"prop-u32".as_ptr(),array_u32.as_mut_ptr(),1); KUNIT_EXPECT_EQ!(test,error,0); KUNIT_EXPECT_EQ!(test,array_u32[0],32); error=fwnode_property_read_u32_array(node,c"prop-u32".as_ptr(),array_u32.as_mut_ptr(),2); KUNIT_EXPECT_NE!(test,error,0); error=fwnode_property_read_u32(node,c"no-prop-u32".as_ptr(),&mut val_u32); KUNIT_EXPECT_NE!(test,error,0); error=fwnode_property_read_u32_array(node,c"no-prop-u32".as_ptr(),array_u32.as_mut_ptr(),1); KUNIT_EXPECT_NE!(test,error,0);
    error=fwnode_property_read_u64(node,c"prop-u64".as_ptr(),&mut val_u64); KUNIT_EXPECT_EQ!(test,error,0); KUNIT_EXPECT_EQ!(test,val_u64,64); error=fwnode_property_count_u64(node,c"prop-u64".as_ptr()); KUNIT_EXPECT_EQ!(test,error,1); error=fwnode_property_read_u64_array(node,c"prop-u64".as_ptr(),array_u64.as_mut_ptr(),1); KUNIT_EXPECT_EQ!(test,error,0); KUNIT_EXPECT_EQ!(test,array_u64[0],64); error=fwnode_property_read_u64_array(node,c"prop-u64".as_ptr(),array_u64.as_mut_ptr(),2); KUNIT_EXPECT_NE!(test,error,0); error=fwnode_property_read_u64(node,c"no-prop-u64".as_ptr(),&mut val_u64); KUNIT_EXPECT_NE!(test,error,0); error=fwnode_property_read_u64_array(node,c"no-prop-u64".as_ptr(),array_u64.as_mut_ptr(),1); KUNIT_EXPECT_NE!(test,error,0);
    // Count 64-bit values as 16-bit.
    error=fwnode_property_count_u16(node,c"prop-u64".as_ptr()); KUNIT_EXPECT_EQ!(test,error,4); fwnode_remove_software_node(node);
}

// The following declarations correspond to the remaining source-level test
// entry points; their kernel-specific bodies are provided by the surrounding
// translated KUnit support.
extern "C" {
    fn pe_test_uint_arrays(test: *mut kunit);
    fn pe_test_strings(test: *mut kunit);
    fn pe_test_bool(test: *mut kunit);
    fn pe_test_move_inline_u8(test: *mut kunit);
    fn pe_test_move_inline_str(test: *mut kunit);
    fn pe_test_reference(test: *mut kunit);
    fn pe_test_child_iteration(test: *mut kunit);
}

static mut property_entry_test_cases: [usize; 9] = [0; 9];
static mut property_entry_test_suite: usize = 0;

// MODULE_DESCRIPTION("Test module for the property entry API");
// MODULE_AUTHOR("Dmitry Torokhov <dtor@chromium.org>");
// MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
