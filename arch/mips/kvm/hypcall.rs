/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * KVM/MIPS: Hypercall handling.
 *
 * Copyright (C) 2015  Imagination Technologies Ltd.
 */

// Dependencies supplied by the surrounding KVM/MIPS sources are intentionally
// referenced here rather than reimplemented in this translation unit.

const MAX_HYPCALL_ARGS: usize = 4;

#[repr(C)]
pub struct kvm_vcpu {
    pub arch: kvm_vcpu_arch,
}

#[repr(C)]
pub struct kvm_vcpu_arch {
    pub pc: ::core::ffi::c_ulong,
    pub gprs: [::core::ffi::c_ulong; 32],
}

#[repr(C)]
pub struct mips_instruction_co_format {
    pub code: u32,
}

#[repr(C)]
pub union mips_instruction {
    pub co_format: mips_instruction_co_format,
}

#[repr(C)]
pub enum emulation_result {
    EMULATE_HYPERCALL,
    EMULATE_FAIL,
}

extern "C" {
    // Provided by the kernel headers/translation unit.
    static KVM_ENOSYS: ::core::ffi::c_int;
}

pub fn kvm_mips_emul_hypcall(
    vcpu: *mut kvm_vcpu,
    inst: mips_instruction,
) -> emulation_result {
    let code: u32;
    unsafe {
        code = (inst.co_format.code >> 5) & 0x3ff;

        kvm_debug!("[{:#x}] HYPCALL {:#03x}\n", (*vcpu).arch.pc, code);
    }

    match code {
        0 => emulation_result::EMULATE_HYPERCALL,
        _ => emulation_result::EMULATE_FAIL,
    }
}

unsafe fn kvm_mips_hypercall(
    _vcpu: *mut kvm_vcpu,
    _num: ::core::ffi::c_ulong,
    _args: *const ::core::ffi::c_ulong,
    hret: *mut ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    /* Report unimplemented hypercall to guest */
    *hret = (-(KVM_ENOSYS as isize)) as ::core::ffi::c_ulong;
    RESUME_GUEST
}

pub unsafe fn kvm_mips_handle_hypcall(vcpu: *mut kvm_vcpu) -> ::core::ffi::c_int {
    let num: ::core::ffi::c_ulong;
    let mut args = [0 as ::core::ffi::c_ulong; MAX_HYPCALL_ARGS];

    /* read hypcall number and arguments */
    num = (*vcpu).arch.gprs[2]; /* v0 */
    args[0] = (*vcpu).arch.gprs[4]; /* a0 */
    args[1] = (*vcpu).arch.gprs[5]; /* a1 */
    args[2] = (*vcpu).arch.gprs[6]; /* a2 */
    args[3] = (*vcpu).arch.gprs[7]; /* a3 */

    kvm_mips_hypercall(
        vcpu,
        num,
        args.as_ptr(),
        &mut (*vcpu).arch.gprs[2], /* v0 */
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
