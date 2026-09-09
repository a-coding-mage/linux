// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2023 Oracle and/or its affiliates.
 *
 * KUnit test of the handshake upcall mechanism.
 */

// C includes and kernel-provided declarations are supplied by the surrounding
// kernel translation unit.

static unsafe extern "C" fn test_accept_func(
    _req: *mut handshake_req,
    _info: *mut genl_info,
    _fd: i32,
) -> i32 { 0 }

static unsafe extern "C" fn test_done_func(
    _req: *mut handshake_req,
    _status: i32,
    _info: *mut genl_info,
) {}

#[repr(C)]
struct handshake_req_alloc_test_param {
    desc: *const core::ffi::c_char,
    proto: *mut handshake_proto,
    gfp: gfp_t,
    expect_success: bool,
}

static mut handshake_req_alloc_proto_2: handshake_proto = handshake_proto {
    hp_handler_class: HANDSHAKE_HANDLER_CLASS_NONE,
    ..unsafe { core::mem::zeroed() }
};
static mut handshake_req_alloc_proto_3: handshake_proto = handshake_proto {
    hp_handler_class: HANDSHAKE_HANDLER_CLASS_MAX,
    ..unsafe { core::mem::zeroed() }
};
static mut handshake_req_alloc_proto_4: handshake_proto = handshake_proto {
    hp_handler_class: HANDSHAKE_HANDLER_CLASS_TLSHD,
    ..unsafe { core::mem::zeroed() }
};
static mut handshake_req_alloc_proto_5: handshake_proto = handshake_proto {
    hp_handler_class: HANDSHAKE_HANDLER_CLASS_TLSHD,
    hp_accept: Some(test_accept_func),
    ..unsafe { core::mem::zeroed() }
};
static mut handshake_req_alloc_proto_6: handshake_proto = handshake_proto {
    hp_handler_class: HANDSHAKE_HANDLER_CLASS_TLSHD,
    hp_privsize: u32::MAX as _,
    hp_accept: Some(test_accept_func),
    hp_done: Some(test_done_func),
    ..unsafe { core::mem::zeroed() }
};
static mut handshake_req_alloc_proto_good: handshake_proto = handshake_proto {
    hp_handler_class: HANDSHAKE_HANDLER_CLASS_TLSHD,
    hp_accept: Some(test_accept_func),
    hp_done: Some(test_done_func),
    ..unsafe { core::mem::zeroed() }
};

static handshake_req_alloc_params: [handshake_req_alloc_test_param; 7] = [
    handshake_req_alloc_test_param { desc: c"handshake_req_alloc NULL proto".as_ptr(), proto: core::ptr::null_mut(), gfp: GFP_KERNEL, expect_success: false },
    handshake_req_alloc_test_param { desc: c"handshake_req_alloc CLASS_NONE".as_ptr(), proto: unsafe { &raw mut handshake_req_alloc_proto_2 }, gfp: GFP_KERNEL, expect_success: false },
    handshake_req_alloc_test_param { desc: c"handshake_req_alloc CLASS_MAX".as_ptr(), proto: unsafe { &raw mut handshake_req_alloc_proto_3 }, gfp: GFP_KERNEL, expect_success: false },
    handshake_req_alloc_test_param { desc: c"handshake_req_alloc no callbacks".as_ptr(), proto: unsafe { &raw mut handshake_req_alloc_proto_4 }, gfp: GFP_KERNEL, expect_success: false },
    handshake_req_alloc_test_param { desc: c"handshake_req_alloc no done callback".as_ptr(), proto: unsafe { &raw mut handshake_req_alloc_proto_5 }, gfp: GFP_KERNEL, expect_success: false },
    handshake_req_alloc_test_param { desc: c"handshake_req_alloc excessive privsize".as_ptr(), proto: unsafe { &raw mut handshake_req_alloc_proto_6 }, gfp: GFP_KERNEL | __GFP_NOWARN, expect_success: false },
    handshake_req_alloc_test_param { desc: c"handshake_req_alloc all good".as_ptr(), proto: unsafe { &raw mut handshake_req_alloc_proto_good }, gfp: GFP_KERNEL, expect_success: true },
];

unsafe extern "C" fn handshake_req_alloc_get_desc(param: *const handshake_req_alloc_test_param, desc: *mut core::ffi::c_char) {
    strscpy(desc, (*param).desc, KUNIT_PARAM_DESC_SIZE);
}

/* Creates the function handshake_req_alloc_gen_params */
// KUNIT_ARRAY_PARAM(handshake_req_alloc, handshake_req_alloc_params, handshake_req_alloc_get_desc);

unsafe extern "C" fn handshake_req_alloc_case(test: *mut kunit) {
    let param = (*test).param_value as *const handshake_req_alloc_test_param;
    let result = handshake_req_alloc((*param).proto, (*param).gfp);
    if (*param).expect_success { KUNIT_EXPECT_NOT_NULL(test, result); }
    else { KUNIT_EXPECT_NULL(test, result); }
    kfree(result);
}

unsafe extern "C" fn handshake_req_submit_test1(test: *mut kunit) {
    let mut sock: *mut socket = core::ptr::null_mut();
    let err = __sock_create(&raw mut init_net, PF_INET, SOCK_STREAM, IPPROTO_TCP, &mut sock, 1);
    KUNIT_ASSERT_EQ(test, err, 0);
    let result = handshake_req_submit(sock, core::ptr::null_mut(), GFP_KERNEL);
    KUNIT_EXPECT_EQ(test, result, -EINVAL);
    sock_release(sock);
}

unsafe extern "C" fn handshake_req_submit_test2(test: *mut kunit) {
    let req = handshake_req_alloc(&raw mut handshake_req_alloc_proto_good, GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, req);
    let result = handshake_req_submit(core::ptr::null_mut(), req, GFP_KERNEL);
    KUNIT_EXPECT_EQ(test, result, -EINVAL);
}

unsafe extern "C" fn handshake_req_submit_test3(test: *mut kunit) {
    let req = handshake_req_alloc(&raw mut handshake_req_alloc_proto_good, GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, req);
    let mut sock: *mut socket = core::ptr::null_mut();
    let err = __sock_create(&raw mut init_net, PF_INET, SOCK_STREAM, IPPROTO_TCP, &mut sock, 1);
    KUNIT_ASSERT_EQ(test, err, 0);
    (*sock).file = core::ptr::null_mut();
    let result = handshake_req_submit(sock, req, GFP_KERNEL);
    KUNIT_EXPECT_EQ(test, result, -EINVAL);
    sock_release(sock);
}

unsafe extern "C" fn handshake_req_submit_test4(test: *mut kunit) {
    let req = handshake_req_alloc(&raw mut handshake_req_alloc_proto_good, GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, req);
    let mut sock: *mut socket = core::ptr::null_mut();
    let err = __sock_create(&raw mut init_net, PF_INET, SOCK_STREAM, IPPROTO_TCP, &mut sock, 1);
    KUNIT_ASSERT_EQ(test, err, 0);
    let filp = sock_alloc_file(sock, O_NONBLOCK, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, filp);
    KUNIT_ASSERT_NOT_NULL(test, (*sock).sk);
    (*sock).file = filp;
    let fcount_before = file_count(filp);
    let err = handshake_req_submit(sock, req, GFP_KERNEL);
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_EXPECT_EQ(test, file_count(filp), fcount_before + 1);
    let result = handshake_req_hash_lookup((*sock).sk);
    KUNIT_EXPECT_NOT_NULL(test, result);
    KUNIT_EXPECT_PTR_EQ(test, req, result);
    handshake_req_cancel((*sock).sk);
    KUNIT_EXPECT_EQ(test, file_count(filp), fcount_before);
    fput(filp);
}

unsafe extern "C" fn handshake_req_submit_test5(test: *mut kunit) {
    let req = handshake_req_alloc(&raw mut handshake_req_alloc_proto_good, GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, req);
    let mut sock: *mut socket = core::ptr::null_mut();
    let err = __sock_create(&raw mut init_net, PF_INET, SOCK_STREAM, IPPROTO_TCP, &mut sock, 1);
    KUNIT_ASSERT_EQ(test, err, 0);
    let filp = sock_alloc_file(sock, O_NONBLOCK, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, filp);
    KUNIT_ASSERT_NOT_NULL(test, (*sock).sk);
    (*sock).file = filp;
    let hn = handshake_pernet(sock_net((*sock).sk));
    KUNIT_ASSERT_NOT_NULL(test, hn);
    let saved = (*hn).hn_pending;
    (*hn).hn_pending = (*hn).hn_pending_max + 1;
    let before = file_count(filp);
    let err = handshake_req_submit(sock, req, GFP_KERNEL);
    KUNIT_EXPECT_EQ(test, err, -EAGAIN);
    KUNIT_EXPECT_EQ(test, file_count(filp), before);
    fput(filp);
    (*hn).hn_pending = saved;
}

unsafe extern "C" fn handshake_req_submit_test6(test: *mut kunit) {
    let req1 = handshake_req_alloc(&raw mut handshake_req_alloc_proto_good, GFP_KERNEL);
    let req2 = handshake_req_alloc(&raw mut handshake_req_alloc_proto_good, GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, req1); KUNIT_ASSERT_NOT_NULL(test, req2);
    let mut sock: *mut socket = core::ptr::null_mut();
    let err = __sock_create(&raw mut init_net, PF_INET, SOCK_STREAM, IPPROTO_TCP, &mut sock, 1);
    KUNIT_ASSERT_EQ(test, err, 0);
    let filp = sock_alloc_file(sock, O_NONBLOCK, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, filp); (*sock).file = filp;
    let before = file_count(filp);
    let err = handshake_req_submit(sock, req1, GFP_KERNEL); KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_EXPECT_EQ(test, file_count(filp), before + 1);
    let err = handshake_req_submit(sock, req2, GFP_KERNEL);
    KUNIT_EXPECT_EQ(test, err, -EBUSY);
    handshake_req_cancel((*sock).sk); KUNIT_EXPECT_EQ(test, file_count(filp), before); fput(filp);
}

unsafe extern "C" fn handshake_req_cancel_test1(test: *mut kunit) {
    let req = handshake_req_alloc(&raw mut handshake_req_alloc_proto_good, GFP_KERNEL); KUNIT_ASSERT_NOT_NULL(test, req);
    let mut sock: *mut socket = core::ptr::null_mut(); let err = __sock_create(&raw mut init_net, PF_INET, SOCK_STREAM, IPPROTO_TCP, &mut sock, 1); KUNIT_ASSERT_EQ(test, err, 0);
    let filp = sock_alloc_file(sock, O_NONBLOCK, core::ptr::null_mut()); KUNIT_ASSERT_NOT_ERR_OR_NULL(test, filp); (*sock).file = filp;
    let before = file_count(filp); KUNIT_ASSERT_EQ(test, handshake_req_submit(sock, req, GFP_KERNEL), 0);
    let result = handshake_req_cancel((*sock).sk); KUNIT_EXPECT_TRUE(test, result); KUNIT_EXPECT_EQ(test, file_count(filp), before); fput(filp);
}

static mut handshake_req_destroy_test: *mut handshake_req = core::ptr::null_mut();
unsafe extern "C" fn test_destroy_func(req: *mut handshake_req) { handshake_req_destroy_test = req; }
static mut handshake_req_alloc_proto_destroy: handshake_proto = handshake_proto { hp_handler_class: HANDSHAKE_HANDLER_CLASS_TLSHD, hp_accept: Some(test_accept_func), hp_done: Some(test_done_func), hp_destroy: Some(test_destroy_func), ..unsafe { core::mem::zeroed() } };

unsafe extern "C" fn handshake_req_destroy_test1(test: *mut kunit) {
    handshake_req_destroy_test = core::ptr::null_mut();
    let req = handshake_req_alloc(&raw mut handshake_req_alloc_proto_destroy, GFP_KERNEL); KUNIT_ASSERT_NOT_NULL(test, req);
    let mut sock: *mut socket = core::ptr::null_mut(); let err = __sock_create(&raw mut init_net, PF_INET, SOCK_STREAM, IPPROTO_TCP, &mut sock, 1); KUNIT_ASSERT_EQ(test, err, 0);
    let filp = sock_alloc_file(sock, O_NONBLOCK, core::ptr::null_mut()); KUNIT_ASSERT_NOT_ERR_OR_NULL(test, filp); (*sock).file = filp;
    let before = file_count(filp); KUNIT_ASSERT_EQ(test, handshake_req_submit(sock, req, GFP_KERNEL), 0); handshake_req_cancel((*sock).sk);
    KUNIT_EXPECT_EQ(test, file_count(filp), before); __fput_sync(filp); KUNIT_EXPECT_PTR_EQ(test, handshake_req_destroy_test, req);
}

// KUNIT_ARRAY_PARAM and KUNIT test-suite registration are kernel build-time
// declarations; their original case names and callbacks remain available here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
