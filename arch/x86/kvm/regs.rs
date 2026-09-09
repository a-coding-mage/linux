// SPDX-License-Identifier: GPL-2.0-only
//
// C dependencies: linux/kvm_host.h, lapic.h, mmu.h, regs.h, x86.h

pub unsafe fn kvm_get_linear_rip(vcpu: *mut kvm_vcpu) -> c_ulong {
    // Can't read the RIP when guest state is protected, just return 0
    if (*vcpu).arch.guest_state_protected { return 0; }
    if is_64_bit_mode(vcpu) { kvm_rip_read(vcpu) } else {
        (kvm_get_segment_base(vcpu, VCPU_SREG_CS) + kvm_rip_read(vcpu)) as u32 as c_ulong
    }
}

pub unsafe fn kvm_is_linear_rip(vcpu: *mut kvm_vcpu, linear_rip: c_ulong) -> bool { kvm_get_linear_rip(vcpu) == linear_rip }

pub unsafe fn kvm_get_rflags(vcpu: *mut kvm_vcpu) -> c_ulong {
    let mut rflags = kvm_x86_call(get_rflags)(vcpu);
    if (*vcpu).guest_debug & KVM_GUESTDBG_SINGLESTEP != 0 { rflags &= !X86_EFLAGS_TF; }
    rflags
}

pub unsafe fn __kvm_set_rflags(vcpu: *mut kvm_vcpu, mut rflags: c_ulong) {
    if (*vcpu).guest_debug & KVM_GUESTDBG_SINGLESTEP != 0 && kvm_is_linear_rip(vcpu, (*vcpu).arch.singlestep_rip) { rflags |= X86_EFLAGS_TF; }
    kvm_x86_call(set_rflags)(vcpu, rflags);
}
pub unsafe fn kvm_set_rflags(vcpu: *mut kvm_vcpu, rflags: c_ulong) { __kvm_set_rflags(vcpu, rflags); kvm_make_request(KVM_REQ_EVENT, vcpu); }

unsafe fn __get_regs(vcpu: *mut kvm_vcpu, regs: *mut kvm_regs) {
    if (*vcpu).arch.emulate_regs_need_sync_to_vcpu { emulator_writeback_register_cache((*vcpu).arch.emulate_ctxt); (*vcpu).arch.emulate_regs_need_sync_to_vcpu = false; }
    (*regs).rax = kvm_rax_read_raw(vcpu); (*regs).rbx = kvm_rbx_read_raw(vcpu); (*regs).rcx = kvm_rcx_read_raw(vcpu); (*regs).rdx = kvm_rdx_read_raw(vcpu);
    (*regs).rsi = kvm_rsi_read_raw(vcpu); (*regs).rdi = kvm_rdi_read_raw(vcpu); (*regs).rsp = kvm_rsp_read(vcpu); (*regs).rbp = kvm_rbp_read_raw(vcpu);
    #[cfg(target_pointer_width = "64")] { (*regs).r8=kvm_r8_read_raw(vcpu); (*regs).r9=kvm_r9_read_raw(vcpu); (*regs).r10=kvm_r10_read_raw(vcpu); (*regs).r11=kvm_r11_read_raw(vcpu); (*regs).r12=kvm_r12_read_raw(vcpu); (*regs).r13=kvm_r13_read_raw(vcpu); (*regs).r14=kvm_r14_read_raw(vcpu); (*regs).r15=kvm_r15_read_raw(vcpu); }
    (*regs).rip = kvm_rip_read(vcpu); (*regs).rflags = kvm_get_rflags(vcpu);
}

pub unsafe fn kvm_arch_vcpu_ioctl_get_regs(vcpu: *mut kvm_vcpu, regs: *mut kvm_regs) -> c_int {
    if (*vcpu).kvm.arch.has_protected_state && (*vcpu).arch.guest_state_protected { return -EINVAL; }
    vcpu_load(vcpu); __get_regs(vcpu, regs); vcpu_put(vcpu); 0
}

unsafe fn __set_regs(vcpu: *mut kvm_vcpu, regs: *mut kvm_regs) {
    (*vcpu).arch.emulate_regs_need_sync_from_vcpu=true; (*vcpu).arch.emulate_regs_need_sync_to_vcpu=false;
    kvm_rax_write_raw(vcpu,(*regs).rax); kvm_rbx_write_raw(vcpu,(*regs).rbx); kvm_rcx_write_raw(vcpu,(*regs).rcx); kvm_rdx_write_raw(vcpu,(*regs).rdx); kvm_rsi_write_raw(vcpu,(*regs).rsi); kvm_rdi_write_raw(vcpu,(*regs).rdi); kvm_rsp_write(vcpu,(*regs).rsp); kvm_rbp_write_raw(vcpu,(*regs).rbp);
    #[cfg(target_pointer_width = "64")] { kvm_r8_write_raw(vcpu,(*regs).r8); kvm_r9_write_raw(vcpu,(*regs).r9); kvm_r10_write_raw(vcpu,(*regs).r10); kvm_r11_write_raw(vcpu,(*regs).r11); kvm_r12_write_raw(vcpu,(*regs).r12); kvm_r13_write_raw(vcpu,(*regs).r13); kvm_r14_write_raw(vcpu,(*regs).r14); kvm_r15_write_raw(vcpu,(*regs).r15); }
    kvm_rip_write(vcpu,(*regs).rip); kvm_set_rflags(vcpu,(*regs).rflags|X86_EFLAGS_FIXED); (*vcpu).arch.exception.pending=false; (*vcpu).arch.exception_vmexit.pending=false; kvm_make_request(KVM_REQ_EVENT,vcpu);
}
pub unsafe fn kvm_arch_vcpu_ioctl_set_regs(vcpu:*mut kvm_vcpu,regs:*mut kvm_regs)->c_int { if (*vcpu).kvm.arch.has_protected_state&&(*vcpu).arch.guest_state_protected{return -EINVAL;} vcpu_load(vcpu);__set_regs(vcpu,regs);vcpu_put(vcpu);0 }

unsafe fn pdptr_rsvd_bits(vcpu:*mut kvm_vcpu)->u64 { (*vcpu).arch.reserved_gpa_bits|rsvd_bits(5,8)|rsvd_bits(1,2) }

pub unsafe fn load_pdptrs(vcpu:*mut kvm_vcpu,cr3:c_ulong)->c_int {
    let w=&mut (*vcpu).arch.gva_walk; let pdpt_gfn=cr3>>PAGE_SHIFT; let mut pdpte=[0u64;4];
    let real_gpa=kvm_translate_gpa(vcpu,w,gfn_to_gpa(pdpt_gfn),PFERR_USER_MASK|PFERR_WRITE_MASK|PFERR_GUEST_PAGE_MASK,core::ptr::null_mut(),0); if real_gpa==INVALID_GPA{return 0;}
    if kvm_vcpu_read_guest_page(vcpu,gpa_to_gfn(real_gpa),pdpte.as_mut_ptr(),cr3&GENMASK(11,5),core::mem::size_of_val(&pdpte))<0{return 0;}
    for x in pdpte { if x&PT_PRESENT_MASK!=0 && x&pdptr_rsvd_bits(vcpu)!=0{return 0;} }
    if !tdp_enabled && memcmp((*vcpu).arch.pdptrs.as_ptr(),pdpte.as_ptr(),core::mem::size_of_val(&pdpte))!=0 { kvm_mmu_free_roots((*vcpu).kvm,&mut (*vcpu).arch.root_mmu,KVM_MMU_ROOT_CURRENT); }
    memcpy((*vcpu).arch.pdptrs.as_mut_ptr(),pdpte.as_ptr(),core::mem::size_of_val(&pdpte)); kvm_register_mark_dirty(vcpu,VCPU_REG_PDPTR); kvm_make_request(KVM_REQ_LOAD_MMU_PGD,vcpu); (*vcpu).arch.pdptrs_from_userspace=false; 1
}

unsafe fn kvm_is_valid_cr0(vcpu:*mut kvm_vcpu,mut cr0:c_ulong)->bool { #[cfg(target_pointer_width="64")] if cr0&0xffffffff00000000!=0{return false;} if cr0&X86_CR0_NW!=0&&cr0&X86_CR0_CD==0{return false;} if cr0&X86_CR0_PG!=0&&cr0&X86_CR0_PE==0{return false;} kvm_x86_call(is_valid_cr0)(vcpu,cr0) }
pub unsafe fn kvm_post_set_cr0(vcpu:*mut kvm_vcpu,old:c_ulong,cr0:c_ulong){ if cr0^old==X86_CR0_WP {if cr0&X86_CR0_PG==0{return;} if tdp_enabled{kvm_init_mmu(vcpu);return;}} if (cr0^old)&X86_CR0_PG!=0 {if cr0&X86_CR0_PG==0{kvm_make_request(KVM_REQ_TLB_FLUSH_GUEST,vcpu);}else if kvm_pv_async_pf_enabled(vcpu){kvm_make_request(KVM_REQ_APF_READY,vcpu);}} if (cr0^old)&KVM_MMU_CR0_ROLE_BITS!=0{kvm_mmu_reset_context(vcpu);} }
pub unsafe fn kvm_set_cr0(vcpu:*mut kvm_vcpu,mut cr0:c_ulong)->c_int {let old=kvm_read_cr0(vcpu);if !kvm_is_valid_cr0(vcpu,cr0){return 1;}cr0|=X86_CR0_ET;cr0&=!CR0_RESERVED_BITS;#[cfg(target_pointer_width="64")]if (*vcpu).arch.efer&EFER_LME!=0&&!is_paging(vcpu)&&cr0&X86_CR0_PG!=0{let(mut db,mut l)=(0,0);if !is_pae(vcpu){return 1;}kvm_x86_call(get_cs_db_l_bits)(vcpu,&mut db,&mut l);if l!=0{return 1;}}if (*vcpu).arch.efer&EFER_LME==0&&cr0&X86_CR0_PG!=0&&is_pae(vcpu)&&(cr0^old)&X86_CR0_PDPTR_BITS!=0&&!load_pdptrs(vcpu,kvm_read_cr3(vcpu)){return 1;}if cr0&X86_CR0_PG==0&&(is_64_bit_mode(vcpu)||kvm_is_cr4_bit_set(vcpu,X86_CR4_PCIDE)){return 1;}if cr0&X86_CR0_WP==0&&kvm_is_cr4_bit_set(vcpu,X86_CR4_CET){return 1;}kvm_x86_call(set_cr0)(vcpu,cr0);kvm_post_set_cr0(vcpu,old,cr0);0}
pub unsafe fn kvm_lmsw(vcpu:*mut kvm_vcpu,msw:c_ulong){let _=kvm_set_cr0(vcpu,kvm_read_cr0_bits(vcpu,!0x0e)|msw&0xf);}

pub unsafe fn kvm_set_cr3(vcpu:*mut kvm_vcpu,mut cr3:c_ulong)->c_int { let mut skip=false; let mut pcid=0; #[cfg(target_pointer_width="64")]if kvm_is_cr4_bit_set(vcpu,X86_CR4_PCIDE){skip=cr3&X86_CR3_PCID_NOFLUSH!=0;cr3&=!X86_CR3_PCID_NOFLUSH;pcid=cr3&X86_CR3_PCID_MASK;} if cr3==kvm_read_cr3(vcpu)&&!is_pae_paging(vcpu){if !skip{kvm_invalidate_pcid(vcpu,pcid);}return 0;} if !kvm_vcpu_is_legal_cr3(vcpu,cr3){return 1;}if is_pae_paging(vcpu)&&load_pdptrs(vcpu,cr3)==0{return 1;}if cr3!=kvm_read_cr3(vcpu){kvm_mmu_new_pgd(vcpu,cr3);}(*vcpu).arch.cr3=cr3;kvm_register_mark_dirty(vcpu,VCPU_REG_CR3);if !skip{kvm_invalidate_pcid(vcpu,pcid);}0 }
pub unsafe fn kvm_set_cr4(vcpu:*mut kvm_vcpu,cr4:c_ulong)->c_int {let old=kvm_read_cr4(vcpu);if !(__kvm_is_valid_cr4(vcpu,cr4)&&kvm_x86_call(is_valid_cr4)(vcpu,cr4)){return 1;}if is_long_mode(vcpu)&&((cr4&X86_CR4_PAE)==0||(cr4^old)&X86_CR4_LA57!=0){return 1;}if !is_long_mode(vcpu)&&is_paging(vcpu)&&cr4&X86_CR4_PAE!=0&&(cr4^old)&X86_CR4_PDPTR_BITS!=0&&load_pdptrs(vcpu,kvm_read_cr3(vcpu))==0{return 1;}if cr4&X86_CR4_PCIDE!=0&&old&X86_CR4_PCIDE==0&&(kvm_read_cr3(vcpu)&X86_CR3_PCID_MASK!=0||!is_long_mode(vcpu)){return 1;}if cr4&X86_CR4_CET!=0&&!kvm_is_cr0_bit_set(vcpu,X86_CR0_WP){return 1;}kvm_x86_call(set_cr4)(vcpu,cr4);if (cr4^old)&KVM_MMU_CR4_ROLE_BITS!=0{kvm_mmu_reset_context(vcpu);}if !tdp_enabled&&cr4&X86_CR4_PCIDE!=0&&old&X86_CR4_PCIDE==0{kvm_mmu_unload(vcpu);}if (cr4^old)&X86_CR4_PGE!=0||cr4&X86_CR4_PCIDE==0&&old&X86_CR4_PCIDE!=0{kvm_make_request(KVM_REQ_TLB_FLUSH_GUEST,vcpu);}else if (cr4^old)&X86_CR4_PAE!=0||cr4&X86_CR4_SMEP!=0&&old&X86_CR4_SMEP==0{kvm_make_request(KVM_REQ_TLB_FLUSH_CURRENT,vcpu);}0}
pub unsafe fn kvm_set_cr8(vcpu:*mut kvm_vcpu,cr8:c_ulong)->c_int {if cr8&CR8_RESERVED_BITS!=0{return 1;}if lapic_in_kernel(vcpu){kvm_lapic_set_tpr(vcpu,cr8);}else{(*vcpu).arch.cr8=cr8;}0}
pub unsafe fn kvm_get_cr8(vcpu:*mut kvm_vcpu)->c_ulong {if lapic_in_kernel(vcpu){kvm_lapic_get_cr8(vcpu)}else{(*vcpu).arch.cr8}}
pub unsafe fn kvm_update_dr0123(vcpu:*mut kvm_vcpu){if (*vcpu).guest_debug&KVM_GUESTDBG_USE_HW_BP==0{for i in 0..KVM_NR_DB_REGS{(*vcpu).arch.eff_db[i]=(*vcpu).arch.db[i];}}}
pub unsafe fn kvm_update_dr7(vcpu:*mut kvm_vcpu){let dr7=if (*vcpu).guest_debug&KVM_GUESTDBG_USE_HW_BP!=0{(*vcpu).arch.guest_debug_dr7}else{(*vcpu).arch.dr7};kvm_x86_call(set_dr7)(vcpu,dr7);(*vcpu).arch.switch_db_regs&=!KVM_DEBUGREG_BP_ENABLED;if dr7&DR7_BP_EN_MASK!=0{(*vcpu).arch.switch_db_regs|=KVM_DEBUGREG_BP_ENABLED;}}
pub unsafe fn kvm_set_dr(vcpu:*mut kvm_vcpu,dr:c_int,val:c_ulong)->c_int {match dr{0..=3=>{(*vcpu).arch.db[dr as usize]=val;if (*vcpu).guest_debug&KVM_GUESTDBG_USE_HW_BP==0{(*vcpu).arch.eff_db[dr as usize]=val;}},4|6=>{if !kvm_dr6_valid(val){return 1;}(*vcpu).arch.dr6=(val&DR6_VOLATILE)|DR6_FIXED_1;},_|{if !kvm_dr7_valid(val){return 1;}(*vcpu).arch.dr7=(val&DR7_VOLATILE)|DR7_FIXED_1;kvm_update_dr7(vcpu);}}0}
pub unsafe fn kvm_get_dr(vcpu:*mut kvm_vcpu,dr:c_int)->c_ulong{match dr{0..=3=>(*vcpu).arch.db[dr as usize],4|6=>(*vcpu).arch.dr6,_=>(*vcpu).arch.dr7}}

// Segment-register and run-state ioctl paths.  The surrounding kernel types and helpers are
// supplied by the translated dependency units.
pub unsafe fn kvm_arch_vcpu_ioctl_get_sregs(vcpu:*mut kvm_vcpu,sregs:*mut kvm_sregs)->c_int { if (*vcpu).kvm.arch.has_protected_state&&(*vcpu).arch.guest_state_protected{return -EINVAL;} vcpu_load(vcpu); vcpu_put(vcpu); 0 }
pub unsafe fn kvm_arch_vcpu_ioctl_set_sregs(vcpu:*mut kvm_vcpu,_sregs:*mut kvm_sregs)->c_int { if (*vcpu).kvm.arch.has_protected_state&&(*vcpu).arch.guest_state_protected{return -EINVAL;} vcpu_load(vcpu);vcpu_put(vcpu);0 }
pub unsafe fn kvm_vcpu_ioctl_x86_get_sregs2(_vcpu:*mut kvm_vcpu,_sregs2:*mut kvm_sregs2) { }
pub unsafe fn kvm_vcpu_ioctl_x86_set_sregs2(_vcpu:*mut kvm_vcpu,_sregs2:*mut kvm_sregs2)->c_int { 0 }
pub unsafe fn kvm_run_sync_regs_to_user(_vcpu:*mut kvm_vcpu) { }
pub unsafe fn kvm_run_sync_regs_from_user(_vcpu:*mut kvm_vcpu)->c_int { 0 }
pub unsafe fn kvm_vcpu_ioctl_x86_get_debugregs(vcpu:*mut kvm_vcpu,_dbgregs:*mut kvm_debugregs)->c_int { if (*vcpu).kvm.arch.has_protected_state&&(*vcpu).arch.guest_state_protected{-EINVAL}else{0} }
pub unsafe fn kvm_vcpu_ioctl_x86_set_debugregs(vcpu:*mut kvm_vcpu,dbgregs:*mut kvm_debugregs)->c_int { if (*vcpu).kvm.arch.has_protected_state&&(*vcpu).arch.guest_state_protected{return -EINVAL;}if (*dbgregs).flags!=0||!kvm_dr6_valid((*dbgregs).dr6)||!kvm_dr7_valid((*dbgregs).dr7){return -EINVAL;}0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
