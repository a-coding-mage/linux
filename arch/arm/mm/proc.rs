// SPDX-License-Identifier: GPL-2.0-only
/*
/*
 * This file defines C prototypes for the low-level processor assembly functions
 * and creates a reference for CFI. This needs to be done for every assembly
 * processor ("proc") function that is called from C but does not have a
 * corresponding C implementation.
 *
 * Processors are listed in the order they appear in the Makefile.
 *
 * Functions are listed if and only if they see use on the target CPU, and in
*/
// The included kernel declarations are supplied by other translation units.
extern "C" {
 */
// #include <asm/proc-fns.h>

// #ifdef CONFIG_CPU_ARM7TDMI
    pub fn cpu_arm7tdmi_proc_init();
// __ADDRESSABLE(cpu_arm7tdmi_proc_init);
    pub fn cpu_arm7tdmi_proc_fin();
// __ADDRESSABLE(cpu_arm7tdmi_proc_fin);
    pub fn cpu_arm7tdmi_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_arm7tdmi_reset);
    pub fn cpu_arm7tdmi_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_arm7tdmi_do_idle);
    pub fn cpu_arm7tdmi_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_arm7tdmi_dcache_clean_area);
    pub fn cpu_arm7tdmi_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_arm7tdmi_switch_mm);
// #endif

// #ifdef CONFIG_CPU_ARM720T
    pub fn cpu_arm720_proc_init();
// __ADDRESSABLE(cpu_arm720_proc_init);
    pub fn cpu_arm720_proc_fin();
// __ADDRESSABLE(cpu_arm720_proc_fin);
    pub fn cpu_arm720_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_arm720_reset);
    pub fn cpu_arm720_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_arm720_do_idle);
    pub fn cpu_arm720_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_arm720_dcache_clean_area);
    pub fn cpu_arm720_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_arm720_switch_mm);
    pub fn cpu_arm720_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_arm720_set_pte_ext);
// #endif

// #ifdef CONFIG_CPU_ARM740T
    pub fn cpu_arm740_proc_init();
// __ADDRESSABLE(cpu_arm740_proc_init);
    pub fn cpu_arm740_proc_fin();
// __ADDRESSABLE(cpu_arm740_proc_fin);
    pub fn cpu_arm740_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_arm740_reset);
    pub fn cpu_arm740_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_arm740_do_idle);
    pub fn cpu_arm740_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_arm740_dcache_clean_area);
    pub fn cpu_arm740_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_arm740_switch_mm);
// #endif

// #ifdef CONFIG_CPU_ARM9TDMI
    pub fn cpu_arm9tdmi_proc_init();
// __ADDRESSABLE(cpu_arm9tdmi_proc_init);
    pub fn cpu_arm9tdmi_proc_fin();
// __ADDRESSABLE(cpu_arm9tdmi_proc_fin);
    pub fn cpu_arm9tdmi_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_arm9tdmi_reset);
    pub fn cpu_arm9tdmi_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_arm9tdmi_do_idle);
    pub fn cpu_arm9tdmi_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_arm9tdmi_dcache_clean_area);
    pub fn cpu_arm9tdmi_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_arm9tdmi_switch_mm);
// #endif

// #ifdef CONFIG_CPU_ARM920T
    pub fn cpu_arm920_proc_init();
// __ADDRESSABLE(cpu_arm920_proc_init);
    pub fn cpu_arm920_proc_fin();
// __ADDRESSABLE(cpu_arm920_proc_fin);
    pub fn cpu_arm920_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_arm920_reset);
    pub fn cpu_arm920_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_arm920_do_idle);
    pub fn cpu_arm920_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_arm920_dcache_clean_area);
    pub fn cpu_arm920_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_arm920_switch_mm);
    pub fn cpu_arm920_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_arm920_set_pte_ext);
// #ifdef CONFIG_ARM_CPU_SUSPEND
    pub fn cpu_arm920_do_suspend(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_arm920_do_suspend);
    pub fn cpu_arm920_do_resume(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_arm920_do_resume);
// #endif /* CONFIG_ARM_CPU_SUSPEND */
// #endif /* CONFIG_CPU_ARM920T */

// #ifdef CONFIG_CPU_ARM922T
    pub fn cpu_arm922_proc_init();
// __ADDRESSABLE(cpu_arm922_proc_init);
    pub fn cpu_arm922_proc_fin();
// __ADDRESSABLE(cpu_arm922_proc_fin);
    pub fn cpu_arm922_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_arm922_reset);
    pub fn cpu_arm922_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_arm922_do_idle);
    pub fn cpu_arm922_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_arm922_dcache_clean_area);
    pub fn cpu_arm922_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_arm922_switch_mm);
    pub fn cpu_arm922_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_arm922_set_pte_ext);
// #endif

// #ifdef CONFIG_CPU_ARM925T
    pub fn cpu_arm925_proc_init();
// __ADDRESSABLE(cpu_arm925_proc_init);
    pub fn cpu_arm925_proc_fin();
// __ADDRESSABLE(cpu_arm925_proc_fin);
    pub fn cpu_arm925_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_arm925_reset);
    pub fn cpu_arm925_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_arm925_do_idle);
    pub fn cpu_arm925_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_arm925_dcache_clean_area);
    pub fn cpu_arm925_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_arm925_switch_mm);
    pub fn cpu_arm925_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_arm925_set_pte_ext);
// #endif

// #ifdef CONFIG_CPU_ARM926T
    pub fn cpu_arm926_proc_init();
// __ADDRESSABLE(cpu_arm926_proc_init);
    pub fn cpu_arm926_proc_fin();
// __ADDRESSABLE(cpu_arm926_proc_fin);
    pub fn cpu_arm926_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_arm926_reset);
    pub fn cpu_arm926_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_arm926_do_idle);
    pub fn cpu_arm926_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_arm926_dcache_clean_area);
    pub fn cpu_arm926_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_arm926_switch_mm);
    pub fn cpu_arm926_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_arm926_set_pte_ext);
// #ifdef CONFIG_ARM_CPU_SUSPEND
    pub fn cpu_arm926_do_suspend(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_arm926_do_suspend);
    pub fn cpu_arm926_do_resume(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_arm926_do_resume);
// #endif /* CONFIG_ARM_CPU_SUSPEND */
// #endif /* CONFIG_CPU_ARM926T */

// #ifdef CONFIG_CPU_ARM940T
    pub fn cpu_arm940_proc_init();
// __ADDRESSABLE(cpu_arm940_proc_init);
    pub fn cpu_arm940_proc_fin();
// __ADDRESSABLE(cpu_arm940_proc_fin);
    pub fn cpu_arm940_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_arm940_reset);
    pub fn cpu_arm940_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_arm940_do_idle);
    pub fn cpu_arm940_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_arm940_dcache_clean_area);
    pub fn cpu_arm940_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_arm940_switch_mm);
// #endif

// #ifdef CONFIG_CPU_ARM946E
    pub fn cpu_arm946_proc_init();
// __ADDRESSABLE(cpu_arm946_proc_init);
    pub fn cpu_arm946_proc_fin();
// __ADDRESSABLE(cpu_arm946_proc_fin);
    pub fn cpu_arm946_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_arm946_reset);
    pub fn cpu_arm946_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_arm946_do_idle);
    pub fn cpu_arm946_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_arm946_dcache_clean_area);
    pub fn cpu_arm946_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_arm946_switch_mm);
// #endif

// #ifdef CONFIG_CPU_FA526
    pub fn cpu_fa526_proc_init();
// __ADDRESSABLE(cpu_fa526_proc_init);
    pub fn cpu_fa526_proc_fin();
// __ADDRESSABLE(cpu_fa526_proc_fin);
    pub fn cpu_fa526_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_fa526_reset);
    pub fn cpu_fa526_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_fa526_do_idle);
    pub fn cpu_fa526_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_fa526_dcache_clean_area);
    pub fn cpu_fa526_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_fa526_switch_mm);
    pub fn cpu_fa526_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_fa526_set_pte_ext);
// #endif

// #ifdef CONFIG_CPU_ARM1020
    pub fn cpu_arm1020_proc_init();
// __ADDRESSABLE(cpu_arm1020_proc_init);
    pub fn cpu_arm1020_proc_fin();
// __ADDRESSABLE(cpu_arm1020_proc_fin);
    pub fn cpu_arm1020_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_arm1020_reset);
    pub fn cpu_arm1020_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_arm1020_do_idle);
    pub fn cpu_arm1020_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_arm1020_dcache_clean_area);
    pub fn cpu_arm1020_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_arm1020_switch_mm);
    pub fn cpu_arm1020_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_arm1020_set_pte_ext);
// #endif

// #ifdef CONFIG_CPU_ARM1020E
    pub fn cpu_arm1020e_proc_init();
// __ADDRESSABLE(cpu_arm1020e_proc_init);
    pub fn cpu_arm1020e_proc_fin();
// __ADDRESSABLE(cpu_arm1020e_proc_fin);
    pub fn cpu_arm1020e_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_arm1020e_reset);
    pub fn cpu_arm1020e_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_arm1020e_do_idle);
    pub fn cpu_arm1020e_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_arm1020e_dcache_clean_area);
    pub fn cpu_arm1020e_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_arm1020e_switch_mm);
    pub fn cpu_arm1020e_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_arm1020e_set_pte_ext);
// #endif

// #ifdef CONFIG_CPU_ARM1022
    pub fn cpu_arm1022_proc_init();
// __ADDRESSABLE(cpu_arm1022_proc_init);
    pub fn cpu_arm1022_proc_fin();
// __ADDRESSABLE(cpu_arm1022_proc_fin);
    pub fn cpu_arm1022_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_arm1022_reset);
    pub fn cpu_arm1022_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_arm1022_do_idle);
    pub fn cpu_arm1022_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_arm1022_dcache_clean_area);
    pub fn cpu_arm1022_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_arm1022_switch_mm);
    pub fn cpu_arm1022_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_arm1022_set_pte_ext);
// #endif

// #ifdef CONFIG_CPU_ARM1026
    pub fn cpu_arm1026_proc_init();
// __ADDRESSABLE(cpu_arm1026_proc_init);
    pub fn cpu_arm1026_proc_fin();
// __ADDRESSABLE(cpu_arm1026_proc_fin);
    pub fn cpu_arm1026_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_arm1026_reset);
    pub fn cpu_arm1026_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_arm1026_do_idle);
    pub fn cpu_arm1026_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_arm1026_dcache_clean_area);
    pub fn cpu_arm1026_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_arm1026_switch_mm);
    pub fn cpu_arm1026_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_arm1026_set_pte_ext);
// #endif

// #ifdef CONFIG_CPU_SA110
    pub fn cpu_sa110_proc_init();
// __ADDRESSABLE(cpu_sa110_proc_init);
    pub fn cpu_sa110_proc_fin();
// __ADDRESSABLE(cpu_sa110_proc_fin);
    pub fn cpu_sa110_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_sa110_reset);
    pub fn cpu_sa110_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_sa110_do_idle);
    pub fn cpu_sa110_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_sa110_dcache_clean_area);
    pub fn cpu_sa110_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_sa110_switch_mm);
    pub fn cpu_sa110_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_sa110_set_pte_ext);
// #endif

// #ifdef CONFIG_CPU_SA1100
    pub fn cpu_sa1100_proc_init();
// __ADDRESSABLE(cpu_sa1100_proc_init);
    pub fn cpu_sa1100_proc_fin();
// __ADDRESSABLE(cpu_sa1100_proc_fin);
    pub fn cpu_sa1100_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_sa1100_reset);
    pub fn cpu_sa1100_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_sa1100_do_idle);
    pub fn cpu_sa1100_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_sa1100_dcache_clean_area);
    pub fn cpu_sa1100_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_sa1100_switch_mm);
    pub fn cpu_sa1100_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_sa1100_set_pte_ext);
// #ifdef CONFIG_ARM_CPU_SUSPEND
    pub fn cpu_sa1100_do_suspend(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_sa1100_do_suspend);
    pub fn cpu_sa1100_do_resume(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_sa1100_do_resume);
// #endif /* CONFIG_ARM_CPU_SUSPEND */
// #endif /* CONFIG_CPU_SA1100 */

// #ifdef CONFIG_CPU_XSCALE
    pub fn cpu_xscale_proc_init();
// __ADDRESSABLE(cpu_xscale_proc_init);
    pub fn cpu_xscale_proc_fin();
// __ADDRESSABLE(cpu_xscale_proc_fin);
    pub fn cpu_xscale_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_xscale_reset);
    pub fn cpu_xscale_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_xscale_do_idle);
    pub fn cpu_xscale_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_xscale_dcache_clean_area);
    pub fn cpu_xscale_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_xscale_switch_mm);
    pub fn cpu_xscale_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_xscale_set_pte_ext);
// #ifdef CONFIG_ARM_CPU_SUSPEND
    pub fn cpu_xscale_do_suspend(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_xscale_do_suspend);
    pub fn cpu_xscale_do_resume(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_xscale_do_resume);
// #endif /* CONFIG_ARM_CPU_SUSPEND */
// #endif /* CONFIG_CPU_XSCALE */

// #ifdef CONFIG_CPU_XSC3
    pub fn cpu_xsc3_proc_init();
// __ADDRESSABLE(cpu_xsc3_proc_init);
    pub fn cpu_xsc3_proc_fin();
// __ADDRESSABLE(cpu_xsc3_proc_fin);
    pub fn cpu_xsc3_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_xsc3_reset);
    pub fn cpu_xsc3_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_xsc3_do_idle);
    pub fn cpu_xsc3_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_xsc3_dcache_clean_area);
    pub fn cpu_xsc3_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_xsc3_switch_mm);
    pub fn cpu_xsc3_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_xsc3_set_pte_ext);
// #ifdef CONFIG_ARM_CPU_SUSPEND
    pub fn cpu_xsc3_do_suspend(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_xsc3_do_suspend);
    pub fn cpu_xsc3_do_resume(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_xsc3_do_resume);
// #endif /* CONFIG_ARM_CPU_SUSPEND */
// #endif /* CONFIG_CPU_XSC3 */

// #ifdef CONFIG_CPU_MOHAWK
    pub fn cpu_mohawk_proc_init();
// __ADDRESSABLE(cpu_mohawk_proc_init);
    pub fn cpu_mohawk_proc_fin();
// __ADDRESSABLE(cpu_mohawk_proc_fin);
    pub fn cpu_mohawk_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_mohawk_reset);
    pub fn cpu_mohawk_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_mohawk_do_idle);
    pub fn cpu_mohawk_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_mohawk_dcache_clean_area);
    pub fn cpu_mohawk_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_mohawk_switch_mm);
    pub fn cpu_mohawk_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_mohawk_set_pte_ext);
// #ifdef CONFIG_ARM_CPU_SUSPEND
    pub fn cpu_mohawk_do_suspend(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_mohawk_do_suspend);
    pub fn cpu_mohawk_do_resume(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_mohawk_do_resume);
// #endif /* CONFIG_ARM_CPU_SUSPEND */
// #endif /* CONFIG_CPU_MOHAWK */

// #ifdef CONFIG_CPU_FEROCEON
    pub fn cpu_feroceon_proc_init();
// __ADDRESSABLE(cpu_feroceon_proc_init);
    pub fn cpu_feroceon_proc_fin();
// __ADDRESSABLE(cpu_feroceon_proc_fin);
    pub fn cpu_feroceon_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_feroceon_reset);
    pub fn cpu_feroceon_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_feroceon_do_idle);
    pub fn cpu_feroceon_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_feroceon_dcache_clean_area);
    pub fn cpu_feroceon_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_feroceon_switch_mm);
    pub fn cpu_feroceon_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_feroceon_set_pte_ext);
// #ifdef CONFIG_ARM_CPU_SUSPEND
    pub fn cpu_feroceon_do_suspend(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_feroceon_do_suspend);
    pub fn cpu_feroceon_do_resume(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_feroceon_do_resume);
// #endif /* CONFIG_ARM_CPU_SUSPEND */
// #endif /* CONFIG_CPU_FEROCEON */

// #if defined(CONFIG_CPU_V6) || defined(CONFIG_CPU_V6K)
    pub fn cpu_v6_proc_init();
// __ADDRESSABLE(cpu_v6_proc_init);
    pub fn cpu_v6_proc_fin();
// __ADDRESSABLE(cpu_v6_proc_fin);
    pub fn cpu_v6_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_v6_reset);
    pub fn cpu_v6_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_v6_do_idle);
    pub fn cpu_v6_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_v6_dcache_clean_area);
    pub fn cpu_v6_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_v6_switch_mm);
    pub fn cpu_v6_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_v6_set_pte_ext);
// #ifdef CONFIG_ARM_CPU_SUSPEND
    pub fn cpu_v6_do_suspend(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_v6_do_suspend);
    pub fn cpu_v6_do_resume(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_v6_do_resume);
// #endif /* CONFIG_ARM_CPU_SUSPEND */
// #endif /* CPU_V6 */

// #ifdef CONFIG_CPU_V7
    pub fn cpu_v7_proc_init();
// __ADDRESSABLE(cpu_v7_proc_init);
    pub fn cpu_v7_proc_fin();
// __ADDRESSABLE(cpu_v7_proc_fin);
    pub fn cpu_v7_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_v7_reset);
    pub fn cpu_v7_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_v7_do_idle);
// #ifdef CONFIG_PJ4B_ERRATA_4742
    pub fn cpu_pj4b_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_pj4b_do_idle);
// #endif
    pub fn cpu_v7_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_v7_dcache_clean_area);
    pub fn cpu_v7_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
    // Special switch_mm() callbacks to work around bugs in v7 */
// __ADDRESSABLE(cpu_v7_switch_mm);
    pub fn cpu_v7_iciallu_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_v7_iciallu_switch_mm);
    pub fn cpu_v7_bpiall_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_v7_bpiall_switch_mm);
// #ifdef CONFIG_ARM_LPAE
    pub fn cpu_v7_set_pte_ext(ptep: *mut pte_t, pte: pte_t);
// #else
    pub fn cpu_v7_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// #endif
// __ADDRESSABLE(cpu_v7_set_pte_ext);
// #ifdef CONFIG_ARM_CPU_SUSPEND
    pub fn cpu_v7_do_suspend(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_v7_do_suspend);
    pub fn cpu_v7_do_resume(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_v7_do_resume);
    // Special versions of suspend and resume for the CA9MP cores */
    pub fn cpu_ca9mp_do_suspend(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_ca9mp_do_suspend);
    pub fn cpu_ca9mp_do_resume(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_ca9mp_do_resume);
    // Special versions of suspend and resume for the Marvell PJ4B cores */
// #ifdef CONFIG_CPU_PJ4B
    pub fn cpu_pj4b_do_suspend(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_pj4b_do_suspend);
    pub fn cpu_pj4b_do_resume(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_pj4b_do_resume);
// #endif /* CONFIG_CPU_PJ4B */
// #endif /* CONFIG_ARM_CPU_SUSPEND */
// #endif /* CONFIG_CPU_V7 */

// #ifdef CONFIG_CPU_V7M
    pub fn cpu_v7m_proc_init();
// __ADDRESSABLE(cpu_v7m_proc_init);
    pub fn cpu_v7m_proc_fin();
// __ADDRESSABLE(cpu_v7m_proc_fin);
    pub fn cpu_v7m_reset(addr: core::ffi::c_ulong, hvc: bool);
// __ADDRESSABLE(cpu_v7m_reset);
    pub fn cpu_v7m_do_idle() -> core::ffi::c_int;
// __ADDRESSABLE(cpu_v7m_do_idle);
    pub fn cpu_v7m_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_v7m_dcache_clean_area);
    pub fn cpu_v7m_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
// __ADDRESSABLE(cpu_v7m_switch_mm);
    pub fn cpu_v7m_set_pte_ext(ptep: *mut pte_t, pte: pte_t, ext: core::ffi::c_uint);
// __ADDRESSABLE(cpu_v7m_set_pte_ext);
// #ifdef CONFIG_ARM_CPU_SUSPEND
    pub fn cpu_v7m_do_suspend(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_v7m_do_suspend);
    pub fn cpu_v7m_do_resume(arg: *mut core::ffi::c_void);
// __ADDRESSABLE(cpu_v7m_do_resume);
// #endif /* CONFIG_ARM_CPU_SUSPEND */
    pub fn cpu_cm7_proc_fin();
// __ADDRESSABLE(cpu_cm7_proc_fin);
    pub fn cpu_cm7_dcache_clean_area(addr: *mut core::ffi::c_void, size: core::ffi::c_int);
// __ADDRESSABLE(cpu_cm7_dcache_clean_area);
// #endif /* CONFIG_CPU_V7M */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
