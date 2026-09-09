// SPDX-License-Identifier: GPL-2.0
/*
 * Common functionality for HPPA32 and HPPA64 BPF JIT compilers
 *
 * Copyright (c) 2023 Helge Deller <deller@gmx.de>
 *
 */

// Dependencies supplied by the surrounding kernel translation unit.

/* Number of iterations to try until offsets converge. */
const NR_JIT_ITERATIONS: i32 = 35;

unsafe fn build_body(
    ctx: *mut hppa_jit_context,
    extra_pass: bool,
    offset: *mut i32,
) -> i32 {
    let prog = (*ctx).prog;
    let mut i: i32;

    (*ctx).reg_seen_collect = true;
    i = 0;
    while i < (*prog).len {
        let insn = &(*prog).insnsi.add(i as usize);
        let ret: i32;

        ret = bpf_jit_emit_insn(insn as *const bpf_insn, ctx, extra_pass);
        /* BPF_LD | BPF_IMM | BPF_DW: skip the next instruction. */
        if ret > 0 {
            i += 1;
        }
        if !offset.is_null() {
            *offset.add(i as usize) = (*ctx).ninsns;
        }
        if ret < 0 {
            return ret;
        }
        i += 1;
    }
    (*ctx).reg_seen_collect = false;
    0
}

pub unsafe fn bpf_jit_needs_zext() -> bool {
    true
}

pub unsafe fn bpf_int_jit_compile(
    env: *mut bpf_verifier_env,
    prog: *mut bpf_prog,
) -> *mut bpf_prog {
    let mut prog_size: usize = 0;
    let mut extable_size: usize = 0;
    let mut extra_pass = false;
    let mut pass: i32 = 0;
    let mut prev_ninsns: i32 = 0;
    let mut prologue_len: i32;
    let mut i: i32;
    let mut jit_data: *mut hppa_jit_data;
    let ctx: *mut hppa_jit_context;

    let _ = env;
    if !(*prog).jit_requested {
        return prog;
    }

    jit_data = (*(*prog).aux).jit_data;
    if jit_data.is_null() {
        jit_data = kzalloc_obj::<hppa_jit_data>();
        if jit_data.is_null() {
            return prog;
        }
        (*(*prog).aux).jit_data = jit_data;
    }

    ctx = &mut (*jit_data).ctx;

    if !(*ctx).offset.is_null() {
        extra_pass = true;
        prog_size = core::mem::size_of::<u32>() * (*ctx).ninsns as usize;
        goto_skip_init_ctx!(skip_init_ctx);
    }

    (*ctx).prog = prog;
    (*ctx).offset = kzalloc_objs::<i32>((*prog).len as usize);
    if (*ctx).offset.is_null() {
        goto_out_err!(out_err);
    }
    i = 0;
    while i < (*prog).len {
        prev_ninsns += 20;
        *(*ctx).offset.add(i as usize) = prev_ninsns;
        i += 1;
    }

    i = 0;
    while i < NR_JIT_ITERATIONS {
        pass += 1;
        (*ctx).ninsns = 0;
        if build_body(ctx, extra_pass, (*ctx).offset) != 0 {
            goto_out_err!(out_err);
        }
        (*ctx).body_len = (*ctx).ninsns;
        bpf_jit_build_prologue(ctx);
        (*ctx).prologue_len = (*ctx).ninsns - (*ctx).body_len;
        (*ctx).epilogue_offset = (*ctx).ninsns;
        bpf_jit_build_epilogue(ctx);

        if (*ctx).ninsns == prev_ninsns {
            if !(*jit_data).header.is_null() {
                break;
            }
            /* obtain the actual image size */
            extable_size = (*(*prog).aux).num_exentries as usize
                * core::mem::size_of::<exception_table_entry>();
            prog_size = core::mem::size_of::<u32>() * (*ctx).ninsns as usize;

            (*jit_data).header = bpf_jit_binary_alloc(
                prog_size + extable_size,
                &mut (*jit_data).image,
                core::mem::size_of::<c_long>(),
                bpf_fill_ill_insns,
            );
            if (*jit_data).header.is_null() {
                goto_out_err!(out_err);
            }

            (*ctx).insns = (*jit_data).image as *mut u32;
            /*
             * Now, when the image is allocated, the image can
             * potentially shrink more (auipc/jalr -> jal).
             */
        }
        prev_ninsns = (*ctx).ninsns;
        i += 1;
    }

    if i == NR_JIT_ITERATIONS {
        pr_err!("bpf-jit: image did not converge in <%d passes!\n", i);
        if !(*jit_data).header.is_null() {
            bpf_jit_binary_free((*jit_data).header);
        }
        goto_out_err!(out_err);
    }

    if extable_size != 0 {
        (*(*prog).aux).extable = (*ctx).insns.add(prog_size / core::mem::size_of::<u32>()) as *mut core::ffi::c_void;
    }

    skip_init_ctx: {
        pass += 1;
        (*ctx).ninsns = 0;

        bpf_jit_build_prologue(ctx);
        if build_body(ctx, extra_pass, core::ptr::null_mut()) != 0 {
            bpf_jit_binary_free((*jit_data).header);
            goto_out_err!(out_err);
        }
        bpf_jit_build_epilogue(ctx);

        if HPPA_JIT_DEBUG || bpf_jit_enable > 1 {
            if HPPA_JIT_DUMP {
                bpf_jit_dump((*prog).len, prog_size, pass, (*ctx).insns);
            }
            if HPPA_JIT_REBOOT {
                machine_restart("");
            }
        }

        if !(*prog).is_func || extra_pass {
            if bpf_jit_binary_lock_ro((*jit_data).header) != 0 {
                bpf_jit_binary_free((*jit_data).header);
                goto_out_err!(out_err);
            }
            bpf_flush_icache((*jit_data).header, (*ctx).insns.add((*ctx).ninsns as usize));
        }

        (*prog).bpf_func = (*ctx).insns as *mut core::ffi::c_void;
        (*prog).jited = 1;
        (*prog).jited_len = prog_size;

        if !(*prog).is_func || extra_pass {
            prologue_len = (*ctx).epilogue_offset - (*ctx).body_len;
            i = 0;
            while i < (*prog).len {
                *(*ctx).offset.add(i as usize) += prologue_len;
                i += 1;
            }
            bpf_prog_fill_jited_linfo(prog, (*ctx).offset);
        }
    }

    if HPPA_JIT_REBOOT {
        machine_restart("");
    }

    kfree((*ctx).offset as *mut core::ffi::c_void);
    kfree(jit_data as *mut core::ffi::c_void);
    (*(*prog).aux).jit_data = core::ptr::null_mut();
    return prog;

    out_err: {
        if extra_pass {
            (*prog).bpf_func = core::ptr::null_mut();
            (*prog).jited = 0;
            (*prog).jited_len = 0;
        }
        kfree((*ctx).offset as *mut core::ffi::c_void);
        kfree(jit_data as *mut core::ffi::c_void);
        (*(*prog).aux).jit_data = core::ptr::null_mut();
    }
    prog
}

pub unsafe fn hppa_div64(mut div: u64, divisor: u64) -> u64 {
    div = div64_u64(div, divisor);
    div
}

pub unsafe fn hppa_div64_rem(div: u64, divisor: u64) -> u64 {
    let mut rem: u64 = 0;
    div64_u64_rem(div, divisor, &mut rem);
    rem
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
