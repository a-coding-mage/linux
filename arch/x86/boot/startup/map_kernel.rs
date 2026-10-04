// SPDX-License-Identifier: GPL-2.0
//! Relocate the initial page tables and install the startup identity mapping.

use crate::startup_map_bindings as b;
use core::arch::asm;
use core::ffi::{c_int, c_uint, c_ulong, c_void};
use core::ptr::addr_of;

// These casts are the casts in map_kernel.c: the generated native wrappers
// must each contain precisely one native page-table value.
const _: [(); core::mem::size_of::<b::pmd_t>()] = [(); core::mem::size_of::<b::pmdval_t>()];
const _: [(); core::mem::size_of::<b::pud_t>()] = [(); core::mem::size_of::<b::pudval_t>()];
const _: [(); core::mem::size_of::<b::p4d_t>()] = [(); core::mem::size_of::<b::p4dval_t>()];
const _: [(); core::mem::size_of::<b::pgd_t>()] = [(); core::mem::size_of::<b::pgdval_t>()];

// A plain address expression can denote the link-time high mapping. Match
// rip_rel_ptr() explicitly so every address is in the current 1:1 mapping.
// Symbols remain unprefixed here; the existing %.pi.o rule prefixes them all.
macro_rules! rip_rel_ptr {
    ($symbol:path) => {{
        let ptr: *mut c_void;
        asm!(
            "lea {ptr}, [rip + {symbol}]",
            ptr = out(reg) ptr,
            symbol = sym $symbol,
            options(pure, nomem, nostack, preserves_flags),
        );
        ptr
    }};
}

#[inline(always)]
unsafe fn check_la57_support() -> bool {
    // SAFETY: called in privileged early startup, before secondary CPUs exist.
    unsafe {
        let cr4: c_ulong;
        asm!("mov {}, cr4", out(reg) cr4, options(nostack, preserves_flags));
        if cr4 & b::LUPOS_STARTUP_MAP_CR4_LA57 as c_ulong == 0 {
            return false;
        }
        *rip_rel_ptr!(b::__pgtable_l5_enabled).cast::<c_uint>() = 1;
        *rip_rel_ptr!(b::pgdir_shift).cast::<c_uint>() = 48;
        *rip_rel_ptr!(b::ptrs_per_p4d).cast::<c_uint>() = 512;
        true
    }
}

#[inline(always)]
unsafe fn max_physmem_bits() -> c_uint {
    // map_kernel.c does NOT define USE_EARLY_PGTABLE_L5. Its
    // MAX_PHYSMEM_BITS therefore tests cpu_feature_enabled(X86_FEATURE_LA57),
    // not CR4 or __pgtable_l5_enabled. Startup runs before alternatives are
    // patched; preserve that native fallback and both configured masks.
    // SAFETY: the native boot CPU object is available in the identity mapping.
    unsafe {
        let la57 = if b::LUPOS_STARTUP_MAP_LA57_DISABLED != 0 {
            false
        } else if b::LUPOS_STARTUP_MAP_LA57_REQUIRED != 0 {
            true
        } else {
            let cpu = rip_rel_ptr!(b::boot_cpu_data).cast::<b::cpuinfo_x86>();
            let feature = b::LUPOS_STARTUP_MAP_FEATURE_LA57 as usize;
            // Native cpuinfo_x86 keeps this array in its third anonymous union
            // to preserve unsigned-long alignment. Bindgen retains that union.
            let capability = addr_of!((*cpu).__bindgen_anon_3.x86_capability).cast::<u32>();
            *capability.add(feature / 32) & (1u32 << (feature % 32)) != 0
        };
        if la57 {
            b::LUPOS_STARTUP_MAP_MAX_PHYSMEM_L5 as c_uint
        } else {
            b::LUPOS_STARTUP_MAP_MAX_PHYSMEM_L4 as c_uint
        }
    }
}

#[inline(always)]
unsafe fn sme_get_me_mask() -> c_ulong {
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    // SAFETY: the native encryption mask has a startup-visible __pi_ alias.
    unsafe {
        *rip_rel_ptr!(b::sme_me_mask).cast::<b::u64_>() as c_ulong
    }
    #[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
    {
        0
    }
}

#[inline(always)]
fn pmd_index(address: c_ulong) -> c_ulong {
    (address >> b::LUPOS_STARTUP_MAP_PMD_SHIFT) & (b::LUPOS_STARTUP_MAP_PTRS_PER_PMD as c_ulong - 1)
}

#[inline(always)]
unsafe fn pgd_index(address: c_ulong) -> c_ulong {
    // SAFETY: check_la57_support has performed the native startup update.
    unsafe {
        (address >> *rip_rel_ptr!(b::pgdir_shift).cast::<c_uint>())
            & (b::LUPOS_STARTUP_MAP_PTRS_PER_PGD as c_ulong - 1)
    }
}

#[cold]
#[link_section = ".init.text"]
unsafe fn sme_postprocess_startup(
    bp: *mut b::boot_params,
    pmd: *mut b::pmdval_t,
    p2v_offset: c_ulong,
) -> c_ulong {
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    // SAFETY: these are the original startup encryption entry points; only
    // the mapped .bss..decrypted PMDs have their encryption attribute removed.
    unsafe {
        b::sme_encrypt_kernel(bp);
        if sme_get_me_mask() != 0 {
            let mut paddr = rip_rel_ptr!(b::__start_bss_decrypted) as c_ulong;
            let paddr_end = rip_rel_ptr!(b::__end_bss_decrypted) as c_ulong;
            while paddr < paddr_end {
                // SNP needs a currently valid identity-mapped VA for PVALIDATE.
                b::early_snp_set_memory_shared(
                    paddr,
                    paddr,
                    b::LUPOS_STARTUP_MAP_PTRS_PER_PMD as c_ulong,
                );
                let i = pmd_index(paddr.wrapping_sub(p2v_offset)) as c_int;
                let entry = pmd.offset(i as isize);
                *entry = (*entry).wrapping_sub(sme_get_me_mask());
                paddr = paddr.wrapping_add(b::LUPOS_STARTUP_MAP_PMD_SIZE as c_ulong);
            }
        }
    }
    #[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
    let _ = (bp, pmd, p2v_offset);
    // SAFETY: returns the same mask used to modify the startup CR3 value.
    unsafe { sme_get_me_mask() }
}

/// Relocate the native kernel page tables and construct the switchover mapping.
///
/// # Safety
/// Called by head_64.S in the early 1:1 mapping with its physical-to-virtual
/// offset and boot parameters. The initial tables must have their native layout.
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub(crate) unsafe extern "C" fn __startup_64(
    p2v_offset: c_ulong,
    bp: *mut b::boot_params,
) -> c_ulong {
    // SAFETY: all native symbol accesses use their RIP-relative identity aliases;
    // table dimensions, initial contents, and boot state are established by head_64.S.
    unsafe {
        let early_pgts = rip_rel_ptr!(b::early_dynamic_pgts)
            .cast::<[b::pmd_t; b::LUPOS_STARTUP_MAP_PTRS_PER_PMD as usize]>();
        let physaddr = rip_rel_ptr!(b::_text) as c_ulong;
        let la57 = check_la57_support();

        // Preserve both of the source's non-returning invalid-load checks.
        if physaddr >> max_physmem_bits() != 0 {
            loop {}
        }
        let mut load_delta =
            (b::LUPOS_STARTUP_MAP_START_KERNEL_MAP as c_ulong).wrapping_add(p2v_offset);
        *rip_rel_ptr!(b::phys_base).cast::<c_ulong>() = load_delta;
        if load_delta & !(b::LUPOS_STARTUP_MAP_PMD_MASK as c_ulong) != 0 {
            loop {}
        }
        let va_text = physaddr.wrapping_sub(p2v_offset);
        let va_end = (rip_rel_ptr!(b::_end) as c_ulong).wrapping_sub(p2v_offset);
        load_delta = load_delta.wrapping_add(sme_get_me_mask());

        let pgd = rip_rel_ptr!(b::early_top_pgt).cast::<b::pgdval_t>();
        let kernel_index = pgd_index(b::LUPOS_STARTUP_MAP_START_KERNEL_MAP as c_ulong);
        let entry = pgd.add(kernel_index as usize);
        *entry = (*entry).wrapping_add(load_delta);
        if la57 {
            let p4d = rip_rel_ptr!(b::level4_kernel_pgt).cast::<b::p4dval_t>();
            let entry = p4d.add(b::LUPOS_STARTUP_MAP_MAX_PTRS_PER_P4D as usize - 1);
            *entry = (*entry).wrapping_add(load_delta);
            // _PAGE_TABLE includes the dynamic encryption mask by bitwise OR.
            *pgd.add(pgd_index(b::LUPOS_STARTUP_MAP_START_KERNEL_MAP as c_ulong) as usize) = p4d
                as b::pgdval_t
                | b::LUPOS_STARTUP_MAP_PAGE_TABLE_NOENC as b::pgdval_t
                | sme_get_me_mask();
        }
        let level3 = rip_rel_ptr!(b::level3_kernel_pgt).cast::<b::pud_t>();
        for index in [
            b::LUPOS_STARTUP_MAP_PTRS_PER_PUD as usize - 2,
            b::LUPOS_STARTUP_MAP_PTRS_PER_PUD as usize - 1,
        ] {
            let entry = level3.add(index);
            (*entry).pud = (*entry).pud.wrapping_add(load_delta);
        }
        let fixmap = rip_rel_ptr!(b::level2_fixmap_pgt).cast::<b::pmd_t>();
        let mut i = b::LUPOS_STARTUP_MAP_FIXMAP_PMD_TOP as c_int;
        while i
            > (b::LUPOS_STARTUP_MAP_FIXMAP_PMD_TOP as c_int)
                .wrapping_sub(b::LUPOS_STARTUP_MAP_FIXMAP_PMD_NUM as c_int)
        {
            let entry = fixmap.offset(i as isize);
            (*entry).pmd = (*entry).pmd.wrapping_add(load_delta);
            i = i.wrapping_sub(1);
        }

        // Rows 0 and 1, not adjacent individual PMDs. Native pmd_t wraps one
        // pmdval_t, so each row is a complete native page-table page.
        let pud = early_pgts.cast::<b::pmdval_t>();
        let pmd = early_pgts.add(1).cast::<b::pmdval_t>();
        let next_early_pgt = rip_rel_ptr!(b::next_early_pgt).cast::<c_uint>();
        *next_early_pgt = 2;
        let pgtable_flags =
            (b::LUPOS_STARTUP_MAP_KERNPG_TABLE_NOENC as c_ulong).wrapping_add(sme_get_me_mask());
        if la57 {
            let p4d = early_pgts
                .add(*next_early_pgt as usize)
                .cast::<b::pmdval_t>();
            *next_early_pgt = (*next_early_pgt).wrapping_add(1);
            i = ((physaddr >> *rip_rel_ptr!(b::pgdir_shift).cast::<c_uint>())
                % b::LUPOS_STARTUP_MAP_PTRS_PER_PGD as c_ulong) as c_int;
            let value = (p4d as b::pgdval_t).wrapping_add(pgtable_flags);
            *pgd.offset(i as isize) = value;
            *pgd.offset(i.wrapping_add(1) as isize) = value;
            i = (physaddr >> b::LUPOS_STARTUP_MAP_P4D_SHIFT) as c_int;
            let ptrs = *rip_rel_ptr!(b::ptrs_per_p4d).cast::<c_uint>();
            let value = (pud as b::pgdval_t).wrapping_add(pgtable_flags);
            // PTRS_PER_P4D is unsigned int: C promotes i before this remainder.
            *p4d.add((i as c_uint % ptrs) as usize) = value;
            *p4d.add((i.wrapping_add(1) as c_uint % ptrs) as usize) = value;
        } else {
            i = ((physaddr >> *rip_rel_ptr!(b::pgdir_shift).cast::<c_uint>())
                % b::LUPOS_STARTUP_MAP_PTRS_PER_PGD as c_ulong) as c_int;
            let value = (pud as b::pgdval_t).wrapping_add(pgtable_flags);
            *pgd.offset(i as isize) = value;
            *pgd.offset(i.wrapping_add(1) as isize) = value;
        }
        i = (physaddr >> b::LUPOS_STARTUP_MAP_PUD_SHIFT) as c_int;
        let value = (pmd as b::pudval_t).wrapping_add(pgtable_flags);
        *pud.offset((i % b::LUPOS_STARTUP_MAP_PTRS_PER_PUD as c_int) as isize) = value;
        *pud.offset((i.wrapping_add(1) % b::LUPOS_STARTUP_MAP_PTRS_PER_PUD as c_int) as isize) =
            value;

        // The identity entries must not carry _PAGE_GLOBAL. Keep the original
        // unsigned-long sum, int narrowing, and signed remainder at the PMD edge.
        let pmd_entry = ((b::LUPOS_STARTUP_MAP_PAGE_KERNEL_LARGE_EXEC as b::pmdval_t)
            & !(b::LUPOS_STARTUP_MAP_PAGE_GLOBAL as b::pmdval_t))
            .wrapping_add(sme_get_me_mask())
            .wrapping_add(physaddr);
        let pmd_size = b::LUPOS_STARTUP_MAP_PMD_SIZE as c_ulong;
        let nr_pmds = va_end.wrapping_sub(va_text).wrapping_add(pmd_size - 1) / pmd_size;
        i = 0;
        while (i as c_ulong) < nr_pmds {
            let idx =
                (i as c_ulong).wrapping_add(physaddr >> b::LUPOS_STARTUP_MAP_PMD_SHIFT) as c_int;
            *pmd.offset((idx % b::LUPOS_STARTUP_MAP_PTRS_PER_PMD as c_int) as isize) =
                pmd_entry.wrapping_add((i as c_ulong).wrapping_mul(pmd_size));
            i = i.wrapping_add(1);
        }

        // Only fix up present kernel-image PMDs. Invalidate every PMD outside
        // the image so speculation cannot touch unvalidated firmware regions.
        let pmd = rip_rel_ptr!(b::level2_kernel_pgt).cast::<b::pmdval_t>();
        i = 0;
        while (i as c_ulong) < pmd_index(va_text) {
            *pmd.offset(i as isize) &= !(b::LUPOS_STARTUP_MAP_PAGE_PRESENT as b::pmdval_t);
            i = i.wrapping_add(1);
        }
        while (i as c_ulong) <= pmd_index(va_end) {
            let entry = pmd.offset(i as isize);
            if *entry & b::LUPOS_STARTUP_MAP_PAGE_PRESENT as b::pmdval_t != 0 {
                *entry = (*entry).wrapping_add(load_delta);
            }
            i = i.wrapping_add(1);
        }
        while i < b::LUPOS_STARTUP_MAP_PTRS_PER_PMD as c_int {
            *pmd.offset(i as isize) &= !(b::LUPOS_STARTUP_MAP_PAGE_PRESENT as b::pmdval_t);
            i = i.wrapping_add(1);
        }
        sme_postprocess_startup(bp, pmd, p2v_offset)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
