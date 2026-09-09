// SPDX-License-Identifier: GPL-2.0

// `has_cpuflag` is provided by the non-compressed build configuration.

unsafe fn vc_check_opcode_bytes(ctxt: *mut es_em_ctxt, exit_code: c_ulong) -> es_result {
    let opcode = (*ctxt).insn.opcode.value as u32;
    let modrm = (*ctxt).insn.modrm.value;

    match exit_code {
        SVM_EXIT_IOIO | SVM_EXIT_NPF => ES_OK,
        SVM_EXIT_CPUID => if opcode == 0xa20f { ES_OK } else { vc_wrong_opcode(ctxt, opcode, exit_code) },
        SVM_EXIT_INVD => if opcode == 0x080f { ES_OK } else { vc_wrong_opcode(ctxt, opcode, exit_code) },
        SVM_EXIT_MONITOR => if opcode == 0x010f && (modrm == 0xc8 || modrm == 0xfa) { ES_OK } else { vc_wrong_opcode(ctxt, opcode, exit_code) },
        SVM_EXIT_MWAIT => if opcode == 0x010f && (modrm == 0xc9 || modrm == 0xfb) { ES_OK } else { vc_wrong_opcode(ctxt, opcode, exit_code) },
        SVM_EXIT_MSR => if opcode == 0x320f || opcode == 0x300f { ES_OK } else { vc_wrong_opcode(ctxt, opcode, exit_code) },
        SVM_EXIT_RDPMC => if opcode == 0x330f { ES_OK } else { vc_wrong_opcode(ctxt, opcode, exit_code) },
        SVM_EXIT_RDTSC => if opcode == 0x310f { ES_OK } else { vc_wrong_opcode(ctxt, opcode, exit_code) },
        SVM_EXIT_RDTSCP => if opcode == 0x010f && modrm == 0xf9 { ES_OK } else { vc_wrong_opcode(ctxt, opcode, exit_code) },
        SVM_EXIT_READ_DR7 => if opcode == 0x210f && X86_MODRM_REG((*ctxt).insn.modrm.value) == 7 { ES_OK } else { vc_wrong_opcode(ctxt, opcode, exit_code) },
        SVM_EXIT_VMMCALL => if opcode == 0x010f && modrm == 0xd9 { ES_OK } else { vc_wrong_opcode(ctxt, opcode, exit_code) },
        SVM_EXIT_WRITE_DR7 => if opcode == 0x230f && X86_MODRM_REG((*ctxt).insn.modrm.value) == 7 { ES_OK } else { vc_wrong_opcode(ctxt, opcode, exit_code) },
        SVM_EXIT_WBINVD => if opcode == 0x90f { ES_OK } else { vc_wrong_opcode(ctxt, opcode, exit_code) },
        _ => vc_wrong_opcode(ctxt, opcode, exit_code),
    }
}

unsafe fn vc_wrong_opcode(ctxt: *mut es_em_ctxt, opcode: u32, exit_code: c_ulong) -> es_result {
    sev_printk(KERN_ERR, "Wrong/unhandled opcode bytes: 0x%x, exit_code: 0x%lx, rIP: 0x%lx\n", opcode, exit_code, (*(*ctxt).regs).ip);
    ES_UNSUPPORTED
}

unsafe fn vc_decoding_needed(exit_code: c_ulong) -> bool {
    !(exit_code >= SVM_EXIT_EXCP_BASE && exit_code <= SVM_EXIT_LAST_EXCP)
}

unsafe fn vc_init_em_ctxt(ctxt: *mut es_em_ctxt, regs: *mut pt_regs, exit_code: c_ulong) -> es_result {
    let mut ret = ES_OK;
    memset(ctxt as *mut c_void, 0, core::mem::size_of::<es_em_ctxt>());
    (*ctxt).regs = regs;
    if vc_decoding_needed(exit_code) { ret = vc_decode_insn(ctxt); }
    ret
}

unsafe fn vc_finish_insn(ctxt: *mut es_em_ctxt) { (*(*ctxt).regs).ip += (*ctxt).insn.length as u64; }

unsafe fn vc_insn_string_check(ctxt: *mut es_em_ctxt, address: c_ulong, write: bool) -> es_result {
    if user_mode((*ctxt).regs) && fault_in_kernel_space(address) {
        (*ctxt).fi.vector = X86_TRAP_PF;
        (*ctxt).fi.error_code = X86_PF_USER;
        (*ctxt).fi.cr2 = address;
        if write { (*ctxt).fi.error_code |= X86_PF_WRITE; }
        return ES_EXCEPTION;
    }
    ES_OK
}

unsafe fn vc_insn_string_read(ctxt: *mut es_em_ctxt, src: *mut c_void, buf: *mut c_char, data_size: c_uint, count: c_uint, backwards: bool) -> es_result {
    let step: isize = if backwards { -1 } else { 1 };
    let mut ret = vc_insn_string_check(ctxt, src as c_ulong, false);
    if ret != ES_OK { return ret; }
    for i in 0..count { ret = vc_read_mem(ctxt, src.offset((i * data_size) as isize * step), buf.offset((i * data_size) as isize), data_size); if ret != ES_OK { break; } }
    ret
}

unsafe fn vc_insn_string_write(ctxt: *mut es_em_ctxt, dst: *mut c_void, buf: *mut c_char, data_size: c_uint, count: c_uint, backwards: bool) -> es_result {
    let step: isize = if backwards { -1 } else { 1 };
    let mut ret = vc_insn_string_check(ctxt, dst as c_ulong, true);
    if ret != ES_OK { return ret; }
    for i in 0..count { ret = vc_write_mem(ctxt, dst.offset((i * data_size) as isize * step), buf.offset((i * data_size) as isize), data_size); if ret != ES_OK { break; } }
    ret
}

const IOIO_TYPE_STR: u64 = 1 << 2;
const IOIO_TYPE_IN: u64 = 1;
const IOIO_TYPE_INS: u64 = IOIO_TYPE_IN | IOIO_TYPE_STR;
const IOIO_TYPE_OUT: u64 = 0;
const IOIO_TYPE_OUTS: u64 = IOIO_TYPE_OUT | IOIO_TYPE_STR;
const IOIO_REP: u64 = 1 << 3;
const IOIO_ADDR_64: u64 = 1 << 9;
const IOIO_ADDR_32: u64 = 1 << 8;
const IOIO_ADDR_16: u64 = 1 << 7;
const IOIO_DATA_32: u64 = 1 << 6;
const IOIO_DATA_16: u64 = 1 << 5;
const IOIO_DATA_8: u64 = 1 << 4;
const IOIO_SEG_ES: u64 = 0 << 10;
const IOIO_SEG_DS: u64 = 3 << 10;

unsafe fn vc_ioio_exitinfo(ctxt: *mut es_em_ctxt, exitinfo: *mut u64) -> es_result {
    let insn = &(*ctxt).insn;
    let op = insn.opcode.bytes[0];
    *exitinfo = 0;
    let port: u64;
    match op {
        0x6c | 0x6d => { *exitinfo |= IOIO_TYPE_INS | IOIO_SEG_ES; port = (*(*ctxt).regs).dx & 0xffff; }
        0x6e | 0x6f => { *exitinfo |= IOIO_TYPE_OUTS | IOIO_SEG_DS; port = (*(*ctxt).regs).dx & 0xffff; }
        0xe4 | 0xe5 | 0xe6 | 0xe7 => { if op <= 0xe5 { *exitinfo |= IOIO_TYPE_IN; } port = (insn.immediate.value as u8) as u64; }
        0xec | 0xed => { *exitinfo |= IOIO_TYPE_IN; port = (*(*ctxt).regs).dx & 0xffff; }
        0xee | 0xef => { port = (*(*ctxt).regs).dx & 0xffff; }
        _ => return ES_DECODE_FAILED,
    }
    *exitinfo |= port << 16;
    match op { 0x6c | 0x6e | 0xe4 | 0xe6 | 0xec | 0xee => *exitinfo |= IOIO_DATA_8, _ => *exitinfo |= if insn.opnd_bytes == 2 { IOIO_DATA_16 } else { IOIO_DATA_32 } }
    match insn.addr_bytes { 2 => *exitinfo |= IOIO_ADDR_16, 4 => *exitinfo |= IOIO_ADDR_32, 8 => *exitinfo |= IOIO_ADDR_64, _ => {} }
    if insn_has_rep_prefix(insn) { *exitinfo |= IOIO_REP; }
    vc_ioio_check(ctxt, port as u16, if matches!(op, 0x6c | 0x6e | 0xe4 | 0xe6 | 0xec | 0xee) { 1 } else if insn.opnd_bytes == 2 { 2 } else { 4 })
}

unsafe fn verify_exception_info(ghcb: *mut ghcb, ctxt: *mut es_em_ctxt) -> es_result {
    let ret = (*ghcb).save.sw_exit_info_1 & GENMASK_ULL(31, 0);
    if ret == 0 { return ES_OK; }
    if ret == 1 {
        let info = (*ghcb).save.sw_exit_info_2;
        let v = info & SVM_EVTINJ_VEC_MASK;
        if info & SVM_EVTINJ_VALID != 0 && (v == X86_TRAP_GP || v == X86_TRAP_UD) && info & SVM_EVTINJ_TYPE_MASK == SVM_EVTINJ_TYPE_EXEPT {
            (*ctxt).fi.vector = v;
            if info & SVM_EVTINJ_VALID_ERR != 0 { (*ctxt).fi.error_code = info >> 32; }
            return ES_EXCEPTION;
        }
    }
    ES_VMM_ERROR
}

unsafe fn sev_es_ghcb_hv_call(ghcb: *mut ghcb, ctxt: *mut es_em_ctxt, exit_code: u64, exit_info_1: u64, exit_info_2: u64) -> es_result {
    (*ghcb).protocol_version = ghcb_version;
    (*ghcb).ghcb_usage = GHCB_DEFAULT_USAGE;
    ghcb_set_sw_exit_code(ghcb, exit_code); ghcb_set_sw_exit_info_1(ghcb, exit_info_1); ghcb_set_sw_exit_info_2(ghcb, exit_info_2);
    sev_es_wr_ghcb_msr(__pa(ghcb)); VMGEXIT(); verify_exception_info(ghcb, ctxt)
}

unsafe fn vc_handle_ioio(_ghcb: *mut ghcb, _ctxt: *mut es_em_ctxt) -> es_result { ES_UNSUPPORTED }
unsafe fn vc_handle_cpuid(_ghcb: *mut ghcb, _ctxt: *mut es_em_ctxt) -> es_result { ES_UNSUPPORTED }
unsafe fn vc_handle_rdtsc(_ghcb: *mut ghcb, _ctxt: *mut es_em_ctxt, _exit_code: c_ulong) -> es_result { ES_UNSUPPORTED }
unsafe fn snp_register_ghcb_early(_paddr: c_ulong) { }
unsafe fn sev_es_check_cpu_features() -> bool { has_cpuflag(X86_FEATURE_RDRAND) }
unsafe fn sev_es_negotiate_protocol() -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
