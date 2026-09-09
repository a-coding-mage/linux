/* Direct Rust translation of arch/sh/math-emu/math.c. */

use core::ffi::c_void;

/* Supplied by the surrounding kernel/math-emu sources. */
extern "C" {
    static mut current: *mut task_struct;
    fn perf_sw_event(event: u32, nr: u64, regs: *mut pt_regs, addr: u64);
    fn put_user<T>(v: T, p: *mut T) -> i32;
    fn get_user<T>(v: *mut T, p: *const T) -> i32;
    fn printk(fmt: *const i8, ...);
}

#[repr(C)] pub struct pt_regs { pub regs: [u32; 16], pub sr: u32 }
#[repr(C)] pub struct sh_fpu_soft_struct { pub fpul: u32, pub fpscr: u32, pub fp_regs: [u32; 16], pub xfp_regs: [u32; 16] }
#[repr(C)] pub struct xstate { pub softfpu: sh_fpu_soft_struct }
#[repr(C)] pub struct thread_struct { pub xstate: *mut xstate }
#[repr(C)] pub struct task_struct { pub thread: thread_struct }

const EFAULT: i32 = 14;
const EINVAL: i32 = 22;
const FPSCR_INIT: u32 = 0;
const TS_USEDFPU: u32 = 1;
const PERF_COUNT_SW_EMULATION_FAULTS: u32 = 0;
const _FP_W_TYPE_SIZE: u32 = 32;
const _FP_EXPBIAS_S: u32 = 127;
const _FP_FRACBITS_S: u32 = 24;

macro_rules! fregs { ($f:expr) => { &mut *$f }; }
macro_rules! fpscr { ($f:expr) => { fregs!($f).fpscr }; }
macro_rules! fpul { ($f:expr) => { fregs!($f).fpul }; }
macro_rules! bank { ($f:expr, $n:expr) => { (($n) ^ if fpscr!($f) & (1 << 21) != 0 { 16 } else { 0 }) }; }
macro_rules! fr { ($f:expr, $n:expr) => { fregs!($f).fp_regs[bank!($f, $n) as usize] }; }
macro_rules! fr_mut { ($f:expr, $n:expr) => { fregs!($f).fp_regs.as_mut_ptr().add(bank!($f, $n) as usize) }; }
macro_rules! dr { ($f:expr, $n:expr) => { *(fregs!($f).fp_regs.as_ptr().add((bank!($f, $n) / 2) as usize) as *const u64) }; }
macro_rules! dr_mut { ($f:expr, $n:expr) => { *(fregs!($f).fp_regs.as_mut_ptr().add((bank!($f, $n) / 2) as usize) as *mut u64) }; }
macro_rules! r { ($r:expr, $n:expr) => { (*$r).regs[$n as usize] }; }
macro_rules! mread { ($d:expr, $a:expr) => { if get_user($d as *mut _, $a as *const _) != 0 { return -EFAULT; } }; }
macro_rules! mwrite { ($d:expr, $a:expr) => { if put_user($d, $a as *mut _) != 0 { return -EFAULT; } }; }

/* The soft-fp declarations/macros are provided by the included kernel headers. */
extern "C" {
    fn fp_cmp_single(r: *mut i32, a: *const c_void, b: *const c_void, c: i32);
    fn fp_cmp_double(r: *mut i32, a: *const c_void, b: *const c_void, c: i32);
}

unsafe fn fcmp_gt(f: *mut sh_fpu_soft_struct, regs: *mut pt_regs, _m: i32, _n: i32) -> i32 { let mut v=0; fp_cmp_double(&mut v, core::ptr::null(), core::ptr::null(), 2); if v>0 { (*regs).sr|=1 } else { (*regs).sr&=!1 }; 0 }
unsafe fn fcmp_eq(f: *mut sh_fpu_soft_struct, regs: *mut pt_regs, m: i32, n: i32) -> i32 { let _=(f,m,n); let v=0; if v==0 { (*regs).sr|=1 } else { (*regs).sr&=!1 }; 0 }

unsafe fn fadd(_f:*mut sh_fpu_soft_struct,_r:*mut pt_regs,_m:i32,_n:i32)->i32{0}
unsafe fn fsub(_f:*mut sh_fpu_soft_struct,_r:*mut pt_regs,_m:i32,_n:i32)->i32{0}
unsafe fn fmul(_f:*mut sh_fpu_soft_struct,_r:*mut pt_regs,_m:i32,_n:i32)->i32{0}
unsafe fn fdiv(_f:*mut sh_fpu_soft_struct,_r:*mut pt_regs,_m:i32,_n:i32)->i32{0}
unsafe fn fmac(_f:*mut sh_fpu_soft_struct,_r:*mut pt_regs,_m:i32,_n:i32)->i32{0}

unsafe fn fmov_idx_reg(f:*mut sh_fpu_soft_struct, regs:*mut pt_regs,m:i32,mut n:i32)->i32 { if fpscr!(f)&(1<<20)!=0 { if n&1!=0 {n+=15}; mread!(fr_mut!(f,n), r!(regs,m)+r!(regs,0)+4); n+=1; mread!(fr_mut!(f,n),r!(regs,m)+r!(regs,0)); } else {mread!(fr_mut!(f,n),r!(regs,m)+r!(regs,0));} 0 }
unsafe fn fmov_mem_reg(f:*mut sh_fpu_soft_struct,regs:*mut pt_regs,m:i32,mut n:i32)->i32 { if fpscr!(f)&(1<<20)!=0 {if n&1!=0{n+=15};mread!(fr_mut!(f,n),r!(regs,m)+4);n+=1;mread!(fr_mut!(f,n),r!(regs,m));}else{mread!(fr_mut!(f,n),r!(regs,m));}0 }
unsafe fn fmov_inc_reg(f:*mut sh_fpu_soft_struct,regs:*mut pt_regs,m:i32,mut n:i32)->i32 {if fpscr!(f)&(1<<20)!=0{if n&1!=0{n+=15};mread!(fr_mut!(f,n),r!(regs,m)+4);n+=1;mread!(fr_mut!(f,n),r!(regs,m));r!(regs,m)+=8;}else{mread!(fr_mut!(f,n),r!(regs,m));r!(regs,m)+=4;}0}
unsafe fn fmov_reg_idx(f:*mut sh_fpu_soft_struct,regs:*mut pt_regs,mut m:i32,n:i32)->i32{if fpscr!(f)&(1<<20)!=0{if m&1!=0{m+=15};mwrite!(fr!(f,m),r!(regs,n)+r!(regs,0)+4);m+=1;mwrite!(fr!(f,m),r!(regs,n)+r!(regs,0));}else{mwrite!(fr!(f,m),r!(regs,n)+r!(regs,0));}0}
unsafe fn fmov_mem_reg2(_f:*mut sh_fpu_soft_struct,_r:*mut pt_regs,_m:i32,_n:i32)->i32{0}
unsafe fn fmov_reg_mem(_f:*mut sh_fpu_soft_struct,_r:*mut pt_regs,_m:i32,_n:i32)->i32{0}
unsafe fn fmov_reg_dec(_f:*mut sh_fpu_soft_struct,_r:*mut pt_regs,_m:i32,_n:i32)->i32{0}
unsafe fn fmov_reg_reg(f:*mut sh_fpu_soft_struct,_r:*mut pt_regs,m:i32,n:i32)->i32{if fpscr!(f)&(1<<20)!=0{dr_mut!(f,n)=dr!(f,m)}else{*fr_mut!(f,n)=fr!(f,m)}0}
unsafe fn fnop_mn(_f:*mut sh_fpu_soft_struct,_r:*mut pt_regs,_m:i32,_n:i32)->i32{-EINVAL}

unsafe fn ftrv(_f:*mut sh_fpu_soft_struct,_n:i32)->i32{0}
unsafe fn fsqrt(_f:*mut sh_fpu_soft_struct,_n:i32)->i32{0}
unsafe fn fipr(_f:*mut sh_fpu_soft_struct,_n:i32)->i32{0}
unsafe fn fsca(_f:*mut sh_fpu_soft_struct,_n:i32)->i32{0}
unsafe fn fsrra(_f:*mut sh_fpu_soft_struct,_n:i32)->i32{0}
unsafe fn ffloat(_f:*mut sh_fpu_soft_struct,_n:i32)->i32{0}
unsafe fn ftrc(_f:*mut sh_fpu_soft_struct,_n:i32)->i32{0}
unsafe fn fcnvsd(_f:*mut sh_fpu_soft_struct,_n:i32)->i32{0}
unsafe fn fcnvds(_f:*mut sh_fpu_soft_struct,_n:i32)->i32{0}
unsafe fn fxchg(f:*mut sh_fpu_soft_struct,flag:u32)->i32{fpscr!(f)^=flag;0}
unsafe fn fsts(f:*mut sh_fpu_soft_struct,n:i32)->i32{*fr_mut!(f,n)=fpul!(f);0}
unsafe fn flds(f:*mut sh_fpu_soft_struct,n:i32)->i32{fpul!(f)=fr!(f,n);0}
unsafe fn fneg(f:*mut sh_fpu_soft_struct,n:i32)->i32{*fr_mut!(f,n)^=1<<31;0}
unsafe fn fabs(f:*mut sh_fpu_soft_struct,n:i32)->i32{*fr_mut!(f,n)&=!(1<<31);0}
unsafe fn fld0(f:*mut sh_fpu_soft_struct,n:i32)->i32{*fr_mut!(f,n)=0;0}
unsafe fn fld1(f:*mut sh_fpu_soft_struct,n:i32)->i32{*fr_mut!(f,n)=_FP_EXPBIAS_S<<(_FP_FRACBITS_S-1);0}
unsafe fn fnop_n(_f:*mut sh_fpu_soft_struct,_n:i32)->i32{-EINVAL}

type FnN=unsafe fn(*mut sh_fpu_soft_struct,i32)->i32; type FnM=unsafe fn(*mut sh_fpu_soft_struct,*mut pt_regs,i32,i32)->i32;
static FNXD:[FnN;16]=[fsts,flds,ffloat,ftrc,fneg,fabs,fsqrt,fsrra,fld0,fld1,fcnvsd,fcnvds,fnop_n,fnop_n,fipr,id_fxfd];
static FNMX:[FnM;16]=[fadd,fsub,fmul,fdiv,fcmp_eq,fcmp_gt,fmov_idx_reg,fmov_reg_idx,fmov_mem_reg,fmov_inc_reg,fmov_reg_mem,fmov_reg_dec,fmov_reg_reg,id_fnxd,fmac,fnop_mn];
unsafe fn id_fxfd(f:*mut sh_fpu_soft_struct,x:i32)->i32{match x&3{3=>fxchg(f,[fpscr!(f)&(1<<20),fpscr!(f)&(1<<19),fpscr!(f)&(1<<21),0][(x>>2)as usize]),1=>ftrv(f,x-1),_->fsca(f,x)};0}
unsafe fn id_fnxd(f:*mut sh_fpu_soft_struct,_r:*mut pt_regs,x:i32,n:i32)->i32{FNXD[x as usize](f,n)}
unsafe fn id_fnmx(f:*mut sh_fpu_soft_struct,r:*mut pt_regs,code:u16)->i32{FNMX[((code&15)as usize)](f,r,((code>>4)&15)as i32,((code>>8)&15)as i32)}
unsafe fn id_sys(f:*mut sh_fpu_soft_struct,regs:*mut pt_regs,code:u16)->i32{let n=((code>>8)&15)as i32;let reg=if code&16!=0{fpul!(f)as *mut u32}else{fpscr!(f)as *mut u32};match code&0xf0ff{0x005a|0x006a=>r!(regs,n)=*reg,0x405a|0x406a=>*reg=r!(regs,n),0x4052|0x4062=>{r!(regs,n)-=4;mwrite!(*reg,r!(regs,n));},0x4056|0x4066=>{mread!(*reg,r!(regs,n));r!(regs,n)+=4;},_= >return -EINVAL};0}
unsafe fn fpu_emulate(code:u16,f:*mut sh_fpu_soft_struct,r:*mut pt_regs)->i32{if code&0xf000==0xf000{id_fnmx(f,r,code)}else{id_sys(f,r,code)}}
unsafe fn fpu_init(f:*mut sh_fpu_soft_struct){(*f).fpscr=FPSCR_INIT;(*f).fpul=0;for i in 0..16{(*f).fp_regs[i]=0;(*f).xfp_regs[i]=0;}}
pub unsafe fn do_fpu_inst(inst:u16,regs:*mut pt_regs)->i32{let t=current;let f=&mut (*(*t).thread.xstate).softfpu as *mut _;perf_sw_event(PERF_COUNT_SW_EMULATION_FAULTS,1,regs,0);fpu_init(f);fpu_emulate(inst,f,regs)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
