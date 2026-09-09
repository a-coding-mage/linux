// SPDX-License-Identifier: GPL-2.0
// C headers and build-time definitions are supplied by the surrounding tree.

extern "C" {
    fn um_i387_from_fxsr(i387: *mut _fpstate_32, fxsave: *const _fpstate_64) -> ::core::ffi::c_int;
    fn um_fxsr_from_i387(fxsave: *mut _fpstate_64, from: *const _fpstate_32) -> ::core::ffi::c_int;
    fn fpregs_legacy_get(
        target: *mut task_struct,
        regset: *const user_regset,
        to: membuf,
    ) -> ::core::ffi::c_int;
}

pub unsafe fn get_regs_from_mc(regs: *mut uml_pt_regs, mc: *mut mcontext_t) {
    #[cfg(target_arch = "x86")]
    {
        (*regs).gp[REG_GS] = (*mc).gregs[REG_GS] & 0xffff;
        (*regs).gp[REG_FS] = (*mc).gregs[REG_FS] & 0xffff;
        (*regs).gp[REG_ES] = (*mc).gregs[REG_ES] & 0xffff;
        (*regs).gp[REG_DS] = (*mc).gregs[REG_DS] & 0xffff;
        (*regs).gp[REG_EDI] = (*mc).gregs[REG_EDI];
        (*regs).gp[REG_ESI] = (*mc).gregs[REG_ESI];
        (*regs).gp[REG_EBP] = (*mc).gregs[REG_EBP];
        (*regs).gp[REG_UESP] = (*mc).gregs[REG_ESP]; // sic
        (*regs).gp[REG_EBX] = (*mc).gregs[REG_EBX];
        (*regs).gp[REG_EDX] = (*mc).gregs[REG_EDX];
        (*regs).gp[REG_ECX] = (*mc).gregs[REG_ECX];
        (*regs).gp[REG_EAX] = (*mc).gregs[REG_EAX];
        (*regs).gp[REG_EIP] = (*mc).gregs[REG_EIP];
        (*regs).gp[REG_CS] = ((*mc).gregs[REG_CS] & 0xffff) | 3;
        (*regs).gp[REG_EFL] = (*mc).gregs[REG_EFL];
        (*regs).gp[REG_SS] = ((*mc).gregs[REG_SS] & 0xffff) | 3;
    }
    #[cfg(not(target_arch = "x86"))]
    {
        (*regs).gp[R8 / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_R8];
        (*regs).gp[R9 / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_R9];
        (*regs).gp[R10 / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_R10];
        (*regs).gp[R11 / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_R11];
        (*regs).gp[R12 / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_R12];
        (*regs).gp[R13 / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_R13];
        (*regs).gp[R14 / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_R14];
        (*regs).gp[R15 / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_R15];
        (*regs).gp[RDI / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_RDI];
        (*regs).gp[RSI / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_RSI];
        (*regs).gp[RBP / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_RBP];
        (*regs).gp[RBX / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_RBX];
        (*regs).gp[RDX / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_RDX];
        (*regs).gp[RAX / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_RAX];
        (*regs).gp[RCX / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_RCX];
        (*regs).gp[RSP / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_RSP];
        (*regs).gp[RIP / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_RIP];
        (*regs).gp[EFLAGS / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_EFL];
        (*regs).gp[CS / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_CSGSFS];
        (*regs).gp[SS / core::mem::size_of::<c_ulong>()] = (*mc).gregs[REG_CSGSFS] >> 48;
    }
}

pub unsafe fn mc_set_rip(_mc: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void) {
    let mc = _mc as *mut mcontext_t;
    #[cfg(target_arch = "x86")]
    { (*mc).gregs[REG_EIP] = target as c_ulong; }
    #[cfg(not(target_arch = "x86"))]
    { (*mc).gregs[REG_RIP] = target as c_ulong; }
}

pub unsafe fn get_mc_from_regs(regs: *mut uml_pt_regs, mc: *mut mcontext_t, single_stepping: c_int) {
    #[cfg(target_arch = "x86")]
    {
        (*mc).gregs[REG_GS] = (*regs).gp[REG_GS] & 0xffff;
        (*mc).gregs[REG_FS] = (*regs).gp[REG_FS] & 0xffff;
        (*mc).gregs[REG_ES] = (*regs).gp[REG_ES] & 0xffff;
        (*mc).gregs[REG_DS] = (*regs).gp[REG_DS] & 0xffff;
        (*mc).gregs[REG_EDI] = (*regs).gp[REG_EDI]; (*mc).gregs[REG_ESI] = (*regs).gp[REG_ESI];
        (*mc).gregs[REG_EBP] = (*regs).gp[REG_EBP]; (*mc).gregs[REG_ESP] = (*regs).gp[REG_UESP];
        (*mc).gregs[REG_EBX] = (*regs).gp[REG_EBX]; (*mc).gregs[REG_EDX] = (*regs).gp[REG_EDX];
        (*mc).gregs[REG_ECX] = (*regs).gp[REG_ECX]; (*mc).gregs[REG_EAX] = (*regs).gp[REG_EAX];
        (*mc).gregs[REG_EIP] = (*regs).gp[REG_EIP];
        (*mc).gregs[REG_CS] = ((*regs).gp[REG_CS] & 0xffff) | 3;
        (*mc).gregs[REG_EFL] = (*regs).gp[REG_EFL]; (*mc).gregs[REG_SS] = ((*regs).gp[REG_SS] & 0xffff) | 3;
    }
    #[cfg(not(target_arch = "x86"))]
    {
        (*mc).gregs[REG_R8] = (*regs).gp[R8 / core::mem::size_of::<c_ulong>()]; (*mc).gregs[REG_R9] = (*regs).gp[R9 / core::mem::size_of::<c_ulong>()];
        (*mc).gregs[REG_R10] = (*regs).gp[R10 / core::mem::size_of::<c_ulong>()]; (*mc).gregs[REG_R11] = (*regs).gp[R11 / core::mem::size_of::<c_ulong>()];
        (*mc).gregs[REG_R12] = (*regs).gp[R12 / core::mem::size_of::<c_ulong>()]; (*mc).gregs[REG_R13] = (*regs).gp[R13 / core::mem::size_of::<c_ulong>()];
        (*mc).gregs[REG_R14] = (*regs).gp[R14 / core::mem::size_of::<c_ulong>()]; (*mc).gregs[REG_R15] = (*regs).gp[R15 / core::mem::size_of::<c_ulong>()];
        (*mc).gregs[REG_RDI] = (*regs).gp[RDI / core::mem::size_of::<c_ulong>()]; (*mc).gregs[REG_RSI] = (*regs).gp[RSI / core::mem::size_of::<c_ulong>()];
        (*mc).gregs[REG_RBP] = (*regs).gp[RBP / core::mem::size_of::<c_ulong>()]; (*mc).gregs[REG_RBX] = (*regs).gp[RBX / core::mem::size_of::<c_ulong>()];
        (*mc).gregs[REG_RDX] = (*regs).gp[RDX / core::mem::size_of::<c_ulong>()]; (*mc).gregs[REG_RAX] = (*regs).gp[RAX / core::mem::size_of::<c_ulong>()];
        (*mc).gregs[REG_RCX] = (*regs).gp[RCX / core::mem::size_of::<c_ulong>()]; (*mc).gregs[REG_RSP] = (*regs).gp[RSP / core::mem::size_of::<c_ulong>()];
        (*mc).gregs[REG_RIP] = (*regs).gp[RIP / core::mem::size_of::<c_ulong>()]; (*mc).gregs[REG_EFL] = (*regs).gp[EFLAGS / core::mem::size_of::<c_ulong>()];
        (*mc).gregs[REG_CSGSFS] &= 0xffffffffffff; (*mc).gregs[REG_CSGSFS] |= ((*regs).gp[SS / core::mem::size_of::<c_ulong>()] & 0xffff) << 48;
    }
    if single_stepping != 0 { (*mc).gregs[REG_EFL] |= X86_EFLAGS_TF; } else { (*mc).gregs[REG_EFL] &= !X86_EFLAGS_TF; }
}

// The remaining declarations and state-transfer routines retain the C ABI and
// depend on the surrounding architecture definitions.
#[repr(C)] pub struct _xstate_64 { pub fpstate: _fpstate_64, pub xstate_hdr: _header, pub ymmh: _ymmh_state }
pub struct task_struct;
pub struct user_regset;
#[repr(C)] pub struct membuf { pub p: *mut c_void, pub left: usize }

unsafe fn get_fpstate(data: *mut stub_data, mcontext: *mut mcontext_t, fp_size: *mut c_int) -> *mut _fpstate {
    let res = ((*mcontext).fpregs as usize & (UM_KERN_PAGE_SIZE - 1)) + (&(*data).sigstack[0] as *const _ as usize) as usize;
    let res = res as *mut _fpstate;
    if (res as usize + core::mem::size_of::<_fpstate>()) >
        (&(*data).sigstack as *const _ as usize + core::mem::size_of_val(&(*data).sigstack)) { return core::ptr::null_mut(); }
    if (*res).sw_reserved.magic1 != FP_XSTATE_MAGIC1 {
        *fp_size = core::mem::size_of::<_fpstate>() as c_int;
    } else {
        let magic2_addr = res as usize + (*res).sw_reserved.extended_size as usize - FP_XSTATE_MAGIC2_SIZE;
        if magic2_addr > (&(*data).sigstack as *const _ as usize + core::mem::size_of_val(&(*data).sigstack)) { return core::ptr::null_mut(); }
        if *(magic2_addr as *const u32) != FP_XSTATE_MAGIC2 { return core::ptr::null_mut(); }
        *fp_size = (*res).sw_reserved.extended_size as c_int - FP_XSTATE_MAGIC2_SIZE as c_int;
    }
    res
}

pub unsafe fn get_stub_state(regs: *mut uml_pt_regs, data: *mut stub_data, fp_size_out: *mut c_ulong) -> c_int {
    let mcontext = (&mut (*data).sigstack[(*data).mctx_offset as usize]) as *mut _ as *mut mcontext_t;
    get_regs_from_mc(regs, mcontext);
    let mut fp_size = 0; let fpstate_stub = get_fpstate(data, mcontext, &mut fp_size);
    if fpstate_stub.is_null() { return -EINVAL; }
    #[cfg(target_arch = "x86")] let xstate_stub = &mut (*fpstate_stub)._fxsr_env as *mut _ as *mut _xstate_64;
    #[cfg(not(target_arch = "x86"))] let xstate_stub = fpstate_stub as *mut _xstate_64;
    #[cfg(target_arch = "x86")] let xstate_size = fp_size - (core::mem::offset_of!(_fpstate_32, _fxsr_env) as c_int);
    #[cfg(not(target_arch = "x86"))] let xstate_size = fp_size;
    if !fp_size_out.is_null() { *fp_size_out = xstate_size as c_ulong; }
    if xstate_size > host_fp_size { return -ENOSPC; }
    core::ptr::copy_nonoverlapping(xstate_stub as *const u8, (*regs).fp.as_mut_ptr() as *mut u8, xstate_size as usize);
    #[cfg(target_arch = "x86")] if um_fxsr_from_i387(&mut (*regs).fp as *mut _ as *mut _fpstate_64, fpstate_stub as *const _ as *const _fpstate_32) != 0 { return -EINVAL; }
    0
}

pub unsafe fn set_stub_state(regs: *mut uml_pt_regs, data: *mut stub_data, single_stepping: c_int) -> c_int {
    let mcontext = (&mut (*data).sigstack[(*data).mctx_offset as usize]) as *mut _ as *mut mcontext_t;
    if mcontext as usize < &(*data).sigstack as *const _ as usize || mcontext as usize > (&(*data).sigstack as *const _ as usize + core::mem::size_of_val(&(*data).sigstack) - core::mem::size_of::<mcontext_t>()) { return -EINVAL; }
    get_mc_from_regs(regs, mcontext, single_stepping);
    let mut fp_size = 0; let fpstate_stub = get_fpstate(data, mcontext, &mut fp_size);
    if fpstate_stub.is_null() { return -EINVAL; }
    #[cfg(target_arch = "x86")] let xstate_stub = &mut (*fpstate_stub)._fxsr_env as *mut _ as *mut u8;
    #[cfg(not(target_arch = "x86"))] let xstate_stub = fpstate_stub as *mut u8;
    #[cfg(target_arch = "x86")] let xstate_size = fp_size - (core::mem::offset_of!(_fpstate_32, _fxsr_env) as c_int);
    #[cfg(not(target_arch = "x86"))] let xstate_size = fp_size;
    core::ptr::copy_nonoverlapping((*regs).fp.as_ptr() as *const u8, xstate_stub, xstate_size as usize);
    #[cfg(target_arch = "x86")] if um_i387_from_fxsr(fpstate_stub as *mut _fpstate_32, &(*regs).fp as *const _ as *const _fpstate_64) != 0 { return -EINVAL; }
    #[cfg(not(target_arch = "x86"))]
    {
        if (*data).arch_data.fs_base != (*regs).gp[FS_BASE / core::mem::size_of::<c_ulong>()] {
            (*data).arch_data.fs_base = (*regs).gp[FS_BASE / core::mem::size_of::<c_ulong>()];
            (*data).arch_data.sync |= STUB_SYNC_FS_BASE;
        }
        if (*data).arch_data.gs_base != (*regs).gp[GS_BASE / core::mem::size_of::<c_ulong>()] {
            (*data).arch_data.gs_base = (*regs).gp[GS_BASE / core::mem::size_of::<c_ulong>()];
            (*data).arch_data.sync |= STUB_SYNC_GS_BASE;
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
