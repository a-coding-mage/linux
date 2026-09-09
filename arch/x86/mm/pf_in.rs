// SPDX-License-Identifier: GPL-2.0-or-later
/* Fault Injection Test harness (FI) */

#[cfg(target_arch = "x86")]
static PREFIX_CODES: &[u8] = &[0xf0, 0xf2, 0xf3, 0x2e, 0x36, 0x3e, 0x26, 0x64, 0x65, 0x66, 0x67];
#[cfg(not(target_arch = "x86"))]
static PREFIX_CODES: &[u8] = &[0x66, 0x67, 0x2e, 0x3e, 0x26, 0x64, 0x65, 0x36, 0xf0, 0xf3, 0xf2,
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f];

#[cfg(target_arch = "x86")]
static REG_ROP: &[u32] = &[0x8a, 0x8b, 0xb60f, 0xb70f, 0xbe0f, 0xbf0f];
#[cfg(not(target_arch = "x86"))]
static REG_ROP: &[u32] = &[0x8a, 0x8b, 0xb60f, 0xb70f, 0xbe0f, 0xbf0f];
#[cfg(target_arch = "x86")]
static REG_WOP: &[u32] = &[0x88, 0x89, 0xaa, 0xab];
#[cfg(not(target_arch = "x86"))]
static REG_WOP: &[u32] = &[0x88, 0x89, 0xaa, 0xab];
static IMM_WOP: &[u32] = &[0xc6, 0xc7];
#[cfg(target_arch = "x86")]
static RW8: &[u32] = &[0x88, 0x8a, 0xc6, 0xaa];
#[cfg(not(target_arch = "x86"))]
static RW8: &[u32] = &[0xc6, 0x88, 0x8a, 0xaa];
#[cfg(target_arch = "x86")]
static RW32: &[u32] = &[0x89, 0x8b, 0xc7, 0xb60f, 0xb70f, 0xbe0f, 0xbf0f, 0xab];
#[cfg(not(target_arch = "x86"))]
static RW32: &[u32] = &[0xc7, 0x89, 0x8b, 0xb60f, 0xb70f, 0xbe0f, 0xbf0f, 0xab];
#[cfg(target_arch = "x86")]
static MW8: &[u32] = &[0x88, 0x8a, 0xc6, 0xb60f, 0xbe0f, 0xaa];
#[cfg(not(target_arch = "x86"))]
static MW8: &[u32] = &[0xc6, 0x88, 0x8a, 0xb60f, 0xbe0f, 0xaa];
static MW16: &[u32] = &[0xb70f, 0xbf0f];
#[cfg(target_arch = "x86")]
static MW32: &[u32] = &[0x89, 0x8b, 0xc7, 0xab];
#[cfg(not(target_arch = "x86"))]
static MW32: &[u32] = &[0xc7];
#[cfg(target_arch = "x86")]
static MW64: &[u32] = &[];
#[cfg(not(target_arch = "x86"))]
static MW64: &[u32] = &[0x89, 0x8b, 0xab];

#[repr(C)]
pub struct prefix_bits { pub shorted: u16, pub enlarged: u16, pub rexr: u16, pub rex: u16 }

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum reason_type { REG_READ, REG_WRITE, IMM_WRITE, OTHERS }

#[repr(C)]
pub struct pt_regs {
    pub ax: u64, pub bx: u64, pub cx: u64, pub dx: u64,
    pub sp: u64, pub bp: u64, pub si: u64, pub di: u64,
    #[cfg(target_arch = "x86_64")]
    pub r8: u64, #[cfg(target_arch = "x86_64")] pub r9: u64,
    #[cfg(target_arch = "x86_64")] pub r10: u64, #[cfg(target_arch = "x86_64")] pub r11: u64,
    #[cfg(target_arch = "x86_64")] pub r12: u64, #[cfg(target_arch = "x86_64")] pub r13: u64,
    #[cfg(target_arch = "x86_64")] pub r14: u64, #[cfg(target_arch = "x86_64")] pub r15: u64,
}

unsafe fn skip_prefix(addr: *mut u8, prf: *mut prefix_bits) -> isize {
    (*prf).shorted = 0; (*prf).enlarged = 0; (*prf).rexr = 0; (*prf).rex = 0;
    let mut p = addr;
    loop {
        let b = *p;
        if !PREFIX_CODES.contains(&b) { return p.offset_from(addr); }
        if b == 0x66 { (*prf).shorted = 1; }
        #[cfg(target_arch = "x86_64")]
        { if b & 0xf8 == 0x48 { (*prf).enlarged = 1; } if b & 0xf4 == 0x44 { (*prf).rexr = 1; } if b & 0xf0 == 0x40 { (*prf).rex = 1; } }
        p = p.add(1);
    }
}

unsafe fn get_opcode(addr: *mut u8, opcode: *mut u32) -> isize {
    if *addr == 0x0f { *opcode = *(addr as *mut u16) as u32; 2 } else { *opcode = *addr as u32; 1 }
}

pub unsafe fn get_ins_type(ins_addr: usize) -> reason_type {
    let mut op = 0; let mut p = ins_addr as *mut u8; let mut prf = prefix_bits { shorted: 0, enlarged: 0, rexr: 0, rex: 0 };
    p = p.add(skip_prefix(p, &mut prf)); p = p.add(get_opcode(p, &mut op) as usize);
    if REG_ROP.contains(&op) { reason_type::REG_READ } else if REG_WOP.contains(&op) { reason_type::REG_WRITE } else if IMM_WOP.contains(&op) { reason_type::IMM_WRITE } else { reason_type::OTHERS }
}

pub unsafe fn get_ins_reg_width(ins_addr: usize) -> u32 {
    let mut op = 0; let mut p = ins_addr as *mut u8; let mut prf = prefix_bits { shorted: 0, enlarged: 0, rexr: 0, rex: 0 };
    p = p.add(skip_prefix(p, &mut prf)); get_opcode(p, &mut op);
    if RW8.contains(&op) { 1 } else if RW32.contains(&op) { if prf.shorted != 0 { 2 } else if prf.enlarged != 0 { 8 } else { 4 } } else { 0 }
}

pub unsafe fn get_ins_mem_width(ins_addr: usize) -> u32 {
    let mut op = 0; let mut p = ins_addr as *mut u8; let mut prf = prefix_bits { shorted: 0, enlarged: 0, rexr: 0, rex: 0 };
    p = p.add(skip_prefix(p, &mut prf)); get_opcode(p, &mut op);
    if MW8.contains(&op) { 1 } else if MW16.contains(&op) { 2 } else if MW32.contains(&op) { if prf.shorted != 0 { 2 } else { 4 } } else if MW64.contains(&op) { if prf.shorted != 0 { 2 } else if prf.enlarged != 0 { 8 } else { 4 } } else { 0 }
}

const ARG_AL: i32 = 0; const ARG_CL: i32 = 1; const ARG_DL: i32 = 2; const ARG_BL: i32 = 3;
const ARG_AH: i32 = 4; const ARG_CH: i32 = 5; const ARG_DH: i32 = 6; const ARG_BH: i32 = 7;
const ARG_AX: i32 = 0; const ARG_CX: i32 = 1; const ARG_DX: i32 = 2; const ARG_BX: i32 = 3;
const ARG_SP: i32 = 4; const ARG_BP: i32 = 5; const ARG_SI: i32 = 6; const ARG_DI: i32 = 7;

unsafe fn get_reg_w32(no: i32, r: *mut pt_regs) -> *mut u64 {
    match no { 0 => &mut (*r).ax, 1 => &mut (*r).cx, 2 => &mut (*r).dx, 3 => &mut (*r).bx,
        4 => &mut (*r).sp, 5 => &mut (*r).bp, 6 => &mut (*r).si, 7 => &mut (*r).di,
        #[cfg(target_arch = "x86_64")] 8 => &mut (*r).r8, #[cfg(target_arch = "x86_64")] 9 => &mut (*r).r9,
        #[cfg(target_arch = "x86_64")] 10 => &mut (*r).r10, #[cfg(target_arch = "x86_64")] 11 => &mut (*r).r11,
        #[cfg(target_arch = "x86_64")] 12 => &mut (*r).r12, #[cfg(target_arch = "x86_64")] 13 => &mut (*r).r13,
        #[cfg(target_arch = "x86_64")] 14 => &mut (*r).r14, #[cfg(target_arch = "x86_64")] 15 => &mut (*r).r15,
        _ => core::ptr::null_mut() }
}

unsafe fn get_reg_w8(no: i32, rex: u16, r: *mut pt_regs) -> *mut u8 {
    match no { 0 => &mut (*r).ax as *mut u64 as *mut u8, 1 => &mut (*r).cx as *mut u64 as *mut u8,
        2 => &mut (*r).dx as *mut u64 as *mut u8, 3 => &mut (*r).bx as *mut u64 as *mut u8,
        4 if rex != 0 => &mut (*r).sp as *mut u64 as *mut u8, 5 if rex != 0 => &mut (*r).bp as *mut u64 as *mut u8,
        6 if rex != 0 => &mut (*r).si as *mut u64 as *mut u8, 7 if rex != 0 => &mut (*r).di as *mut u64 as *mut u8,
        4 => (&mut (*r).ax as *mut u64 as *mut u8).add(1), 5 => (&mut (*r).cx as *mut u64 as *mut u8).add(1),
        6 => (&mut (*r).dx as *mut u64 as *mut u8).add(1), 7 => (&mut (*r).bx as *mut u64 as *mut u8).add(1),
        #[cfg(target_arch = "x86_64")] 8 => &mut (*r).r8 as *mut u64 as *mut u8,
        #[cfg(target_arch = "x86_64")] 9 => &mut (*r).r9 as *mut u64 as *mut u8,
        #[cfg(target_arch = "x86_64")] 10 => &mut (*r).r10 as *mut u64 as *mut u8,
        #[cfg(target_arch = "x86_64")] 11 => &mut (*r).r11 as *mut u64 as *mut u8,
        #[cfg(target_arch = "x86_64")] 12 => &mut (*r).r12 as *mut u64 as *mut u8,
        #[cfg(target_arch = "x86_64")] 13 => &mut (*r).r13 as *mut u64 as *mut u8,
        #[cfg(target_arch = "x86_64")] 14 => &mut (*r).r14 as *mut u64 as *mut u8,
        #[cfg(target_arch = "x86_64")] 15 => &mut (*r).r15 as *mut u64 as *mut u8, _ => core::ptr::null_mut() }
}

pub unsafe fn get_ins_reg_val(ins_addr: usize, regs: *mut pt_regs) -> usize {
    let mut op=0; let mut p=ins_addr as *mut u8; let mut f=prefix_bits{shorted:0,enlarged:0,rexr:0,rex:0}; p=p.add(skip_prefix(p,&mut f)); p=p.add(get_opcode(p,&mut op) as usize);
    if !REG_ROP.contains(&op) && !REG_WOP.contains(&op) { return 0; }
    let reg=if op==0xaa || op==0xab { 0 } else { (((*p as i32)>>3)&7) | ((f.rexr as i32)<<3) };
    let w=get_ins_reg_width(ins_addr); let q=get_reg_w32(reg,regs); if w==1 { get_reg_w8(reg,f.rex,regs).read() as usize } else if q.is_null(){0} else if w==2 { q.read() as u16 as usize } else if w==4 { q.read() as u32 as usize } else { q.read() as usize }
}

pub unsafe fn get_ins_imm_val(ins_addr: usize) -> usize {
    let mut op=0; let mut p=ins_addr as *mut u8; let mut f=prefix_bits{shorted:0,enlarged:0,rexr:0,rex:0}; p=p.add(skip_prefix(p,&mut f)); p=p.add(get_opcode(p,&mut op) as usize); if !IMM_WOP.contains(&op){return 0;}
    let rm=*p; let mode=rm>>6; p=p.add(1); if mode==0 && rm&7==5 {p=p.add(4)} else if mode==1 {p=p.add(1)} else if mode==2 {p=p.add(4)} else if mode==3{return 0};
    match get_ins_reg_width(ins_addr) {1=>*p as usize,2=>*(p as *mut u16) as usize,4=>*(p as *mut u32) as usize,8=>*(p as *mut u64) as usize,_=>0}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
