/* Machine-dependent software floating-point definitions.
 * Sparc userland (_Q_*) version. Translated from sfp-machine_32.h.
 */

pub const _FP_W_TYPE_SIZE: usize = 32;
pub type _FP_W_TYPE = u32;
pub type _FP_WS_TYPE = i32;
pub type _FP_I_TYPE = i32;

/* The following meat operations are supplied by the generic soft-float
 * implementation. */
macro_rules! _FP_MUL_MEAT_S { ($r:ident, $x:ident, $y:ident) => {
    _FP_MUL_MEAT_1_wide!(_FP_WFRACBITS_S, $r, $x, $y, umul_ppmm)
} }
macro_rules! _FP_MUL_MEAT_D { ($r:ident, $x:ident, $y:ident) => {
    _FP_MUL_MEAT_2_wide!(_FP_WFRACBITS_D, $r, $x, $y, umul_ppmm)
} }
macro_rules! _FP_MUL_MEAT_Q { ($r:ident, $x:ident, $y:ident) => {
    _FP_MUL_MEAT_4_wide!(_FP_WFRACBITS_Q, $r, $x, $y, umul_ppmm)
} }
macro_rules! _FP_DIV_MEAT_S { ($r:ident, $x:ident, $y:ident) => { _FP_DIV_MEAT_1_udiv!(S, $r, $x, $y) } }
macro_rules! _FP_DIV_MEAT_D { ($r:ident, $x:ident, $y:ident) => { _FP_DIV_MEAT_2_udiv!(D, $r, $x, $y) } }
macro_rules! _FP_DIV_MEAT_Q { ($r:ident, $x:ident, $y:ident) => { _FP_DIV_MEAT_4_udiv!(Q, $r, $x, $y) } }

/* NaN fractions and signs. */
pub const _FP_NANFRAC_S: u32 = (_FP_QNANBIT_S << 1) - 1;
pub const _FP_NANFRAC_D: [u32; 2] = [(_FP_QNANBIT_D << 1) - 1, u32::MAX];
pub const _FP_NANFRAC_Q: [u32; 4] = [(_FP_QNANBIT_Q << 1) - 1, u32::MAX, u32::MAX, u32::MAX];
pub const _FP_NANSIGN_S: i32 = 0;
pub const _FP_NANSIGN_D: i32 = 0;
pub const _FP_NANSIGN_Q: i32 = 0;
pub const _FP_KEEPNANFRACP: i32 = 1;

/* If one NaN is signaling and the other is not, choose that one; otherwise
 * choose X. For _Qp_* and _Q_* this prefers X, while CPU instruction
 * emulation prefers Y (SPAMv9 B.2.2). */
macro_rules! _FP_CHOOSENAN {
    ($fs:ident, $wc:ident, $r:ident, $x:ident, $y:ident, $op:ident) => {{
        if (_FP_FRAC_HIGH_RAW_$fs!($y) & _FP_QNANBIT_$fs! != 0)
            && (_FP_FRAC_HIGH_RAW_$fs!($x) & _FP_QNANBIT_$fs! == 0) {
            $r##_s = $x##_s;
            _FP_FRAC_COPY_$wc!($r, $x);
        } else {
            $r##_s = $y##_s;
            _FP_FRAC_COPY_$wc!($r, $y);
        }
        $r##_c = FP_CLS_NAN;
    }};
}

/* SPARC inline assembly is expressed as limb-wise wrapping arithmetic. */
macro_rules! __FP_FRAC_ADD_3 { ($r2:ident,$r1:ident,$r0:ident,$x2:expr,$x1:expr,$x0:expr,$y2:expr,$y1:expr,$y0:expr) => {{
    let (v0, c0) = ($x0 as u32).overflowing_add($y0 as u32);
    let (v1a, c1a) = ($x1 as u32).overflowing_add($y1 as u32);
    let (v1, c1b) = v1a.overflowing_add(c0 as u32);
    let (v2a, c2a) = ($x2 as u32).overflowing_add($y2 as u32);
    let (v2, _) = v2a.overflowing_add((c1a || c1b) as u32);
    $r0 = v0; $r1 = v1; $r2 = v2;
}} }
macro_rules! __FP_FRAC_SUB_3 { ($r2:ident,$r1:ident,$r0:ident,$x2:expr,$x1:expr,$x0:expr,$y2:expr,$y1:expr,$y0:expr) => {{
    let (v0, b0) = ($x0 as u32).overflowing_sub($y0 as u32);
    let (v1a, b1a) = ($x1 as u32).overflowing_sub($y1 as u32);
    let (v1, b1b) = v1a.overflowing_sub(b0 as u32);
    let (v2a, _) = ($x2 as u32).overflowing_sub($y2 as u32);
    let (v2, _) = v2a.overflowing_sub((b1a || b1b) as u32);
    $r0 = v0; $r1 = v1; $r2 = v2;
}} }
macro_rules! __FP_FRAC_ADD_4 { ($r3:ident,$r2:ident,$r1:ident,$r0:ident,$x3:expr,$x2:expr,$x1:expr,$x0:expr,$y3:expr,$y2:expr,$y1:expr,$y0:expr) => {{
    let (a0,c0)=($x0 as u32).overflowing_add($y0 as u32); let (a1,c1)=($x1 as u32).overflowing_add(($y1 as u32).wrapping_add(c0 as u32)); let (a2,c2)=($x2 as u32).overflowing_add(($y2 as u32).wrapping_add(c1 as u32)); let (a3,_)=($x3 as u32).overflowing_add(($y3 as u32).wrapping_add(c2 as u32)); $r0=a0;$r1=a1;$r2=a2;$r3=a3;
}} }
macro_rules! __FP_FRAC_SUB_4 { ($r3:ident,$r2:ident,$r1:ident,$r0:ident,$x3:expr,$x2:expr,$x1:expr,$x0:expr,$y3:expr,$y2:expr,$y1:expr,$y0:expr) => {{ let (a0,b0)=($x0 as u32).overflowing_sub($y0 as u32); let (a1,b1)=($x1 as u32).overflowing_sub(($y1 as u32).wrapping_add(b0 as u32)); let (a2,b2)=($x2 as u32).overflowing_sub(($y2 as u32).wrapping_add(b1 as u32)); let (a3,_)=($x3 as u32).overflowing_sub(($y3 as u32).wrapping_add(b2 as u32)); $r0=a0;$r1=a1;$r2=a2;$r3=a3; }} }
macro_rules! __FP_FRAC_DEC_3 { ($($a:tt)*) => { __FP_FRAC_SUB_3!($($a)*) } }
macro_rules! __FP_FRAC_DEC_4 { ($($a:tt)*) => { __FP_FRAC_SUB_4!($($a)*) } }
macro_rules! __FP_FRAC_ADDI_4 { ($x3:ident,$x2:ident,$x1:ident,$x0:ident,$i:expr) => {{ let (v,c)=($x0 as u32).overflowing_add($i as u32); $x0=v; let (v,c2)=($x1 as u32).overflowing_add(c as u32); $x1=v; let (v,c3)=($x2 as u32).overflowing_add(c2 as u32); $x2=v; $x3=($x3 as u32).wrapping_add(c3 as u32); }} }

/* CONFIG_SMP selects the current task's FSR; the non-SMP declaration is an
 * external dependency and is intentionally left as a declaration. */
#[cfg(not(CONFIG_SMP))]
extern "C" { pub static mut last_task_used_math: *mut task_struct; }

#[cfg(not(any(FP_ROUNDMODE_DEFINED)))]
macro_rules! FP_ROUNDMODE { () => { ((last_task_used_math_ref().thread.fsr >> 30) & 0x3) } }

pub const FP_EX_INVALID: i32 = 1 << 4;
pub const FP_EX_OVERFLOW: i32 = 1 << 3;
pub const FP_EX_UNDERFLOW: i32 = 1 << 2;
pub const FP_EX_DIVZERO: i32 = 1 << 1;
pub const FP_EX_INEXACT: i32 = 1 << 0;

macro_rules! FP_HANDLE_EXCEPTIONS { () => { return _fex; } }
macro_rules! FP_INHIBIT_RESULTS { () => { ((last_task_used_math_ref().thread.fsr >> 23) & _fex) } }
macro_rules! FP_TRAPPING_EXCEPTIONS { () => { ((last_task_used_math_ref().thread.fsr >> 23) & 0x1f) } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
