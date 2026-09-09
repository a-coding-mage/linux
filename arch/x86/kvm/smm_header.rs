/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <linux/build_bug.h>
// The following declarations are enabled when CONFIG_KVM_SMM is defined.

/*
 * 32 bit KVM's emulated SMM layout. Based on Intel P6 layout
 * (https://www.sandpile.org/x86/smm.htm).
 */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct kvm_smm_seg_state_32 {
    pub flags: u32,
    pub limit: u32,
    pub base: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct kvm_smram_state_32 {
    pub reserved1: [u32; 62],
    pub smbase: u32,
    pub smm_revision: u32,
    pub io_inst_restart: u16,
    pub auto_hlt_restart: u16,
    pub io_restart_rdi: u32,
    pub io_restart_rcx: u32,
    pub io_restart_rsi: u32,
    pub io_restart_rip: u32,
    pub cr4: u32,
    // A20M#, CPL, shutdown and other reserved/undocumented fields
    pub reserved2: u16,
    // KVM extension
    pub int_shadow: u8,
    pub reserved3: [u8; 17],
    pub ds: kvm_smm_seg_state_32,
    pub fs: kvm_smm_seg_state_32,
    pub gs: kvm_smm_seg_state_32,
    // IDTR has only base and limit
    pub idtr: kvm_smm_seg_state_32,
    pub tr: kvm_smm_seg_state_32,
    pub reserved: u32,
    // GDTR has only base and limit
    pub gdtr: kvm_smm_seg_state_32,
    pub ldtr: kvm_smm_seg_state_32,
    pub es: kvm_smm_seg_state_32,
    pub cs: kvm_smm_seg_state_32,
    pub ss: kvm_smm_seg_state_32,
    pub es_sel: u32,
    pub cs_sel: u32,
    pub ss_sel: u32,
    pub ds_sel: u32,
    pub fs_sel: u32,
    pub gs_sel: u32,
    pub ldtr_sel: u32,
    pub tr_sel: u32,
    pub dr7: u32,
    pub dr6: u32,
    // GPRS in the "natural" X86 order (EAX/ECX/EDX.../EDI)
    pub gprs: [u32; 8],
    pub eip: u32,
    pub eflags: u32,
    pub cr3: u32,
    pub cr0: u32,
}

/* 64 bit KVM's emulated SMM layout. Based on AMD64 layout */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_smm_seg_state_64 {
    pub selector: u16,
    pub attributes: u16,
    pub limit: u32,
    pub base: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_smram_state_64 {
    pub es: kvm_smm_seg_state_64,
    pub cs: kvm_smm_seg_state_64,
    pub ss: kvm_smm_seg_state_64,
    pub ds: kvm_smm_seg_state_64,
    pub fs: kvm_smm_seg_state_64,
    pub gs: kvm_smm_seg_state_64,
    // GDTR has only base and limit
    pub gdtr: kvm_smm_seg_state_64,
    pub ldtr: kvm_smm_seg_state_64,
    // IDTR has only base and limit
    pub idtr: kvm_smm_seg_state_64,
    pub tr: kvm_smm_seg_state_64,
    // I/O restart and auto halt restart are not implemented by KVM
    pub io_restart_rip: u64,
    pub io_restart_rcx: u64,
    pub io_restart_rsi: u64,
    pub io_restart_rdi: u64,
    pub io_restart_dword: u32,
    pub reserved1: u32,
    pub io_inst_restart: u8,
    pub auto_hlt_restart: u8,
    // Documented in AMD BKDG as NMI mask, not used by KVM
    pub amd_nmi_mask: u8,
    pub int_shadow: u8,
    pub reserved2: u32,
    pub efer: u64,
    /*
     * Two fields below are implemented on AMD only, to store
     * SVM guest vmcb address if the #SMI was received while in the guest mode.
     */
    pub svm_guest_flag: u64,
    pub svm_guest_vmcb_gpa: u64,
    // unknown purpose, not implemented
    pub svm_guest_virtual_int: u64,
    pub reserved3: [u32; 3],
    pub smm_revison: u32,
    pub smbase: u32,
    pub reserved4: [u32; 5],
    pub ssp: u64,
    // svm_* fields below are not implemented by KVM
    pub svm_guest_pat: u64,
    pub svm_host_efer: u64,
    pub svm_host_cr4: u64,
    pub svm_host_cr3: u64,
    pub svm_host_cr0: u64,
    pub cr4: u64,
    pub cr3: u64,
    pub cr0: u64,
    pub dr7: u64,
    pub dr6: u64,
    pub rflags: u64,
    pub rip: u64,
    // GPRS in a reversed "natural" X86 order (R15/R14/../RCX/RAX.)
    pub gprs: [u64; 16],
}

#[repr(C)]
pub union kvm_smram {
    pub smram64: kvm_smram_state_64,
    pub smram32: kvm_smram_state_32,
    pub bytes: [u8; 512],
}

#[cfg(feature = "CONFIG_KVM_SMM")]
pub unsafe fn kvm_inject_smi(vcpu: *mut kvm_vcpu) -> i32 {
    if !kvm_x86_call(has_emulated_msr)((*vcpu).kvm, MSR_IA32_SMBASE) {
        return -ENOTTY;
    }

    kvm_make_request(KVM_REQ_SMI, vcpu);
    0
}

#[cfg(feature = "CONFIG_KVM_SMM")]
pub unsafe fn is_smm(vcpu: *mut kvm_vcpu) -> bool {
    ((*vcpu).arch.hflags & HF_SMM_MASK) != 0
}

#[cfg(feature = "CONFIG_KVM_SMM")]
unsafe extern "C" {
    pub fn kvm_smm_changed(vcpu: *mut kvm_vcpu, in_smm: bool);
    pub fn enter_smm(vcpu: *mut kvm_vcpu);
    pub fn emulator_leave_smm(ctxt: *mut x86_emulate_ctxt) -> i32;
    pub fn process_smi(vcpu: *mut kvm_vcpu);
}

#[cfg(not(feature = "CONFIG_KVM_SMM"))]
pub unsafe fn kvm_inject_smi(_vcpu: *mut kvm_vcpu) -> i32 { -ENOTTY }

#[cfg(not(feature = "CONFIG_KVM_SMM"))]
pub unsafe fn is_smm(_vcpu: *mut kvm_vcpu) -> bool { false }

// emulator_leave_smm is used as a function pointer, so the stub is defined in x86.c.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
