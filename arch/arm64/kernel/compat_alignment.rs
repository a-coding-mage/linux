// SPDX-License-Identifier: GPL-2.0-only
// based on arch/arm/mm/alignment.c

// Kernel and architecture dependencies are supplied by the surrounding build.

#[repr(C)]
pub union OffsetUnion {
    pub un: libc::c_ulong,
    pub sn: libc::c_long,
}

#[repr(C)]
pub struct PtRegs {
    pub regs: [u32; 18],
    pub pc: u32,
}

extern "C" {
    fn get_user<T>(dst: *mut T, src: *const T) -> i32;
    fn put_user<T>(val: T, dst: *mut T) -> i32;
    fn instruction_pointer(regs: *const PtRegs) -> libc::c_ulong;
    fn compat_thumb_mode(regs: *const PtRegs) -> bool;
    fn perf_sw_event(event: u32, count: u64, regs: *mut PtRegs, addr: u32);
    fn arm64_skip_faulting_instruction(regs: *mut PtRegs, size: i32);
    fn hweight16(value: u32) -> u32;
}

const TYPE_ERROR: i32 = 0;
const TYPE_FAULT: i32 = 1;
const TYPE_LDST: i32 = 2;
const TYPE_DONE: i32 = 3;
const BAD_INSTR: u32 = 0xdeadc0de;

#[inline]
fn coding_bits(i: u32) -> u32 { i & 0x0e000000 }
#[inline]
fn ldst_p_bit(i: u32) -> u32 { i & (1 << 24) }
#[inline]
fn ldst_u_bit(i: u32) -> u32 { i & (1 << 23) }
#[inline]
fn ldst_w_bit(i: u32) -> u32 { i & (1 << 21) }
#[inline]
fn ldst_l_bit(i: u32) -> u32 { i & (1 << 20) }
#[inline]
fn ldst_p_eq_u(i: u32) -> bool { (((i ^ (i >> 1)) & (1 << 23)) == 0) }
#[inline]
fn ldsthd_i_bit(i: u32) -> u32 { i & (1 << 22) }
#[inline]
fn rn_bits(i: u32) -> usize { ((i >> 16) & 15) as usize }
#[inline]
fn rd_bits(i: u32) -> usize { ((i >> 12) & 15) as usize }
#[inline]
fn rm_bits(i: u32) -> usize { (i & 15) as usize }
#[inline]
fn regmask_bits(i: u32) -> u32 { i & 0xffff }
#[inline]
fn is_t32(hi16: u16) -> bool { (hi16 & 0xe000) == 0xe000 && (hi16 & 0x1800) != 0 }

unsafe fn do_alignment_finish_ldst(mut addr: libc::c_ulong, instr: u32, regs: *mut PtRegs, mut offset: OffsetUnion) {
    if ldst_u_bit(instr) == 0 { offset.un = 0usize.wrapping_sub(offset.un); }
    if ldst_p_bit(instr) == 0 { addr = addr.wrapping_add(offset.un); }
    if ldst_p_bit(instr) == 0 || ldst_w_bit(instr) != 0 { (*regs).regs[rn_bits(instr)] = addr as u32; }
}

unsafe fn do_alignment_ldrdstrd(addr: libc::c_ulong, instr: u32, regs: *mut PtRegs) -> i32 {
    let rd = rd_bits(instr);
    let (rd2, load) = if instr & 0xfe000000 == 0xe8000000 {
        (((instr >> 8) & 0xf) as usize, ldst_l_bit(instr) != 0)
    } else if rd & 1 == 1 || rd == 14 { return TYPE_ERROR; } else {
        (rd + 1, instr & 0xf0 == 0xd0)
    };
    if load {
        let mut val = 0u32; let mut val2 = 0u32;
        if get_user(&mut val, addr as *const u32) != 0 || get_user(&mut val2, addr.wrapping_add(4) as *const u32) != 0 { return TYPE_FAULT; }
        (*regs).regs[rd] = val; (*regs).regs[rd2] = val2;
    } else if put_user((*regs).regs[rd], addr as *mut u32) != 0 || put_user((*regs).regs[rd2], addr.wrapping_add(4) as *mut u32) != 0 { return TYPE_FAULT; }
    TYPE_LDST
}

unsafe fn do_alignment_ldmstm(mut addr: libc::c_ulong, instr: u32, regs: *mut PtRegs) -> i32 {
    let rn = rn_bits(instr); let mut nr_regs = hweight16(regmask_bits(instr)) as libc::c_ulong * 4;
    let mut newaddr = (*regs).regs[rn] as libc::c_ulong; let mut eaddr = newaddr;
    if ldst_u_bit(instr) == 0 { nr_regs = 0usize.wrapping_sub(nr_regs); }
    newaddr = newaddr.wrapping_add(nr_regs); if ldst_u_bit(instr) == 0 { eaddr = newaddr; }
    if ldst_p_eq_u(instr) { eaddr = eaddr.wrapping_add(4); }
    let mut regbits = regmask_bits(instr); let mut rd = 0usize;
    while regbits != 0 { if regbits & 1 != 0 {
        if ldst_l_bit(instr) != 0 { let mut val = 0u32; if get_user(&mut val, eaddr as *const u32) != 0 { return TYPE_FAULT; } if rd < 15 { (*regs).regs[rd] = val; } else { (*regs).pc = val; }
        } else { let val = if rd < 15 { (*regs).regs[rd] } else { (*regs).pc.wrapping_add(8) }; if put_user(val, eaddr as *mut u32) != 0 { return TYPE_FAULT; } }
        eaddr = eaddr.wrapping_add(4);
    } regbits >>= 1; rd += 1; }
    if ldst_w_bit(instr) != 0 { (*regs).regs[rn] = newaddr as u32; } TYPE_DONE
}

unsafe fn thumb2arm(tinstr: u16) -> u32 {
    let l = ((tinstr & (1 << 11)) >> 11) as u32;
    match (tinstr & 0xf800) >> 11 {
        0xc000 >> 11 | 0xc800 >> 11 => { let rn = ((tinstr & (7 << 8)) >> 8) as u32; let w = if ((l << rn) & (tinstr as u32 & 255)) != 0 { 0 } else { 1 << 21 }; 0xe8800000 | w | (l << 20) | (rn << 16) | (tinstr as u32 & 255) }
        0xb000 >> 11 | 0xb800 >> 11 => { if tinstr & (3 << 9) == 0x0400 { const S: [u32; 4] = [0xe92d0000,0xe92d4000,0xe8bd0000,0xe8bd8000]; return S[((l << 1) | (((tinstr & (1 << 8)) >> 8) as u32)) as usize] | (tinstr as u32 & 255); } BAD_INSTR }
        _ => BAD_INSTR,
    }
}

// Thumb-2 conversion handler. The returned function reuses the ARM handlers.
unsafe fn do_alignment_t32_to_handler(pinstr: *mut u32, regs: *mut PtRegs, poffset: *mut OffsetUnion) -> Option<unsafe fn(libc::c_ulong,u32,*mut PtRegs)->i32> {
    let instr = *pinstr; let tinst1 = (instr >> 16) as u16; let tinst2 = instr as u16;
    match tinst1 & 0xffe0 {
        0xe880 | 0xe8a0 | 0xe900 | 0xe920 => Some(do_alignment_ldmstm),
        0xf840 => { if rn_bits(instr)==13 && (tinst2 & 0x09ff)==0x0904 { let l=if ldst_l_bit(instr)!=0 {1} else {0}; let s=[0xe92d0000,0xe8bd0000]; *pinstr=s[l] | (1<<rd_bits(instr)); Some(do_alignment_ldmstm) } else { None } }
        0xe860 | 0xe960 | 0xe8e0 | 0xe9e0 => { (*poffset).un=((tinst2 as u32&0xff)<<2) as libc::c_ulong; Some(do_alignment_ldrdstrd) }
        0xe940 | 0xe9c0 => Some(do_alignment_ldrdstrd),
        _ => None,
    }
}

pub unsafe fn do_compat_alignment_fixup(mut addr: libc::c_ulong, regs: *mut PtRegs) -> i32 {
    let instrptr = instruction_pointer(regs); let mut instr = 0u32; let mut isize = 4;
    if compat_thumb_mode(regs) { let ptr = (instrptr & !1) as *const u16; let mut t = 0u16; if get_user(&mut t, ptr) != 0 { return 1; } if is_t32(t) { let mut t2=0u16; if get_user(&mut t2, ptr.add(1)) != 0 { return 1; } instr=((t as u32)<<16)|t2 as u32; } else { isize=2; instr=thumb2arm(t); } } else if get_user(&mut instr, instrptr as *const u32) != 0 { return 1; }
    let mut offset=OffsetUnion{un:0}; let handler: Option<unsafe fn(libc::c_ulong,u32,*mut PtRegs)->i32> = match coding_bits(instr) { 0 => { if ldsthd_i_bit(instr)!=0 { offset.un=((instr&0xf00)>>4 | instr&15) as libc::c_ulong; } else { offset.un=(*regs).regs[rm_bits(instr)] as libc::c_ulong; } if instr&0x001000f0==0xd0 || instr&0x001000f0==0xf0 { Some(do_alignment_ldrdstrd) } else { return 1 } }, 0x08000000 => Some(do_alignment_ldmstm), _ => return 1 };
    let typ=handler.unwrap()(addr,instr,regs); if typ==TYPE_ERROR || typ==TYPE_FAULT { return 1; } if typ==TYPE_LDST { do_alignment_finish_ldst(addr,instr,regs,offset); } perf_sw_event(0,1,regs,(*regs).pc); arm64_skip_faulting_instruction(regs,isize); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
