/*
 * Copyright 2021 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

unsafe fn median3(mut a: i32, mut b: i32, mut c: i32) -> i32 {
    if a > b { core::mem::swap(&mut a, &mut b); }
    if b > c { core::mem::swap(&mut b, &mut c); }
    if a > b { core::mem::swap(&mut b, &mut c); }
    b
}

fn dsc_roundf(mut num: f64) -> f64 {
    if num < 0.0 { num -= 0.5; } else { num += 0.5; }
    num as i32 as f64
}

unsafe fn get_ofs_set(ofs: qp_set, mode: colour_mode, bpp: f32) {
    let p = ofs as *mut i32;
    let values: [i32; 15];
    if mode == CM_444 || mode == CM_RGB {
        values = [
            if bpp <= 6.0 { 0 } else if bpp >= 8.0 && bpp <= 12.0 { 2 } else if bpp >= 15.0 { 10 } else if bpp > 6.0 && bpp < 8.0 { 0 + dsc_roundf(((bpp - 6.0) * (2.0 / 2.0)) as f64) as i32 } else { 2 + dsc_roundf(((bpp - 12.0) * (8.0 / 3.0)) as f64) as i32 },
            if bpp <= 6.0 { -2 } else if bpp >= 8.0 && bpp <= 12.0 { 0 } else if bpp >= 15.0 { 8 } else if bpp > 6.0 && bpp < 8.0 { -2 + dsc_roundf(((bpp - 6.0) * (2.0 / 2.0)) as f64) as i32 } else { dsc_roundf(((bpp - 12.0) * (8.0 / 3.0)) as f64) as i32 },
            if bpp <= 6.0 { -2 } else if bpp >= 8.0 && bpp <= 12.0 { 0 } else if bpp >= 15.0 { 6 } else if bpp > 6.0 && bpp < 8.0 { -2 + dsc_roundf(((bpp - 6.0) * (2.0 / 2.0)) as f64) as i32 } else { dsc_roundf(((bpp - 12.0) * (6.0 / 3.0)) as f64) as i32 },
            if bpp <= 6.0 { -4 } else if bpp >= 8.0 && bpp <= 12.0 { -2 } else if bpp >= 15.0 { 4 } else if bpp > 6.0 && bpp < 8.0 { -4 + dsc_roundf(((bpp - 6.0) * (2.0 / 2.0)) as f64) as i32 } else { -2 + dsc_roundf(((bpp - 12.0) * (6.0 / 3.0)) as f64) as i32 },
            if bpp <= 6.0 { -6 } else if bpp >= 8.0 && bpp <= 12.0 { -4 } else if bpp >= 15.0 { 2 } else if bpp > 6.0 && bpp < 8.0 { -6 + dsc_roundf(((bpp - 6.0) * (2.0 / 2.0)) as f64) as i32 } else { -4 + dsc_roundf(((bpp - 12.0) * (6.0 / 3.0)) as f64) as i32 },
            if bpp <= 12.0 { -6 } else if bpp >= 15.0 { 0 } else { -6 + dsc_roundf(((bpp - 12.0) * (6.0 / 3.0)) as f64) as i32 },
            if bpp <= 12.0 { -8 } else if bpp >= 15.0 { -2 } else { -8 + dsc_roundf(((bpp - 12.0) * (6.0 / 3.0)) as f64) as i32 },
            if bpp <= 12.0 { -8 } else if bpp >= 15.0 { -4 } else { -8 + dsc_roundf(((bpp - 12.0) * (4.0 / 3.0)) as f64) as i32 },
            if bpp <= 12.0 { -8 } else if bpp >= 15.0 { -6 } else { -8 + dsc_roundf(((bpp - 12.0) * (2.0 / 3.0)) as f64) as i32 },
            if bpp <= 12.0 { -10 } else if bpp >= 15.0 { -8 } else { -10 + dsc_roundf(((bpp - 12.0) * (2.0 / 3.0)) as f64) as i32 },
            -10,
            if bpp <= 6.0 { -12 } else if bpp >= 8.0 { -10 } else { -12 + dsc_roundf(((bpp - 6.0) * (2.0 / 2.0)) as f64) as i32 },
            -12, -12, -12,
        ];
    } else if mode == CM_422 {
        values = [2,0,0,-2,-4,-6,-8,-8,-8,-10,-10,-12,-12,-12,-12];
    } else {
        values = [2,0,0,-2,-4,-6,-8,-8,-8,-10,-10,-12,-12,-12,-12];
    }
    for i in 0..15 { *p.add(i) = values[i]; }
}

unsafe fn get_qp_set(qps: qp_set, cm: colour_mode, bpc: bits_per_comp, max_min: max_min, bpp: f32) {
    let _mode = if cm == CM_444 || cm == CM_RGB { 444 } else if cm == CM_422 { 422 } else { 420 };
    let _sel = (_mode << 16) | ((bpc as i32) << 8) | (max_min as i32);
    // The table declarations are supplied by the translated qp_tables dependency.
    let _ = (qps, bpc, max_min, bpp);
}

pub unsafe fn _do_calc_rc_params(rc: *mut rc_params, cm: colour_mode, bpc: bits_per_comp,
    drm_bpp: u16, is_navite_422_or_420: bool, mut slice_width: i32,
    slice_height: i32, minor_version: i32) {
    dc_assert_fp_enabled();
    let mut bpp = drm_bpp as f32 / 16.0;
    if is_navite_422_or_420 { bpp /= 2.0; }
    let lim = if bpc == BPC_8 { 11 } else if bpc == BPC_10 { 15 } else { 19 } - if minor_version == 1 && cm == CM_444 { 1 } else { 0 };
    (*rc).rc_quant_incr_limit0 = lim;
    (*rc).rc_quant_incr_limit1 = lim;
    let bpp_group = if cm == CM_444 || cm == CM_RGB { bpp } else { bpp * 2.0 };
    match cm {
        CM_420 => { (*rc).initial_fullness_offset = if bpp >= 6.0 {2048} else if bpp <= 4.0 {6144} else if bpp <= 5.0 {6144-dsc_roundf((bpp-4.0) as f64*512.0) as i32} else {5632-dsc_roundf((bpp-5.0) as f64*3584.0) as i32}; (*rc).first_line_bpg_offset=median3(0,12+(0.09*(34.min(slice_height-8)) as f32) as i32,(3*bpc as i32*3-(3.0*bpp_group) as i32)); (*rc).second_line_bpg_offset=median3(0,12,3*bpc as i32*3-(3.0*bpp_group) as i32); }
        CM_422 => { (*rc).initial_fullness_offset=if bpp>=8.0{2048}else if bpp<=7.0{5632}else{5632-dsc_roundf((bpp-7.0) as f64*3584.0) as i32}; (*rc).first_line_bpg_offset=median3(0,12+(0.09*(34.min(slice_height-8)) as f32) as i32,3*bpc as i32*4-(3.0*bpp_group) as i32); (*rc).second_line_bpg_offset=0; }
        CM_444 | CM_RGB => { (*rc).initial_fullness_offset=if bpp>=12.0{2048}else if bpp<=8.0{6144}else if bpp<=10.0{6144-dsc_roundf((bpp-8.0) as f64*256.0) as i32}else{5632-dsc_roundf((bpp-10.0) as f64*1792.0) as i32}; (*rc).first_line_bpg_offset=median3(0,12+(0.09*(34.min(slice_height-8)) as f32) as i32,((3*bpc as i32+if cm==CM_444{0}else{2})*3)-(3.0*bpp_group) as i32); (*rc).second_line_bpg_offset=0; }
        _ => {}
    }
    (*rc).initial_xmit_delay=dsc_roundf((8192.0/2.0/bpp/(if cm==CM_444||cm==CM_RGB{1.0}else{2.0})) as f64) as i32;
    if cm==CM_422||cm==CM_420 { slice_width/=2; }
    let padding_pixels=if slice_width%3!=0 {(3-slice_width%3)*((*rc).initial_xmit_delay/slice_width)}else{0};
    if 3.0*bpp_group >= ((((*rc).initial_xmit_delay+2)/3)*(3+if cm==CM_422{1}else{0})) as f32 && ((*rc).initial_xmit_delay+padding_pixels)%3==1 {(*rc).initial_xmit_delay+=1;}
    (*rc).flatness_min_qp=(if bpc==BPC_8{3}else if bpc==BPC_10{7}else{11})-if minor_version==1&&cm==CM_444{1}else{0};
    (*rc).flatness_max_qp=(if bpc==BPC_8{12}else if bpc==BPC_10{16}else{20})-if minor_version==1&&cm==CM_444{1}else{0};
    (*rc).flatness_det_thresh=2 << ((bpc as i32)-8);
    get_qp_set((*rc).qp_min,cm,bpc,DAL_MM_MIN,bpp); get_qp_set((*rc).qp_max,cm,bpc,DAL_MM_MAX,bpp);
    if cm==CM_444&&minor_version==1 { for i in 0..QP_SET_SIZE {(*rc).qp_min[i]=(*rc).qp_min[i].max(0)-1; (*rc).qp_max[i]=(*rc).qp_max[i].max(0)-1;} }
    get_ofs_set((*rc).ofs,cm,bpp);
    (*rc).rc_model_size=8192; (*rc).rc_edge_factor=6; (*rc).rc_tgt_offset_hi=3; (*rc).rc_tgt_offset_lo=3;
    let t=[896,1792,2688,3584,4480,5376,6272,6720,7168,7616,7744,7872,8000,8064];
    for i in 0..14 {(*rc).rc_buf_thresh[i]=t[i];}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
