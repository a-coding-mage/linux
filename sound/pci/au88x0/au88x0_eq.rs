// SPDX-License-Identifier: GPL-2.0-or-later
/***************************************************************************
 *            au88x0_eq.c
 *  Aureal Vortex Hardware EQ control/access.
 *
 *  Sun Jun  8 18:19:19 2003
 *  2003  Manuel Jander (mjander@users.sourceforge.net)
 *
 *  02 July 2003: First time something works :)
 *  November 2003: A3D Bypass code completed but untested.
 *
 *  TODO:
 *     - Debug (testing)
 *     - Test peak visualization support.
 *
 ****************************************************************************/

/*
 */

/*
 The Aureal Hardware EQ is found on AU8810 and AU8830 chips only.
 it has 4 inputs (2 for general mix, 2 for A3D) and 2 outputs (supposed
 to be routed to the codec).
*/

/* Depends on declarations from au88x0.h, au88x0_eq.h, and au88x0_eqdata.c. */

const VORTEX_EQ_BASE: u32 = 0x2b000;
const VORTEX_EQ_DEST: u32 = VORTEX_EQ_BASE + 0x410;
const VORTEX_EQ_SOURCE: u32 = VORTEX_EQ_BASE + 0x430;
const VORTEX_EQ_CTRL: u32 = VORTEX_EQ_BASE + 0x440;

const VORTEX_BAND_COEFF_SIZE: i32 = 0x30;

extern "C" {
    static asEqCoefsZeros: *const u16;
    static asEqCoefsPipes: *const u16;
    static asEqOutStateZeros: *const u16;
    static eq_states_zero: *const u16;
    static eq_gains_zero: *const u16;
    static eq_gains_current: *const u16;
    static eq_gains_normal: *const u16;
    static eq_levels: *const u16;
    static asEqCoefsNormal: auxxEqCoeffSet_t;

    fn hwwrite(mmio: *mut core::ffi::c_void, reg: u32, value: u32);
    fn hwread(mmio: *mut core::ffi::c_void, reg: u32) -> u32;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn memset(dst: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut vortex_t;
    fn snd_ctl_new1(n: *const snd_kcontrol_new, private_data: *mut core::ffi::c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> i32;
    fn snprintf(s: *mut i8, n: usize, format: *const i8, ...) -> i32;
    fn dev_err(dev: *mut device, format: *const i8, ...);
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32;
}

/* CEqHw.s */
unsafe fn vortex_EqHw_SetTimeConsts(vortex: *mut vortex_t, gain: u16, level: u16) {
    hwwrite((*vortex).mmio, 0x2b3c4, gain as u32);
    hwwrite((*vortex).mmio, 0x2b3c8, level as u32);
}

#[inline]
fn sign_invert(a: u16) -> u16 {
    /* -(-32768) -> -32768 so we do -(-32768) -> 32767 to make the result positive */
    if a == 0x8000 {
        32767
    } else {
        a.wrapping_neg()
    }
}

unsafe fn vortex_EqHw_SetLeftCoefs(vortex: *mut vortex_t, coefs: *const u16) {
    let eqhw: *mut eqhw_t = &mut (*vortex).eq.this04;
    let mut i: i32 = 0;
    let mut n: i32;

    n = 0;
    while n < (*eqhw).this04 {
        hwwrite((*vortex).mmio, (0x2b000 + n * 0x30) as u32, *coefs.add((i + 0) as usize) as u32);
        hwwrite((*vortex).mmio, (0x2b004 + n * 0x30) as u32, *coefs.add((i + 1) as usize) as u32);

        if (*eqhw).this08 == 0 {
            hwwrite((*vortex).mmio, (0x2b008 + n * 0x30) as u32, *coefs.add((i + 2) as usize) as u32);
            hwwrite((*vortex).mmio, (0x2b00c + n * 0x30) as u32, *coefs.add((i + 3) as usize) as u32);
            hwwrite((*vortex).mmio, (0x2b010 + n * 0x30) as u32, *coefs.add((i + 4) as usize) as u32);
        } else {
            hwwrite((*vortex).mmio, (0x2b008 + n * 0x30) as u32, sign_invert(*coefs.add((2 + i) as usize)) as u32);
            hwwrite((*vortex).mmio, (0x2b00c + n * 0x30) as u32, sign_invert(*coefs.add((3 + i) as usize)) as u32);
            hwwrite((*vortex).mmio, (0x2b010 + n * 0x30) as u32, sign_invert(*coefs.add((4 + i) as usize)) as u32);
        }
        i += 5;
        n += 1;
    }
}

unsafe fn vortex_EqHw_SetRightCoefs(vortex: *mut vortex_t, coefs: *const u16) {
    let eqhw: *mut eqhw_t = &mut (*vortex).eq.this04;
    let mut i: i32 = 0;
    let mut n: i32;

    n = 0;
    while n < (*eqhw).this04 {
        hwwrite((*vortex).mmio, (0x2b1e0 + n * 0x30) as u32, *coefs.add((0 + i) as usize) as u32);
        hwwrite((*vortex).mmio, (0x2b1e4 + n * 0x30) as u32, *coefs.add((1 + i) as usize) as u32);

        if (*eqhw).this08 == 0 {
            hwwrite((*vortex).mmio, (0x2b1e8 + n * 0x30) as u32, *coefs.add((2 + i) as usize) as u32);
            hwwrite((*vortex).mmio, (0x2b1ec + n * 0x30) as u32, *coefs.add((3 + i) as usize) as u32);
            hwwrite((*vortex).mmio, (0x2b1f0 + n * 0x30) as u32, *coefs.add((4 + i) as usize) as u32);
        } else {
            hwwrite((*vortex).mmio, (0x2b1e8 + n * 0x30) as u32, sign_invert(*coefs.add((2 + i) as usize)) as u32);
            hwwrite((*vortex).mmio, (0x2b1ec + n * 0x30) as u32, sign_invert(*coefs.add((3 + i) as usize)) as u32);
            hwwrite((*vortex).mmio, (0x2b1f0 + n * 0x30) as u32, sign_invert(*coefs.add((4 + i) as usize)) as u32);
        }
        i += 5;
        n += 1;
    }
}

unsafe fn vortex_EqHw_SetLeftStates(vortex: *mut vortex_t, a: *const u16, b: *const u16) {
    let eqhw: *mut eqhw_t = &mut (*vortex).eq.this04;
    let mut i: i32 = 0;
    let mut ebx: i32;

    hwwrite((*vortex).mmio, 0x2b3fc, *a.add(0) as u32);
    hwwrite((*vortex).mmio, 0x2b400, *a.add(1) as u32);

    ebx = 0;
    while ebx < (*eqhw).this04 {
        hwwrite((*vortex).mmio, (0x2b014 + (i * 0xc)) as u32, *b.add(i as usize) as u32);
        hwwrite((*vortex).mmio, (0x2b018 + (i * 0xc)) as u32, *b.add((1 + i) as usize) as u32);
        hwwrite((*vortex).mmio, (0x2b01c + (i * 0xc)) as u32, *b.add((2 + i) as usize) as u32);
        hwwrite((*vortex).mmio, (0x2b020 + (i * 0xc)) as u32, *b.add((3 + i) as usize) as u32);
        i += 4;
        ebx += 1;
    }
}

unsafe fn vortex_EqHw_SetRightStates(vortex: *mut vortex_t, a: *const u16, b: *const u16) {
    let eqhw: *mut eqhw_t = &mut (*vortex).eq.this04;
    let mut i: i32 = 0;
    let mut ebx: i32;

    hwwrite((*vortex).mmio, 0x2b404, *a.add(0) as u32);
    hwwrite((*vortex).mmio, 0x2b408, *a.add(1) as u32);

    ebx = 0;
    while ebx < (*eqhw).this04 {
        hwwrite((*vortex).mmio, (0x2b1f4 + (i * 0xc)) as u32, *b.add(i as usize) as u32);
        hwwrite((*vortex).mmio, (0x2b1f8 + (i * 0xc)) as u32, *b.add((1 + i) as usize) as u32);
        hwwrite((*vortex).mmio, (0x2b1fc + (i * 0xc)) as u32, *b.add((2 + i) as usize) as u32);
        hwwrite((*vortex).mmio, (0x2b200 + (i * 0xc)) as u32, *b.add((3 + i) as usize) as u32);
        i += 4;
        ebx += 1;
    }
}

/* #if 0: disabled C hardware getter helpers intentionally not compiled. */

/* Mix Gains */
unsafe fn vortex_EqHw_SetBypassGain(vortex: *mut vortex_t, a: u16, b: u16) {
    let eqhw: *mut eqhw_t = &mut (*vortex).eq.this04;
    if (*eqhw).this08 == 0 {
        hwwrite((*vortex).mmio, 0x2b3d4, a as u32);
        hwwrite((*vortex).mmio, 0x2b3ec, b as u32);
    } else {
        hwwrite((*vortex).mmio, 0x2b3d4, sign_invert(a) as u32);
        hwwrite((*vortex).mmio, 0x2b3ec, sign_invert(b) as u32);
    }
}

unsafe fn vortex_EqHw_SetA3DBypassGain(vortex: *mut vortex_t, a: u16, b: u16) {
    hwwrite((*vortex).mmio, 0x2b3e0, a as u32);
    hwwrite((*vortex).mmio, 0x2b3f8, b as u32);
}

/* #if 0: disabled C current bypass gain setters intentionally not compiled. */

unsafe fn vortex_EqHw_SetLeftGainsSingleTarget(vortex: *mut vortex_t, index: u16, b: u16) {
    hwwrite((*vortex).mmio, 0x2b02c + ((index as u32) * 0x30), b as u32);
}

unsafe fn vortex_EqHw_SetRightGainsSingleTarget(vortex: *mut vortex_t, index: u16, b: u16) {
    hwwrite((*vortex).mmio, 0x2b20c + ((index as u32) * 0x30), b as u32);
}

unsafe fn vortex_EqHw_SetLeftGainsTarget(vortex: *mut vortex_t, a: *const u16) {
    let eqhw: *mut eqhw_t = &mut (*vortex).eq.this04;
    let mut ebx: i32;

    ebx = 0;
    while ebx < (*eqhw).this04 {
        hwwrite((*vortex).mmio, (0x2b02c + ebx * 0x30) as u32, *a.add(ebx as usize) as u32);
        ebx += 1;
    }
}

unsafe fn vortex_EqHw_SetRightGainsTarget(vortex: *mut vortex_t, a: *const u16) {
    let eqhw: *mut eqhw_t = &mut (*vortex).eq.this04;
    let mut ebx: i32;

    ebx = 0;
    while ebx < (*eqhw).this04 {
        hwwrite((*vortex).mmio, (0x2b20c + ebx * 0x30) as u32, *a.add(ebx as usize) as u32);
        ebx += 1;
    }
}

unsafe fn vortex_EqHw_SetLeftGainsCurrent(vortex: *mut vortex_t, a: *const u16) {
    let eqhw: *mut eqhw_t = &mut (*vortex).eq.this04;
    let mut ebx: i32;

    ebx = 0;
    while ebx < (*eqhw).this04 {
        hwwrite((*vortex).mmio, (0x2b028 + ebx * 0x30) as u32, *a.add(ebx as usize) as u32);
        ebx += 1;
    }
}

unsafe fn vortex_EqHw_SetRightGainsCurrent(vortex: *mut vortex_t, a: *const u16) {
    let eqhw: *mut eqhw_t = &mut (*vortex).eq.this04;
    let mut ebx: i32;

    ebx = 0;
    while ebx < (*eqhw).this04 {
        hwwrite((*vortex).mmio, (0x2b208 + ebx * 0x30) as u32, *a.add(ebx as usize) as u32);
        ebx += 1;
    }
}

/* #if 0: disabled C gain getter helpers intentionally not compiled. */

/* EQ band levels settings */
unsafe fn vortex_EqHw_SetLevels(vortex: *mut vortex_t, peaks: *const u16) {
    let eqhw: *mut eqhw_t = &mut (*vortex).eq.this04;
    let mut i: i32;

    /* set left peaks */
    i = 0;
    while i < (*eqhw).this04 {
        hwwrite((*vortex).mmio, (0x2b024 + i * VORTEX_BAND_COEFF_SIZE) as u32, *peaks.add(i as usize) as u32);
        i += 1;
    }

    hwwrite((*vortex).mmio, 0x2b3cc, *peaks.add((*eqhw).this04 as usize) as u32);
    hwwrite((*vortex).mmio, 0x2b3d8, *peaks.add(((*eqhw).this04 + 1) as usize) as u32);

    /* set right peaks */
    i = 0;
    while i < (*eqhw).this04 {
        hwwrite(
            (*vortex).mmio,
            (0x2b204 + i * VORTEX_BAND_COEFF_SIZE) as u32,
            *peaks.add((i + ((*eqhw).this04 + 2)) as usize) as u32,
        );
        i += 1;
    }

    hwwrite((*vortex).mmio, 0x2b3e4, *peaks.add((2 + ((*eqhw).this04 * 2)) as usize) as u32);
    hwwrite((*vortex).mmio, 0x2b3f0, *peaks.add((3 + ((*eqhw).this04 * 2)) as usize) as u32);
}

/* #if 0: disabled C levels getter intentionally not compiled. */

/* Global Control */
unsafe fn vortex_EqHw_SetControlReg(vortex: *mut vortex_t, reg: u32) {
    hwwrite((*vortex).mmio, 0x2b440, reg);
}

unsafe fn vortex_EqHw_SetSampleRate(vortex: *mut vortex_t, sr: u32) {
    hwwrite((*vortex).mmio, 0x2b440, ((sr & 0x1f) << 3) | 0xb800);
}

/* #if 0: disabled C control/sample-rate getters intentionally not compiled. */

unsafe fn vortex_EqHw_Enable(vortex: *mut vortex_t) {
    hwwrite((*vortex).mmio, VORTEX_EQ_CTRL, 0xf001);
}

unsafe fn vortex_EqHw_Disable(vortex: *mut vortex_t) {
    hwwrite((*vortex).mmio, VORTEX_EQ_CTRL, 0xf000);
}

/* Reset (zero) buffers */
unsafe fn vortex_EqHw_ZeroIO(vortex: *mut vortex_t) {
    let mut i: i32;
    i = 0;
    while i < 0x8 {
        hwwrite((*vortex).mmio, VORTEX_EQ_DEST + ((i << 2) as u32), 0x0);
        i += 1;
    }
    i = 0;
    while i < 0x4 {
        hwwrite((*vortex).mmio, VORTEX_EQ_SOURCE + ((i << 2) as u32), 0x0);
        i += 1;
    }
}

unsafe fn vortex_EqHw_ZeroA3DIO(vortex: *mut vortex_t) {
    let mut i: i32;
    i = 0;
    while i < 0x4 {
        hwwrite((*vortex).mmio, VORTEX_EQ_DEST + ((i << 2) as u32), 0x0);
        i += 1;
    }
}

unsafe fn vortex_EqHw_ZeroState(vortex: *mut vortex_t) {
    vortex_EqHw_SetControlReg(vortex, 0);
    vortex_EqHw_ZeroIO(vortex);
    hwwrite((*vortex).mmio, 0x2b3c0, 0);

    vortex_EqHw_SetTimeConsts(vortex, 0, 0);

    vortex_EqHw_SetLeftCoefs(vortex, asEqCoefsZeros);
    vortex_EqHw_SetRightCoefs(vortex, asEqCoefsZeros);

    vortex_EqHw_SetLeftGainsCurrent(vortex, eq_gains_zero);
    vortex_EqHw_SetRightGainsCurrent(vortex, eq_gains_zero);
    vortex_EqHw_SetLeftGainsTarget(vortex, eq_gains_zero);
    vortex_EqHw_SetRightGainsTarget(vortex, eq_gains_zero);

    vortex_EqHw_SetBypassGain(vortex, 0, 0);
    //vortex_EqHw_SetCurrBypassGain(vortex, 0, 0);
    vortex_EqHw_SetA3DBypassGain(vortex, 0, 0);
    //vortex_EqHw_SetCurrA3DBypassGain(vortex, 0, 0);
    vortex_EqHw_SetLeftStates(vortex, eq_states_zero, asEqOutStateZeros);
    vortex_EqHw_SetRightStates(vortex, eq_states_zero, asEqOutStateZeros);
    vortex_EqHw_SetLevels(vortex, eq_levels as *const u16);
}

/* Program coeficients as pass through */
unsafe fn vortex_EqHw_ProgramPipe(vortex: *mut vortex_t) {
    vortex_EqHw_SetTimeConsts(vortex, 0, 0);

    vortex_EqHw_SetLeftCoefs(vortex, asEqCoefsPipes);
    vortex_EqHw_SetRightCoefs(vortex, asEqCoefsPipes);

    vortex_EqHw_SetLeftGainsCurrent(vortex, eq_gains_current);
    vortex_EqHw_SetRightGainsCurrent(vortex, eq_gains_current);
    vortex_EqHw_SetLeftGainsTarget(vortex, eq_gains_current);
    vortex_EqHw_SetRightGainsTarget(vortex, eq_gains_current);
}

/* Program EQ block as 10 band Equalizer */
unsafe fn vortex_EqHw_Program10Band(vortex: *mut vortex_t, coefset: *mut auxxEqCoeffSet_t) {
    vortex_EqHw_SetTimeConsts(vortex, 0xc, 0x7fe0);

    vortex_EqHw_SetLeftCoefs(vortex, (*coefset).LeftCoefs.as_ptr());
    vortex_EqHw_SetRightCoefs(vortex, (*coefset).RightCoefs.as_ptr());

    vortex_EqHw_SetLeftGainsCurrent(vortex, (*coefset).LeftGains.as_ptr());

    vortex_EqHw_SetRightGainsTarget(vortex, (*coefset).RightGains.as_ptr());
    vortex_EqHw_SetLeftGainsTarget(vortex, (*coefset).LeftGains.as_ptr());

    vortex_EqHw_SetRightGainsCurrent(vortex, (*coefset).RightGains.as_ptr());
}

/* Read all EQ peaks. (think VU meter) */
unsafe fn vortex_EqHw_GetTenBandLevels(vortex: *mut vortex_t, peaks: *mut u16) {
    let eqhw: *mut eqhw_t = &mut (*vortex).eq.this04;
    let mut i: i32;

    if (*eqhw).this04 <= 0 {
        return;
    }

    i = 0;
    while i < (*eqhw).this04 {
        *peaks.add(i as usize) = hwread((*vortex).mmio, (0x2B024 + i * 0x30) as u32) as u16;
        i += 1;
    }
    i = 0;
    while i < (*eqhw).this04 {
        *peaks.add((i + (*eqhw).this04) as usize) = hwread((*vortex).mmio, (0x2B204 + i * 0x30) as u32) as u16;
        i += 1;
    }
}

/* CEqlzr.s */

unsafe fn vortex_Eqlzr_GetLeftGain(vortex: *mut vortex_t, index: u16, gain: *mut u16) -> i32 {
    let eq: *mut eqlzr_t = &mut (*vortex).eq;

    if (*eq).this28 != 0 {
        *gain = (*eq).this130[index as usize];
        return 0;
    }
    1
}

unsafe fn vortex_Eqlzr_SetLeftGain(vortex: *mut vortex_t, index: u16, gain: u16) {
    let eq: *mut eqlzr_t = &mut (*vortex).eq;

    if (*eq).this28 == 0 {
        return;
    }

    (*eq).this130[index as usize] = gain;
    if (*eq).this54 != 0 {
        return;
    }

    vortex_EqHw_SetLeftGainsSingleTarget(vortex, index, gain);
}

unsafe fn vortex_Eqlzr_GetRightGain(vortex: *mut vortex_t, index: u16, gain: *mut u16) -> i32 {
    let eq: *mut eqlzr_t = &mut (*vortex).eq;

    if (*eq).this28 != 0 {
        *gain = (*eq).this130[(index as i32 + (*eq).this10) as usize];
        return 0;
    }
    1
}

unsafe fn vortex_Eqlzr_SetRightGain(vortex: *mut vortex_t, index: u16, gain: u16) {
    let eq: *mut eqlzr_t = &mut (*vortex).eq;

    if (*eq).this28 == 0 {
        return;
    }

    (*eq).this130[(index as i32 + (*eq).this10) as usize] = gain;
    if (*eq).this54 != 0 {
        return;
    }

    vortex_EqHw_SetRightGainsSingleTarget(vortex, index, gain);
}

/* #if 0: disabled C all-band getter intentionally not compiled. */

unsafe fn vortex_Eqlzr_SetAllBandsFromActiveCoeffSet(vortex: *mut vortex_t) -> i32 {
    let eq: *mut eqlzr_t = &mut (*vortex).eq;

    vortex_EqHw_SetLeftGainsTarget(vortex, (*eq).this130.as_ptr());
    vortex_EqHw_SetRightGainsTarget(vortex, (*eq).this130.as_ptr().add((*eq).this10 as usize));

    0
}

unsafe fn vortex_Eqlzr_SetAllBands(vortex: *mut vortex_t, gains: *const u16, count: i32) -> i32 {
    let eq: *mut eqlzr_t = &mut (*vortex).eq;
    let mut i: i32;

    if (((*eq).this10) * 2 != count) || ((*eq).this28 == 0) {
        return 1;
    }

    i = 0;
    while i < count {
        (*eq).this130[i as usize] = *gains.add(i as usize);
        i += 1;
    }

    if (*eq).this54 != 0 {
        return 0;
    }
    vortex_Eqlzr_SetAllBandsFromActiveCoeffSet(vortex)
}

unsafe fn vortex_Eqlzr_SetA3dBypassGain(vortex: *mut vortex_t, a: u32, b: u32) {
    let eq: *mut eqlzr_t = &mut (*vortex).eq;
    let mut eax: u32;
    let ebx: u32;

    (*eq).this58 = a;
    (*eq).this5c = b;
    if (*eq).this54 != 0 {
        eax = (*eq).this0e as u32;
    } else {
        eax = (*eq).this0a as u32;
    }
    ebx = (eax.wrapping_mul((*eq).this58)) >> 0x10;
    eax = (eax.wrapping_mul((*eq).this5c)) >> 0x10;
    vortex_EqHw_SetA3DBypassGain(vortex, ebx as u16, eax as u16);
}

unsafe fn vortex_Eqlzr_ProgramA3dBypassGain(vortex: *mut vortex_t) {
    let eq: *mut eqlzr_t = &mut (*vortex).eq;
    let mut eax: u32;
    let ebx: u32;

    if (*eq).this54 != 0 {
        eax = (*eq).this0e as u32;
    } else {
        eax = (*eq).this0a as u32;
    }
    ebx = (eax.wrapping_mul((*eq).this58)) >> 0x10;
    eax = (eax.wrapping_mul((*eq).this5c)) >> 0x10;
    vortex_EqHw_SetA3DBypassGain(vortex, ebx as u16, eax as u16);
}

unsafe fn vortex_Eqlzr_ShutDownA3d(vortex: *mut vortex_t) {
    if !vortex.is_null() {
        vortex_EqHw_ZeroA3DIO(vortex);
    }
}

unsafe fn vortex_Eqlzr_SetBypass(vortex: *mut vortex_t, bp: u32) {
    let eq: *mut eqlzr_t = &mut (*vortex).eq;

    if ((*eq).this28 != 0) && (bp == 0) {
        /* EQ enabled */
        vortex_Eqlzr_SetAllBandsFromActiveCoeffSet(vortex);
        vortex_EqHw_SetBypassGain(vortex, (*eq).this08, (*eq).this08);
    } else {
        /* EQ disabled. */
        vortex_EqHw_SetLeftGainsTarget(vortex, (*eq).this14_array.as_ptr());
        vortex_EqHw_SetRightGainsTarget(vortex, (*eq).this14_array.as_ptr());
        vortex_EqHw_SetBypassGain(vortex, (*eq).this0c, (*eq).this0c);
    }
    vortex_Eqlzr_ProgramA3dBypassGain(vortex);
}

unsafe fn vortex_Eqlzr_ReadAndSetActiveCoefSet(vortex: *mut vortex_t) {
    let eq: *mut eqlzr_t = &mut (*vortex).eq;

    /* Set EQ BiQuad filter coeficients */
    memcpy(
        &mut (*eq).coefset as *mut auxxEqCoeffSet_t as *mut core::ffi::c_void,
        &asEqCoefsNormal as *const auxxEqCoeffSet_t as *const core::ffi::c_void,
        core::mem::size_of::<auxxEqCoeffSet_t>(),
    );
    /* Set EQ Band gain levels and dump into hardware registers. */
    vortex_Eqlzr_SetAllBands(vortex, eq_gains_normal, (*eq).this10 * 2);
}

unsafe fn vortex_Eqlzr_GetAllPeaks(vortex: *mut vortex_t, peaks: *mut u16, count: *mut i32) -> i32 {
    let eq: *mut eqlzr_t = &mut (*vortex).eq;

    if (*eq).this10 == 0 {
        return 1;
    }
    *count = (*eq).this10 * 2;
    vortex_EqHw_GetTenBandLevels(vortex, peaks);
    0
}

/* #if 0: disabled C active coef-set getter intentionally not compiled. */

unsafe fn vortex_Eqlzr_init(vortex: *mut vortex_t) {
    let eq: *mut eqlzr_t = &mut (*vortex).eq;

    /* Object constructor */
    //eq->this04 = 0;
    (*eq).this08 = 0; /* Bypass gain with EQ in use. */
    (*eq).this0a = 0x5999;
    (*eq).this0c = 0x5999; /* Bypass gain with EQ disabled. */
    (*eq).this0e = 0x5999;

    (*eq).this10 = 0xa; /* 10 eq frequency bands. */
    (*eq).this04.this04 = (*eq).this10;
    (*eq).this28 = 0x1; /* if 1 => Allow read access to this130 (gains) */
    (*eq).this54 = 0x0; /* if 1 => Dont Allow access to hardware (gains) */
    (*eq).this58 = 0xffff;
    (*eq).this5c = 0xffff;

    /* Set gains. */
    memset(
        (*eq).this14_array.as_mut_ptr() as *mut core::ffi::c_void,
        0,
        core::mem::size_of_val(&(*eq).this14_array),
    );

    /* Actual init. */
    vortex_EqHw_ZeroState(vortex);
    vortex_EqHw_SetSampleRate(vortex, 0x11);
    vortex_Eqlzr_ReadAndSetActiveCoefSet(vortex);

    vortex_EqHw_Program10Band(vortex, &mut (*eq).coefset);
    vortex_Eqlzr_SetBypass(vortex, (*eq).this54 as u32);
    vortex_Eqlzr_SetA3dBypassGain(vortex, 0, 0);
    vortex_EqHw_Enable(vortex);
}

unsafe fn vortex_Eqlzr_shutdown(vortex: *mut vortex_t) {
    vortex_Eqlzr_ShutDownA3d(vortex);
    vortex_EqHw_ProgramPipe(vortex);
    vortex_EqHw_Disable(vortex);
}

/* ALSA interface */

/* Control interface */
const snd_vortex_eqtoggle_info: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> i32 =
    snd_ctl_boolean_mono_info;

unsafe extern "C" fn snd_vortex_eqtoggle_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let vortex: *mut vortex_t = snd_kcontrol_chip(kcontrol);
    let eq: *mut eqlzr_t = &mut (*vortex).eq;
    //int i = kcontrol->private_value;

    (*ucontrol).value.integer.value[0] = if (*eq).this54 != 0 { 0 } else { 1 };

    0
}

unsafe extern "C" fn snd_vortex_eqtoggle_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let vortex: *mut vortex_t = snd_kcontrol_chip(kcontrol);
    let eq: *mut eqlzr_t = &mut (*vortex).eq;
    //int i = kcontrol->private_value;

    (*eq).this54 = if (*ucontrol).value.integer.value[0] != 0 { 0 } else { 1 };
    vortex_Eqlzr_SetBypass(vortex, (*eq).this54 as u32);

    1 /* Allways changes */
}

static vortex_eqtoggle_kcontrol: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"EQ Enable".as_ptr(),
    index: 0,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    private_value: 0,
    info: Some(snd_vortex_eqtoggle_info),
    get: Some(snd_vortex_eqtoggle_get),
    put: Some(snd_vortex_eqtoggle_put),
};

unsafe extern "C" fn snd_vortex_eq_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0x0000;
    (*uinfo).value.integer.max = 0x7fff;
    0
}

unsafe extern "C" fn snd_vortex_eq_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let vortex: *mut vortex_t = snd_kcontrol_chip(kcontrol);
    let i: i32 = (*kcontrol).private_value as i32;
    let mut gainL: u16 = 0;
    let mut gainR: u16 = 0;

    vortex_Eqlzr_GetLeftGain(vortex, i as u16, &mut gainL);
    vortex_Eqlzr_GetRightGain(vortex, i as u16, &mut gainR);
    (*ucontrol).value.integer.value[0] = gainL as _;
    (*ucontrol).value.integer.value[1] = gainR as _;
    0
}

unsafe extern "C" fn snd_vortex_eq_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let vortex: *mut vortex_t = snd_kcontrol_chip(kcontrol);
    let mut changed: i32 = 0;
    let i: i32 = (*kcontrol).private_value as i32;
    let mut gainL: u16 = 0;
    let mut gainR: u16 = 0;

    vortex_Eqlzr_GetLeftGain(vortex, i as u16, &mut gainL);
    vortex_Eqlzr_GetRightGain(vortex, i as u16, &mut gainR);

    if gainL as _ != (*ucontrol).value.integer.value[0] {
        vortex_Eqlzr_SetLeftGain(vortex, i as u16, (*ucontrol).value.integer.value[0] as u16);
        changed = 1;
    }
    if gainR as _ != (*ucontrol).value.integer.value[1] {
        vortex_Eqlzr_SetRightGain(vortex, i as u16, (*ucontrol).value.integer.value[1] as u16);
        changed = 1;
    }
    changed
}

static vortex_eq_kcontrol: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"                        .".as_ptr(),
    index: 0,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    private_value: 0,
    info: Some(snd_vortex_eq_info),
    get: Some(snd_vortex_eq_get),
    put: Some(snd_vortex_eq_put),
};

unsafe extern "C" fn snd_vortex_peaks_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 20;
    (*uinfo).value.integer.min = 0x0000;
    (*uinfo).value.integer.max = 0x7fff;
    0
}

unsafe extern "C" fn snd_vortex_peaks_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let vortex: *mut vortex_t = snd_kcontrol_chip(kcontrol);
    let mut i: i32;
    let mut count: i32 = 0;
    let mut peaks: [u16; 20] = [0; 20];

    vortex_Eqlzr_GetAllPeaks(vortex, peaks.as_mut_ptr(), &mut count);
    if count != 20 {
        dev_err((*(*vortex).card).dev, c"peak count error 20 != %d\n".as_ptr(), count);
        return -1;
    }
    i = 0;
    while i < 20 {
        (*ucontrol).value.integer.value[i as usize] = peaks[i as usize] as _;
        i += 1;
    }

    0
}

static vortex_levels_kcontrol: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"EQ Peaks".as_ptr(),
    access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
    info: Some(snd_vortex_peaks_info),
    get: Some(snd_vortex_peaks_get),
    ..snd_kcontrol_new::ZERO
};

/* EQ band gain labels. */
static EqBandLabels: [*const i8; 10] = [
    c"EQ0 31Hz\0".as_ptr(),
    c"EQ1 63Hz\0".as_ptr(),
    c"EQ2 125Hz\0".as_ptr(),
    c"EQ3 250Hz\0".as_ptr(),
    c"EQ4 500Hz\0".as_ptr(),
    c"EQ5 1KHz\0".as_ptr(),
    c"EQ6 2KHz\0".as_ptr(),
    c"EQ7 4KHz\0".as_ptr(),
    c"EQ8 8KHz\0".as_ptr(),
    c"EQ9 16KHz\0".as_ptr(),
];

/* ALSA driver entry points. Init and exit. */
unsafe fn vortex_eq_init(vortex: *mut vortex_t) -> i32 {
    let mut kcontrol: *mut snd_kcontrol;
    let mut err: i32;
    let mut i: i32;

    vortex_Eqlzr_init(vortex);

    kcontrol = snd_ctl_new1(&vortex_eqtoggle_kcontrol, vortex as *mut core::ffi::c_void);
    if kcontrol.is_null() {
        return -ENOMEM;
    }
    (*kcontrol).private_value = 0;
    err = snd_ctl_add((*vortex).card, kcontrol);
    if err < 0 {
        return err;
    }

    /* EQ gain controls */
    i = 0;
    while i < 10 {
        kcontrol = snd_ctl_new1(&vortex_eq_kcontrol, vortex as *mut core::ffi::c_void);
        if kcontrol.is_null() {
            return -ENOMEM;
        }
        snprintf(
            (*kcontrol).id.name.as_mut_ptr(),
            core::mem::size_of_val(&(*kcontrol).id.name),
            c"%s Playback Volume".as_ptr(),
            EqBandLabels[i as usize],
        );
        (*kcontrol).private_value = i as _;
        err = snd_ctl_add((*vortex).card, kcontrol);
        if err < 0 {
            return err;
        }
        //vortex->eqctrl[i] = kcontrol;
        i += 1;
    }
    /* EQ band levels */
    kcontrol = snd_ctl_new1(&vortex_levels_kcontrol, vortex as *mut core::ffi::c_void);
    if kcontrol.is_null() {
        return -ENOMEM;
    }
    err = snd_ctl_add((*vortex).card, kcontrol);
    if err < 0 {
        return err;
    }

    0
}

unsafe fn vortex_eq_free(vortex: *mut vortex_t) -> i32 {
    /*
       //FIXME: segfault because vortex->eqctrl[i] == 4
       int i;
       for (i=0; i<10; i++) {
       if (vortex->eqctrl[i])
       snd_ctl_remove(vortex->card, vortex->eqctrl[i]);
       }
     */
    vortex_Eqlzr_shutdown(vortex);
    0
}

/* End */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
