/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding KVM translation unit. */

unsafe fn check_smram_offsets() {
    /* 32 bit and 64 bit SMRAM layout checks are provided by the C ABI. */
    BUILD_BUG_ON(core::mem::size_of::<kvm_smram>() != 512);
}

unsafe fn kvm_smm_changed(vcpu: *mut kvm_vcpu, entering_smm: bool) {
    trace_kvm_smm_transition((*vcpu).vcpu_id, (*vcpu).arch.smbase, entering_smm);
    if entering_smm {
        (*vcpu).arch.hflags |= HF_SMM_MASK;
    } else {
        (*vcpu).arch.hflags &= !(HF_SMM_MASK | HF_SMM_INSIDE_NMI_MASK);
        kvm_make_request(KVM_REQ_EVENT, vcpu);
        (*vcpu).arch.pdptrs_from_userspace = false;
    }
    kvm_mmu_reset_context(vcpu);
}

unsafe fn process_smi(vcpu: *mut kvm_vcpu) {
    (*vcpu).arch.smi_pending = true;
    kvm_make_request(KVM_REQ_EVENT, vcpu);
}

unsafe fn enter_smm_get_segment_flags(seg: *mut kvm_segment) -> u32 {
    ((*seg).g << 23) | ((*seg).db << 22) | ((*seg).l << 21) |
    ((*seg).avl << 20) | ((*seg).present << 15) | ((*seg).dpl << 13) |
    ((*seg).s << 12) | ((*seg).type_ << 8)
}

unsafe fn enter_smm_save_seg_32(vcpu: *mut kvm_vcpu, state: *mut kvm_smm_seg_state_32,
                                selector: *mut u32, n: i32) {
    let mut seg = core::mem::zeroed::<kvm_segment>();
    kvm_get_segment(vcpu, &mut seg, n);
    *selector = seg.selector;
    (*state).base = seg.base;
    (*state).limit = seg.limit;
    (*state).flags = enter_smm_get_segment_flags(&mut seg);
}

unsafe fn enter_smm_save_state_32(vcpu: *mut kvm_vcpu, smram: *mut kvm_smram_state_32) {
    let mut dt = core::mem::zeroed::<desc_ptr>();
    (*smram).cr0 = kvm_read_cr0(vcpu); (*smram).cr3 = kvm_read_cr3(vcpu);
    (*smram).eflags = kvm_get_rflags(vcpu); (*smram).eip = kvm_rip_read(vcpu);
    for i in 0..8 { (*smram).gprs[i] = kvm_register_read_raw(vcpu, i); }
    (*smram).dr6 = (*vcpu).arch.dr6 as u32; (*smram).dr7 = (*vcpu).arch.dr7 as u32;
    enter_smm_save_seg_32(vcpu, &mut (*smram).tr, &mut (*smram).tr_sel, VCPU_SREG_TR);
    enter_smm_save_seg_32(vcpu, &mut (*smram).ldtr, &mut (*smram).ldtr_sel, VCPU_SREG_LDTR);
    kvm_x86_call(get_gdt)(vcpu, &mut dt); (*smram).gdtr.base = dt.address; (*smram).gdtr.limit = dt.size;
    kvm_x86_call(get_idt)(vcpu, &mut dt); (*smram).idtr.base = dt.address; (*smram).idtr.limit = dt.size;
    enter_smm_save_seg_32(vcpu, &mut (*smram).es, &mut (*smram).es_sel, VCPU_SREG_ES);
    enter_smm_save_seg_32(vcpu, &mut (*smram).cs, &mut (*smram).cs_sel, VCPU_SREG_CS);
    enter_smm_save_seg_32(vcpu, &mut (*smram).ss, &mut (*smram).ss_sel, VCPU_SREG_SS);
    enter_smm_save_seg_32(vcpu, &mut (*smram).ds, &mut (*smram).ds_sel, VCPU_SREG_DS);
    enter_smm_save_seg_32(vcpu, &mut (*smram).fs, &mut (*smram).fs_sel, VCPU_SREG_FS);
    enter_smm_save_seg_32(vcpu, &mut (*smram).gs, &mut (*smram).gs_sel, VCPU_SREG_GS);
    (*smram).cr4 = kvm_read_cr4(vcpu); (*smram).smm_revision = 0x00020000;
    (*smram).smbase = (*vcpu).arch.smbase;
    (*smram).int_shadow = kvm_x86_call(get_interrupt_shadow)(vcpu);
}

unsafe fn rsm_set_desc_flags(desc: *mut kvm_segment, flags: u32) {
    (*desc).g=(flags>>23)&1; (*desc).db=(flags>>22)&1; (*desc).l=(flags>>21)&1;
    (*desc).avl=(flags>>20)&1; (*desc).present=(flags>>15)&1; (*desc).dpl=(flags>>13)&3;
    (*desc).s=(flags>>12)&1; (*desc).type_=(flags>>8)&15;
    (*desc).unusable = !(*desc).present; (*desc).padding = 0;
}

unsafe fn rsm_load_seg_32(vcpu: *mut kvm_vcpu, state: *const kvm_smm_seg_state_32,
                          selector: u16, n: i32) -> i32 {
    let mut desc = core::mem::zeroed::<kvm_segment>(); desc.selector=selector;
    desc.base=(*state).base; desc.limit=(*state).limit;
    rsm_set_desc_flags(&mut desc, (*state).flags); kvm_set_segment(vcpu, &mut desc, n); X86EMUL_CONTINUE
}

unsafe fn rsm_enter_protected_mode(vcpu: *mut kvm_vcpu, cr0: u64, mut cr3: u64, cr4: u64) -> i32 {
    let pcid = if cr4 & X86_CR4_PCIDE != 0 { let p=cr3&0xfff; cr3&=!0xfff; p } else { 0 };
    if kvm_set_cr3(vcpu, cr3) != 0 || kvm_set_cr4(vcpu, cr4 & !X86_CR4_PCIDE) != 0 || kvm_set_cr0(vcpu, cr0) != 0 { return X86EMUL_UNHANDLEABLE; }
    if cr4 & X86_CR4_PCIDE != 0 { if kvm_set_cr4(vcpu, cr4)!=0 { return X86EMUL_UNHANDLEABLE; } if pcid!=0 && kvm_set_cr3(vcpu, cr3|pcid)!=0 { return X86EMUL_UNHANDLEABLE; } }
    X86EMUL_CONTINUE
}

unsafe fn rsm_load_state_32(ctxt: *mut x86_emulate_ctxt, smstate: *const kvm_smram_state_32) -> i32 {
    let vcpu=(*ctxt).vcpu; let mut dt=core::mem::zeroed::<desc_ptr>();
    (*ctxt).eflags=(*smstate).eflags|X86_EFLAGS_FIXED; (*ctxt)._eip=(*smstate).eip;
    for i in 0..8 { *reg_write(ctxt,i)=(*smstate).gprs[i]; }
    if kvm_set_dr(vcpu,6,(*smstate).dr6)!=0 || kvm_set_dr(vcpu,7,(*smstate).dr7)!=0 { return X86EMUL_UNHANDLEABLE; }
    rsm_load_seg_32(vcpu,&(*smstate).tr,(*smstate).tr_sel,VCPU_SREG_TR); rsm_load_seg_32(vcpu,&(*smstate).ldtr,(*smstate).ldtr_sel,VCPU_SREG_LDTR);
    dt.address=(*smstate).gdtr.base; dt.size=(*smstate).gdtr.limit; kvm_x86_call(set_gdt)(vcpu,&mut dt);
    dt.address=(*smstate).idtr.base; dt.size=(*smstate).idtr.limit; kvm_x86_call(set_idt)(vcpu,&mut dt);
    rsm_load_seg_32(vcpu,&(*smstate).es,(*smstate).es_sel,VCPU_SREG_ES); rsm_load_seg_32(vcpu,&(*smstate).cs,(*smstate).cs_sel,VCPU_SREG_CS); rsm_load_seg_32(vcpu,&(*smstate).ss,(*smstate).ss_sel,VCPU_SREG_SS);
    rsm_load_seg_32(vcpu,&(*smstate).ds,(*smstate).ds_sel,VCPU_SREG_DS); rsm_load_seg_32(vcpu,&(*smstate).fs,(*smstate).fs_sel,VCPU_SREG_FS); rsm_load_seg_32(vcpu,&(*smstate).gs,(*smstate).gs_sel,VCPU_SREG_GS);
    (*vcpu).arch.smbase=(*smstate).smbase; let r=rsm_enter_protected_mode(vcpu,(*smstate).cr0,(*smstate).cr3,(*smstate).cr4); if r!=X86EMUL_CONTINUE{return r;}
    kvm_x86_call(set_interrupt_shadow)(vcpu,0); (*ctxt).interruptibility=(*smstate).int_shadow as u8; r
}

unsafe fn enter_smm(vcpu: *mut kvm_vcpu) {
    let mut smram=core::mem::zeroed::<kvm_smram>(); check_smram_offsets();
    core::ptr::write_bytes((&mut smram as *mut kvm_smram) as *mut u8,0,core::mem::size_of::<kvm_smram>());
    enter_smm_save_state_32(vcpu,&mut smram.smram32); if kvm_x86_call(enter_smm)(vcpu,&mut smram)!=0 { kvm_vm_dead((*vcpu).kvm); return; }
    kvm_smm_changed(vcpu,true); if kvm_vcpu_write_guest(vcpu,(*vcpu).arch.smbase+0xfe00,&smram,core::mem::size_of::<kvm_smram>())!=0 { kvm_vm_dead((*vcpu).kvm); return; }
    if kvm_x86_call(get_nmi_mask)(vcpu)!=0 { (*vcpu).arch.hflags|=HF_SMM_INSIDE_NMI_MASK; } else { kvm_x86_call(set_nmi_mask)(vcpu,true); }
    kvm_set_rflags(vcpu,X86_EFLAGS_FIXED); kvm_rip_write(vcpu,0x8000); kvm_x86_call(set_interrupt_shadow)(vcpu,0);
    let cr0=(*vcpu).arch.cr0 & !(X86_CR0_PE|X86_CR0_EM|X86_CR0_TS|X86_CR0_PG); if kvm_x86_call(set_cr0)(vcpu,cr0)!=0 { kvm_vm_dead((*vcpu).kvm); return; } kvm_x86_call(set_cr4)(vcpu,0); kvm_mmu_reset_context(vcpu);
}

unsafe fn emulator_leave_smm(ctxt: *mut x86_emulate_ctxt) -> i32 {
    let vcpu=(*ctxt).vcpu; let mut smram=core::mem::zeroed::<kvm_smram>(); let smbase=(*vcpu).arch.smbase;
    if kvm_vcpu_read_guest(vcpu,smbase+0xfe00,&mut smram,core::mem::size_of::<kvm_smram>())<0{return X86EMUL_UNHANDLEABLE;}
    if (*vcpu).arch.hflags&HF_SMM_INSIDE_NMI_MASK==0{kvm_x86_call(set_nmi_mask)(vcpu,false);} kvm_smm_changed(vcpu,false);
    if kvm_x86_call(leave_smm)(vcpu,&mut smram)!=0{return X86EMUL_UNHANDLEABLE;} rsm_load_state_32(ctxt,&smram.smram32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
