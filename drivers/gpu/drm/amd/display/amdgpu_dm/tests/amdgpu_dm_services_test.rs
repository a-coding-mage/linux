// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_services.c
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel/Rust bindings are intentionally
// referenced here rather than reimplemented.

/* Tests for dm_get_elapse_time_in_ns() */

/**
 * dm_test_get_elapse_time_zero_delta - Test Get elapse time zero delta
 * @test: The KUnit test context
 */
unsafe fn dm_test_get_elapse_time_zero_delta(test: *mut kunit) {
    let ts: u64 = 1_000_000;

    KUNIT_EXPECT_EQ!(test, dm_get_elapse_time_in_ns(core::ptr::null_mut(), ts, ts), 0u64);
}

/**
 * dm_test_get_elapse_time_positive_delta - Test Get elapse time positive delta
 * @test: The KUnit test context
 */
unsafe fn dm_test_get_elapse_time_positive_delta(test: *mut kunit) {
    let current_ts: u64 = 5_000_000;
    let last_ts: u64 = 1_000_000;

    KUNIT_EXPECT_EQ!(
        test,
        dm_get_elapse_time_in_ns(core::ptr::null_mut(), current_ts, last_ts),
        4_000_000u64
    );
}

/**
 * dm_test_get_elapse_time_large_delta - Test Get elapse time large delta
 * @test: The KUnit test context
 */
unsafe fn dm_test_get_elapse_time_large_delta(test: *mut kunit) {
    let current_ts: u64 = u64::MAX;
    let last_ts: u64 = 0;

    KUNIT_EXPECT_EQ!(
        test,
        dm_get_elapse_time_in_ns(core::ptr::null_mut(), current_ts, last_ts),
        u64::MAX
    );
}

/**
 * dm_test_get_elapse_time_wraparound - Test Get elapse time wraparound
 * @test: The KUnit test context
 */
unsafe fn dm_test_get_elapse_time_wraparound(test: *mut kunit) {
    // Unsigned wraparound: result = ULLONG_MAX - last + current + 1
    let current_ts: u64 = 5;
    let last_ts: u64 = u64::MAX - 4;

    KUNIT_EXPECT_EQ!(
        test,
        dm_get_elapse_time_in_ns(core::ptr::null_mut(), current_ts, last_ts),
        10u64
    );
}

/* Tests for dm_perf_trace_timestamp() */

/**
 * dm_test_perf_trace_timestamp_basic - Test Perf trace timestamp basic
 * @test: The KUnit test context
 *
 * The tracepoint is a no-op without an attached probe, so this verifies the
 * function dereferences ctx->perf_trace safely and does not crash.
 */
unsafe fn dm_test_perf_trace_timestamp_basic(test: *mut kunit) {
    let ctx: *mut dc_context = kunit_kzalloc(test, core::mem::size_of::<dc_context>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, ctx);
    (*ctx).perf_trace = kunit_kzalloc(test, core::mem::size_of::<perf_trace>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, (*ctx).perf_trace);

    (*(*ctx).perf_trace).read_count = 10;
    (*(*ctx).perf_trace).write_count = 20;

    dm_perf_trace_timestamp(file!().as_ptr() as *const _, line!(), ctx);
}

/* Tests for dm_trace_smu_enter() */

/**
 * dm_test_trace_smu_enter_null_ctx - Test Trace smu enter null ctx
 * @test: The KUnit test context
 */
unsafe fn dm_test_trace_smu_enter_null_ctx(_test: *mut kunit) {
    // Empty stub — must not crash with NULL ctx
    dm_trace_smu_enter(0, 0, 0, core::ptr::null_mut());
}

/**
 * dm_test_trace_smu_enter_with_params - Test Trace smu enter with params
 * @test: The KUnit test context
 */
unsafe fn dm_test_trace_smu_enter_with_params(_test: *mut kunit) {
    // Exercise non-zero msg_id, param_in, and delay
    dm_trace_smu_enter(0xFF, 0x12345678, 1000, core::ptr::null_mut());
}

/* Tests for dm_trace_smu_exit() */

/**
 * dm_test_trace_smu_exit_success_null_ctx - Test Trace smu exit success null ctx
 * @test: The KUnit test context
 */
unsafe fn dm_test_trace_smu_exit_success_null_ctx(_test: *mut kunit) {
    // Empty stub — must not crash on success path with NULL ctx
    dm_trace_smu_exit(true, 0x0, core::ptr::null_mut());
}

/**
 * dm_test_trace_smu_exit_failure_null_ctx - Test Trace smu exit failure null ctx
 * @test: The KUnit test context
 */
unsafe fn dm_test_trace_smu_exit_failure_null_ctx(_test: *mut kunit) {
    // Empty stub — must not crash on failure path with NULL ctx
    dm_trace_smu_exit(false, 0x0, core::ptr::null_mut());
}

/**
 * dm_test_trace_smu_exit_with_response - Test Trace smu exit with response
 * @test: The KUnit test context
 */
unsafe fn dm_test_trace_smu_exit_with_response(_test: *mut kunit) {
    // Exercise non-zero response value
    dm_trace_smu_exit(true, 0xDEADBEEF, core::ptr::null_mut());
}

/* Tests for dm_query_extended_brightness_caps() */

/**
 * dm_test_query_brightness_caps_null_ctx - Test Query brightness caps null ctx
 * @test: The KUnit test context
 */
unsafe fn dm_test_query_brightness_caps_null_ctx(test: *mut kunit) {
    let mut caps = core::mem::zeroed::<dm_acpi_atif_backlight_caps>();

    KUNIT_EXPECT_FALSE!(
        test,
        dm_query_extended_brightness_caps(core::ptr::null_mut(), AcpiDisplayType_LCD1, &mut caps)
    );
}

/**
 * dm_test_query_brightness_caps_null_caps - Test Query brightness caps null caps
 * @test: The KUnit test context
 */
unsafe fn dm_test_query_brightness_caps_null_caps(test: *mut kunit) {
    let mut ctx = core::mem::zeroed::<dc_context>();

    ctx.driver_context = 1usize as *mut core::ffi::c_void; // non-NULL sentinel

    KUNIT_EXPECT_FALSE!(
        test,
        dm_query_extended_brightness_caps(&mut ctx, AcpiDisplayType_LCD1, core::ptr::null_mut())
    );
}

/**
 * dm_test_query_brightness_caps_null_driver_ctx - Test Query brightness caps null driver ctx
 * @test: The KUnit test context
 */
unsafe fn dm_test_query_brightness_caps_null_driver_ctx(test: *mut kunit) {
    let mut ctx = core::mem::zeroed::<dc_context>();
    let mut caps = core::mem::zeroed::<dm_acpi_atif_backlight_caps>();

    ctx.driver_context = core::ptr::null_mut();

    KUNIT_EXPECT_FALSE!(
        test,
        dm_query_extended_brightness_caps(&mut ctx, AcpiDisplayType_LCD1, &mut caps)
    );
}

/**
 * dm_test_query_brightness_caps_lcd2_null_ctx - Test Query brightness caps lcd2 null ctx
 * @test: The KUnit test context
 */
unsafe fn dm_test_query_brightness_caps_lcd2_null_ctx(test: *mut kunit) {
    let mut caps = core::mem::zeroed::<dm_acpi_atif_backlight_caps>();

    KUNIT_EXPECT_FALSE!(
        test,
        dm_query_extended_brightness_caps(core::ptr::null_mut(), AcpiDisplayType_LCD2, &mut caps)
    );
}

/**
 * dm_test_query_brightness_caps_lcd1_success - Test Query brightness caps lcd1 success
 * @test: The KUnit test context
 */
unsafe fn dm_test_query_brightness_caps_lcd1_success(test: *mut kunit) {
    let adev: *mut amdgpu_device = kunit_kzalloc(test, core::mem::size_of::<amdgpu_device>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, adev);

    let source_caps = &mut (*adev).dm.backlight_caps[0];
    source_caps.caps_valid = true;
    source_caps.min_input_signal = 12;
    source_caps.max_input_signal = 240;
    source_caps.ac_level = 80;
    source_caps.dc_level = 40;
    source_caps.data_points = 2;
    source_caps.luminance_data[0].luminance = 10;
    source_caps.luminance_data[0].input_signal = 22;
    source_caps.luminance_data[1].luminance = 90;
    source_caps.luminance_data[1].input_signal = 200;

    let mut ctx = core::mem::zeroed::<dc_context>();
    let mut caps = core::mem::zeroed::<dm_acpi_atif_backlight_caps>();
    ctx.driver_context = adev as *mut core::ffi::c_void;

    KUNIT_EXPECT_TRUE!(
        test,
        dm_query_extended_brightness_caps(&mut ctx, AcpiDisplayType_LCD1, &mut caps)
    );
    KUNIT_EXPECT_EQ!(test, caps.num_data_points, 2);
    KUNIT_EXPECT_EQ!(test, caps.max_input_signal, 240);
    KUNIT_EXPECT_EQ!(test, caps.min_input_signal, 12);
    KUNIT_EXPECT_EQ!(test, caps.ac_level_percentage, 80);
    KUNIT_EXPECT_EQ!(test, caps.dc_level_percentage, 40);
    KUNIT_EXPECT_EQ!(test, caps.data_points[0].luminance, 10);
    KUNIT_EXPECT_EQ!(test, caps.data_points[0].signal_level, 22);
    KUNIT_EXPECT_EQ!(test, caps.data_points[1].luminance, 90);
    KUNIT_EXPECT_EQ!(test, caps.data_points[1].signal_level, 200);
}

/**
 * dm_test_query_brightness_caps_non_lcd1_uses_second_slot - Test Query brightness caps non lcd1 uses second slot
 * @test: The KUnit test context
 */
unsafe fn dm_test_query_brightness_caps_non_lcd1_uses_second_slot(test: *mut kunit) {
    let adev: *mut amdgpu_device = kunit_kzalloc(test, core::mem::size_of::<amdgpu_device>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL!(test, adev);

    (*adev).dm.backlight_caps[0].caps_valid = true;
    (*adev).dm.backlight_caps[0].min_input_signal = 1;
    (*adev).dm.backlight_caps[0].max_input_signal = 2;
    let source_caps = &mut (*adev).dm.backlight_caps[1];
    source_caps.caps_valid = true;
    source_caps.min_input_signal = 33;
    source_caps.max_input_signal = 199;
    source_caps.ac_level = 70;
    source_caps.dc_level = 30;
    source_caps.data_points = 0;

    let mut ctx = core::mem::zeroed::<dc_context>();
    let mut caps = core::mem::zeroed::<dm_acpi_atif_backlight_caps>();
    ctx.driver_context = adev as *mut core::ffi::c_void;

    KUNIT_EXPECT_TRUE!(
        test,
        dm_query_extended_brightness_caps(&mut ctx, AcpiDisplayType_DFP1, &mut caps)
    );
    KUNIT_EXPECT_EQ!(test, caps.num_data_points, 0);
    KUNIT_EXPECT_EQ!(test, caps.max_input_signal, 199);
    KUNIT_EXPECT_EQ!(test, caps.min_input_signal, 33);
    KUNIT_EXPECT_EQ!(test, caps.ac_level_percentage, 70);
    KUNIT_EXPECT_EQ!(test, caps.dc_level_percentage, 30);
    KUNIT_EXPECT_EQ!(test, caps.data_points[0].luminance, 0);
    KUNIT_EXPECT_EQ!(test, caps.data_points[0].signal_level, 0);
}

static amdgpu_dm_services_test_cases: &[KunitCase] = &[
    /* dm_get_elapse_time_in_ns */
    KunitCase::new(dm_test_get_elapse_time_zero_delta),
    KunitCase::new(dm_test_get_elapse_time_positive_delta),
    KunitCase::new(dm_test_get_elapse_time_large_delta),
    KunitCase::new(dm_test_get_elapse_time_wraparound),
    /* dm_perf_trace_timestamp */
    KunitCase::new(dm_test_perf_trace_timestamp_basic),
    /* dm_trace_smu_enter */
    KunitCase::new(dm_test_trace_smu_enter_null_ctx),
    KunitCase::new(dm_test_trace_smu_enter_with_params),
    /* dm_trace_smu_exit */
    KunitCase::new(dm_test_trace_smu_exit_success_null_ctx),
    KunitCase::new(dm_test_trace_smu_exit_failure_null_ctx),
    KunitCase::new(dm_test_trace_smu_exit_with_response),
    /* dm_query_extended_brightness_caps */
    KunitCase::new(dm_test_query_brightness_caps_null_ctx),
    KunitCase::new(dm_test_query_brightness_caps_null_caps),
    KunitCase::new(dm_test_query_brightness_caps_null_driver_ctx),
    KunitCase::new(dm_test_query_brightness_caps_lcd2_null_ctx),
    KunitCase::new(dm_test_query_brightness_caps_lcd1_success),
    KunitCase::new(dm_test_query_brightness_caps_non_lcd1_uses_second_slot),
];

static amdgpu_dm_services_test_suite: KunitSuite = KunitSuite {
    name: "amdgpu_dm_services",
    test_cases: amdgpu_dm_services_test_cases,
};

// Equivalent of kunit_test_suite(amdgpu_dm_services_test_suite).
// MODULE_DESCRIPTION("KUnit tests for amdgpu_dm_services");
// MODULE_LICENSE("Dual MIT/GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
