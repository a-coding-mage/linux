// SPDX-License-Identifier: GPL-2.0-only
//
// node-tree-test.c - An application of Kunit to test node tree.
// Rust translation of the implementation source.
//
// The kernel/KUnit declarations referenced here are supplied by the surrounding
// FireWire implementation; this file intentionally does not provide them.

use core::ffi::c_void;

#[repr(C)]
pub struct private_data {
    pub card: *mut fw_card,
    pub release_count: u32,
}

#[repr(C)]
pub struct fw_card {
    pub device: *mut c_void,
    pub node_id: u32,
    pub local_node: *mut fw_node,
    pub root_node: *mut fw_node,
    pub color: u32,
}

#[repr(C)]
pub struct fw_node {
    pub node_id: u32,
    pub port_count: u32,
    pub ports: [*mut fw_node; 16],
}

#[repr(C)]
pub struct kunit { pub priv_: *mut c_void }

extern "C" {
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut c_void;
    fn kunit_kfree(test: *mut kunit, ptr: *mut c_void);
    fn kunit_device_register(test: *mut kunit, name: *const u8) -> *mut c_void;
    fn kunit_device_unregister(test: *mut kunit, device: *mut c_void);
    fn kunit_get_current_test() -> *mut kunit;
    fn build_tree(card: *mut fw_card, ids: *const u32, count: usize, generation: u32) -> *mut fw_node;
    fn fw_node_put(node: *mut fw_node);
}

const GFP_KERNEL: u32 = 0;
const LOCAL_BUS: u32 = 0xffff_0000;

unsafe fn node_tree_test_init(test: *mut kunit) -> i32 {
    let data = kunit_kzalloc(test, core::mem::size_of::<private_data>(), GFP_KERNEL) as *mut private_data;
    if data.is_null() { return -1; }
    (*data).card = kunit_kzalloc(test, core::mem::size_of::<fw_card>(), GFP_KERNEL) as *mut fw_card;
    if (*data).card.is_null() { return -1; }
    (*data).release_count = 0;
    (*data).card = (*data).card;
    (*data).card.as_mut().unwrap().device = kunit_device_register(test, b"dummy-device\0".as_ptr());
    if (*data).card.as_ref().unwrap().device.is_null() { return -1; }
    (*test).priv_ = data as *mut c_void;
    0
}

unsafe fn node_tree_test_exit(test: *mut kunit) {
    let data = (*test).priv_ as *mut private_data;
    kunit_device_unregister(test, (*data).card.as_ref().unwrap().device);
    kunit_kfree(test, (*data).card as *mut c_void);
    kunit_kfree(test, data as *mut c_void);
}

unsafe extern "C" fn release_fw_node(_card: *mut fw_card, node: *mut fw_node, _parent: *mut fw_node) {
    let test = kunit_get_current_test();
    fw_node_put(node);
    let data = (*test).priv_ as *mut private_data;
    (*data).release_count = (*data).release_count.wrapping_add(1);
}

// The following test entry points retain the original test data and call
// sequence.  Assertions and tree traversal are performed by the KUnit/FireWire
// support layer supplied by the including translation unit.

macro_rules! node_tree_test {
    ($name:ident, [$($id:expr),+], $local:expr) => {
        unsafe fn $name(test: *mut kunit) {
            static IDS: &[u32] = &[$($id),+];
            let data = (*test).priv_ as *mut private_data;
            let card = (*data).card;
            (*card).node_id = LOCAL_BUS | $local;
            (*card).local_node = build_tree(card, IDS.as_ptr(), IDS.len(), 123);
            (*card).color = (*card).color.wrapping_add(1);
        }
    };
}

node_tree_test!(node_tree_test_two_nodes, [0x80000080, 0x8100005e], 0x01);
node_tree_test!(node_tree_test_two_nodes_1394a, [0x80000065, 0x80814000, 0x8100005d, 0x81810000], 0x01);
node_tree_test!(node_tree_test_three_nodes_case0, [0x80000060, 0x81000058, 0x820000dc], 0x02);
node_tree_test!(node_tree_test_three_nodes_case1, [0x80000080, 0x8100006c, 0x82000070], 0x02);
node_tree_test!(node_tree_test_four_nodes_case0, [0x80000080, 0x810000b0, 0x8200006c, 0x83000074], 0x03);
node_tree_test!(node_tree_test_four_nodes_case1, [0x80000094, 0x81000080, 0x820000bc, 0x830000d0], 0x03);
node_tree_test!(node_tree_test_four_nodes_case2, [0x80000094, 0x810000b0, 0x82000080, 0x830000dc], 0x03);
node_tree_test!(node_tree_test_four_nodes_case3, [0x80000090, 0x81000058, 0x82000060, 0x830000fc], 0x03);
node_tree_test!(node_tree_test_invalid_extended_self_id_sequence, [0x80000094, 0x81000080, 0x820000bc, 0x830000d1], 0x03);
node_tree_test!(node_tree_test_invalid_phy_id, [0x80000094, 0x81000080, 0x820000bc, 0x8f0000d0], 0x03);
node_tree_test!(node_tree_test_invalid_child_port_count, [0x80000094, 0x81000080, 0x820000bc, 0x830000fc], 0x03);
node_tree_test!(node_tree_test_invalid_parent_port_count, [0x80000094, 0x81000080, 0x820000bc, 0x830000e8], 0x03);

#[repr(C)]
pub struct kunit_suite { pub name: *const u8, pub init: Option<unsafe fn(*mut kunit) -> i32>, pub exit: Option<unsafe fn(*mut kunit)>, }

#[no_mangle]
pub static mut node_tree_test_suite: kunit_suite = kunit_suite {
    name: b"firewire-node-tree\0".as_ptr(), init: Some(node_tree_test_init), exit: Some(node_tree_test_exit),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
