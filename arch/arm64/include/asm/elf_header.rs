/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2012 ARM Ltd. */

// Dependency intent: asm/hwcap.h, asm/ptrace.h, asm/user.h, uapi/linux/elf.h,
// linux/bug.h, linux/errno.h, linux/fs.h, linux/types.h, and asm/processor.h.

/* AArch64 static relocation types. */
pub const R_ARM_NONE: i32 = 0;
pub const R_AARCH64_NONE: i32 = 256;
pub const R_AARCH64_ABS64: i32 = 257;
pub const R_AARCH64_ABS32: i32 = 258;
pub const R_AARCH64_ABS16: i32 = 259;
pub const R_AARCH64_PREL64: i32 = 260;
pub const R_AARCH64_PREL32: i32 = 261;
pub const R_AARCH64_PREL16: i32 = 262;
pub const R_AARCH64_MOVW_UABS_G0: i32 = 263;
pub const R_AARCH64_MOVW_UABS_G0_NC: i32 = 264;
pub const R_AARCH64_MOVW_UABS_G1: i32 = 265;
pub const R_AARCH64_MOVW_UABS_G1_NC: i32 = 266;
pub const R_AARCH64_MOVW_UABS_G2: i32 = 267;
pub const R_AARCH64_MOVW_UABS_G2_NC: i32 = 268;
pub const R_AARCH64_MOVW_UABS_G3: i32 = 269;
pub const R_AARCH64_MOVW_SABS_G0: i32 = 270;
pub const R_AARCH64_MOVW_SABS_G1: i32 = 271;
pub const R_AARCH64_MOVW_SABS_G2: i32 = 272;
pub const R_AARCH64_LD_PREL_LO19: i32 = 273;
pub const R_AARCH64_ADR_PREL_LO21: i32 = 274;
pub const R_AARCH64_ADR_PREL_PG_HI21: i32 = 275;
pub const R_AARCH64_ADR_PREL_PG_HI21_NC: i32 = 276;
pub const R_AARCH64_ADD_ABS_LO12_NC: i32 = 277;
pub const R_AARCH64_LDST8_ABS_LO12_NC: i32 = 278;
pub const R_AARCH64_TSTBR14: i32 = 279;
pub const R_AARCH64_CONDBR19: i32 = 280;
pub const R_AARCH64_JUMP26: i32 = 282;
pub const R_AARCH64_CALL26: i32 = 283;
pub const R_AARCH64_LDST16_ABS_LO12_NC: i32 = 284;
pub const R_AARCH64_LDST32_ABS_LO12_NC: i32 = 285;
pub const R_AARCH64_LDST64_ABS_LO12_NC: i32 = 286;
pub const R_AARCH64_MOVW_PREL_G0: i32 = 287;
pub const R_AARCH64_MOVW_PREL_G0_NC: i32 = 288;
pub const R_AARCH64_MOVW_PREL_G1: i32 = 289;
pub const R_AARCH64_MOVW_PREL_G1_NC: i32 = 290;
pub const R_AARCH64_MOVW_PREL_G2: i32 = 291;
pub const R_AARCH64_MOVW_PREL_G2_NC: i32 = 292;
pub const R_AARCH64_MOVW_PREL_G3: i32 = 293;
pub const R_AARCH64_LDST128_ABS_LO12_NC: i32 = 299;
pub const R_AARCH64_RELATIVE: i32 = 1027;

pub const ELF_CLASS: _ = ELFCLASS64;
// __AARCH64EB__ selects ELFDATA2MSB; otherwise ELFDATA2LSB.
pub const ELF_DATA: _ = ELFDATA2LSB;
pub const ELF_ARCH: _ = EM_AARCH64;
pub const ELF_PLATFORM_SIZE: usize = 16;
// __AARCH64EB__: "aarch64_be", otherwise "aarch64".
pub const ELF_PLATFORM: &str = "aarch64";

pub unsafe fn elf_check_arch(x: *const elf64_hdr) -> bool {
    (*x).e_machine == EM_AARCH64
}

pub unsafe fn compat_elf_read_implies_exec(_ex: *mut core::ffi::c_void, stk: i32) -> bool {
    stk == EXSTACK_DEFAULT
}

pub const CORE_DUMP_USE_REGSET: bool = true;
pub const ELF_EXEC_PAGESIZE: _ = PAGE_SIZE;
// CONFIG_ARM64_FORCE_52BIT selects 2 * TASK_SIZE_64 / 3; otherwise
// 2 * DEFAULT_MAP_WINDOW_64 / 3.
pub const ELF_ET_DYN_BASE: _ = 2 * DEFAULT_MAP_WINDOW_64 / 3;

pub type elf_greg_t = core::ffi::c_ulong;
pub const ELF_NGREG: usize = core::mem::size_of::<user_pt_regs>() / core::mem::size_of::<elf_greg_t>();
pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];
pub type elf_fpregset_t = user_fpsimd_state;

pub unsafe fn elf_plat_init(r: *mut user_pt_regs, _load_addr: usize) {
    (*r).regs[0] = 0;
}

// SET_PERSONALITY(ex): clear_thread_flag(TIF_32BIT); current->personality &= !READ_IMPLIES_EXEC.
// ARCH_DLINFO emits AT_SYSINFO_EHDR and AT_MINSIGSTKSZ (or AT_IGNORE).
pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: bool = true;
#[repr(C)]
pub struct linux_binprm;
unsafe extern "C" {
    pub fn arch_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: i32) -> i32;
}

// CONFIG_COMPAT selects test_thread_flag(TIF_32BIT) ? 0x7ff : 0x3ffff;
pub const STACK_RND_MASK: _ = 0x3ffff >> (PAGE_SHIFT - 12);
// __AARCH64EB__: "v8b", otherwise "v8l".
pub const COMPAT_ELF_PLATFORM: &str = "v8l";
pub const COMPAT_ELF_NGREG: usize = 18;
pub type compat_elf_greg_t = core::ffi::c_uint;
pub type compat_elf_gregset_t = [compat_elf_greg_t; COMPAT_ELF_NGREG];

// CONFIG_COMPAT
pub const COMPAT_ELF_ET_DYN_BASE: u64 = 0x000400000;
pub const EF_ARM_EABI_MASK: u32 = 0xff000000;
unsafe extern "C" {
    pub fn compat_elf_check_arch(hdr: *const elf32_hdr) -> i32;
    pub fn compat_start_thread();
    pub fn aarch32_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: i32) -> i32;
}
// CONFIG_COMPAT_VDSO selects COMPAT_ARCH_DLINFO emitting AT_SYSINFO_EHDR.

#[repr(C)]
pub struct arch_elf_state {
    pub flags: i32,
}

pub const ARM64_ELF_BTI: i32 = 1 << 0;
pub const INIT_ARCH_ELF_STATE: arch_elf_state = arch_elf_state { flags: 0 };

pub unsafe fn arch_parse_elf_property(
    type_: u32,
    data: *const core::ffi::c_void,
    datasz: usize,
    compat: bool,
    arch: *mut arch_elf_state,
) -> i32 {
    // CONFIG_COMPAT: if enabled and compat, return 0.
    if compat { return 0; }
    if type_ == GNU_PROPERTY_AARCH64_FEATURE_1_AND {
        if datasz != core::mem::size_of::<u32>() { return -ENOEXEC; }
        let p = data as *const u32;
        if system_supports_bti() && (*p & GNU_PROPERTY_AARCH64_FEATURE_1_BTI) != 0 {
            (*arch).flags |= ARM64_ELF_BTI;
        }
    }
    0
}

pub unsafe fn arch_elf_pt_proc(
    _ehdr: *mut core::ffi::c_void, _phdr: *mut core::ffi::c_void,
    _f: *mut file, _is_interp: bool, _state: *mut arch_elf_state,
) -> i32 { 0 }

pub unsafe fn arch_check_elf(
    _ehdr: *mut core::ffi::c_void, _has_interp: bool,
    _interp_ehdr: *mut core::ffi::c_void, _state: *mut arch_elf_state,
) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
