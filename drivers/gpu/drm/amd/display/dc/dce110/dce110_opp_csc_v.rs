/* Rust translation of dce110_opp_csc_v.c. External types, constants, and
 * register helpers are supplied by the surrounding driver. */

const OUTPUT_CSC_MATRIX_SIZE: usize = 12;
const UNDERLAY_CONTRAST_DEFAULT: i32 = 100;
const UNDERLAY_CONTRAST_MAX: i32 = 200;
const UNDERLAY_CONTRAST_MIN: i32 = 0;
const UNDERLAY_CONTRAST_STEP: i32 = 1;
const UNDERLAY_CONTRAST_DIVIDER: i32 = 100;
const UNDERLAY_SATURATION_DEFAULT: i32 = 100;
const UNDERLAY_SATURATION_MIN: i32 = 0;
const UNDERLAY_SATURATION_MAX: i32 = 200;
const UNDERLAY_SATURATION_STEP: i32 = 1;
const UNDERLAY_HUE_DEFAULT: i32 = 0;
const UNDERLAY_HUE_MIN: i32 = -300;
const UNDERLAY_HUE_MAX: i32 = 300;
const UNDERLAY_HUE_STEP: i32 = 5;
const UNDERLAY_HUE_DIVIDER: i32 = 10;
const UNDERLAY_SATURATION_DIVIDER: i32 = 100;
const UNDERLAY_BRIGHTNESS_DEFAULT: i32 = 0;
const UNDERLAY_BRIGHTNESS_MIN: i32 = -46;
const UNDERLAY_BRIGHTNESS_MAX: i32 = 46;
const UNDERLAY_BRIGHTNESS_STEP: i32 = 1;
const UNDERLAY_BRIGHTNESS_DIVIDER: i32 = 100;

#[repr(C)]
struct InputCscMatrix { color_space: dc_color_space, regval: [u32; 12] }

static GLOBAL_COLOR_MATRIX: &[(dc_color_space, [u32; 12])] = &[
    (COLOR_SPACE_SRGB, [0x2000,0,0,0,0,0x2000,0,0,0,0,0x2000,0]),
    (COLOR_SPACE_SRGB_LIMITED, [0x1b60,0,0,0x200,0,0x1b60,0,0x200,0,0,0x1b60,0x200]),
    (COLOR_SPACE_YCBCR601, [0xe00,0xf447,0xfdb9,0x1000,0x82f,0x1012,0x31f,0x200,0xfb47,0xf6b9,0xe00,0x1000]),
    (COLOR_SPACE_YCBCR709, [0xe00,0xf349,0xfeb7,0x1000,0x5d2,0x1394,0x1fa,0x200,0xfccb,0xf535,0xe00,0x1000]),
    (COLOR_SPACE_YCBCR601_LIMITED, [0xe00,0xf447,0xfdb9,0x1000,0x991,0x12c9,0x3a6,0x200,0xfb47,0xf6b9,0xe00,0x1000]),
    (COLOR_SPACE_YCBCR709_LIMITED, [0xe00,0xf349,0xfeb7,0x1000,0x6ce,0x16e3,0x24f,0x200,0xfccb,0xf535,0xe00,0x1000]),
    (COLOR_SPACE_2020_RGB_FULLRANGE, [0x2000,0,0,0,0,0x2000,0,0,0,0,0x2000,0]),
    (COLOR_SPACE_2020_RGB_LIMITEDRANGE, [0x1b67,0,0,0x201,0,0x1b67,0,0x201,0,0,0x1b67,0x201]),
    (COLOR_SPACE_2020_YCBCR_LIMITED, [0x0e04,0xf31d,0xfedf,0x1004,0x733,0x1294,0x1a0,0x201,0xfc16,0xf5e6,0x0e04,0x1004]),
    (COLOR_SPACE_2020_YCBCR_FULL, [0x1000,0xf149,0xfeb7,0x1004,0x868,0x15b2,0x1e6,0,0xfb88,0xf478,0x1000,0x1004]),
];

#[repr(C)]
struct InputCscMatrixTable { color_space: dc_color_space, regval: [u32; 12] }
static INPUT_CSC_MATRIX: &[InputCscMatrixTable] = &[
    InputCscMatrixTable { color_space: COLOR_SPACE_SRGB, regval: [0x2000,0,0,0,0,0x2000,0,0,0,0,0x2000,0] },
    InputCscMatrixTable { color_space: COLOR_SPACE_SRGB_LIMITED, regval: [0x2000,0,0,0,0,0x2000,0,0,0,0,0x2000,0] },
    InputCscMatrixTable { color_space: COLOR_SPACE_YCBCR601, regval: [0x2cdd,0x2000,0,0xe991,0xe926,0x2000,0xf4fd,0x10ef,0,0x2000,0x38b4,0xe3a6] },
    InputCscMatrixTable { color_space: COLOR_SPACE_YCBCR601_LIMITED, regval: [0x3353,0x2568,0,0xe400,0xe5dc,0x2568,0xf367,0x1108,0,0x2568,0x40de,0xdd3a] },
    InputCscMatrixTable { color_space: COLOR_SPACE_YCBCR709, regval: [0x3265,0x2000,0,0xe6ce,0xf105,0x2000,0xfa01,0xa7d,0,0x2000,0x3b61,0xe24f] },
    InputCscMatrixTable { color_space: COLOR_SPACE_YCBCR709_LIMITED, regval: [0x39a6,0x2568,0,0xe0d6,0xeedd,0x2568,0xf925,0x9a8,0,0x2568,0x43ee,0xdbb2] },
];

#[repr(C)]
struct OutCscColorMatrix { color_space: dc_color_space, regval: [u32; 12] }

unsafe fn program_color_matrix_v(xfm: *mut dce_transform, tbl: *const OutCscColorMatrix, _options: grph_color_adjust_option) {
    let ctx = (*(*xfm).base.ctx);
    let mut control = dm_read_reg(&ctx, mmCOL_MAN_OUTPUT_CSC_CONTROL);
    let use_a = get_reg_field_value(control, COL_MAN_OUTPUT_CSC_CONTROL, OUTPUT_CSC_MODE) != 4;
    set_reg_field_value(control, 0, COL_MAN_OUTPUT_CSC_CONTROL, OUTPUT_CSC_MODE);
    let (regs, fields) = if use_a { ([mmOUTPUT_CSC_C11_C12_A,mmOUTPUT_CSC_C13_C14_A,mmOUTPUT_CSC_C21_C22_A,mmOUTPUT_CSC_C23_C24_A,mmOUTPUT_CSC_C31_C32_A,mmOUTPUT_CSC_C33_C34_A], [OUTPUT_CSC_C11_A,OUTPUT_CSC_C13_A,OUTPUT_CSC_C21_A,OUTPUT_CSC_C23_A,OUTPUT_CSC_C31_A,OUTPUT_CSC_C33_A]) } else { ([mmOUTPUT_CSC_C11_C12_B,mmOUTPUT_CSC_C13_C14_B,mmOUTPUT_CSC_C21_C22_B,mmOUTPUT_CSC_C23_C24_B,mmOUTPUT_CSC_C31_C32_B,mmOUTPUT_CSC_C33_C34_B], [OUTPUT_CSC_C11_B,OUTPUT_CSC_C13_B,OUTPUT_CSC_C21_B,OUTPUT_CSC_C23_B,OUTPUT_CSC_C31_B,OUTPUT_CSC_C33_B]) };
    let pair_fields = if use_a { [OUTPUT_CSC_C12_A,OUTPUT_CSC_C14_A,OUTPUT_CSC_C22_A,OUTPUT_CSC_C24_A,OUTPUT_CSC_C32_A,OUTPUT_CSC_C34_A] } else { [OUTPUT_CSC_C12_B,OUTPUT_CSC_C14_B,OUTPUT_CSC_C22_B,OUTPUT_CSC_C24_B,OUTPUT_CSC_C32_B,OUTPUT_CSC_C34_B] };
    let pair_regs = if use_a { regs } else { regs };
    for i in 0..6 { let mut value = 0; set_reg_field_value(value, (*tbl).regval[i*2], pair_regs[i], fields[i]); set_reg_field_value(value, (*tbl).regval[i*2+1], pair_regs[i], pair_fields[i]); dm_write_reg(&ctx, pair_regs[i], value); }
    set_reg_field_value(control, if use_a { 4 } else { 5 }, COL_MAN_OUTPUT_CSC_CONTROL, OUTPUT_CSC_MODE);
    dm_write_reg(&ctx, mmCOL_MAN_OUTPUT_CSC_CONTROL, control);
}

unsafe fn configure_graphics_mode_v(xfm: *mut dce_transform, config: csc_color_mode, adjust: graphics_csc_adjust_type, space: dc_color_space) -> bool {
    let ctx = &*(*xfm).base.ctx; let mut value = dm_read_reg(ctx, mmCOL_MAN_OUTPUT_CSC_CONTROL);
    set_reg_field_value(value, 0, COL_MAN_OUTPUT_CSC_CONTROL, OUTPUT_CSC_MODE);
    if adjust == GRAPHICS_CSC_ADJUST_TYPE_SW && config == CSC_COLOR_MODE_GRAPHICS_OUTPUT_CSC { return true; }
    let mode = match space { COLOR_SPACE_SRGB => 0, COLOR_SPACE_YCBCR601 | COLOR_SPACE_YCBCR601_LIMITED => 2, COLOR_SPACE_YCBCR709 | COLOR_SPACE_YCBCR709_LIMITED => 3, COLOR_SPACE_SRGB_LIMITED => return false, _ => return false };
    if adjust == GRAPHICS_CSC_ADJUST_TYPE_SW && space == COLOR_SPACE_YCBCR601 { return false; }
    set_reg_field_value(value, mode, COL_MAN_OUTPUT_CSC_CONTROL, OUTPUT_CSC_MODE); dm_write_reg(ctx, mmCOL_MAN_OUTPUT_CSC_CONTROL, value); true
}

unsafe fn set_Denormalization(xfm: *mut transform, depth: dc_color_depth) { let ctx=&*(*xfm).ctx; let mut value=dm_read_reg(ctx,mmDENORM_CLAMP_CONTROL); let mode=match depth { COLOR_DEPTH_888=>1, COLOR_DEPTH_101010=>2, COLOR_DEPTH_121212=>3, _=>0 }; if mode != 0 { set_reg_field_value(value,mode,DENORM_CLAMP_CONTROL,DENORM_MODE); } set_reg_field_value(value,1,DENORM_CLAMP_CONTROL,DENORM_10BIT_OUT); dm_write_reg(ctx,mmDENORM_CLAMP_CONTROL,value); }

#[derive(Copy, Clone, PartialEq)] enum csc_color_mode { CSC_COLOR_MODE_GRAPHICS_BYPASS, CSC_COLOR_MODE_GRAPHICS_PREDEFINED, CSC_COLOR_MODE_GRAPHICS_OUTPUT_CSC }
#[derive(Copy, Clone)] enum grph_color_adjust_option { GRPH_COLOR_MATRIX_HW_DEFAULT=1, GRPH_COLOR_MATRIX_SW }

unsafe fn program_input_csc(xfm: *mut transform, space: dc_color_space) { let ctx=&*(*xfm).ctx; let entry=match INPUT_CSC_MATRIX.iter().find(|x|x.color_space==space) { Some(x)=>x, None=>{ BREAK_TO_DEBUGGER!(); return; } }; let mut value=dm_read_reg(ctx,mmCOL_MAN_INPUT_CSC_CONTROL); let use_a=get_reg_field_value(value,COL_MAN_INPUT_CSC_CONTROL,INPUT_CSC_MODE)!=1; let regs_a=[mmINPUT_CSC_C11_C12_A,mmINPUT_CSC_C13_C14_A,mmINPUT_CSC_C21_C22_A,mmINPUT_CSC_C23_C24_A,mmINPUT_CSC_C31_C32_A,mmINPUT_CSC_C33_C34_A]; let regs_b=[mmINPUT_CSC_C11_C12_B,mmINPUT_C13_C14_B,mmINPUT_C21_C22_B,mmINPUT_CSC_C23_C24_B,mmINPUT_CSC_C31_C32_B,mmINPUT_CSC_C33_C34_B]; let regs=if use_a {regs_a} else {regs_b}; for i in 0..6 { let mut v=0; set_reg_field_value(v,entry.regval[i*2],regs[i],0); set_reg_field_value(v,entry.regval[i*2+1],regs[i],1); dm_write_reg(ctx,regs[i],v); } value=0; set_reg_field_value(value,2,COL_MAN_INPUT_CSC_CONTROL,INPUT_CSC_INPUT_TYPE); set_reg_field_value(value,if use_a{1}else{2},COL_MAN_INPUT_CSC_CONTROL,INPUT_CSC_MODE); dm_write_reg(ctx,mmCOL_MAN_INPUT_CSC_CONTROL,value); }

pub unsafe fn dce110_opp_v_set_csc_default(xfm:*mut transform, adj:*const default_adjustment) { let dce=TO_DCE_TRANSFORM!(xfm); let mut config=csc_color_mode::CSC_COLOR_MODE_GRAPHICS_PREDEFINED; if !(*adj).force_hw_default { for (space, matrix) in GLOBAL_COLOR_MATRIX { if *space==(*adj).out_color_space { let e=OutCscColorMatrix{color_space:*space,regval:*matrix}; program_color_matrix_v(dce,&e,grph_color_adjust_option::GRPH_COLOR_MATRIX_SW); config=csc_color_mode::CSC_COLOR_MODE_GRAPHICS_OUTPUT_CSC; break; } } } program_input_csc(xfm,(*adj).in_color_space); configure_graphics_mode_v(dce,config,(*adj).csc_adjust_type,(*adj).out_color_space); set_Denormalization(xfm,(*adj).color_depth); }

pub unsafe fn dce110_opp_v_set_csc_adjustment(xfm:*mut transform, tbl:*const OutCscColorMatrix) { let dce=TO_DCE_TRANSFORM!(xfm); program_color_matrix_v(dce,tbl,grph_color_adjust_option::GRPH_COLOR_MATRIX_SW); configure_graphics_mode_v(dce,csc_color_mode::CSC_COLOR_MODE_GRAPHICS_OUTPUT_CSC,GRAPHICS_CSC_ADJUST_TYPE_SW,(*tbl).color_space); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
