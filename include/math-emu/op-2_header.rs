/* Software floating-point emulation: two-word fraction operations.
 * Rust translation of math-emu/op-2.h.  External types, constants, unions,
 * and arithmetic primitives are supplied by the including implementation.
 */

/* C token-pasting is represented by explicit field expressions in these
 * macros; callers pass the two words as `(high, low)` expressions. */
macro_rules! _FP_FRAC_COPY_2 { (($dh:expr,$dl:expr),($sh:expr,$sl:expr)) => {{ $dl=$sl; $dh=$sh; }} }
macro_rules! _FP_FRAC_HIGH_2 { ($x:expr) => { $x.1 } }
macro_rules! _FP_FRAC_LOW_2 { ($x:expr) => { $x.0 } }
macro_rules! _FP_FRAC_SET_2 { (($h:expr,$l:expr),$i1:expr,$i0:expr) => {{ $h=$i1; $l=$i0; }} }
macro_rules! _FP_FRAC_SLL_2 { (($h:expr,$l:expr),$n:expr,$bits:expr) => {{ let n=$n; if n < $bits { if n == 1 { $h=$h.wrapping_add($h).wrapping_add((($l as i32)<0) as _); $l=$l.wrapping_add($l); } else { $h=($h<<n)|($l>>($bits-n)); $l<<=n; } } else { $h=$l << (n-$bits); $l=0; } }} }
macro_rules! _FP_FRAC_SRL_2 { (($h:expr,$l:expr),$n:expr,$bits:expr) => {{ let n=$n; if n < $bits { $l=($l>>n)|($h<<($bits-n)); $h>>=n; } else { $l=$h>>(n-$bits); $h=0; } }} }
macro_rules! _FP_FRAC_SRS_2 { (($h:expr,$l:expr),$n:expr,$bits:expr) => {{ let n=$n; if n < $bits { let sticky=if n==1 { $l&1 } else { (($l<<($bits-n))!=0) as _ }; $l=($h<<($bits-n))|($l>>n)|sticky; $h>>=n; } else { $l=($h>>(n-$bits))|((if n==$bits {0} else {$h<<(2*$bits-n)})|$l != 0) as _; $h=0; } }} }
macro_rules! _FP_FRAC_NEGP_2 { (($h:expr,$l:expr),$st:ty) => { ($h as $st) < 0 } }
macro_rules! _FP_FRAC_ZEROP_2 { (($h:expr,$l:expr)) => { ($h|$l)==0 } }
macro_rules! _FP_FRAC_EQ_2 { (($xh:expr,$xl:expr),($yh:expr,$yl:expr)) => { $xh==$yh && $xl==$yl } }
macro_rules! _FP_FRAC_GT_2 { (($xh:expr,$xl:expr),($yh:expr,$yl:expr)) => { $xh>$yh || ($xh==$yh && $xl>$yl) } }
macro_rules! _FP_FRAC_GE_2 { (($xh:expr,$xl:expr),($yh:expr,$yl:expr)) => { $xh>$yh || ($xh==$yh && $xl>=$yl) } }

pub const _FP_ZEROFRAC_2: (u128,u128) = (0,0);
pub const _FP_MINFRAC_2: (u128,u128) = (0,1);

macro_rules! __FP_FRAC_SET_2 { (($h:expr,$l:expr),$i1:expr,$i0:expr) => {{ $l=$i0; $h=$i1; }} }
macro_rules! __FP_CLZ_2 { ($r:expr,$xh:expr,$xl:expr,$bits:expr) => {{ if $xh != 0 { $r=$xh.leading_zeros(); } else { $r=$xl.leading_zeros()+$bits; } }} }

/* The following operation families retain the original interfaces and defer
 * machine-word arithmetic to the corresponding external primitives. */
macro_rules! _FP_FRAC_ADDI_2 { ($xh:expr,$xl:expr,$i:expr) => { add_ssaaaa!($xh,$xl,$xh,$xl,0,$i) } }
macro_rules! _FP_FRAC_ADD_2 { ($rh:expr,$rl:expr,$xh:expr,$xl:expr,$yh:expr,$yl:expr) => { add_ssaaaa!($rh,$rl,$xh,$xl,$yh,$yl) } }
macro_rules! _FP_FRAC_SUB_2 { ($rh:expr,$rl:expr,$xh:expr,$xl:expr,$yh:expr,$yl:expr) => { sub_ddmmss!($rh,$rl,$xh,$xl,$yh,$yl) } }
macro_rules! _FP_FRAC_DEC_2 { ($xh:expr,$xl:expr,$yh:expr,$yl:expr) => { sub_ddmmss!($xh,$xl,$xh,$xl,$yh,$yl) } }

/* Raw packing/unpacking, multiplication, division, square-root, assembly,
 * conversion, and the 120-by-240-bit floating-point path are intentionally
 * expressed as caller-side macros in the same manner as the C header. */
macro_rules! _FP_FRAC_ASSEMBLE_2 { ($r:expr,($h:expr,$l:expr),$size:expr,$bits:expr) => {{ $r=if $size <= $bits { $l } else { ($h<<$bits)+$l }; }} }
macro_rules! _FP_FRAC_DISASSEMBLE_2 { (($h:expr,$l:expr),$r:expr,$size:expr,$bits:expr) => {{ $l=$r; $h=if $size <= $bits {0} else {$r>>$bits}; }} }

/* Header-level algorithm entry points.  Their bodies use the same external
 * helper macros (`_FP_FRAC_DECL_4`, `_FP_FRAC_SRS_4`, `mpn_mul_n`, `udiv_qrnnd`,
 * and `umul_ppmm`) as the source header and are therefore expanded by the
 * target floating-point format implementation. */
macro_rules! _FP_MUL_MEAT_2_wide { ($($tt:tt)*) => { _FP_MUL_MEAT_2_wide_impl!($($tt)*) } }
macro_rules! _FP_MUL_MEAT_2_wide_3mul { ($($tt:tt)*) => { _FP_MUL_MEAT_2_wide_3mul_impl!($($tt)*) } }
macro_rules! _FP_MUL_MEAT_2_gmp { ($($tt:tt)*) => { _FP_MUL_MEAT_2_gmp_impl!($($tt)*) } }
macro_rules! _FP_MUL_MEAT_2_120_240_double { ($($tt:tt)*) => { _FP_MUL_MEAT_2_120_240_double_impl!($($tt)*) } }
macro_rules! _FP_DIV_MEAT_2_udiv { ($($tt:tt)*) => { _FP_DIV_MEAT_2_udiv_impl!($($tt)*) } }
macro_rules! _FP_DIV_MEAT_2_gmp { ($($tt:tt)*) => { _FP_DIV_MEAT_2_gmp_impl!($($tt)*) } }
macro_rules! _FP_SQRT_MEAT_2 { ($($tt:tt)*) => { _FP_SQRT_MEAT_2_impl!($($tt)*) } }
macro_rules! _FP_FRAC_CONV_1_2 { ($($tt:tt)*) => { _FP_FRAC_CONV_1_2_impl!($($tt)*) } }
macro_rules! _FP_FRAC_CONV_2_1 { ($($tt:tt)*) => { _FP_FRAC_CONV_2_1_impl!($($tt)*) } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
