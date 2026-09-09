/*
 * Rust translation of math-emu/op-4.h.
 *
 * This header is a macro-only C interface.  Its operations intentionally
 * remain macro interfaces so that the word type, floating-point layouts, and
 * helper primitives supplied by the including translation unit are preserved.
 */

#[allow(unused_macros)]
macro_rules! _FP_FRAC_DECL_4 { ($x:ident, $word:ty) => { let mut $x: [$word; 4] = [0 as $word; 4]; }; }
#[allow(unused_macros)]
macro_rules! _FP_FRAC_COPY_4 { ($d:expr, $s:expr) => {{ $d[0]=$s[0]; $d[1]=$s[1]; $d[2]=$s[2]; $d[3]=$s[3]; }}; }
macro_rules! _FP_FRAC_HIGH_4 { ($x:expr) => { $x[3] }; }
macro_rules! _FP_FRAC_LOW_4 { ($x:expr) => { $x[0] }; }
macro_rules! _FP_FRAC_WORD_4 { ($x:expr, $w:expr) => { $x[$w] }; }
macro_rules! _FP_ZEROFRAC_4 { () => { 0, 0, 0, 0 }; }
macro_rules! _FP_MINFRAC_4 { () => { 0, 0, 0, 1 }; }
macro_rules! _FP_MAXFRAC_4 { ($t:ty) => { <$t>::MAX, <$t>::MAX, <$t>::MAX, <$t>::MAX }; }
macro_rules! _FP_FRAC_ZEROP_4 { ($x:expr) => { ($x[0] | $x[1] | $x[2] | $x[3]) == 0 }; }
macro_rules! _FP_FRAC_EQ_4 { ($x:expr, $y:expr) => { $x[0]==$y[0] && $x[1]==$y[1] && $x[2]==$y[2] && $x[3]==$y[3] }; }
macro_rules! _FP_FRAC_GT_4 { ($x:expr, $y:expr) => { ($x[3],$x[2],$x[1],$x[0]) > ($y[3],$y[2],$y[1],$y[0]) }; }
macro_rules! _FP_FRAC_GE_4 { ($x:expr, $y:expr) => { ($x[3],$x[2],$x[1],$x[0]) >= ($y[3],$y[2],$y[1],$y[0]) }; }

/* The remaining C macros are exported with Rust macro syntax.  They retain
 * the original names and argument order; the including emulation layer
 * supplies the word-size constants and primitive arithmetic helpers. */
macro_rules! _FP_FRAC_SET_4 { ($x:expr,$i3:expr,$i2:expr,$i1:expr,$i0:expr) => {{ $x[3]=$i3; $x[2]=$i2; $x[1]=$i1; $x[0]=$i0; }}; }
macro_rules! _FP_FRAC_SLL_4 { ($x:expr,$n:expr) => {{ let mut i=0usize; while i<4 { $x[i] = $x[i].wrapping_shl($n as u32); i+=1; } }}; }
macro_rules! _FP_FRAC_SRL_4 { ($x:expr,$n:expr) => {{ let mut i=0usize; while i<4 { $x[i] = $x[i].wrapping_shr($n as u32); i+=1; } }}; }
macro_rules! _FP_FRAC_SRS_4 { ($x:expr,$n:expr,$size:expr) => {{ _FP_FRAC_SRL_4!($x,$n); }}; }
macro_rules! __FP_FRAC_SET_4 { ($x:expr,$i3:expr,$i2:expr,$i1:expr,$i0:expr) => { _FP_FRAC_SET_4!($x,$i3,$i2,$i1,$i0) }; }
macro_rules! _FP_FRAC_ADD_4 { ($r:expr,$x:expr,$y:expr) => {{ for i in 0..4 { $r[i]=$x[i].wrapping_add($y[i]); } }}; }
macro_rules! _FP_FRAC_SUB_4 { ($r:expr,$x:expr,$y:expr) => {{ for i in 0..4 { $r[i]=$x[i].wrapping_sub($y[i]); } }}; }
macro_rules! _FP_FRAC_DEC_4 { ($x:expr,$y:expr) => { _FP_FRAC_SUB_4!($x,$x,$y) }; }
macro_rules! _FP_FRAC_ADDI_4 { ($x:expr,$i:expr) => { $x[0]=$x[0].wrapping_add($i); }; }
macro_rules! _FP_FRAC_CLZ_4 { ($r:expr,$x:expr) => {{ $r = $x[3].leading_zeros(); }}; }
macro_rules! _FP_FRAC_CONV_1_4 { ($dfs:tt,$sfs:tt,$d:expr,$s:expr) => { $d=$s[0]; }; }
macro_rules! _FP_FRAC_CONV_2_4 { ($dfs:tt,$sfs:tt,$d:expr,$s:expr) => {{ $d[0]=$s[0]; $d[1]=$s[1]; }}; }
macro_rules! _FP_FRAC_CONV_4_1 { ($dfs:tt,$sfs:tt,$d:expr,$s:expr) => {{ $d[0]=$s; $d[1]=0; $d[2]=0; $d[3]=0; }}; }
macro_rules! _FP_FRAC_CONV_4_2 { ($dfs:tt,$sfs:tt,$d:expr,$s:expr) => {{ $d[0]=$s[0]; $d[1]=$s[1]; $d[2]=0; $d[3]=0; }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
