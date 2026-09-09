// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright IBM Corp. 2007
 * Copyright 2011 Freescale Semiconductor, Inc.
 *
 * Authors: Hollis Blanchard <hollisb@us.ibm.com>
 */

// Linux kernel and PowerPC dependencies are supplied externally.

#[cfg(CONFIG_PPC_FPU)]
unsafe fn kvmppc_check_fp_disabled(vcpu: *mut kvm_vcpu) -> bool {
    if kvmppc_get_msr(vcpu) & MSR_FP == 0 {
        kvmppc_core_queue_fpunavail(vcpu, kvmppc_get_msr(vcpu) & SRR1_PREFIXED);
        return true;
    }
    false
}

#[cfg(CONFIG_VSX)]
unsafe fn kvmppc_check_vsx_disabled(vcpu: *mut kvm_vcpu) -> bool {
    if kvmppc_get_msr(vcpu) & MSR_VSX == 0 {
        kvmppc_core_queue_vsx_unavail(vcpu, kvmppc_get_msr(vcpu) & SRR1_PREFIXED);
        return true;
    }
    false
}

#[cfg(CONFIG_ALTIVEC)]
unsafe fn kvmppc_check_altivec_disabled(vcpu: *mut kvm_vcpu) -> bool {
    if kvmppc_get_msr(vcpu) & MSR_VEC == 0 {
        kvmppc_core_queue_vec_unavail(vcpu, kvmppc_get_msr(vcpu) & SRR1_PREFIXED);
        return true;
    }
    false
}

/*
 * XXX to do:
 * lfiwax, lfiwzx
 * vector loads and stores
 *
 * Instructions that trap when used on cache-inhibited mappings
 * are not emulated here: multiple and string instructions,
 * lq/stq, and the load-reserve/store-conditional instructions.
 */
unsafe fn kvmppc_emulate_loadstore(vcpu: *mut kvm_vcpu) -> emulation_result {
    let mut inst: ppc_inst_t = core::mem::zeroed();
    let mut emulated = EMULATE_FAIL;
    let mut op: instruction_op = core::mem::zeroed();

    kvmppc_set_exit_type(vcpu, EMULATED_INST_EXITS);
    emulated = kvmppc_get_last_inst(vcpu, INST_GENERIC, &mut inst);
    if emulated != EMULATE_DONE { return emulated; }

    (*vcpu).arch.mmio_vsx_copy_nums = 0;
    (*vcpu).arch.mmio_vsx_offset = 0;
    (*vcpu).arch.mmio_copy_type = KVMPPC_VSX_COPY_NONE;
    (*vcpu).arch.mmio_sp64_extend = 0;
    (*vcpu).arch.mmio_sign_extend = 0;
    (*vcpu).arch.mmio_vmx_copy_nums = 0;
    (*vcpu).arch.mmio_vmx_offset = 0;
    (*vcpu).arch.mmio_host_swabbed = 0;

    emulated = EMULATE_FAIL;
    (*vcpu).arch.regs.msr = kvmppc_get_msr(vcpu);
    if analyse_instr(&mut op, &mut (*vcpu).arch.regs, inst) == 0 {
        let typ = op.type_ & INSTR_TYPE_MASK;
        let size = GETSIZE(op.type_);
        (*vcpu).mmio_is_write = OP_IS_STORE(typ);

        match typ {
            LOAD => {
                let instr_byte_swap = op.type_ & BYTEREV;
                emulated = if op.type_ & SIGNEXT != 0 {
                    kvmppc_handle_loads(vcpu, op.reg, size, instr_byte_swap == 0)
                } else {
                    kvmppc_handle_load(vcpu, op.reg, size, instr_byte_swap == 0)
                };
                if op.type_ & UPDATE != 0 && emulated != EMULATE_FAIL {
                    kvmppc_set_gpr(vcpu, op.update_reg, (*vcpu).arch.vaddr_accessed);
                }
            }
            #[cfg(CONFIG_PPC_FPU)] LOAD_FP => {
                if kvmppc_check_fp_disabled(vcpu) { return EMULATE_DONE; }
                if op.type_ & FPCONV != 0 { (*vcpu).arch.mmio_sp64_extend = 1; }
                emulated = if op.type_ & SIGNEXT != 0 { kvmppc_handle_loads(vcpu, KVM_MMIO_REG_FPR | op.reg, size, true) } else { kvmppc_handle_load(vcpu, KVM_MMIO_REG_FPR | op.reg, size, true) };
                if op.type_ & UPDATE != 0 && emulated != EMULATE_FAIL { kvmppc_set_gpr(vcpu, op.update_reg, (*vcpu).arch.vaddr_accessed); }
            }
            #[cfg(CONFIG_ALTIVEC)] LOAD_VMX => {
                if kvmppc_check_altivec_disabled(vcpu) { return EMULATE_DONE; }
                (*vcpu).arch.vaddr_accessed &= !((size as usize) - 1);
                (*vcpu).arch.paddr_accessed &= !((size as usize) - 1);
                (*vcpu).arch.mmio_copy_type = match size { 16 => KVMPPC_VMX_COPY_DWORD, 4 => KVMPPC_VMX_COPY_WORD, 2 => KVMPPC_VMX_COPY_HWORD, 1 => KVMPPC_VMX_COPY_BYTE, _ => return emulated };
                (*vcpu).arch.mmio_vmx_offset = ((*vcpu).arch.vaddr_accessed & 0xf) / size;
                if size == 16 { (*vcpu).arch.mmio_vmx_copy_nums = 2; emulated = kvmppc_handle_vmx_load(vcpu, KVM_MMIO_REG_VMX | op.reg, 8, 1); } else { (*vcpu).arch.mmio_vmx_copy_nums = 1; emulated = kvmppc_handle_vmx_load(vcpu, KVM_MMIO_REG_VMX | op.reg, size, 1); }
            }
            #[cfg(CONFIG_VSX)] LOAD_VSX => {
                if op.vsx_flags & VSX_CHECK_VEC != 0 { if kvmppc_check_altivec_disabled(vcpu) { return EMULATE_DONE; } } else if kvmppc_check_vsx_disabled(vcpu) { return EMULATE_DONE; }
                if op.vsx_flags & VSX_FPCONV != 0 { (*vcpu).arch.mmio_sp64_extend = 1; }
                (*vcpu).arch.mmio_copy_type = if op.element_size == 8 { if op.vsx_flags & VSX_SPLAT != 0 { KVMPPC_VSX_COPY_DWORD_LOAD_DUMP } else { KVMPPC_VSX_COPY_DWORD } } else if op.element_size == 4 { if op.vsx_flags & VSX_SPLAT != 0 { KVMPPC_VSX_COPY_WORD_LOAD_DUMP } else { KVMPPC_VSX_COPY_WORD } } else { return emulated };
                let io_size_each;
                if size < op.element_size { (*vcpu).arch.mmio_vsx_copy_nums = 1; io_size_each = size; } else { (*vcpu).arch.mmio_vsx_copy_nums = size / op.element_size; io_size_each = op.element_size; }
                emulated = kvmppc_handle_vsx_load(vcpu, KVM_MMIO_REG_VSX | op.reg, io_size_each, 1, op.type_ & SIGNEXT);
            }
            #[cfg(CONFIG_ALTIVEC)] STORE_VMX => {
                if kvmppc_check_altivec_disabled(vcpu) { return EMULATE_DONE; }
                (*vcpu).arch.vaddr_accessed &= !((size as usize) - 1); (*vcpu).arch.paddr_accessed &= !((size as usize) - 1);
                if let Some(giveup_ext) = (*(*vcpu).kvm).arch.kvm_ops.giveup_ext { giveup_ext(vcpu, MSR_VEC); }
                (*vcpu).arch.mmio_copy_type = match size { 16 => KVMPPC_VMX_COPY_DWORD, 4 => KVMPPC_VMX_COPY_WORD, 2 => KVMPPC_VMX_COPY_HWORD, 1 => KVMPPC_VMX_COPY_BYTE, _ => return emulated };
                (*vcpu).arch.mmio_vmx_offset = ((*vcpu).arch.vaddr_accessed & 0xf) / size;
                if size == 16 { (*vcpu).arch.mmio_vmx_copy_nums = 2; emulated = kvmppc_handle_vmx_store(vcpu, op.reg, 8, 1); } else { (*vcpu).arch.mmio_vmx_copy_nums = 1; emulated = kvmppc_handle_vmx_store(vcpu, op.reg, size, 1); }
            }
            #[cfg(CONFIG_VSX)] STORE_VSX => {
                if op.vsx_flags & VSX_CHECK_VEC != 0 { if kvmppc_check_altivec_disabled(vcpu) { return EMULATE_DONE; } } else if kvmppc_check_vsx_disabled(vcpu) { return EMULATE_DONE; }
                if let Some(giveup_ext) = (*(*vcpu).kvm).arch.kvm_ops.giveup_ext { giveup_ext(vcpu, MSR_VSX); }
                if op.vsx_flags & VSX_FPCONV != 0 { (*vcpu).arch.mmio_sp64_extend = 1; }
                (*vcpu).arch.mmio_copy_type = match op.element_size { 8 => KVMPPC_VSX_COPY_DWORD, 4 => KVMPPC_VSX_COPY_WORD, _ => return emulated };
                let io_size_each;
                if size < op.element_size { (*vcpu).arch.mmio_vsx_copy_nums = 1; io_size_each = size; } else { (*vcpu).arch.mmio_vsx_copy_nums = size / op.element_size; io_size_each = op.element_size; }
                emulated = kvmppc_handle_vsx_store(vcpu, op.reg, io_size_each, 1);
            }
            #[cfg(CONFIG_PPC_FPU)] STORE_FP => {
                if kvmppc_check_fp_disabled(vcpu) { return EMULATE_DONE; }
                if let Some(giveup_ext) = (*(*vcpu).kvm).arch.kvm_ops.giveup_ext { giveup_ext(vcpu, MSR_FP); }
                if op.type_ & FPCONV != 0 { (*vcpu).arch.mmio_sp64_extend = 1; }
                emulated = kvmppc_handle_store(vcpu, kvmppc_get_fpr(vcpu, op.reg), size, 1);
                if op.type_ & UPDATE != 0 && emulated != EMULATE_FAIL { kvmppc_set_gpr(vcpu, op.update_reg, (*vcpu).arch.vaddr_accessed); }
            }
            STORE => {
                let instr_byte_swap = op.type_ & BYTEREV;
                emulated = kvmppc_handle_store(vcpu, kvmppc_get_gpr(vcpu, op.reg), size, instr_byte_swap == 0);
                if op.type_ & UPDATE != 0 && emulated != EMULATE_FAIL { kvmppc_set_gpr(vcpu, op.update_reg, (*vcpu).arch.vaddr_accessed); }
            }
            CACHEOP => { emulated = EMULATE_DONE; }
            _ => {}
        }
    }
    trace_kvm_ppc_instr(ppc_inst_val(inst), kvmppc_get_pc(vcpu), emulated);
    if emulated != EMULATE_FAIL { kvmppc_set_pc(vcpu, kvmppc_get_pc(vcpu) + ppc_inst_len(inst)); }
    emulated
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
