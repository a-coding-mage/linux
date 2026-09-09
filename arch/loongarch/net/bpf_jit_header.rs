/* SPDX-License-Identifier: GPL-2.0-only */
/* BPF JIT compiler for LoongArch */

// C includes are supplied by the surrounding translation unit.

#[repr(C)]
pub struct jit_ctx {
    pub prog: *const bpf_prog,
    pub idx: u32,
    pub flags: u32,
    pub epilogue_offset: u32,
    pub offset: *mut u32,
    pub num_exentries: i32,
    pub image: *mut loongarch_instruction,
    pub ro_image: *mut loongarch_instruction,
    pub stack_size: u32,
    pub arena_vm_start: u64,
    pub user_vm_start: u64,
}

#[repr(C)]
pub struct jit_data {
    pub header: *mut bpf_binary_header,
    pub ro_header: *mut bpf_binary_header,
    pub ctx: jit_ctx,
}

#[inline]
unsafe fn emit_nop(insn: *mut loongarch_instruction) {
    (*insn).word = INSN_NOP;
}

// The C token-pasting form emit_##func is represented by passing the emitter
// function explicitly, since stable Rust has no token concatenation macro.
macro_rules! emit_insn {
    ($ctx:expr, $func:path $(, $arg:expr)*) => {{
        unsafe {
            if !(*$ctx).image.is_null() {
                let insn = (*$ctx).image.add((*$ctx).idx as usize);
                $func(insn $(, $arg)*);
            }
            (*$ctx).idx += 1;
        }
    }};
}

macro_rules! is_signed_imm12 { ($val:expr) => { signed_imm_check($val, 12) }; }
macro_rules! is_signed_imm14 { ($val:expr) => { signed_imm_check($val, 14) }; }
macro_rules! is_signed_imm16 { ($val:expr) => { signed_imm_check($val, 16) }; }
macro_rules! is_signed_imm26 { ($val:expr) => { signed_imm_check($val, 26) }; }
macro_rules! is_signed_imm32 { ($val:expr) => { signed_imm_check($val, 32) }; }
macro_rules! is_signed_imm52 { ($val:expr) => { signed_imm_check($val, 52) }; }
macro_rules! is_unsigned_imm12 { ($val:expr) => { unsigned_imm_check($val, 12) }; }

#[inline]
unsafe fn bpf2la_offset(bpf_insn: i32, off: i32, ctx: *const jit_ctx) -> i32 {
    let bpf_insn = bpf_insn + 1;
    (*ctx).offset.offset((bpf_insn + off) as isize).read() as i32
        - ((*ctx).offset.offset(bpf_insn as isize).read() as i32 - 1)
}

#[inline]
unsafe fn epilogue_offset(ctx: *const jit_ctx) -> i32 {
    (*ctx).epilogue_offset as i32 - (*ctx).idx as i32
}

#[inline]
unsafe fn emit_zext_32(ctx: *mut jit_ctx, reg: loongarch_gpr, is32: bool) {
    if is32 { emit_insn!(ctx, emit_lu32id, reg, 0); }
}

#[inline]
unsafe fn emit_sext_32(ctx: *mut jit_ctx, reg: loongarch_gpr, is32: bool) {
    if is32 { emit_insn!(ctx, emit_addiw, reg, reg, 0); }
}

#[inline]
unsafe fn emit_abi_ext(ctx: *mut jit_ctx, reg: i32, size: u8, sign: bool) {
    if !sign && (size == 1 || size == 2) { return; }
    match size {
        1 => emit_insn!(ctx, emit_extwb, reg, reg),
        2 => emit_insn!(ctx, emit_extwh, reg, reg),
        4 => emit_insn!(ctx, emit_addiw, reg, reg, 0),
        8 => (),
        _ => pr_warn!("bpf_jit: invalid size %d for extension\n", size),
    }
}

#[inline]
unsafe fn move_addr(ctx: *mut jit_ctx, rd: loongarch_gpr, addr: u64) {
    let imm_31_12 = (addr >> 12) & 0xfffff;
    emit_insn!(ctx, emit_lu12iw, rd, imm_31_12);
    let imm_11_0 = addr & 0xfff;
    emit_insn!(ctx, emit_ori, rd, rd, imm_11_0);
    let imm_51_32 = (addr >> 32) & 0xfffff;
    emit_insn!(ctx, emit_lu32id, rd, imm_51_32);
    let imm_63_52 = (addr >> 52) & 0xfff;
    emit_insn!(ctx, emit_lu52id, rd, rd, imm_63_52);
}

#[inline]
unsafe fn move_imm(ctx: *mut jit_ctx, rd: loongarch_gpr, imm: i64, is32: bool) {
    if imm == 0 { emit_insn!(ctx, emit_or, rd, LOONGARCH_GPR_ZERO, LOONGARCH_GPR_ZERO); return; }
    if is_signed_imm12!(imm) { emit_insn!(ctx, emit_addiw, rd, LOONGARCH_GPR_ZERO, imm); emit_zext_32(ctx, rd, is32); return; }
    if is_unsigned_imm12!(imm) { emit_insn!(ctx, emit_ori, rd, LOONGARCH_GPR_ZERO, imm); return; }
    let imm_63_52 = ((imm as u64) >> 52) & 0xfff;
    let imm_51_0 = (imm as u64) & 0xfffffffffffff;
    if imm_63_52 != 0 && imm_51_0 == 0 { emit_insn!(ctx, emit_lu52id, rd, LOONGARCH_GPR_ZERO, imm_63_52); return; }
    let imm_31_12 = ((imm as u64) >> 12) & 0xfffff;
    emit_insn!(ctx, emit_lu12iw, rd, imm_31_12);
    let imm_11_0 = (imm as u64) & 0xfff;
    if imm_11_0 != 0 { emit_insn!(ctx, emit_ori, rd, rd, imm_11_0); }
    if !is_signed_imm32!(imm) && imm_51_0 != 0 {
        let imm_51_31 = ((imm as u64) >> 31) & 0x1fffff;
        if imm_51_31 != 0 && imm_51_31 != 0x1fffff {
            emit_insn!(ctx, emit_lu32id, rd, ((imm as u64) >> 32) & 0xfffff);
        }
        if !is_signed_imm52!(imm) { emit_insn!(ctx, emit_lu52id, rd, rd, imm_63_52); }
    }
    emit_zext_32(ctx, rd, is32);
}

#[inline]
unsafe fn move_reg(ctx: *mut jit_ctx, rd: loongarch_gpr, rj: loongarch_gpr) {
    emit_insn!(ctx, emit_or, rd, rj, LOONGARCH_GPR_ZERO);
}

#[inline]
fn invert_jmp_cond(cond: u8) -> i32 {
    match cond { BPF_JEQ => BPF_JNE as i32, BPF_JNE | BPF_JSET => BPF_JEQ as i32,
        BPF_JGT => BPF_JLE as i32, BPF_JGE => BPF_JLT as i32, BPF_JLT => BPF_JGE as i32,
        BPF_JLE => BPF_JGT as i32, BPF_JSGT => BPF_JSLE as i32, BPF_JSGE => BPF_JSLT as i32,
        BPF_JSLT => BPF_JSGE as i32, BPF_JSLE => BPF_JSGT as i32, _ => -1 }
}

#[inline]
unsafe fn cond_jmp_offset(ctx: *mut jit_ctx, cond: u8, rj: loongarch_gpr, rd: loongarch_gpr, off: i32) {
    match cond { BPF_JEQ => emit_insn!(ctx, emit_beq, rj, rd, off), BPF_JNE | BPF_JSET => emit_insn!(ctx, emit_bne, rj, rd, off),
        BPF_JGT => emit_insn!(ctx, emit_bltu, rd, rj, off), BPF_JLT => emit_insn!(ctx, emit_bltu, rj, rd, off),
        BPF_JGE => emit_insn!(ctx, emit_bgeu, rj, rd, off), BPF_JLE => emit_insn!(ctx, emit_bgeu, rd, rj, off),
        BPF_JSGT => emit_insn!(ctx, emit_blt, rd, rj, off), BPF_JSLT => emit_insn!(ctx, emit_blt, rj, rd, off),
        BPF_JSGE => emit_insn!(ctx, emit_bge, rj, rd, off), BPF_JSLE => emit_insn!(ctx, emit_bge, rd, rj, off), _ => () }
}

#[inline] unsafe fn cond_jmp_offs26(ctx: *mut jit_ctx, cond: u8, rj: loongarch_gpr, rd: loongarch_gpr, off: i32) {
    cond_jmp_offset(ctx, invert_jmp_cond(cond) as u8, rj, rd, 2); emit_insn!(ctx, emit_b, off);
}
#[inline] unsafe fn uncond_jmp_offs26(ctx: *mut jit_ctx, off: i32) { emit_insn!(ctx, emit_b, off); }
#[inline] unsafe fn emit_cond_jmp(ctx: *mut jit_ctx, cond: u8, rj: loongarch_gpr, rd: loongarch_gpr, off: i32) -> i32 {
    if is_signed_imm26!(off) { cond_jmp_offs26(ctx, cond, rj, rd, off); 0 } else { -EINVAL }
}
#[inline] unsafe fn emit_uncond_jmp(ctx: *mut jit_ctx, off: i32) -> i32 {
    if is_signed_imm26!(off) { uncond_jmp_offs26(ctx, off); 0 } else { -EINVAL }
}
#[inline] unsafe fn emit_tailcall_jmp(ctx: *mut jit_ctx, cond: u8, rj: loongarch_gpr, rd: loongarch_gpr, off: i32) -> i32 {
    if is_signed_imm16!(off) { cond_jmp_offset(ctx, cond, rj, rd, off); 0 } else { -EINVAL }
}

#[inline]
unsafe fn bpf_flush_icache(start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) {
    flush_icache_range(start as usize as u64, end as usize as u64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
