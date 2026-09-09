/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Linker script variables to be set after section resolution, as
 * ld.lld does not like variables assigned before SECTIONS is processed.
 *
 * This header is consumed by the linker-script generation path.  The items
 * below are retained as linker directives because they have no standalone
 * Rust object-file equivalent.
 */

// Original build-time requirement:
// #ifndef LINKER_SCRIPT
// #error This file should only be included in vmlinux.lds.S
// #endif

// For old LLD versions, ASSERT(...) is intentionally disabled:
// #if defined(CONFIG_LD_IS_LLD) && CONFIG_LLD_VERSION < 210000
// #define ASSERT(...)
// #endif

// #define PI_EXPORT_SYM(sym) \
//     __PI_EXPORT_SYM(sym, __pi_ ## sym, Cannot export BSS symbol sym to startup code)
// #define __PI_EXPORT_SYM(sym, pisym, msg) \
//     PROVIDE(pisym = sym); \
//     ASSERT((sym - KIMAGE_VADDR) < (__bss_start - KIMAGE_VADDR), #msg)

// PROVIDE(__efistub_primary_entry = primary_entry);

/*
 * The EFI stub has its own symbol namespace prefixed by __efistub_, to
 * isolate it from the kernel proper. The following symbols are legally
 * accessed by the stub, so provide some aliases to make them accessible.
 * Only include data symbols here, or text symbols of functions that are
 * guaranteed to be safe when executed at another offset than they were
 * linked at. The routines below are all implemented in assembler in a
 * position independent manner.
 */
// PROVIDE(__efistub_caches_clean_inval_pou = __pi_caches_clean_inval_pou);
// PROVIDE(__efistub__text = _text);
// PROVIDE(__efistub__end = _end);
// PROVIDE(__efistub___inittext_end = __inittext_end);
// PROVIDE(__efistub__edata = _edata);
// #if defined(CONFIG_EFI_EARLYCON) || defined(CONFIG_SYSFB)
// PROVIDE(__efistub_sysfb_primary_display = sysfb_primary_display);
// #endif
// PROVIDE(__efistub__ctype = _ctype);
// PROVIDE(__pi___memcpy = __pi_memcpy);
// PROVIDE(__pi___memmove = __pi_memmove);
// PROVIDE(__pi___memset = __pi_memset);

// PI_EXPORT_SYM(id_aa64isar1_override);
// PI_EXPORT_SYM(id_aa64isar2_override);
// PI_EXPORT_SYM(id_aa64mmfr0_override);
// PI_EXPORT_SYM(id_aa64mmfr1_override);
// PI_EXPORT_SYM(id_aa64mmfr2_override);
// PI_EXPORT_SYM(id_aa64mmfr4_override);
// PI_EXPORT_SYM(id_aa64pfr0_override);
// PI_EXPORT_SYM(id_aa64pfr1_override);
// PI_EXPORT_SYM(id_aa64smfr0_override);
// PI_EXPORT_SYM(id_aa64zfr0_override);
// PI_EXPORT_SYM(arm64_sw_feature_override);
// PI_EXPORT_SYM(arm64_use_ng_mappings);
// PI_EXPORT_SYM(_ctype);
// PI_EXPORT_SYM(swapper_pg_dir);
// PI_EXPORT_SYM(_text);
// PI_EXPORT_SYM(_stext);
// PI_EXPORT_SYM(_etext);
// PI_EXPORT_SYM(__start_rodata);
// PI_EXPORT_SYM(__inittext_begin);
// PI_EXPORT_SYM(__inittext_end);
// PI_EXPORT_SYM(__initdata_begin);
// PI_EXPORT_SYM(__initdata_end);
// PI_EXPORT_SYM(_data);

// #ifdef CONFIG_KVM
// KVM nVHE aliases, preserving the original conditional linker interface.
// KVM_NVHE_ALIAS(kvm_patch_vector_branch);
// KVM_NVHE_ALIAS(kvm_update_va_mask);
// KVM_NVHE_ALIAS(kvm_get_kimage_voffset);
// KVM_NVHE_ALIAS(kvm_compute_final_ctr_el0);
// KVM_NVHE_ALIAS(spectre_bhb_patch_loop_iter);
// KVM_NVHE_ALIAS(spectre_bhb_patch_loop_mitigation_enable);
// KVM_NVHE_ALIAS(spectre_bhb_patch_wa3);
// KVM_NVHE_ALIAS(spectre_bhb_patch_clearbhb);
// KVM_NVHE_ALIAS(alt_cb_patch_nops);
// KVM_NVHE_ALIAS(kvm_compute_ich_hcr_trap_bits);
// KVM_NVHE_ALIAS(kvm_patch_ich_vtr_el2);
// KVM_NVHE_ALIAS(kvm_vgic_global_state);
// KVM_NVHE_ALIAS(nvhe_hyp_panic_handler);
// KVM_NVHE_ALIAS(__hyp_stub_vectors);
// KVM_NVHE_ALIAS(vgic_v2_cpuif_trap);
// KVM_NVHE_ALIAS(vgic_v3_cpuif_trap);
// KVM_NVHE_ALIAS(vgic_v3_has_v2_compat);
// KVM_NVHE_ALIAS(broken_cntvoff_key);
// KVM_NVHE_ALIAS(__start___kvm_ex_table);
// KVM_NVHE_ALIAS(__stop___kvm_ex_table);
// KVM_NVHE_ALIAS_HYP(clear_page, __pi_clear_page);
// KVM_NVHE_ALIAS_HYP(copy_page, __pi_copy_page);
// KVM_NVHE_ALIAS_HYP(memcpy, __pi_memcpy);
// KVM_NVHE_ALIAS_HYP(memset, __pi_memset);
// #ifdef CONFIG_KASAN
// KVM_NVHE_ALIAS_HYP(__memcpy, __pi_memcpy);
// KVM_NVHE_ALIAS_HYP(__memset, __pi_memset);
// #endif
// KVM_NVHE_ALIAS(__hyp_idmap_text_start);
// KVM_NVHE_ALIAS(__hyp_idmap_text_end);
// KVM_NVHE_ALIAS(__hyp_text_start);
// KVM_NVHE_ALIAS(__hyp_text_end);
// KVM_NVHE_ALIAS(__hyp_bss_start);
// KVM_NVHE_ALIAS(__hyp_bss_end);
// KVM_NVHE_ALIAS(__hyp_data_start);
// KVM_NVHE_ALIAS(__hyp_data_end);
// KVM_NVHE_ALIAS(__hyp_rodata_start);
// KVM_NVHE_ALIAS(__hyp_rodata_end);
// #ifdef CONFIG_NVHE_EL2_TRACING
// KVM_NVHE_ALIAS(__hyp_event_ids_start);
// KVM_NVHE_ALIAS(__hyp_event_ids_end);
// #endif
// KVM_NVHE_ALIAS(kvm_protected_mode_initialized);
// #endif /* CONFIG_KVM */

// #ifdef CONFIG_EFI_ZBOOT
// _kernel_codesize = ABSOLUTE(__inittext_end - _text);
// #endif

/*
 * LLD will occasionally error out with a '__init_end does not converge' error
 * if INIT_IDMAP_DIR_SIZE is defined in terms of _end, as this results in a
 * circular dependency. Counter this by dimensioning the initial IDMAP page
 * tables based on kimage_limit, which is defined such that its value should
 * not change as a result of the initdata segment being pushed over a 64k
 * segment boundary due to changes in INIT_IDMAP_DIR_SIZE, provided that its
 * value doesn't change by more than 2M between linker passes.
 */
// kimage_limit = ALIGN(ABSOLUTE(_end + SZ_64K), SZ_2M);

// #undef ASSERT

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
