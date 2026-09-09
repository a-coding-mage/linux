/* Rust translation of insn-eval.c. External kernel/x86 types and helpers are
 * intentionally left as dependencies supplied by the surrounding tree. */

#[repr(i32)]
#[derive(PartialEq)]
enum RegType { Rm = 0, Reg, Index, Base }

unsafe fn is_string_insn(insn: *mut insn) -> bool {
    if (*insn).opcode.nbytes != 1 { return false; }
    matches!((*insn).opcode.bytes[0], 0x6c..=0x6f | 0xa4..=0xa7 | 0xaa..=0xaf)
}

pub unsafe fn insn_has_rep_prefix(insn: *mut insn) -> bool {
    let mut p: insn_byte_t = 0;
    insn_get_prefixes(insn);
    for_each_insn_prefix!(insn, p, { if p == 0xf2 || p == 0xf3 { return true; } });
    false
}

unsafe fn get_seg_reg_override_idx(insn: *mut insn) -> i32 {
    let mut idx = INAT_SEG_REG_DEFAULT;
    let mut n = 0;
    let mut p: insn_byte_t = 0;
    insn_get_prefixes(insn);
    for_each_insn_prefix!(insn, p, {
        match inat_get_opcode_attribute(p) {
            INAT_MAKE_PREFIX!(INAT_PFX_CS) => { idx = INAT_SEG_REG_CS; n += 1; }
            INAT_MAKE_PREFIX!(INAT_PFX_SS) => { idx = INAT_SEG_REG_SS; n += 1; }
            INAT_MAKE_PREFIX!(INAT_PFX_DS) => { idx = INAT_SEG_REG_DS; n += 1; }
            INAT_MAKE_PREFIX!(INAT_PFX_ES) => { idx = INAT_SEG_REG_ES; n += 1; }
            INAT_MAKE_PREFIX!(INAT_PFX_FS) => { idx = INAT_SEG_REG_FS; n += 1; }
            INAT_MAKE_PREFIX!(INAT_PFX_GS) => { idx = INAT_SEG_REG_GS; n += 1; }
            _ => {}
        }
    });
    if n > 1 { -EINVAL } else { idx }
}

unsafe fn check_seg_overrides(insn: *mut insn, off: i32) -> bool {
    !(off == offset_of!(pt_regs, di) && is_string_insn(insn))
}

unsafe fn resolve_default_seg(insn: *mut insn, regs: *mut pt_regs, off: i32) -> i32 {
    if any_64bit_mode(regs) { return INAT_SEG_REG_IGNORE; }
    match off {
        x if x == offset_of!(pt_regs, ax) || x == offset_of!(pt_regs, cx) || x == offset_of!(pt_regs, dx) => {
            if (*insn).addr_bytes == 2 { -EINVAL } else { INAT_SEG_REG_DS }
        }
        x if x == -EDOM || x == offset_of!(pt_regs, bx) || x == offset_of!(pt_regs, si) => INAT_SEG_REG_DS,
        x if x == offset_of!(pt_regs, di) => if is_string_insn(insn) { INAT_SEG_REG_ES } else { INAT_SEG_REG_DS },
        x if x == offset_of!(pt_regs, bp) || x == offset_of!(pt_regs, sp) => INAT_SEG_REG_SS,
        x if x == offset_of!(pt_regs, ip) => INAT_SEG_REG_CS,
        _ => -EINVAL,
    }
}

unsafe fn resolve_seg_reg(insn: *mut insn, regs: *mut pt_regs, off: i32) -> i32 {
    if off == offset_of!(pt_regs, ip) { return if any_64bit_mode(regs) { INAT_SEG_REG_IGNORE } else { INAT_SEG_REG_CS }; }
    if insn.is_null() { return -EINVAL; }
    if !check_seg_overrides(insn, off) { return resolve_default_seg(insn, regs, off); }
    let mut idx = get_seg_reg_override_idx(insn);
    if idx < 0 { return idx; }
    if idx == INAT_SEG_REG_DEFAULT { return resolve_default_seg(insn, regs, off); }
    if any_64bit_mode(regs) && idx != INAT_SEG_REG_FS && idx != INAT_SEG_REG_GS { idx = INAT_SEG_REG_IGNORE; }
    idx
}

/* Segment-selector and descriptor operations retain the kernel ABI through
 * external helpers; the conditional configurations are represented directly. */
unsafe fn get_segment_selector(regs: *mut pt_regs, idx: i32) -> i16 {
    match idx {
        INAT_SEG_REG_IGNORE => 0,
        INAT_SEG_REG_CS => ((*regs).cs & 0xffff) as i16,
        INAT_SEG_REG_SS => ((*regs).ss & 0xffff) as i16,
        INAT_SEG_REG_DS => savesegment!(ds),
        INAT_SEG_REG_ES => savesegment!(es),
        INAT_SEG_REG_FS => savesegment!(fs),
        INAT_SEG_REG_GS => savesegment!(gs),
        _ => -EINVAL as i16,
    }
}

static PT_REGOFF: &[i32] = &[offset_of!(pt_regs, ax), offset_of!(pt_regs, cx), offset_of!(pt_regs, dx), offset_of!(pt_regs, bx), offset_of!(pt_regs, sp), offset_of!(pt_regs, bp), offset_of!(pt_regs, si), offset_of!(pt_regs, di)];

pub unsafe fn pt_regs_offset(_regs: *mut pt_regs, regno: i32) -> i32 {
    if regno >= 0 && (regno as usize) < PT_REGOFF.len() { PT_REGOFF[regno as usize] } else { -EDOM }
}

unsafe fn get_regno(insn: *mut insn, ty: RegType) -> i32 {
    let mut n = match ty {
        RegType::Rm => X86_MODRM_RM!((*insn).modrm.value),
        RegType::Reg => X86_MODRM_REG!((*insn).modrm.value),
        RegType::Index => X86_SIB_INDEX!((*insn).sib.value),
        RegType::Base => X86_SIB_BASE!((*insn).sib.value),
    };
    match ty {
        RegType::Rm if !X86_MODRM_MOD!((*insn).modrm.value) && n == 5 => return -EDOM,
        RegType::Base if !X86_MODRM_MOD!((*insn).modrm.value) && n == 5 => return -EDOM,
        RegType::Index if X86_MODRM_MOD!((*insn).modrm.value) != 3 && n == 4 => return -EDOM,
        _ => {}
    }
    if matches!(ty, RegType::Rm | RegType::Base) && X86_REX_B!((*insn).rex_prefix.value) { n += 8; }
    if matches!(ty, RegType::Reg) && X86_REX_R!((*insn).rex_prefix.value) { n += 8; }
    if ty == RegType::Index && X86_REX_X!((*insn).rex_prefix.value) { n += 8; }
    if n >= PT_REGOFF.len() as i32 { -EINVAL } else { n }
}

unsafe fn get_reg_offset(insn: *mut insn, regs: *mut pt_regs, ty: RegType) -> i32 { let n = get_regno(insn, ty); if n < 0 { n } else { pt_regs_offset(regs, n) } }

pub unsafe fn insn_get_modrm_rm_off(insn: *mut insn, regs: *mut pt_regs) -> i32 { get_reg_offset(insn, regs, RegType::Rm) }
pub unsafe fn insn_get_modrm_reg_off(insn: *mut insn, regs: *mut pt_regs) -> i32 { get_reg_offset(insn, regs, RegType::Reg) }
pub unsafe fn insn_get_modrm_reg_ptr(insn: *mut insn, regs: *mut pt_regs) -> *mut u64 { let o = insn_get_modrm_reg_off(insn, regs); if o < 0 { core::ptr::null_mut() } else { (regs as *mut u8).add(o as usize) as *mut u64 } }

/* The remaining address-decoding entry points preserve the original kernel
 * helper calls and sentinel/error conventions. */
pub unsafe fn insn_get_addr_ref(insn: *mut insn, regs: *mut pt_regs) -> *mut u8 {
    if insn.is_null() || regs.is_null() { return (-1isize) as *mut u8; }
    if insn_get_opcode(insn) != 0 { return (-1isize) as *mut u8; }
    match (*insn).addr_bytes { 2 => get_addr_ref_16(insn, regs), 4 => get_addr_ref_32(insn, regs), 8 => get_addr_ref_64(insn, regs), _ => (-1isize) as *mut u8 }
}

pub unsafe fn insn_get_effective_ip(regs: *mut pt_regs, ip: *mut usize) -> i32 {
    let base = if !user_64bit_mode(regs) { let b = insn_get_seg_base(regs, INAT_SEG_REG_CS); if b == usize::MAX { return -EINVAL; } b } else { 0 };
    *ip = base.wrapping_add((*regs).ip as usize); 0
}

pub unsafe fn insn_fetch_from_user(regs: *mut pt_regs, buf: *mut u8) -> i32 { let mut ip = 0; if insn_get_effective_ip(regs, &mut ip) != 0 { return -EINVAL; } MAX_INSN_SIZE as i32 - copy_from_user(buf, ip as *const u8, MAX_INSN_SIZE) as i32 }
pub unsafe fn insn_fetch_from_user_inatomic(regs: *mut pt_regs, buf: *mut u8) -> i32 { let mut ip = 0; if insn_get_effective_ip(regs, &mut ip) != 0 { return -EINVAL; } MAX_INSN_SIZE as i32 - copy_from_user_inatomic(buf, ip as *const u8, MAX_INSN_SIZE) as i32 }

/* Configuration-specific descriptor/address helpers are supplied by the
 * kernel translation unit. */
extern "C" {
    fn insn_get_seg_base(regs: *mut pt_regs, idx: i32) -> usize;
    fn get_addr_ref_16(insn: *mut insn, regs: *mut pt_regs) -> *mut u8;
    fn get_addr_ref_32(insn: *mut insn, regs: *mut pt_regs) -> *mut u8;
    fn get_addr_ref_64(insn: *mut insn, regs: *mut pt_regs) -> *mut u8;
}

pub unsafe fn insn_decode_from_regs(insn: *mut insn, regs: *mut pt_regs, buf: *mut u8, size: i32) -> bool {
    insn_init(insn, buf, size, user_64bit_mode(regs));
    let defs = insn_get_code_seg_params(regs);
    if defs == -EINVAL { return false; }
    (*insn).addr_bytes = INSN_CODE_SEG_ADDR_SZ!(defs);
    (*insn).opnd_bytes = INSN_CODE_SEG_OPND_SZ!(defs);
    if insn_get_length(insn) != 0 || size < (*insn).length { return false; }
    true
}

pub unsafe fn insn_decode_mmio(insn: *mut insn, bytes: *mut i32) -> insn_mmio_type {
    *bytes = 0;
    if insn_get_opcode(insn) != 0 { return INSN_MMIO_DECODE_FAILED; }
    match (*insn).opcode.bytes[0] {
        0x88 => { *bytes = 1; INSN_MMIO_WRITE }
        0x89 => { *bytes = (*insn).opnd_bytes; INSN_MMIO_WRITE }
        0xc6 => { *bytes = 1; INSN_MMIO_WRITE_IMM }
        0xc7 => { *bytes = (*insn).opnd_bytes; INSN_MMIO_WRITE_IMM }
        0x8a => { *bytes = 1; INSN_MMIO_READ }
        0x8b => { *bytes = (*insn).opnd_bytes; INSN_MMIO_READ }
        0xa4 => { *bytes = 1; INSN_MMIO_MOVS }
        0xa5 => { *bytes = (*insn).opnd_bytes; INSN_MMIO_MOVS }
        0x0f => match (*insn).opcode.bytes[1] {
            0xb6 | 0xbe => { *bytes = 1; if (*insn).opcode.bytes[1] == 0xb6 { INSN_MMIO_READ_ZERO_EXTEND } else { INSN_MMIO_READ_SIGN_EXTEND } }
            0xb7 | 0xbf => { *bytes = 2; if (*insn).opcode.bytes[1] == 0xb7 { INSN_MMIO_READ_ZERO_EXTEND } else { INSN_MMIO_READ_SIGN_EXTEND } }
            _ => INSN_MMIO_DECODE_FAILED,
        },
        _ => INSN_MMIO_DECODE_FAILED,
    }
}

pub unsafe fn insn_is_nop(insn: *mut insn) -> bool {
    let mut rep = false; let mut p: insn_byte_t = 0;
    if (*insn).vex_prefix.nbytes != 0 { return false; }
    for_each_insn_prefix!(insn, p, { if p == 0xf3 { rep = true; } });
    let op = (*insn).opcode.bytes[0];
    match op {
        0x90 => !rep && !X86_REX_B!((*insn).rex_prefix.value),
        0xe9 | 0xeb => (*insn).immediate.value == 0,
        0x89 => (*insn).opnd_bytes == 4 * (1 + (*insn).x86_64 as u8) && X86_MODRM_MOD!((*insn).modrm.value) == 3 && X86_MODRM_REG!((*insn).modrm.value) == X86_MODRM_RM!((*insn).modrm.value),
        0x0f => (*insn).opcode.bytes[1] == 0x1f && X86_MODRM_REG!((*insn).modrm.value) == 0,
        _ => false,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
