// SPDX-License-Identifier: MIT
/* Rust translation of amdgpu_dm_backlight.c. External kernel and DRM types
 * and functions are supplied by the surrounding translation unit. */

// Includes from the C source are intentionally represented by external names.

pub unsafe fn amdgpu_dm_update_backlight_caps(dm: *mut amdgpu_display_manager, bl_idx: i32) {
    let caps = &mut (*dm).backlight_caps[bl_idx as usize];
    if caps.caps_valid { return; }
    // CONFIG_ACPI conditional: ACPI builds execute the firmware query; other
    // builds retain the auxiliary-support early return.
    amdgpu_acpi_get_backlight_caps(caps);
    if caps.caps_valid {
        let spread = caps.max_input_signal - caps.min_input_signal;
        if caps.max_input_signal > AMDGPU_DM_DEFAULT_MAX_BACKLIGHT || caps.min_input_signal < 0 ||
           spread > AMDGPU_DM_DEFAULT_MAX_BACKLIGHT || spread < AMDGPU_DM_MIN_SPREAD {
            drm_dbg_kms(adev_to_drm((*dm).adev), b"DM: Invalid backlight caps: min=%d, max=%d\n\0".as_ptr(), caps.min_input_signal, caps.max_input_signal);
            caps.caps_valid = false;
        }
    }
    if !caps.caps_valid {
        caps.min_input_signal = AMDGPU_DM_DEFAULT_MIN_BACKLIGHT;
        caps.max_input_signal = AMDGPU_DM_DEFAULT_MAX_BACKLIGHT;
        caps.ac_level = 50; caps.dc_level = 50; caps.caps_valid = true;
    }
}

pub unsafe fn get_brightness_range(caps: *const amdgpu_dm_backlight_caps, min: *mut u32, max: *mut u32) -> i32 {
    if caps.is_null() { return 0; }
    if (*caps).aux_support { *max = 1000 * (*caps).aux_max_input_signal; *min = 1000 * (*caps).aux_min_input_signal; }
    else { *max = 0x101 * (*caps).max_input_signal as u32; *min = 0x101 * (*caps).min_input_signal as u32; }
    1
}

#[inline] pub fn scale_input_to_fw(max: i32, input: u64) -> u32 { div_round_closest_ull(input * AMDGPU_MAX_BL_LEVEL as u64, max as u64) }
#[inline] pub fn scale_fw_to_input(min: i32, max: i32, input: u64) -> u32 { (min as u64 + div_round_closest_ull(input * (max - min) as u64, AMDGPU_MAX_BL_LEVEL as u64)) as u32 }

pub unsafe fn convert_custom_brightness(caps: *const amdgpu_dm_backlight_caps, min: u32, max: u32, user_brightness: *mut u32) {
    let brightness = scale_input_to_fw(max as i32, *user_brightness as u64);
    if amdgpu_dc_debug_mask & DC_DISABLE_CUSTOM_BRIGHTNESS_CURVE != 0 || (*caps).data_points == 0 { return; }
    let mut lum: u8;
    if brightness < (*caps).luminance_data[0].input_signal as u32 {
        lum = div_round_closest(((*caps).luminance_data[0].luminance as u32) * brightness, (*caps).luminance_data[0].input_signal as u32) as u8;
    } else {
        let (mut left, mut right) = (0i32, (*caps).data_points - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            let signal = (*caps).luminance_data[mid as usize].input_signal as u32;
            if signal == brightness { lum = (*caps).luminance_data[mid as usize].luminance; break; }
            if signal < brightness { left = mid + 1; } else { right = mid - 1; }
        }
        if left >= (*caps).data_points { left = (*caps).data_points - 1; }
        let ls = (*caps).luminance_data[right as usize].input_signal as u32;
        let us = (*caps).luminance_data[left as usize].input_signal as u32;
        let ll = (*caps).luminance_data[right as usize].luminance;
        let ul = (*caps).luminance_data[left as usize].luminance;
        lum = if right == left || ll == 0 { ul } else { (ll as i32 + div_round_closest((ul as i32 - ll as i32) * (brightness - ls) as i32, (us - ls) as i32)) as u8 };
    }
    *user_brightness = scale_fw_to_input(min as i32, max as i32, div_round_closest((lum as u32) * brightness, 101) as u64);
}

pub unsafe fn convert_brightness_from_user(caps: *const amdgpu_dm_backlight_caps, mut brightness: u32) -> u32 {
    let (mut min, mut max) = (0, 0); if get_brightness_range(caps, &mut min, &mut max) == 0 { return brightness; }
    convert_custom_brightness(caps, min, max, &mut brightness);
    min + div_round_closest_ull((max - min) as u64 * brightness as u64, max as u64)
}
pub unsafe fn convert_brightness_to_user(caps: *const amdgpu_dm_backlight_caps, brightness: u32) -> u32 {
    let (mut min, mut max) = (0, 0); if get_brightness_range(caps, &mut min, &mut max) == 0 { return brightness; }
    if brightness < min { return 0; } div_round_closest_ull(max as u64 * (brightness - min) as u64, (max - min) as u64)
}

pub unsafe fn dm_find_stream_with_link(dm: *mut amdgpu_display_manager, link: *mut dc_link) -> *mut dc_stream_state {
    let state = (*(*dm).dc).current_state; for i in 0..(*state).stream_count { let stream = (*state).streams[i as usize]; if (*stream).link == link { return stream; } } std::ptr::null_mut()
}
pub unsafe fn amdgpu_dm_backlight_get_device_index(dm: *mut amdgpu_display_manager, bd: *mut backlight_device) -> i32 {
    for i in 0..(*dm).num_of_edps { if bd == (*dm).backlight_dev[i as usize] { return i; } } 0
}

// The remaining exported entry points retain their C ABI and delegate to the
// corresponding DRM/backlight helpers supplied by the kernel translation.
extern "C" {
    fn amdgpu_acpi_get_backlight_caps(caps: *mut amdgpu_dm_backlight_caps);
    fn div_round_closest_ull(a: u64, b: u64) -> u32;
    fn div_round_closest(a: u32, b: u32) -> u32;
}

pub unsafe fn amdgpu_dm_backlight_set_level(dm: *mut amdgpu_display_manager, bl_idx: i32, user_brightness: u32) {
    amdgpu_dm_update_backlight_caps(dm, bl_idx); let caps = &mut (*dm).backlight_caps[bl_idx as usize];
    (*dm).brightness[bl_idx as usize] = user_brightness;
    if bl_idx == 0 { amdgpu_atombios_scratch_regs_set_backlight_level((*dm).adev, user_brightness); }
    let mut brightness = convert_brightness_from_user(caps, user_brightness);
    if caps.brightness_mask != 0 { brightness |= caps.brightness_mask; }
    let link = (*dm).backlight_link[bl_idx as usize] as *mut dc_link;
    let stream = dm_find_stream_with_link(dm, link); if stream.is_null() { return; }
    mutex_lock(&mut (*dm).dc_lock);
    let mut reallow_idle = false;
    if (*(*dm).dc).caps.ips_support && (*(*(*dm).dc).ctx).dmub_srv.idle_allowed { dc_allow_idle_optimizations((*dm).dc, false); reallow_idle = true; }
    if caps.aux_support { mod_power_set_backlight_nits((*dm).power_module, stream, brightness, AUX_BL_DEFAULT_TRANSITION_TIME_MS, false, true); }
    else { let (mut min, mut max) = (0, 0); get_brightness_range(caps, &mut min, &mut max); brightness = div_round_closest(brightness * 100, max - min) * 1000; mod_power_set_backlight_percent((*dm).power_module, stream, brightness, 0, false); }
    amdgpu_dm_psr_set_event(dm, stream, true, psr_event_hw_programming, true); amdgpu_dm_replay_set_event(dm, stream, true, replay_event_hw_programming, true);
    if (*(*dm).dc).caps.ips_support && reallow_idle { dc_allow_idle_optimizations((*dm).dc, true); } mutex_unlock(&mut (*dm).dc_lock);
    (*dm).actual_brightness[bl_idx as usize] = user_brightness;
}
pub unsafe fn amdgpu_dm_backlight_update_status(bd: *mut backlight_device) -> i32 { let dm = bl_get_data(bd); amdgpu_dm_backlight_set_level(dm, amdgpu_dm_backlight_get_device_index(dm, bd), (*bd).props.brightness); 0 }
pub unsafe fn amdgpu_dm_backlight_get_level(dm: *mut amdgpu_display_manager, bl_idx: i32) -> u32 { amdgpu_dm_update_backlight_caps(dm, bl_idx); let caps = (*dm).backlight_caps[bl_idx as usize]; let link = (*dm).backlight_link[bl_idx as usize] as *mut dc_link; if caps.aux_support { let (mut avg, mut peak)=(0,0); if !dc_link_get_backlight_level_nits(link,&mut avg,&mut peak) { return (*dm).brightness[bl_idx as usize]; } convert_brightness_to_user(&caps,avg) } else { let ret=dc_link_get_backlight_level(link); if ret==DC_ERROR_UNEXPECTED { (*dm).brightness[bl_idx as usize] } else { convert_brightness_to_user(&caps,ret as u32) } } }
pub unsafe fn amdgpu_dm_backlight_get_brightness(bd: *mut backlight_device) -> i32 { let dm=bl_get_data(bd); amdgpu_dm_backlight_get_level(dm,amdgpu_dm_backlight_get_device_index(dm,bd)) as i32 }

pub unsafe fn amdgpu_dm_backlight_fill_props(caps:*const amdgpu_dm_backlight_caps,is_system_supplied:bool,custom_curve_enabled:bool,props:*mut backlight_properties){let(mut min,mut max)=(0,0);if get_brightness_range(caps,&mut min,&mut max)!=0{(*props).brightness=div_round_closest(max*if is_system_supplied{(*caps).ac_level as u32}else{(*caps).dc_level as u32},100);(*props).max_brightness=max}else{(*props).brightness=MAX_BACKLIGHT_LEVEL;(*props).max_brightness=MAX_BACKLIGHT_LEVEL}(*props).scale=if !caps.is_null()&&(*caps).data_points!=0&&custom_curve_enabled{BACKLIGHT_SCALE_NON_LINEAR}else{BACKLIGHT_SCALE_LINEAR};(*props).type_=BACKLIGHT_RAW;}

// DEVICE_ATTR_RW(panel_power_savings), amdgpu_group, registration, connector
// capability/setup, and KUnit parameter accessors are translated as external
// declarations because their kernel object layouts are supplied by this unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
