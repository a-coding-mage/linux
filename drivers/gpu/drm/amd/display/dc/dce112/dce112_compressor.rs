/* Translated from dce112_compressor.c. */

/* External declarations, register definitions, structures, and helper macros
 * are supplied by the surrounding driver translation. */

const DCE11_ONE_LPT_CHANNEL_MAX_RESOLUTION: u32 = 2560 * 1600;

#[repr(C)]
pub struct Dce112CompressorRegOffsets { pub dcp_offset: u32, pub dmif_offset: u32 }

static REG_OFFSETS: [Dce112CompressorRegOffsets; 3] = [
    Dce112CompressorRegOffsets { dcp_offset: mmDCP0_GRPH_CONTROL - mmDCP0_GRPH_CONTROL, dmif_offset: mmDMIF_PG0_DPG_PIPE_DPM_CONTROL - mmDMIF_PG0_DPG_PIPE_DPM_CONTROL },
    Dce112CompressorRegOffsets { dcp_offset: mmDCP1_GRPH_CONTROL - mmDCP0_GRPH_CONTROL, dmif_offset: mmDMIF_PG1_DPG_PIPE_DPM_CONTROL - mmDMIF_PG0_DPG_PIPE_DPM_CONTROL },
    Dce112CompressorRegOffsets { dcp_offset: mmDCP2_GRPH_CONTROL - mmDCP0_GRPH_CONTROL, dmif_offset: mmDMIF_PG2_DPG_PIPE_DPM_CONTROL - mmDMIF_PG0_DPG_PIPE_DPM_CONTROL },
];

#[repr(u32)]
enum FbcIdleForce {
    FBC_IDLE_FORCE_DISPLAY_REGISTER_UPDATE = 0x00000001,
    FBC_IDLE_FORCE_GRPH_COMP_EN = 0x00000002,
    FBC_IDLE_FORCE_SRC_SEL_CHANGE = 0x00000004,
    FBC_IDLE_FORCE_MIN_COMPRESSION_CHANGE = 0x00000008,
    FBC_IDLE_FORCE_ALPHA_COMP_EN = 0x00000010,
    FBC_IDLE_FORCE_ZERO_ALPHA_CHUNK_SKIP_EN = 0x00000020,
    FBC_IDLE_FORCE_FORCE_COPY_TO_COMP_BUF = 0x00000040,
    FBC_IDLE_FORCE_MEMORY_WRITE_TO_REGION0 = 0x01000000,
    FBC_IDLE_FORCE_MEMORY_WRITE_TO_REGION1 = 0x02000000,
    FBC_IDLE_FORCE_MEMORY_WRITE_TO_REGION2 = 0x04000000,
    FBC_IDLE_FORCE_MEMORY_WRITE_TO_REGION3 = 0x08000000,
    FBC_IDLE_FORCE_MEMORY_WRITE_OTHER_THAN_MCIF = 0x10000000,
    FBC_IDLE_FORCE_CG_STATIC_SCREEN_IS_INACTIVE = 0x20000000,
}

unsafe fn lpt_size_alignment(cp110: *mut dce112_compressor) -> u32 {
    (*cp110).base.raw_size * (*cp110).base.banks_num * (*cp110).base.dram_channels_num
}

unsafe fn lpt_memory_control_config(cp110: *mut dce112_compressor, mut lpt_control: u32) -> u32 {
    if (*cp110).base.options.bits.LPT_MC_CONFIG == 1 {
        match (*cp110).base.dram_channels_num {
            2 => set_reg_field_value!(lpt_control, 1, LOW_POWER_TILING_CONTROL, LOW_POWER_TILING_NUM_PIPES),
            1 => set_reg_field_value!(lpt_control, 0, LOW_POWER_TILING_CONTROL, LOW_POWER_TILING_NUM_PIPES),
            _ => DC_LOG_WARNING!("%s: Invalid LPT NUM_PIPES!!!", __func__),
        }
        match (*cp110).base.banks_num {
            16 => set_reg_field_value!(lpt_control, 3, LOW_POWER_TILING_CONTROL, LOW_POWER_TILING_NUM_BANKS),
            8 => set_reg_field_value!(lpt_control, 2, LOW_POWER_TILING_CONTROL, LOW_POWER_TILING_NUM_BANKS),
            4 => set_reg_field_value!(lpt_control, 1, LOW_POWER_TILING_CONTROL, LOW_POWER_TILING_NUM_BANKS),
            2 => set_reg_field_value!(lpt_control, 0, LOW_POWER_TILING_CONTROL, LOW_POWER_TILING_NUM_BANKS),
            _ => DC_LOG_WARNING!("%s: Invalid LPT NUM_BANKS!!!", __func__),
        }
        match (*cp110).base.channel_interleave_size {
            256 => set_reg_field_value!(lpt_control, 0, LOW_POWER_TILING_CONTROL, LOW_POWER_TILING_PIPE_INTERLEAVE_SIZE),
            512 => set_reg_field_value!(lpt_control, 1, LOW_POWER_TILING_CONTROL, LOW_POWER_TILING_PIPE_INTERLEAVE_SIZE),
            _ => DC_LOG_WARNING!("%s: Invalid LPT INTERLEAVE_SIZE!!!", __func__),
        }
        match (*cp110).base.raw_size {
            4096 => set_reg_field_value!(lpt_control, 2, LOW_POWER_TILING_CONTROL, LOW_POWER_TILING_ROW_SIZE),
            2048 => set_reg_field_value!(lpt_control, 1, LOW_POWER_TILING_CONTROL, LOW_POWER_TILING_ROW_SIZE),
            1024 => set_reg_field_value!(lpt_control, 0, LOW_POWER_TILING_CONTROL, LOW_POWER_TILING_ROW_SIZE),
            _ => DC_LOG_WARNING!("%s: Invalid LPT ROW_SIZE!!!", __func__),
        }
    } else { DC_LOG_WARNING!("%s: LPT MC Configuration is not provided", __func__); }
    lpt_control
}

unsafe fn is_source_bigger_than_epanel_size(cp110: *mut dce112_compressor, w: u32, h: u32) -> bool {
    (*cp110).base.embedded_panel_h_size != 0 && (*cp110).base.embedded_panel_v_size != 0 &&
        w * h > (*cp110).base.embedded_panel_h_size * (*cp110).base.embedded_panel_v_size
}
unsafe fn align_to_chunks_number_per_line(_cp110: *mut dce112_compressor, pixels: u32) -> u32 { 256 * ((pixels + 255) / 256) }

unsafe fn wait_for_fbc_state_changed(cp110: *mut dce112_compressor, enabled: bool) {
    let mut counter: u8 = 0;
    while counter < 10 {
        let value = dm_read_reg((*cp110).base.ctx, mmFBC_STATUS);
        if get_reg_field_value!(value, FBC_STATUS, FBC_ENABLE_STATUS) == enabled { break; }
        udelay(10); counter += 1;
    }
    if counter == 10 { DC_LOG_WARNING!("%s: wait counter exceeded, changes to HW not applied", __func__); }
}

pub unsafe fn dce112_compressor_power_up_fbc(compressor: *mut compressor) {
    let mut value = dm_read_reg((*compressor).ctx, mmFBC_CNTL);
    set_reg_field_value!(value, 0, FBC_CNTL, FBC_GRPH_COMP_EN); set_reg_field_value!(value, 1, FBC_CNTL, FBC_EN); set_reg_field_value!(value, 2, FBC_CNTL, FBC_COHERENCY_MODE);
    if (*compressor).options.bits.CLK_GATING_DISABLED == 1 { set_reg_field_value!(value, 0, FBC_CNTL, FBC_COMP_CLK_GATE_EN); }
    dm_write_reg((*compressor).ctx, mmFBC_CNTL, value);
    value = dm_read_reg((*compressor).ctx, mmFBC_COMP_MODE); set_reg_field_value!(value, 1, FBC_COMP_MODE, FBC_RLE_EN); set_reg_field_value!(value, 1, FBC_COMP_MODE, FBC_DPCM4_RGB_EN); set_reg_field_value!(value, 1, FBC_COMP_MODE, FBC_IND_EN); dm_write_reg((*compressor).ctx, mmFBC_COMP_MODE, value);
    value = dm_read_reg((*compressor).ctx, mmFBC_COMP_CNTL); set_reg_field_value!(value, 1, FBC_COMP_CNTL, FBC_DEPTH_RGB08_EN); dm_write_reg((*compressor).ctx, mmFBC_COMP_CNTL, value); set_reg_field_value!(value, 0xF, FBC_COMP_CNTL, FBC_MIN_COMPRESSION); dm_write_reg((*compressor).ctx, mmFBC_COMP_CNTL, value); (*compressor).min_compress_ratio = FBC_COMPRESS_RATIO_1TO1;
    dm_write_reg((*compressor).ctx, mmFBC_IND_LUT0, 0); dm_write_reg((*compressor).ctx, mmFBC_IND_LUT1, 0xFFFFFF);
}

pub unsafe fn dce112_compressor_enable_fbc(compressor: *mut compressor, paths_num: u32, params: *mut compr_addr_and_pitch_params) {
    let cp110 = TO_DCE112_COMPRESSOR!(compressor);
    if (*compressor).options.bits.FBC_SUPPORT && (*compressor).options.bits.DUMMY_BACKEND == 0 && !dce112_compressor_is_fbc_enabled_in_hw(compressor, core::ptr::null_mut()) && !is_source_bigger_than_epanel_size(cp110, (*params).source_view_width, (*params).source_view_height) {
        if (*compressor).options.bits.LPT_SUPPORT && paths_num < 2 && (*params).source_view_width * (*params).source_view_height <= DCE11_ONE_LPT_CHANNEL_MAX_RESOLUTION { dce112_compressor_enable_lpt(compressor); }
        let addr = mmFBC_CNTL; let mut value = dm_read_reg((*compressor).ctx, addr); set_reg_field_value!(value, 1, FBC_CNTL, FBC_GRPH_COMP_EN); set_reg_field_value!(value, (*params).inst, FBC_CNTL, FBC_SRC_SEL); dm_write_reg((*compressor).ctx, addr, value);
        (*compressor).is_enabled = true; (*compressor).attached_inst = (*params).inst; (*cp110).offsets = REG_OFFSETS[(*params).inst as usize];
        set_reg_field_value!(value, 0, FBC_CNTL, FBC_GRPH_COMP_EN); dm_write_reg((*compressor).ctx, addr, value); set_reg_field_value!(value, 1, FBC_CNTL, FBC_GRPH_COMP_EN); dm_write_reg((*compressor).ctx, addr, value); wait_for_fbc_state_changed(cp110, true);
    }
}

pub unsafe fn dce112_compressor_disable_fbc(compressor: *mut compressor) { let cp110 = TO_DCE112_COMPRESSOR!(compressor); if (*compressor).options.bits.FBC_SUPPORT && dce112_compressor_is_fbc_enabled_in_hw(compressor, core::ptr::null_mut()) { let mut v=dm_read_reg((*compressor).ctx,mmFBC_CNTL); set_reg_field_value!(v,0,FBC_CNTL,FBC_GRPH_COMP_EN); dm_write_reg((*compressor).ctx,mmFBC_CNTL,v); (*compressor).attached_inst=0; (*compressor).is_enabled=false; if (*compressor).options.bits.LPT_SUPPORT { dce112_compressor_disable_lpt(compressor); } wait_for_fbc_state_changed(cp110,false); } }

pub unsafe fn dce112_compressor_is_fbc_enabled_in_hw(compressor: *mut compressor, inst: *mut u32) -> bool { let mut value=dm_read_reg((*compressor).ctx,mmFBC_STATUS); if get_reg_field_value!(value,FBC_STATUS,FBC_ENABLE_STATUS) != 0 { if !inst.is_null(){*inst=(*compressor).attached_inst;} return true; } value=dm_read_reg((*compressor).ctx,mmFBC_MISC); if get_reg_field_value!(value,FBC_MISC,FBC_STOP_ON_HFLIP_EVENT)!=0 { value=dm_read_reg((*compressor).ctx,mmFBC_CNTL); if get_reg_field_value!(value,FBC_CNTL,FBC_GRPH_COMP_EN)!=0 { if !inst.is_null(){*inst=(*compressor).attached_inst;} return true; } } false }
pub unsafe fn dce112_compressor_is_lpt_enabled_in_hw(compressor: *mut compressor) -> bool { let value=dm_read_reg((*compressor).ctx,mmLOW_POWER_TILING_CONTROL); get_reg_field_value!(value,LOW_POWER_TILING_CONTROL,LOW_POWER_TILING_ENABLE) != 0 }

pub unsafe fn dce112_compressor_program_compressed_surface_address_and_pitch(compressor: *mut compressor, params: *mut compr_addr_and_pitch_params) {
    let cp110=TO_DCE112_COMPRESSOR!(compressor); let mut value=0; let mut fbc_pitch=0; let mut low=(*compressor).compr_surface_address.addr.low_part;
    dm_write_reg((*compressor).ctx,DCP_REG!(mmGRPH_COMPRESS_SURFACE_ADDRESS_HIGH,(*cp110).offsets),0); dm_write_reg((*compressor).ctx,DCP_REG!(mmGRPH_COMPRESS_SURFACE_ADDRESS,(*cp110).offsets),0);
    if (*compressor).options.bits.LPT_SUPPORT { let a=lpt_size_alignment(cp110); if a!=0 { low=((low+a-1)/a)*a; } }
    dm_write_reg((*compressor).ctx,DCP_REG!(mmGRPH_COMPRESS_SURFACE_ADDRESS_HIGH,(*cp110).offsets),(*compressor).compr_surface_address.addr.high_part); dm_write_reg((*compressor).ctx,DCP_REG!(mmGRPH_COMPRESS_SURFACE_ADDRESS,(*cp110).offsets),low);
    fbc_pitch=align_to_chunks_number_per_line(cp110,(*params).source_view_width); if (*compressor).min_compress_ratio==FBC_COMPRESS_RATIO_1TO1 { fbc_pitch/=8; } else { DC_LOG_WARNING!("%s: Unexpected DCE11 compression ratio",__func__); }
    dm_write_reg((*compressor).ctx,DCP_REG!(mmGRPH_COMPRESS_PITCH,(*cp110).offsets),0); set_reg_field_value!(value,fbc_pitch,GRPH_COMPRESS_PITCH,GRPH_COMPRESS_PITCH); dm_write_reg((*compressor).ctx,DCP_REG!(mmGRPH_COMPRESS_PITCH,(*cp110).offsets),value);
}

pub unsafe fn dce112_compressor_disable_lpt(compressor: *mut compressor) {
    let cp110=TO_DCE112_COMPRESSOR!(compressor); for _ in 0..3 { let mut v=dm_read_reg((*compressor).ctx,DMIF_REG!(mmDPG_PIPE_STUTTER_CONTROL_NONLPTCH,(*cp110).offsets)); set_reg_field_value!(v,0,DPG_PIPE_STUTTER_CONTROL_NONLPTCH,STUTTER_ENABLE_NONLPTCH); dm_write_reg((*compressor).ctx,DMIF_REG!(mmDPG_PIPE_STUTTER_CONTROL_NONLPTCH,(*cp110).offsets),v); }
    let addr=mmDPGV0_PIPE_STUTTER_CONTROL_NONLPTCH; let mut v=dm_read_reg((*compressor).ctx,addr); set_reg_field_value!(v,0,DPGV0_PIPE_STUTTER_CONTROL_NONLPTCH,STUTTER_ENABLE_NONLPTCH); dm_write_reg((*compressor).ctx,addr,v);
    let addr=mmLOW_POWER_TILING_CONTROL; let mut v=dm_read_reg((*compressor).ctx,addr); set_reg_field_value!(v,0,LOW_POWER_TILING_CONTROL,LOW_POWER_TILING_ENABLE); dm_write_reg((*compressor).ctx,addr,v);
    let addr=mmGMCON_LPT_TARGET; v=dm_read_reg((*compressor).ctx,addr); set_reg_field_value!(v,0xFFFFFFFF,GMCON_LPT_TARGET,STCTRL_LPT_TARGET); dm_write_reg((*compressor).ctx,addr,v);
}

pub unsafe fn dce112_compressor_enable_lpt(compressor: *mut compressor) {
    let cp110=TO_DCE112_COMPRESSOR!(compressor); let mut v=dm_read_reg((*compressor).ctx,DMIF_REG!(mmDPG_PIPE_STUTTER_CONTROL_NONLPTCH,(*cp110).offsets)); set_reg_field_value!(v,1,DPG_PIPE_STUTTER_CONTROL_NONLPTCH,STUTTER_ENABLE_NONLPTCH); dm_write_reg((*compressor).ctx,DMIF_REG!(mmDPG_PIPE_STUTTER_CONTROL_NONLPTCH,(*cp110).offsets),v);
    let addr=mmDPGV0_PIPE_STUTTER_CONTROL_NONLPTCH; v=dm_read_reg((*compressor).ctx,addr); set_reg_field_value!(v,1,DPGV0_PIPE_STUTTER_CONTROL_NONLPTCH,STUTTER_ENABLE_NONLPTCH); dm_write_reg((*compressor).ctx,addr,v);
    let addr=mmLOW_POWER_TILING_CONTROL; let control=dm_read_reg((*compressor).ctx,addr); let channels=get_reg_field_value!(control,LOW_POWER_TILING_CONTROL,LOW_POWER_TILING_MODE); let addr=mmGMCON_LPT_TARGET; v=dm_read_reg((*compressor).ctx,addr); set_reg_field_value!(v,channels+1,GMCON_LPT_TARGET,STCTRL_LPT_TARGET); dm_write_reg((*compressor).ctx,addr,v);
    let addr=mmLOW_POWER_TILING_CONTROL; v=dm_read_reg((*compressor).ctx,addr); set_reg_field_value!(v,1,LOW_POWER_TILING_CONTROL,LOW_POWER_TILING_ENABLE); dm_write_reg((*compressor).ctx,addr,v);
}

pub unsafe fn dce112_compressor_program_lpt_control(compressor: *mut compressor, params: *mut compr_addr_and_pitch_params) {
    let cp110=TO_DCE112_COMPRESSOR!(compressor); if !(*compressor).options.bits.LPT_SUPPORT { return; } let mut control=dm_read_reg((*compressor).ctx,mmLOW_POWER_TILING_CONTROL);
    if (*compressor).lpt_channels_num==1 { set_reg_field_value!(control,0,LOW_POWER_TILING_CONTROL,LOW_POWER_TILING_MODE); } else { DC_LOG_WARNING!("%s: Invalid selected DRAM channels for LPT!!!",__func__); }
    control=lpt_memory_control_config(cp110,control); let alignment=lpt_size_alignment(cp110); let width=align_to_chunks_number_per_line(cp110,(*params).source_view_width); let height=((*params).source_view_height+1)&(!0x1); let mut rows=0; if alignment!=0 { let bytes=width*height*4; rows=if bytes%alignment!=0 { bytes/alignment+1 } else { bytes/alignment }; } set_reg_field_value!(control,rows,LOW_POWER_TILING_CONTROL,LOW_POWER_TILING_ROWS_PER_CHAN); dm_write_reg((*compressor).ctx,mmLOW_POWER_TILING_CONTROL,control);
}

pub unsafe fn dce112_compressor_set_fbc_invalidation_triggers(compressor: *mut compressor, fbc_trigger: u32) {
    let addr=mmFBC_CLIENT_REGION_MASK; let mut value=dm_read_reg((*compressor).ctx,addr); set_reg_field_value!(value,0,FBC_CLIENT_REGION_MASK,FBC_MEMORY_REGION_MASK); dm_write_reg((*compressor).ctx,addr,value);
    let addr=mmFBC_IDLE_FORCE_CLEAR_MASK; value=dm_read_reg((*compressor).ctx,addr); let mask=fbc_trigger|FBC_IDLE_FORCE_GRPH_COMP_EN as u32|FBC_IDLE_FORCE_SRC_SEL_CHANGE as u32|FBC_IDLE_FORCE_MIN_COMPRESSION_CHANGE as u32|FBC_IDLE_FORCE_ALPHA_COMP_EN as u32|FBC_IDLE_FORCE_ZERO_ALPHA_CHUNK_SKIP_EN as u32|FBC_IDLE_FORCE_FORCE_COPY_TO_COMP_BUF as u32; set_reg_field_value!(value,mask,FBC_IDLE_FORCE_CLEAR_MASK,FBC_IDLE_FORCE_CLEAR_MASK); dm_write_reg((*compressor).ctx,addr,value);
}

pub unsafe fn dce112_compressor_construct(compressor: *mut dce112_compressor, ctx: *mut dc_context) {
    let bp=(*ctx).dc_bios; (*compressor).base.options.raw=0; (*compressor).base.options.bits.FBC_SUPPORT=true; (*compressor).base.options.bits.LPT_SUPPORT=true; (*compressor).base.lpt_channels_num=1; (*compressor).base.options.bits.DUMMY_BACKEND=false; if (*compressor).base.memory_bus_width==64 { (*compressor).base.options.bits.LPT_SUPPORT=false; } (*compressor).base.options.bits.CLK_GATING_DISABLED=false; (*compressor).base.ctx=ctx; (*compressor).base.embedded_panel_h_size=0; (*compressor).base.embedded_panel_v_size=0; (*compressor).base.memory_bus_width=(*ctx).asic_id.vram_width; (*compressor).base.allocated_size=0; (*compressor).base.preferred_requested_size=0; (*compressor).base.min_compress_ratio=FBC_COMPRESS_RATIO_INVALID; (*compressor).base.banks_num=0; (*compressor).base.raw_size=0; (*compressor).base.channel_interleave_size=0; (*compressor).base.dram_channels_num=0; (*compressor).base.lpt_channels_num=0; (*compressor).base.attached_inst=0; (*compressor).base.is_enabled=false;
    let mut panel_info=core::mem::zeroed(); if BP_RESULT_OK==(*bp).funcs.get_embedded_panel_info(bp,&mut panel_info) { (*compressor).base.embedded_panel_h_size=panel_info.lcd_timing.horizontal_addressable; (*compressor).base.embedded_panel_v_size=panel_info.lcd_timing.vertical_addressable; }
}
pub unsafe fn dce112_compressor_create(ctx: *mut dc_context) -> *mut compressor { let cp110=kzalloc_obj::<dce112_compressor>(); if cp110.is_null(){return core::ptr::null_mut();} dce112_compressor_construct(cp110,ctx); &mut (*cp110).base }
pub unsafe fn dce112_compressor_destroy(compressor: *mut *mut compressor) { kfree(TO_DCE112_COMPRESSOR!(*compressor)); *compressor=core::ptr::null_mut(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
