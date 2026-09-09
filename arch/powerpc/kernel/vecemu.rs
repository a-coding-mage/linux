// SPDX-License-Identifier: GPL-2.0
/*
 * Routines to emulate some Altivec/VMX instructions, specifically
 * those that can trap when given denormalized operands in Java mode.
 */

// Dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    fn vaddfp(dst: *mut vector128, a: *mut vector128, b: *mut vector128);
    fn vsubfp(dst: *mut vector128, a: *mut vector128, b: *mut vector128);
    fn vmaddfp(dst: *mut vector128, a: *mut vector128, b: *mut vector128, c: *mut vector128);
    fn vnmsubfp(dst: *mut vector128, a: *mut vector128, b: *mut vector128, c: *mut vector128);
    fn vrefp(dst: *mut vector128, src: *mut vector128);
    fn vrsqrtefp(dst: *mut vector128, src: *mut vector128);
    fn vexptep(dst: *mut vector128, src: *mut vector128);
}

#[repr(C)]
pub struct vector128 {
    pub u: [u32; 4],
}

#[allow(dead_code)]
static EXP2S: [u32; 8] = [
    0x800000, 0x8b95c2, 0x9837f0, 0xa5fed7,
    0xb504f3, 0xc5672a, 0xd744fd, 0xeac0c7,
];

#[inline]
fn mulhwu(a: u32, b: u32) -> u32 { (((a as u64) * (b as u64)) >> 32) as u32 }

/*
 * Computes an estimate of 2^x.  The `s' argument is the 32-bit
 * single-precision floating-point representation of x.
 */
unsafe fn eexp2(s: u32) -> u32 {
    let mut exp = ((s >> 23) & 0xff) as i32 - 127;
    if exp > 7 {
        if exp == 128 && (s & 0x7fffff) != 0 { return s | 0x400000; }
        return if (s & 0x80000000) != 0 { 0 } else { 0x7f800000 };
    }
    if exp < -23 { return 0x3f800000; }
    let mut pwr = (s & 0x7fffff) | 0x800000;
    if exp > 0 { pwr <<= exp; } else { pwr >>= -exp; }
    if (s & 0x80000000) != 0 { pwr = (-(pwr as i32)) as u32; }
    exp = ((pwr >> 23) as i32) + 126;
    if exp >= 254 { return 0x7f800000; }
    if exp < -23 { return 0; }
    let mut mant = EXP2S[((pwr >> 20) & 7) as usize];
    let mut frac = mulhwu(pwr << 12, 0x172b83ff);
    frac = mulhwu(frac, mant);
    mant = mant.wrapping_add(frac);
    if exp >= 0 { return mant.wrapping_add((exp as u32) << 23); }
    let n = -exp;
    mant = mant.wrapping_add(1u32 << (n - 1));
    mant >> n
}

/* Computes an estimate of log_2(x). */
unsafe fn elog2(mut s: u32) -> u32 {
    let mut exp = s & 0x7f800000;
    let mut mant = s & 0x7fffff;
    if exp == 0x7f800000 { if mant != 0 { s |= 0x400000; } return s; }
    if (exp | mant) == 0 { return 0xff800000; }
    if exp == 0 {
        let lz = mant.leading_zeros() as i32;
        mant <<= (lz - 8) as u32;
        exp = ((-118 - lz) << 23) as u32;
    } else { mant |= 0x800000; exp = exp.wrapping_sub(127 << 23); }
    if mant >= 0xb504f3 { exp |= 0x400000; mant = mulhwu(mant, 0xb504f334); }
    if mant >= 0x9837f0 { exp |= 0x200000; mant = mulhwu(mant, 0xd744fccb); }
    if mant >= 0x8b95c2 { exp |= 0x100000; mant = mulhwu(mant, 0xeac0c6e8); }
    if mant > 0x800000 { let frac = mulhwu((mant - 0x800000) << 1, 0xb0c7cd3a); exp = exp.wrapping_add(frac); }
    s = exp & 0x80000000;
    if exp != 0 {
        if s != 0 { exp = (-(exp as i32)) as u32; }
        let lz = 8 - exp.leading_zeros() as i32;
        if lz > 0 { exp >>= lz; } else if lz < 0 { exp <<= -lz; }
        s = s.wrapping_add((((lz + 126) as u32) << 23).wrapping_add(exp));
    }
    s
}

const VSCR_SAT: u32 = 1;

unsafe fn ctsxs(x: u32, scale: i32, vscrp: *mut u32) -> i32 {
    let e = ((x >> 23) & 0xff) as i32; let mut mant = (x & 0x7fffff) as i32;
    if e == 255 && mant != 0 { return 0; }
    let exp = e - 127 + scale; if exp < 0 { return 0; }
    if exp >= 31 { if x.wrapping_add((scale << 23) as u32) != 0xcf000000 { *vscrp |= VSCR_SAT; } return if x & 0x80000000 != 0 { i32::MIN } else { i32::MAX }; }
    mant |= 0x800000; let value = (mant << 7) >> (30 - exp);
    if x & 0x80000000 != 0 { -value } else { value }
}

unsafe fn ctuxs(x: u32, scale: i32, vscrp: *mut u32) -> u32 {
    let e = ((x >> 23) & 0xff) as i32; let mut mant = x & 0x7fffff;
    if e == 255 && mant != 0 { return 0; } let exp = e - 127 + scale; if exp < 0 { return 0; }
    if x & 0x80000000 != 0 { *vscrp |= VSCR_SAT; return 0; }
    if exp >= 32 { *vscrp |= VSCR_SAT; return u32::MAX; }
    mant |= 0x800000; (mant << 8) >> (31 - exp)
}

unsafe fn rfiz(x: u32) -> u32 { let exp = ((x >> 23) & 0xff) as i32 - 127; if exp == 128 && x & 0x7fffff != 0 { return x | 0x400000; } if exp >= 23 { return x; } if exp < 0 { return x & 0x80000000; } x & !(0x7fffff >> exp) }
unsafe fn rfii(x: u32) -> u32 { let exp = ((x >> 23) & 0xff) as i32 - 127; if exp == 128 && x & 0x7fffff != 0 { return x | 0x400000; } if exp >= 23 { return x; } if x & 0x7fffffff == 0 { return x; } if exp < 0 { return (x & 0x80000000) | 0x3f800000; } let mask = 0x7fffff >> exp; (x.wrapping_add(mask)) & !mask }
unsafe fn rfin(x: u32) -> u32 { let exp = ((x >> 23) & 0xff) as i32 - 127; if exp == 128 && x & 0x7fffff != 0 { return x | 0x400000; } if exp >= 23 { return x; } if exp < -1 { return x & 0x80000000; } if exp == -1 { return (x & 0x80000000) | 0x3f800000; } let half = 0x400000 >> exp; (x.wrapping_add(half)) & !(0x7fffff >> exp) }

// The surrounding kernel supplies these ABI-specific types and accessors.
pub unsafe fn emulate_altivec(regs: *mut pt_regs) -> i32 {
    let mut instr: ppc_inst_t = core::mem::zeroed();
    let mut i: usize;
    let mut word: u32;
    let mut va: usize;
    let mut vb: usize;
    let mut vc: usize;
    let mut vd: usize;
    let vrs: *mut vector128;

    if get_user_instr(&mut instr, (*regs).nip as *const core::ffi::c_void) != 0 { return -EFAULT; }
    word = ppc_inst_val(instr);
    if ppc_inst_primary_opcode(instr) != 4 { return -EINVAL; }
    vd = ((word >> 21) & 0x1f) as usize;
    va = ((word >> 16) & 0x1f) as usize;
    vb = ((word >> 11) & 0x1f) as usize;
    vc = ((word >> 6) & 0x1f) as usize;
    vrs = (*current).thread.vr_state.vr;
    match word & 0x3f {
        10 => match vc {
            0 => vaddfp(vrs.add(vd), vrs.add(va), vrs.add(vb)),
            1 => vsubfp(vrs.add(vd), vrs.add(va), vrs.add(vb)),
            4 => vrefp(vrs.add(vd), vrs.add(vb)),
            5 => vrsqrtefp(vrs.add(vd), vrs.add(vb)),
            6 => { for i in 0..4 { (*vrs.add(vd)).u[i] = eexp2((*vrs.add(vb)).u[i]); } }
            7 => { for i in 0..4 { (*vrs.add(vd)).u[i] = elog2((*vrs.add(vb)).u[i]); } }
            8 => { for i in 0..4 { (*vrs.add(vd)).u[i] = rfin((*vrs.add(vb)).u[i]); } }
            9 => { for i in 0..4 { (*vrs.add(vd)).u[i] = rfiz((*vrs.add(vb)).u[i]); } }
            10 => { for i in 0..4 { let mut x = (*vrs.add(vb)).u[i]; x = if x & 0x80000000 != 0 { rfiz(x) } else { rfii(x) }; (*vrs.add(vd)).u[i] = x; } }
            11 => { for i in 0..4 { let mut x = (*vrs.add(vb)).u[i]; x = if x & 0x80000000 != 0 { rfii(x) } else { rfiz(x) }; (*vrs.add(vd)).u[i] = x; } }
            14 => { for i in 0..4 { (*vrs.add(vd)).u[i] = ctuxs((*vrs.add(vb)).u[i], va as i32, &mut (*current).thread.vr_state.vscr.u[3]); } }
            15 => { for i in 0..4 { (*vrs.add(vd)).u[i] = ctsxs((*vrs.add(vb)).u[i], va as i32, &mut (*current).thread.vr_state.vscr.u[3]) as u32; } }
            _ => return -EINVAL,
        },
        46 => vmaddfp(vrs.add(vd), vrs.add(va), vrs.add(vb), vrs.add(vc)),
        47 => vnmsubfp(vrs.add(vd), vrs.add(va), vrs.add(vb), vrs.add(vc)),
        _ => return -EINVAL,
    }
    0
}

// External kernel declarations.
extern "C" {
    fn get_user_instr(instr: *mut ppc_inst_t, addr: *const core::ffi::c_void) -> i32;
    fn ppc_inst_val(instr: ppc_inst_t) -> u32;
    fn ppc_inst_primary_opcode(instr: ppc_inst_t) -> u32;
}
pub enum pt_regs {}
pub enum ppc_inst_t {}
extern "C" { static mut current: *mut current_task; }
pub enum current_task {}
const EFAULT: i32 = 14;
const EINVAL: i32 = 22;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
