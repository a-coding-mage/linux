/* SPDX-License-Identifier: GPL-2.0-or-later */

// C header guard: _ASM_POWERPC_MODULE_H
// This header is active only under __KERNEL__.
// Dependencies supplied by other translated headers:
// linux/list.h, asm/bug.h, asm-generic/module.h

#[cfg(not(target_arch = "powerpc64"))]
#[repr(C)]
pub struct ppc_plt_entry {
    /* 16 byte jump instruction sequence (4 instructions) */
    pub jump: [core::ffi::c_uint; 4],
}

#[repr(C)]
pub struct mod_arch_specific {
    #[cfg(target_arch = "powerpc64")]
    pub stubs_section: core::ffi::c_uint, /* Index of stubs section in module */
    #[cfg(target_arch = "powerpc64")]
    pub stub_count: core::ffi::c_uint, /* Number of stubs used */

    #[cfg(all(target_arch = "powerpc64", feature = "CONFIG_PPC_KERNEL_PCREL"))]
    pub got_section: core::ffi::c_uint, /* What section is the GOT? */
    #[cfg(all(target_arch = "powerpc64", feature = "CONFIG_PPC_KERNEL_PCREL"))]
    pub pcpu_section: core::ffi::c_uint, /* .data..percpu section */
    #[cfg(all(
        target_arch = "powerpc64",
        not(feature = "CONFIG_PPC_KERNEL_PCREL")
    ))]
    pub toc_section: core::ffi::c_uint, /* What section is the TOC? */
    #[cfg(all(
        target_arch = "powerpc64",
        not(feature = "CONFIG_PPC_KERNEL_PCREL")
    ))]
    pub toc_fixed: bool, /* Have we fixed up .TOC.? */

    #[cfg(all(target_arch = "powerpc64", feature = "CONFIG_PPC64_ELF_ABI_V1"))]
    /* For module function descriptor dereference */
    pub start_opd: core::ffi::c_ulong,
    #[cfg(all(target_arch = "powerpc64", feature = "CONFIG_PPC64_ELF_ABI_V1"))]
    pub end_opd: core::ffi::c_ulong,

    #[cfg(not(target_arch = "powerpc64"))]
    /* Indices of PLT sections within module. */
    pub core_plt_section: core::ffi::c_uint,
    #[cfg(not(target_arch = "powerpc64"))]
    pub init_plt_section: core::ffi::c_uint,

    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    pub tramp: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
    pub tramp_regs: core::ffi::c_ulong,
    #[cfg(all(
        feature = "CONFIG_DYNAMIC_FTRACE",
        feature = "CONFIG_PPC_FTRACE_OUT_OF_LINE"
    ))]
    pub ool_stubs: *mut ftrace_ool_stub,
    #[cfg(all(
        feature = "CONFIG_DYNAMIC_FTRACE",
        feature = "CONFIG_PPC_FTRACE_OUT_OF_LINE"
    ))]
    pub ool_stub_count: core::ffi::c_uint,
    #[cfg(all(
        feature = "CONFIG_DYNAMIC_FTRACE",
        feature = "CONFIG_PPC_FTRACE_OUT_OF_LINE"
    ))]
    pub ool_stub_index: core::ffi::c_uint,
}

// The C asm directives create empty ELF sections for module_frob_arch_sections
// to expand; they have no direct Rust equivalent.

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
unsafe extern "C" {
    pub fn module_trampoline_target(
        modu: *mut module,
        trampoline: core::ffi::c_ulong,
        target: *mut core::ffi::c_ulong,
    ) -> core::ffi::c_int;
    pub fn module_finalize_ftrace(
        modu: *mut module,
        sechdrs: *const Elf_Shdr,
    ) -> core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE"))]
#[inline]
pub unsafe fn module_finalize_ftrace(
    _modu: *mut module,
    _sechdrs: *const Elf_Shdr,
) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
