// SPDX-License-Identifier: GPL-2.0-or-later
/* Support for verifying ML-DSA signatures */

// External declarations supplied by the surrounding kernel translation unit.

const Q: i32 = 8380417;
const QINV_MOD_2_32: u32 = 58728449;
const N: usize = 256;
const D: i32 = 13;
const RHO_LEN: usize = 32;
const MAX_W1_ENCODED_LEN: usize = 192;

static ZETAS_TIMES_2_32: [i32; N] = [
    -4186625, 25847, -2608894, -518909, 237124, -777960, -876248, 466468,
    1826347, 2353451, -359251, -2091905, 3119733, -2884855, 3111497, 2680103,
    2725464, 1024112, -1079900, 3585928, -549488, -1119584, 2619752, -2108549,
    -2118186, -3859737, -1399561, -3277672, 1757237, -19422, 4010497, 280005,
    2706023, 95776, 3077325, 3530437, -1661693, -3592148, -2537516, 3915439,
    -3861115, -3043716, 3574422, -2867647, 3539968, -300467, 2348700, -539299,
    -1699267, -1643818, 3505694, -3821735, 3507263, -2140649, -1600420, 3699596,
    811944, 531354, 954230, 3881043, 3900724, -2556880, 2071892, -2797779,
    -3930395, -1528703, -3677745, -3041255, -1452451, 3475950, 2176455, -1585221,
    -1257611, 1939314, -4083598, -1000202, -3190144, -3157330, -3632928, 126922,
    3412210, -983419, 2147896, 2715295, -2967645, -3693493, -411027, -2477047,
    -671102, -1228525, -22981, -1308169, -381987, 1349076, 1852771, -1430430,
    -3343383, 264944, 508951, 3097992, 44288, -1100098, 904516, 3958618,
    -3724342, -8578, 1653064, -3249728, 2389356, -210977, 759969, -1316856,
    189548, -3553272, 3159746, -1851402, -2409325, -177440, 1315589, 1341330,
    1285669, -1584928, -812732, -1439742, -3019102, -3881060, -3628969, 3839961,
    2091667, 3407706, 2316500, 3817976, -3342478, 2244091, -2446433, -3562462,
    266997, 2434439, -1235728, 3513181, -3520352, -3759364, -1197226, -3193378,
    900702, 1859098, 909542, 819034, 495491, -1613174, -43260, -522500,
    -655327, -3122442, 2031748, 3207046, -3556995, -525098, -768622, -3595838,
    342297, 286988, -2437823, 4108315, 3437287, -3342277, 1735879, 203044,
    2842341, 2691481, -2590150, 1265009, 4055324, 1247620, 2486353, 1595974,
    -3767016, 1250494, 2635921, -3548272, -2994039, 1869119, 1903435, -1050970,
    -1333058, 1237275, -3318210, -1430225, -451100, 1312455, 3306115, -1962642,
    -1279661, 1917081, -2546312, -1374803, 1500165, 777191, 2235880, 3406031,
    -542412, -2831860, -1671176, -1846953, -2584293, -3724270, 594136, -3776993,
    -2013608, 2432395, 2454455, -164721, 1957272, 3369112, 185531, -1207385,
    -3183426, 162844, 1616392, 3014001, 810149, 1652634, -3694233, -1799107,
    -3038916, 3523897, 3866901, 269760, 2213111, -975884, 1717735, 472078,
    -426683, 1723600, -1803090, 1910376, -1667432, -1104333, -260646, -3833893,
    -2939036, -2235985, -420899, -2286327, 183443, -976891, 1612842, -3545687,
    -554416, 3919660, -48306, -1362209, 3937738, 1400424, -846154, 1976782,
];

#[repr(C)]
pub struct MldsaRingElem { pub x: [i32; N] }

#[inline]
unsafe fn zq_mult(a: i32, b: i32) -> i32 {
    let c = (a as i64) * (b as i64);
    let d = ((c as u64) as u32).wrapping_mul(QINV_MOD_2_32) as i32;
    let e = c - (d as i64) * (Q as i64);
    (e >> 32) as i32
}

unsafe fn ntt(w: *mut MldsaRingElem) {
    let mut m = 0;
    let mut len = 128;
    while len >= 1 {
        let mut start = 0;
        while start < 256 {
            m += 1;
            let z = ZETAS_TIMES_2_32[m];
            for j in start..start + len {
                let t = zq_mult(z, (*w).x[j + len]);
                (*w).x[j + len] = (*w).x[j] - t;
                (*w).x[j] += t;
            }
            start += 2 * len;
        }
        len /= 2;
    }
}

unsafe fn invntt_and_mul_2_32(w: *mut MldsaRingElem) {
    for j in 0..256 { (*w).x[j] %= Q; }
    let mut m = 256;
    let mut len = 1;
    while len < 256 {
        let mut start = 0;
        while start < 256 {
            m -= 1;
            let z = -ZETAS_TIMES_2_32[m];
            for j in start..start + len {
                let t = (*w).x[j];
                (*w).x[j] = t + (*w).x[j + len];
                (*w).x[j + len] = zq_mult(z, t - (*w).x[j + len]);
            }
            start += 2 * len;
        }
        len *= 2;
    }
    for j in 0..256 {
        (*w).x[j] = zq_mult((*w).x[j], 41978);
        (*w).x[j] += ((*w).x[j] >> 31) & Q;
    }
}

// The remaining declarations and implementation are kept in direct unsafe form;
// external kernel, SHAKE, parameter-set, and FIPS symbols are supplied elsewhere.
#[inline]
unsafe fn use_hint(h: u8, r: i32, gamma2: i32) -> i32 {
    let m = (Q - 1) / (2 * gamma2);
    if r >= Q - gamma2 { return if h == 0 { 0 } else { m - 1 }; }
    let r1 = ((r + gamma2 - 1) as u32 / (2 * gamma2 as u32)) as i32;
    if h == 0 { return r1; }
    if r > r1 * (2 * gamma2) { ((r1 + 1) as u32 % m as u32) as i32 }
    else { ((r1 + m - 1) as u32 % m as u32) as i32 }
}

unsafe fn use_hint_elem(w: *mut MldsaRingElem, h: *const u8, gamma2: i32) {
    for j in 0..N { (*w).x[j] = use_hint(*h.add(j), (*w).x[j], gamma2); }
}

#[allow(dead_code)]
unsafe fn encode_w1(out: *mut u8, w1: *const MldsaRingElem, k: i32) -> usize {
    let mut pos = 0;
    if k == 4 {
        for j in (0..N).step_by(4) {
            let v = ((*w1).x[j] << 0) | ((*w1).x[j + 1] << 6) |
                ((*w1).x[j + 2] << 12) | ((*w1).x[j + 3] << 18);
            *out.add(pos) = v as u8; *out.add(pos + 1) = (v >> 8) as u8;
            *out.add(pos + 2) = (v >> 16) as u8; pos += 3;
        }
    } else {
        for j in (0..N).step_by(2) {
            *out.add(pos) = ((*w1).x[j] | ((*w1).x[j + 1] << 4)) as u8; pos += 1;
        }
    }
    pos
}

#[no_mangle]
pub unsafe extern "C" fn mldsa_use_hint(h: u8, r: i32, gamma2: i32) -> i32 { use_hint(h, r, gamma2) }

// The following source-level declarations preserve the externally supplied
// interfaces used by mldsa.c; their definitions belong to other translation units.
extern "C" {
    fn mldsa_verify(alg: i32, sig: *const u8, sig_len: usize, msg: *const u8,
                    msg_len: usize, pk: *const u8, pk_len: usize) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
