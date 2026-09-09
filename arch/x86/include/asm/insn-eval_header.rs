/*
 * A collection of utility functions for x86 instruction analysis to be
 * used in a kernel context. Useful when, for instance, making sense
 * of the registers indicated by operands.
 *
 * C header dependencies and build-time type definitions are supplied by
 * the surrounding translation unit.
 */

#[allow(non_camel_case_types)]
pub struct pt_regs;

#[allow(non_camel_case_types)]
pub struct insn;

pub const fn INSN_CODE_SEG_ADDR_SZ(params: u32) -> u32 {
    (params >> 4) & 0xf
}

pub const fn INSN_CODE_SEG_OPND_SZ(params: u32) -> u32 {
    params & 0xf
}

pub const fn INSN_CODE_SEG_PARAMS(oper_sz: u32, addr_sz: u32) -> u32 {
    oper_sz | (addr_sz << 4)
}

extern "C" {
    pub fn pt_regs_offset(regs: *mut pt_regs, regno: i32) -> i32;

    pub fn insn_has_rep_prefix(insn: *mut insn) -> bool;
    pub fn insn_get_addr_ref(insn: *mut insn, regs: *mut pt_regs) -> *mut core::ffi::c_void;
    pub fn insn_get_modrm_rm_off(insn: *mut insn, regs: *mut pt_regs) -> i32;
    pub fn insn_get_modrm_reg_off(insn: *mut insn, regs: *mut pt_regs) -> i32;
    pub fn insn_get_modrm_reg_ptr(insn: *mut insn, regs: *mut pt_regs) -> *mut usize;
    pub fn insn_get_seg_base(regs: *mut pt_regs, seg_reg_idx: i32) -> usize;
    pub fn insn_get_code_seg_params(regs: *mut pt_regs) -> i32;
    pub fn insn_get_effective_ip(regs: *mut pt_regs, ip: *mut usize) -> i32;
    pub fn insn_fetch_from_user(regs: *mut pt_regs, buf: *mut u8) -> i32;
    pub fn insn_fetch_from_user_inatomic(regs: *mut pt_regs, buf: *mut u8) -> i32;
    pub fn insn_decode_from_regs(
        insn: *mut insn,
        regs: *mut pt_regs,
        buf: *mut u8,
        buf_size: i32,
    ) -> bool;

    pub fn insn_decode_mmio(insn: *mut insn, bytes: *mut i32) -> insn_mmio_type;

    pub fn insn_is_nop(insn: *mut insn) -> bool;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum insn_mmio_type {
    INSN_MMIO_DECODE_FAILED,
    INSN_MMIO_WRITE,
    INSN_MMIO_WRITE_IMM,
    INSN_MMIO_READ,
    INSN_MMIO_READ_ZERO_EXTEND,
    INSN_MMIO_READ_SIGN_EXTEND,
    INSN_MMIO_MOVS,
}

/*
 * Write @val into *@reg following the x86 rules for writes to
 * general-purpose registers (Intel SDM Vol. 1, "General-Purpose
 * Registers in 64-Bit Mode"): an 8- or 16-bit write leaves the rest of
 * the register untouched, a 32-bit write zero-extends the result into
 * the upper 32 bits, and a 64-bit write replaces the whole register.
 *
 * @bytes is the width of the write, not a property of the instruction:
 * an instruction that, say, sign-extends a 32-bit immediate into a
 * 64-bit register does a 64-bit write here.
 *
 * @reg need not be 8-byte aligned: KVM's instruction emulator offsets
 * the pointer by one byte to address the high-byte registers (AH, CH,
 * DH, BH). Use narrow stores for the sub-word cases so the access
 * width matches @bytes and the adjacent bytes are left alone.
 */
pub unsafe fn insn_assign_reg(reg: *mut usize, val: u64, bytes: i32) {
    match bytes {
        1 => {
            *(reg as *mut u8) = val as u8;
        }
        2 => {
            *(reg as *mut u16) = val as u16;
        }
        4 => {
            /* A 32-bit write zero-extends into the upper 32 bits. */
            *reg = val as u32 as usize;
        }
        8 => {
            *reg = val as usize;
        }
        _ => {}
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
