// SPDX-License-Identifier: GPL-2.0-only
/*
 * alternative runtime patching
 * inspired by the x86 version
 *
 * Copyright (C) 2014 ARM Ltd.
 */

// Dependencies supplied by the kernel translation unit are intentionally external.

use core::ffi::c_void;

static mut ALL_ALTERNATIVES_APPLIED: core::ffi::c_int = 0;

#[repr(C)]
struct AltRegion {
    begin: *mut alt_instr,
    end: *mut alt_instr,
}

static mut APPLIED_ALTERNATIVES: [usize; (ARM64_NCAPS + usize::BITS as usize - 1) / usize::BITS as usize] =
    [0; (ARM64_NCAPS + usize::BITS as usize - 1) / usize::BITS as usize];

#[inline]
unsafe fn alt_orig_ptr(a: *mut alt_instr) -> *mut __le32 {
    (a.cast::<u8>().offset((*a).orig_offset as isize)).cast()
}

#[inline]
unsafe fn alt_repl_ptr(a: *mut alt_instr) -> *mut __le32 {
    (a.cast::<u8>().offset((*a).alt_offset as isize)).cast()
}

#[inline]
unsafe fn alt_cap(a: *mut alt_instr) -> u16 {
    (*a).cpucap & !ARM64_CB_BIT
}

#[inline]
unsafe fn alt_has_cb(a: *mut alt_instr) -> bool {
    ((*a).cpucap & ARM64_CB_BIT) != 0
}

pub unsafe fn alternative_is_applied(cpucap: u16) -> bool {
    if cpucap >= ARM64_NCAPS {
        // WARN_ON(cpucap >= ARM64_NCAPS)
        return false;
    }
    test_bit(cpucap as usize, APPLIED_ALTERNATIVES.as_ptr())
}

#[inline]
unsafe fn branch_insn_requires_update(alt: *mut alt_instr, pc: usize) -> bool {
    let replptr = alt_repl_ptr(alt) as usize;
    !(pc >= replptr && pc <= replptr + (*alt).alt_len as usize)
}

#[inline]
unsafe fn get_alt_insn(alt: *mut alt_instr, insnptr: *mut __le32, altinsnptr: *mut __le32) -> u32 {
    let mut insn = le32_to_cpu(*altinsnptr);
    if aarch64_insn_is_branch_imm(insn) {
        let mut offset = aarch64_get_branch_offset(insn);
        let target = (altinsnptr as usize).wrapping_add(offset as usize);
        if branch_insn_requires_update(alt, target) {
            offset = target.wrapping_sub(insnptr as usize) as i32;
            insn = aarch64_set_branch_offset(insn, offset);
        }
    } else if aarch64_insn_is_adrp(insn) {
        let orig_offset = aarch64_insn_adrp_get_offset(insn);
        let target = align_down(altinsnptr as usize, SZ_4K) + orig_offset as usize;
        let new_offset = target.wrapping_sub(align_down(insnptr as usize, SZ_4K)) as i32;
        insn = aarch64_insn_adrp_set_offset(insn, new_offset);
    } else if aarch64_insn_uses_literal(insn) {
        // BUG()
        panic!("BUG")
    }
    insn
}

#[inline]
unsafe fn align_down(x: usize, a: usize) -> usize { x & !(a - 1) }

unsafe fn patch_alternative(alt: *mut alt_instr, origptr: *mut __le32, updptr: *mut __le32, nr_inst: i32) {
    let replptr = alt_repl_ptr(alt);
    for i in 0..nr_inst {
        let insn = get_alt_insn(alt, origptr.add(i as usize), replptr.add(i as usize));
        *updptr.add(i as usize) = cpu_to_le32(insn);
    }
}

unsafe fn clean_dcache_range_nopatch(start: u64, end: u64) {
    let ctr_el0 = arm64_ftr_reg_ctrel0.sys_val;
    let d_size = 4u64 << cpuid_feature_extract_unsigned_field(ctr_el0, CTR_EL0_DminLine_SHIFT);
    let mut cur = start & !(d_size - 1);
    loop {
        core::arch::asm!("dc civac, {0}", in(reg) cur, options(nostack));
        cur = cur.wrapping_add(d_size);
        if cur >= end { break; }
    }
}

unsafe fn __apply_alternatives(region: *const AltRegion, is_module: bool, cpucap_mask: *mut usize) -> i32 {
    let mut alt = (*region).begin;
    while alt < (*region).end {
        let cap = alt_cap(alt) as usize;
        if !test_bit(cap, cpucap_mask) || !cpus_have_cap(cap) { alt = alt.add(1); continue; }
        if alt_has_cb(alt) { BUG_ON((*alt).alt_len != 0); } else { BUG_ON((*alt).alt_len != (*alt).orig_len); }
        let origptr = alt_orig_ptr(alt);
        let updptr = if is_module { origptr } else { lm_alias(origptr) };
        let nr_inst = (*alt).orig_len as i32 / AARCH64_INSN_SIZE;
        let alt_cb: alternative_cb_t;
        if alt_has_cb(alt) {
            alt_cb = alt_repl_ptr(alt);
            if is_module && !core_kernel_text(alt_cb as usize) { return -ENOEXEC; }
        } else { alt_cb = patch_alternative; }
        alt_cb(alt, origptr, updptr, nr_inst);
        if !is_module { clean_dcache_range_nopatch(origptr as u64, origptr.add(nr_inst as usize) as u64); }
        alt = alt.add(1);
    }
    if !is_module {
        dsb(ish); icache_inval_all_pou(); isb();
        bitmap_or(APPLIED_ALTERNATIVES.as_mut_ptr(), APPLIED_ALTERNATIVES.as_ptr(), cpucap_mask, ARM64_NCAPS);
        bitmap_and(APPLIED_ALTERNATIVES.as_mut_ptr(), APPLIED_ALTERNATIVES.as_ptr(), system_cpucaps, ARM64_NCAPS);
    }
    0
}

unsafe fn apply_alternatives_vdso() {
    let mut all_capabilities = [usize::MAX; (ARM64_NCAPS + usize::BITS as usize - 1) / usize::BITS as usize];
    let hdr = vdso_start as *const elf64_hdr;
    let shdr = (hdr as *const u8).add((*hdr).e_shoff as usize) as *const elf64_shdr;
    let alt = find_section(hdr, shdr, ".altinstructions\0".as_ptr() as *const i8);
    if alt.is_null() { return; }
    let region = AltRegion { begin: (hdr as *const u8).add((*alt).sh_offset as usize) as *mut alt_instr, end: (hdr as *const u8).add((*alt).sh_offset as usize + (*alt).sh_size as usize) as *mut alt_instr };
    __apply_alternatives(&region, false, all_capabilities.as_mut_ptr());
}

static mut KERNEL_ALTERNATIVES: AltRegion = AltRegion { begin: __alt_instructions as *mut alt_instr, end: __alt_instructions_end as *mut alt_instr };

unsafe fn __apply_alternatives_multi_stop(_unused: *mut c_void) -> i32 {
    if smp_processor_id() != 0 { while ALL_ALTERNATIVES_APPLIED == 0 { cpu_relax(); } isb(); }
    else {
        let mut remaining_capabilities = [0usize; (ARM64_NCAPS + usize::BITS as usize - 1) / usize::BITS as usize];
        bitmap_complement(remaining_capabilities.as_mut_ptr(), boot_cpucaps, ARM64_NCAPS);
        BUG_ON(ALL_ALTERNATIVES_APPLIED != 0);
        __apply_alternatives(&KERNEL_ALTERNATIVES, false, remaining_capabilities.as_mut_ptr());
        ALL_ALTERNATIVES_APPLIED = 1;
    }
    0
}

pub unsafe fn apply_alternatives_all() { pr_info!("applying system-wide alternatives\n"); apply_alternatives_vdso(); stop_machine(__apply_alternatives_multi_stop, core::ptr::null_mut(), cpu_online_mask); }

pub unsafe fn apply_boot_alternatives() { WARN_ON(smp_processor_id() != 0); pr_info!("applying boot alternatives\n"); __apply_alternatives(&KERNEL_ALTERNATIVES, false, boot_cpucaps); }

#[cfg(CONFIG_MODULES)]
pub unsafe fn apply_alternatives_module(start: *mut c_void, length: usize) -> i32 { let region = AltRegion { begin: start as *mut alt_instr, end: (start as *mut u8).add(length) as *mut alt_instr }; let mut caps = [usize::MAX; (ARM64_NCAPS + usize::BITS as usize - 1) / usize::BITS as usize]; __apply_alternatives(&region, true, caps.as_mut_ptr()) }

pub unsafe fn alt_cb_patch_nops(_alt: *mut alt_instr, _origptr: *mut __le32, updptr: *mut __le32, nr_inst: i32) { for i in 0..nr_inst { *updptr.add(i as usize) = cpu_to_le32(aarch64_insn_gen_nop()); } }

// EXPORT_SYMBOL(alt_cb_patch_nops)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
