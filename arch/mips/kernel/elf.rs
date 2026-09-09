// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2014 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// External kernel and architecture declarations are supplied by other files.

#[cfg(CONFIG_MIPS_FP_SUPPORT)]
pub static mut mips_use_nan_legacy: bool = false;
#[cfg(CONFIG_MIPS_FP_SUPPORT)]
pub static mut mips_use_nan_2008: bool = false;

#[cfg(CONFIG_MIPS_FP_SUPPORT)]
const FP_FRE: i32 = 0;
#[cfg(CONFIG_MIPS_FP_SUPPORT)]
const FP_FR0: i32 = 1;
#[cfg(CONFIG_MIPS_FP_SUPPORT)]
const FP_FR1: i32 = 2;

#[cfg(CONFIG_MIPS_FP_SUPPORT)]
#[repr(C)]
struct mode_req {
    single: bool,
    soft: bool,
    fr1: bool,
    frdefault: bool,
    fre: bool,
}

#[cfg(CONFIG_MIPS_FP_SUPPORT)]
static fpu_reqs: [mode_req; 8] = [
    mode_req { single: true, soft: true, fr1: true, frdefault: true, fre: true },
    mode_req { single: false, soft: false, fr1: false, frdefault: true, fre: true },
    mode_req { single: true, soft: false, fr1: false, frdefault: false, fre: false },
    mode_req { single: false, soft: true, fr1: false, frdefault: false, fre: false },
    mode_req { single: false, soft: false, fr1: false, frdefault: false, fre: false },
    mode_req { single: false, soft: false, fr1: true, frdefault: true, fre: true },
    mode_req { single: false, soft: false, fr1: true, frdefault: false, fre: false },
    mode_req { single: false, soft: false, fr1: true, frdefault: false, fre: true },
];

#[cfg(CONFIG_MIPS_FP_SUPPORT)]
static mut none_req: mode_req = mode_req {
    single: true, soft: true, fr1: false, frdefault: true, fre: true,
};

#[cfg(CONFIG_MIPS_FP_SUPPORT)]
pub unsafe fn arch_elf_pt_proc(
    _ehdr: *mut core::ffi::c_void,
    _phdr: *mut core::ffi::c_void,
    elf: *mut file,
    is_interp: bool,
    state: *mut arch_elf_state,
) -> i32 {
    let ehdr = _ehdr as *mut elf_header_union;
    let phdr32 = _phdr as *mut elf32_phdr;
    let phdr64 = _phdr as *mut elf64_phdr;
    let mut abiflags: mips_elf_abiflags_v0 = core::mem::zeroed();
    let elf32 = (*ehdr).e32.e_ident[EI_CLASS] == ELFCLASS32;
    let flags = if elf32 { (*ehdr).e32.e_flags } else { (*ehdr).e64.e_flags };
    let pos: *mut loff_t;

    if elf32 {
        if flags & EF_MIPS_FP64 != 0 {
            if is_interp { (*state).interp_fp_abi = MIPS_ABI_FP_OLD_64; }
            else { (*state).fp_abi = MIPS_ABI_FP_OLD_64; }
        }
        if (*phdr32).p_type != PT_MIPS_ABIFLAGS { return 0; }
        if (*phdr32).p_filesz < core::mem::size_of::<mips_elf_abiflags_v0>() as _ { return -EINVAL; }
        pos = &mut (*phdr32).p_offset;
    } else {
        if (*phdr64).p_type != PT_MIPS_ABIFLAGS { return 0; }
        if (*phdr64).p_filesz < core::mem::size_of::<mips_elf_abiflags_v0>() as _ { return -EINVAL; }
        pos = &mut (*phdr64).p_offset;
    }
    let ret = kernel_read(elf, &mut abiflags as *mut _ as *mut core::ffi::c_void,
                          core::mem::size_of::<mips_elf_abiflags_v0>(), pos);
    if ret < 0 { return ret; }
    if ret != core::mem::size_of::<mips_elf_abiflags_v0>() as i32 { return -EIO; }
    if is_interp { (*state).interp_fp_abi = abiflags.fp_abi; }
    else { (*state).fp_abi = abiflags.fp_abi; }
    0
}

#[cfg(CONFIG_MIPS_FP_SUPPORT)]
pub unsafe fn arch_check_elf(
    _ehdr: *mut core::ffi::c_void, has_interpreter: bool,
    _interp_ehdr: *mut core::ffi::c_void, state: *mut arch_elf_state,
) -> i32 {
    let ehdr = _ehdr as *mut elf_header_union;
    let iehdr = _interp_ehdr as *mut elf_header_union;
    let elf32 = (*ehdr).e32.e_ident[EI_CLASS] == ELFCLASS32;
    let flags = if elf32 { (*ehdr).e32.e_flags } else { (*ehdr).e64.e_flags };
    if flags & EF_MIPS_NAN2008 != 0 {
        if mips_use_nan_2008 { (*state).nan_2008 = 1; } else { return -ENOEXEC; }
    } else if mips_use_nan_legacy { (*state).nan_2008 = 0; } else { return -ENOEXEC; }
    if has_interpreter {
        let ielf32 = (*iehdr).e32.e_ident[EI_CLASS] == ELFCLASS32;
        let iflags = if ielf32 { (*iehdr).e32.e_flags } else { (*iehdr).e64.e_flags };
        if (flags ^ iflags) & EF_MIPS_NAN2008 != 0 { return -ELIBBAD; }
    }
    if !IS_ENABLED_CONFIG_MIPS_O32_FP64_SUPPORT { return 0; }
    let fp_abi = (*state).fp_abi;
    let (abi0, abi1) = if has_interpreter {
        let interp_fp_abi = (*state).interp_fp_abi;
        (core::cmp::min(fp_abi, interp_fp_abi), core::cmp::max(fp_abi, interp_fp_abi))
    } else { (fp_abi, fp_abi) };
    let max_abi;
    if elf32 && flags & EF_MIPS_ABI2 == 0 {
        (*state).overall_fp_mode = if cpu_has_mips_r6 { FP_FRE } else { FP_FR0 };
        max_abi = MIPS_ABI_FP_64A;
    } else {
        (*state).overall_fp_mode = FP_FR1;
        max_abi = MIPS_ABI_FP_SOFT;
    }
    if (abi0 > max_abi && abi0 != MIPS_ABI_FP_UNKNOWN) || (abi1 > max_abi && abi1 != MIPS_ABI_FP_UNKNOWN) { return -ELIBBAD; }
    let mut prog_req = if abi0 == MIPS_ABI_FP_UNKNOWN { none_req } else { fpu_reqs[abi0 as usize] };
    let interp_req = if abi1 == MIPS_ABI_FP_UNKNOWN { none_req } else { fpu_reqs[abi1 as usize] };
    prog_req.single &= interp_req.single;
    prog_req.soft &= interp_req.soft;
    prog_req.fr1 &= interp_req.fr1;
    prog_req.frdefault &= interp_req.frdefault;
    prog_req.fre &= interp_req.fre;
    if prog_req.fre && !prog_req.frdefault && !prog_req.fr1 { (*state).overall_fp_mode = FP_FRE; }
    else if (prog_req.fr1 && prog_req.frdefault) || (prog_req.single && !prog_req.frdefault) {
        (*state).overall_fp_mode = if (raw_current_cpu_data.fpu_id & MIPS_FPIR_F64 != 0) && cpu_has_mips_r2_r6 { FP_FR1 } else { FP_FR0 };
    } else if prog_req.fr1 { (*state).overall_fp_mode = FP_FR1; }
    else if !prog_req.fre && !prog_req.frdefault && !prog_req.fr1 && !prog_req.single && !prog_req.soft { return -ELIBBAD; }
    0
}

#[cfg(CONFIG_MIPS_FP_SUPPORT)]
unsafe fn set_thread_fp_mode(hybrid: i32, regs32: i32) {
    if hybrid != 0 { set_thread_flag(TIF_HYBRID_FPREGS); } else { clear_thread_flag(TIF_HYBRID_FPREGS); }
    if regs32 != 0 { set_thread_flag(TIF_32BIT_FPREGS); } else { clear_thread_flag(TIF_32BIT_FPREGS); }
}

#[cfg(CONFIG_MIPS_FP_SUPPORT)]
pub unsafe fn mips_set_personality_fp(state: *mut arch_elf_state) {
    if !IS_ENABLED_CONFIG_MIPS_O32_FP64_SUPPORT { return; }
    match (*state).overall_fp_mode {
        FP_FRE => set_thread_fp_mode(1, 0),
        FP_FR0 => set_thread_fp_mode(0, 1),
        FP_FR1 => set_thread_fp_mode(0, 0),
        _ => BUG(),
    }
}

#[cfg(CONFIG_MIPS_FP_SUPPORT)]
pub unsafe fn mips_set_personality_nan(state: *mut arch_elf_state) {
    let c = &boot_cpu_data;
    let t = current;
    lose_fpu(0);
    (*t).thread.fpu.fcr31 = c.fpu_csr31;
    match (*state).nan_2008 {
        0 => {
            if c.fpu_msk31 & FPU_CSR_NAN2008 == 0 { (*t).thread.fpu.fcr31 &= !FPU_CSR_NAN2008; }
            if c.fpu_msk31 & FPU_CSR_ABS2008 == 0 { (*t).thread.fpu.fcr31 &= !FPU_CSR_ABS2008; }
        }
        1 => {
            if c.fpu_msk31 & FPU_CSR_NAN2008 == 0 { (*t).thread.fpu.fcr31 |= FPU_CSR_NAN2008; }
            if c.fpu_msk31 & FPU_CSR_ABS2008 == 0 { (*t).thread.fpu.fcr31 |= FPU_CSR_ABS2008; }
        }
        _ => BUG(),
    }
}

pub unsafe fn mips_elf_read_implies_exec(_elf_ex: *mut core::ffi::c_void, exstack: i32) -> i32 {
    if !cpu_has_rixi && exstack == EXSTACK_DEFAULT { 1 } else { 0 }
}

// EXPORT_SYMBOL(mips_elf_read_implies_exec);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
