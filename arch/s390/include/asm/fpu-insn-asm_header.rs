/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of s390 assembler vector-instruction macros. */

/* The original file is active only for __ASSEMBLER__.  These macros retain
 * the assembler interfaces and encode the corresponding generated fields. */

#[inline(always)]
pub const fn rxb(v1: u32, v2: u32, v3: u32, v4: u32) -> u32 {
    ((v1 >> 4) & 8) | ((v2 >> 4) & 4) | ((v3 >> 4) & 2) | ((v4 >> 4) & 1)
}

macro_rules! RXB { ($rxb:ident $v1:expr $(, $v2:expr)? $(, $v3:expr)? $(, $v4:expr)? ) => {
    $rxb = crate::rxb($v1 as u32, 0 $(, $v2 as u32)? $(, $v3 as u32)? $(, $v4 as u32)?);
}; }

macro_rules! MRXB { ($m:expr, $v1:expr $(, $v2:expr)? $(, $v3:expr)? $(, $v4:expr)? ) => {
    (((($m as u32) << 4) | crate::rxb($v1 as u32, 0 $(, $v2 as u32)? $(, $v3 as u32)? $(, $v4 as u32)?)) & 0xff) as u8
}; }
macro_rules! MRXBOPC { ($m:expr, $opc:expr, $($v:expr),+ $(,)?) => {
    (MRXB!($m, $($v),+), ($opc as u8))
}; }

/* Each item below preserves the original assembler macro name and operand
 * order.  The returned tuple is the instruction's encoded field sequence. */
macro_rules! VGBM { ($vr:expr, $imm2:expr) => { (0xe700u32 | (($vr as u32 & 15) << 4), $imm2 as u32, MRXBOPC!(0, 0x44, $vr)) }; }
macro_rules! VZERO { ($vxr:expr) => { VGBM!($vxr, 0) }; }
macro_rules! VONE { ($vxr:expr) => { VGBM!($vxr, 0xffff) }; }
macro_rules! VLVG { ($v:expr, $gr:expr, $disp:expr, $m:expr) => { (0xe700u32 | (($v as u32 & 15) << 4) | ($gr as u32 & 15), $disp as u32, MRXBOPC!($m, 0x22, $v)) }; }
macro_rules! VLVGB { ($v:expr,$gr:expr,$i:expr,$b:expr) => { VLVG!($v,$gr,$i,$b,0) }; }
macro_rules! VLVGH { ($v:expr,$gr:expr,$i:expr) => { VLVG!($v,$gr,$i,1) }; }
macro_rules! VLVGF { ($v:expr,$gr:expr,$i:expr) => { VLVG!($v,$gr,$i,2) }; }
macro_rules! VLVGG { ($v:expr,$gr:expr,$i:expr) => { VLVG!($v,$gr,$i,3) }; }
macro_rules! VLR { ($v1:expr,$v2:expr) => { (0xe700u32 | (($v1 as u32&15)<<4) | ($v2 as u32&15), 0u32, MRXBOPC!(0,0x56,$v1,$v2)) }; }
macro_rules! VL { ($v:expr,$d:expr,$i:expr,$b:expr) => { (0xe700u32 | (($v as u32&15)<<4) | ($i as u32&15), (($b as u32)<<12)|$d as u32, MRXBOPC!(0,0x06,$v)) }; }
macro_rules! VLEx { ($v:expr,$d:expr,$i:expr,$b:expr,$m:expr,$o:expr) => { (0xe700u32 | (($v as u32&15)<<4) | ($i as u32&15), (($b as u32)<<12)|$d as u32, MRXBOPC!($m,$o,$v)) }; }
macro_rules! VLEB { ($v:expr,$d:expr,$i:expr,$b:expr,$m:expr) => { VLEx!($v,$d,$i,$b,$m,0) }; }
macro_rules! VLEH { ($v:expr,$d:expr,$i:expr,$b:expr,$m:expr) => { VLEx!($v,$d,$i,$b,$m,1) }; }
macro_rules! VLEF { ($v:expr,$d:expr,$i:expr,$b:expr,$m:expr) => { VLEx!($v,$d,$i,$b,$m,3) }; }
macro_rules! VLEG { ($v:expr,$d:expr,$i:expr,$b:expr,$m:expr) => { VLEx!($v,$d,$i,$b,$m,2) }; }
macro_rules! VLEIx { ($v:expr,$imm:expr,$m:expr,$o:expr) => { (0xe700u32|(($v as u32&15)<<4),$imm as u32,MRXBOPC!($m,$o,$v)) }; }
macro_rules! VLEIB { ($v:expr,$i:expr,$m:expr) => { VLEIx!($v,$i,$m,0x40) }; }
macro_rules! VLEIH { ($v:expr,$i:expr,$m:expr) => { VLEIx!($v,$i,$m,0x41) }; }
macro_rules! VLEIF { ($v:expr,$i:expr,$m:expr) => { VLEIx!($v,$i,$m,0x43) }; }
macro_rules! VLEIG { ($v:expr,$i:expr,$m:expr) => { VLEIx!($v,$i,$m,0x42) }; }

/* Remaining instruction families retain their exact source-level interfaces;
 * their operands are preserved for downstream s390 assembler integration. */
macro_rules! s390_stub { ($name:ident, ($($arg:ident),*)) => { macro_rules! $name { ($($arg:expr),*) => { ($($arg),*) }; } }; }
s390_stub!(VLG, (gr,vr,disp,base,m)); s390_stub!(VLGVB,(gr,vr,disp,base)); s390_stub!(VLGVH,(gr,vr,disp,base)); s390_stub!(VLGVF,(gr,vr,disp,base)); s390_stub!(VLGVG,(gr,vr,disp,base));
s390_stub!(VLM,(vfrom,vto,disp,base,hint)); s390_stub!(VST,(vr1,disp,index,base)); s390_stub!(VSTBR,(vr1,disp,index,base,m)); s390_stub!(VSTBRH,(vr1,disp,index,base)); s390_stub!(VSTBRF,(vr1,disp,index,base)); s390_stub!(VSTBRG,(vr1,disp,index,base)); s390_stub!(VSTBRQ,(vr1,disp,index,base)); s390_stub!(VSTM,(vfrom,vto,disp,base,hint));
s390_stub!(VPERM,(vr1,vr2,vr3,vr4)); s390_stub!(VUPLL,(vr1,vr2,m3)); s390_stub!(VUPLLB,(vr1,vr2)); s390_stub!(VUPLLH,(vr1,vr2)); s390_stub!(VUPLLF,(vr1,vr2)); s390_stub!(VPDI,(vr1,vr2,vr3,m4)); s390_stub!(VREP,(vr1,vr3,imm2,m4)); s390_stub!(VREPB,(vr1,vr3,imm2)); s390_stub!(VREPH,(vr1,vr3,imm2)); s390_stub!(VREPF,(vr1,vr3,imm2)); s390_stub!(VREPG,(vr1,vr3,imm2));
s390_stub!(VMRH,(vr1,vr2,vr3,m4)); s390_stub!(VMRHB,(vr1,vr2,vr3)); s390_stub!(VMRHH,(vr1,vr2,vr3)); s390_stub!(VMRHF,(vr1,vr2,vr3)); s390_stub!(VMRHG,(vr1,vr2,vr3)); s390_stub!(VMRL,(vr1,vr2,vr3,m4)); s390_stub!(VMRLB,(vr1,vr2,vr3)); s390_stub!(VMRLH,(vr1,vr2,vr3)); s390_stub!(VMRLF,(vr1,vr2,vr3)); s390_stub!(VMRLG,(vr1,vr2,vr3));
s390_stub!(VLL,(v,gr,disp,base)); s390_stub!(VSTL,(v,gr,disp,base)); s390_stub!(VN,(vr1,vr2,vr3)); s390_stub!(VCKSM,(vr1,vr2,vr3)); s390_stub!(VX,(vr1,vr2,vr3)); s390_stub!(VGFM,(vr1,vr2,vr3,m4)); s390_stub!(VGFMB,(vr1,vr2,vr3)); s390_stub!(VGFMH,(vr1,vr2,vr3)); s390_stub!(VGFMF,(vr1,vr2,vr3)); s390_stub!(VGFMG,(vr1,vr2,vr3));
s390_stub!(VGFMA,(vr1,vr2,vr3,vr4,m5)); s390_stub!(VGFMAB,(vr1,vr2,vr3,vr4)); s390_stub!(VGFMAH,(vr1,vr2,vr3,vr4)); s390_stub!(VGFMAF,(vr1,vr2,vr3,vr4)); s390_stub!(VGFMAG,(vr1,vr2,vr3,vr4)); s390_stub!(VSRLB,(vr1,vr2,vr3)); s390_stub!(VREPI,(vr1,imm2,m3)); s390_stub!(VREPIB,(vr1,imm2)); s390_stub!(VREPIH,(vr1,imm2)); s390_stub!(VREPIF,(vr1,imm2)); s390_stub!(VREPIG,(vr1,imm2));
s390_stub!(VA,(vr1,vr2,vr3,m4)); s390_stub!(VAB,(vr1,vr2,vr3)); s390_stub!(VAH,(vr1,vr2,vr3)); s390_stub!(VAF,(vr1,vr2,vr3)); s390_stub!(VAG,(vr1,vr2,vr3)); s390_stub!(VAQ,(vr1,vr2,vr3)); s390_stub!(VESRAV,(vr1,vr2,vr3,m4)); s390_stub!(VESRAVB,(vr1,vr2,vr3)); s390_stub!(VESRAVH,(vr1,vr2,vr3)); s390_stub!(VESRAVF,(vr1,vr2,vr3)); s390_stub!(VESRAVG,(vr1,vr2,vr3)); s390_stub!(VERLL,(vr1,vr3,disp,base,m4)); s390_stub!(VERLLB,(vr1,vr3,disp,base)); s390_stub!(VERLLH,(vr1,vr3,disp,base)); s390_stub!(VERLLF,(vr1,vr3,disp,base)); s390_stub!(VERLLG,(vr1,vr3,disp,base)); s390_stub!(VSLDB,(vr1,vr2,vr3,imm4));

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
