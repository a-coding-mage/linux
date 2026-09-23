// SPDX-License-Identifier: GPL-2.0
//! Options, symbol classes, and x86 ELF diagnostic names.

#[derive(Default)]
pub(crate) struct Options {
    pub(crate) absolute_symbols: bool,
    pub(crate) absolute_relocations: bool,
    pub(crate) relocation_info: bool,
    pub(crate) text: bool,
    pub(crate) real_mode: bool,
}

#[derive(Clone, Copy)]
pub(crate) enum Width {
    Bits16,
    Bits32,
    Bits64,
}

pub(crate) enum SymbolClass {
    Absolute,
    Relative,
    Segment,
    Linear,
}

// These are the exact languages of the audited POSIX expressions in relocs.c.
// Operating on bytes also preserves matching for non-UTF-8 ELF symbol names.
pub(crate) fn matches(class: SymbolClass, name: &[u8], is_64: bool, real: bool) -> bool {
    use SymbolClass::*;
    if real {
        return match class {
            Absolute => false,
            Relative | Linear => name.starts_with(b"pa_"),
            Segment => name == b"real_mode_seg",
        };
    }
    match class {
        Absolute => {
            matches!(
                name,
                b"xen_irq_disable_direct_reloc" | b"xen_save_fl_direct_reloc"
            ) || name.starts_with(b"VDSO")
                || name.starts_with(b"__kcfi_typeid_")
                || name.starts_with(b"__crc_")
                || (name.starts_with(b"xen_elfnote_")
                    && name.ends_with(b"_offset")
                    && name.len() > b"xen_elfnote__offset".len())
        }
        Relative => {
            matches!(
                name,
                b"__init_begin"
                    | b"__init_end"
                    | b"__x86_cpu_dev_start"
                    | b"__x86_cpu_dev_end"
                    | b"__alt_instructions"
                    | b"__alt_instructions_end"
                    | b"__iommu_table"
                    | b"__iommu_table_end"
                    | b"__apicdrivers"
                    | b"__apicdrivers_end"
                    | b"__start___ksymtab"
                    | b"__stop___ksymtab"
                    | b"__start___ksymtab_gpl"
                    | b"__stop___ksymtab_gpl"
                    | b"__start___kcrctab"
                    | b"__stop___kcrctab"
                    | b"__start___kcrctab_gpl"
                    | b"__stop___kcrctab_gpl"
                    | b"__start___param"
                    | b"__stop___param"
                    | b"__start___modver"
                    | b"__stop___modver"
                    | b"__start___bug_table"
                    | b"__stop___bug_table"
                    | b"__tracedata_start"
                    | b"__tracedata_end"
                    | b"__start_notes"
                    | b"__stop_notes"
                    | b"__end_rodata"
                    | b"__end_rodata_aligned"
                    | b"__initramfs_start"
                    | b"jiffies"
                    | b"jiffies_64"
                    | b"_end"
            ) || name.starts_with(b"__start_pci_")
                || name.starts_with(b"__end_pci_")
                || (is_64 && name == b"__end_rodata_hpage_align")
                || (cfg!(CONFIG_FW_LOADER)
                    && matches!(name, b"__start_builtin_fw" | b"__end_builtin_fw"))
        }
        Segment | Linear => false,
    }
}

pub(crate) fn symbol_type(kind: u8) -> &'static str {
    match kind {
        0 => "STT_NOTYPE",
        1 => "STT_OBJECT",
        2 => "STT_FUNC",
        3 => "STT_SECTION",
        4 => "STT_FILE",
        5 => "STT_COMMON",
        6 => "STT_TLS",
        _ => "unknown sym type name",
    }
}

pub(crate) fn symbol_binding(binding: u8) -> &'static str {
    match binding {
        0 => "STB_LOCAL",
        1 => "STB_GLOBAL",
        2 => "STB_WEAK",
        _ => "unknown sym bind name",
    }
}

pub(crate) fn visibility(value: u8) -> &'static str {
    match value {
        0 => "STV_DEFAULT",
        1 => "STV_INTERNAL",
        2 => "STV_HIDDEN",
        _ => "STV_PROTECTED",
    }
}

pub(crate) fn relocation_type(kind: u32, is_64: bool) -> &'static str {
    if is_64 {
        match kind {
            0 => "R_X86_64_NONE",
            1 => "R_X86_64_64",
            2 => "R_X86_64_PC32",
            3 => "R_X86_64_GOT32",
            4 => "R_X86_64_PLT32",
            5 => "R_X86_64_COPY",
            6 => "R_X86_64_GLOB_DAT",
            7 => "R_X86_64_JUMP_SLOT",
            8 => "R_X86_64_RELATIVE",
            9 => "R_X86_64_GOTPCREL",
            10 => "R_X86_64_32",
            11 => "R_X86_64_32S",
            12 => "R_X86_64_16",
            13 => "R_X86_64_PC16",
            14 => "R_X86_64_8",
            15 => "R_X86_64_PC8",
            24 => "R_X86_64_PC64",
            42 => "R_X86_64_REX_GOTPCRELX",
            _ => "unknown type rel type name",
        }
    } else {
        match kind {
            0 => "R_386_NONE",
            1 => "R_386_32",
            2 => "R_386_PC32",
            3 => "R_386_GOT32",
            4 => "R_386_PLT32",
            5 => "R_386_COPY",
            6 => "R_386_GLOB_DAT",
            7 => "R_386_JMP_SLOT",
            8 => "R_386_RELATIVE",
            9 => "R_386_GOTOFF",
            10 => "R_386_GOTPC",
            20 => "R_386_16",
            21 => "R_386_PC16",
            22 => "R_386_8",
            23 => "R_386_PC8",
            _ => "unknown type rel type name",
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
