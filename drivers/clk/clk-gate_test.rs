// SPDX-License-Identifier: GPL-2.0
/*
 * Kunit tests for clk gate
 */

unsafe fn clk_gate_register_test_dev(test: *mut kunit) {
    let mut ret: *mut clk_hw;
    let pdev: *mut platform_device;

    pdev = platform_device_register_simple(c_str!("test_gate_device"), -1, core::ptr::null_mut(), 0);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, pdev);
    ret = clk_hw_register_gate(&mut (*pdev).dev, c_str!("test_gate"), core::ptr::null(), 0, core::ptr::null_mut(), 0, 0, core::ptr::null());
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, ret);
    KUNIT_EXPECT_STREQ!(test, c_str!("test_gate"), clk_hw_get_name(ret));
    KUNIT_EXPECT_EQ!(test, 0u64, clk_hw_get_flags(ret));
    clk_hw_unregister_gate(ret);
    platform_device_put(pdev);
}

unsafe fn clk_gate_register_test_parent_names(test: *mut kunit) {
    let parent = clk_hw_register_fixed_rate(core::ptr::null_mut(), c_str!("test_parent"), core::ptr::null(), 0, 1000000);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, parent);
    let ret = clk_hw_register_gate(core::ptr::null_mut(), c_str!("test_gate"), c_str!("test_parent"), 0, core::ptr::null_mut(), 0, 0, core::ptr::null());
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, ret);
    KUNIT_EXPECT_PTR_EQ!(test, parent, clk_hw_get_parent(ret));
    clk_hw_unregister_gate(ret);
    clk_hw_unregister_fixed_rate(parent);
}

unsafe fn clk_gate_register_test_parent_data(test: *mut kunit) {
    let parent = clk_hw_register_fixed_rate(core::ptr::null_mut(), c_str!("test_parent"), core::ptr::null(), 0, 1000000);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, parent);
    let mut pdata = clk_parent_data { hw: parent, name: core::ptr::null(), index: 0 };
    let ret = clk_hw_register_gate_parent_data(core::ptr::null_mut(), c_str!("test_gate"), &mut pdata, 0, core::ptr::null_mut(), 0, 0, core::ptr::null());
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, ret);
    KUNIT_EXPECT_PTR_EQ!(test, parent, clk_hw_get_parent(ret));
    clk_hw_unregister_gate(ret);
    clk_hw_unregister_fixed_rate(parent);
}

unsafe fn clk_gate_register_test_parent_data_legacy(test: *mut kunit) {
    let parent = clk_hw_register_fixed_rate(core::ptr::null_mut(), c_str!("test_parent"), core::ptr::null(), 0, 1000000);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, parent);
    let mut pdata = clk_parent_data { hw: core::ptr::null_mut(), name: c_str!("test_parent"), index: 0 };
    let ret = clk_hw_register_gate_parent_data(core::ptr::null_mut(), c_str!("test_gate"), &mut pdata, 0, core::ptr::null_mut(), 0, 0, core::ptr::null());
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, ret);
    KUNIT_EXPECT_PTR_EQ!(test, parent, clk_hw_get_parent(ret));
    clk_hw_unregister_gate(ret);
    clk_hw_unregister_fixed_rate(parent);
}

unsafe fn clk_gate_register_test_parent_hw(test: *mut kunit) {
    let parent = clk_hw_register_fixed_rate(core::ptr::null_mut(), c_str!("test_parent"), core::ptr::null(), 0, 1000000);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, parent);
    let ret = clk_hw_register_gate_parent_hw(core::ptr::null_mut(), c_str!("test_gate"), parent, 0, core::ptr::null_mut(), 0, 0, core::ptr::null());
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, ret);
    KUNIT_EXPECT_PTR_EQ!(test, parent, clk_hw_get_parent(ret));
    clk_hw_unregister_gate(ret);
    clk_hw_unregister_fixed_rate(parent);
}

unsafe fn clk_gate_register_test_hiword_invalid(test: *mut kunit) {
    let ret = clk_hw_register_gate(core::ptr::null_mut(), c_str!("test_gate"), core::ptr::null(), 0, core::ptr::null_mut(), 20, CLK_GATE_HIWORD_MASK, core::ptr::null());
    KUNIT_EXPECT_TRUE!(test, IS_ERR(ret));
}

static mut clk_gate_register_test_cases: [kunit_case; 7] = [
    KUNIT_CASE!(clk_gate_register_test_dev), KUNIT_CASE!(clk_gate_register_test_parent_names),
    KUNIT_CASE!(clk_gate_register_test_parent_data), KUNIT_CASE!(clk_gate_register_test_parent_data_legacy),
    KUNIT_CASE!(clk_gate_register_test_parent_hw), KUNIT_CASE!(clk_gate_register_test_hiword_invalid), KUNIT_CASE_END!(),
];
static mut clk_gate_register_test_suite: kunit_suite = kunit_suite { name: c_str!("clk-gate-register-test"), test_cases: clk_gate_register_test_cases.as_mut_ptr(), ..kunit_suite::default() };

#[repr(C)]
struct clk_gate_test_context { fake_mem: *mut core::ffi::c_void, hw: *mut clk_hw, parent: *mut clk_hw, fake_reg: u32 }

unsafe fn clk_gate_test_alloc_ctx(test: *mut kunit) -> *mut clk_gate_test_context {
    let ctx = kunit_kzalloc(test, core::mem::size_of::<clk_gate_test_context>(), GFP_KERNEL) as *mut clk_gate_test_context;
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, ctx);
    (*ctx).fake_mem = &mut (*ctx).fake_reg as *mut u32 as *mut core::ffi::c_void;
    ctx
}

unsafe fn clk_gate_test_parent_rate(test: *mut kunit) { let ctx = (*test).priv_ as *mut clk_gate_test_context; KUNIT_EXPECT_EQ!(test, clk_hw_get_rate((*ctx).parent), clk_hw_get_rate((*ctx).hw)); }

unsafe fn clk_gate_test_enable(test: *mut kunit) {
    let ctx = (*test).priv_ as *mut clk_gate_test_context; let parent = (*ctx).parent; let hw = (*ctx).hw; let clk = (*hw).clk; let enable_val = BIT!(5);
    KUNIT_ASSERT_EQ!(test, clk_prepare_enable(clk), 0); KUNIT_EXPECT_EQ!(test, enable_val, le32_to_cpu((*ctx).fake_reg));
    KUNIT_EXPECT_TRUE!(test, clk_hw_is_enabled(hw)); KUNIT_EXPECT_TRUE!(test, clk_hw_is_prepared(hw)); KUNIT_EXPECT_TRUE!(test, clk_hw_is_enabled(parent)); KUNIT_EXPECT_TRUE!(test, clk_hw_is_prepared(parent));
}

unsafe fn clk_gate_test_disable(test: *mut kunit) { let ctx = (*test).priv_ as *mut clk_gate_test_context; let parent = (*ctx).parent; let hw = (*ctx).hw; let clk = (*hw).clk; let enable_val = BIT!(5); KUNIT_ASSERT_EQ!(test, clk_prepare_enable(clk), 0); KUNIT_ASSERT_EQ!(test, enable_val, le32_to_cpu((*ctx).fake_reg)); clk_disable_unprepare(clk); KUNIT_EXPECT_EQ!(test, 0, le32_to_cpu((*ctx).fake_reg)); KUNIT_EXPECT_FALSE!(test, clk_hw_is_enabled(hw)); KUNIT_EXPECT_FALSE!(test, clk_hw_is_prepared(hw)); KUNIT_EXPECT_FALSE!(test, clk_hw_is_enabled(parent)); KUNIT_EXPECT_FALSE!(test, clk_hw_is_prepared(parent)); }

unsafe fn clk_gate_test_invert_enable(test: *mut kunit) { let ctx = (*test).priv_ as *mut clk_gate_test_context; let parent = (*ctx).parent; let hw = (*ctx).hw; let clk = (*hw).clk; KUNIT_ASSERT_EQ!(test, clk_prepare_enable(clk), 0); KUNIT_EXPECT_EQ!(test, 0, le32_to_cpu((*ctx).fake_reg)); KUNIT_EXPECT_TRUE!(test, clk_hw_is_enabled(hw)); KUNIT_EXPECT_TRUE!(test, clk_hw_is_prepared(hw)); KUNIT_EXPECT_TRUE!(test, clk_hw_is_enabled(parent)); KUNIT_EXPECT_TRUE!(test, clk_hw_is_prepared(parent)); }
unsafe fn clk_gate_test_invert_disable(test: *mut kunit) { let ctx = (*test).priv_ as *mut clk_gate_test_context; let parent = (*ctx).parent; let hw = (*ctx).hw; let clk = (*hw).clk; KUNIT_ASSERT_EQ!(test, clk_prepare_enable(clk), 0); KUNIT_ASSERT_EQ!(test, 0, le32_to_cpu((*ctx).fake_reg)); clk_disable_unprepare(clk); KUNIT_EXPECT_EQ!(test, BIT!(15), le32_to_cpu((*ctx).fake_reg)); KUNIT_EXPECT_FALSE!(test, clk_hw_is_enabled(hw)); KUNIT_EXPECT_FALSE!(test, clk_hw_is_prepared(hw)); KUNIT_EXPECT_FALSE!(test, clk_hw_is_enabled(parent)); KUNIT_EXPECT_FALSE!(test, clk_hw_is_prepared(parent)); }
unsafe fn clk_gate_test_hiword_enable(test: *mut kunit) { let ctx = (*test).priv_ as *mut clk_gate_test_context; let parent = (*ctx).parent; let hw = (*ctx).hw; let clk = (*hw).clk; let enable_val = BIT!(9) | BIT!(25); KUNIT_ASSERT_EQ!(test, clk_prepare_enable(clk), 0); KUNIT_EXPECT_EQ!(test, enable_val, le32_to_cpu((*ctx).fake_reg)); KUNIT_EXPECT_TRUE!(test, clk_hw_is_enabled(hw)); KUNIT_EXPECT_TRUE!(test, clk_hw_is_prepared(hw)); KUNIT_EXPECT_TRUE!(test, clk_hw_is_enabled(parent)); KUNIT_EXPECT_TRUE!(test, clk_hw_is_prepared(parent)); }
unsafe fn clk_gate_test_hiword_disable(test: *mut kunit) { let ctx = (*test).priv_ as *mut clk_gate_test_context; let parent = (*ctx).parent; let hw = (*ctx).hw; let clk = (*hw).clk; let enable_val = BIT!(9) | BIT!(25); KUNIT_ASSERT_EQ!(test, clk_prepare_enable(clk), 0); KUNIT_ASSERT_EQ!(test, enable_val, le32_to_cpu((*ctx).fake_reg)); clk_disable_unprepare(clk); KUNIT_EXPECT_EQ!(test, BIT!(25), le32_to_cpu((*ctx).fake_reg)); KUNIT_EXPECT_FALSE!(test, clk_hw_is_enabled(hw)); KUNIT_EXPECT_FALSE!(test, clk_hw_is_prepared(hw)); KUNIT_EXPECT_FALSE!(test, clk_hw_is_enabled(parent)); KUNIT_EXPECT_FALSE!(test, clk_hw_is_prepared(parent)); }
unsafe fn clk_gate_test_is_enabled(test: *mut kunit) { let ctx = clk_gate_test_alloc_ctx(test); (*ctx).fake_reg = cpu_to_le32(BIT!(7)); let hw = clk_hw_register_gate(core::ptr::null_mut(), c_str!("test_gate"), core::ptr::null(), 0, (*ctx).fake_mem, 7, 0, core::ptr::null()); KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, hw); KUNIT_ASSERT_TRUE!(test, clk_hw_is_enabled(hw)); clk_hw_unregister_gate(hw); }
unsafe fn clk_gate_test_is_disabled(test: *mut kunit) { let ctx = clk_gate_test_alloc_ctx(test); (*ctx).fake_reg = cpu_to_le32(BIT!(4)); let hw = clk_hw_register_gate(core::ptr::null_mut(), c_str!("test_gate"), core::ptr::null(), 0, (*ctx).fake_mem, 7, 0, core::ptr::null()); KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, hw); KUNIT_ASSERT_FALSE!(test, clk_hw_is_enabled(hw)); clk_hw_unregister_gate(hw); }
unsafe fn clk_gate_test_is_enabled_inverted(test: *mut kunit) { let ctx = clk_gate_test_alloc_ctx(test); (*ctx).fake_reg = cpu_to_le32(BIT!(31)); let hw = clk_hw_register_gate(core::ptr::null_mut(), c_str!("test_gate"), core::ptr::null(), 0, (*ctx).fake_mem, 2, CLK_GATE_SET_TO_DISABLE, core::ptr::null()); KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, hw); KUNIT_ASSERT_TRUE!(test, clk_hw_is_enabled(hw)); clk_hw_unregister_gate(hw); }
unsafe fn clk_gate_test_is_disabled_inverted(test: *mut kunit) { let ctx = clk_gate_test_alloc_ctx(test); (*ctx).fake_reg = cpu_to_le32(BIT!(29)); let hw = clk_hw_register_gate(core::ptr::null_mut(), c_str!("test_gate"), core::ptr::null(), 0, (*ctx).fake_mem, 29, CLK_GATE_SET_TO_DISABLE, core::ptr::null()); KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, hw); KUNIT_ASSERT_FALSE!(test, clk_hw_is_enabled(hw)); clk_hw_unregister_gate(hw); }

kunit_test_suites!(&mut clk_gate_register_test_suite);
MODULE_DESCRIPTION!(c_str!("Kunit tests for clk gate"));
MODULE_LICENSE!(c_str!("GPL v2"));

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
