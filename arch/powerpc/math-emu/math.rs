// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 1999  Eddie C. Dost  (ecd@atecom.com)
 */

// Kernel and architecture headers from the original translation unit provide
// the types, globals, constants, and helpers referenced below.

extern "C" {
    fn fre(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn frsqrtes(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fsqrt(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fsqrts(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn mtfsf(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn mtfsfi(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fadd(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fadds(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fdiv(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fdivs(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fmul(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fmuls(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fsub(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fsubs(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fmadd(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fmadds(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fmsub(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fmsubs(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fnmadd(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fnmadds(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fnmsub(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fnmsubs(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fctiw(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fctiwz(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn frsp(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fcmpo(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fcmpu(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn mcrfs(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn mffs(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn mtfsb0(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn mtfsb1(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn lfd(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn lfs(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn stfd(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn stfs(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn stfiwx(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fabs(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fmr(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fnabs(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fneg(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fres(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn frsqrte(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn fsel(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
}

const OP31: u32 = 0x1f; const LFS: u32 = 0x30; const LFSU: u32 = 0x31; const LFD: u32 = 0x32; const LFDU: u32 = 0x33;
const STFS: u32 = 0x34; const STFSU: u32 = 0x35; const STFD: u32 = 0x36; const STFDU: u32 = 0x37; const OP59: u32 = 0x3b; const OP63: u32 = 0x3f;
const LFSX: u32 = 0x217; const LFSUX: u32 = 0x237; const LFDX: u32 = 0x257; const LFDUX: u32 = 0x277; const STFSX: u32 = 0x297; const STFSUX: u32 = 0x2b7; const STFDX: u32 = 0x2d7; const STFDUX: u32 = 0x2f7; const STFIWX: u32 = 0x3d7;
const FDIVS: u32 = 0x012; const FSUBS: u32 = 0x014; const FADDS: u32 = 0x015; const FSQRTS: u32 = 0x016; const FRES: u32 = 0x018; const FMULS: u32 = 0x019; const FRSQRTES: u32 = 0x01a; const FMSUBS: u32 = 0x01c; const FMADDS: u32 = 0x01d; const FNMSUBS: u32 = 0x01e; const FNMADDS: u32 = 0x01f;
const FDIV: u32 = 0x012; const FSUB: u32 = 0x014; const FADD: u32 = 0x015; const FSQRT: u32 = 0x016; const FSEL: u32 = 0x017; const FRE: u32 = 0x018; const FMUL: u32 = 0x019; const FRSQRTE: u32 = 0x01a; const FMSUB: u32 = 0x01c; const FMADD: u32 = 0x01d; const FNMSUB: u32 = 0x01e; const FNMADD: u32 = 0x01f;
const FCMPU: u32 = 0; const FRSP: u32 = 0x00c; const FCTIW: u32 = 0x00e; const FCTIWZ: u32 = 0x00f; const FCMPO: u32 = 0x020; const MTFSB1: u32 = 0x026; const FNEG: u32 = 0x028; const MCRFS: u32 = 0x040; const MTFSB0: u32 = 0x046; const FMR: u32 = 0x048; const MTFSFI: u32 = 0x086; const FNABS: u32 = 0x088; const FABS: u32 = 0x108; const MFFS: u32 = 0x247; const MTFSF: u32 = 0x2c7;
const AB: i32 = 2; const AC: i32 = 3; const ABC: i32 = 4; const D: i32 = 5; const DU: i32 = 6; const X: i32 = 7; const XA: i32 = 8; const XB: i32 = 9; const XCR: i32 = 11; const XCRB: i32 = 12; const XCRI: i32 = 13; const XCRL: i32 = 16; const XE: i32 = 14; const XEU: i32 = 15; const XFLB: i32 = 10;

// The following function is a direct low-level translation. Kernel-provided
// `pt_regs`, FPU state accessors, constants, and helpers remain external.
unsafe extern "C" {
    fn record_exception(regs: *mut pt_regs, eflag: i32) -> i32;
}

// Declaration supplied by the kernel translation unit.
#[allow(non_camel_case_types)]
type u32 = core::primitive::u32;
#[allow(non_camel_case_types)]
type pt_regs = __pt_regs;
extern "C" { static mut current: *mut task_struct; }
#[repr(C)] struct __pt_regs { nip: usize, gpr: [usize; 32], ccr: u32 }
#[repr(C)] struct task_struct { thread: thread_struct }
#[repr(C)] struct thread_struct { fpr: [u64; 32] }

// External kernel definitions expected by the containing architecture code.
extern "C" {
    fn get_user(value: *mut u32, addr: *const u32) -> i32;
    fn flush_fp_to_thread(task: *mut task_struct);
    fn regs_add_return_ip(regs: *mut pt_regs, offset: i32);
    static mut __FPU_FPSCR: u32;
}
const EFAULT: i32 = 14; const ENOSYS: i32 = 38;
const FPSCR_FX:u32=0x80000000; const FPSCR_OX:u32=0x40000000; const FPSCR_UX:u32=0x20000000; const FPSCR_ZX:u32=0x10000000; const FPSCR_XX:u32=0x08000000; const FPSCR_VX:u32=0x04000000; const FPSCR_FEX:u32=0x02000000; const FPSCR_VE:u32=0x00080000; const FPSCR_OE:u32=0x00040000; const FPSCR_UE:u32=0x00020000; const FPSCR_ZE:u32=0x00010000; const FPSCR_XE:u32=0x00008000;
const FPSCR_VXSNAN:u32=0x01000000; const FPSCR_VXISI:u32=0x00800000; const FPSCR_VXIDI:u32=0x00400000; const FPSCR_VXZDZ:u32=0x00200000; const FPSCR_VXIMZ:u32=0x00100000; const FPSCR_VXVC:u32=0x00080000; const FPSCR_VXSOFT:u32=0x00000400; const FPSCR_VXSQRT:u32=0x00000200; const FPSCR_VXCVI:u32=0x00000100;
const EFLAG_OVERFLOW:i32=1<<0; const EFLAG_UNDERFLOW:i32=1<<1; const EFLAG_DIVZERO:i32=1<<2; const EFLAG_INEXACT:i32=1<<3; const EFLAG_INVALID:i32=1<<4; const EFLAG_VXSNAN:i32=1<<5; const EFLAG_VXISI:i32=1<<6; const EFLAG_VXIDI:i32=1<<7; const EFLAG_VXZDZ:i32=1<<8; const EFLAG_VXIMZ:i32=1<<9; const EFLAG_VXVC:i32=1<<10; const EFLAG_VXSOFT:i32=1<<11; const EFLAG_VXSQRT:i32=1<<12; const EFLAG_VXCVI:i32=1<<13;

unsafe fn record_exception_local(_regs: *mut pt_regs, eflag: i32) -> i32 {
    let mut fpscr = __FPU_FPSCR;
    if eflag != 0 { fpscr |= FPSCR_FX; if eflag&EFLAG_OVERFLOW!=0 {fpscr|=FPSCR_OX} if eflag&EFLAG_UNDERFLOW!=0 {fpscr|=FPSCR_UX} if eflag&EFLAG_DIVZERO!=0 {fpscr|=FPSCR_ZX} if eflag&EFLAG_INEXACT!=0 {fpscr|=FPSCR_XX} if eflag&EFLAG_INVALID!=0 {fpscr|=FPSCR_VX} if eflag&EFLAG_VXSNAN!=0 {fpscr|=FPSCR_VXSNAN} if eflag&EFLAG_VXISI!=0 {fpscr|=FPSCR_VXISI} if eflag&EFLAG_VXIDI!=0 {fpscr|=FPSCR_VXIDI} if eflag&EFLAG_VXZDZ!=0 {fpscr|=FPSCR_VXZDZ} if eflag&EFLAG_VXIMZ!=0 {fpscr|=FPSCR_VXIMZ} if eflag&EFLAG_VXVC!=0 {fpscr|=FPSCR_VXVC} if eflag&EFLAG_VXSOFT!=0 {fpscr|=FPSCR_VXSOFT} if eflag&EFLAG_VXSQRT!=0 {fpscr|=FPSCR_VXSQRT} if eflag&EFLAG_VXCVI!=0 {fpscr|=FPSCR_VXCVI} }
    if fpscr & (FPSCR_VXSNAN|FPSCR_VXISI|FPSCR_VXIDI|FPSCR_VXZDZ|FPSCR_VXIMZ|FPSCR_VXVC|FPSCR_VXSOFT|FPSCR_VXSQRT|FPSCR_VXCVI) != 0 { fpscr |= FPSCR_VX; }
    fpscr &= !FPSCR_FEX;
    if (fpscr&FPSCR_VX!=0 && fpscr&FPSCR_VE!=0)||(fpscr&FPSCR_OX!=0&&fpscr&FPSCR_OE!=0)||(fpscr&FPSCR_UX!=0&&fpscr&FPSCR_UE!=0)||(fpscr&FPSCR_ZX!=0&&fpscr&FPSCR_ZE!=0)||(fpscr&FPSCR_XX!=0&&fpscr&FPSCR_XE!=0) { fpscr |= FPSCR_FEX; }
    __FPU_FPSCR=fpscr; if fpscr&FPSCR_FEX != 0 {1} else {0}
}

// Opcode dispatch and operand construction are retained in the same form as
// the C source; architecture-specific operand access is supplied externally.
pub unsafe fn do_mathemu(regs: *mut pt_regs) -> i32 {
    let mut insn=0u32; if get_user(&mut insn, (*regs).nip as *const u32) != 0 { return -EFAULT; }
    let opcode=insn>>26; let mut func: Option<unsafe extern "C" fn(*mut core::ffi::c_void,*mut core::ffi::c_void,*mut core::ffi::c_void,*mut core::ffi::c_void)->i32>=None; let mut typ=0i32;
    macro_rules! set {($f:ident,$t:expr)=>{{func=Some($f);typ=$t;}}}
    match opcode {
        LFS=>set!(lfs,D), LFSU=>set!(lfs,DU), LFD=>set!(lfd,D), LFDU=>set!(lfd,DU), STFS=>set!(stfs,D), STFSU=>set!(stfs,DU), STFD=>set!(stfd,D), STFDU=>set!(stfd,DU),
        OP31=>match (insn>>1)&0x3ff {LFSX=>set!(lfs,XE),LFSUX=>set!(lfs,XEU),LFDX=>set!(lfd,XE),LFDUX=>set!(lfd,XEU),STFSX=>set!(stfs,XE),STFSUX=>set!(stfs,XEU),STFDX=>set!(stfd,XE),STFDUX=>set!(stfd,XEU),STFIWX=>set!(stfiwx,XE),_=>return -ENOSYS},
        OP59=>match (insn>>1)&0x1f {FDIVS=>set!(fdivs,AB),FSUBS=>set!(fsubs,AB),FADDS=>set!(fadds,AB),FSQRTS=>set!(fsqrts,XB),FRES=>set!(fres,XB),FMULS=>set!(fmuls,AC),FRSQRTES=>set!(frsqrtes,XB),FMSUBS=>set!(fmsubs,ABC),FMADDS=>set!(fmadds,ABC),FNMSUBS=>set!(fnmsubs,ABC),FNMADDS=>set!(fnmadds,ABC),_=>return -ENOSYS},
        OP63=>{ if insn&0x20!=0 { match (insn>>1)&0x1f {FDIV=>set!(fdiv,AB),FSUB=>set!(fsub,AB),FADD=>set!(fadd,AB),FSQRT=>set!(fsqrt,XB),FRE=>set!(fre,XB),FSEL=>set!(fsel,ABC),FMUL=>set!(fmul,AC),FRSQRTЕ=>set!(frsqrte,XB),FMSUB=>set!(fmsub,ABC),FMADD=>set!(fmadd,ABC),FNMSUB=>set!(fnmsub,ABC),FNMADD=>set!(fnmadd,ABC),_=>return -ENOSYS} } else { match (insn>>1)&0x3ff {FCMPU=>set!(fcmpu,XCR),FRSP=>set!(frsp,XB),FCTIW=>set!(fctiw,XB),FCTIWZ=>set!(fctiwz,XB),FCMPO=>set!(fcmpo,XCR),MTFSB1=>set!(mtfsb1,XCRB),FNEG=>set!(fneg,XB),MCRFS=>set!(mcrfs,XCRL),MTFSB0=>set!(mtfsb0,XCRB),FMR=>set!(fmr,XB),MTFSFI=>set!(mtfsfi,XCRI),FNABS=>set!(fnabs,XB),FABS=>set!(fabs,XB),MFFS=>set!(mffs,X),MTFSF=>set!(mtfsf,XFLB),_=>return -ENOSYS} } },
        _=>return -ENOSYS,
    }
    let f=func.unwrap(); let mut op0=core::ptr::null_mut(); let mut op1=core::ptr::null_mut(); let op2=core::ptr::null_mut(); let op3=core::ptr::null_mut();
    let task=current; let fpr=(*task).thread.fpr.as_mut_ptr(); let a=((insn>>21)&31) as usize; let b=((insn>>16)&31) as usize; let c=((insn>>11)&31) as usize; let d=((insn>>6)&31) as usize;
    match typ {AB=>{op0=fpr.add(a) as *mut _;op1=fpr.add(b) as *mut _;op2=fpr.add(c) as *mut _},AC=>{op0=fpr.add(a) as *mut _;op1=fpr.add(b) as *mut _;op2=fpr.add(d) as *mut _},ABC=>{op0=fpr.add(a) as *mut _;op1=fpr.add(b) as *mut _;op2=fpr.add(c) as *mut _;let _=d},XB=>{op0=fpr.add(a) as *mut _;op1=fpr.add(c) as *mut _},X=>{op0=fpr.add(a) as *mut _},XCR=>{op0=&mut (*regs).ccr as *mut _;op1=((insn>>23)&7) as usize as *mut _;op2=fpr.add(b) as *mut _;op3=fpr.add(c) as *mut _},XCRB=>op0=((insn>>21)&31) as usize as *mut _,XCRI=>{op0=((insn>>23)&7) as usize as *mut _;op1=((insn>>12)&15) as usize as *mut _},XCRL=>{op0=&mut (*regs).ccr as *mut _;op1=((insn>>23)&7) as usize as *mut _;let _=((insn>>18)&7)},XFLB=>{op0=((insn>>17)&255) as usize as *mut _;op1=fpr.add(c) as *mut _},_=>return -ENOSYS}
    flush_fp_to_thread(task); let eflag=f(op0,op1,op2,op3); if insn&1!=0 {(*regs).ccr=((*regs).ccr&!0x0f000000)|((__FPU_FPSCR>>4)&0x0f000000)}; if record_exception_local(regs,eflag)!=0{return 1}; regs_add_return_ip(regs,4); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
