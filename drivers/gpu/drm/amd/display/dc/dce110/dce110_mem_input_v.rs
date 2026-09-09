/* Rust translation of dce110_mem_input_v.c. External types, constants, and
 * register helpers are supplied by the surrounding driver. */

unsafe fn set_flip_control(mem_input110: *mut dce_mem_input, _immediate: bool) {
    let mut value: u32 = dm_read_reg((*mem_input110).base.ctx, mmUNP_FLIP_CONTROL);
    set_reg_field_value(value, 1, UNP_FLIP_CONTROL, GRPH_SURFACE_UPDATE_PENDING_MODE);
    dm_write_reg((*mem_input110).base.ctx, mmUNP_FLIP_CONTROL, value);
}

unsafe fn program_pri_addr_c(mem_input110: *mut dce_mem_input, address: PHYSICAL_ADDRESS_LOC) {
    let temp = address.high_part & UNP_GRPH_PRIMARY_SURFACE_ADDRESS_HIGH_C__GRPH_PRIMARY_SURFACE_ADDRESS_HIGH_C_MASK;
    let mut value = 0u32;
    set_reg_field_value(value, temp, UNP_GRPH_PRIMARY_SURFACE_ADDRESS_HIGH_C, GRPH_PRIMARY_SURFACE_ADDRESS_HIGH_C);
    dm_write_reg((*mem_input110).base.ctx, mmUNP_GRPH_PRIMARY_SURFACE_ADDRESS_HIGH_C, value);
    value = 0;
    let temp = address.low_part >> UNP_GRPH_PRIMARY_SURFACE_ADDRESS_C__GRPH_PRIMARY_SURFACE_ADDRESS_C__SHIFT;
    set_reg_field_value(value, temp, UNP_GRPH_PRIMARY_SURFACE_ADDRESS_C, GRPH_PRIMARY_SURFACE_ADDRESS_C);
    dm_write_reg((*mem_input110).base.ctx, mmUNP_GRPH_PRIMARY_SURFACE_ADDRESS_C, value);
}

unsafe fn program_pri_addr_l(mem_input110: *mut dce_mem_input, address: PHYSICAL_ADDRESS_LOC) {
    let temp = address.high_part & UNP_GRPH_PRIMARY_SURFACE_ADDRESS_HIGH_L__GRPH_PRIMARY_SURFACE_ADDRESS_HIGH_L_MASK;
    let mut value = 0u32;
    set_reg_field_value(value, temp, UNP_GRPH_PRIMARY_SURFACE_ADDRESS_HIGH_L, GRPH_PRIMARY_SURFACE_ADDRESS_HIGH_L);
    dm_write_reg((*mem_input110).base.ctx, mmUNP_GRPH_PRIMARY_SURFACE_ADDRESS_HIGH_L, value);
    value = 0;
    let temp = address.low_part >> UNP_GRPH_PRIMARY_SURFACE_ADDRESS_L__GRPH_PRIMARY_SURFACE_ADDRESS_L__SHIFT;
    set_reg_field_value(value, temp, UNP_GRPH_PRIMARY_SURFACE_ADDRESS_L, GRPH_PRIMARY_SURFACE_ADDRESS_L);
    dm_write_reg((*mem_input110).base.ctx, mmUNP_GRPH_PRIMARY_SURFACE_ADDRESS_L, value);
}

unsafe fn program_addr(mi: *mut dce_mem_input, addr: *const dc_plane_address) {
    match (*addr).type_ {
        PLN_ADDR_TYPE_GRAPHICS => program_pri_addr_l(mi, (*addr).grph.addr),
        PLN_ADDR_TYPE_VIDEO_PROGRESSIVE => {
            program_pri_addr_c(mi, (*addr).video_progressive.chroma_addr);
            program_pri_addr_l(mi, (*addr).video_progressive.luma_addr);
        }
        _ => BREAK_TO_DEBUGGER!(),
    }
}

unsafe fn enable(mi: *mut dce_mem_input) {
    let mut value = dm_read_reg((*mi).base.ctx, mmUNP_GRPH_ENABLE);
    set_reg_field_value(value, 1, UNP_GRPH_ENABLE, GRPH_ENABLE);
    dm_write_reg((*mi).base.ctx, mmUNP_GRPH_ENABLE, value);
}

unsafe fn program_tiling(mi: *mut dce_mem_input, info: *const dc_tiling_info, _pixel_format: surface_pixel_format) {
    let mut value = 0u32;
    macro_rules! f { ($v:expr, $r:ident, $n:ident) => { set_reg_field_value(value, $v, $r, $n); }; }
    f!((*info).gfx8.num_banks, UNP_GRPH_CONTROL, GRPH_NUM_BANKS); f!((*info).gfx8.bank_width, UNP_GRPH_CONTROL, GRPH_BANK_WIDTH_L);
    f!((*info).gfx8.bank_height, UNP_GRPH_CONTROL, GRPH_BANK_HEIGHT_L); f!((*info).gfx8.tile_aspect, UNP_GRPH_CONTROL, GRPH_MACRO_TILE_ASPECT_L);
    f!((*info).gfx8.tile_split, UNP_GRPH_CONTROL, GRPH_TILE_SPLIT_L); f!((*info).gfx8.tile_mode, UNP_GRPH_CONTROL, GRPH_MICRO_TILE_MODE_L);
    f!((*info).gfx8.pipe_config, UNP_GRPH_CONTROL, GRPH_PIPE_CONFIG); f!((*info).gfx8.array_mode, UNP_GRPH_CONTROL, GRPH_ARRAY_MODE);
    f!(1, UNP_GRPH_CONTROL, GRPH_COLOR_EXPANSION_MODE); f!(0, UNP_GRPH_CONTROL, GRPH_Z);
    dm_write_reg((*mi).base.ctx, mmUNP_GRPH_CONTROL, value); value = 0;
    f!((*info).gfx8.bank_width_c, UNP_GRPH_CONTROL_C, GRPH_BANK_WIDTH_C); f!((*info).gfx8.bank_height_c, UNP_GRPH_CONTROL_C, GRPH_BANK_HEIGHT_C);
    f!((*info).gfx8.tile_aspect_c, UNP_GRPH_CONTROL_C, GRPH_MACRO_TILE_ASPECT_C); f!((*info).gfx8.tile_split_c, UNP_GRPH_CONTROL_C, GRPH_TILE_SPLIT_C);
    f!((*info).gfx8.tile_mode_c, UNP_GRPH_CONTROL_C, GRPH_MICRO_TILE_MODE_C);
    dm_write_reg((*mi).base.ctx, mmUNP_GRPH_CONTROL_C, value);
}

unsafe fn program_size_and_rotation(mi: *mut dce_mem_input, rotation: dc_rotation_angle, plane_size: *const plane_size) {
    let mut s = *plane_size;
    if rotation == ROTATION_ANGLE_90 || rotation == ROTATION_ANGLE_270 { swap(&mut s.surface_size.x, &mut s.surface_size.y); swap(&mut s.surface_size.width, &mut s.surface_size.height); swap(&mut s.chroma_size.x, &mut s.chroma_size.y); swap(&mut s.chroma_size.width, &mut s.chroma_size.height); }
    let mut value = 0u32;
    macro_rules! w { ($r:ident, $n:ident, $v:expr) => { value=0; set_reg_field_value(value,$v,$r,$n); dm_write_reg((*mi).base.ctx,$r,value); }; }
    w!(UNP_GRPH_PITCH_L, GRPH_PITCH_L, s.surface_pitch); w!(UNP_GRPH_PITCH_C, GRPH_PITCH_C, s.chroma_pitch);
    w!(UNP_GRPH_X_START_L, GRPH_X_START_L, 0); w!(UNP_GRPH_X_START_C, GRPH_X_START_C, 0); w!(UNP_GRPH_Y_START_L, GRPH_Y_START_L, 0); w!(UNP_GRPH_Y_START_C, GRPH_Y_START_C, 0);
    w!(UNP_GRPH_X_END_L, GRPH_X_END_L, s.surface_size.x+s.surface_size.width); w!(UNP_GRPH_X_END_C, GRPH_X_END_C, s.chroma_size.x+s.chroma_size.width); w!(UNP_GRPH_Y_END_L, GRPH_Y_END_L, s.surface_size.y+s.surface_size.height); w!(UNP_GRPH_Y_END_C, GRPH_Y_END_C, s.chroma_size.y+s.chroma_size.height);
    value=0; let rot=match rotation { ROTATION_ANGLE_90=>3, ROTATION_ANGLE_180=>2, ROTATION_ANGLE_270=>1, _=>0 }; set_reg_field_value(value,rot,UNP_HW_ROTATION,ROTATION_ANGLE); dm_write_reg((*mi).base.ctx,mmUNP_HW_ROTATION,value);
}

unsafe fn program_pixel_format(mi: *mut dce_mem_input, format: surface_pixel_format) {
    if format < SURFACE_PIXEL_FORMAT_VIDEO_BEGIN { let mut value=dm_read_reg((*mi).base.ctx,mmUNP_GRPH_CONTROL); let (depth,fmt)=match format { SURFACE_PIXEL_FORMAT_GRPH_PALETA_256_COLORS=>(0,0), SURFACE_PIXEL_FORMAT_GRPH_RGB565=>(1,1), SURFACE_PIXEL_FORMAT_GRPH_ARGB8888|SURFACE_PIXEL_FORMAT_GRPH_ABGR8888=>(2,0), SURFACE_PIXEL_FORMAT_GRPH_ARGB2101010|SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010|SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010_XR_BIAS=>(2,1), SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616|SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616|SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F|SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616F=>(3,0), _=>(2,0) }; set_reg_field_value(value,depth,UNP_GRPH_CONTROL,GRPH_DEPTH); set_reg_field_value(value,fmt,UNP_GRPH_CONTROL,GRPH_FORMAT); dm_write_reg((*mi).base.ctx,mmUNP_GRPH_CONTROL,value); value=dm_read_reg((*mi).base.ctx,mmUNP_GRPH_CONTROL_EXP); set_reg_field_value(value,0,UNP_GRPH_CONTROL_EXP,VIDEO_FORMAT); dm_write_reg((*mi).base.ctx,mmUNP_GRPH_CONTROL_EXP,value); } else { let mut value=dm_read_reg((*mi).base.ctx,mmUNP_GRPH_CONTROL_EXP); let vf=match format { SURFACE_PIXEL_FORMAT_VIDEO_420_YCbCr=>2, SURFACE_PIXEL_FORMAT_VIDEO_420_YCrCb=>3, _=>0 }; set_reg_field_value(value,vf,UNP_GRPH_CONTROL_EXP,VIDEO_FORMAT); dm_write_reg((*mi).base.ctx,mmUNP_GRPH_CONTROL_EXP,value); }
}

unsafe fn dce_mem_input_v_is_surface_pending(mem_input: *mut mem_input) -> bool { let mi=TO_DCE_MEM_INPUT(mem_input); let value=dm_read_reg((*mi).base.ctx,mmUNP_GRPH_UPDATE); if get_reg_field_value(value,UNP_GRPH_UPDATE,GRPH_SURFACE_UPDATE_PENDING)!=0 { true } else { (*mem_input).current_address=(*mem_input).request_address; false } }
unsafe fn dce_mem_input_v_program_surface_flip_and_addr(mem_input:*mut mem_input,address:*const dc_plane_address,flip_immediate:bool)->bool { let mi=TO_DCE_MEM_INPUT(mem_input); set_flip_control(mi,flip_immediate); program_addr(mi,address); (*mem_input).request_address=*address; true }

static DVMM_HW_SETTING_2D_TILING: [[u32;9];4] = [[8,64,64,8,8,1,4,0,0],[16,64,32,8,16,1,8,0,0],[32,32,32,16,16,1,8,0,0],[64,8,32,16,16,1,8,0,0]];
static DVMM_HW_SETTING_1D_TILING: [[u32;9];4] = [[8,512,8,1,0,1,0,0,0],[16,256,8,2,0,1,0,0,0],[32,128,8,4,0,1,0,0,0],[64,64,8,4,0,1,0,0,0]];
static DVMM_HW_SETTING_LINEAR: [[u32;9];4] = [[8,4096,1,8,0,1,0,0,0],[16,2048,1,8,0,1,0,0,0],[32,1024,1,8,0,1,0,0,0],[64,512,1,8,0,1,0,0,0]];

unsafe fn get_dvmm_hw_setting(t:*mut dc_tiling_info, format:surface_pixel_format, chroma:bool)->*const u32 { let bpp=if format>=SURFACE_PIXEL_FORMAT_INVALID {2} else if format>=SURFACE_PIXEL_FORMAT_VIDEO_BEGIN {if chroma {1}else{0}} else {0}; match (*t).gfx8.array_mode { DC_ARRAY_1D_TILED_THIN1|DC_ARRAY_1D_TILED_THICK|DC_ARRAY_PRT_TILED_THIN1=>DVMM_HW_SETTING_1D_TILING[bpp].as_ptr(), DC_ARRAY_LINEAR_GENERAL|DC_ARRAY_LINEAR_ALLIGNED=>DVMM_HW_SETTING_LINEAR[bpp].as_ptr(), _=>DVMM_HW_SETTING_2D_TILING[bpp].as_ptr() } }

unsafe fn dce_mem_input_v_program_pte_vm(mem_input:*mut mem_input, format:surface_pixel_format, tiling:*mut dc_tiling_info, rotation:dc_rotation_angle) { let mi=TO_DCE_MEM_INPUT(mem_input); let pte=get_dvmm_hw_setting(tiling,format,false); let pc=get_dvmm_hw_setting(tiling,format,true); let mut pw=0; let mut ph=0; let mut x=(*pte.add(1)); while {x>>=1;x!=0} {pw+=1;} x=*pte.add(2); while {x>>=1;x!=0} {ph+=1;} let mut pwc=0;let mut phc=0;x=*pc.add(1);while{x>>=1;x!=0}{pwc+=1;}x=*pc.add(2);while{x>>=1;x!=0}{phc+=1;} let (min,minc)=if rotation==ROTATION_ANGLE_90||rotation==ROTATION_ANGLE_270 {(*pte.add(4),*pc.add(4))}else{(*pte.add(3),*pc.add(3))}; let mut v=dm_read_reg((*mi).base.ctx,mmUNP_PIPE_OUTSTANDING_REQUEST_LIMIT);set_reg_field_value(v,0xff,UNP_PIPE_OUTSTANDING_REQUEST_LIMIT,UNP_PIPE_OUTSTANDING_REQUEST_LIMIT_L);set_reg_field_value(v,0xff,UNP_PIPE_OUTSTANDING_REQUEST_LIMIT,UNP_PIPE_OUTSTANDING_REQUEST_LIMIT_C);dm_write_reg((*mi).base.ctx,mmUNP_PIPE_OUTSTANDING_REQUEST_LIMIT,v); v=dm_read_reg((*mi).base.ctx,mmUNP_DVMM_PTE_CONTROL);set_reg_field_value(v,pw,UNP_DVMM_PTE_CONTROL,DVMM_PAGE_WIDTH);set_reg_field_value(v,ph,UNP_DVMM_PTE_CONTROL,DVMM_PAGE_HEIGHT);set_reg_field_value(v,min,UNP_DVMM_PTE_CONTROL,DVMM_MIN_PTE_BEFORE_FLIP);dm_write_reg((*mi).base.ctx,mmUNP_DVMM_PTE_CONTROL,v); v=dm_read_reg((*mi).base.ctx,mmUNP_DVMM_PTE_ARB_CONTROL);set_reg_field_value(v,*pte.add(5),UNP_DVMM_PTE_ARB_CONTROL,DVMM_PTE_REQ_PER_CHUNK);set_reg_field_value(v,0xff,UNP_DVMM_PTE_ARB_CONTROL,DVMM_MAX_PTE_REQ_OUTSTANDING);dm_write_reg((*mi).base.ctx,mmUNP_DVMM_PTE_ARB_CONTROL,v); v=dm_read_reg((*mi).base.ctx,mmUNP_DVMM_PTE_CONTROL_C);set_reg_field_value(v,pwc,UNP_DVMM_PTE_CONTROL_C,DVMM_PAGE_WIDTH_C);set_reg_field_value(v,phc,UNP_DVMM_PTE_CONTROL_C,DVMM_PAGE_HEIGHT_C);set_reg_field_value(v,minc,UNP_DVMM_PTE_CONTROL_C,DVMM_MIN_PTE_BEFORE_FLIP_C);dm_write_reg((*mi).base.ctx,mmUNP_DVMM_PTE_CONTROL_C,v); v=dm_read_reg((*mi).base.ctx,mmUNP_DVMM_PTE_ARB_CONTROL_C);set_reg_field_value(v,*pc.add(5),UNP_DVMM_PTE_ARB_CONTROL_C,DVMM_PTE_REQ_PER_CHUNK_C);set_reg_field_value(v,0xff,UNP_DVMM_PTE_ARB_CONTROL_C,DVMM_MAX_PTE_REQ_OUTSTANDING_C);dm_write_reg((*mi).base.ctx,mmUNP_DVMM_PTE_ARB_CONTROL_C,v); }

unsafe fn dce_mem_input_v_program_surface_config(mi:*mut mem_input, format:surface_pixel_format, tiling:*mut dc_tiling_info, size:*mut plane_size, rotation:dc_rotation_angle, _dcc:*mut dc_plane_dcc_param, _horizontal_mirror:bool) { let d=TO_DCE_MEM_INPUT(mi);enable(d);program_tiling(d,tiling,format);program_size_and_rotation(d,rotation,size);program_pixel_format(d,format); }

unsafe fn program_urgency_watermark(ctx:*const dc_context, urgency:u32, wm:u32, marks:dce_watermarks, total:u32) { let mut m=dm_read_reg(ctx,wm);set_reg_field_value(m,1,DPGV0_WATERMARK_MASK_CONTROL,URGENCY_WATERMARK_MASK);dm_write_reg(ctx,wm,m);let mut v=dm_read_reg(ctx,urgency);set_reg_field_value(v,marks.a_mark,DPGV0_PIPE_URGENCY_CONTROL,URGENCY_LOW_WATERMARK);set_reg_field_value(v,total,DPGV0_PIPE_URGENCY_CONTROL,URGENCY_HIGH_WATERMARK);dm_write_reg(ctx,urgency,v);m=dm_read_reg(ctx,wm);set_reg_field_value(m,2,DPGV0_WATERMARK_MASK_CONTROL,URGENCY_WATERMARK_MASK);dm_write_reg(ctx,wm,m);v=dm_read_reg(ctx,urgency);set_reg_field_value(v,marks.b_mark,DPGV0_PIPE_URGENCY_CONTROL,URGENCY_LOW_WATERMARK);set_reg_field_value(v,total,DPGV0_PIPE_URGENCY_CONTROL,URGENCY_HIGH_WATERMARK);dm_write_reg(ctx,urgency,v); }
unsafe fn program_urgency_watermark_l(c:*const dc_context,m:dce_watermarks,t:u32){program_urgency_watermark(c,mmDPGV0_PIPE_URGENCY_CONTROL,mmDPGV0_WATERMARK_MASK_CONTROL,m,t)}
unsafe fn program_urgency_watermark_c(c:*const dc_context,m:dce_watermarks,t:u32){program_urgency_watermark(c,mmDPGV1_PIPE_URGENCY_CONTROL,mmDPGV1_WATERMARK_MASK_CONTROL,m,t)}

unsafe fn program_stutter_watermark(c:*const dc_context,s:u32,w:u32,m:dce_watermarks){let mut v=dm_read_reg(c,w);set_reg_field_value(v,1,DPGV0_WATERMARK_MASK_CONTROL,STUTTER_EXIT_SELF_REFRESH_WATERMARK_MASK);dm_write_reg(c,w,v);v=dm_read_reg(c,s);set_reg_field_value(v,if (*c).dc.debug.disable_stutter {0}else{1},DPGV0_PIPE_STUTTER_CONTROL,STUTTER_ENABLE);set_reg_field_value(v,1,DPGV0_PIPE_STUTTER_CONTROL,STUTTER_IGNORE_FBC);set_reg_field_value(v,m.a_mark,DPGV0_PIPE_STUTTER_CONTROL,STUTTER_EXIT_SELF_REFRESH_WATERMARK);dm_write_reg(c,s,v);v=dm_read_reg(c,w);set_reg_field_value(v,2,DPGV0_WATERMARK_MASK_CONTROL,STUTTER_EXIT_SELF_REFRESH_WATERMARK_MASK);dm_write_reg(c,w,v);v=dm_read_reg(c,s);set_reg_field_value(v,m.b_mark,DPGV0_PIPE_STUTTER_CONTROL,STUTTER_EXIT_SELF_REFRESH_WATERMARK);dm_write_reg(c,s,v)}
unsafe fn program_stutter_watermark_l(c:*const dc_context,m:dce_watermarks){program_stutter_watermark(c,mmDPGV0_PIPE_STUTTER_CONTROL,mmDPGV0_WATERMARK_MASK_CONTROL,m)}
unsafe fn program_stutter_watermark_c(c:*const dc_context,m:dce_watermarks){program_stutter_watermark(c,mmDPGV1_PIPE_STUTTER_CONTROL,mmDPGV1_WATERMARK_MASK_CONTROL,m)}

unsafe fn program_nbp_watermark(c:*const dc_context,w:u32,n:u32,m:dce_watermarks){let mut v=dm_read_reg(c,w);set_reg_field_value(v,1,DPGV0_WATERMARK_MASK_CONTROL,NB_PSTATE_CHANGE_WATERMARK_MASK);dm_write_reg(c,w,v);v=dm_read_reg(c,n);for f in [NB_PSTATE_CHANGE_ENABLE,NB_PSTATE_CHANGE_URGENT_DURING_REQUEST,NB_PSTATE_CHANGE_NOT_SELF_REFRESH_DURING_REQUEST]{set_reg_field_value(v,1,DPGV0_PIPE_NB_PSTATE_CHANGE_CONTROL,f);}dm_write_reg(c,n,v);v=dm_read_reg(c,n);set_reg_field_value(v,m.a_mark,DPGV0_PIPE_NB_PSTATE_CHANGE_CONTROL,NB_PSTATE_CHANGE_WATERMARK);dm_write_reg(c,n,v);v=dm_read_reg(c,w);set_reg_field_value(v,2,DPGV0_WATERMARK_MASK_CONTROL,NB_PSTATE_CHANGE_WATERMARK_MASK);dm_write_reg(c,w,v);v=dm_read_reg(c,n);for f in [NB_PSTATE_CHANGE_ENABLE,NB_PSTATE_CHANGE_URGENT_DURING_REQUEST,NB_PSTATE_CHANGE_NOT_SELF_REFRESH_DURING_REQUEST]{set_reg_field_value(v,1,DPGV0_PIPE_NB_PSTATE_CHANGE_CONTROL,f);}dm_write_reg(c,n,v);v=dm_read_reg(c,n);set_reg_field_value(v,m.b_mark,DPGV0_PIPE_NB_PSTATE_CHANGE_CONTROL,NB_PSTATE_CHANGE_WATERMARK);dm_write_reg(c,n,v)}
unsafe fn program_nbp_watermark_l(c:*const dc_context,m:dce_watermarks){program_nbp_watermark(c,mmDPGV0_WATERMARK_MASK_CONTROL,mmDPGV0_PIPE_NB_PSTATE_CHANGE_CONTROL,m)}
unsafe fn program_nbp_watermark_c(c:*const dc_context,m:dce_watermarks){program_nbp_watermark(c,mmDPGV1_WATERMARK_MASK_CONTROL,mmDPGV1_PIPE_NB_PSTATE_CHANGE_CONTROL,m)}

unsafe fn dce_mem_input_v_program_display_marks(mi:*mut mem_input,nbp:dce_watermarks,stutter:dce_watermarks,_stutter_enter:dce_watermarks,urgent:dce_watermarks,total:u32){program_urgency_watermark_l((*mi).ctx,urgent,total);program_nbp_watermark_l((*mi).ctx,nbp);program_stutter_watermark_l((*mi).ctx,stutter)}
unsafe fn dce_mem_input_program_chroma_display_marks(mi:*mut mem_input,nbp:dce_watermarks,stutter:dce_watermarks,urgent:dce_watermarks,total:u32){program_urgency_watermark_c((*mi).ctx,urgent,total);program_nbp_watermark_c((*mi).ctx,nbp);program_stutter_watermark_c((*mi).ctx,stutter)}
unsafe fn dce110_allocate_mem_input_v(mi:*mut mem_input,_h:u32,_v:u32,pix:u32,_n:u32){if pix!=0{let mut a=mmDPGV0_PIPE_ARBITRATION_CONTROL1;let mut v=dm_read_reg((*mi).ctx,a);set_reg_field_value(v,1000000000u32/pix,DPGV0_PIPE_ARBITRATION_CONTROL1,PIXEL_DURATION);dm_write_reg((*mi).ctx,a,v);a=mmDPGV1_PIPE_ARBITRATION_CONTROL1;v=dm_read_reg((*mi).ctx,a);set_reg_field_value(v,1000000000u32/pix,DPGV1_PIPE_ARBITRATION_CONTROL1,PIXEL_DURATION);dm_write_reg((*mi).ctx,a,v);dm_write_reg((*mi).ctx,mmDPGV0_PIPE_ARBITRATION_CONTROL2,0x4000800);dm_write_reg((*mi).ctx,mmDPGV1_PIPE_ARBITRATION_CONTROL2,0x4000800)}}
unsafe fn dce110_free_mem_input_v(_mi:*mut mem_input,_n:u32){}

static DCE110_MEM_INPUT_V_FUNCS: mem_input_funcs = mem_input_funcs { mem_input_program_display_marks:dce_mem_input_v_program_display_marks, mem_input_program_chroma_display_marks:dce_mem_input_program_chroma_display_marks, allocate_mem_input:dce110_allocate_mem_input_v, free_mem_input:dce110_free_mem_input_v, mem_input_program_surface_flip_and_addr:dce_mem_input_v_program_surface_flip_and_addr, mem_input_program_pte_vm:dce_mem_input_v_program_pte_vm, mem_input_program_surface_config:dce_mem_input_v_program_surface_config, mem_input_is_flip_pending:dce_mem_input_v_is_surface_pending };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
