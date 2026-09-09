// SPDX-License-Identifier: GPL-2.0
// Dependency intent: <kunit/test.h> and "protocol.h" provide the referenced
// KUnit, kernel, networking, and MPTCP declarations.

unsafe fn build_req_sock(test: *mut kunit) -> *mut mptcp_subflow_request_sock {
    let req = kunit_kzalloc(
        test,
        core::mem::size_of::<mptcp_subflow_request_sock>(),
        GFP_USER,
    );
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, req);
    mptcp_token_init_request(req as *mut request_sock);
    sock_net_set(req as *mut sock, &raw mut init_net);
    req
}

unsafe fn mptcp_token_test_req_basic(test: *mut kunit) {
    let req = build_req_sock(test);
    let null_msk: *mut mptcp_sock = core::ptr::null_mut();

    KUNIT_ASSERT_EQ(
        test,
        0,
        mptcp_token_new_request(req as *mut request_sock),
    );
    KUNIT_EXPECT_NE(test, 0, (*req).token as i32);
    KUNIT_EXPECT_PTR_EQ(test, null_msk, mptcp_token_get_sock(&raw mut init_net, (*req).token));

    /* cleanup */
    mptcp_token_destroy_request(req as *mut request_sock);
}

unsafe fn build_icsk(test: *mut kunit) -> *mut inet_connection_sock {
    let icsk = kunit_kzalloc(
        test,
        core::mem::size_of::<inet_connection_sock>(),
        GFP_USER,
    );
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, icsk);
    icsk
}

unsafe fn build_ctx(test: *mut kunit) -> *mut mptcp_subflow_context {
    let ctx = kunit_kzalloc(
        test,
        core::mem::size_of::<mptcp_subflow_context>(),
        GFP_USER,
    );
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, ctx);
    ctx
}

unsafe fn build_msk(test: *mut kunit) -> *mut mptcp_sock {
    let msk = kunit_kzalloc(test, core::mem::size_of::<mptcp_sock>(), GFP_USER);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, msk);
    refcount_set(&mut (*(msk as *mut sock)).sk_refcnt, 1);
    sock_net_set(msk as *mut sock, &raw mut init_net);

    let sk = msk as *mut sock;

    /* be sure the token helpers can dereference sk->sk_prot */
    (*sk).sk_prot = &raw mut tcp_prot;
    (*sk).sk_protocol = IPPROTO_MPTCP;

    msk
}

unsafe fn mptcp_token_test_msk_basic(test: *mut kunit) {
    let icsk = build_icsk(test);
    let ctx = build_ctx(test);
    let msk = build_msk(test);
    let null_msk: *mut mptcp_sock = core::ptr::null_mut();
    let sk: *mut sock;

    rcu_assign_pointer(&mut (*icsk).icsk_ulp_data, ctx);
    (*ctx).conn = msk as *mut sock;
    sk = msk as *mut sock;

    KUNIT_ASSERT_EQ(test, 0, mptcp_token_new_connect(icsk as *mut sock));
    KUNIT_EXPECT_NE(test, 0, (*ctx).token as i32);
    KUNIT_EXPECT_EQ(test, (*ctx).token, (*msk).token);
    KUNIT_EXPECT_PTR_EQ(test, msk, mptcp_token_get_sock(&raw mut init_net, (*ctx).token));
    KUNIT_EXPECT_EQ(test, 2, refcount_read(&(*sk).sk_refcnt) as i32);

    mptcp_token_destroy(msk);
    KUNIT_EXPECT_PTR_EQ(test, null_msk, mptcp_token_get_sock(&raw mut init_net, (*ctx).token));
}

unsafe fn mptcp_token_test_accept(test: *mut kunit) {
    let req = build_req_sock(test);
    let msk = build_msk(test);

    KUNIT_ASSERT_EQ(test, 0, mptcp_token_new_request(req as *mut request_sock));
    (*msk).token = (*req).token;
    mptcp_token_accept(req, msk);
    KUNIT_EXPECT_PTR_EQ(test, msk, mptcp_token_get_sock(&raw mut init_net, (*msk).token));

    /* this is now a no-op */
    mptcp_token_destroy_request(req as *mut request_sock);
    KUNIT_EXPECT_PTR_EQ(test, msk, mptcp_token_get_sock(&raw mut init_net, (*msk).token));

    /* cleanup */
    mptcp_token_destroy(msk);
}

unsafe fn mptcp_token_test_destroyed(test: *mut kunit) {
    let req = build_req_sock(test);
    let msk = build_msk(test);
    let null_msk: *mut mptcp_sock = core::ptr::null_mut();
    let sk = msk as *mut sock;

    KUNIT_ASSERT_EQ(test, 0, mptcp_token_new_request(req as *mut request_sock));
    (*msk).token = (*req).token;
    mptcp_token_accept(req, msk);

    /* simulate race on removal */
    refcount_set(&mut (*sk).sk_refcnt, 0);
    KUNIT_EXPECT_PTR_EQ(test, null_msk, mptcp_token_get_sock(&raw mut init_net, (*msk).token));

    /* cleanup */
    mptcp_token_destroy(msk);
}

static mut mptcp_token_test_cases: [kunit_case; 5] = [
    KUNIT_CASE(mptcp_token_test_req_basic),
    KUNIT_CASE(mptcp_token_test_msk_basic),
    KUNIT_CASE(mptcp_token_test_accept),
    KUNIT_CASE(mptcp_token_test_destroyed),
    kunit_case { ..core::mem::zeroed() },
];

static mut mptcp_token_suite: kunit_suite = kunit_suite {
    name: "mptcp-token\0".as_ptr() as *const i8,
    test_cases: mptcp_token_test_cases.as_mut_ptr(),
};

// kunit_test_suite(mptcp_token_suite);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("KUnit tests for MPTCP Token");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
