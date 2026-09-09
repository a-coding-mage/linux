/* SPDX-License-Identifier: MIT */
/* Faithful low-level translation of display_mode_util.c.  Types and external
 * declarations are supplied by display_mode_util's Rust translation unit. */

unsafe fn _log(mut input: f32) -> f32 {
    let mut bits = input.to_bits() as i32;
    let log_2 = ((bits >> 23) & 255) - 128;
    bits &= !(255 << 23);
    bits += 127 << 23;
    input = f32::from_bits(bits as u32);
    input = ((-1.0f32 / 3.0) * input + 2.0) * input - 2.0 / 3.0;
    input + log_2 as f32
}

#[inline]
unsafe fn dcn_bw_pow(a: f32, exp: f32) -> f32 {
    if exp as i32 == 0 { return 1.0; }
    let temp = dcn_bw_pow(a, exp / 2.0);
    if exp as i32 % 2 == 0 { temp * temp }
    else if exp as i32 > 0 { a * temp * temp }
    else { (temp * temp) / a }
}

#[inline] unsafe fn dcn_bw_ceil2(arg: f32, significance: f32) -> f32 {
    ASSERT!(significance != 0.0);
    (arg / significance + 0.99999) as i32 as f32 * significance
}
#[inline] unsafe fn dcn_bw_floor2(arg: f32, significance: f32) -> f32 {
    ASSERT!(significance != 0.0);
    (arg / significance) as i32 as f32 * significance
}

pub unsafe fn dml_util_is_420(source_format: enum_dml_source_format_class) -> dml_bool_t {
    match source_format {
        dml_444_16 | dml_444_32 | dml_444_64 | dml_422_8 | dml_422_10 => 0,
        dml_420_8 | dml_420_10 => 1,
        _ => { ASSERT!(false); 0 }
    }
}
pub unsafe fn dml_ceil(x: dml_float_t, granularity: dml_float_t) -> dml_float_t { if granularity == 0.0 { 0.0 } else { dcn_bw_ceil2(x as f32, granularity as f32) as dml_float_t } }
pub unsafe fn dml_floor(x: dml_float_t, granularity: dml_float_t) -> dml_float_t { if granularity == 0.0 { 0.0 } else { dcn_bw_floor2(x as f32, granularity as f32) as dml_float_t } }
pub fn dml_min(x: dml_float_t, y: dml_float_t) -> dml_float_t { if x.is_nan() { y } else if y.is_nan() || x < y { x } else { y } }
pub fn dml_min3(x: dml_float_t,y: dml_float_t,z: dml_float_t)->dml_float_t { dml_min(dml_min(x,y),z) }
pub fn dml_min4(x: dml_float_t,y: dml_float_t,z: dml_float_t,w: dml_float_t)->dml_float_t { dml_min(dml_min(dml_min(x,y),z),w) }
pub fn dml_max(x: dml_float_t,y: dml_float_t)->dml_float_t { if x.is_nan() { y } else if y.is_nan() || x > y { x } else { y } }
pub fn dml_max3(x:dml_float_t,y:dml_float_t,z:dml_float_t)->dml_float_t { dml_max(dml_max(x,y),z) }
pub fn dml_max4(a:dml_float_t,b:dml_float_t,c:dml_float_t,d:dml_float_t)->dml_float_t { dml_max(dml_max(a,b),dml_max(c,d)) }
pub fn dml_max5(a:dml_float_t,b:dml_float_t,c:dml_float_t,d:dml_float_t,e:dml_float_t)->dml_float_t { dml_max(dml_max4(a,b,c,d),e) }
pub unsafe fn dml_log(x:dml_float_t,base:dml_float_t)->dml_float_t { (_log(x as f32)/_log(base as f32)) as dml_float_t }
pub unsafe fn dml_log2(x:dml_float_t)->dml_float_t { (_log(x as f32)/_log(2.0)) as dml_float_t }
pub unsafe fn dml_round(val:dml_float_t,_bankers_rounding:dml_bool_t)->dml_float_t { let ceil=dml_ceil(val,1.0); let floor=dml_floor(val,1.0); if val-floor>=0.5 {ceil} else {floor} }
pub unsafe fn dml_pow(base:dml_float_t,exp:i32)->dml_float_t { dcn_bw_pow(base as f32,exp as f32) as dml_float_t }
pub fn dml_round_to_multiple(num:dml_uint_t,multiple:dml_uint_t,up:dml_bool_t)->dml_uint_t { if multiple==0{return num} let r=num%multiple; if r==0{return num} if up!=0 {num+multiple-r} else {num-r} }

pub unsafe fn dml_is_vertical_rotation(scan: enum_dml_rotation_angle)->dml_bool_t { if scan==dml_rotation_90||scan==dml_rotation_90m||scan==dml_rotation_270||scan==dml_rotation_270m {1} else {0} }
pub fn dml_get_cursor_bit_per_pixel(ebpp: enum_dml_cursor_bpp)->dml_uint_t { match ebpp { dml_cur_2bit=>2,dml_cur_32bit=>32,dml_cur_64bit=>64,_=>0 } }

pub unsafe fn dml_get_num_active_planes(display_cfg:*const dml_display_cfg_st)->dml_uint_t { let mut n=0; for k in 0..__DML_NUM_PLANES__ { if (*display_cfg).plane.ViewportWidth[k]>0 {n+=1;} } n }
pub unsafe fn dml_get_num_active_pipes(display_cfg:*const dml_display_cfg_st)->dml_uint_t { let mut n=0; for j in 0..dml_get_num_active_planes(display_cfg) {n+=(*display_cfg).hw.DPPPerSurface[j];} n }
pub unsafe fn dml_get_plane_idx(mode_lib:*const display_mode_lib_st,pipe_idx:dml_uint_t)->dml_uint_t { (*mode_lib).mp.pipe_plane[pipe_idx] }
pub unsafe fn dml_get_pipe_idx(mode_lib:*const display_mode_lib_st,plane_idx:dml_uint_t)->dml_uint_t { ASSERT!(plane_idx<__DML_NUM_PLANES__); for i in 0..__DML_NUM_PLANES__ {if plane_idx==(*mode_lib).mp.pipe_plane[i]{return i;}} ASSERT!(false); 0 }
pub unsafe fn dml_calc_pipe_plane_mapping(hw:*const dml_hw_resource_st,pipe_plane:*mut dml_uint_t) { let mut p=0; for k in 0..__DML_NUM_PLANES__ {*pipe_plane.add(k)=__DML_PIPE_NO_PLANE__;} for plane in 0..__DML_NUM_PLANES__ {for _ in 0..(*hw).DPPPerSurface[plane] {*pipe_plane.add(p)=plane;p+=1;}} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
