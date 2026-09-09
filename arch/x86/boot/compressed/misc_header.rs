/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Special hack: paravirt and debugging indirections are disabled for the
 * bare-metal boot path. Build-time configuration symbols are preserved here
 * as comments because they are supplied by the surrounding build.
 */
/* #undef CONFIG_PARAVIRT */
/* #undef CONFIG_PARAVIRT_XXL */
/* #undef CONFIG_PARAVIRT_SPINLOCKS */
/* #undef CONFIG_ARCH_HAS_LAZY_MMU_MODE */
/* #undef CONFIG_KASAN */
/* #undef CONFIG_KASAN_GENERIC */

/* #define __NO_FORTIFY */
/* cpu_feature_enabled() cannot be used this early. */
/* #define USE_EARLY_PGTABLE_L5 */

/* Identity mappings: physical and virtual addresses are the same. */
#[inline]
pub unsafe fn __pa<T>(x: *const T) -> ::core::ffi::c_ulong {
    x as ::core::ffi::c_ulong
}

#[inline]
pub unsafe fn __va<T>(x: ::core::ffi::c_ulong) -> *mut T {
    x as *mut T
}

/* C header dependencies are supplied by the surrounding translation unit. */
/* linux/linkage.h, linux/screen_info.h, linux/elf.h, asm/page.h,
 * asm/boot.h, asm/bootparam.h, asm/desc_defs.h, tdx.h, linux/acpi.h,
 * ../ctype.h, ../io.h, and efi.h */

#[cfg(target_pointer_width = "64")]
pub type memptr = ::core::ffi::c_long;
#[cfg(not(target_pointer_width = "64"))]
pub type memptr = ::core::ffi::c_uint;

extern "C" {
    pub static mut _head: [::core::ffi::c_char; 0];
    pub static mut _end: [::core::ffi::c_char; 0];

    pub static mut free_mem_ptr: memptr;
    pub static mut free_mem_end_ptr: memptr;
    pub static mut spurious_nmi_count: ::core::ffi::c_int;
    pub fn malloc(size: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    pub fn free(where_: *mut ::core::ffi::c_void);
    pub fn __putstr(s: *const ::core::ffi::c_char);
    pub fn __puthex(value: ::core::ffi::c_ulong);
    pub fn __putdec(value: ::core::ffi::c_ulong);
}

#[inline]
pub unsafe fn error_putstr(x: *const ::core::ffi::c_char) { __putstr(x); }
#[inline]
pub unsafe fn error_puthex(x: ::core::ffi::c_ulong) { __puthex(x); }
#[inline]
pub unsafe fn error_putdec(x: ::core::ffi::c_ulong) { __putdec(x); }

#[cfg(CONFIG_X86_VERBOSE_BOOTUP)]
#[inline]
pub unsafe fn debug_putstr(s: *const ::core::ffi::c_char) { __putstr(s); }
#[cfg(CONFIG_X86_VERBOSE_BOOTUP)]
#[inline]
pub unsafe fn debug_puthex(value: ::core::ffi::c_ulong) { __puthex(value); }
#[cfg(CONFIG_X86_VERBOSE_BOOTUP)]
#[inline]
pub unsafe fn debug_putaddr<T>(x: *const T, name: *const ::core::ffi::c_char) {
    debug_putstr(name);
    debug_putstr(b": 0x\0".as_ptr() as *const _);
    debug_puthex(x as ::core::ffi::c_ulong);
    debug_putstr(b"\n\0".as_ptr() as *const _);
}
#[cfg(not(CONFIG_X86_VERBOSE_BOOTUP))]
#[inline]
pub unsafe fn debug_putstr(_s: *const ::core::ffi::c_char) {}
#[cfg(not(CONFIG_X86_VERBOSE_BOOTUP))]
#[inline]
pub unsafe fn debug_puthex(_value: ::core::ffi::c_ulong) {}
#[cfg(not(CONFIG_X86_VERBOSE_BOOTUP))]
#[inline]
pub unsafe fn debug_putaddr<T>(_x: *const T, _name: *const ::core::ffi::c_char) {}

extern "C" {
    pub fn cmdline_find_option(option: *const ::core::ffi::c_char,
                               buffer: *mut ::core::ffi::c_char,
                               bufsize: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn cmdline_find_option_bool(option: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct mem_vector {
    pub start: u64,
    pub size: u64,
}

#[cfg(CONFIG_RANDOMIZE_BASE)]
extern "C" {
    pub fn choose_random_location(input: ::core::ffi::c_ulong,
                                  input_size: ::core::ffi::c_ulong,
                                  output: *mut ::core::ffi::c_ulong,
                                  output_size: ::core::ffi::c_ulong,
                                  virt_addr: *mut ::core::ffi::c_ulong);
}
#[cfg(not(CONFIG_RANDOMIZE_BASE))]
#[inline]
pub unsafe fn choose_random_location(_input: ::core::ffi::c_ulong,
                                     _input_size: ::core::ffi::c_ulong,
                                     _output: *mut ::core::ffi::c_ulong,
                                     _output_size: ::core::ffi::c_ulong,
                                     _virt_addr: *mut ::core::ffi::c_ulong) {}

extern "C" {
    pub fn has_cpuflag(flag: ::core::ffi::c_int) -> bool;
}

#[cfg(target_pointer_width = "64")]
extern "C" {
    pub fn set_page_decrypted(address: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn set_page_encrypted(address: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn set_page_non_present(address: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub static mut _pgtable: [u8; 0];
}

#[cfg(CONFIG_EARLY_PRINTK)]
extern "C" {
    pub static mut early_serial_base: ::core::ffi::c_int;
    pub fn console_init();
}
#[cfg(not(CONFIG_EARLY_PRINTK))]
pub static early_serial_base: ::core::ffi::c_int = 0;
#[cfg(not(CONFIG_EARLY_PRINTK))]
#[inline]
pub unsafe fn console_init() {}

/* AMD memory-encryption declarations and stubs. */
#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
extern "C" {
    pub fn sev_enable(bp: *mut boot_params);
    pub fn snp_check_features();
    pub fn sev_es_shutdown_ghcb();
    pub fn sev_es_check_ghcb_fault(address: ::core::ffi::c_ulong) -> bool;
    pub fn snp_set_page_private(paddr: ::core::ffi::c_ulong);
    pub fn snp_set_page_shared(paddr: ::core::ffi::c_ulong);
    pub fn sev_prep_identity_maps(top_level_pgt: ::core::ffi::c_ulong);
    pub fn vc_decode_insn(ctxt: *mut es_em_ctxt) -> es_result;
    pub fn insn_has_rep_prefix(insn: *mut insn) -> bool;
    pub fn sev_insn_decode_init();
    pub fn early_setup_ghcb() -> bool;
}
#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
pub unsafe fn snp_check_features() {}
#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
pub unsafe fn sev_es_shutdown_ghcb() {}
#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
pub unsafe fn sev_es_check_ghcb_fault(_address: ::core::ffi::c_ulong) -> bool { false }
#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
pub unsafe fn snp_set_page_private(_paddr: ::core::ffi::c_ulong) {}
#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
pub unsafe fn snp_set_page_shared(_paddr: ::core::ffi::c_ulong) {}
#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
pub unsafe fn sev_prep_identity_maps(_top_level_pgt: ::core::ffi::c_ulong) {}

#[cfg(CONFIG_ACPI)]
extern "C" { pub fn get_rsdp_addr() -> acpi_physical_address; }
#[cfg(not(CONFIG_ACPI))]
pub unsafe fn get_rsdp_addr() -> acpi_physical_address { 0 }

#[cfg(all(CONFIG_RANDOMIZE_BASE, CONFIG_MEMORY_HOTREMOVE, CONFIG_ACPI))]
extern "C" {
    pub static mut immovable_mem: [mem_vector; MAX_NUMNODES * 2];
    pub fn count_immovable_mem_regions() -> ::core::ffi::c_int;
}
#[cfg(not(all(CONFIG_RANDOMIZE_BASE, CONFIG_MEMORY_HOTREMOVE, CONFIG_ACPI)))]
pub unsafe fn count_immovable_mem_regions() -> ::core::ffi::c_int { 0 }

extern "C" {
    pub static mut __pgtable_l5_enabled: ::core::ffi::c_uint;
    pub static mut pgdir_shift: ::core::ffi::c_uint;
    pub static mut ptrs_per_p4d: ::core::ffi::c_uint;
    pub fn kernel_add_identity_map(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub static mut __default_kernel_pte_mask: pteval_t;
    pub static mut boot_idt: [gate_desc; BOOT_IDT_ENTRIES];
    pub static mut boot_idt_desc: desc_ptr;
    pub fn boot_page_fault();
    pub fn boot_nmi_trap();
    pub fn boot_stage1_vc();
    pub fn boot_stage2_vc();
    pub fn sev_verify_cbit(cr3: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
}

#[cfg(target_pointer_width = "64")]
extern "C" { pub fn cleanup_exception_handling(); }
#[cfg(not(target_pointer_width = "64"))]
pub unsafe fn cleanup_exception_handling() {}

#[repr(C)]
pub enum efi_type { EFI_TYPE_64, EFI_TYPE_32, EFI_TYPE_NONE }

#[cfg(CONFIG_EFI)]
extern "C" {
    pub fn efi_get_type(bp: *mut boot_params) -> efi_type;
    pub fn efi_get_system_table(bp: *mut boot_params) -> ::core::ffi::c_ulong;
    pub fn efi_get_conf_table(bp: *mut boot_params, cfg_tbl_pa: *mut ::core::ffi::c_ulong,
                              cfg_tbl_len: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn efi_find_vendor_table(bp: *mut boot_params, cfg_tbl_pa: ::core::ffi::c_ulong,
                                 cfg_tbl_len: ::core::ffi::c_uint, guid: efi_guid_t)
                                 -> ::core::ffi::c_ulong;
}
#[cfg(not(CONFIG_EFI))]
pub unsafe fn efi_get_type(_bp: *mut boot_params) -> efi_type { efi_type::EFI_TYPE_NONE }
#[cfg(not(CONFIG_EFI))]
pub unsafe fn efi_get_system_table(_bp: *mut boot_params) -> ::core::ffi::c_ulong { 0 }
#[cfg(not(CONFIG_EFI))]
pub unsafe fn efi_get_conf_table(_bp: *mut boot_params, _cfg_tbl_pa: *mut ::core::ffi::c_ulong,
                                 _cfg_tbl_len: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int { -ENOENT }
#[cfg(not(CONFIG_EFI))]
pub unsafe fn efi_find_vendor_table(_bp: *mut boot_params, _cfg_tbl_pa: ::core::ffi::c_ulong,
                                    _cfg_tbl_len: ::core::ffi::c_uint, _guid: efi_guid_t)
                                    -> ::core::ffi::c_ulong { 0 }

#[cfg(CONFIG_UNACCEPTED_MEMORY)]
extern "C" { pub fn init_unaccepted_memory() -> bool; }
#[cfg(not(CONFIG_UNACCEPTED_MEMORY))]
pub unsafe fn init_unaccepted_memory() -> bool { false }

extern "C" {
    pub static mut unaccepted_table: *mut efi_unaccepted_memory;
    pub fn accept_memory(start: phys_addr_t, size: ::core::ffi::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
