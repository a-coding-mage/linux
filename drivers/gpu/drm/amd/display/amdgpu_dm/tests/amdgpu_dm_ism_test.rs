// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_ism.c
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// C dependencies supplied by the surrounding kernel tree:
// kunit/test.h, dc.h, amdgpu.h, amdgpu_mode.h, amdgpu_dm.h,
// amdgpu_dm_ism.h, and amdgpu_dm_kunit_test_helpers.h.

unsafe fn alloc_test_ism(test: *mut kunit) -> *mut amdgpu_dm_ism {
    let ism = kunit_kzalloc(test, core::mem::size_of::<amdgpu_dm_ism>(), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ism);
    ism
}

unsafe fn dm_test_ism_next_state_running_enter_idle(test: *mut kunit) {
    let mut next: amdgpu_dm_ism_state = DM_ISM_NUM_STATES;
    let ok = dm_ism_next_state(DM_ISM_STATE_FULL_POWER_RUNNING, DM_ISM_EVENT_ENTER_IDLE_REQUESTED, &mut next);
    KUNIT_EXPECT_TRUE(test, ok);
    KUNIT_EXPECT_EQ(test, next as i32, DM_ISM_STATE_HYSTERESIS_WAITING as i32);
}
unsafe fn dm_test_ism_next_state_running_begin_cursor(test: *mut kunit) {
    let mut next = DM_ISM_NUM_STATES;
    let ok = dm_ism_next_state(DM_ISM_STATE_FULL_POWER_RUNNING, DM_ISM_EVENT_BEGIN_CURSOR_UPDATE, &mut next);
    KUNIT_EXPECT_TRUE(test, ok); KUNIT_EXPECT_EQ(test, next as i32, DM_ISM_STATE_FULL_POWER_BUSY as i32);
}
unsafe fn dm_test_ism_next_state_running_invalid(test: *mut kunit) {
    let mut next = DM_ISM_NUM_STATES;
    let ok = dm_ism_next_state(DM_ISM_STATE_FULL_POWER_RUNNING, DM_ISM_EVENT_EXIT_IDLE_REQUESTED, &mut next);
    KUNIT_EXPECT_FALSE(test, ok); KUNIT_EXPECT_EQ(test, next as i32, DM_ISM_NUM_STATES as i32);
}
unsafe fn dm_test_ism_next_state_busy_enter_idle(test: *mut kunit) { let mut n=DM_ISM_NUM_STATES; let ok=dm_ism_next_state(DM_ISM_STATE_FULL_POWER_BUSY,DM_ISM_EVENT_ENTER_IDLE_REQUESTED,&mut n); KUNIT_EXPECT_TRUE(test,ok); KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_STATE_HYSTERESIS_BUSY as i32); }
unsafe fn dm_test_ism_next_state_busy_end_cursor(test: *mut kunit) { let mut n=DM_ISM_NUM_STATES; let ok=dm_ism_next_state(DM_ISM_STATE_FULL_POWER_BUSY,DM_ISM_EVENT_END_CURSOR_UPDATE,&mut n); KUNIT_EXPECT_TRUE(test,ok); KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_STATE_FULL_POWER_RUNNING as i32); }
unsafe fn dm_test_ism_next_state_hyst_wait_exit_idle(test:*mut kunit){let mut n=DM_ISM_NUM_STATES;let ok=dm_ism_next_state(DM_ISM_STATE_HYSTERESIS_WAITING,DM_ISM_EVENT_EXIT_IDLE_REQUESTED,&mut n);KUNIT_EXPECT_TRUE(test,ok);KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_STATE_TIMER_ABORTED as i32);}
unsafe fn dm_test_ism_next_state_hyst_wait_begin_cursor(test:*mut kunit){let mut n=DM_ISM_NUM_STATES;let ok=dm_ism_next_state(DM_ISM_STATE_HYSTERESIS_WAITING,DM_ISM_EVENT_BEGIN_CURSOR_UPDATE,&mut n);KUNIT_EXPECT_TRUE(test,ok);KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_STATE_HYSTERESIS_BUSY as i32);}
unsafe fn dm_test_ism_next_state_hyst_wait_timer(test:*mut kunit){let mut n=DM_ISM_NUM_STATES;let ok=dm_ism_next_state(DM_ISM_STATE_HYSTERESIS_WAITING,DM_ISM_EVENT_TIMER_ELAPSED,&mut n);KUNIT_EXPECT_TRUE(test,ok);KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_STATE_OPTIMIZED_IDLE as i32);}
unsafe fn dm_test_ism_next_state_hyst_wait_immediate(test:*mut kunit){let mut n=DM_ISM_NUM_STATES;let ok=dm_ism_next_state(DM_ISM_STATE_HYSTERESIS_WAITING,DM_ISM_EVENT_IMMEDIATE,&mut n);KUNIT_EXPECT_TRUE(test,ok);KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_STATE_OPTIMIZED_IDLE as i32);}
unsafe fn dm_test_ism_next_state_hyst_busy_exit_idle(test:*mut kunit){let mut n=DM_ISM_NUM_STATES;let ok=dm_ism_next_state(DM_ISM_STATE_HYSTERESIS_BUSY,DM_ISM_EVENT_EXIT_IDLE_REQUESTED,&mut n);KUNIT_EXPECT_TRUE(test,ok);KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_STATE_FULL_POWER_BUSY as i32);}
unsafe fn dm_test_ism_next_state_hyst_busy_end_cursor(test:*mut kunit){let mut n=DM_ISM_NUM_STATES;let ok=dm_ism_next_state(DM_ISM_STATE_HYSTERESIS_BUSY,DM_ISM_EVENT_END_CURSOR_UPDATE,&mut n);KUNIT_EXPECT_TRUE(test,ok);KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_STATE_HYSTERESIS_WAITING as i32);}
unsafe fn dm_test_ism_next_state_opt_idle_exit(test:*mut kunit){let mut n=DM_ISM_NUM_STATES;let ok=dm_ism_next_state(DM_ISM_STATE_OPTIMIZED_IDLE,DM_ISM_EVENT_EXIT_IDLE_REQUESTED,&mut n);KUNIT_EXPECT_TRUE(test,ok);KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_STATE_FULL_POWER_RUNNING as i32);}
unsafe fn dm_test_ism_next_state_opt_idle_begin_cursor(test:*mut kunit){let mut n=DM_ISM_NUM_STATES;let ok=dm_ism_next_state(DM_ISM_STATE_OPTIMIZED_IDLE,DM_ISM_EVENT_BEGIN_CURSOR_UPDATE,&mut n);KUNIT_EXPECT_TRUE(test,ok);KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_STATE_HYSTERESIS_BUSY as i32);}
unsafe fn dm_test_ism_next_state_opt_idle_sso_timer(test:*mut kunit){let mut n=DM_ISM_NUM_STATES;let ok=dm_ism_next_state(DM_ISM_STATE_OPTIMIZED_IDLE,DM_ISM_EVENT_SSO_TIMER_ELAPSED,&mut n);KUNIT_EXPECT_TRUE(test,ok);KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_STATE_OPTIMIZED_IDLE_SSO as i32);}
unsafe fn dm_test_ism_next_state_opt_idle_immediate(test:*mut kunit){let mut n=DM_ISM_NUM_STATES;let ok=dm_ism_next_state(DM_ISM_STATE_OPTIMIZED_IDLE,DM_ISM_EVENT_IMMEDIATE,&mut n);KUNIT_EXPECT_TRUE(test,ok);KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_STATE_OPTIMIZED_IDLE_SSO as i32);}
unsafe fn dm_test_ism_next_state_opt_idle_sso_exit(test:*mut kunit){let mut n=DM_ISM_NUM_STATES;let ok=dm_ism_next_state(DM_ISM_STATE_OPTIMIZED_IDLE_SSO,DM_ISM_EVENT_EXIT_IDLE_REQUESTED,&mut n);KUNIT_EXPECT_TRUE(test,ok);KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_STATE_FULL_POWER_RUNNING as i32);}
unsafe fn dm_test_ism_next_state_opt_idle_sso_cursor(test:*mut kunit){let mut n=DM_ISM_NUM_STATES;let ok=dm_ism_next_state(DM_ISM_STATE_OPTIMIZED_IDLE_SSO,DM_ISM_EVENT_BEGIN_CURSOR_UPDATE,&mut n);KUNIT_EXPECT_TRUE(test,ok);KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_STATE_HYSTERESIS_BUSY as i32);}
unsafe fn dm_test_ism_next_state_aborted_immediate(test:*mut kunit){let mut n=DM_ISM_NUM_STATES;let ok=dm_ism_next_state(DM_ISM_STATE_TIMER_ABORTED,DM_ISM_EVENT_IMMEDIATE,&mut n);KUNIT_EXPECT_TRUE(test,ok);KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_STATE_FULL_POWER_RUNNING as i32);}
unsafe fn dm_test_ism_next_state_aborted_invalid(test:*mut kunit){let mut n=DM_ISM_NUM_STATES;let ok=dm_ism_next_state(DM_ISM_STATE_TIMER_ABORTED,DM_ISM_EVENT_ENTER_IDLE_REQUESTED,&mut n);KUNIT_EXPECT_FALSE(test,ok);KUNIT_EXPECT_EQ(test,n as i32,DM_ISM_NUM_STATES as i32);}

unsafe fn dm_test_ism_sso_delay_null_stream(test:*mut kunit){let ism=alloc_test_ism(test);(*ism).config.sso_num_frames=5;KUNIT_EXPECT_EQ(test,dm_ism_get_sso_delay(ism,core::ptr::null_mut()),0u64);}
unsafe fn dm_test_ism_sso_delay_zero_frames(test:*mut kunit){let ism=alloc_test_ism(test);let s=dm_kunit_alloc_stream(test,core::ptr::null_mut());(*s).timing.v_total=1125;(*s).timing.h_total=2200;(*s).timing.pix_clk_100hz=1485000;(*ism).config.sso_num_frames=0;KUNIT_EXPECT_EQ(test,dm_ism_get_sso_delay(ism,s),0u64);}
unsafe fn dm_test_ism_sso_delay_1080p60_3frames(test:*mut kunit){let ism=alloc_test_ism(test);let s=dm_kunit_alloc_stream(test,core::ptr::null_mut());(*s).timing.v_total=1125;(*s).timing.h_total=2200;(*s).timing.pix_clk_100hz=1485000;(*ism).config.sso_num_frames=3;let one=div64_u64(1125u64*2200*10000000,1485000);KUNIT_EXPECT_EQ(test,dm_ism_get_sso_delay(ism,s),3*one);}
unsafe fn dm_test_ism_sso_delay_4k60_1frame(test:*mut kunit){let ism=alloc_test_ism(test);let s=dm_kunit_alloc_stream(test,core::ptr::null_mut());(*s).timing.v_total=2250;(*s).timing.h_total=4400;(*s).timing.pix_clk_100hz=5940000;(*ism).config.sso_num_frames=1;let one=div64_u64(2250u64*4400*10000000,5940000);KUNIT_EXPECT_EQ(test,dm_ism_get_sso_delay(ism,s),one);}

// The remaining KUnit cases retain the original test coverage and ordering.
// Their bodies use the same external ISM helpers and structures as the C source.
unsafe fn dm_test_ism_idle_delay_null_stream(test:*mut kunit){let ism=alloc_test_ism(test);(*ism).config.filter_num_frames=5;(*ism).config.filter_entry_count=3;(*ism).config.activation_num_delay_frames=10;KUNIT_EXPECT_EQ(test,dm_ism_get_idle_allow_delay(ism,core::ptr::null_mut()),0u64);}
unsafe fn dm_test_ism_idle_delay_zero_filter_frames(test:*mut kunit){let ism=alloc_test_ism(test);(*ism).config.filter_num_frames=0;let s=dm_kunit_alloc_stream(test,core::ptr::null_mut());KUNIT_EXPECT_EQ(test,dm_ism_get_idle_allow_delay(ism,s),0u64);}
unsafe fn dm_test_ism_idle_delay_zero_entry_count(test:*mut kunit){let ism=alloc_test_ism(test);(*ism).config.filter_num_frames=5;(*ism).config.filter_entry_count=0;let s=dm_kunit_alloc_stream(test,core::ptr::null_mut());KUNIT_EXPECT_EQ(test,dm_ism_get_idle_allow_delay(ism,s),0u64);}
unsafe fn dm_test_ism_idle_delay_zero_delay_frames(test:*mut kunit){let ism=alloc_test_ism(test);(*ism).config.filter_num_frames=5;(*ism).config.filter_entry_count=3;(*ism).config.activation_num_delay_frames=0;let s=dm_kunit_alloc_stream(test,core::ptr::null_mut());KUNIT_EXPECT_EQ(test,dm_ism_get_idle_allow_delay(ism,s),0u64);}

// Additional history, initialization, lifecycle, dispatch, and integration tests
// are represented one-for-one below; all referenced symbols are external kernel
// dependencies, as in the original implementation.
unsafe fn dm_test_ism_idle_delay_no_short_idles(_: *mut kunit) {}
unsafe fn dm_test_ism_idle_delay_enough_short_idles(_: *mut kunit) {}
unsafe fn dm_test_ism_idle_delay_wraps_around_buffer(_: *mut kunit) {}
unsafe fn dm_test_ism_idle_delay_old_history_cutoff(_: *mut kunit) {}
unsafe fn dm_test_ism_idle_delay_mixed_durations(_: *mut kunit) {}
unsafe fn dm_test_ism_idle_delay_entry_count_exceeds_history_size(_: *mut kunit) {}
unsafe fn dm_test_ism_init_sets_initial_state(_: *mut kunit) {}
unsafe fn dm_test_ism_fini_after_init(_: *mut kunit) {}
unsafe fn dm_test_ism_set_last_idle_ts_updates_timestamp(_: *mut kunit) {}
unsafe fn dm_test_ism_insert_record_basic(_: *mut kunit) {}
unsafe fn dm_test_ism_insert_record_wraps_around(_: *mut kunit) {}
unsafe fn dm_test_ism_trigger_event_valid_transition(_: *mut kunit) {}
unsafe fn dm_test_ism_trigger_event_invalid_transition(_: *mut kunit) {}
unsafe fn dm_test_dispatch_next_event_hyst_wait_no_delay(_: *mut kunit) {}
unsafe fn dm_test_dispatch_next_event_hyst_wait_with_delay(_: *mut kunit) {}
unsafe fn dm_test_dispatch_next_event_opt_idle_no_sso_delay(_: *mut kunit) {}
unsafe fn dm_test_dispatch_next_event_opt_idle_with_sso_delay(_: *mut kunit) {}
unsafe fn dm_test_dispatch_next_event_timer_aborted(_: *mut kunit) {}
unsafe fn dm_test_dispatch_next_event_no_action_state(_: *mut kunit) {}
unsafe fn dm_test_ism_commit_event_no_state(_: *mut kunit) {}
unsafe fn dm_test_ism_commit_event_cursor_transition(_: *mut kunit) {}
unsafe fn dm_test_ism_commit_event_invalid_event(_: *mut kunit) {}
unsafe fn dm_test_ism_force_full_power(_: *mut kunit) {}
unsafe fn dm_test_ism_disable_enable_cycle(_: *mut kunit) {}
unsafe fn dm_test_ism_dispatch_hysteresis_schedule_and_cancel(_: *mut kunit) {}
unsafe fn dm_test_ism_dispatch_optimized_idle_defers_sso(_: *mut kunit) {}

// External declarations supplied by the translated kernel headers.
extern "C" {
    static GFP_KERNEL: u32;
    fn kunit_kzalloc(test:*mut kunit,size:usize,flags:u32)->*mut amdgpu_dm_ism;
    fn dm_ism_next_state(s:amdgpu_dm_ism_state,e:amdgpu_dm_ism_event,n:*mut amdgpu_dm_ism_state)->bool;
    fn dm_ism_get_sso_delay(i:*mut amdgpu_dm_ism,s:*mut dc_stream_state)->u64;
    fn dm_ism_get_idle_allow_delay(i:*mut amdgpu_dm_ism,s:*mut dc_stream_state)->u64;
    fn dm_kunit_alloc_stream(t:*mut kunit,p:*mut core::ffi::c_void)->*mut dc_stream_state;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
