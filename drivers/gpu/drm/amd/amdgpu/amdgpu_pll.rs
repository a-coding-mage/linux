/*
 * Copyright 2014 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

/* External kernel/driver declarations are supplied by the surrounding crate. */

#[inline]
unsafe fn amdgpu_pll_reduce_ratio(nom: *mut u32, den: *mut u32, nom_min: u32, den_min: u32) {
    let mut tmp: u32;

    /* reduce the numbers to a simpler ratio */
    tmp = gcd(unsafe { *nom }, unsafe { *den });
    unsafe {
        *nom /= tmp;
        *den /= tmp;
    }

    /* make sure nominator is large enough */
    if unsafe { *nom } < nom_min {
        tmp = div_round_up(nom_min, unsafe { *nom });
        unsafe {
            *nom *= tmp;
            *den *= tmp;
        }
    }

    /* make sure the denominator is large enough */
    if unsafe { *den } < den_min {
        tmp = div_round_up(den_min, unsafe { *den });
        unsafe {
            *nom *= tmp;
            *den *= tmp;
        }
    }
}

#[inline]
unsafe fn amdgpu_pll_get_fb_ref_div(
    adev: *mut amdgpu_device,
    nom: u32,
    den: u32,
    post_div: u32,
    fb_div_max: u32,
    mut ref_div_max: u32,
    fb_div: *mut u32,
    ref_div: *mut u32,
) {
    /* limit reference * post divider to a maximum */
    if unsafe { (*adev).family } == AMDGPU_FAMILY_SI {
        ref_div_max = core::cmp::min(100 / post_div, ref_div_max);
    } else {
        ref_div_max = core::cmp::min(128 / post_div, ref_div_max);
    }

    /* get matching reference and feedback divider */
    unsafe {
        *ref_div = clamp(div_round_closest(den, post_div), 1, ref_div_max);
        *fb_div = div_round_closest(nom * *ref_div * post_div, den);
    }

    /* limit fb divider to its maximum */
    if unsafe { *fb_div } > fb_div_max {
        unsafe {
            *ref_div = div_round_closest(*ref_div * fb_div_max, *fb_div);
            *fb_div = fb_div_max;
        }
    }
}

pub unsafe fn amdgpu_pll_compute(
    adev: *mut amdgpu_device,
    pll: *mut amdgpu_pll,
    freq: u32,
    dot_clock_p: *mut u32,
    fb_div_p: *mut u32,
    frac_fb_div_p: *mut u32,
    ref_div_p: *mut u32,
    post_div_p: *mut u32,
) {
    let mut target_clock = if ((*pll).flags & AMDGPU_PLL_USE_FRAC_FB_DIV) != 0 { freq } else { freq / 10 };
    let mut fb_div_min = (*pll).min_feedback_div;
    let mut fb_div_max = (*pll).max_feedback_div;
    let mut fb_div: u32;
    let (mut post_div_min, mut post_div_max, mut post_div): (u32, u32, u32);
    let (mut ref_div_min, mut ref_div_max, mut ref_div): (u32, u32, u32);
    let (mut post_div_best, mut diff_best): (u32, u32);
    let (mut nom, mut den): (u32, u32);

    if ((*pll).flags & AMDGPU_PLL_USE_FRAC_FB_DIV) != 0 {
        fb_div_min *= 10;
        fb_div_max *= 10;
    }

    ref_div_min = if ((*pll).flags & AMDGPU_PLL_USE_REF_DIV) != 0 { (*pll).reference_div } else { (*pll).min_ref_div };
    ref_div_max = if ((*pll).flags & AMDGPU_PLL_USE_FRAC_FB_DIV) != 0 && ((*pll).flags & AMDGPU_PLL_USE_REF_DIV) != 0 { (*pll).reference_div } else { (*pll).max_ref_div };

    if ((*pll).flags & AMDGPU_PLL_USE_POST_DIV) != 0 {
        post_div_min = (*pll).post_div;
        post_div_max = (*pll).post_div;
    } else {
        let (mut vco_min, mut vco_max) = if ((*pll).flags & AMDGPU_PLL_IS_LCD) != 0 { ((*pll).lcd_pll_out_min, (*pll).lcd_pll_out_max) } else { ((*pll).pll_out_min, (*pll).pll_out_max) };
        if ((*pll).flags & AMDGPU_PLL_USE_FRAC_FB_DIV) != 0 { vco_min *= 10; vco_max *= 10; }
        post_div_min = vco_min / target_clock;
        if target_clock * post_div_min < vco_min { post_div_min += 1; }
        if post_div_min < (*pll).min_post_div { post_div_min = (*pll).min_post_div; }
        post_div_max = vco_max / target_clock;
        if target_clock * post_div_max > vco_max { post_div_max -= 1; }
        if post_div_max > (*pll).max_post_div { post_div_max = (*pll).max_post_div; }
    }

    nom = target_clock;
    den = (*pll).reference_freq;
    amdgpu_pll_reduce_ratio(&mut nom, &mut den, fb_div_min, post_div_min);

    post_div_best = if ((*pll).flags & AMDGPU_PLL_PREFER_MINM_OVER_MAXP) != 0 { post_div_min } else { post_div_max };
    diff_best = u32::MAX;
    post_div = post_div_min;
    while post_div <= post_div_max {
        let diff;
        amdgpu_pll_get_fb_ref_div(adev, nom, den, post_div, fb_div_max, ref_div_max, &mut fb_div, &mut ref_div);
        diff = abs_diff(target_clock, ((*pll).reference_freq * fb_div) / (ref_div * post_div));
        if diff < diff_best || (diff == diff_best && ((*pll).flags & AMDGPU_PLL_PREFER_MINM_OVER_MAXP) == 0) {
            post_div_best = post_div;
            diff_best = diff;
        }
        post_div += 1;
    }
    post_div = post_div_best;
    amdgpu_pll_get_fb_ref_div(adev, nom, den, post_div, fb_div_max, ref_div_max, &mut fb_div, &mut ref_div);
    amdgpu_pll_reduce_ratio(&mut fb_div, &mut ref_div, fb_div_min, ref_div_min);

    if ((*pll).flags & AMDGPU_PLL_USE_FRAC_FB_DIV) != 0 && fb_div % 10 != 0 {
        fb_div_min = core::cmp::max(fb_div_min, (9 - (fb_div % 10)) * 20 + 60);
        if fb_div < fb_div_min {
            let tmp = div_round_up(fb_div_min, fb_div);
            fb_div *= tmp;
            ref_div *= tmp;
        }
    }

    if ((*pll).flags & AMDGPU_PLL_USE_FRAC_FB_DIV) != 0 {
        *fb_div_p = fb_div / 10;
        *frac_fb_div_p = fb_div % 10;
    } else {
        *fb_div_p = fb_div;
        *frac_fb_div_p = 0;
    }
    *dot_clock_p = ((*pll).reference_freq * *fb_div_p * 10 + (*pll).reference_freq * *frac_fb_div_p) / (ref_div * post_div * 10);
    *ref_div_p = ref_div;
    *post_div_p = post_div;
    drm_debug_kms("%d - %d, pll dividers - fb: %d.%d ref: %d, post %d\n", freq, *dot_clock_p * 10, *fb_div_p, *frac_fb_div_p, ref_div, post_div);
}

pub unsafe fn amdgpu_pll_get_use_mask(crtc: *mut drm_crtc) -> u32 {
    let dev = (*crtc).dev;
    let mut pll_in_use = 0;
    list_for_each_entry!(test_crtc, &(*dev).mode_config.crtc_list, head, {
        if crtc == test_crtc { continue; }
        let test_amdgpu_crtc = to_amdgpu_crtc(test_crtc);
        if (*test_amdgpu_crtc).pll_id != ATOM_PPLL_INVALID { pll_in_use |= 1 << (*test_amdgpu_crtc).pll_id; }
    });
    pll_in_use
}

pub unsafe fn amdgpu_pll_get_shared_dp_ppll(crtc: *mut drm_crtc) -> i32 {
    let dev = (*crtc).dev;
    list_for_each_entry!(test_crtc, &(*dev).mode_config.crtc_list, head, {
        if crtc == test_crtc { continue; }
        let test_amdgpu_crtc = to_amdgpu_crtc(test_crtc);
        if !(*test_amdgpu_crtc).encoder.is_null() && encoder_mode_is_dp(amdgpu_atombios_encoder_get_encoder_mode((*test_amdgpu_crtc).encoder)) && (*test_amdgpu_crtc).pll_id != ATOM_PPLL_INVALID { return (*test_amdgpu_crtc).pll_id; }
    });
    ATOM_PPLL_INVALID
}

pub unsafe fn amdgpu_pll_get_shared_nondp_ppll(crtc: *mut drm_crtc) -> i32 {
    let amdgpu_crtc = to_amdgpu_crtc(crtc);
    let dev = (*crtc).dev;
    let adjusted_clock = (*amdgpu_crtc).adjusted_clock;
    if adjusted_clock == 0 { return ATOM_PPLL_INVALID; }
    list_for_each_entry!(test_crtc, &(*dev).mode_config.crtc_list, head, {
        if crtc == test_crtc { continue; }
        let test_amdgpu_crtc = to_amdgpu_crtc(test_crtc);
        if !(*test_amdgpu_crtc).encoder.is_null() && !encoder_mode_is_dp(amdgpu_atombios_encoder_get_encoder_mode((*test_amdgpu_crtc).encoder)) {
            if (*test_amdgpu_crtc).connector == (*amdgpu_crtc).connector && (*test_amdgpu_crtc).pll_id != ATOM_PPLL_INVALID { return (*test_amdgpu_crtc).pll_id; }
            if (*crtc).mode.clock == (*test_crtc).mode.clock && adjusted_clock == (*test_amdgpu_crtc).adjusted_clock && (*amdgpu_crtc).ss_enabled == (*test_amdgpu_crtc).ss_enabled && (*test_amdgpu_crtc).pll_id != ATOM_PPLL_INVALID { return (*test_amdgpu_crtc).pll_id; }
        }
    });
    ATOM_PPLL_INVALID
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
