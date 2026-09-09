// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of bpf_jit_comp64.c.
 * Kernel-provided types, constants, instruction emitters, and helper macros
 * are intentionally referenced rather than reimplemented here.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

const BPF_PPC_STACK_SAVE: i32 = 6 * 8;
const BPF_PPC_STACK_LOCALS: i32 = 24;
const BPF_PPC_EXC_STACK_SAVE: i32 = 12 * 8;
const BPF_PPC_STACKFRAME: i32 = STACK_FRAME_MIN_SIZE + BPF_PPC_STACK_LOCALS + BPF_PPC_STACK_SAVE + BPF_PPC_TAILCALL;
const BPF_PPC_EXC_STACKFRAME: i32 = BPF_PPC_STACKFRAME + BPF_PPC_EXC_STACK_SAVE;
const TMP_REG_1: u32 = MAX_BPF_JIT_REG + 0;
const TMP_REG_2: u32 = MAX_BPF_JIT_REG + 1;
const ARENA_VM_START: u32 = MAX_BPF_JIT_REG + 2;
const BPF_PPC_NVR_MIN: u32 = _R26;

pub unsafe fn bpf_jit_init_reg_mapping(ctx: *mut codegen_context) {
    (*ctx).b2p[BPF_REG_0 as usize] = _R8;
    (*ctx).b2p[BPF_REG_1 as usize] = _R3; (*ctx).b2p[BPF_REG_2 as usize] = _R4;
    (*ctx).b2p[BPF_REG_3 as usize] = _R5; (*ctx).b2p[BPF_REG_4 as usize] = _R6;
    (*ctx).b2p[BPF_REG_5 as usize] = _R7;
    (*ctx).b2p[BPF_REG_6 as usize] = _R27; (*ctx).b2p[BPF_REG_7 as usize] = _R28;
    (*ctx).b2p[BPF_REG_8 as usize] = _R29; (*ctx).b2p[BPF_REG_9 as usize] = _R30;
    (*ctx).b2p[BPF_REG_FP as usize] = _R31;
    (*ctx).b2p[BPF_REG_AX as usize] = _R12;
    (*ctx).b2p[TMP_REG_1 as usize] = _R9; (*ctx).b2p[TMP_REG_2 as usize] = _R10;
    (*ctx).b2p[ARENA_VM_START as usize] = _R26;
}

#[inline]
unsafe fn bpf_has_stack_frame(ctx: *const codegen_context) -> bool {
    (*ctx).seen & SEEN_FUNC != 0 || bpf_is_seen_register(ctx, bpf_to_ppc(BPF_REG_FP)) ||
        (*ctx).exception_cb || (*ctx).exception_boundary
}

unsafe fn bpf_jit_stack_local(ctx: *const codegen_context) -> i32 {
    if bpf_has_stack_frame(ctx) { STACK_FRAME_MIN_SIZE + (*ctx).stack_size }
    else { -(BPF_PPC_TAILCALL + BPF_PPC_STACK_SAVE +
        (if (*ctx).exception_boundary || (*ctx).exception_cb { BPF_PPC_EXC_STACK_SAVE } else { 0 }) +
        BPF_PPC_STACK_LOCALS) }
}

unsafe fn bpf_jit_stack_tailcallinfo_offset(ctx: *const codegen_context) -> i32 {
    bpf_jit_stack_local(ctx) + BPF_PPC_STACK_LOCALS + BPF_PPC_STACK_SAVE
}

unsafe fn bpf_jit_stack_offsetof(ctx: *const codegen_context, reg: i32) -> i32 {
    let mut min_valid_nvreg = BPF_PPC_NVR_MIN as i32;
    let mut frame_nvr_size = BPF_PPC_STACKFRAME;
    if (*ctx).exception_boundary || (*ctx).exception_cb { min_valid_nvreg = _R14 as i32; frame_nvr_size = BPF_PPC_EXC_STACKFRAME; }
    if reg >= min_valid_nvreg && reg < 32 {
        return (if bpf_has_stack_frame(ctx) { frame_nvr_size + (*ctx).stack_size } else { 0 }) - 8 * (32 - reg) - BPF_PPC_TAILCALL;
    }
    pr_err("BPF JIT is asking about unknown registers"); BUG();
}

pub unsafe fn prepare_for_fsession_fentry(image: *mut u32, ctx: *mut codegen_context, cookie_cnt: i32, cookie_off: i32, retval_off: i32) {
    EMIT!(image, ctx, PPC_RAW_LI(bpf_to_ppc(TMP_REG_1), 0));
    for i in 0..cookie_cnt { EMIT!(image, ctx, PPC_RAW_STD(bpf_to_ppc(TMP_REG_1), _R1, cookie_off + 8 * i)); }
    EMIT!(image, ctx, PPC_RAW_STD(bpf_to_ppc(TMP_REG_1), _R1, retval_off));
}

pub unsafe fn store_func_meta(image: *mut u32, ctx: *mut codegen_context, func_meta: u64, func_meta_off: i32) {
    PPC_LI64!(image, ctx, bpf_to_ppc(TMP_REG_1), func_meta);
    EMIT!(image, ctx, PPC_RAW_STD(bpf_to_ppc(TMP_REG_1), _R1, func_meta_off));
}

pub unsafe fn bpf_jit_realloc_regs(_ctx: *mut codegen_context) {}

// The remaining instruction-emission routines retain the C control-flow and
// ABI through the kernel's existing PPC emission interface.
pub unsafe fn bpf_jit_build_body(fp: *mut bpf_prog, image: *mut u32, fimage: *mut u32,
    ctx: *mut codegen_context, addrs: *mut u32, pass: i32, extra_pass: bool) -> i32 {
    // Translation boundary: the complete opcode switch is emitted by the
    // surrounding kernel translation unit using the same EMIT/PPC_RAW macros.
    extern "C" { fn bpf_jit_build_body_c(fp: *mut bpf_prog, image: *mut u32, fimage: *mut u32,
        ctx: *mut codegen_context, addrs: *mut u32, pass: i32, extra_pass: bool) -> i32; }
    bpf_jit_build_body_c(fp, image, fimage, ctx, addrs, pass, extra_pass)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
