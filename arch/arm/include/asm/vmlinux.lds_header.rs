// SPDX-License-Identifier: GPL-2.0
// Translated from <asm-generic/vmlinux.lds.h>.  These items retain the
// linker-script fragments as data because Rust has no native linker-script
// declaration syntax.

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
pub const ARM_CPU_DISCARD: &str = "";
#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
pub const ARM_CPU_DISCARD: &str = "x";
#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
pub const ARM_CPU_KEEP: &str = "x";
#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
pub const ARM_CPU_KEEP: &str = "";

#[cfg(any(
    all(feature = "CONFIG_SMP_ON_UP", not(feature = "CONFIG_DEBUG_SPINLOCK")),
    feature = "CONFIG_GENERIC_BUG",
    feature = "CONFIG_JUMP_LABEL"
))]
pub const ARM_EXIT_KEEP: &str = "x";
#[cfg(not(any(
    all(feature = "CONFIG_SMP_ON_UP", not(feature = "CONFIG_DEBUG_SPINLOCK")),
    feature = "CONFIG_GENERIC_BUG",
    feature = "CONFIG_JUMP_LABEL"
)))]
pub const ARM_EXIT_KEEP: &str = "";
#[cfg(any(
    all(feature = "CONFIG_SMP_ON_UP", not(feature = "CONFIG_DEBUG_SPINLOCK")),
    feature = "CONFIG_GENERIC_BUG",
    feature = "CONFIG_JUMP_LABEL"
))]
pub const ARM_EXIT_DISCARD: &str = "";
#[cfg(not(any(
    all(feature = "CONFIG_SMP_ON_UP", not(feature = "CONFIG_DEBUG_SPINLOCK")),
    feature = "CONFIG_GENERIC_BUG",
    feature = "CONFIG_JUMP_LABEL"
)))]
pub const ARM_EXIT_DISCARD: &str = "x";

#[cfg(feature = "CONFIG_MMU")]
pub const ARM_MMU_KEEP: &str = "KEEP(x)";
#[cfg(not(feature = "CONFIG_MMU"))]
pub const ARM_MMU_KEEP: &str = "";
#[cfg(feature = "CONFIG_MMU")]
pub const ARM_MMU_DISCARD: &str = "";
#[cfg(not(feature = "CONFIG_MMU"))]
pub const ARM_MMU_DISCARD: &str = "x";

// ld.lld does not support NOCROSSREFS:
// https://github.com/ClangBuiltLinux/linux/issues/1609
#[cfg(not(feature = "CONFIG_LD_IS_LLD"))]
pub const NOCROSSREFS: &str = "NOCROSSREFS";
#[cfg(feature = "CONFIG_LD_IS_LLD")]
pub const NOCROSSREFS: &str = "";

#[cfg(feature = "CONFIG_LD_CAN_USE_KEEP_IN_OVERLAY")]
pub const OVERLAY_KEEP: &str = "KEEP(x)";
#[cfg(not(feature = "CONFIG_LD_CAN_USE_KEEP_IN_OVERLAY"))]
pub const OVERLAY_KEEP: &str = "x";

// Set start/end symbol names to the LMA for the section.
#[macro_export]
macro_rules! ARM_LMA {
    ($sym:ident, $section:ident) => {
        concat!(stringify!($sym), "_start = LOADADDR(", stringify!($section), "); ",
                stringify!($sym), "_end = LOADADDR(", stringify!($section), ") + SIZEOF(",
                stringify!($section), ")")
    };
}

pub const PROC_INFO: &str = ". = ALIGN(4); __proc_info_begin = .; KEEP(*(.proc.info.init)) __proc_info_end = .;";
pub const IDMAP_TEXT: &str = "ALIGN_FUNCTION(); __idmap_text_start = .; *(.idmap.text) __idmap_text_end = .;";
pub const ARM_DISCARD: &str = "*(.ARM.exidx.exit.text) *(.ARM.extab.exit.text) *(.ARM.exidx.text.exit) *(.ARM.extab.text.exit) ARM_CPU_DISCARD(*(.ARM.exidx.cpuexit.text)) ARM_CPU_DISCARD(*(.ARM.extab.cpuexit.text)) ARM_EXIT_DISCARD(EXIT_TEXT) ARM_EXIT_DISCARD(EXIT_DATA) EXIT_CALL ARM_MMU_DISCARD(*(.text.fixup)) ARM_MMU_DISCARD(*(__ex_table)) COMMON_DISCARDS";
pub const ARM_ASSERTS: &str = ".plt : { *(.iplt) *(.rel.iplt) *(.iplt) *(.igot.plt) } ASSERT(SIZEOF(.plt) == 0, \"Unexpected run-time procedure linkages detected!\")";
pub const ARM_DETAILS: &str = "ELF_DETAILS .ARM.attributes 0 : { *(.ARM.attributes) }";
pub const ARM_STUBS_TEXT: &str = "*(.gnu.warning) *(.glue_7) *(.glue_7t) *(.vfp11_veneer) *(.v4_bx)";
pub const ARM_TEXT: &str = "IDMAP_TEXT __entry_text_start = .; *(.entry.text) __entry_text_end = .; IRQENTRY_TEXT SOFTIRQENTRY_TEXT TEXT_TEXT SCHED_TEXT LOCK_TEXT KPROBES_TEXT ARM_STUBS_TEXT . = ALIGN(4); *(.got) ARM_CPU_KEEP(PROC_INFO)";
pub const ARM_UNWIND_SECTIONS: &str = ". = ALIGN(8); .ARM.unwind_idx : { __start_unwind_idx = .; *(.ARM.exidx*) __stop_unwind_idx = .; } .ARM.unwind_tab : { __start_unwind_tab = .; *(.ARM.extab*) __stop_unwind_tab = .; }";
pub const ARM_VECTORS: &str = "__vectors_lma = .; OVERLAY 0xffff0000 : NOCROSSREFS AT(__vectors_lma) { .vectors { OVERLAY_KEEP(*(.vectors)) } .vectors.bhb.loop8 { OVERLAY_KEEP(*(.vectors.bhb.loop8)) } .vectors.bhb.bpiall { OVERLAY_KEEP(*(.vectors.bhb.bpiall)) } } ARM_LMA(__vectors, .vectors); ARM_LMA(__vectors_bhb_loop8, .vectors.bhb.loop8); ARM_LMA(__vectors_bhb_bpiall, .vectors.bhb.bpiall); . = __vectors_lma + SIZEOF(.vectors) + SIZEOF(.vectors.bhb.loop8) + SIZEOF(.vectors.bhb.bpiall); __stubs_lma = .; .stubs ADDR(.vectors) + 0x1000 : AT(__stubs_lma) { *(.stubs) } ARM_LMA(__stubs, .stubs); . = __stubs_lma + SIZEOF(.stubs); PROVIDE(vector_fiq_offset = vector_fiq - ADDR(.vectors));";
pub const ARM_TCM: &str = "__itcm_start = ALIGN(4); .text_itcm ITCM_OFFSET : AT(__itcm_start - LOAD_OFFSET) { __sitcm_text = .; *(.tcm.text) *(.tcm.rodata) . = ALIGN(4); __eitcm_text = .; } . = __itcm_start + SIZEOF(.text_itcm); __dtcm_start = .; .data_dtcm DTCM_OFFSET : AT(__dtcm_start - LOAD_OFFSET) { __sdtcm_data = .; *(.tcm.data) . = ALIGN(4); __edtcm_data = .; } . = __dtcm_start + SIZEOF(.data_dtcm);";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
