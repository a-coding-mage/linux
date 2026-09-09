// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation unit:
// misc.h, error.h, sev.h, linux/kernel.h, linux/string.h, asm/insn.h,
// asm/pgtable_types.h, asm/ptrace.h, asm/sev.h, asm/trapnr.h, asm/trap_pf.h,
// and asm/fpu/xcr.h.

// __BOOT_COMPRESSED
// __init is intentionally empty in the compressed boot environment.

/* Basic instruction decoding support needed */
// ../../lib/inat.c
// ../../lib/insn.c

/*
 * Copy a version of this function here - insn-eval.c can't be used in
 * pre-decompression code.
 */
pub unsafe fn insn_has_rep_prefix(insn: *mut Insn) -> bool {
    let mut p: InsnByteT;

    insn_get_prefixes(insn);

    for_each_insn_prefix!(insn, p, {
        if p == 0xf2 || p == 0xf3 {
            return true;
        }
    });

    false
}

pub unsafe fn vc_decode_insn(ctxt: *mut EsEmCtxt) -> EsResult {
    let mut buffer = [0i8; MAX_INSN_SIZE];
    let ret: i32;

    memcpy(
        buffer.as_mut_ptr() as *mut u8,
        (*(*ctxt).regs).ip as *const u8,
        MAX_INSN_SIZE,
    );

    ret = insn_decode(
        &mut (*ctxt).insn,
        buffer.as_mut_ptr(),
        MAX_INSN_SIZE,
        INSN_MODE_64,
    );
    if ret < 0 {
        return ES_DECODE_FAILED;
    }

    ES_OK
}

pub unsafe extern "C" fn sev_insn_decode_init(); // __alias(inat_init_tables)

/*
 * Only a dummy for insn_get_seg_base() - Early boot-code is 64bit only and
 * doesn't use segments.
 */
unsafe fn insn_get_seg_base(_regs: *mut PtRegs, _seg_reg_idx: i32) -> c_ulong {
    0UL
}

unsafe fn vc_write_mem(
    _ctxt: *mut EsEmCtxt,
    dst: *mut c_void,
    buf: *mut i8,
    size: usize,
) -> EsResult {
    memcpy(dst, buf as *const c_void, size);

    ES_OK
}

unsafe fn vc_read_mem(
    _ctxt: *mut EsEmCtxt,
    src: *mut c_void,
    buf: *mut i8,
    size: usize,
) -> EsResult {
    memcpy(buf as *mut c_void, src as *const c_void, size);

    ES_OK
}

unsafe fn vc_ioio_check(_ctxt: *mut EsEmCtxt, _port: u16, _size: usize) -> EsResult {
    ES_OK
}

unsafe fn fault_in_kernel_space(_address: c_ulong) -> bool {
    false
}

// #define sev_printk(fmt, ...)

// ../../coco/sev/vc-shared.c

pub unsafe fn do_boot_stage2_vc(regs: *mut PtRegs, exit_code: c_ulong) {
    let mut ctxt: EsEmCtxt = core::mem::zeroed();
    let mut result: EsResult;

    if boot_ghcb.is_null() && !early_setup_ghcb() {
        sev_es_terminate(SEV_TERM_SET_GEN, GHCB_SEV_ES_GEN_REQ);
    }

    vc_ghcb_invalidate(boot_ghcb);
    result = vc_init_em_ctxt(&mut ctxt, regs, exit_code);
    if result != ES_OK {
        goto_finish(result, &mut ctxt);
        return;
    }

    result = vc_check_opcode_bytes(&mut ctxt, exit_code);
    if result != ES_OK {
        goto_finish(result, &mut ctxt);
        return;
    }

    result = match exit_code {
        SVM_EXIT_RDTSC | SVM_EXIT_RDTSCP => {
            vc_handle_rdtsc(boot_ghcb, &mut ctxt, exit_code)
        }
        SVM_EXIT_IOIO => vc_handle_ioio(boot_ghcb, &mut ctxt),
        SVM_EXIT_CPUID => vc_handle_cpuid(boot_ghcb, &mut ctxt),
        _ => ES_UNSUPPORTED,
    };

    goto_finish(result, &mut ctxt);
}

unsafe fn goto_finish(result: EsResult, ctxt: &mut EsEmCtxt) {
    if result == ES_OK {
        vc_finish_insn(ctxt);
    } else if result != ES_RETRY {
        sev_es_terminate(SEV_TERM_SET_GEN, GHCB_SEV_ES_GEN_REQ);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
