// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2021 Intel Corporation. */
// C dependencies are supplied by the surrounding kernel translation.

pub static mut enable_sgx: bool = true;
static mut sgx_pubkey_hash: [u64; 4] = [0; 4];

unsafe fn sgx_get_encls_gva(vcpu: *mut kvm_vcpu, offset: c_ulong, size: c_int,
                            alignment: c_int, gva: *mut gva_t) -> c_int {
    let mut s: kvm_segment = core::mem::zeroed();
    let mut fault: bool;
    *gva = offset;
    if !is_64_bit_mode(vcpu) {
        vmx_get_segment(vcpu, &mut s, VCPU_SREG_DS);
        *gva = (*gva).wrapping_add(s.base);
    }
    if !IS_ALIGNED!(*gva, alignment) {
        fault = true;
    } else if likely(is_64_bit_mode(vcpu)) {
        *gva = vmx_get_untagged_addr(vcpu, *gva, 0);
        fault = is_noncanonical_address(*gva, vcpu, 0);
    } else {
        *gva &= 0xffff_ffff;
        fault = s.unusable || (s.type_ != 2 && s.type_ != 3) || *gva > s.limit ||
            ((s.base != 0 || s.limit != 0xffff_ffff) &&
             ((*gva as u64).wrapping_add(size as u64).wrapping_sub(1) > s.limit as u64 + 1));
    }
    if fault { kvm_inject_gp(vcpu, 0); }
    if fault { -EINVAL } else { 0 }
}

unsafe fn sgx_handle_emulation_failure(vcpu: *mut kvm_vcpu, addr: u64, size: c_uint) {
    let data = [addr, size as u64];
    __kvm_prepare_emulation_failure_exit(vcpu, data.as_ptr(), data.len());
}

unsafe fn sgx_read_hva(vcpu: *mut kvm_vcpu, hva: c_ulong, data: *mut c_void, size: c_uint) -> c_int {
    if __copy_from_user(data, hva as *const c_void, size) != 0 {
        sgx_handle_emulation_failure(vcpu, hva, size); return -EFAULT;
    }
    0
}

unsafe fn sgx_gva_to_gpa(vcpu: *mut kvm_vcpu, gva: gva_t, write: bool, gpa: *mut gpa_t) -> c_int {
    let mut ex: x86_exception = core::mem::zeroed();
    *gpa = if write { kvm_mmu_gva_to_gpa_write(vcpu, gva, &mut ex) }
           else { kvm_mmu_gva_to_gpa_read(vcpu, gva, &mut ex) };
    if *gpa == INVALID_GPA { kvm_inject_emulated_page_fault(vcpu, &mut ex); return -EFAULT; }
    0
}

unsafe fn sgx_gpa_to_hva(vcpu: *mut kvm_vcpu, gpa: gpa_t, hva: *mut c_ulong) -> c_int {
    *hva = kvm_vcpu_gfn_to_hva(vcpu, PFN_DOWN!(gpa));
    if kvm_is_error_hva(*hva) { sgx_handle_emulation_failure(vcpu, gpa, 1); return -EFAULT; }
    *hva |= gpa & !PAGE_MASK;
    0
}

unsafe fn sgx_inject_fault(vcpu: *mut kvm_vcpu, gva: gva_t, trapnr: c_int) -> c_int {
    let mut ex: x86_exception = core::mem::zeroed();
    if trapnr == PF_VECTOR && !boot_cpu_has(X86_FEATURE_SGX2) { kvm_prepare_emulation_failure_exit(vcpu); return 0; }
    if (trapnr == PF_VECTOR || !boot_cpu_has(X86_FEATURE_SGX2)) && guest_cpu_cap_has(vcpu, X86_FEATURE_SGX2) {
        core::ptr::write_bytes(&mut ex, 0, 1);
        ex.vector = PF_VECTOR;
        ex.error_code = PFERR_PRESENT_MASK | PFERR_WRITE_MASK | PFERR_SGX_MASK;
        ex.address = gva;
        ex.error_code_valid = true;
        ex.nested_page_fault = false;
        kvm_inject_emulated_page_fault(vcpu, &mut ex);
    } else { kvm_inject_gp(vcpu, 0); }
    1
}

unsafe fn __handle_encls_ecreate(vcpu: *mut kvm_vcpu, pageinfo: *mut sgx_pageinfo,
                                 secs_hva: c_ulong, secs_gva: gva_t) -> c_int {
    let contents = (*pageinfo).contents as *mut sgx_secs;
    let sgx_12_0 = kvm_find_cpuid_entry_index(vcpu, 0x12, 0);
    let sgx_12_1 = kvm_find_cpuid_entry_index(vcpu, 0x12, 1);
    if sgx_12_0.is_null() || sgx_12_1.is_null() { kvm_prepare_emulation_failure_exit(vcpu); return 0; }
    let miscselect = (*contents).miscselect;
    let attributes = (*contents).attributes;
    let xfrm = (*contents).xfrm;
    let size = (*contents).size;
    if !(*(*vcpu).kvm).arch.sgx_provisioning_allowed && (attributes & SGX_ATTR_PROVISIONKEY) != 0 {
        if (*sgx_12_1).eax & SGX_ATTR_PROVISIONKEY != 0 { pr_warn_once!("SGX PROVISIONKEY advertised but not allowed\\n"); }
        kvm_inject_gp(vcpu, 0); return 1;
    }
    if (miscselect as u32 & !(*sgx_12_0).ebx) != 0 ||
       (attributes as u32 & !(*sgx_12_1).eax) != 0 ||
       ((attributes >> 32) as u32 & !(*sgx_12_1).ebx) != 0 ||
       (xfrm as u32 & !(*sgx_12_1).ecx) != 0 ||
       ((xfrm >> 32) as u32 & !(*sgx_12_1).edx) != 0 ||
       xfrm & !((*vcpu).arch.guest_supported_xcr0 | XFEATURE_MASK_FPSSE) != 0 ||
       xfrm & XFEATURE_MASK_FPSSE != XFEATURE_MASK_FPSSE { kvm_inject_gp(vcpu, 0); return 1; }
    let max_size_log2 = if attributes & SGX_ATTR_MODE64BIT != 0 { (*sgx_12_0).edx >> 8 } else { (*sgx_12_0).edx };
    if size >= BIT_ULL!(max_size_log2) { kvm_inject_gp(vcpu, 0); return 1; }
    let mut trapnr = 0;
    let ret = sgx_virt_ecreate(pageinfo, secs_hva as *mut c_void, &mut trapnr);
    if ret == 0 { return kvm_skip_emulated_instruction(vcpu); }
    if ret == -EFAULT { return sgx_inject_fault(vcpu, secs_gva, trapnr); }
    ret
}

unsafe fn handle_encls_ecreate(vcpu: *mut kvm_vcpu) -> c_int {
    let (mut pageinfo_gva, mut secs_gva, mut metadata_gva, mut contents_gva) = (0,0,0,0);
    let (mut metadata_gpa, mut contents_gpa, mut secs_gpa) = (0,0,0);
    let (mut metadata_hva, mut contents_hva, mut secs_hva) = (0,0,0);
    let mut pageinfo: sgx_pageinfo = core::mem::zeroed();
    let mut ex: x86_exception = core::mem::zeroed();
    if sgx_get_encls_gva(vcpu, kvm_rbx_read(vcpu), 32, 32, &mut pageinfo_gva) != 0 ||
       sgx_get_encls_gva(vcpu, kvm_rcx_read(vcpu), 4096, 4096, &mut secs_gva) != 0 { return 1; }
    let r = kvm_read_guest_virt(vcpu, pageinfo_gva, &mut pageinfo, core::mem::size_of::<sgx_pageinfo>(), &mut ex);
    if r == X86EMUL_PROPAGATE_FAULT { kvm_inject_emulated_page_fault(vcpu, &mut ex); return 1; }
    if r != X86EMUL_CONTINUE { sgx_handle_emulation_failure(vcpu, pageinfo_gva, core::mem::size_of::<sgx_pageinfo>() as u32); return 0; }
    if sgx_get_encls_gva(vcpu, pageinfo.metadata, 64, 64, &mut metadata_gva) != 0 || sgx_get_encls_gva(vcpu, pageinfo.contents, 4096, 4096, &mut contents_gva) != 0 { return 1; }
    if sgx_gva_to_gpa(vcpu, metadata_gva, false, &mut metadata_gpa) != 0 || sgx_gva_to_gpa(vcpu, contents_gva, false, &mut contents_gpa) != 0 || sgx_gva_to_gpa(vcpu, secs_gva, true, &mut secs_gpa) != 0 { return 1; }
    if sgx_gpa_to_hva(vcpu, metadata_gpa, &mut metadata_hva) != 0 || sgx_gpa_to_hva(vcpu, contents_gpa, &mut contents_hva) != 0 || sgx_gpa_to_hva(vcpu, secs_gpa, &mut secs_hva) != 0 { return 0; }
    let contents = __get_free_page(GFP_KERNEL) as *mut sgx_secs;
    if contents.is_null() { return -ENOMEM; }
    if sgx_read_hva(vcpu, contents_hva, contents as *mut c_void, PAGE_SIZE) != 0 { free_page(contents as c_ulong); return 0; }
    pageinfo.metadata = metadata_hva; pageinfo.contents = contents as u64;
    let r = __handle_encls_ecreate(vcpu, &mut pageinfo, secs_hva, secs_gva);
    free_page(contents as c_ulong); r
}

unsafe fn handle_encls_einit(vcpu: *mut kvm_vcpu) -> c_int {
    let (mut sig_hva, mut secs_hva, mut token_hva, mut rflags) = (0,0,0,0);
    let vmx = to_vmx(vcpu); let (mut sig_gva, mut secs_gva, mut token_gva) = (0,0,0);
    let (mut sig_gpa, mut secs_gpa, mut token_gpa) = (0,0,0); let mut trapnr = 0;
    if sgx_get_encls_gva(vcpu,kvm_rbx_read(vcpu),1808,4096,&mut sig_gva)!=0 || sgx_get_encls_gva(vcpu,kvm_rcx_read(vcpu),4096,4096,&mut secs_gva)!=0 || sgx_get_encls_gva(vcpu,kvm_rdx_read(vcpu),304,512,&mut token_gva)!=0 { return 1; }
    if sgx_gva_to_gpa(vcpu,sig_gva,false,&mut sig_gpa)!=0 || sgx_gva_to_gpa(vcpu,secs_gva,true,&mut secs_gpa)!=0 || sgx_gva_to_gpa(vcpu,token_gva,false,&mut token_gpa)!=0 { return 1; }
    if sgx_gpa_to_hva(vcpu,sig_gpa,&mut sig_hva)!=0 || sgx_gpa_to_hva(vcpu,secs_gpa,&mut secs_hva)!=0 || sgx_gpa_to_hva(vcpu,token_gpa,&mut token_hva)!=0 { return 0; }
    let ret=sgx_virt_einit(sig_hva as *mut c_void,token_hva as *mut c_void,secs_hva as *mut c_void,(*vmx).msr_ia32_sgxlepubkeyhash.as_ptr(),&mut trapnr);
    if ret == -EFAULT { return sgx_inject_fault(vcpu,secs_gva,trapnr); } if ret < 0 { return ret; }
    rflags=vmx_get_rflags(vcpu)&!(X86_EFLAGS_CF|X86_EFLAGS_PF|X86_EFLAGS_AF|X86_EFLAGS_SF|X86_EFLAGS_OF);
    if ret != 0 { rflags|=X86_EFLAGS_ZF; } else { rflags&=!X86_EFLAGS_ZF; } vmx_set_rflags(vcpu,rflags); kvm_eax_write(vcpu,ret); kvm_skip_emulated_instruction(vcpu)
}

#[inline] unsafe fn encls_leaf_enabled_in_guest(vcpu:*mut kvm_vcpu,leaf:u32)->bool { if leaf>=ECREATE&&leaf<=ETRACK {true} else if leaf>=EAUG&&leaf<=EMODT {guest_cpu_cap_has(vcpu,X86_FEATURE_SGX2)} else {false} }
#[inline] unsafe fn sgx_enabled_in_guest_bios(vcpu:*mut kvm_vcpu)->bool { let bits=FEAT_CTL_SGX_ENABLED|FEAT_CTL_LOCKED; (*to_vmx(vcpu)).msr_ia32_feature_control&bits==bits }

pub unsafe fn handle_encls(vcpu:*mut kvm_vcpu)->c_int { let leaf=kvm_eax_read(vcpu); if !enable_sgx||!guest_cpu_cap_has(vcpu,X86_FEATURE_SGX)||!guest_cpu_cap_has(vcpu,X86_FEATURE_SGX1){kvm_queue_exception(vcpu,UD_VECTOR);}else if !encls_leaf_enabled_in_guest(vcpu,leaf)||!sgx_enabled_in_guest_bios(vcpu)||!is_paging(vcpu){kvm_inject_gp(vcpu,0);}else{if leaf==ECREATE{return handle_encls_ecreate(vcpu)}if leaf==EINIT{return handle_encls_einit(vcpu)}WARN_ONCE!(true,"unexpected exit on ENCLS[%u]",leaf);(*(*vcpu).run).exit_reason=KVM_EXIT_UNKNOWN;(*(*vcpu).run).hw.hardware_exit_reason=EXIT_REASON_ENCLS;return 0;}1 }

pub unsafe fn setup_default_sgx_lepubkeyhash(){if !enable_sgx||boot_cpu_has(X86_FEATURE_SGX_LC)||rdmsrq_safe(MSR_IA32_SGXLEPUBKEYHASH0,&mut sgx_pubkey_hash[0]){sgx_pubkey_hash=[0xa6053e051270b7ac,0x6cfbe8ba8b3b413d,0xc4916d99f2b3735d,0xd4f8c05909f9bb3b];}else{rdmsrq(MSR_IA32_SGXLEPUBKEYHASH1,&mut sgx_pubkey_hash[1]);rdmsrq(MSR_IA32_SGXLEPUBKEYHASH2,&mut sgx_pubkey_hash[2]);rdmsrq(MSR_IA32_SGXLEPUBKEYHASH3,&mut sgx_pubkey_hash[3]);}}
pub unsafe fn vcpu_setup_sgx_lepubkeyhash(vcpu:*mut kvm_vcpu){memcpy((*to_vmx(vcpu)).msr_ia32_sgxlepubkeyhash.as_mut_ptr() as *mut c_void,sgx_pubkey_hash.as_ptr() as *const c_void,core::mem::size_of_val(&sgx_pubkey_hash));}

unsafe fn sgx_intercept_encls_ecreate(vcpu:*mut kvm_vcpu)->bool{let mut eax=0;let(mut ebx,mut ecx,mut edx)=(0,0,0);if !(*(*vcpu).kvm).arch.sgx_provisioning_allowed{return true}let mut guest=kvm_find_cpuid_entry_index(vcpu,0x12,0);if guest.is_null(){return true}cpuid_count(0x12,0,&mut eax,&mut ebx,&mut ecx,&mut edx);if(*guest).ebx!=ebx||(*guest).edx!=edx{return true}guest=kvm_find_cpuid_entry_index(vcpu,0x12,1);if guest.is_null(){return true}cpuid_count(0x12,1,&mut eax,&mut ebx,&mut ecx,&mut edx);(*guest).eax!=eax||(*guest).ebx!=ebx||(*guest).ecx!=ecx||(*guest).edx!=edx}

pub unsafe fn vmx_write_encls_bitmap(vcpu:*mut kvm_vcpu,vmcs12:*mut vmcs12){let mut bitmap=!0u64;if !cpu_has_vmx_encls_vmexit(){return}if guest_cpu_cap_has(vcpu,X86_FEATURE_SGX)&&sgx_enabled_in_guest_bios(vcpu){if guest_cpu_cap_has(vcpu,X86_FEATURE_SGX1){bitmap&=!GENMASK_ULL!(ETRACK,ECREATE);if sgx_intercept_encls_ecreate(vcpu){bitmap|=1u64<<ECREATE}}if guest_cpu_cap_has(vcpu,X86_FEATURE_SGX2){bitmap&=!GENMASK_ULL!(EMODT,EAUG)}if boot_cpu_has(X86_FEATURE_SGX_LC){bitmap|=1u64<<EINIT}if vmcs12.is_null()&&is_guest_mode(vcpu){vmcs12=get_vmcs12(vcpu)}if !vmcs12.is_null()&&nested_cpu_has_encls_exit(vmcs12){bitmap|=(*vmcs12).encls_exiting_bitmap}}vmcs_write64(ENCLS_EXITING_BITMAP,bitmap);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
