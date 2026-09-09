/* Direct Rust translation of power.c. Included C headers provide the types and
 * external declarations used below in the surrounding translation unit. */

const MOD_POWER_MAX_CONCURRENT_STREAMS: usize = 32;
const SMOOTH_BRIGHTNESS_ADJUSTMENT_TIME_IN_MS: u32 = 500;
const LOW_REFRESH_RATE_DURATION_US_UPPER_BOUND: u32 = 25000;

static PWR_DEFAULT_MIN_BRIGHTNESS_MILLINITS: u32 = 1000;
static PWR_DEFAULT_SDR_BRIGHTNESS_MILLINITS: u32 = 270000;
static DEFAULT_AC_BACKLIGHT_PERCENT: u32 = 100;
static DEFAULT_DC_BACKLIGHT_PERCENT: u32 = 70;

#[inline]
unsafe fn mod_power_to_core(mod_power: *mut mod_power) -> *mut core_power {
    container_of!(mod_power, core_power, mod_public)
}

pub unsafe fn map_index_from_stream(
    core_power: *mut core_power,
    stream: *const dc_stream_state,
) -> u32 {
    let mut index: u32 = 0;
    while index < (*core_power).num_entities {
        if (*core_power).map[index as usize].stream == stream {
            return index;
        }
        index += 1;
    }
    /* Could not find stream requested, this is not trivial, fix when hit */
    ASSERT!(false);
    /* In good cases this is used before stream creation, where index zero is
     * the dummy stream slot. */
    0
}

pub unsafe fn mod_power_hw_init(mod_power: *mut mod_power) -> bool {
    /* Call backlight initialization */
    mod_power_hw_init_backlight(mod_power)
    /* Future: Add other HW init here */
}

pub unsafe fn mod_power_create(
    dc: *mut dc,
    init_params: *mut mod_power_init_params,
    edp_num: u32,
) -> *mut mod_power {
    let mut core_power: *mut core_power = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut abm_max_config: u32 = 0;
    let mut inst: u32 = 0;
    let mut is_brightness_range_valid = false;

    if dc.is_null() { return core::ptr::null_mut(); }
    core_power = kzalloc(core::mem::size_of::<core_power>(), GFP_KERNEL) as *mut core_power;
    if core_power.is_null() { return core::ptr::null_mut(); }
    (*core_power).edp_num = edp_num;
    (*core_power).map = kzalloc(core::mem::size_of::<power_entity>() * MOD_POWER_MAX_CONCURRENT_STREAMS, GFP_KERNEL) as *mut power_entity;
    if (*core_power).map.is_null() { kfree(core_power as *mut _); return core::ptr::null_mut(); }
    for i in 0..MOD_POWER_MAX_CONCURRENT_STREAMS as i32 { (*core_power).map.add(i as usize).as_mut().unwrap().stream = core::ptr::null_mut(); }
    for i in 0..MOD_POWER_MAX_CONCURRENT_STREAMS as i32 {
        (*core_power).map.add(i as usize).as_mut().unwrap().psr_context = kzalloc(core::mem::size_of::<mod_power_psr_context>(), GFP_KERNEL) as *mut mod_power_psr_context;
        if (*core_power).map.add(i as usize).as_ref().unwrap().psr_context.is_null() { break; }
    }
    (*core_power).psr_smu_optimizations_support = (*init_params).allow_psr_smu_optimizations;
    (*core_power).multi_disp_optimizations_support = (*init_params).allow_psr_multi_disp_optimizations;
    for inst in 0..edp_num {
        let p = &mut (*core_power).bl_prop[inst as usize];
        let q = &*init_params.add(inst as usize);
        p.min_abm_backlight = q.min_abm_backlight; p.disable_fractional_pwm = q.disable_fractional_pwm;
        p.use_linear_backlight_curve = q.use_linear_backlight_curve; p.use_nits_based_brightness = q.use_nits_based_brightness;
        p.backlight_ramping_override = q.backlight_ramping_override; p.backlight_ramping_reduction = q.backlight_ramping_reduction;
        p.backlight_ramping_start = q.backlight_ramping_start; p.use_custom_backlight_caps = q.use_custom_backlight_caps;
        p.custom_backlight_caps_config_no = q.custom_backlight_caps_config_no;
        p.num_backlight_levels = if q.num_backlight_levels < 101 { 101 } else { q.num_backlight_levels };
        p.backlight_lut = kzalloc(core::mem::size_of::<u32>() * p.num_backlight_levels as usize, GFP_KERNEL) as *mut u32;
        if p.backlight_lut.is_null() { break; }
    }
    (*core_power).varibright_prop.varibright_active = false;
    (*core_power).varibright_prop.varibright_user_enable = (*init_params).def_varibright_enable;
    (*core_power).varibright_prop.varibright_level = if (*init_params).varibright_level <= abm_defines_max_level { (*init_params).varibright_level } else { 3 };
    (*core_power).varibright_prop.def_varibright_level = if (*init_params).def_varibright_level <= abm_defines_max_level { (*init_params).def_varibright_level } else { 3 };
    abm_max_config = if !(*(*dc).res_pool).dmcu.is_null() && (*(*(*dc).res_pool).dmcu).dmcu_version.abm_version < 0x23 { 4 } else { 3 };
    (*core_power).varibright_prop.varibright_config_setting = if (*init_params).abm_config_setting < abm_max_config { (*init_params).abm_config_setting } else { 0 };
    for inst in 0..edp_num {
        let p = &mut (*core_power).bl_prop[inst as usize]; let q = &*init_params.add(inst as usize);
        *p.backlight_lut = q.min_backlight_pwm; *p.backlight_lut.add((p.num_backlight_levels - 1) as usize) = q.max_backlight_pwm;
        p.min_backlight_pwm = q.min_backlight_pwm; p.max_backlight_pwm = q.max_backlight_pwm;
        p.ac_backlight_percent = DEFAULT_AC_BACKLIGHT_PERCENT; p.dc_backlight_percent = DEFAULT_DC_BACKLIGHT_PERCENT; p.backlight_caps_valid = false;
        p.min_brightness_millinits = if p.use_nits_based_brightness { q.panel_min_millinits } else { PWR_DEFAULT_MIN_BRIGHTNESS_MILLINITS };
        p.max_brightness_millinits = if p.use_nits_based_brightness { q.panel_max_millinits } else { PWR_DEFAULT_SDR_BRIGHTNESS_MILLINITS };
        p.backlight_range = p.max_backlight_pwm - p.min_backlight_pwm; p.nits_range = p.max_brightness_millinits - p.min_brightness_millinits;
        (*core_power).bl_state[inst as usize].smooth_brightness_enabled = true;
        if p.nits_range != 0 && p.backlight_range != 0 { is_brightness_range_valid = true; }
    }
    if !is_brightness_range_valid { goto!(fail_bad_brightness_range); }
    (*core_power).num_entities = 0; (*core_power).dc = dc;
    for inst in 0..edp_num { initialize_backlight_caps(core_power, inst); let s = &mut (*core_power).bl_state[inst as usize]; s.backlight_millipercent = (*core_power).bl_prop[inst as usize].dc_backlight_percent * 1000; s.backlight_pwm = backlight_millipercent_to_pwm(core_power, s.backlight_millipercent, inst); s.backlight_millinit = backlight_millipercent_to_millinit(core_power, s.backlight_millipercent, inst); }
    &mut (*core_power).mod_public
}

pub unsafe fn mod_power_destroy(mod_power: *mut mod_power) { if !mod_power.is_null() { let c = mod_power_to_core(mod_power); for i in 0..MOD_POWER_MAX_CONCURRENT_STREAMS { kfree((*c).map[i].psr_context as *mut _); } for i in 0..(*c).num_entities as usize { if !(*c).map[i].stream.is_null() { dc_stream_release((*c).map[i].stream); } } kfree((*c).map as *mut _); for i in 0..MAX_NUM_EDP { kfree((*c).bl_prop[i].backlight_lut as *mut _); } kfree(c as *mut _); } }

pub unsafe fn mod_power_add_stream(mod_power: *mut mod_power, stream: *mut dc_stream_state, caps: *mut psr_caps) -> bool { if mod_power.is_null() { return false; } let c = mod_power_to_core(mod_power); if (*c).num_entities < MOD_POWER_MAX_CONCURRENT_STREAMS as u32 { dc_stream_retain(stream); let e=&mut (*c).map[(*c).num_entities as usize]; e.stream=stream; e.caps=caps; e.psr_enabled=0; e.psr_events=psr_event_vsync; e.psr_power_opt=0; (*c).num_entities+=1; return true; } false }

pub unsafe fn mod_power_remove_stream(mod_power: *mut mod_power, stream: *const dc_stream_state) -> bool { if mod_power.is_null() { return false; } let c=mod_power_to_core(mod_power); if (*c).num_entities==0 { BREAK_TO_DEBUGGER!(); return false; } let index=map_index_from_stream(c,stream); if index>=(*c).num_entities { BREAK_TO_DEBUGGER!(); return false; } dc_stream_release((*c).map[index as usize].stream); for i in index as usize..((*c).num_entities-1) as usize { (*c).map[i]=(*c).map[i+1]; } (*c).num_entities-=1; true }

pub unsafe fn mod_power_replace_stream(mod_power: *mut mod_power, current_stream: *const dc_stream_state, new_stream: *mut dc_stream_state, new_caps: *mut psr_caps) -> bool { if mod_power.is_null() { return false; } let c=mod_power_to_core(mod_power); if (*c).num_entities==0 { BREAK_TO_DEBUGGER!(); return false; } let i=map_index_from_stream(c,current_stream); if i>=(*c).num_entities { BREAK_TO_DEBUGGER!(); return false; } dc_stream_release((*c).map[i as usize].stream); dc_stream_retain(new_stream); (*c).map[i as usize].stream=new_stream; (*c).map[i as usize].caps=new_caps; memset((*c).map[i as usize].psr_context as *mut _,0,core::mem::size_of::<mod_power_psr_context>()); true }

pub unsafe fn mod_power_notify_mode_change(mod_power:*mut mod_power, stream:*const dc_stream_state, is_hdr:bool)->bool { if mod_power.is_null()||stream.is_null(){return false;} let c=mod_power_to_core(mod_power); if (*c).num_entities==0{return false;} let i=map_index_from_stream(c,stream); if i>=(*c).num_entities{return false;} let dc=(*c).dc; let link=dc_stream_get_link(stream); if !link.is_null(){let mut panel=0; if dc_get_edp_link_panel_inst(dc,link,&mut panel){let aux=(*(*link).dc).link_srv.get_ddc_aux_inst(link); mod_power_update_backlight_on_mode_change(c,link,panel,aux,is_hdr); mod_power_psr_notify_mode_change(mod_power,stream,link,i); mod_power_replay_notify_mode_change(mod_power,dc,link,stream,i);}} true }

pub unsafe fn mod_power_only_edp(context:*const dc_state, stream:*const dc_stream_state)->bool { !context.is_null() && (*context).stream_count==1 && dc_is_embedded_signal((*stream).signal) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
