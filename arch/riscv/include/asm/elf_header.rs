/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * Copyright (C) 2012 Regents of the University of California
 */

// C includes omitted; referenced symbols are supplied by other headers.

/* These are used to set parameters in the core dumps. */
pub const ELF_ARCH: _ = EM_RISCV;

// CONFIG_64BIT selects ELFCLASS64; otherwise ELFCLASS32.
#[cfg(CONFIG_64BIT)]
pub const ELF_CLASS: _ = ELFCLASS64;
#[cfg(not(CONFIG_64BIT))]
pub const ELF_CLASS: _ = ELFCLASS32;

pub const ELF_DATA: _ = ELFDATA2LSB;

/* This is used to ensure we don't load something for the wrong architecture. */
#[inline]
pub fn elf_check_arch(x: &Elf32_Ehdr) -> bool {
    x.e_machine == EM_RISCV && x.e_ident[EI_CLASS] == ELF_CLASS
}

extern "C" {
    pub fn compat_elf_check_arch(hdr: *mut Elf32_Ehdr) -> bool;
}

pub const CORE_DUMP_USE_REGSET: bool = true;
pub const ELF_FDPIC_CORE_EFLAGS: _ = 0;
pub const ELF_EXEC_PAGESIZE: _ = PAGE_SIZE;

/* This is the location that an ET_DYN program is loaded if exec'ed. */
pub const ELF_ET_DYN_BASE: _ = ((DEFAULT_MAP_WINDOW / 3) * 2);

// CONFIG_64BIT
#[cfg(CONFIG_64BIT)]
#[inline]
pub fn STACK_RND_MASK() -> _ {
    if is_compat_task() {
        0x7ff >> (PAGE_SHIFT - 12)
    } else {
        0x3ffff >> (PAGE_SHIFT - 12)
    }
}

/*
 * Provides information on the available set of ISA extensions to userspace,
 * via a bitmap that corresponds to each single-letter ISA extension.  This is
 * essentially defunct, but will remain for compatibility with userspace.
 */
pub const ELF_HWCAP: _ = riscv_get_elf_hwcap();
extern "C" {
    pub static mut elf_hwcap: ::core::ffi::c_ulong;
}

#[macro_export]
macro_rules! ELF_FDPIC_PLAT_INIT {
    ($r:expr, $exec_map_addr:expr, $interp_map_addr:expr, $dynamic_addr:expr) => {{
        $r.a1 = $exec_map_addr;
        $r.a2 = $interp_map_addr;
        $r.a3 = $dynamic_addr;
    }};
}

pub const ELF_PLATFORM: *const core::ffi::c_void = core::ptr::null();
pub const COMPAT_ELF_PLATFORM: *const core::ffi::c_void = core::ptr::null();

#[macro_export]
macro_rules! ARCH_DLINFO {
    () => {{
        NEW_AUX_ENT!(AT_SYSINFO_EHDR, (elf_addr_t)(ulong)current->mm->context.vdso);
        NEW_AUX_ENT!(AT_L1I_CACHESIZE, get_cache_size(1, CACHE_TYPE_INST));
        NEW_AUX_ENT!(AT_L1I_CACHEGEOMETRY, get_cache_geometry(1, CACHE_TYPE_INST));
        NEW_AUX_ENT!(AT_L1D_CACHESIZE, get_cache_size(1, CACHE_TYPE_DATA));
        NEW_AUX_ENT!(AT_L1D_CACHEGEOMETRY, get_cache_geometry(1, CACHE_TYPE_DATA));
        NEW_AUX_ENT!(AT_L2_CACHESIZE, get_cache_size(2, CACHE_TYPE_UNIFIED));
        NEW_AUX_ENT!(AT_L2_CACHEGEOMETRY, get_cache_geometry(2, CACHE_TYPE_UNIFIED));
        NEW_AUX_ENT!(AT_L3_CACHESIZE, get_cache_size(3, CACHE_TYPE_UNIFIED));
        NEW_AUX_ENT!(AT_L3_CACHEGEOMETRY, get_cache_geometry(3, CACHE_TYPE_UNIFIED));
        if likely(signal_minsigstksz) {
            NEW_AUX_ENT!(AT_MINSIGSTKSZ, signal_minsigstksz);
        } else {
            NEW_AUX_ENT!(AT_IGNORE, 0);
        }
    }};
}

// CONFIG_MMU
#[cfg(CONFIG_MMU)]
pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: bool = true;
#[cfg(CONFIG_MMU)]
#[repr(C)]
pub struct linux_binprm;
#[cfg(CONFIG_MMU)]
extern "C" {
    pub fn arch_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: i32) -> i32;
}

#[macro_export]
macro_rules! ELF_CORE_COPY_REGS {
    ($dest:expr, $regs:expr) => {{
        *(($dest as *mut user_regs_struct)) = *(($regs as *mut user_regs_struct));
    }};
}

// CONFIG_COMPAT
#[cfg(CONFIG_COMPAT)]
#[macro_export]
macro_rules! SET_PERSONALITY {
    ($ex:expr) => {{
        set_compat_task($ex.e_ident[EI_CLASS] == ELFCLASS32);
        if personality(current.personality) != PER_LINUX32 {
            set_personality(PER_LINUX | (current.personality & !PER_MASK));
        }
    }};
}

#[cfg(CONFIG_COMPAT)]
pub const COMPAT_ELF_ET_DYN_BASE: _ = ((TASK_SIZE_32 / 3) * 2);

/* rv32 registers */
#[cfg(CONFIG_COMPAT)]
pub type compat_elf_greg_t = compat_ulong_t;
#[cfg(CONFIG_COMPAT)]
pub type compat_elf_gregset_t = [compat_elf_greg_t; ELF_NGREG];

#[cfg(CONFIG_COMPAT)]
extern "C" {
    pub fn compat_arch_setup_additional_pages(
        bprm: *mut linux_binprm,
        uses_interp: i32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
