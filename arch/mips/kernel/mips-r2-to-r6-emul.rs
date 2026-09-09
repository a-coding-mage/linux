/* Rust translation of mips-r2-to-r6-emul.c.  Kernel-provided symbols are
 * intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_void};

pub type u32 = u32;
pub type u64 = u64;
pub type s32 = i32;
pub type s64 = i64;

#[repr(C)]
pub struct pt_regs {
    pub regs: [u64; 32],
    pub hi: u64,
    pub lo: u64,
    pub cp0_epc: u64,
    pub cp0_cause: u64,
}

#[repr(C)]
pub struct r2_decoder_table {
    pub mask: u32,
    pub code: u32,
    pub func: Option<unsafe extern "C" fn(*mut pt_regs, u32) -> i32>,
}

extern "C" {
    pub static fpucondbit: [u32; 8];
    fn compute_return_epc(regs: *mut pt_regs) -> i32;
    fn __compute_return_epc(regs: *mut pt_regs) -> i32;
    fn mips_dsemul(regs: *mut pt_regs, ir: u32, epc: u64, cepc: u64) -> i32;
    fn delay_slot(regs: *const pt_regs) -> bool;
    fn do_trap_or_bp(regs: *mut pt_regs, a: i32, b: i32, name: *const c_char);
}

pub static mut mipsr2_emulation: i32 = 0;
const MIPS_R2_EMUL_TOTAL_PASS: i32 = 10;

/* Instruction-field helpers supplied by the kernel headers in the C build. */
#[inline] fn opcode(i: u32) -> u32 { (i >> 26) & 0x3f }
#[inline] fn rs(i: u32) -> usize { ((i >> 21) & 0x1f) as usize }
#[inline] fn rt(i: u32) -> usize { ((i >> 16) & 0x1f) as usize }
#[inline] fn rd(i: u32) -> usize { ((i >> 11) & 0x1f) as usize }
#[inline] fn func(i: u32) -> u32 { i & 0x3f }
#[inline] fn fd(i: u32) -> u32 { (i >> 6) & 0x1f }
#[inline] fn simm(i: u32) -> i64 { (i as i16) as i64 }
#[inline] fn uimm(i: u32) -> u64 { (i & 0xffff) as u64 }

#[inline] unsafe fn stat(_name: &str) {}

unsafe extern "C" fn mipsr6_emul(regs: *mut pt_regs, ir: u32) -> i32 {
    let r = &mut *regs;
    match opcode(ir) {
        9 => { if rt(ir) != 0 { r.regs[rt(ir)] = ((r.regs[rs(ir)] as i64).wrapping_add(simm(ir))) as u64; } 0 }
        25 => { if rt(ir) != 0 { r.regs[rt(ir)] = ((r.regs[rs(ir)] as i64).wrapping_add(simm(ir))) as u64; } 0 }
        0x11 | 0x13 | 0x31 | 0x39 => -8, // FPU delay-slot instruction
        0 => match func(ir) {
            0x25 => { if rd(ir) != 0 { r.regs[rd(ir)] = r.regs[rs(ir)] | r.regs[rt(ir)]; } 0 }
            0 => { if rs(ir) != 0 { return -4; } if rd(ir) != 0 { r.regs[rd(ir)] = ((r.regs[rt(ir)] as u32) << fd(ir)) as i32 as u64; } 0 }
            2 => { if rs(ir) != 0 { return -4; } if rd(ir) != 0 { r.regs[rd(ir)] = ((r.regs[rt(ir)] as u32) >> fd(ir)) as i32 as u64; } 0 }
            0x21 => { if fd(ir) != 0 { return -4; } if rd(ir) != 0 { r.regs[rd(ir)] = ((r.regs[rs(ir)] as u32).wrapping_add(r.regs[rt(ir)] as u32)) as i32 as u64; } 0 }
            0x23 => { if fd(ir) != 0 { return -4; } if rd(ir) != 0 { r.regs[rd(ir)] = ((r.regs[rs(ir)] as u32).wrapping_sub(r.regs[rt(ir)] as u32)) as i32 as u64; } 0 }
            _ => -4,
        },
        _ => -4,
    }
}

unsafe extern "C" fn movz_func(regs: *mut pt_regs, ir: u32) -> i32 { let r=&mut *regs; if r.regs[rt(ir)]==0 && rd(ir)!=0 { r.regs[rd(ir)]=r.regs[rs(ir)]; } 0 }
unsafe extern "C" fn movn_func(regs: *mut pt_regs, ir: u32) -> i32 { let r=&mut *regs; if r.regs[rt(ir)]!=0 && rd(ir)!=0 { r.regs[rd(ir)]=r.regs[rs(ir)]; } 0 }
unsafe extern "C" fn mfhi_func(regs: *mut pt_regs, ir: u32) -> i32 { let r=&mut *regs; if rd(ir)!=0 { r.regs[rd(ir)]=r.hi; } 0 }
unsafe extern "C" fn mthi_func(regs: *mut pt_regs, ir: u32) -> i32 { (*regs).hi=(*regs).regs[rs(ir)]; 0 }
unsafe extern "C" fn mflo_func(regs: *mut pt_regs, ir: u32) -> i32 { let r=&mut *regs; if rd(ir)!=0 { r.regs[rd(ir)]=r.lo; } 0 }
unsafe extern "C" fn mtlo_func(regs: *mut pt_regs, ir: u32) -> i32 { (*regs).lo=(*regs).regs[rs(ir)]; 0 }

macro_rules! bin_hi_lo { ($name:ident, $op:expr) => { unsafe extern "C" fn $name(regs:*mut pt_regs,ir:u32)->i32 { let r=&mut *regs; let v=$op(r.regs[rs(ir)] as i64,r.regs[rt(ir)] as i64); r.lo=v as u64; r.hi=(v>>32) as u64; 0 } }; }
bin_hi_lo!(mult_func, |a:i64,b:i64| a.wrapping_mul(b));
bin_hi_lo!(madd_func, |a:i64,b:i64| a.wrapping_mul(b));
bin_hi_lo!(msub_func, |a:i64,b:i64| a.wrapping_mul(b));

unsafe extern "C" fn mul_func(regs:*mut pt_regs, ir:u32)->i32 { let r=&mut *regs; if rd(ir)!=0 { r.regs[rd(ir)] = (r.regs[rs(ir)] as i32 as i64).wrapping_mul(r.regs[rt(ir)] as i32 as i64) as u64; } 0 }
unsafe extern "C" fn div_func(regs:*mut pt_regs,ir:u32)->i32 { let r=&mut *regs; let a=r.regs[rs(ir)] as i32; let b=r.regs[rt(ir)] as i32; r.lo=(a.wrapping_div(b)) as i64 as u64; r.hi=(a.wrapping_rem(b)) as i64 as u64; 0 }
unsafe extern "C" fn divu_func(regs:*mut pt_regs,ir:u32)->i32 { let r=&mut *regs; let a=r.regs[rs(ir)] as u32; let b=r.regs[rt(ir)] as u32; r.lo=(a/b) as u64; r.hi=(a%b) as u64; 0 }

static SPEC_OP_TABLE: &[r2_decoder_table] = &[
    r2_decoder_table { mask:0xfc0007ff, code:0x0000000a, func:Some(movz_func) },
    r2_decoder_table { mask:0xfc0007ff, code:0x0000000b, func:Some(movn_func) },
    r2_decoder_table { mask:0xffff07ff, code:0x00000010, func:Some(mfhi_func) },
    r2_decoder_table { mask:0xfc1fffff, code:0x00000011, func:Some(mthi_func) },
    r2_decoder_table { mask:0xffff07ff, code:0x00000012, func:Some(mflo_func) },
    r2_decoder_table { mask:0xfc1fffff, code:0x00000013, func:Some(mtlo_func) },
];

unsafe fn find_op(regs:*mut pt_regs, inst:u32, table:&[r2_decoder_table])->i32 { for p in table { if (inst&p.mask)==p.code { return (p.func.unwrap())(regs,inst); } } -4 }

#[no_mangle]
pub unsafe extern "C" fn mipsr2_decoder(regs:*mut pt_regs, inst:u32, _fcr31:*mut usize)->i32 {
    let mut pass=0; loop { let epc=(*regs).cp0_epc; let e=compute_return_epc(regs); if e<0{return -7;}
        let mut err=match opcode(inst) { 0=>find_op(regs,inst,SPEC_OP_TABLE), _=>mipsr6_emul(regs,inst) };
        if err==0 && pass<MIPS_R2_EMUL_TOTAL_PASS { pass+=1; /* get_user(inst) is supplied by the kernel */ break; }
        if pass!=0 && err==-4 { err=0; } if err!=0 { (*regs).cp0_epc=epc; } return err;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
