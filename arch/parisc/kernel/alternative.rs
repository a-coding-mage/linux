// SPDX-License-Identifier: GPL-2.0
/*
 *    Alternative live-patching for parisc.
 *    Copyright (C) 2018 Helge Deller <deller@gmx.de>
 *
 */

// External declarations supplied by the kernel headers and other translation units.

static mut no_alternatives: i32 = 0;

unsafe fn setup_no_alternatives(_str: *mut core::ffi::c_char) -> i32 {
    no_alternatives = 1;
    1
}

// __setup("no-alternatives", setup_no_alternatives);

pub unsafe fn apply_alternatives(
    mut start: *mut alt_instr,
    end: *mut alt_instr,
    module_name: *const core::ffi::c_char,
) {
    let mut entry: *mut alt_instr;
    let mut index: i32 = 0;
    let mut applied: i32 = 0;
    let num_cpus: i32 = num_present_cpus();
    let cond_check: u16 = ALT_COND_ALWAYS
        | if num_cpus == 1 { ALT_COND_NO_SMP } else { 0 }
        | if (*cache_info).dc_size == 0 { ALT_COND_NO_DCACHE } else { 0 }
        | if (*cache_info).ic_size == 0 { ALT_COND_NO_ICACHE } else { 0 }
        | if running_on_qemu != 0 { ALT_COND_RUN_ON_QEMU } else { 0 }
        | if split_tlb == 0 { ALT_COND_NO_SPLIT_TLB } else { 0 }
        /*
         * If the PDC_MODEL capabilities has Non-coherent IO-PDIR bit
         * set (bit #61, big endian), we have to flush and sync every
         * time IO-PDIR is changed in Ike/Astro.
         */
        | if (*boot_cpu_data).cpu_type > pcxw_
            && ((*boot_cpu_data).pdc.capabilities & PDC_MODEL_IOPDIR_FDC) == 0
        {
            ALT_COND_NO_IOC_FDC
        } else {
            0
        };

    entry = start;
    while entry < end {
        let from: *mut u32 = ((&mut (*entry).orig_offset as *mut _ as usize)
            .wrapping_add((*entry).orig_offset as isize as usize)) as *mut u32;
        let mut replacement: u32 = (*entry).replacement;
        let mut cond: u16 = (*entry).cond;
        let mut len: i16 = (*entry).len;

        // WARN_ON(!cond);

        if (cond & ALT_COND_ALWAYS) == 0 && no_alternatives != 0 {
            entry = entry.add(1);
            index += 1;
            continue;
        }

        // pr_debug("Check %d: Cond 0x%x, Replace %02d instructions @ 0x%px with 0x%08x\n", ...);

        /* Bounce out if none of the conditions are true. */
        if (cond & cond_check) == 0 {
            entry = entry.add(1);
            index += 1;
            continue;
        }

        /* Want to replace pdtlb by a pdtlb,l instruction? */
        if replacement == INSN_PxTLB {
            replacement = *from;
            if (*boot_cpu_data).cpu_type >= pcxu /* >= pa2.0 ? */ {
                replacement |= 1 << 10; /* set el bit */
            }
        }

        /*
         * Replace instruction with NOPs?
         * For long distance insert a branch instruction instead.
         */
        if replacement == INSN_NOP && len > 1 {
            replacement = 0xe8000002u32.wrapping_add((len as u32 - 2).wrapping_mul(8));
            /* "b,n .+8" */
        }

        // pr_debug("ALTERNATIVE %3d: Cond %2x, Replace %2d instructions to 0x%08x @ 0x%px (%pS)\n", ...);

        if len < 0 {
            /* Replace multiple instruction by new code */
            let source: *const u32 = ((&(*entry).replacement as *const _ as usize)
                .wrapping_add((*entry).replacement as usize)) as *const u32;
            len = -len;
            core::ptr::copy_nonoverlapping(source, from, 4usize.wrapping_mul(len as usize));
        } else {
            /* Replace by one instruction */
            *from = replacement;
        }
        applied += 1;
        entry = entry.add(1);
        index += 1;
    }

    // pr_info("%s%salternatives: applied %d out of %d patches\n", ...);
    let _ = (module_name, applied, index, cond_check, start);
}

pub unsafe fn apply_alternatives_all() {
    set_kernel_text_rw(1);

    apply_alternatives(
        &mut __alt_instructions as *mut _ as *mut alt_instr,
        &mut __alt_instructions_end as *mut _ as *mut alt_instr,
        core::ptr::null(),
    );

    if (*cache_info).dc_size == 0 && (*cache_info).ic_size == 0 {
        // pr_info("alternatives: optimizing cache-flushes.\n");
        static_branch_disable(&mut parisc_has_cache);
    }
    if (*cache_info).dc_size == 0 {
        static_branch_disable(&mut parisc_has_dcache);
    }
    if (*cache_info).ic_size == 0 {
        static_branch_disable(&mut parisc_has_icache);
    }

    set_kernel_text_rw(0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
