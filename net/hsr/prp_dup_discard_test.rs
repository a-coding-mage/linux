// SPDX-License-Identifier: GPL-2.0
// Translated from the C implementation. Kernel and KUnit dependencies are
// supplied by the surrounding build.

use core::ffi::c_void;

extern "C" {
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut c_void;
    fn kunit_kcalloc(test: *mut kunit, n: usize, size: usize, flags: u32) -> *mut c_void;
    fn hsr_seq_block_size(node: *const hsr_node) -> usize;
    fn xa_init(xa: *mut xarray);
    fn spin_lock_init(lock: *mut spinlock);
    fn xa_load(xa: *const xarray, index: u16) -> *mut hsr_seq_block;
    fn hsr_get_seq_block(node: *mut hsr_node, index: u16) -> *mut hsr_seq_block;
    fn test_bit(bit: u16, value: *const u64) -> bool;
    fn prp_register_frame_out(port: *mut hsr_port, frame: *mut hsr_frame_info) -> i32;
    static mut jiffies: u64;
    fn msecs_to_jiffies(ms: u64) -> u64;
}

#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct xarray { _private: [u8; 0] }
#[repr(C)] pub struct spinlock { _private: [u8; 0] }
#[repr(C)] pub struct hsr_port { pub r#type: i32 }
#[repr(C)] pub struct hsr_frame_info { pub node_src: *mut hsr_node, pub port_rcv: *mut hsr_port, pub sequence_nr: u16 }
#[repr(C)] pub struct hsr_node { pub seq_port_cnt: u8, pub block_buf: *mut c_void, pub seq_blocks: xarray, pub seq_out_lock: spinlock }
#[repr(C)] pub struct hsr_seq_block { pub seq_nrs: [u64; 1], pub time: u64 }

const GFP_USER: u32 = 0;
const GFP_ATOMIC: u32 = 0;
const HSR_MAX_SEQ_BLOCKS: usize = 0;
const HSR_SEQ_BLOCK_SHIFT: u16 = 0;
const HSR_SEQ_BLOCK_MASK: u16 = 0;
const HSR_ENTRY_FORGET_TIME: u64 = 0;
const HSR_PT_SLAVE_A: i32 = 0;
const HSR_PT_SLAVE_B: i32 = 0;
const HSR_PT_MASTER: i32 = 0;

struct prp_test_data {
    port: hsr_port,
    port_rcv: hsr_port,
    frame: hsr_frame_info,
    node: hsr_node,
}

unsafe fn build_prp_test_data(test: *mut kunit) -> *mut prp_test_data {
    let block_sz: usize;
    let data = kunit_kzalloc(test, core::mem::size_of::<prp_test_data>(), GFP_USER) as *mut prp_test_data;
    assert!(!data.is_null());

    (*data).node.seq_port_cnt = 1;
    block_sz = hsr_seq_block_size(&(*data).node);
    (*data).node.block_buf = kunit_kcalloc(test, HSR_MAX_SEQ_BLOCKS, block_sz, GFP_ATOMIC);
    assert!(!(*data).node.block_buf.is_null());

    xa_init(&mut (*data).node.seq_blocks);
    spin_lock_init(&mut (*data).node.seq_out_lock);
    (*data).frame.node_src = &mut (*data).node;
    (*data).frame.port_rcv = &mut (*data).port_rcv;
    (*data).port_rcv.r#type = HSR_PT_SLAVE_A;
    (*data).port.r#type = HSR_PT_MASTER;
    data
}

unsafe fn check_prp_frame_seen(_test: *mut kunit, data: *mut prp_test_data, sequence_nr: u16) {
    let block_idx = sequence_nr >> HSR_SEQ_BLOCK_SHIFT;
    let block = xa_load(&(*data).node.seq_blocks, block_idx);
    assert!(!block.is_null());
    let seq_bit = sequence_nr & HSR_SEQ_BLOCK_MASK;
    assert!(test_bit(seq_bit, (*block).seq_nrs.as_ptr()));
}

unsafe fn check_prp_frame_unseen(_test: *mut kunit, data: *mut prp_test_data, sequence_nr: u16) {
    let block_idx = sequence_nr >> HSR_SEQ_BLOCK_SHIFT;
    let block = hsr_get_seq_block(&mut (*data).node, block_idx);
    assert!(!block.is_null());
    let seq_bit = sequence_nr & HSR_SEQ_BLOCK_MASK;
    assert!(!test_bit(seq_bit, (*block).seq_nrs.as_ptr()));
}

unsafe fn prp_dup_discard_forward(test: *mut kunit) {
    /* Normal situation, both LANs in sync. Next frame is forwarded */
    let data = build_prp_test_data(test);
    (*data).frame.sequence_nr = 2;
    assert_eq!(0, prp_register_frame_out(&mut (*data).port, &mut (*data).frame));
    check_prp_frame_seen(test, data, (*data).frame.sequence_nr);
}

unsafe fn prp_dup_discard_drop_duplicate(test: *mut kunit) {
    let data = build_prp_test_data(test);
    (*data).frame.sequence_nr = 2;
    assert_eq!(0, prp_register_frame_out(&mut (*data).port, &mut (*data).frame));
    check_prp_frame_seen(test, data, (*data).frame.sequence_nr);
    assert_eq!(1, prp_register_frame_out(&mut (*data).port, &mut (*data).frame));
    check_prp_frame_seen(test, data, (*data).frame.sequence_nr);
}

unsafe fn prp_dup_discard_entry_timeout(test: *mut kunit) {
    /* Timeout situation, node hasn't sent anything for a while */
    let data = build_prp_test_data(test);
    (*data).frame.sequence_nr = 7;
    assert_eq!(0, prp_register_frame_out(&mut (*data).port, &mut (*data).frame));
    check_prp_frame_seen(test, data, (*data).frame.sequence_nr);
    (*data).frame.sequence_nr = 11;
    assert_eq!(0, prp_register_frame_out(&mut (*data).port, &mut (*data).frame));
    check_prp_frame_seen(test, data, (*data).frame.sequence_nr);
    let block_idx = (*data).frame.sequence_nr >> HSR_SEQ_BLOCK_SHIFT;
    let block = hsr_get_seq_block(&mut (*data).node, block_idx);
    (*block).time = jiffies - msecs_to_jiffies(HSR_ENTRY_FORGET_TIME) - 1;
    assert_eq!(0, prp_register_frame_out(&mut (*data).port, &mut (*data).frame));
    check_prp_frame_seen(test, data, (*data).frame.sequence_nr);
    check_prp_frame_unseen(test, data, 7);
}

unsafe fn prp_dup_discard_out_of_sequence(test: *mut kunit) {
    /* One frame is received out of sequence on both LANs */
    let data = build_prp_test_data(test);
    (*data).frame.sequence_nr = 9;
    assert_eq!(0, prp_register_frame_out(&mut (*data).port, &mut (*data).frame));
    check_prp_frame_seen(test, data, (*data).frame.sequence_nr);
    (*data).frame.sequence_nr = 8;
    assert_eq!(0, prp_register_frame_out(&mut (*data).port, &mut (*data).frame));
    check_prp_frame_seen(test, data, (*data).frame.sequence_nr);
    (*data).port_rcv.r#type = HSR_PT_SLAVE_B;
    assert_eq!(1, prp_register_frame_out(&mut (*data).port, &mut (*data).frame));
    (*data).frame.sequence_nr = 10;
    (*data).port_rcv.r#type = HSR_PT_SLAVE_A;
    assert_eq!(0, prp_register_frame_out(&mut (*data).port, &mut (*data).frame));
    check_prp_frame_seen(test, data, (*data).frame.sequence_nr);
    (*data).port_rcv.r#type = HSR_PT_SLAVE_B;
    assert_eq!(1, prp_register_frame_out(&mut (*data).port, &mut (*data).frame));
}

unsafe fn prp_dup_discard_lan_b_late(test: *mut kunit) {
    /* LAN B is behind */
    let data = build_prp_test_data(test);
    for seq in [9u16, 10u16] {
        (*data).frame.sequence_nr = seq;
        assert_eq!(0, prp_register_frame_out(&mut (*data).port, &mut (*data).frame));
        check_prp_frame_seen(test, data, seq);
    }
    (*data).port_rcv.r#type = HSR_PT_SLAVE_B;
    for seq in [9u16, 10u16] {
        (*data).frame.sequence_nr = seq;
        assert_eq!(1, prp_register_frame_out(&mut (*data).port, &mut (*data).frame));
    }
}

#[repr(C)]
struct kunit_case {
    run_case: unsafe fn(*mut kunit),
}

static prp_dup_discard_test_cases: &[kunit_case] = &[
    kunit_case { run_case: prp_dup_discard_forward },
    kunit_case { run_case: prp_dup_discard_drop_duplicate },
    kunit_case { run_case: prp_dup_discard_entry_timeout },
    kunit_case { run_case: prp_dup_discard_out_of_sequence },
    kunit_case { run_case: prp_dup_discard_lan_b_late },
];

#[repr(C)]
struct kunit_suite {
    name: &'static str,
    test_cases: &'static [kunit_case],
}

static prp_dup_discard_suite: kunit_suite = kunit_suite {
    name: "prp_duplicate_discard",
    test_cases: prp_dup_discard_test_cases,
};

// kunit_test_suite(prp_dup_discard_suite);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("KUnit tests for PRP duplicate discard");
// MODULE_AUTHOR("Jaakko Karrenpalo <jkarrenpalo@gmail.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
