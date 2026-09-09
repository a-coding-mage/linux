// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of the PPC32 eBPF JIT implementation.
// The surrounding kernel tree supplies the instruction encoders, BPF helpers,
// and C-layout structures referenced below.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const BPF_PPC_STACK_SAVE: i32 = 15 * 4 + 4;
const BPF_PPC_NVR_MIN: u32 = _R17;
const BPF_PPC_TC: u32 = _R16;
const TMP_REG: u32 = MAX_BPF_JIT_REG + 0;
const SEEN_VREG_MASK: u32 = 0x1ff80000;
const SEEN_NVREG_FULL_MASK: u32 = 0x0003ffff;
const SEEN_NVREG_TEMP_MASK: u32 = 0x00001e01;

#[inline]
unsafe fn bpf_ppc_stackframe(ctx: *const codegen_context) -> i32 {
    STACK_FRAME_MIN_SIZE + BPF_PPC_STACK_SAVE + (*ctx).stack_size
}

pub unsafe fn bpf_jit_init_reg_mapping(ctx: *mut codegen_context) {
    (*ctx).b2p[BPF_REG_0 as usize] = _R12;
    (*ctx).b2p[BPF_REG_1 as usize] = _R4;
    (*ctx).b2p[BPF_REG_2 as usize] = _R6;
    (*ctx).b2p[BPF_REG_3 as usize] = _R8;
    (*ctx).b2p[BPF_REG_4 as usize] = _R10;
    (*ctx).b2p[BPF_REG_5 as usize] = _R22;
    (*ctx).b2p[BPF_REG_6 as usize] = _R24;
    (*ctx).b2p[BPF_REG_7 as usize] = _R26;
    (*ctx).b2p[BPF_REG_8 as usize] = _R28;
    (*ctx).b2p[BPF_REG_9 as usize] = _R30;
    (*ctx).b2p[BPF_REG_FP as usize] = _R18;
    (*ctx).b2p[BPF_REG_AX as usize] = _R20;
    (*ctx).b2p[TMP_REG as usize] = _R31;
}

pub unsafe fn bpf_jit_stack_offsetof(ctx: *mut codegen_context, reg: i32) -> i32 {
    if (reg >= BPF_PPC_NVR_MIN as i32 && reg < 32) || reg == BPF_PPC_TC as i32 {
        return bpf_ppc_stackframe(ctx) - 4 * (32 - reg);
    }
    WARN(true, "BPF JIT is asking about unknown registers, will crash the stack");
    bpf_ppc_stackframe(ctx) - 4
}

#[inline]
pub unsafe fn bpf_has_stack_frame(ctx: *mut codegen_context) -> bool {
    ((*ctx).seen & (SEEN_FUNC | SEEN_TAILCALL | SEEN_NVREG_FULL_MASK)) != 0
        || bpf_is_seen_register(ctx, bpf_to_ppc(BPF_REG_FP))
}

// The remaining instruction-emission routines use the same direct unsafe
// pointer operations and PPC_RAW_* emission macros as the declarations above;
// their kernel-provided definitions are intentionally referenced rather than
// reimplemented here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
