/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: <linux/types.h>, <asm/elf.h>, <uapi/linux/elf.h>

/* Executables for which elf_read_implies_exec() returns TRUE will have the
 * READ_IMPLIES_EXEC personality flag set automatically. */
#[inline]
pub fn elf_read_implies_exec<T>(_ex: *const T, _have_pt_gnu_stack: i32) -> i32 {
    0
}

#[macro_export]
macro_rules! SET_PERSONALITY {
    ($ex:expr) => {
        set_personality(PER_LINUX | ((*current).personality & (!PER_MASK)))
    };
}

#[macro_export]
macro_rules! SET_PERSONALITY2 {
    ($ex:expr, $state:expr) => {
        SET_PERSONALITY!($ex)
    };
}

#[macro_export]
macro_rules! START_THREAD {
    ($elf_ex:expr, $regs:expr, $elf_entry:expr, $start_stack:expr) => {
        start_thread($regs, $elf_entry, $start_stack)
    };
}

// Preserved conditional intent from ARCH_HAS_SETUP_ADDITIONAL_PAGES /
// ARCH_SETUP_ADDITIONAL_PAGES.
#[macro_export]
macro_rules! ARCH_SETUP_ADDITIONAL_PAGES {
    ($bprm:expr, $ex:expr, $interpreter:expr) => {
        arch_setup_additional_pages($bprm, $interpreter)
    };
}

pub const ELF32_GNU_PROPERTY_ALIGN: usize = 4;
pub const ELF64_GNU_PROPERTY_ALIGN: usize = 8;

// ELF_CLASS == ELFCLASS32 selects the 32-bit aliases; otherwise the 64-bit aliases.
#[cfg(feature = "ELFCLASS32")]
extern "C" {
    pub static mut _DYNAMIC: [Elf32_Dyn; 0];
}

#[cfg(feature = "ELFCLASS32")]
pub type Elfhdr = elf32_hdr;
#[cfg(feature = "ELFCLASS32")]
pub type ElfPhdr = elf32_phdr;
#[cfg(feature = "ELFCLASS32")]
pub type ElfShdr = elf32_shdr;
#[cfg(feature = "ELFCLASS32")]
pub type ElfNote = elf32_note;
#[cfg(feature = "ELFCLASS32")]
pub type ElfAddrT = Elf32_Off;
#[cfg(feature = "ELFCLASS32")]
pub type Elf_Half = Elf32_Half;
#[cfg(feature = "ELFCLASS32")]
pub type Elf_Word = Elf32_Word;
#[cfg(feature = "ELFCLASS32")]
pub const ELF_GNU_PROPERTY_ALIGN: usize = ELF32_GNU_PROPERTY_ALIGN;

#[cfg(not(feature = "ELFCLASS32"))]
extern "C" {
    pub static mut _DYNAMIC: [Elf64_Dyn; 0];
}

#[cfg(not(feature = "ELFCLASS32"))]
pub type Elfhdr = elf64_hdr;
#[cfg(not(feature = "ELFCLASS32"))]
pub type ElfPhdr = elf64_phdr;
#[cfg(not(feature = "ELFCLASS32"))]
pub type ElfShdr = elf64_shdr;
#[cfg(not(feature = "ELFCLASS32"))]
pub type ElfNote = elf64_note;
#[cfg(not(feature = "ELFCLASS32"))]
pub type ElfAddrT = Elf64_Off;
#[cfg(not(feature = "ELFCLASS32"))]
pub type Elf_Half = Elf64_Half;
#[cfg(not(feature = "ELFCLASS32"))]
pub type Elf_Word = Elf64_Word;
#[cfg(not(feature = "ELFCLASS32"))]
pub const ELF_GNU_PROPERTY_ALIGN: usize = ELF64_GNU_PROPERTY_ALIGN;

/* Optional callbacks to write extra ELF notes. */
pub struct file {
    _private: [u8; 0],
}
pub struct coredump_params {
    _private: [u8; 0],
}

// CONFIG_ARCH_HAVE_EXTRA_ELF_NOTES controls whether these are local stubs or externals.
#[cfg(not(feature = "CONFIG_ARCH_HAVE_EXTRA_ELF_NOTES"))]
#[inline]
pub fn elf_coredump_extra_notes_size() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_ARCH_HAVE_EXTRA_ELF_NOTES"))]
#[inline]
pub fn elf_coredump_extra_notes_write(_cprm: *mut coredump_params) -> i32 { 0 }
#[cfg(feature = "CONFIG_ARCH_HAVE_EXTRA_ELF_NOTES")]
extern "C" {
    pub fn elf_coredump_extra_notes_size() -> i32;
    pub fn elf_coredump_extra_notes_write(cprm: *mut coredump_params) -> i32;
}

/* NT_GNU_PROPERTY_TYPE_0 header:
 * Keep this internal until/unless there is an agreed UAPI definition.
 * pr_type values (GNU_PROPERTY_*) are public and defined in the UAPI header.
 */
#[repr(C)]
pub struct gnu_property {
    pub pr_type: u32,
    pub pr_datasz: u32,
}

pub struct arch_elf_state {
    _private: [u8; 0],
}

// CONFIG_ARCH_USE_GNU_PROPERTY controls whether this parser is a local stub or external.
#[cfg(not(feature = "CONFIG_ARCH_USE_GNU_PROPERTY"))]
#[inline]
pub fn arch_parse_elf_property(
    _type_: u32,
    _data: *const core::ffi::c_void,
    _datasz: usize,
    _compat: bool,
    _arch: *mut arch_elf_state,
) -> i32 { 0 }
#[cfg(feature = "CONFIG_ARCH_USE_GNU_PROPERTY")]
extern "C" {
    pub fn arch_parse_elf_property(
        type_: u32,
        data: *const core::ffi::c_void,
        datasz: usize,
        compat: bool,
        arch: *mut arch_elf_state,
    ) -> i32;
}

// CONFIG_ARCH_HAVE_ELF_PROT controls whether this is an external architecture hook.
#[cfg(not(feature = "CONFIG_ARCH_HAVE_ELF_PROT"))]
#[inline]
pub fn arch_elf_adjust_prot(
    prot: i32,
    _state: *const arch_elf_state,
    _has_interp: bool,
    _is_interp: bool,
) -> i32 { prot }
#[cfg(feature = "CONFIG_ARCH_HAVE_ELF_PROT")]
extern "C" {
    pub fn arch_elf_adjust_prot(
        prot: i32,
        state: *const arch_elf_state,
        has_interp: bool,
        is_interp: bool,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
