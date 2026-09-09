/* SPDX-License-Identifier: GPL-2.0-or-later */
// Rust translation of powerpc/include/asm/feature-fixups.h.
// The original items emit PowerPC assembler sections; these macros preserve
// their names and source-level intent for consumers that provide the assembler
// integration.

// CONFIG_PPC64 / __powerpc64__ select the assembler entry width in the C
// header.  The actual widths are build-time assembler properties.

macro_rules! START_FTR_SECTION { ($label:tt) => { concat!(stringify!($label), "1:") }; }
macro_rules! FTR_SECTION_ELSE_NESTED { ($label:tt) => { concat!(stringify!($label), "2: .pushsection __ftr_alt_", stringify!($label), "; .align 2; ", stringify!($label), "3:") }; }
macro_rules! CHECK_ALT_SIZE { ($else_size:expr, $body_size:expr) => { ($else_size, $body_size) }; }
macro_rules! MAKE_FTR_SECTION_ENTRY { ($msk:expr, $val:expr, $label:tt, $sect:tt) => { ($msk, $val, stringify!($label), stringify!($sect)) }; }

macro_rules! BEGIN_FTR_SECTION_NESTED { ($label:tt) => { START_FTR_SECTION!($label) }; }
macro_rules! BEGIN_FTR_SECTION { () => { START_FTR_SECTION!(97) }; }
macro_rules! END_FTR_SECTION_NESTED { ($msk:expr, $val:expr, $label:tt) => { (FTR_SECTION_ELSE_NESTED!($label), MAKE_FTR_SECTION_ENTRY!($msk, $val, $label, __ftr_fixup)) }; }
macro_rules! END_FTR_SECTION { ($msk:expr, $val:expr) => { END_FTR_SECTION_NESTED!($msk, $val, 97) }; }
macro_rules! END_FTR_SECTION_NESTED_IFSET { ($msk:expr, $label:tt) => { END_FTR_SECTION_NESTED!($msk, $msk, $label) }; }
macro_rules! END_FTR_SECTION_IFSET { ($msk:expr) => { END_FTR_SECTION!($msk, $msk) }; }
macro_rules! END_FTR_SECTION_IFCLR { ($msk:expr) => { END_FTR_SECTION!($msk, 0) }; }
macro_rules! FTR_SECTION_ELSE { () => { FTR_SECTION_ELSE_NESTED!(97) }; }
macro_rules! ALT_FTR_SECTION_END_NESTED { ($msk:expr, $val:expr, $label:tt) => { MAKE_FTR_SECTION_ENTRY!($msk, $val, $label, __ftr_fixup) }; }
macro_rules! ALT_FTR_SECTION_END_NESTED_IFSET { ($msk:expr, $label:tt) => { ALT_FTR_SECTION_END_NESTED!($msk, $msk, $label) }; }
macro_rules! ALT_FTR_SECTION_END_NESTED_IFCLR { ($msk:expr, $label:tt) => { ALT_FTR_SECTION_END_NESTED!($msk, 0, $label) }; }
macro_rules! ALT_FTR_SECTION_END { ($msk:expr, $val:expr) => { ALT_FTR_SECTION_END_NESTED!($msk, $val, 97) }; }
macro_rules! ALT_FTR_SECTION_END_IFSET { ($msk:expr) => { ALT_FTR_SECTION_END_NESTED_IFSET!($msk, 97) }; }
macro_rules! ALT_FTR_SECTION_END_IFCLR { ($msk:expr) => { ALT_FTR_SECTION_END_NESTED_IFCLR!($msk, 97) }; }

macro_rules! BEGIN_MMU_FTR_SECTION_NESTED { ($label:tt) => { START_FTR_SECTION!($label) }; }
macro_rules! BEGIN_MMU_FTR_SECTION { () => { START_FTR_SECTION!(97) }; }
macro_rules! END_MMU_FTR_SECTION_NESTED { ($msk:expr, $val:expr, $label:tt) => { (FTR_SECTION_ELSE_NESTED!($label), MAKE_FTR_SECTION_ENTRY!($msk, $val, $label, __mmu_ftr_fixup)) }; }
macro_rules! END_MMU_FTR_SECTION { ($msk:expr, $val:expr) => { END_MMU_FTR_SECTION_NESTED!($msk, $val, 97) }; }
macro_rules! END_MMU_FTR_SECTION_NESTED_IFSET { ($msk:expr, $label:tt) => { END_MMU_FTR_SECTION_NESTED!($msk, $msk, $label) }; }
macro_rules! END_MMU_FTR_SECTION_NESTED_IFCLR { ($msk:expr, $label:tt) => { END_MMU_FTR_SECTION_NESTED!($msk, 0, $label) }; }
macro_rules! END_MMU_FTR_SECTION_IFSET { ($msk:expr) => { END_MMU_FTR_SECTION!($msk, $msk) }; }
macro_rules! END_MMU_FTR_SECTION_IFCLR { ($msk:expr) => { END_MMU_FTR_SECTION!($msk, 0) }; }
macro_rules! MMU_FTR_SECTION_ELSE_NESTED { ($label:tt) => { FTR_SECTION_ELSE_NESTED!($label) }; }
macro_rules! MMU_FTR_SECTION_ELSE { () => { MMU_FTR_SECTION_ELSE_NESTED!(97) }; }
macro_rules! ALT_MMU_FTR_SECTION_END_NESTED { ($msk:expr, $val:expr, $label:tt) => { MAKE_FTR_SECTION_ENTRY!($msk, $val, $label, __mmu_ftr_fixup) }; }
macro_rules! ALT_MMU_FTR_SECTION_END_NESTED_IFSET { ($msk:expr, $label:tt) => { ALT_MMU_FTR_SECTION_END_NESTED!($msk, $msk, $label) }; }
macro_rules! ALT_MMU_FTR_SECTION_END_NESTED_IFCLR { ($msk:expr, $label:tt) => { ALT_MMU_FTR_SECTION_END_NESTED!($msk, 0, $label) }; }
macro_rules! ALT_MMU_FTR_SECTION_END { ($msk:expr, $val:expr) => { ALT_MMU_FTR_SECTION_END_NESTED!($msk, $val, 97) }; }
macro_rules! ALT_MMU_FTR_SECTION_END_IFSET { ($msk:expr) => { ALT_MMU_FTR_SECTION_END_NESTED_IFSET!($msk, 97) }; }
macro_rules! ALT_MMU_FTR_SECTION_END_IFCLR { ($msk:expr) => { ALT_MMU_FTR_SECTION_END_NESTED_IFCLR!($msk, 97) }; }

macro_rules! BEGIN_FW_FTR_SECTION_NESTED { ($label:tt) => { START_FTR_SECTION!($label) }; }
macro_rules! BEGIN_FW_FTR_SECTION { () => { START_FTR_SECTION!(97) }; }
macro_rules! END_FW_FTR_SECTION_NESTED { ($msk:expr, $val:expr, $label:tt) => { (FTR_SECTION_ELSE_NESTED!($label), MAKE_FTR_SECTION_ENTRY!($msk, $val, $label, __fw_ftr_fixup)) }; }
macro_rules! END_FW_FTR_SECTION { ($msk:expr, $val:expr) => { END_FW_FTR_SECTION_NESTED!($msk, $val, 97) }; }
macro_rules! END_FW_FTR_SECTION_IFSET { ($msk:expr) => { END_FW_FTR_SECTION!($msk, $msk) }; }
macro_rules! END_FW_FTR_SECTION_IFCLR { ($msk:expr) => { END_FW_FTR_SECTION!($msk, 0) }; }
macro_rules! FW_FTR_SECTION_ELSE_NESTED { ($label:tt) => { FTR_SECTION_ELSE_NESTED!($label) }; }
macro_rules! FW_FTR_SECTION_ELSE { () => { FTR_SECTION_ELSE_NESTED!(97) }; }
macro_rules! ALT_FW_FTR_SECTION_END_NESTED { ($msk:expr, $val:expr, $label:tt) => { MAKE_FTR_SECTION_ENTRY!($msk, $val, $label, __fw_ftr_fixup) }; }
macro_rules! ALT_FW_FTR_SECTION_END_NESTED_IFSET { ($msk:expr, $label:tt) => { ALT_FW_FTR_SECTION_END_NESTED!($msk, $msk, $label) }; }
macro_rules! ALT_FW_FTR_SECTION_END_NESTED_IFCLR { ($msk:expr, $label:tt) => { ALT_FW_FTR_SECTION_END_NESTED!($msk, 0, $label) }; }
macro_rules! ALT_FW_FTR_SECTION_END { ($msk:expr, $val:expr) => { ALT_FW_FTR_SECTION_END_NESTED!($msk, $val, 97) }; }
macro_rules! ALT_FW_FTR_SECTION_END_IFSET { ($msk:expr) => { ALT_FW_FTR_SECTION_END_NESTED_IFSET!($msk, 97) }; }
macro_rules! ALT_FW_FTR_SECTION_END_IFCLR { ($msk:expr) => { ALT_FW_FTR_SECTION_END_NESTED_IFCLR!($msk, 97) }; }

macro_rules! ASM_FTR_IF { ($section_if:expr, $section_else:expr, $msk:expr, $val:expr) => { ($section_if, $section_else, $msk, $val) }; }
macro_rules! ASM_FTR_IFSET { ($section_if:expr, $section_else:expr, $msk:expr) => { ASM_FTR_IF!($section_if, $section_else, $msk, $msk) }; }
macro_rules! ASM_FTR_IFCLR { ($section_if:expr, $section_else:expr, $msk:expr) => { ASM_FTR_IF!($section_if, $section_else, $msk, 0) }; }
macro_rules! ASM_MMU_FTR_IF { ($section_if:expr, $section_else:expr, $msk:expr, $val:expr) => { ($section_if, $section_else, $msk, $val) }; }
macro_rules! ASM_MMU_FTR_IFSET { ($section_if:expr, $section_else:expr, $msk:expr) => { ASM_MMU_FTR_IF!($section_if, $section_else, $msk, $msk) }; }
macro_rules! ASM_MMU_FTR_IFCLR { ($section_if:expr, $section_else:expr, $msk:expr) => { ASM_MMU_FTR_IF!($section_if, $section_else, $msk, 0) }; }

macro_rules! START_LWSYNC_SECTION { ($label:tt) => { concat!(stringify!($label), "1:") }; }
macro_rules! MAKE_LWSYNC_SECTION_ENTRY { ($label:tt, $sect:tt) => { (stringify!($label), stringify!($sect)) }; }
macro_rules! STF_ENTRY_BARRIER_FIXUP_SECTION { () => { "953: .pushsection __stf_entry_barrier_fixup" }; }
macro_rules! STF_EXIT_BARRIER_FIXUP_SECTION { () => { "955: .pushsection __stf_exit_barrier_fixup" }; }
macro_rules! UACCESS_FLUSH_FIXUP_SECTION { () => { "959: .pushsection __uaccess_flush_fixup" }; }
macro_rules! ENTRY_FLUSH_FIXUP_SECTION { () => { "957: .pushsection __entry_flush_fixup" }; }
macro_rules! SCV_ENTRY_FLUSH_FIXUP_SECTION { () => { "957: .pushsection __scv_entry_flush_fixup" }; }
macro_rules! RFI_FLUSH_FIXUP_SECTION { () => { "951: .pushsection __rfi_flush_fixup" }; }
macro_rules! NOSPEC_BARRIER_FIXUP_SECTION { () => { "953: .pushsection __barrier_nospec_fixup" }; }
macro_rules! START_BTB_FLUSH_SECTION { () => { "955:" }; }
macro_rules! END_BTB_FLUSH_SECTION { () => { "956: .pushsection __btb_flush_fixup" }; }

extern "C" {
    pub static mut stf_barrier_fallback: core::ffi::c_long;
    pub static mut entry_flush_fallback: core::ffi::c_long;
    pub static mut scv_entry_flush_fallback: core::ffi::c_long;
    pub static mut __start___stf_entry_barrier_fixup: core::ffi::c_long;
    pub static mut __stop___stf_entry_barrier_fixup: core::ffi::c_long;
    pub static mut __start___stf_exit_barrier_fixup: core::ffi::c_long;
    pub static mut __stop___stf_exit_barrier_fixup: core::ffi::c_long;
    pub static mut __start___uaccess_flush_fixup: core::ffi::c_long;
    pub static mut __stop___uaccess_flush_fixup: core::ffi::c_long;
    pub static mut __start___entry_flush_fixup: core::ffi::c_long;
    pub static mut __stop___entry_flush_fixup: core::ffi::c_long;
    pub static mut __start___scv_entry_flush_fixup: core::ffi::c_long;
    pub static mut __stop___scv_entry_flush_fixup: core::ffi::c_long;
    pub static mut __start___rfi_flush_fixup: core::ffi::c_long;
    pub static mut __stop___rfi_flush_fixup: core::ffi::c_long;
    pub static mut __start___barrier_nospec_fixup: core::ffi::c_long;
    pub static mut __stop___barrier_nospec_fixup: core::ffi::c_long;
    pub static mut __start__btb_flush_fixup: core::ffi::c_long;
    pub static mut __stop__btb_flush_fixup: core::ffi::c_long;
    pub static mut static_key_feature_checks_initialized: bool;

    pub fn apply_feature_fixups();
    pub fn update_mmu_feature_fixups(mask: usize);
    pub fn setup_feature_keys();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
