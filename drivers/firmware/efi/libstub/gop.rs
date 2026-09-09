// SPDX-License-Identifier: GPL-2.0
/* -----------------------------------------------------------------------
 *
 *   Copyright 2011 Intel Corporation; author Matt Fleming
 *
 * ----------------------------------------------------------------------- */

#[repr(C)]
enum EfiCmdlineOption { EfiCmdlineNone, EfiCmdlineModeNum, EfiCmdlineRes, EfiCmdlineAuto, EfiCmdlineList }

#[repr(C)]
union CmdlineValue {
    mode: u32,
    res: CmdlineRes,
}

#[repr(C)]
struct CmdlineRes { width: u32, height: u32, format: i32, depth: u8 }

#[repr(C)]
struct Cmdline { option: EfiCmdlineOption, value: CmdlineValue }

static mut CMDLINE: Cmdline = Cmdline { option: EfiCmdlineOption::EfiCmdlineNone, value: CmdlineValue { mode: 0 } };

unsafe fn parse_modenum(mut option: *mut i8, next: *mut *mut i8) -> bool {
    let mut m: u32;
    if !strstarts(option, b"mode=\0".as_ptr() as *const i8) { return false; }
    option = option.add(strlen(b"mode=\0".as_ptr() as *const i8));
    m = simple_strtoull(option, &mut option, 0) as u32;
    if *option != 0 && { option = option.add(1); *option.sub(1) != b',' as i8 } { return false; }
    CMDLINE.option = EfiCmdlineOption::EfiCmdlineModeNum;
    CMDLINE.value.mode = m;
    *next = option;
    true
}

unsafe fn parse_res(mut option: *mut i8, next: *mut *mut i8) -> bool {
    let mut w: u32; let mut h: u32; let mut d: u8 = 0; let mut pf: i32 = -1;
    if !isdigit(*option as u8) { return false; }
    w = simple_strtoull(option, &mut option, 10) as u32;
    if *option != b'x' as i8 { return false; } option = option.add(1);
    if !isdigit(*option as u8) { return false; }
    h = simple_strtoull(option, &mut option, 10) as u32;
    if *option == b'-' as i8 { option = option.add(1);
        if strstarts(option, b"rgb\0".as_ptr() as *const i8) { option = option.add(3); pf = PIXEL_RGB_RESERVED_8BIT_PER_COLOR as i32; }
        else if strstarts(option, b"bgr\0".as_ptr() as *const i8) { option = option.add(3); pf = PIXEL_BGR_RESERVED_8BIT_PER_COLOR as i32; }
        else if isdigit(*option as u8) { d = simple_strtoull(option, &mut option, 10) as u8; }
        else { return false; }
    }
    if *option != 0 { if *option != b',' as i8 { return false; } option = option.add(1); }
    CMDLINE.option = EfiCmdlineOption::EfiCmdlineRes;
    CMDLINE.value.res = CmdlineRes { width: w, height: h, format: pf, depth: d };
    *next = option; true
}

unsafe fn parse_auto(mut option: *mut i8, next: *mut *mut i8) -> bool {
    if !strstarts(option, b"auto\0".as_ptr() as *const i8) { return false; } option = option.add(4);
    if *option != 0 { if *option != b',' as i8 { return false; } option = option.add(1); }
    CMDLINE.option = EfiCmdlineOption::EfiCmdlineAuto; *next = option; true
}

unsafe fn parse_list(mut option: *mut i8, next: *mut *mut i8) -> bool {
    if !strstarts(option, b"list\0".as_ptr() as *const i8) { return false; } option = option.add(4);
    if *option != 0 { if *option != b',' as i8 { return false; } option = option.add(1); }
    CMDLINE.option = EfiCmdlineOption::EfiCmdlineList; *next = option; true
}

pub unsafe fn efi_parse_option_graphics(mut option: *mut i8) {
    while *option != 0 {
        if parse_modenum(option, &mut option) || parse_res(option, &mut option) || parse_auto(option, &mut option) || parse_list(option, &mut option) { continue; }
        while *option != 0 && { let c = *option; option = option.add(1); c != b',' as i8 } {}
    }
}

unsafe fn choose_mode_modenum(gop: *mut efi_graphics_output_protocol_t) -> u32 {
    let mode = efi_table_attr(gop, mode); let cur_mode = efi_table_attr(mode, mode); let requested = CMDLINE.value.mode;
    if requested == cur_mode { return cur_mode; }
    let max_mode = efi_table_attr(mode, max_mode); if requested >= max_mode { efi_err(b"Requested mode is invalid\n\0"); return cur_mode; }
    let mut info = core::ptr::null_mut(); let mut info_size = 0; let status = efi_call_proto(gop, query_mode, requested, &mut info_size, &mut info);
    if status != EFI_SUCCESS { efi_err(b"Couldn't get mode information\n\0"); return cur_mode; }
    let pf = (*info).pixel_format; if pf == PIXEL_BLT_ONLY || pf >= PIXEL_FORMAT_MAX { efi_err(b"Invalid PixelFormat\n\0"); return cur_mode; }
    requested
}

unsafe fn choose_mode(gop: *mut efi_graphics_output_protocol_t, match_fn: unsafe fn(*const efi_graphics_output_mode_info_t, u32, *mut core::ffi::c_void) -> bool, ctx: *mut core::ffi::c_void) -> u32 {
    let mode = efi_table_attr(gop, mode); let max_mode = efi_table_attr(mode, max_mode);
    for m in 0..max_mode { let mut info = core::ptr::null_mut(); let mut info_size = 0; if efi_call_proto(gop, query_mode, m, &mut info_size, &mut info) != EFI_SUCCESS { continue; } if match_fn(info, m, ctx) { return m; } }
    ctx as usize as u32
}

unsafe fn pixel_bpp(pixel_format: i32, pixel_info: efi_pixel_bitmask_t) -> u8 {
    if pixel_format == PIXEL_BIT_MASK as i32 { let mask = pixel_info.red_mask | pixel_info.green_mask | pixel_info.blue_mask | pixel_info.reserved_mask; if mask == 0 { return 0; } (__fls(mask) - __ffs(mask) + 1) as u8 } else { 32 }
}

unsafe fn match_res(info: *const efi_graphics_output_mode_info_t, _mode: u32, _ctx: *mut core::ffi::c_void) -> bool {
    let r = CMDLINE.value.res; let pf = (*info).pixel_format; pf != PIXEL_BLT_ONLY as i32 && pf < PIXEL_FORMAT_MAX as i32 && r.width == (*info).horizontal_resolution && r.height == (*info).vertical_resolution && (r.format < 0 || r.format == pf) && (r.depth == 0 || r.depth == pixel_bpp(pf, (*info).pixel_information))
}

unsafe fn choose_mode_res(gop: *mut efi_graphics_output_protocol_t) -> u32 { let mode = efi_table_attr(gop, mode); let cur = efi_table_attr(mode, mode); if match_res(efi_table_attr(mode, info), cur, core::ptr::null_mut()) { cur } else { choose_mode(gop, match_res, cur as usize as *mut _) } }

#[repr(C)] struct Match { mode: u32, area: u32, depth: u8 }
unsafe fn match_auto(info: *const efi_graphics_output_mode_info_t, mode: u32, ctx: *mut core::ffi::c_void) -> bool { let area = (*info).horizontal_resolution * (*info).vertical_resolution; let depth = pixel_bpp((*info).pixel_format, (*info).pixel_information); if (*info).pixel_format == PIXEL_BLT_ONLY as i32 || (*info).pixel_format >= PIXEL_FORMAT_MAX as i32 { return false; } let m = &mut *(ctx as *mut Match); if area > m.area || area == m.area && depth > m.depth { *m = Match { mode, area, depth }; } false }
unsafe fn choose_mode_auto(gop: *mut efi_graphics_output_protocol_t) -> u32 { let mut m = Match { mode: 0, area: 0, depth: 0 }; choose_mode(gop, match_auto, &mut m as *mut _ as *mut _); m.mode }

unsafe fn match_list(info: *const efi_graphics_output_mode_info_t, mode: u32, ctx: *mut core::ffi::c_void) -> bool { let pf = (*info).pixel_format; let valid = pf != PIXEL_BLT_ONLY as i32 && pf < PIXEL_FORMAT_MAX as i32; let (dstr, depth) = match pf { x if x == PIXEL_RGB_RESERVED_8BIT_PER_COLOR as i32 => (b"rgb\0".as_ptr(), 0), x if x == PIXEL_BGR_RESERVED_8BIT_PER_COLOR as i32 => (b"bgr\0".as_ptr(), 0), x if x == PIXEL_BIT_MASK as i32 => (b"\0".as_ptr(), pixel_bpp(pf, (*info).pixel_information)), x if x == PIXEL_BLT_ONLY as i32 => (b"blt\0".as_ptr(), 0), _ => (b"xxx\0".as_ptr(), 0) }; efi_printk(b"Mode %3u %c%c: Resolution %ux%u-%s%.0hhu\n\0".as_ptr(), mode, if mode == ctx as usize as u32 { b'*' } else { b' ' }, if valid { b' ' } else { b'-' }, (*info).horizontal_resolution, (*info).vertical_resolution, dstr, depth); false }

unsafe fn choose_mode_list(gop: *mut efi_graphics_output_protocol_t) -> u32 { let mode = efi_table_attr(gop, mode); let cur = efi_table_attr(mode, mode); let max = efi_table_attr(mode, max_mode); efi_printk(b"Available graphics modes are 0-%u\n\0".as_ptr(), max - 1); efi_puts(b"  * = current mode\n  - = unusable mode\n\0".as_ptr()); choose_mode(gop, match_list, cur as usize as *mut _); efi_puts(b"\nPress any key to continue (or wait 10 seconds)\n\0".as_ptr()); cur }

unsafe fn set_mode(gop: *mut efi_graphics_output_protocol_t) { let new_mode = match CMDLINE.option { EfiCmdlineOption::EfiCmdlineModeNum => choose_mode_modenum(gop), EfiCmdlineOption::EfiCmdlineRes => choose_mode_res(gop), EfiCmdlineOption::EfiCmdlineAuto => choose_mode_auto(gop), EfiCmdlineOption::EfiCmdlineList => choose_mode_list(gop), _ => return }; let mode = efi_table_attr(gop, mode); let cur = efi_table_attr(mode, mode); if new_mode != cur && efi_call_proto(gop, set_mode, new_mode) != EFI_SUCCESS { efi_err(b"Failed to set requested mode\n\0"); } }

unsafe fn find_bits(mask: u32, pos: *mut u8, size: *mut u8) { if mask == 0 { *pos = 0; *size = 0; } else { *pos = __ffs(mask) as u8; *size = (__fls(mask) - *pos as u32 + 1) as u8; } }

unsafe fn setup_screen_info(si: *mut screen_info, gop: *const efi_graphics_output_protocol_t) { let mode = efi_table_attr(gop, mode); let info = efi_table_attr(mode, info); (*si).orig_video_isVGA = VIDEO_TYPE_EFI; (*si).lfb_width = (*info).horizontal_resolution; (*si).lfb_height = (*info).vertical_resolution; efi_set_u64_split(efi_table_attr(mode, frame_buffer_base), &mut (*si).lfb_base, &mut (*si).ext_lfb_base); if (*si).ext_lfb_base != 0 { (*si).capabilities |= VIDEO_CAPABILITY_64BIT_BASE; } (*si).pages = 1; if (*info).pixel_format == PIXEL_BIT_MASK { find_bits((*info).pixel_information.red_mask, &mut (*si).red_pos, &mut (*si).red_size); find_bits((*info).pixel_information.green_mask, &mut (*si).green_pos, &mut (*si).green_size); find_bits((*info).pixel_information.blue_mask, &mut (*si).blue_pos, &mut (*si).blue_size); find_bits((*info).pixel_information.reserved_mask, &mut (*si).rsvd_pos, &mut (*si).rsvd_size); (*si).lfb_depth = (*si).red_size + (*si).green_size + (*si).blue_size + (*si).rsvd_size; (*si).lfb_linelength = (*info).pixels_per_scan_line * (*si).lfb_depth as u32 / 8; } else { (*si).red_pos = if (*info).pixel_format == PIXEL_RGB_RESERVED_8BIT_PER_COLOR { 0 } else { 16 }; (*si).blue_pos = if (*info).pixel_format == PIXEL_RGB_RESERVED_8BIT_PER_COLOR { 16 } else { 0 }; (*si).green_pos = 8; (*si).rsvd_pos = 24; (*si).red_size = 8; (*si).green_size = 8; (*si).blue_size = 8; (*si).rsvd_size = 8; (*si).lfb_depth = 32; (*si).lfb_linelength = (*info).pixels_per_scan_line * 4; } (*si).lfb_size = (*si).lfb_linelength * (*si).lfb_height; (*si).capabilities |= VIDEO_CAPABILITY_SKIP_QUIRKS; }

unsafe fn setup_edid_info(edid: *mut edid_info, size: u32, data: *mut u8) { if data.is_null() || size < 128 { memset((*edid).dummy.as_mut_ptr() as *mut _, 0, core::mem::size_of_val(&(*edid).dummy)); } else { memcpy((*edid).dummy.as_mut_ptr() as *mut _, data as *const _, core::cmp::min(size as usize, core::mem::size_of_val(&(*edid).dummy))); } }

unsafe fn find_handle_with_primary_gop(num: usize, handles: *const efi_handle_t, found: *mut *mut efi_graphics_output_protocol_t) -> efi_handle_t { let mut first_h = core::ptr::null_mut(); let mut first_g = core::ptr::null_mut(); for i in 0..num { let h = *handles.add(i); let mut gop = core::ptr::null_mut(); if efi_bs_call(handle_protocol, h, &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID, &mut gop as *mut _ as *mut _) != EFI_SUCCESS { continue; } let mode = efi_table_attr(gop, mode); let info = efi_table_attr(mode, info); if (*info).pixel_format == PIXEL_BLT_ONLY || (*info).pixel_format >= PIXEL_FORMAT_MAX { continue; } let mut dummy = core::ptr::null_mut(); if efi_bs_call(handle_protocol, h, &EFI_CONSOLE_OUT_DEVICE_GUID, &mut dummy) == EFI_SUCCESS { if !found.is_null() { *found = gop; } return h; } else if first_h.is_null() { first_h = h; first_g = gop; } } if !found.is_null() { *found = first_g; } first_h }

pub unsafe fn efi_setup_graphics(si: *mut screen_info, edid: *mut edid_info) -> efi_status_t { let mut handles = core::ptr::null_mut(); let mut num = 0; let status = efi_bs_call(locate_handle_buffer, EFI_LOCATE_BY_PROTOCOL, &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID, core::ptr::null_mut(), &mut num, &mut handles); if status != EFI_SUCCESS { return status; } let mut gop = core::ptr::null_mut(); let handle = find_handle_with_primary_gop(num, handles, &mut gop); if handle.is_null() { return EFI_NOT_FOUND; } set_mode(gop); if !si.is_null() { setup_screen_info(si, gop); } if !edid.is_null() { let mut size = 0; let mut data = core::ptr::null_mut(); let mut active = core::ptr::null_mut(); if efi_bs_call(handle_protocol, handle, &EFI_EDID_ACTIVE_PROTOCOL_GUID, &mut active as *mut _ as *mut _) == EFI_SUCCESS { size = efi_table_attr(active, size_of_edid); data = efi_table_attr(active, edid); } else { let mut discovered = core::ptr::null_mut(); if efi_bs_call(handle_protocol, handle, &EFI_EDID_DISCOVERED_PROTOCOL_GUID, &mut discovered as *mut _ as *mut _) == EFI_SUCCESS { size = efi_table_attr(discovered, size_of_edid); data = efi_table_attr(discovered, edid); } } setup_edid_info(edid, size, data); } EFI_SUCCESS }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
