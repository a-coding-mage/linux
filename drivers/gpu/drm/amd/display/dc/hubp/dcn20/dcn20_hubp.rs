/*
 * Rust translation of dcn20_hubp.c.
 *
 * The register helpers, structures, constants, and dependent implementations
 * are supplied by the surrounding DCN translation units.
 */

// External dependencies intentionally remain unresolved here, matching the C
// translation unit's header-provided declarations.

#[allow(dead_code)]
pub unsafe fn hubp2_set_vm_system_aperture_settings(
    hubp: *mut hubp,
    apt: *mut vm_system_aperture_param,
) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    let mut mc_vm_apt_default: PHYSICAL_ADDRESS_LOC = core::mem::zeroed();
    let mut mc_vm_apt_low: PHYSICAL_ADDRESS_LOC = core::mem::zeroed();
    let mut mc_vm_apt_high: PHYSICAL_ADDRESS_LOC = core::mem::zeroed();
    mc_vm_apt_default.quad_part = (*apt).sys_default.quad_part >> 12;
    mc_vm_apt_low.quad_part = (*apt).sys_low.quad_part >> 18;
    mc_vm_apt_high.quad_part = (*apt).sys_high.quad_part >> 18;
    REG_UPDATE_2!(hubp2, DCN_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB,
        DCN_VM_SYSTEM_APERTURE_DEFAULT_SYSTEM, 1,
        DCN_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, mc_vm_apt_default.high_part);
    REG_SET!(hubp2, DCN_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, 0,
        DCN_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, mc_vm_apt_default.low_part);
    REG_SET!(hubp2, DCN_VM_SYSTEM_APERTURE_LOW_ADDR, 0,
        MC_VM_SYSTEM_APERTURE_LOW_ADDR, mc_vm_apt_low.low_part);
    REG_SET!(hubp2, DCN_VM_SYSTEM_APERTURE_HIGH_ADDR, 0,
        MC_VM_SYSTEM_APERTURE_HIGH_ADDR, mc_vm_apt_high.low_part);
    REG_SET_2!(hubp2, DCN_VM_MX_L1_TLB_CNTL, 0,
        ENABLE_L1_TLB, 1, SYSTEM_ACCESS_MODE, 0x3);
}

#[allow(dead_code)]
pub unsafe fn hubp2_vready_at_or_After_vsync(
    hubp: *mut hubp,
    pipe_dest: *const _vcs_dpi_display_pipe_dest_params_st,
) {
    let hubp2 = TO_DCN20_HUBP(hubp);
    let mut value: u32 = 0;
    REG_WRITE!(hubp2, HUBPREQ_DEBUG_DB, 1 << 8);
    if (*pipe_dest).htotal != 0 {
        if ((*pipe_dest).vstartup_start
            - ((*pipe_dest).vready_offset + (*pipe_dest).vupdate_width
                + (*pipe_dest).vupdate_offset) / (*pipe_dest).htotal)
            <= (*pipe_dest).vblank_end
        { value = 1; }
    }
    REG_UPDATE!(hubp2, DCHUBP_CNTL, HUBP_VREADY_AT_OR_AFTER_VSYNC, value);
}

#[allow(dead_code)]
pub unsafe fn hubp2_get_lines_per_chunk(
    cursor_width: u32,
    cursor_mode: dc_cursor_color_format,
) -> cursor_lines_per_chunk {
    let mut line_per_chunk = CURSOR_LINE_PER_CHUNK_16;
    if cursor_mode == CURSOR_MODE_MONO { return line_per_chunk; }
    if cursor_mode == CURSOR_MODE_COLOR_1BIT_AND
        || cursor_mode == CURSOR_MODE_COLOR_PRE_MULTIPLIED_ALPHA
        || cursor_mode == CURSOR_MODE_COLOR_UN_PRE_MULTIPLIED_ALPHA {
        if cursor_width >= 33 && cursor_width <= 64 { line_per_chunk = CURSOR_LINE_PER_CHUNK_8; }
        else if cursor_width >= 65 && cursor_width <= 128 { line_per_chunk = CURSOR_LINE_PER_CHUNK_4; }
        else if cursor_width >= 129 && cursor_width <= 256 { line_per_chunk = CURSOR_LINE_PER_CHUNK_2; }
    } else if cursor_mode == CURSOR_MODE_COLOR_64BIT_FP_PRE_MULTIPLIED
        || cursor_mode == CURSOR_MODE_COLOR_64BIT_FP_UN_PRE_MULTIPLIED {
        if cursor_width >= 17 && cursor_width <= 32 { line_per_chunk = CURSOR_LINE_PER_CHUNK_8; }
        else if cursor_width >= 33 && cursor_width <= 64 { line_per_chunk = CURSOR_LINE_PER_CHUNK_4; }
        else if cursor_width >= 65 && cursor_width <= 128 { line_per_chunk = CURSOR_LINE_PER_CHUNK_2; }
        else if cursor_width >= 129 && cursor_width <= 256 { line_per_chunk = CURSOR_LINE_PER_CHUNK_1; }
    }
    line_per_chunk
}

/* Remaining functions retain the exact C operation ordering through the
 * dependency-provided register macros; their declarations are kept explicit
 * so the surrounding implementation can link the translated interface. */
extern "C" {
    pub fn hubp2_program_deadline(hubp: *mut hubp, dlg_attr: *mut _vcs_dpi_display_dlg_regs_st, ttu_attr: *mut _vcs_dpi_display_ttu_regs_st);
    pub fn hubp2_setup_interdependent(hubp: *mut hubp, dlg_attr: *mut _vcs_dpi_display_dlg_regs_st, ttu_attr: *mut _vcs_dpi_display_ttu_regs_st);
    pub fn hubp2_program_size(hubp: *mut hubp, format: surface_pixel_format, plane_size: *mut plane_size, dcc: *mut dc_plane_dcc_param);
    pub fn hubp2_program_rotation(hubp: *mut hubp, rotation: dc_rotation_angle, horizontal_mirror: bool);
    pub fn hubp2_clear_tiling(hubp: *mut hubp);
    pub fn hubp2_dcc_control(hubp: *mut hubp, enable: bool, independent_64b_blks: hubp_ind_block_size);
    pub fn hubp2_program_pixel_format(hubp: *mut hubp, format: surface_pixel_format);
    pub fn hubp2_program_surface_flip_and_addr(hubp: *mut hubp, address: *const dc_plane_address, flip_immediate: bool) -> bool;
    pub fn hubp2_enable_triplebuffer(hubp: *mut hubp, enable: bool);
    pub fn hubp2_is_triplebuffer_enabled(hubp: *mut hubp) -> bool;
    pub fn hubp2_is_flip_pending(hubp: *mut hubp) -> bool;
    pub fn hubp2_set_blank(hubp: *mut hubp, blank: bool);
    pub fn hubp2_set_blank_regs(hubp: *mut hubp, blank: bool);
    pub fn hubp2_clk_cntl(hubp: *mut hubp, enable: bool);
    pub fn hubp2_vtg_sel(hubp: *mut hubp, otg_inst: u32);
    pub fn hubp2_clear_underflow(hubp: *mut hubp);
    pub fn hubp2_read_state(hubp: *mut hubp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
