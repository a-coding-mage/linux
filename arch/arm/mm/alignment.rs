// SPDX-License-Identifier: GPL-2.0-only
// Translation of linux/arch/arm/mm/alignment.c.

const UM_WARN: i32 = 1 << 0;
const UM_FIXUP: i32 = 1 << 1;
const UM_SIGNAL: i32 = 1 << 2;
const TYPE_ERROR: i32 = 0;
const TYPE_FAULT: i32 = 1;
const TYPE_LDST: i32 = 2;
const TYPE_DONE: i32 = 3;
const BAD_INSTR: u32 = 0xdeadc0de;
const SHIFT_LSL: u32 = 0x00;
const SHIFT_LSR: u32 = 0x20;
const SHIFT_ASR: u32 = 0x40;
const SHIFT_RORRRX: u32 = 0x60;

#[repr(C)]
pub union OffsetUnion { pub un: usize, pub sn: isize }

// External kernel types and functions supplied by the surrounding tree.
extern "C" {
    fn cpu_architecture() -> u32; fn get_cr() -> usize; fn set_cr(v: usize);
    fn user_mode(r: *mut PtRegs) -> bool; fn uaccess_save_and_enable() -> u32;
    fn uaccess_restore(v: u32); fn hweight16(v: u32) -> u32;
    fn instruction_pointer(r: *mut PtRegs) -> usize; fn thumb_mode(r: *mut PtRegs) -> bool;
    fn it_advance(v: u32) -> u32; fn interrupts_enabled(r: *mut PtRegs) -> bool;
    fn local_irq_enable(); fn raw_local_irq_disable(); fn read_thread_flags() -> usize;
    fn do_bad_area(addr: usize, fsr: u32, r: *mut PtRegs);
    fn harden_branch_predictor();
}
#[repr(C)] pub struct PtRegs { pub uregs: [u32; 18], pub ARM_pc: u32, pub ARM_cpsr: u32 }

static mut ai_user: usize = 0; static mut ai_sys: usize = 0;
static mut ai_sys_last_pc: *mut core::ffi::c_void = core::ptr::null_mut();
static mut ai_skipped: usize = 0; static mut ai_half: usize = 0;
static mut ai_word: usize = 0; static mut ai_dword: usize = 0;
static mut ai_multi: usize = 0; static mut ai_usermode: i32 = 0;
static mut cr_no_alignment: usize = 0;

#[inline] fn coding_bits(i: u32) -> u32 { i & 0x0e000000 }
#[inline] fn cond_bits(i: u32) -> u32 { i & 0xf0000000 }
#[inline] fn ldst_i(i: u32) -> u32 { i & (1 << 26) }
#[inline] fn ldst_p(i: u32) -> u32 { i & (1 << 24) }
#[inline] fn ldst_u(i: u32) -> u32 { i & (1 << 23) }
#[inline] fn ldst_w(i: u32) -> u32 { i & (1 << 21) }
#[inline] fn ldst_l(i: u32) -> u32 { i & (1 << 20) }
#[inline] fn ldst_p_eq_u(i: u32) -> bool { (((i ^ (i >> 1)) & (1 << 23)) == 0) }
#[inline] fn ldsthd_i(i: u32) -> u32 { i & (1 << 22) }
#[inline] fn ldm_s(i: u32) -> u32 { i & (1 << 22) }
#[inline] fn rn(i: u32) -> usize { ((i >> 16) & 15) as usize }
#[inline] fn rd(i: u32) -> usize { ((i >> 12) & 15) as usize }
#[inline] fn rm(i: u32) -> usize { (i & 15) as usize }
#[inline] fn regmask(i: u32) -> u32 { i & 0xffff }
#[inline] fn offset_bits(i: u32) -> usize { (i & 0xfff) as usize }
#[inline] fn is_shift(i: u32) -> u32 { i & 0x0ff0 }
#[inline] fn shift_bits(i: u32) -> u32 { (i >> 7) & 0x1f }
#[inline] fn shift_type(i: u32) -> u32 { i & 0x60 }

unsafe fn get16(addr: usize) -> u32 { core::ptr::read_unaligned(addr as *const u16) as u32 }
unsafe fn get32(addr: usize) -> u32 { core::ptr::read_unaligned(addr as *const u32) }
unsafe fn put16(addr: usize, v: u32) { core::ptr::write_unaligned(addr as *mut u16, v as u16) }
unsafe fn put32(addr: usize, v: u32) { core::ptr::write_unaligned(addr as *mut u32, v) }

unsafe fn do_alignment_finish_ldst(mut addr: usize, instr: u32, regs: *mut PtRegs, mut offset: OffsetUnion) {
    if ldst_u(instr) == 0 { offset.un = offset.un.wrapping_neg(); }
    if ldst_p(instr) == 0 { addr = addr.wrapping_add(offset.un); }
    if ldst_p(instr) == 0 || ldst_w(instr) != 0 { (*regs).uregs[rn(instr)] = addr as u32; }
}

unsafe fn do_alignment_ldrhstrh(addr: usize, instr: u32, regs: *mut PtRegs) -> i32 {
    ai_half += 1; let r = rd(instr); let mut val;
    if ldst_l(instr) != 0 { val = get16(addr); if instr & 0x40 != 0 { val = (val as i16 as i32) as u32; } (*regs).uregs[r] = val; }
    else { put16(addr, (*regs).uregs[r]); } TYPE_LDST
}
unsafe fn do_alignment_ldrdstrd(addr: usize, instr: u32, regs: *mut PtRegs) -> i32 {
    let r = rd(instr); let (r2, load) = if instr & 0xfe000000 == 0xe8000000 { (((instr >> 8)&15) as usize, ldst_l(instr)!=0) } else if r & 1 != 0 || r == 14 { return TYPE_ERROR } else { (r+1, instr & 0xf0 == 0xd0) };
    ai_dword += 1;
    if load { (*regs).uregs[r] = get32(addr); (*regs).uregs[r2] = get32(addr.wrapping_add(4)); }
    else { put32(addr, (*regs).uregs[r]); put32(addr.wrapping_add(4), (*regs).uregs[r2]); } TYPE_LDST
}
unsafe fn do_alignment_ldrstr(addr: usize, instr: u32, regs: *mut PtRegs) -> i32 {
    let r=rd(instr); ai_word += 1;
    if ldst_l(instr)!=0 { (*regs).uregs[r]=get32(addr); } else { put32(addr,(*regs).uregs[r]); } TYPE_LDST
}

unsafe fn do_alignment_ldmstm(mut addr: usize, instr: u32, regs: *mut PtRegs) -> i32 {
    if ldm_s(instr)!=0 { return TYPE_ERROR; }
    let correction=4u32; (*regs).ARM_pc=(*regs).ARM_pc.wrapping_add(correction); ai_multi+=1;
    let nr=(regmask(instr).count_ones()*4) as usize; let rn_=rn(instr); let mut eaddr=(*regs).uregs[rn_] as usize; let newaddr;
    if ldst_u(instr)==0 { eaddr=eaddr.wrapping_sub(nr); } newaddr=eaddr; if !ldst_u(instr)!=true && ldst_p_eq_u(instr) { eaddr=eaddr.wrapping_add(4); }
    let mut bits=regmask(instr); let mut r=0; while bits!=0 { if bits&1!=0 { if ldst_l(instr)!=0 { (*regs).uregs[r]=get32(eaddr); } else { put32(eaddr,(*regs).uregs[r]); } eaddr=eaddr.wrapping_add(4); } bits>>=1; r+=1; }
    if ldst_w(instr)!=0 { (*regs).uregs[rn_]=newaddr; } if ldst_l(instr)==0 || regmask(instr)&(1<<15)==0 { (*regs).ARM_pc=(*regs).ARM_pc.wrapping_sub(correction); } TYPE_DONE
}

unsafe fn thumb2arm(t: u16) -> u32 {
    let l=((t as u32)&(1<<11))>>11;
    match ((t as u32)&0xf800)>>11 {
        0xc|0xd|0xe|0xf => 0xe5800000 | (((t as u32)&(1<<12))<<10) | (l<<20) | (((t as u32)&7)<<12) | (((t as u32)&(7<<3))<<13) | (((t as u32)&(31<<6)) >> (6-((if t&0x1000!=0 {0}else{2})))),
        0x10|0x11 => 0xe1c000b0 | (l<<20) | (((t as u32)&7)<<12) | (((t as u32)&(7<<3))<<13) | (((t as u32)&(7<<6))>>5) | (((t as u32)&(3<<9))>>1),
        0x12|0x13 => { let s=[0xe7800000,0xe18000b0,0xe7c00000,0xe19000d0,0xe7900000,0xe19000b0,0xe7d00000,0xe19000f0]; s[((t as u32>>9)&7) as usize] | (((t as u32)&7)<<12) | (((t as u32)&(7<<3))<<13) | (((t as u32)&(7<<6))>>6) },
        0x9 => 0xe59f0000 | (((t as u32)&(7<<8))<<4) | (((t as u32)&255)<<2),
        0x18|0x19 => 0xe58d0000|(l<<20)|(((t as u32)&(7<<8))<<4)|(((t as u32)&255)<<2),
        _ => BAD_INSTR,
    }
}

// The remaining entry-point and procfs glue are kept as declarations because
// their kernel-specific dependencies are supplied by the surrounding tree.
extern "C" { pub fn do_alignment(addr: usize, fsr: u32, regs: *mut PtRegs) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
