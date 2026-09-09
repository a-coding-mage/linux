// SPDX-License-Identifier: GPL-2.0
/*
 * Common functionality for RV32 and RV64 BPF JIT compilers
 *
 * Copyright (c) 2019 Björn Töpel <bjorn.topel@gmail.com>
 */

// Kernel dependencies supplied by the surrounding Rust translation unit.

const NR_JIT_ITERATIONS: i32 = 32;

unsafe fn build_body(
    ctx: *mut rv_jit_context,
    extra_pass: bool,
    offset: *mut i32,
) -> i32 {
    let prog = (*ctx).prog;
    let mut i: i32 = 0;

    while i < (*prog).len {
        let insn = (*prog).insnsi.add(i as usize);
        let ret = bpf_jit_emit_insn(insn, ctx, extra_pass);
        if ret > 0 {
            i += 1; /* skip the next instruction */
        }
        if !offset.is_null() {
            *offset.add(i as usize) = (*ctx).ninsns;
        }
        if ret < 0 {
            return ret;
        }
        i += 1;
    }
    0
}

pub unsafe fn bpf_jit_needs_zext() -> bool {
    true
}

pub unsafe fn bpf_int_jit_compile(
    env: *mut bpf_verifier_env,
    prog: *mut bpf_prog,
) -> *mut bpf_prog {
    let _ = env;
    let mut prog_size: usize = 0;
    let mut extable_size: usize = 0;
    let mut extra_pass = false;
    let mut pass: i32 = 0;
    let mut prev_ninsns: i32 = 0;
    let mut i: i32 = 0;
    let mut jit_data: *mut rv_jit_data;
    let ctx: *mut rv_jit_context;

    if !(*prog).jit_requested {
        return prog;
    }

    jit_data = (*(*prog).aux).jit_data;
    if jit_data.is_null() {
        jit_data = kzalloc_obj::<rv_jit_data>();
        if jit_data.is_null() {
            return prog;
        }
        (*(*prog).aux).jit_data = jit_data;
    }

    ctx = &mut (*jit_data).ctx;

    if !(*ctx).offset.is_null() {
        extra_pass = true;
        prog_size = core::mem::size_of::<*mut u16>() * (*ctx).ninsns as usize;
        goto_skip_init_ctx!(skip_init_ctx);
    }

    (*ctx).arena_vm_start = bpf_arena_get_kern_vm_start((*(*prog).aux).arena);
    (*ctx).user_vm_start = bpf_arena_get_user_vm_start((*(*prog).aux).arena);
    (*ctx).prog = prog;
    (*ctx).offset = kvzalloc_objs::<i32>((*prog).len as usize);
    if (*ctx).offset.is_null() {
        goto_out_offset!(out_offset);
    }

    if build_body(ctx, extra_pass, core::ptr::null_mut()) != 0 {
        goto_out_offset!(out_offset);
    }

    i = 0;
    while i < (*prog).len {
        prev_ninsns += 32;
        *(*ctx).offset.add(i as usize) = prev_ninsns;
        i += 1;
    }

    i = 0;
    while i < NR_JIT_ITERATIONS {
        pass += 1;
        (*ctx).ninsns = 0;
        bpf_jit_build_prologue(ctx, bpf_is_subprog(prog));
        (*ctx).prologue_len = (*ctx).ninsns;
        if build_body(ctx, extra_pass, (*ctx).offset) != 0 {
            goto_out_offset!(out_offset);
        }
        (*ctx).epilogue_offset = (*ctx).ninsns;
        bpf_jit_build_epilogue(ctx);
        if (*ctx).ninsns == prev_ninsns {
            if !(*jit_data).header.is_null() {
                break;
            }
            extable_size = (*(*prog).aux).num_exentries as usize
                * core::mem::size_of::<exception_table_entry>();
            prog_size = core::mem::size_of::<u16>() * (*ctx).ninsns as usize;
            (*jit_data).ro_header = bpf_jit_binary_pack_alloc(
                prog_size + extable_size, &mut (*jit_data).ro_image,
                core::mem::size_of::<u32>(), &mut (*jit_data).header,
                &mut (*jit_data).image, bpf_fill_ill_insns,
                bpf_prog_was_classic(prog),
            );
            if (*jit_data).ro_header.is_null() {
                goto_out_offset!(out_offset);
            }
            (*ctx).ro_insns = (*jit_data).ro_image as *mut u16;
            (*ctx).insns = (*jit_data).image as *mut u16;
        }
        prev_ninsns = (*ctx).ninsns;
        i += 1;
    }

    if i == NR_JIT_ITERATIONS {
        pr_err("bpf-jit: image did not converge in <%d> passes!\n", i);
        goto_out_free_hdr!(out_free_hdr);
    }
    if extable_size != 0 {
        (*(*prog).aux).extable = ((*ctx).ro_insns as *mut u8).add(prog_size) as *mut _;
    }

skip_init_ctx:
    pass += 1;
    (*ctx).ninsns = 0;
    (*ctx).nexentries = 0;
    bpf_jit_build_prologue(ctx, bpf_is_subprog(prog));
    if build_body(ctx, extra_pass, core::ptr::null_mut()) != 0 {
        goto_out_free_hdr!(out_free_hdr);
    }
    bpf_jit_build_epilogue(ctx);
    if bpf_jit_enable > 1 {
        bpf_jit_dump((*prog).len, prog_size, pass, (*ctx).insns);
    }
    if !(*prog).is_func || extra_pass {
        if WARN_ON(bpf_jit_binary_pack_finalize((*jit_data).ro_header, (*jit_data).header)) {
            (*jit_data).ro_header = core::ptr::null_mut();
            (*jit_data).header = core::ptr::null_mut();
            goto_out_free_hdr!(out_free_hdr);
        }
    }
    (*prog).bpf_func = ((*ctx).ro_insns as *mut u8).add(cfi_get_offset()) as *mut _;
    (*prog).jited = 1;
    (*prog).jited_len = prog_size - cfi_get_offset();
    if !(*prog).is_func || extra_pass {
        i = 0;
        while i < (*prog).len {
            *(*ctx).offset.add(i as usize) = ninsns_rvoff(*(*ctx).offset.add(i as usize));
            i += 1;
        }
        bpf_prog_fill_jited_linfo(prog, (*ctx).offset);
    }
out_offset:
    kvfree((*ctx).offset as *mut _);
    kfree(jit_data as *mut _);
    (*(*prog).aux).jit_data = core::ptr::null_mut();
    return prog;

out_free_hdr:
    if extra_pass {
        (*prog).bpf_func = core::ptr::null_mut();
        (*prog).jited = 0;
        (*prog).jited_len = 0;
    }
    if !(*jit_data).header.is_null() {
        bpf_arch_text_copy(&mut (*(*jit_data).ro_header).size as *mut _, &mut (*(*jit_data).header).size as *mut _, core::mem::size_of_val(&(*(*jit_data).header).size));
        bpf_jit_binary_pack_free((*jit_data).ro_header, (*jit_data).header);
    }
    goto_out_offset!(out_offset);
}

pub unsafe fn bpf_jit_alloc_exec_limit() -> u64 { BPF_JIT_REGION_SIZE }

pub unsafe fn bpf_arch_text_copy(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, len: usize) -> *mut core::ffi::c_void {
    mutex_lock(&mut text_mutex);
    let ret = patch_text_nosync(dst, src, len);
    mutex_unlock(&mut text_mutex);
    if ret != 0 { return ERR_PTR(-EINVAL); }
    dst
}

pub unsafe fn bpf_arch_text_invalidate(dst: *mut core::ffi::c_void, len: usize) -> i32 {
    mutex_lock(&mut text_mutex);
    let ret = patch_text_set_nosync(dst, 0, len);
    mutex_unlock(&mut text_mutex);
    ret
}

pub unsafe fn bpf_jit_free(prog: *mut bpf_prog) {
    if (*prog).jited {
        let jit_data = (*(*prog).aux).jit_data;
        let mut hdr: *mut bpf_binary_header;
        if !jit_data.is_null() {
            bpf_jit_binary_pack_finalize((*jit_data).ro_header, (*jit_data).header);
            kvfree((*jit_data).ctx.offset as *mut _);
            kfree(jit_data as *mut _);
        }
        hdr = bpf_jit_binary_pack_hdr(prog);
        bpf_jit_binary_pack_free(hdr, core::ptr::null_mut());
        WARN_ON_ONCE(!bpf_prog_kallsyms_verify_off(prog));
    }
    bpf_prog_unlock_free(prog);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
