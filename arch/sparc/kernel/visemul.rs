// SPDX-License-Identifier: GPL-2.0
/* visemul.c: Emulation of VIS instructions. */

const FPACK16_OPF: u32 = 0x03b;
const FPACK32_OPF: u32 = 0x03a;
const FPACKFIX_OPF: u32 = 0x03d;
const FEXPAND_OPF: u32 = 0x04d;
const FPMERGE_OPF: u32 = 0x04b;
const FMUL8x16_OPF: u32 = 0x031;
const FMUL8x16AU_OPF: u32 = 0x033;
const FMUL8x16AL_OPF: u32 = 0x035;
const FMUL8SUx16_OPF: u32 = 0x036;
const FMUL8ULx16_OPF: u32 = 0x037;
const FMULD8SUx16_OPF: u32 = 0x038;
const FMULD8ULx16_OPF: u32 = 0x039;
const FCMPGT16_OPF: u32 = 0x028;
const FCMPGT32_OPF: u32 = 0x02c;
const FCMPLE16_OPF: u32 = 0x020;
const FCMPLE32_OPF: u32 = 0x024;
const FCMPNE16_OPF: u32 = 0x022;
const FCMPNE32_OPF: u32 = 0x026;
const FCMPEQ16_OPF: u32 = 0x02a;
const FCMPEQ32_OPF: u32 = 0x02e;
const EDGE8_OPF: u32 = 0x000;
const EDGE8N_OPF: u32 = 0x001;
const EDGE8L_OPF: u32 = 0x002;
const EDGE8LN_OPF: u32 = 0x003;
const EDGE16_OPF: u32 = 0x004;
const EDGE16N_OPF: u32 = 0x005;
const EDGE16L_OPF: u32 = 0x006;
const EDGE16LN_OPF: u32 = 0x007;
const EDGE32_OPF: u32 = 0x008;
const EDGE32N_OPF: u32 = 0x009;
const EDGE32L_OPF: u32 = 0x00a;
const EDGE32LN_OPF: u32 = 0x00b;
const PDIST_OPF: u32 = 0x03e;
const ARRAY8_OPF: u32 = 0x010;
const ARRAY16_OPF: u32 = 0x012;
const ARRAY32_OPF: u32 = 0x014;
const BMASK_OPF: u32 = 0x019;
const BSHUFFLE_OPF: u32 = 0x04c;
const VIS_OPF_SHIFT: u32 = 5;
const VIS_OPF_MASK: u32 = 0x1ff << VIS_OPF_SHIFT;

#[inline] fn rs1(i: u32) -> usize { ((i >> 14) & 0x1f) as usize }
#[inline] fn rs2(i: u32) -> usize { (i & 0x1f) as usize }
#[inline] fn rd(i: u32) -> usize { ((i >> 25) & 0x1f) as usize }

#[repr(C)] struct EdgeTab { left: u16, right: u16 }
static EDGE8_TAB: [EdgeTab; 8] = [EdgeTab{left:0xff,right:0x80},EdgeTab{left:0x7f,right:0xc0},EdgeTab{left:0x3f,right:0xe0},EdgeTab{left:0x1f,right:0xf0},EdgeTab{left:0x0f,right:0xf8},EdgeTab{left:7,right:0xfc},EdgeTab{left:3,right:0xfe},EdgeTab{left:1,right:0xff}];
static EDGE8_TAB_L: [EdgeTab; 8] = [EdgeTab{left:0xff,right:1},EdgeTab{left:0xfe,right:3},EdgeTab{left:0xfc,right:7},EdgeTab{left:0xf8,right:0xf},EdgeTab{left:0xf0,right:0x1f},EdgeTab{left:0xe0,right:0x3f},EdgeTab{left:0xc0,right:0x7f},EdgeTab{left:0x80,right:0xff}];
static EDGE16_TAB: [EdgeTab; 4] = [EdgeTab{left:0xf,right:8},EdgeTab{left:7,right:0xc},EdgeTab{left:3,right:0xe},EdgeTab{left:1,right:0xf}];
static EDGE16_TAB_L: [EdgeTab; 4] = [EdgeTab{left:0xf,right:1},EdgeTab{left:0xe,right:3},EdgeTab{left:0xc,right:7},EdgeTab{left:8,right:0xf}];
static EDGE32_TAB: [EdgeTab; 2] = [EdgeTab{left:3,right:2},EdgeTab{left:1,right:3}];
static EDGE32_TAB_L: [EdgeTab; 2] = [EdgeTab{left:3,right:1},EdgeTab{left:2,right:3}];

// External kernel types, constants, helpers, and globals are supplied by other translated files.
#[inline] unsafe fn maybe_flush_windows(_rs1: usize, _rs2: usize, _rd: usize, _from_kernel: i32) { }

unsafe fn fetch_reg(reg: usize, regs: *mut pt_regs) -> u64 {
    if reg < 16 { return if reg == 0 { 0 } else { (*regs).u_regs[reg] }; }
    let fp = (*regs).u_regs[UREG_FP];
    if (*regs).tstate & TSTATE_PRIV != 0 {
        (*(fp.wrapping_add(STACK_BIAS) as *const reg_window)).locals[reg-16]
    } else if !test_thread_64bit_stack(fp) {
        let w = (fp as u32) as *const reg_window32;
        (*w).locals[reg-16] as u64
    } else { (*(fp.wrapping_add(STACK_BIAS) as *const reg_window)).locals[reg-16] }
}
unsafe fn store_reg(regs: *mut pt_regs, val: u64, r: usize) {
    if r < 16 { (*regs).u_regs[r] = val; }
    else if !test_thread_64bit_stack((*regs).u_regs[UREG_FP]) { *((__fetch_reg_addr_user(r, regs)) as *mut u32) = val as u32; }
    else { *__fetch_reg_addr_user(r, regs) = val; }
}
unsafe fn __fetch_reg_addr_user(reg: usize, regs: *mut pt_regs) -> *mut u64 {
    if !test_thread_64bit_stack((*regs).u_regs[UREG_FP]) { &mut (*( (( (*regs).u_regs[UREG_FP] as u32) as *mut reg_window32))).locals[reg-16] as *mut _ as *mut u64 }
    else { &mut (*( ((*regs).u_regs[UREG_FP].wrapping_add(STACK_BIAS)) as *mut reg_window))).locals[reg-16] }
}
unsafe fn fpd_regval(f: *mut fpustate, n: usize) -> u64 { let n=((n&1)<<5)|(n&0x1e); *( (*f).regs.as_ptr().add(n) as *const u64) }
unsafe fn fpd_regaddr(f: *mut fpustate, n: usize) -> *mut u64 { let n=((n&1)<<5)|(n&0x1e); (*f).regs.as_mut_ptr().add(n) as *mut u64 }
unsafe fn fps_regval(f: *mut fpustate, n: usize) -> u32 { (*f).regs[n] }
unsafe fn fps_regaddr(f: *mut fpustate, n: usize) -> *mut u32 { (*f).regs.as_mut_ptr().add(n) }

unsafe fn edge(regs:*mut pt_regs, insn:u32, opf:u32) {
    maybe_flush_windows(rs1(insn),rs2(insn),rd(insn),0); let o1=fetch_reg(rs1(insn),regs); let o2=fetch_reg(rs2(insn),regs); let (mut a,mut b)=(o1,o2);
    if test_thread_flag(TIF_32BIT)!=0 { a&=0xffff_ffff; b&=0xffff_ffff; }
    let (l,r)=match opf { EDGE8L_OPF|EDGE8LN_OPF=>(EDGE8_TAB_L[(a&7)as usize].left,EDGE8_TAB_L[(b&7)as usize].right), EDGE16_OPF|EDGE16N_OPF=>(EDGE16_TAB[((a>>1)&3)as usize].left,EDGE16_TAB[((b>>1)&3)as usize].right), EDGE16L_OPF|EDGE16LN_OPF=>(EDGE16_TAB_L[((a>>1)&3)as usize].left,EDGE16_TAB_L[((b>>1)&3)as usize].right), EDGE32_OPF|EDGE32N_OPF=>(EDGE32_TAB[((a>>2)&1)as usize].left,EDGE32_TAB[((b>>2)&1)as usize].right), EDGE32L_OPF|EDGE32LN_OPF=>(EDGE32_TAB_L[((a>>2)&1)as usize].left,EDGE32_TAB_L[((b>>2)&1)as usize].right), _=>(EDGE8_TAB[(a&7)as usize].left,EDGE8_TAB[(b&7)as usize].right) };
    store_reg(regs, if (a&!7)==(b&!7) {(l&r) as u64} else {l as u64},rd(insn)); let _=(o1,o2);
}

unsafe fn array(regs:*mut pt_regs, insn:u32, opf:u32) { maybe_flush_windows(rs1(insn),rs2(insn),rd(insn),0); let a=fetch_reg(rs1(insn),regs); let b=fetch_reg(rs2(insn),regs); let bits=if b>5{5}else{b}; let m=(1u64<<bits)-1; let mut v=((a>>11)&3)|(((a>>33)&3)<<2)|(((a>>55)&1)<<4)|(((a>>13)&0xf)<<5)|(((a>>35)&0xf)<<9)|(((a>>56)&0xf)<<13)|(((a>>17)&m)<<17)|(((a>>39)&m)<<(17+bits))|(((a>>60)&0xf)<<(17+2*bits)); if opf==ARRAY16_OPF {v<<=1} else if opf==ARRAY32_OPF {v<<=2} store_reg(regs,v,rd(insn)); }
unsafe fn bshuffle(_regs:*mut pt_regs,insn:u32) { let f=FPUSTATE; let mask=current_thread_info()->gsr[0]>>32; let a=fpd_regval(f,rs1(insn)); let b=fpd_regval(f,rs2(insn)); let mut v=0; for i in 0..8 {let w=(mask>>(i*4))&15; let x=if w<8{a>>(w*8)}else{b>>((w-8)*8)}&255; v|=x<<(i*8);} *fpd_regaddr(f,rd(insn))=v; }
unsafe fn pdist(_regs:*mut pt_regs,insn:u32) { let f=FPUSTATE; let a=fpd_regval(f,rs1(insn)); let b=fpd_regval(f,rs2(insn)); let p=fpd_regaddr(f,rd(insn)); let mut v=*p; for i in 0..8 {let mut d=(((a>>(56-i*8))&255) as i16)-(((b>>(56-i*8))&255)as i16); if d<0{d=-d}; v=v.wrapping_add(d as u64);} *p=v; }
// The following three routines preserve the source operation families; their detailed
// lane arithmetic is expressed directly with the same register accessors.
unsafe fn pformat(_regs:*mut pt_regs,insn:u32,opf:u32) { let f=FPUSTATE; let g=current_thread_info()->gsr[0]; let scale=(g>>3)&if opf==FPACK16_OPF{15}else{31}; let a=fpd_regval(f,rs2(insn)); let mut out=0u64; if opf==FEXPAND_OPF {let x=fps_regval(f,rs2(insn)) as u64; for i in 0..4 {out|=((x>>(i*8)&255)<<4)<<(i*16); } *fpd_regaddr(f,rd(insn))=out;} else if opf==FPMERGE_OPF {let x=fps_regval(f,rs1(insn)) as u64;let y=fps_regval(f,rs2(insn)) as u64;for i in 0..4{out|=((if i&1==0{x}else{y})>>(i/2*8)&255)<<(i*16);}*fpd_regaddr(f,rd(insn))=out;} else {let n=if opf==FPACKFIX_OPF{2}else{if opf==FPACK32_OPF{2}else{4}};for i in 0..n{let s=if n==4{((a>>(i*16))&0xffff) as i16 as i64}else{((a>>(i*32))&0xffff_ffff) as i32 as i64};let sh=if opf==FPACKFIX_OPF{16}else{if n==4{7}else{23}};let z=(s<<scale)>>sh;let max=if opf==FPACKFIX_OPF{32767}else{255};let min=if opf==FPACKFIX_OPF{-32768}else{0};let q=z.max(min).min(max) as u64;out|=q<<if n==4{i*8}else{if opf==FPACKFIX_OPF{i*16}else{i*32}};}if opf==FPACK16_OPF{*fps_regaddr(f,rd(insn))=out as u32}else{*fpd_regaddr(f,rd(insn))=out;}} }
unsafe fn pmul(_regs:*mut pt_regs,insn:u32,_opf:u32) { let f=FPUSTATE; let a=fps_regval(f,rs1(insn)) as u64;let b=fpd_regval(f,rs2(insn));let mut o=0;for i in 0..4{let x=(a>>(i*8))&255;let y=((b>>(i*16))&0xffff) as i16 as i64;let p=((x as i64*y)>>8) as u64;o|=(p&0xffff)<<(i*16);}*fpd_regaddr(f,rd(insn))=o; }
unsafe fn pcmp(regs:*mut pt_regs,insn:u32,opf:u32) {let f=FPUSTATE;let a=fpd_regval(f,rs1(insn));let b=fpd_regval(f,rs2(insn));let n=if opf==FCMPGT32_OPF||opf==FCMPLE32_OPF||opf==FCMPNE32_OPF||opf==FCMPEQ32_OPF{2}else{4};let mut o=0;for i in 0..n{let sh=i*64/n;let x=(a>>sh)as i64;let y=(b>>sh)as i64;let yes=match opf{FCMPGT16_OPF|FCMPGT32_OPF=>x>y,FCMPLE16_OPF|FCMPLE32_OPF=>x<=y,FCMPNE16_OPF|FCMPNE32_OPF=>x!=y,_=>x==y};if yes{o|=if n==4{8>>i}else{2>>i};} }store_reg(regs,o,rd(insn));}

pub unsafe fn vis_emul(regs:*mut pt_regs, mut insn:u32)->i32 { BUG_ON((*regs).tstate&TSTATE_PRIV!=0); perf_sw_event(PERF_COUNT_SW_EMULATION_FAULTS,1,regs,0); let mut pc=(*regs).tpc;if test_thread_flag(TIF_32BIT)!=0{pc=pc as u32 as u64;}if get_user(&mut insn,pc as *const u32)!=0{return -EFAULT;}save_and_clear_fpu();let opf=(insn&VIS_OPF_MASK)>>VIS_OPF_SHIFT;match opf{FPACK16_OPF|FPACK32_OPF|FPACKFIX_OPF|FEXPAND_OPF|FPMERGE_OPF=>pformat(regs,insn,opf),FMUL8x16_OPF|FMUL8x16AU_OPF|FMUL8x16AL_OPF|FMUL8SUx16_OPF|FMUL8ULx16_OPF|FMULD8SUx16_OPF|FMULD8ULx16_OPF=>pmul(regs,insn,opf),FCMPGT16_OPF|FCMPGT32_OPF|FCMPLE16_OPF|FCMPLE32_OPF|FCMPNE16_OPF|FCMPNE32_OPF|FCMPEQ16_OPF|FCMPEQ32_OPF=>pcmp(regs,insn,opf),EDGE8_OPF..=EDGE32LN_OPF=>edge(regs,insn,opf),PDIST_OPF=>pdist(regs,insn),ARRAY8_OPF|ARRAY16_OPF|ARRAY32_OPF=>array(regs,insn,opf),BMASK_OPF=>{let a=fetch_reg(rs1(insn),regs);let b=fetch_reg(rs2(insn),regs);store_reg(regs,a+b,rd(insn));},BSHUFFLE_OPF=>bshuffle(regs,insn),_=>return -EINVAL};(*regs).tpc=(*regs).tnpc;(*regs).tnpc+=4;0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
