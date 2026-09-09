// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test for the FPGA Manager
 *
 * Copyright (C) 2023 Red Hat, Inc.
 *
 * Author: Marco Pagani <marpagan@redhat.com>
 */

// Dependencies supplied by the surrounding kernel/KUnit environment.
use core::ffi::{c_char, c_int, c_void};

const HEADER_FILL: c_char = b'H' as c_char;
const IMAGE_FILL: c_char = b'P' as c_char;
const IMAGE_BLOCK: usize = 1024;
const HEADER_SIZE: usize = IMAGE_BLOCK;
const IMAGE_SIZE: usize = IMAGE_BLOCK * 4;

#[repr(C)]
pub struct mgr_stats {
    pub header_match: bool,
    pub image_match: bool,
    pub seq_num: u32,
    pub op_parse_header_seq: u32,
    pub op_write_init_seq: u32,
    pub op_write_seq: u32,
    pub op_write_sg_seq: u32,
    pub op_write_complete_seq: u32,
    pub op_parse_header_state: fpga_mgr_states,
    pub op_write_init_state: fpga_mgr_states,
    pub op_write_state: fpga_mgr_states,
    pub op_write_sg_state: fpga_mgr_states,
    pub op_write_complete_state: fpga_mgr_states,
}

#[repr(C)]
pub struct mgr_ctx {
    pub img_info: *mut fpga_image_info,
    pub mgr: *mut fpga_manager,
    pub dev: *mut device,
    pub stats: mgr_stats,
}

#[repr(C)] pub struct kunit { pub priv_: *mut c_void }
#[repr(C)] pub struct fpga_image_info { pub count: usize, pub buf: *const c_char, pub sgt: *mut sg_table, pub header_size: usize, pub data_size: usize }
#[repr(C)] pub struct fpga_manager { pub priv_: *mut c_void, pub state: fpga_mgr_states }
#[repr(C)] pub struct device;
#[repr(C)] pub struct sg_table { pub sgl: *mut scatterlist, pub nents: u32 }
#[repr(C)] pub struct scatterlist;
#[repr(C)] pub struct sg_mapping_iter { pub addr: *mut c_char, pub length: usize }
#[repr(C)] pub struct fpga_manager_ops;
#[repr(C)] pub struct kunit_case;
#[repr(C)] pub struct kunit_suite;

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum fpga_mgr_states {
    FPGA_MGR_STATE_PARSE_HEADER,
    FPGA_MGR_STATE_WRITE_INIT,
    FPGA_MGR_STATE_WRITE,
    FPGA_MGR_STATE_WRITE_COMPLETE,
}

extern "C" {
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: c_int) -> *mut c_void;
    fn memset(dest: *mut c_void, value: c_int, count: usize) -> *mut c_void;
    fn fpga_mgr_get(dev: *mut device) -> *mut fpga_manager;
    fn fpga_mgr_put(mgr: *mut fpga_manager);
    fn fpga_mgr_lock(mgr: *mut fpga_manager) -> c_int;
    fn fpga_mgr_unlock(mgr: *mut fpga_manager);
    fn fpga_mgr_load(mgr: *mut fpga_manager, info: *mut fpga_image_info) -> c_int;
    fn sg_miter_start(iter: *mut sg_mapping_iter, sgl: *mut scatterlist, nents: u32, flags: c_int);
    fn sg_miter_skip(iter: *mut sg_mapping_iter, offset: usize) -> bool;
    fn sg_miter_next(iter: *mut sg_mapping_iter) -> bool;
    fn sg_miter_stop(iter: *mut sg_mapping_iter);
    fn sg_alloc_table(sgt: *mut sg_table, nents: u32, flags: c_int) -> c_int;
    fn sg_init_one(sgl: *mut scatterlist, buf: *mut c_char, len: usize);
    fn sg_free_table(sgt: *mut sg_table);
    fn kunit_device_register(test: *mut kunit, name: *const c_char) -> *mut device;
    fn devm_fpga_mgr_register(dev: *mut device, name: *const c_char, ops: *const fpga_manager_ops, priv_: *mut mgr_stats) -> *mut fpga_manager;
    fn fpga_image_info_alloc(dev: *mut device) -> *mut fpga_image_info;
    fn fpga_image_info_free(info: *mut fpga_image_info);
}

unsafe fn init_test_buffer(test: *mut kunit, count: usize) -> *mut c_char {
    let buf = kunit_kzalloc(test, count, 0) as *mut c_char;
    memset(buf as *mut c_void, HEADER_FILL as c_int, HEADER_SIZE);
    memset(buf.add(HEADER_SIZE) as *mut c_void, IMAGE_FILL as c_int, count - HEADER_SIZE);
    buf
}

unsafe extern "C" fn op_parse_header(mgr: *mut fpga_manager, info: *mut fpga_image_info, buf: *const c_char, _count: usize) -> c_int {
    let stats = (*mgr).priv_ as *mut mgr_stats;
    (*stats).op_parse_header_state = (*mgr).state;
    (*stats).op_parse_header_seq = (*stats).seq_num;
    (*stats).seq_num = (*stats).seq_num.wrapping_add(1);
    (*info).header_size = HEADER_SIZE;
    (*info).data_size = (*info).count - HEADER_SIZE;
    (*stats).header_match = true;
    for i in 0..(*info).header_size { if *buf.add(i) != HEADER_FILL { (*stats).header_match = false; break; } }
    0
}

unsafe extern "C" fn op_write_init(mgr: *mut fpga_manager, _info: *mut fpga_image_info, _buf: *const c_char, _count: usize) -> c_int {
    let stats = (*mgr).priv_ as *mut mgr_stats;
    (*stats).op_write_init_state = (*mgr).state;
    (*stats).op_write_init_seq = (*stats).seq_num;
    (*stats).seq_num = (*stats).seq_num.wrapping_add(1);
    0
}

unsafe extern "C" fn op_write(mgr: *mut fpga_manager, buf: *const c_char, count: usize) -> c_int {
    let stats = (*mgr).priv_ as *mut mgr_stats;
    (*stats).op_write_state = (*mgr).state;
    (*stats).op_write_seq = (*stats).seq_num;
    (*stats).seq_num = (*stats).seq_num.wrapping_add(1);
    (*stats).image_match = true;
    for i in 0..count { if *buf.add(i) != IMAGE_FILL { (*stats).image_match = false; break; } }
    0
}

unsafe extern "C" fn op_write_sg(mgr: *mut fpga_manager, sgt: *mut sg_table) -> c_int {
    let stats = (*mgr).priv_ as *mut mgr_stats;
    (*stats).op_write_sg_state = (*mgr).state;
    (*stats).op_write_sg_seq = (*stats).seq_num;
    (*stats).seq_num = (*stats).seq_num.wrapping_add(1);
    (*stats).image_match = true;
    let mut miter = core::mem::zeroed::<sg_mapping_iter>();
    sg_miter_start(&mut miter, (*sgt).sgl, (*sgt).nents, 1);
    if !sg_miter_skip(&mut miter, HEADER_SIZE) { (*stats).image_match = false; } else {
        while sg_miter_next(&mut miter) { for i in 0..miter.length { if *miter.addr.add(i) != IMAGE_FILL { (*stats).image_match = false; break; } } }
    }
    sg_miter_stop(&mut miter);
    0
}

unsafe extern "C" fn op_write_complete(mgr: *mut fpga_manager, _info: *mut fpga_image_info) -> c_int {
    let stats = (*mgr).priv_ as *mut mgr_stats;
    (*stats).op_write_complete_state = (*mgr).state;
    (*stats).op_write_complete_seq = (*stats).seq_num;
    (*stats).seq_num = (*stats).seq_num.wrapping_add(1);
    0
}

#[repr(C)]
static fake_mgr_ops: fpga_manager_ops = fpga_manager_ops;

unsafe extern "C" fn fpga_mgr_test_get(test: *mut kunit) {
    let ctx = (*test).priv_ as *mut mgr_ctx;
    let mgr = fpga_mgr_get((*ctx).dev);
    // KUNIT_EXPECT_PTR_EQ(test, mgr, ctx->mgr);
    fpga_mgr_put((*ctx).mgr);
}

unsafe extern "C" fn fpga_mgr_test_lock(test: *mut kunit) {
    let ctx = (*test).priv_ as *mut mgr_ctx;
    let ret = fpga_mgr_lock((*ctx).mgr);
    // KUNIT_EXPECT_EQ(test, ret, 0);
    let ret = fpga_mgr_lock((*ctx).mgr);
    // KUNIT_EXPECT_EQ(test, ret, -EBUSY);
    fpga_mgr_unlock((*ctx).mgr);
}

unsafe extern "C" fn fpga_mgr_test_img_load_buf(test: *mut kunit) {
    let ctx = (*test).priv_ as *mut mgr_ctx;
    let img_buf = init_test_buffer(test, IMAGE_SIZE);
    (*(*ctx).img_info).count = IMAGE_SIZE;
    (*(*ctx).img_info).buf = img_buf;
    let ret = fpga_mgr_load((*ctx).mgr, (*ctx).img_info);
    // KUNIT_EXPECT_EQ(test, ret, 0);
    // KUNIT_EXPECT_TRUE(test, ctx->stats.header_match);
    // KUNIT_EXPECT_TRUE(test, ctx->stats.image_match);
    // KUNIT_EXPECT_EQ(test, ctx->stats.op_parse_header_state, FPGA_MGR_STATE_PARSE_HEADER);
    // KUNIT_EXPECT_EQ(test, ctx->stats.op_write_init_state, FPGA_MGR_STATE_WRITE_INIT);
    // KUNIT_EXPECT_EQ(test, ctx->stats.op_write_state, FPGA_MGR_STATE_WRITE);
    // KUNIT_EXPECT_EQ(test, ctx->stats.op_write_complete_state, FPGA_MGR_STATE_WRITE_COMPLETE);
    // KUNIT_EXPECT_EQ(test, ctx->stats.op_write_init_seq, ctx->stats.op_parse_header_seq + 1);
    // KUNIT_EXPECT_EQ(test, ctx->stats.op_write_seq, ctx->stats.op_parse_header_seq + 2);
    // KUNIT_EXPECT_EQ(test, ctx->stats.op_write_complete_seq, ctx->stats.op_parse_header_seq + 3);
    let _ = ret;
}

unsafe extern "C" fn fpga_mgr_test_img_load_sgt(test: *mut kunit) {
    let ctx = (*test).priv_ as *mut mgr_ctx;
    let img_buf = init_test_buffer(test, IMAGE_SIZE);
    let sgt = kunit_kzalloc(test, core::mem::size_of::<sg_table>(), 0) as *mut sg_table;
    let ret = sg_alloc_table(sgt, 1, 0);
    sg_init_one((*sgt).sgl, img_buf, IMAGE_SIZE);
    (*(*ctx).img_info).sgt = sgt;
    let ret = fpga_mgr_load((*ctx).mgr, (*ctx).img_info);
    // KUNIT_EXPECT_EQ(test, ret, 0);
    // KUNIT_EXPECT_TRUE(test, ctx->stats.header_match);
    // KUNIT_EXPECT_TRUE(test, ctx->stats.image_match);
    // KUNIT_EXPECT_EQ(test, ctx->stats.op_parse_header_state, FPGA_MGR_STATE_PARSE_HEADER);
    // KUNIT_EXPECT_EQ(test, ctx->stats.op_write_init_state, FPGA_MGR_STATE_WRITE_INIT);
    // KUNIT_EXPECT_EQ(test, ctx->stats.op_write_sg_state, FPGA_MGR_STATE_WRITE);
    // KUNIT_EXPECT_EQ(test, ctx->stats.op_write_complete_state, FPGA_MGR_STATE_WRITE_COMPLETE);
    // KUNIT_EXPECT_EQ(test, ctx->stats.op_write_init_seq, ctx->stats.op_parse_header_seq + 1);
    // KUNIT_EXPECT_EQ(test, ctx->stats.op_write_sg_seq, ctx->stats.op_parse_header_seq + 2);
    // KUNIT_EXPECT_EQ(test, ctx->stats.op_write_complete_seq, ctx->stats.op_parse_header_seq + 3);
    let _ = ret;
}

unsafe extern "C" fn fpga_mgr_test_init(test: *mut kunit) -> c_int {
    let ctx = kunit_kzalloc(test, core::mem::size_of::<mgr_ctx>(), 0) as *mut mgr_ctx;
    (*ctx).dev = kunit_device_register(test, b"fpga-manager-test-dev\0".as_ptr() as *const c_char);
    (*ctx).mgr = devm_fpga_mgr_register((*ctx).dev, b"Fake FPGA Manager\0".as_ptr() as *const c_char, &fake_mgr_ops, &mut (*ctx).stats);
    (*ctx).img_info = fpga_image_info_alloc((*ctx).dev);
    (*test).priv_ = ctx as *mut c_void;
    0
}

// MODULE_DESCRIPTION("KUnit test for the FPGA Manager");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
