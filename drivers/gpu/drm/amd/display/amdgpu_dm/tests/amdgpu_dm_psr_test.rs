// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_psr.c
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// C dependencies are supplied by the surrounding kernel translation unit.

unsafe fn alloc_test_psr_stream(test: *mut kunit) -> *mut dc_stream_state {
    let link = dm_kunit_alloc_link(test);
    (*link).psr_settings.psr_feature_enabled = true;
    dm_kunit_alloc_stream(test, link)
}

unsafe fn create_test_power_module(test: *mut kunit, stream: *mut dc_stream_state,
                                   caps: *mut psr_caps) -> *mut core_power {
    let core_power = kunit_kzalloc(test, core::mem::size_of::<core_power>(), GFP_KERNEL)
        as *mut core_power;
    KUNIT_ASSERT_NOT_NULL(test, core_power);
    (*core_power).map = kunit_kzalloc(test, core::mem::size_of_val(&(*core_power).map), GFP_KERNEL)
        as *mut _;
    KUNIT_ASSERT_NOT_NULL(test, (*core_power).map);
    (*core_power).map[0].stream = stream;
    (*core_power).map[0].caps = caps;
    (*core_power).map[0].psr_events = psr_event_vsync;
    (*core_power).num_entities = 1;
    core_power
}

unsafe fn alloc_test_psrsu_link(test: *mut kunit) -> *mut dc_link {
    let link = dm_kunit_alloc_link_with_ctx(test);
    let ctx = (*link).ctx;
    let dc = (*ctx).dc;
    (*dc).caps.dmcub_support = true;
    (*ctx).dce_version = DCN_VERSION_3_1;
    (*link).dpcd_caps.edp_rev = DP_EDP_14;
    (*link).dpcd_caps.psr_info.psr_version = DP_PSR2_WITH_Y_COORD_ET_SUPPORTED;
    (*link).dpcd_caps.alpm_caps.bits.AUX_WAKE_ALPM_CAP = 1;
    (*link).dpcd_caps.psr_info.psr_dpcd_caps.bits.Y_COORDINATE_REQUIRED = 1;
    link
}

unsafe fn alloc_test_psr_caps_link(test: *mut kunit) -> *mut dc_link {
    let link = alloc_test_psrsu_link(test);
    (*(*link).ctx).dc.caps.dmub_caps.psr = true;
    (*link).connector_signal = SIGNAL_TYPE_EDP;
    (*link).r#type = dc_connection_single;
    link
}

unsafe fn alloc_test_aconnector(test: *mut kunit) -> *mut amdgpu_dm_connector {
    let aconnector = kunit_kzalloc(test, core::mem::size_of::<amdgpu_dm_connector>(), GFP_KERNEL)
        as *mut amdgpu_dm_connector;
    KUNIT_ASSERT_NOT_NULL(test, aconnector);
    aconnector
}

// Tests for link_supports_psrsu().
unsafe fn dm_test_link_supports_psrsu_no_dmcub(test: *mut kunit) { let link=alloc_test_psrsu_link(test); (*(*link).ctx).dc.caps.dmcub_support=false; KUNIT_EXPECT_FALSE(test, link_supports_psrsu(link)); }
unsafe fn dm_test_link_supports_psrsu_old_dcn(test: *mut kunit) { let link=alloc_test_psrsu_link(test); (*(*link).ctx).dce_version=DCN_VERSION_3_0; KUNIT_EXPECT_FALSE(test, link_supports_psrsu(link)); }
unsafe fn dm_test_link_supports_psrsu_panel_unsupported(test: *mut kunit) { let link=alloc_test_psrsu_link(test); (*link).dpcd_caps.psr_info.psr_version=0; KUNIT_EXPECT_FALSE(test, link_supports_psrsu(link)); }
unsafe fn dm_test_link_supports_psrsu_missing_alpm(test: *mut kunit) { let link=alloc_test_psrsu_link(test); (*link).dpcd_caps.alpm_caps.bits.AUX_WAKE_ALPM_CAP=0; KUNIT_EXPECT_FALSE(test, link_supports_psrsu(link)); }
unsafe fn dm_test_link_supports_psrsu_missing_y_coordinate(test: *mut kunit) { let link=alloc_test_psrsu_link(test); (*link).dpcd_caps.psr_info.psr_dpcd_caps.bits.Y_COORDINATE_REQUIRED=0; KUNIT_EXPECT_FALSE(test, link_supports_psrsu(link)); }
unsafe fn dm_test_link_supports_psrsu_missing_granularity(test: *mut kunit) { let link=alloc_test_psrsu_link(test); (*link).dpcd_caps.psr_info.psr_dpcd_caps.bits.SU_GRANULARITY_REQUIRED=1; (*link).dpcd_caps.psr_info.psr2_su_y_granularity_cap=0; KUNIT_EXPECT_FALSE(test, link_supports_psrsu(link)); }
unsafe fn dm_test_link_supports_psrsu_debug_mask_disabled(test: *mut kunit) { let link=alloc_test_psrsu_link(test); let old=amdgpu_dm_psr_get_dc_debug_mask(); amdgpu_dm_psr_set_dc_debug_mask(old|DC_DISABLE_PSR_SU); KUNIT_EXPECT_FALSE(test, link_supports_psrsu(link)); amdgpu_dm_psr_set_dc_debug_mask(old); }
unsafe fn dm_test_link_supports_psrsu_temporarily_disabled(test: *mut kunit) { let link=alloc_test_psrsu_link(test); let old=amdgpu_dm_psr_get_dc_debug_mask(); amdgpu_dm_psr_set_dc_debug_mask(old&!DC_DISABLE_PSR_SU); KUNIT_EXPECT_FALSE(test, link_supports_psrsu(link)); amdgpu_dm_psr_set_dc_debug_mask(old); }

unsafe fn dm_test_set_psr_caps_null_link(test: *mut kunit) { let c=alloc_test_aconnector(test); KUNIT_EXPECT_FALSE(test, amdgpu_dm_set_psr_caps(core::ptr::null_mut(),c)); }
unsafe fn dm_test_set_psr_caps_null_connector(test: *mut kunit) { let l=alloc_test_psr_caps_link(test); KUNIT_EXPECT_FALSE(test, amdgpu_dm_set_psr_caps(l,core::ptr::null_mut())); }
unsafe fn dm_test_set_psr_caps_no_dmub_psr(test: *mut kunit) { let l=alloc_test_psr_caps_link(test); let c=alloc_test_aconnector(test); (*l).psr_settings.psr_version=DC_PSR_VERSION_1; (*(*(*l).ctx).dc).caps.dmub_caps.psr=false; KUNIT_EXPECT_FALSE(test,amdgpu_dm_set_psr_caps(l,c)); KUNIT_EXPECT_EQ(test,(*l).psr_settings.psr_version,DC_PSR_VERSION_UNSUPPORTED); }
unsafe fn dm_test_set_psr_caps_non_edp(test:*mut kunit){let l=alloc_test_psr_caps_link(test);let c=alloc_test_aconnector(test);(*l).connector_signal=SIGNAL_TYPE_DISPLAY_PORT;KUNIT_EXPECT_FALSE(test,amdgpu_dm_set_psr_caps(l,c));}
unsafe fn dm_test_set_psr_caps_disconnected(test:*mut kunit){let l=alloc_test_psr_caps_link(test);let c=alloc_test_aconnector(test);(*l).r#type=dc_connection_none;KUNIT_EXPECT_FALSE(test,amdgpu_dm_set_psr_caps(l,c));}
unsafe fn dm_test_set_psr_caps_no_dpcd_psr(test:*mut kunit){let l=alloc_test_psr_caps_link(test);let c=alloc_test_aconnector(test);(*l).dpcd_caps.psr_info.psr_version=0;KUNIT_EXPECT_FALSE(test,amdgpu_dm_set_psr_caps(l,c));}
unsafe fn dm_test_set_psr_caps_edp1_disabled(test:*mut kunit){let l=alloc_test_psr_caps_link(test);let e=dm_kunit_alloc_link(test);let c=alloc_test_aconnector(test);let dc=(*(*l).ctx).dc;(*e).connector_signal=SIGNAL_TYPE_EDP;(*dc).links[0]=e;(*dc).links[1]=l;(*dc).link_count=2;KUNIT_EXPECT_FALSE(test,amdgpu_dm_set_psr_caps(l,c));}
unsafe fn dm_test_set_psr_caps_success_psr1(test:*mut kunit){let l=alloc_test_psr_caps_link(test);let c=alloc_test_aconnector(test);KUNIT_EXPECT_TRUE(test,amdgpu_dm_set_psr_caps(l,c));KUNIT_EXPECT_EQ(test,(*l).psr_settings.psr_version,DC_PSR_VERSION_1);KUNIT_EXPECT_EQ(test,(*c).psr_caps.psr_version as i32,1);KUNIT_EXPECT_EQ(test,(*c).psr_caps.support_ver as i32,DP_PSR2_WITH_Y_COORD_ET_SUPPORTED);}

unsafe fn dm_test_psr_fill_caps_version_1(test:*mut kunit){let l=dm_kunit_alloc_link(test);let mut c:psr_caps=core::mem::zeroed();(*l).psr_settings.psr_version=DC_PSR_VERSION_1;amdgpu_dm_psr_fill_caps(l,&mut c);KUNIT_EXPECT_EQ(test,c.psr_version as i32,1);}
unsafe fn dm_test_psr_fill_caps_version_su1(test:*mut kunit){let l=dm_kunit_alloc_link(test);let mut c:psr_caps=core::mem::zeroed();(*l).psr_settings.psr_version=DC_PSR_VERSION_SU_1;amdgpu_dm_psr_fill_caps(l,&mut c);KUNIT_EXPECT_EQ(test,c.psr_version as i32,2);}
unsafe fn dm_test_psr_fill_caps_version_unsupported(test:*mut kunit){let l=dm_kunit_alloc_link(test);let mut c:psr_caps=core::mem::zeroed();(*l).psr_settings.psr_version=DC_PSR_VERSION_UNSUPPORTED;amdgpu_dm_psr_fill_caps(l,&mut c);KUNIT_EXPECT_EQ(test,c.psr_version as i32,0);}
unsafe fn dm_test_psr_fill_caps_setup_time_zero(test:*mut kunit){let l=dm_kunit_alloc_link(test);let mut c:psr_caps=core::mem::zeroed();(*l).dpcd_caps.psr_info.psr_dpcd_caps.bits.PSR_SETUP_TIME=0;amdgpu_dm_psr_fill_caps(l,&mut c);KUNIT_EXPECT_EQ(test,c.psr_rfb_setup_time,330u32);}
unsafe fn dm_test_psr_fill_caps_setup_time_mid(test:*mut kunit){let l=dm_kunit_alloc_link(test);let mut c:psr_caps=core::mem::zeroed();(*l).dpcd_caps.psr_info.psr_dpcd_caps.bits.PSR_SETUP_TIME=3;amdgpu_dm_psr_fill_caps(l,&mut c);KUNIT_EXPECT_EQ(test,c.psr_rfb_setup_time,165u32);}
unsafe fn dm_test_psr_fill_caps_setup_time_max(test:*mut kunit){let l=dm_kunit_alloc_link(test);let mut c:psr_caps=core::mem::zeroed();(*l).dpcd_caps.psr_info.psr_dpcd_caps.bits.PSR_SETUP_TIME=6;amdgpu_dm_psr_fill_caps(l,&mut c);KUNIT_EXPECT_EQ(test,c.psr_rfb_setup_time,0u32);}
unsafe fn dm_test_psr_fill_caps_link_training_required(test:*mut kunit){let l=dm_kunit_alloc_link(test);let mut c:psr_caps=core::mem::zeroed();(*l).dpcd_caps.psr_info.psr_dpcd_caps.bits.LINK_TRAINING_ON_EXIT_NOT_REQUIRED=0;amdgpu_dm_psr_fill_caps(l,&mut c);KUNIT_EXPECT_TRUE(test,c.psr_exit_link_training_required);}
unsafe fn dm_test_psr_fill_caps_link_training_not_required(test:*mut kunit){let l=dm_kunit_alloc_link(test);let mut c:psr_caps=core::mem::zeroed();(*l).dpcd_caps.psr_info.psr_dpcd_caps.bits.LINK_TRAINING_ON_EXIT_NOT_REQUIRED=1;amdgpu_dm_psr_fill_caps(l,&mut c);KUNIT_EXPECT_FALSE(test,c.psr_exit_link_training_required);}
unsafe fn dm_test_psr_fill_caps_dpcd_fields(test:*mut kunit){let l=dm_kunit_alloc_link(test);let mut c:psr_caps=core::mem::zeroed();(*l).dpcd_caps.edp_rev=0x14;(*l).dpcd_caps.psr_info.psr_version=2;(*l).dpcd_caps.psr_info.psr_dpcd_caps.bits.SU_GRANULARITY_REQUIRED=1;(*l).dpcd_caps.psr_info.psr_dpcd_caps.bits.Y_COORDINATE_REQUIRED=1;(*l).dpcd_caps.psr_info.psr2_su_y_granularity_cap=4;(*l).dpcd_caps.alpm_caps.bits.AUX_WAKE_ALPM_CAP=1;(*l).dpcd_caps.alpm_caps.bits.PM_STATE_2A_SUPPORT=1;amdgpu_dm_psr_fill_caps(l,&mut c);KUNIT_EXPECT_EQ(test,c.edp_revision as i32,0x14);KUNIT_EXPECT_EQ(test,c.support_ver as i32,2);KUNIT_EXPECT_TRUE(test,c.su_granularity_required);KUNIT_EXPECT_TRUE(test,c.y_coordinate_required);KUNIT_EXPECT_EQ(test,c.su_y_granularity as i32,4);KUNIT_EXPECT_TRUE(test,c.alpm_cap);KUNIT_EXPECT_TRUE(test,c.standby_support);}
unsafe fn dm_test_psr_fill_caps_dpcd_fields_unset(test:*mut kunit){let l=dm_kunit_alloc_link(test);let mut c:psr_caps=core::mem::zeroed();amdgpu_dm_psr_fill_caps(l,&mut c);KUNIT_EXPECT_EQ(test,c.edp_revision as i32,0);KUNIT_EXPECT_EQ(test,c.support_ver as i32,0);KUNIT_EXPECT_FALSE(test,c.su_granularity_required);KUNIT_EXPECT_FALSE(test,c.y_coordinate_required);KUNIT_EXPECT_EQ(test,c.su_y_granularity as i32,0);KUNIT_EXPECT_FALSE(test,c.alpm_cap);KUNIT_EXPECT_FALSE(test,c.standby_support);}
unsafe fn dm_test_psr_fill_caps_rate_control_always_zero(test:*mut kunit){let l=dm_kunit_alloc_link(test);let mut c:psr_caps=core::mem::zeroed();amdgpu_dm_psr_fill_caps(l,&mut c);KUNIT_EXPECT_EQ(test,c.rate_control_caps as i32,0);}
unsafe fn dm_test_psr_fill_caps_power_opts_z10_always_set(test:*mut kunit){let l=dm_kunit_alloc_link(test);let mut c:psr_caps=core::mem::zeroed();amdgpu_dm_psr_fill_caps(l,&mut c);KUNIT_EXPECT_TRUE(test,(c.psr_power_opt_flag&psr_power_opt_z10_static_screen)!=0);}
unsafe fn dm_test_psr_fill_caps_power_opts_smu_opt_set(test:*mut kunit){let l=dm_kunit_alloc_link(test);let mut c:psr_caps=core::mem::zeroed();let old=amdgpu_dm_psr_get_dc_feature_mask();amdgpu_dm_psr_set_dc_feature_mask(old|DC_PSR_ALLOW_SMU_OPT);amdgpu_dm_psr_fill_caps(l,&mut c);amdgpu_dm_psr_set_dc_feature_mask(old);KUNIT_EXPECT_TRUE(test,(c.psr_power_opt_flag&psr_power_opt_smu_opt_static_screen)!=0);}

unsafe fn dm_test_psr_set_event_null_stream(test:*mut kunit){KUNIT_EXPECT_FALSE(test,amdgpu_dm_psr_set_event(core::ptr::null_mut(),core::ptr::null_mut(),true,psr_event_vsync,false));}
unsafe fn dm_test_psr_set_event_null_link(test:*mut kunit){let s=kunit_kzalloc(test,core::mem::size_of::<dc_stream_state>(),GFP_KERNEL) as *mut dc_stream_state;KUNIT_ASSERT_NOT_NULL(test,s);KUNIT_EXPECT_FALSE(test,amdgpu_dm_psr_set_event(core::ptr::null_mut(),s,true,psr_event_vsync,false));}
unsafe fn dm_test_psr_set_event_psr_not_enabled(test:*mut kunit){let s=kunit_kzalloc(test,core::mem::size_of::<dc_stream_state>(),GFP_KERNEL) as *mut dc_stream_state;let l=kunit_kzalloc(test,core::mem::size_of::<dc_link>(),GFP_KERNEL) as *mut dc_link;KUNIT_ASSERT_NOT_NULL(test,s);KUNIT_ASSERT_NOT_NULL(test,l);(*s).link=l;KUNIT_EXPECT_FALSE(test,amdgpu_dm_psr_set_event(core::ptr::null_mut(),s,true,psr_event_vsync,false));}
unsafe fn dm_test_psr_set_event_get_event_fails(test:*mut kunit){let d=dm_kunit_alloc_dm(test);let s=alloc_test_psr_stream(test);(*d).power_module=core::ptr::null_mut();KUNIT_EXPECT_FALSE(test,amdgpu_dm_psr_set_event(d,s,true,psr_event_vsync,false));}
unsafe fn dm_test_psr_set_event_already_set(test:*mut kunit){let d=dm_kunit_alloc_dm(test);let s=alloc_test_psr_stream(test);let mut c:psr_caps=core::mem::zeroed();c.psr_version=1;let p=create_test_power_module(test,s,&mut c);(*d).power_module=&mut (*p).mod_public;KUNIT_EXPECT_TRUE(test,amdgpu_dm_psr_set_event(d,s,true,psr_event_vsync,false));KUNIT_EXPECT_EQ(test,(*p).map[0].psr_events,psr_event_vsync as u32);}
unsafe fn dm_test_psr_set_event_updates_event(test:*mut kunit){let d=dm_kunit_alloc_dm(test);let s=alloc_test_psr_stream(test);let mut c:psr_caps=core::mem::zeroed();c.psr_version=1;let p=create_test_power_module(test,s,&mut c);(*d).power_module=&mut (*p).mod_public;KUNIT_EXPECT_TRUE(test,amdgpu_dm_psr_set_event(d,s,true,psr_event_full_screen,false));KUNIT_EXPECT_EQ(test,(*p).map[0].psr_events,(psr_event_vsync|psr_event_full_screen) as u32);}

unsafe fn dm_test_psr_is_active_allowed_no_streams(test:*mut kunit){let d=dm_kunit_alloc_dm(test);KUNIT_EXPECT_FALSE(test,amdgpu_dm_psr_is_active_allowed(d));}
unsafe fn dm_test_psr_is_active_allowed_null_link(test:*mut kunit){let d=dm_kunit_alloc_dm(test);let state=(*(*d).dc).current_state;dm_kunit_add_stream_to_state(test,state,0,core::ptr::null_mut());KUNIT_EXPECT_FALSE(test,amdgpu_dm_psr_is_active_allowed(d));}
unsafe fn dm_test_psr_is_active_allowed_requires_enabled_and_allowed(test:*mut kunit){let d=dm_kunit_alloc_dm(test);let state=(*(*d).dc).current_state;let l=dm_kunit_alloc_link(test);dm_kunit_add_stream_to_state(test,state,0,l);(*l).psr_settings.psr_allow_active=true;KUNIT_EXPECT_FALSE(test,amdgpu_dm_psr_is_active_allowed(d));(*l).psr_settings.psr_allow_active=false;(*l).psr_settings.psr_feature_enabled=true;KUNIT_EXPECT_FALSE(test,amdgpu_dm_psr_is_active_allowed(d));}
unsafe fn dm_test_psr_is_active_allowed_any_stream(test:*mut kunit){let d=dm_kunit_alloc_dm(test);let state=(*(*d).dc).current_state;let x=dm_kunit_alloc_link(test);let l=dm_kunit_alloc_link(test);(*x).psr_settings.psr_allow_active=true;(*l).psr_settings.psr_feature_enabled=true;(*l).psr_settings.psr_allow_active=true;dm_kunit_add_stream_to_state(test,state,0,x);dm_kunit_add_stream_to_state(test,state,1,l);KUNIT_EXPECT_TRUE(test,amdgpu_dm_psr_is_active_allowed(d));}

// The KUnit case array and suite registration retain the source registration order.
static mut dm_psr_test_cases: [kunit_case; 37] = [
    KUNIT_CASE!(dm_test_link_supports_psrsu_no_dmcub), KUNIT_CASE!(dm_test_link_supports_psrsu_old_dcn), KUNIT_CASE!(dm_test_link_supports_psrsu_panel_unsupported), KUNIT_CASE!(dm_test_link_supports_psrsu_missing_alpm), KUNIT_CASE!(dm_test_link_supports_psrsu_missing_y_coordinate), KUNIT_CASE!(dm_test_link_supports_psrsu_missing_granularity), KUNIT_CASE!(dm_test_link_supports_psrsu_debug_mask_disabled), KUNIT_CASE!(dm_test_link_supports_psrsu_temporarily_disabled),
    KUNIT_CASE!(dm_test_set_psr_caps_null_link), KUNIT_CASE!(dm_test_set_psr_caps_null_connector), KUNIT_CASE!(dm_test_set_psr_caps_no_dmub_psr), KUNIT_CASE!(dm_test_set_psr_caps_non_edp), KUNIT_CASE!(dm_test_set_psr_caps_disconnected), KUNIT_CASE!(dm_test_set_psr_caps_no_dpcd_psr), KUNIT_CASE!(dm_test_set_psr_caps_edp1_disabled), KUNIT_CASE!(dm_test_set_psr_caps_success_psr1),
    KUNIT_CASE!(dm_test_psr_fill_caps_version_1), KUNIT_CASE!(dm_test_psr_fill_caps_version_su1), KUNIT_CASE!(dm_test_psr_fill_caps_version_unsupported), KUNIT_CASE!(dm_test_psr_fill_caps_setup_time_zero), KUNIT_CASE!(dm_test_psr_fill_caps_setup_time_mid), KUNIT_CASE!(dm_test_psr_fill_caps_setup_time_max), KUNIT_CASE!(dm_test_psr_fill_caps_link_training_required), KUNIT_CASE!(dm_test_psr_fill_caps_link_training_not_required), KUNIT_CASE!(dm_test_psr_fill_caps_dpcd_fields), KUNIT_CASE!(dm_test_psr_fill_caps_dpcd_fields_unset), KUNIT_CASE!(dm_test_psr_fill_caps_rate_control_always_zero), KUNIT_CASE!(dm_test_psr_fill_caps_power_opts_z10_always_set), KUNIT_CASE!(dm_test_psr_fill_caps_power_opts_smu_opt_set),
    KUNIT_CASE!(dm_test_psr_set_event_null_stream), KUNIT_CASE!(dm_test_psr_set_event_null_link), KUNIT_CASE!(dm_test_psr_set_event_psr_not_enabled), KUNIT_CASE!(dm_test_psr_set_event_get_event_fails), KUNIT_CASE!(dm_test_psr_set_event_already_set), KUNIT_CASE!(dm_test_psr_set_event_updates_event), KUNIT_CASE!(dm_test_psr_is_active_allowed_no_streams), KUNIT_CASE!(dm_test_psr_is_active_allowed_null_link), KUNIT_CASE!(dm_test_psr_is_active_allowed_requires_enabled_and_allowed), KUNIT_CASE!(dm_test_psr_is_active_allowed_any_stream), KUNIT_CASE_END!(),
];

static mut dm_psr_test_suite: kunit_suite = kunit_suite { name: "amdgpu_dm_psr", test_cases: dm_psr_test_cases.as_mut_ptr() };
// kunit_test_suite(dm_psr_test_suite);
// MODULE_LICENSE("Dual MIT/GPL");
// MODULE_DESCRIPTION("KUnit tests for amdgpu_dm_psr");
// MODULE_AUTHOR("AMD");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
