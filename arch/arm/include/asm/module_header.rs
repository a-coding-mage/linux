/* SPDX-License-Identifier: GPL-2.0 */

// Translated from asm-generic/module.h and asm/unwind.h dependencies.

#[cfg(CONFIG_ARM_UNWIND)]
pub const ELF_SECTION_UNWIND: u32 = 0x7000_0001;

pub const PLT_ENT_STRIDE: usize = L1_CACHE_BYTES;
pub const PLT_ENT_COUNT: usize = PLT_ENT_STRIDE / core::mem::size_of::<u32>();
pub const PLT_ENT_SIZE: usize = core::mem::size_of::<plt_entries>() / PLT_ENT_COUNT;

#[repr(C)]
pub struct plt_entries {
    pub ldr: [u32; PLT_ENT_COUNT],
    pub lit: [u32; PLT_ENT_COUNT],
}

#[repr(C)]
pub struct mod_plt_sec {
    pub plt: *mut elf32_shdr,
    pub plt_ent: *mut plt_entries,
    pub plt_count: i32,
}

#[repr(C)]
pub struct mod_arch_specific {
    #[cfg(CONFIG_ARM_UNWIND)]
    pub unwind_list: list_head,
    #[cfg(CONFIG_ARM_UNWIND)]
    pub init_table: *mut unwind_table,
    #[cfg(CONFIG_ARM_MODULE_PLTS)]
    pub core: mod_plt_sec,
    #[cfg(CONFIG_ARM_MODULE_PLTS)]
    pub init: mod_plt_sec,
}

pub struct module;

extern "C" {
    pub fn get_module_plt(
        mod_: *mut module,
        loc: c_ulong,
        val: Elf32_Addr,
    ) -> u32;
}

#[cfg(CONFIG_ARM_MODULE_PLTS)]
extern "C" {
    pub fn in_module_plt(loc: c_ulong) -> bool;
}

#[cfg(not(CONFIG_ARM_MODULE_PLTS))]
#[inline]
pub fn in_module_plt(_loc: c_ulong) -> bool {
    false
}

#[cfg(CONFIG_THUMB2_KERNEL)]
pub const HAVE_ARCH_KALLSYMS_SYMBOL_VALUE: bool = true;

#[cfg(CONFIG_THUMB2_KERNEL)]
#[inline]
pub unsafe fn kallsyms_symbol_value(sym: *const Elf_Sym) -> c_ulong {
    if ELF_ST_TYPE((*sym).st_info) == STT_FUNC {
        (*sym).st_value & !1
    } else {
        (*sym).st_value
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
