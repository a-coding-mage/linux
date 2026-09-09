// SPDX-License-Identifier: GPL-2.0
/* Rust translation of vcpu_insn.c. External kernel symbols are supplied elsewhere. */

#[repr(C)]
struct insn_func {
    mask: c_ulong,
    match_: c_ulong,
    func: Option<unsafe extern "C" fn(*mut kvm_vcpu, *mut kvm_run, c_ulong) -> c_int>,
}

unsafe extern "C" fn truly_illegal_insn(vcpu: *mut kvm_vcpu, _run: *mut kvm_run, insn: c_ulong) -> c_int {
    let mut utrap: kvm_cpu_trap = core::mem::zeroed();
    utrap.sepc = (*vcpu).arch.guest_context.sepc;
    utrap.scause = EXC_INST_ILLEGAL;
    utrap.stval = insn;
    utrap.htval = 0;
    utrap.htinst = 0;
    kvm_riscv_vcpu_trap_redirect(vcpu, &mut utrap);
    1
}

unsafe extern "C" fn truly_virtual_insn(vcpu: *mut kvm_vcpu, _run: *mut kvm_run, insn: c_ulong) -> c_int {
    let mut utrap: kvm_cpu_trap = core::mem::zeroed();
    utrap.sepc = (*vcpu).arch.guest_context.sepc;
    utrap.scause = EXC_VIRTUAL_INST_FAULT;
    utrap.stval = insn;
    utrap.htval = 0;
    utrap.htinst = 0;
    kvm_riscv_vcpu_trap_redirect(vcpu, &mut utrap);
    1
}

pub unsafe extern "C" fn kvm_riscv_vcpu_wfi(vcpu: *mut kvm_vcpu) {
    if !kvm_arch_vcpu_runnable(vcpu) {
        kvm_vcpu_srcu_read_unlock(vcpu);
        kvm_vcpu_halt(vcpu);
        kvm_vcpu_srcu_read_lock(vcpu);
    }
}

unsafe extern "C" fn wfi_insn(vcpu: *mut kvm_vcpu, _run: *mut kvm_run, _insn: c_ulong) -> c_int {
    (*vcpu).stat.wfi_exit_stat += 1;
    kvm_riscv_vcpu_wfi(vcpu);
    KVM_INSN_CONTINUE_NEXT_SEPC
}

unsafe extern "C" fn wrs_insn(vcpu: *mut kvm_vcpu, _run: *mut kvm_run, _insn: c_ulong) -> c_int {
    (*vcpu).stat.wrs_exit_stat += 1;
    kvm_vcpu_on_spin(vcpu, (*vcpu).arch.guest_context.sstatus & SR_SPP);
    KVM_INSN_CONTINUE_NEXT_SEPC
}

#[repr(C)]
struct csr_func {
    base: c_uint,
    count: c_uint,
    func: Option<unsafe extern "C" fn(*mut kvm_vcpu, c_uint, *mut c_ulong, c_ulong, c_ulong) -> c_int>,
}

unsafe extern "C" fn seed_csr_rmw(vcpu: *mut kvm_vcpu, _csr_num: c_uint, _val: *mut c_ulong, _new_val: c_ulong, _wr_mask: c_ulong) -> c_int {
    if !riscv_isa_extension_available((*vcpu).arch.isa, ZKR) {
        return KVM_INSN_ILLEGAL_TRAP;
    }
    KVM_INSN_EXIT_TO_USER_SPACE
}

// KVM_RISCV_VCPU_AIA_CSR_FUNCS and KVM_RISCV_VCPU_HPMCOUNTER_CSR_FUNCS are
// build-time declarations supplied by the surrounding kernel translation.
static csr_funcs: &[csr_func] = &[
    csr_func { base: CSR_SEED, count: 1, func: Some(seed_csr_rmw) },
];

pub unsafe extern "C" fn kvm_riscv_vcpu_csr_return(vcpu: *mut kvm_vcpu, run: *mut kvm_run) -> c_int {
    if (*vcpu).arch.csr_decode.return_handled != 0 { return 0; }
    (*vcpu).arch.csr_decode.return_handled = 1;
    let mut insn = (*vcpu).arch.csr_decode.insn;
    if ((insn >> SH_RD) & MASK_RX) != 0 {
        SET_RD(insn, &mut (*vcpu).arch.guest_context, (*run).riscv_csr.ret_value);
    }
    (*vcpu).arch.guest_context.sepc += INSN_LEN(insn);
    0
}

unsafe extern "C" fn csr_insn(vcpu: *mut kvm_vcpu, run: *mut kvm_run, insn: c_ulong) -> c_int {
    let mut rc = KVM_INSN_ILLEGAL_TRAP;
    let csr_num = (insn >> SH_RS2) as c_uint;
    let rs1_num = ((insn >> SH_RS1) & MASK_RX) as c_uint;
    let rs1_val = GET_RS1(insn, &mut (*vcpu).arch.guest_context);
    let (wr_mask, new_val) = match GET_FUNCT3(insn) {
        x if x == GET_FUNCT3(INSN_MATCH_CSRRW) => (!0, rs1_val),
        x if x == GET_FUNCT3(INSN_MATCH_CSRRS) => (rs1_val, !0),
        x if x == GET_FUNCT3(INSN_MATCH_CSRRC) => (rs1_val, 0),
        x if x == GET_FUNCT3(INSN_MATCH_CSRRWI) => (!0, rs1_num as c_ulong),
        x if x == GET_FUNCT3(INSN_MATCH_CSRRSI) => (rs1_num as c_ulong, !0),
        x if x == GET_FUNCT3(INSN_MATCH_CSRRCI) => (rs1_num as c_ulong, 0),
        _ => return rc,
    };
    (*vcpu).arch.csr_decode.insn = insn;
    (*vcpu).arch.csr_decode.return_handled = 0;
    (*run).riscv_csr.csr_num = csr_num;
    (*run).riscv_csr.new_value = new_val;
    (*run).riscv_csr.write_mask = wr_mask;
    (*run).riscv_csr.ret_value = 0;
    let mut val = 0;
    let mut found: Option<&csr_func> = None;
    for f in csr_funcs { if f.base <= csr_num && csr_num < f.base + f.count { found = Some(f); break; } }
    if let Some(f) = found { if let Some(func) = f.func {
        rc = func(vcpu, csr_num, &mut val, new_val, wr_mask);
        if rc > KVM_INSN_EXIT_TO_USER_SPACE {
            if rc == KVM_INSN_CONTINUE_NEXT_SEPC {
                (*run).riscv_csr.ret_value = val;
                (*vcpu).stat.csr_exit_kernel += 1;
                kvm_riscv_vcpu_csr_return(vcpu, run);
                rc = KVM_INSN_CONTINUE_SAME_SEPC;
            }
            return rc;
        }
    }}
    if rc <= KVM_INSN_EXIT_TO_USER_SPACE {
        (*vcpu).stat.csr_exit_user += 1;
        (*run).exit_reason = KVM_EXIT_RISCV_CSR;
    }
    rc
}

static system_opcode_funcs: &[insn_func] = &[
    insn_func { mask: INSN_MASK_CSRRW, match_: INSN_MATCH_CSRRW, func: Some(csr_insn) },
    insn_func { mask: INSN_MASK_CSRRS, match_: INSN_MATCH_CSRRS, func: Some(csr_insn) },
    insn_func { mask: INSN_MASK_CSRRC, match_: INSN_MATCH_CSRRC, func: Some(csr_insn) },
    insn_func { mask: INSN_MASK_CSRRWI, match_: INSN_MATCH_CSRRWI, func: Some(csr_insn) },
    insn_func { mask: INSN_MASK_CSRRSI, match_: INSN_MATCH_CSRRSI, func: Some(csr_insn) },
    insn_func { mask: INSN_MASK_CSRRCI, match_: INSN_MATCH_CSRRCI, func: Some(csr_insn) },
    insn_func { mask: INSN_MASK_WFI, match_: INSN_MATCH_WFI, func: Some(wfi_insn) },
    insn_func { mask: INSN_MASK_WRS, match_: INSN_MATCH_WRS, func: Some(wrs_insn) },
];

unsafe extern "C" fn system_opcode_insn(vcpu: *mut kvm_vcpu, run: *mut kvm_run, insn: c_ulong) -> c_int {
    let mut rc = KVM_INSN_ILLEGAL_TRAP;
    for f in system_opcode_funcs { if (insn & f.mask) == f.match_ { rc = f.func.unwrap()(vcpu, run, insn); break; } }
    let result = match rc {
        KVM_INSN_ILLEGAL_TRAP => truly_illegal_insn(vcpu, run, insn),
        KVM_INSN_VIRTUAL_TRAP => truly_virtual_insn(vcpu, run, insn),
        KVM_INSN_CONTINUE_NEXT_SEPC => { (*vcpu).arch.guest_context.sepc += INSN_LEN(insn); rc },
        _ => rc,
    };
    if result <= 0 { result } else { 1 }
}

unsafe fn is_load_guest_page_fault(scause: c_ulong) -> bool { scause == EXC_LOAD_GUEST_PAGE_FAULT }

pub unsafe extern "C" fn kvm_riscv_vcpu_virtual_insn(vcpu: *mut kvm_vcpu, run: *mut kvm_run, trap: *mut kvm_cpu_trap) -> c_int {
    let mut insn = (*trap).stval;
    let mut utrap: kvm_cpu_trap = core::mem::zeroed();
    let ct = &mut (*vcpu).arch.guest_context;
    if INSN_IS_16BIT(insn) {
        if insn == 0 {
            insn = kvm_riscv_vcpu_unpriv_read(vcpu, true, ct.sepc, &mut utrap);
            if utrap.scause != 0 {
                if is_load_guest_page_fault(utrap.scause) { return 1; }
                utrap.sepc = ct.sepc;
                kvm_riscv_vcpu_trap_redirect(vcpu, &mut utrap);
                return 1;
            }
        }
        if INSN_IS_16BIT(insn) { return truly_illegal_insn(vcpu, run, insn); }
    }
    match (insn & INSN_OPCODE_MASK) >> INSN_OPCODE_SHIFT {
        INSN_OPCODE_SYSTEM => system_opcode_insn(vcpu, run, insn),
        _ => truly_illegal_insn(vcpu, run, insn),
    }
}

pub unsafe extern "C" fn kvm_riscv_vcpu_mmio_load(vcpu: *mut kvm_vcpu, run: *mut kvm_run, fault_addr: gpa_t, htinst: c_ulong) -> c_int {
    let mut data_buf = [0u8; 8];
    let (mut insn, raw_insn, insn_len);
    let mut shift = 0;
    let len;
    let mut utrap: kvm_cpu_trap = core::mem::zeroed();
    let ct = &mut (*vcpu).arch.guest_context;
    if htinst & 1 != 0 { insn = htinst | INSN_16BIT_MASK; insn_len = if htinst & BIT(1) != 0 { INSN_LEN(insn) } else { 2 }; }
    else { insn = kvm_riscv_vcpu_unpriv_read(vcpu, true, ct.sepc, &mut utrap); if utrap.scause != 0 { if is_load_guest_page_fault(utrap.scause) { return 1; } utrap.sepc = ct.sepc; kvm_riscv_vcpu_trap_redirect(vcpu, &mut utrap); return 1; } insn_len = INSN_LEN(insn); }
    raw_insn = insn;
    if insn & INSN_MASK_LW == INSN_MATCH_LW { len=4; shift=8*(core::mem::size_of::<c_ulong>()-len); }
    else if insn & INSN_MASK_LB == INSN_MATCH_LB { len=1; shift=8*(core::mem::size_of::<c_ulong>()-len); }
    else if insn & INSN_MASK_LBU == INSN_MATCH_LBU { len=1; }
    else if insn & INSN_MASK_LH == INSN_MATCH_LH { len=2; shift=8*(core::mem::size_of::<c_ulong>()-len); }
    else if insn & INSN_MASK_LHU == INSN_MATCH_LHU { len=2; }
    else if insn & INSN_MASK_C_LW == INSN_MATCH_C_LW { len=4; shift=8*(core::mem::size_of::<c_ulong>()-len); insn=RVC_RS2S(insn)<<SH_RD; }
    else if insn & INSN_MASK_C_LWSP == INSN_MATCH_C_LWSP && ((insn>>SH_RD)&0x1f)!=0 { len=4; shift=8*(core::mem::size_of::<c_ulong>()-len); }
    else { return -EOPNOTSUPP; }
    if fault_addr & (len-1) != 0 { return -EIO; }
    trace_kvm_mmio_emulate((*vcpu).vcpu_id, ct.sepc, raw_insn, fault_addr, false, len);
    (*vcpu).arch.mmio_decode.insn=insn; (*vcpu).arch.mmio_decode.insn_len=insn_len; (*vcpu).arch.mmio_decode.shift=shift; (*vcpu).arch.mmio_decode.len=len; (*vcpu).arch.mmio_decode.return_handled=0;
    (*run).mmio.is_write=false; (*run).mmio.phys_addr=fault_addr; (*run).mmio.len=len;
    if kvm_io_bus_read(vcpu,KVM_MMIO_BUS,fault_addr,len,data_buf.as_mut_ptr()) == 0 { core::ptr::copy_nonoverlapping(data_buf.as_ptr(),(*run).mmio.data.as_mut_ptr(),len); (*vcpu).stat.mmio_exit_kernel+=1; kvm_riscv_vcpu_mmio_return(vcpu,run); return 1; }
    (*vcpu).stat.mmio_exit_user+=1; (*run).exit_reason=KVM_EXIT_MMIO; 0
}

pub unsafe extern "C" fn kvm_riscv_vcpu_mmio_store(vcpu: *mut kvm_vcpu, run: *mut kvm_run, fault_addr: gpa_t, htinst: c_ulong) -> c_int {
    let (mut insn, raw_insn, insn_len); let len; let mut utrap: kvm_cpu_trap=core::mem::zeroed(); let ct=&mut (*vcpu).arch.guest_context;
    if htinst&1!=0 { insn=htinst|INSN_16BIT_MASK; insn_len=if htinst&BIT(1)!=0 { INSN_LEN(insn) } else { 2 }; }
    else { insn=kvm_riscv_vcpu_unpriv_read(vcpu,true,ct.sepc,&mut utrap); if utrap.scause!=0 { if is_load_guest_page_fault(utrap.scause){return 1;} utrap.sepc=ct.sepc; kvm_riscv_vcpu_trap_redirect(vcpu,&mut utrap); return 1;} insn_len=INSN_LEN(insn); }
    raw_insn=insn; let data=GET_RS2(insn,ct); let mut data_buf=[0u8;8];
    if insn&INSN_MASK_SW==INSN_MATCH_SW {len=4;} else if insn&INSN_MASK_SB==INSN_MATCH_SB {len=1;} else if insn&INSN_MASK_SH==INSN_MATCH_SH {len=2;} else if insn&INSN_MASK_C_SW==INSN_MATCH_C_SW {len=4;} else if insn&INSN_MASK_C_SWSP==INSN_MATCH_C_SWSP && ((insn>>SH_RD)&0x1f)!=0 {len=4;} else{return -EOPNOTSUPP;}
    if fault_addr&(len-1)!=0{return -EIO;} trace_kvm_mmio_emulate((*vcpu).vcpu_id,ct.sepc,raw_insn,fault_addr,true,len);
    (*vcpu).arch.mmio_decode.insn=insn;(*vcpu).arch.mmio_decode.insn_len=insn_len;(*vcpu).arch.mmio_decode.shift=0;(*vcpu).arch.mmio_decode.len=len;(*vcpu).arch.mmio_decode.return_handled=0;
    core::ptr::copy_nonoverlapping((&data as *const _ as *const u8),data_buf.as_mut_ptr(),len); core::ptr::copy_nonoverlapping(data_buf.as_ptr(),(*run).mmio.data.as_mut_ptr(),len);
    (*run).mmio.is_write=true;(*run).mmio.phys_addr=fault_addr;(*run).mmio.len=len;
    if kvm_io_bus_write(vcpu,KVM_MMIO_BUS,fault_addr,len,(*run).mmio.data.as_ptr())==0 {(*vcpu).stat.mmio_exit_kernel+=1;kvm_riscv_vcpu_mmio_return(vcpu,run);return 1;} (*vcpu).stat.mmio_exit_user+=1;(*run).exit_reason=KVM_EXIT_MMIO;0
}

pub unsafe extern "C" fn kvm_riscv_vcpu_mmio_return(vcpu:*mut kvm_vcpu,run:*mut kvm_run)->c_int { if (*vcpu).arch.mmio_decode.return_handled!=0{return 0;} (*vcpu).arch.mmio_decode.return_handled=1; let insn=(*vcpu).arch.mmio_decode.insn; if !(*run).mmio.is_write { let len=(*vcpu).arch.mmio_decode.len; let shift=(*vcpu).arch.mmio_decode.shift; let mut v=0u64; core::ptr::copy_nonoverlapping((*run).mmio.data.as_ptr(),(&mut v as *mut _ as *mut u8),len); SET_RD(insn,&mut (*vcpu).arch.guest_context,((v<<shift) as c_long>>shift) as c_ulong); } (*vcpu).arch.guest_context.sepc+=(*vcpu).arch.mmio_decode.insn_len; 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
