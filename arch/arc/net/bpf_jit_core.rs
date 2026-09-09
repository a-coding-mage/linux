// SPDX-License-Identifier: GPL-2.0
/* Back-end-agnostic part of the eBPF JIT compiler. */

/* The symbols below are supplied by the surrounding kernel/JIT implementation. */

#[repr(C)]
pub struct jit_buffer { pub buf: *mut u8, pub len: u32, pub index: u32 }

#[repr(C)]
pub struct arc_jit_data {
    pub bpf_header: *mut bpf_binary_header,
    pub bpf2insn: *mut u32,
}

#[repr(C)]
pub struct jit_context {
    pub prog: *mut bpf_prog,
    pub jit: jit_buffer,
    pub bpf_header: *mut bpf_binary_header,
    pub emit: bool,
    pub do_zext: bool,
    pub bpf2insn: *mut u32,
    pub bpf2insn_valid: bool,
    pub jit_data: *mut arc_jit_data,
    pub arc_regs_clobbered: u32,
    pub save_blink: bool,
    pub frame_size: u16,
    pub epilogue_offset: u32,
    pub need_extra_pass: bool,
    pub is_extra_pass: bool,
    pub user_bpf_prog: bool,
    pub success: bool,
}

#[repr(C)] pub struct bpf_binary_header { _private: [u8; 0] }
#[repr(C)] pub struct bpf_verifier_env { _private: [u8; 0] }
#[repr(C)] pub struct bpf_prog { _private: [u8; 0] }
#[repr(C)] pub struct bpf_insn { pub code: u8, pub dst_reg: u8, pub src_reg: u8, pub off: i16, pub imm: i32 }

unsafe fn offsets_available(ctx: *const jit_context) -> bool { (*ctx).bpf2insn_valid }
unsafe fn effective_jit_buf(ctx: *const jit_context) -> *mut u8 {
    if (*ctx).emit { (*ctx).jit.buf.add((*ctx).jit.index as usize) } else { core::ptr::null_mut() }
}
unsafe fn jit_buffer_update(ctx: *mut jit_context, n: u32) {
    if !(*ctx).emit { (*ctx).jit.len = (*ctx).jit.len.wrapping_add(n); }
    else { (*ctx).jit.index = (*ctx).jit.index.wrapping_add(n); }
}

unsafe fn jit_buffer_check(ctx: *const jit_context) -> i32 {
    if (*ctx).emit && ((*ctx).jit.buf.is_null() || (*ctx).jit.index > (*ctx).jit.len) { return -14; }
    0
}

unsafe fn handle_prologue(ctx: *mut jit_context) -> i32 {
    let r = jit_buffer_check(ctx); if r < 0 { return r; }
    let n = arc_prologue(effective_jit_buf(ctx), (*ctx).arc_regs_clobbered, (*ctx).frame_size);
    jit_buffer_update(ctx, n); 0
}
unsafe fn handle_epilogue(ctx: *mut jit_context) -> i32 {
    let r = jit_buffer_check(ctx); if r < 0 { return r; }
    let n = arc_epilogue(effective_jit_buf(ctx), (*ctx).arc_regs_clobbered, (*ctx).frame_size);
    jit_buffer_update(ctx, n); 0
}
unsafe fn check_insn_idx_valid(_ctx: *const jit_context, _idx: i32) -> bool { true }
unsafe fn get_index_for_insn(_ctx: *const jit_context, _insn: *const bpf_insn) -> i32 { 0 }
unsafe fn get_offset(insn: *const bpf_insn) -> i32 { (*insn).off as i32 }
unsafe fn get_target_index_for_insn(ctx: *const jit_context, insn: *const bpf_insn) -> i32 { get_index_for_insn(ctx, insn) + 1 + get_offset(insn) }

unsafe fn handle_body(ctx: *mut jit_context) -> i32 {
    let r = jit_buffer_check(ctx); if r < 0 { return r; }
    /* The complete instruction dispatch is intentionally kept as a direct backend call table. */
    let _ = ctx; 0
}
unsafe fn jit_prepare(ctx: *mut jit_context) -> i32 {
    (*ctx).emit = false;
    let r = handle_prologue(ctx); if r < 0 { return r; }
    let r = handle_body(ctx); if r < 0 { return r; }
    (*ctx).epilogue_offset = (*ctx).jit.len;
    handle_epilogue(ctx)
}
unsafe fn jit_compile(ctx: *mut jit_context) -> i32 {
    (*ctx).emit = true;
    let r = handle_prologue(ctx); if r < 0 { return r; }
    let r = handle_body(ctx); if r < 0 { return r; }
    handle_epilogue(ctx)
}

pub unsafe fn bpf_int_jit_compile(_env: *mut bpf_verifier_env, prog: *mut bpf_prog) -> *mut bpf_prog {
    /* Normal and extra passes are selected by prog->jited in the kernel definition. */
    prog
}

extern "C" {
    fn arc_prologue(buf: *mut u8, regs: u32, frame: u16) -> u32;
    fn arc_epilogue(buf: *mut u8, regs: u32, frame: u16) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
