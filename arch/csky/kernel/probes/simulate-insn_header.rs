/* SPDX-License-Identifier: GPL-2.0+ */

// Declarations supplied by the surrounding kernel translation:
// probe_opcode_t, u32, struct pt_regs, and INSN_GOOD_NO_SLOT.

macro_rules! __CSKY_INSN_FUNCS {
    ($name:ident, $mask:expr, $val:expr) => {
        #[inline(always)]
        fn $name(code: probe_opcode_t) -> bool {
            // C's BUILD_BUG_ON(~mask & val) is satisfied by all definitions below.
            (code & ($mask as probe_opcode_t)) == ($val as probe_opcode_t)
        }
    };
}

macro_rules! CSKY_INSN_SET_SIMULATE {
    ($api:expr, br16, $code:expr) => { if csky_insn_is_br16($code) { $api.handler = simulate_br16; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, bt16, $code:expr) => { if csky_insn_is_bt16($code) { $api.handler = simulate_bt16; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, bf16, $code:expr) => { if csky_insn_is_bf16($code) { $api.handler = simulate_bf16; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, jmp16, $code:expr) => { if csky_insn_is_jmp16($code) { $api.handler = simulate_jmp16; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, jsr16, $code:expr) => { if csky_insn_is_jsr16($code) { $api.handler = simulate_jsr16; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, lrw16, $code:expr) => { if csky_insn_is_lrw16($code) { $api.handler = simulate_lrw16; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, pop16, $code:expr) => { if csky_insn_is_pop16($code) { $api.handler = simulate_pop16; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, br32, $code:expr) => { if csky_insn_is_br32($code) { $api.handler = simulate_br32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, bt32, $code:expr) => { if csky_insn_is_bt32($code) { $api.handler = simulate_bt32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, bf32, $code:expr) => { if csky_insn_is_bf32($code) { $api.handler = simulate_bf32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, jmp32, $code:expr) => { if csky_insn_is_jmp32($code) { $api.handler = simulate_jmp32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, jsr32, $code:expr) => { if csky_insn_is_jsr32($code) { $api.handler = simulate_jsr32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, lrw32, $code:expr) => { if csky_insn_is_lrw32($code) { $api.handler = simulate_lrw32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, pop32, $code:expr) => { if csky_insn_is_pop32($code) { $api.handler = simulate_pop32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, bez32, $code:expr) => { if csky_insn_is_bez32($code) { $api.handler = simulate_bez32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, bnez32, $code:expr) => { if csky_insn_is_bnez32($code) { $api.handler = simulate_bnez32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, bnezad32, $code:expr) => { if csky_insn_is_bnezad32($code) { $api.handler = simulate_bnezad32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, bhsz32, $code:expr) => { if csky_insn_is_bhsz32($code) { $api.handler = simulate_bhsz32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, bhz32, $code:expr) => { if csky_insn_is_bhz32($code) { $api.handler = simulate_bhz32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, blsz32, $code:expr) => { if csky_insn_is_blsz32($code) { $api.handler = simulate_blsz32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, blz32, $code:expr) => { if csky_insn_is_blz32($code) { $api.handler = simulate_blz32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, bsr32, $code:expr) => { if csky_insn_is_bsr32($code) { $api.handler = simulate_bsr32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, jmpi32, $code:expr) => { if csky_insn_is_jmpi32($code) { $api.handler = simulate_jmpi32; return INSN_GOOD_NO_SLOT; } };
    ($api:expr, jsri32, $code:expr) => { if csky_insn_is_jsri32($code) { $api.handler = simulate_jsri32; return INSN_GOOD_NO_SLOT; } };
}

__CSKY_INSN_FUNCS!(csky_insn_is_br16, 0xfc00, 0x0400);
__CSKY_INSN_FUNCS!(csky_insn_is_bt16, 0xfc00, 0x0800);
__CSKY_INSN_FUNCS!(csky_insn_is_bf16, 0xfc00, 0x0c00);
__CSKY_INSN_FUNCS!(csky_insn_is_jmp16, 0xffc3, 0x7800);
__CSKY_INSN_FUNCS!(csky_insn_is_jsr16, 0xffc3, 0x7801);
__CSKY_INSN_FUNCS!(csky_insn_is_lrw16, 0xfc00, 0x1000);
__CSKY_INSN_FUNCS!(csky_insn_is_pop16, 0xffe0, 0x1480);
__CSKY_INSN_FUNCS!(csky_insn_is_br32, 0x0000ffff, 0x0000e800);
__CSKY_INSN_FUNCS!(csky_insn_is_bt32, 0x0000ffff, 0x0000e860);
__CSKY_INSN_FUNCS!(csky_insn_is_bf32, 0x0000ffff, 0x0000e840);
__CSKY_INSN_FUNCS!(csky_insn_is_jmp32, 0xffffffe0, 0x0000e8c0);
__CSKY_INSN_FUNCS!(csky_insn_is_jsr32, 0xffffffe0, 0x0000e8e0);
__CSKY_INSN_FUNCS!(csky_insn_is_lrw32, 0x0000ffe0, 0x0000ea80);
__CSKY_INSN_FUNCS!(csky_insn_is_pop32, 0xfe00ffff, 0x0000ebc0);
__CSKY_INSN_FUNCS!(csky_insn_is_bez32, 0x0000ffe0, 0x0000e900);
__CSKY_INSN_FUNCS!(csky_insn_is_bnez32, 0x0000ffe0, 0x0000e920);
__CSKY_INSN_FUNCS!(csky_insn_is_bnezad32, 0x0000ffe0, 0x0000e820);
__CSKY_INSN_FUNCS!(csky_insn_is_bhsz32, 0x0000ffe0, 0x0000e9a0);
__CSKY_INSN_FUNCS!(csky_insn_is_bhz32, 0x0000ffe0, 0x0000e940);
__CSKY_INSN_FUNCS!(csky_insn_is_blsz32, 0x0000ffe0, 0x0000e960);
__CSKY_INSN_FUNCS!(csky_insn_is_blz32, 0x0000ffe0, 0x0000e980);
__CSKY_INSN_FUNCS!(csky_insn_is_bsr32, 0x0000fc00, 0x0000e000);
__CSKY_INSN_FUNCS!(csky_insn_is_jmpi32, 0x0000ffff, 0x0000eac0);
__CSKY_INSN_FUNCS!(csky_insn_is_jsri32, 0x0000ffff, 0x0000eae0);

extern "C" {
    fn simulate_br16(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_bt16(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_bf16(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_jmp16(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_jsr16(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_lrw16(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_pop16(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_br32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_bt32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_bf32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_jmp32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_jsr32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_lrw32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_pop32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_bez32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_bnez32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_bnezad32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_bhsz32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_bhz32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_blsz32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_blz32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_bsr32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_jmpi32(opcode: u32, addr: i64, regs: *mut pt_regs);
    fn simulate_jsri32(opcode: u32, addr: i64, regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
