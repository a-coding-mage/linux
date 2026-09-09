// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm64/kernel/probes/decode-insn.c
 *
 * Copyright (C) 2013 Linaro Limited.
 */

// Dependencies supplied by the surrounding kernel translation unit are intentionally unresolved here.

unsafe fn aarch64_insn_is_steppable(insn: u32) -> bool {
    /*
     * Branch instructions will write a new value into the PC which is
     * likely to be relative to the XOL address and therefore invalid.
     * Deliberate generation of an exception during stepping is also not
     * currently safe. Lastly, MSR instructions can do any number of nasty
     * things we can't handle during single-stepping.
     */
    if aarch64_insn_is_class_branch_sys(insn) {
        if aarch64_insn_is_branch(insn)
            || aarch64_insn_is_msr_imm(insn)
            || aarch64_insn_is_msr_reg(insn)
            || aarch64_insn_is_exception(insn)
            || aarch64_insn_is_eret(insn)
            || aarch64_insn_is_eret_auth(insn)
        {
            return false;
        }

        /*
         * The MRS instruction may not return a correct value when
         * executing in the single-stepping environment. We do make one
         * exception, for reading the DAIF bits.
         */
        if aarch64_insn_is_mrs(insn) {
            return aarch64_insn_extract_system_reg(insn) != AARCH64_INSN_SPCLREG_DAIF;
        }

        /*
         * The HINT instruction is steppable only if it is in whitelist
         * and the rest of other such instructions are blocked for
         * single stepping as they may cause exception or other
         * unintended behaviour.
         */
        if aarch64_insn_is_hint(insn) {
            return aarch64_insn_is_steppable_hint(insn);
        }

        return true;
    }

    /*
     * Instructions which load PC relative literals are not going to work
     * when executed from an XOL slot. Instructions doing an exclusive
     * load/store are not going to complete successfully when single-step
     * exception handling happens in the middle of the sequence. Memory
     * copy/set instructions require that all three instructions be placed
     * consecutively in memory.
     */
    if aarch64_insn_uses_literal(insn)
        || aarch64_insn_is_exclusive(insn)
        || aarch64_insn_is_mops(insn)
    {
        return false;
    }

    true
}

/* Return:
 *   INSN_REJECTED     If instruction is one not allowed to kprobe,
 *   INSN_GOOD         If instruction is supported and uses instruction slot,
 *   INSN_GOOD_NO_SLOT If instruction is supported but doesn't use its slot.
 */
pub unsafe fn arm_probe_decode_insn(insn: u32, api: *mut arch_probe_insn) -> probe_insn {
    /*
     * While 'nop' instruction can execute in the out-of-line slot,
     * simulating them in breakpoint handling offers better performance.
     */
    if aarch64_insn_is_nop(insn) {
        (*api).handler = simulate_nop;
        return INSN_GOOD_NO_SLOT;
    }

    /*
     * Instructions reading or modifying the PC won't work from the XOL
     * slot.
     */
    if aarch64_insn_is_steppable(insn) {
        return INSN_GOOD;
    }

    if aarch64_insn_is_bcond(insn) {
        (*api).handler = simulate_b_cond;
    } else if aarch64_insn_is_cbz(insn) || aarch64_insn_is_cbnz(insn) {
        (*api).handler = simulate_cbz_cbnz;
    } else if aarch64_insn_is_tbz(insn) || aarch64_insn_is_tbnz(insn) {
        (*api).handler = simulate_tbz_tbnz;
    } else if aarch64_insn_is_adr_adrp(insn) {
        (*api).handler = simulate_adr_adrp;
    } else if aarch64_insn_is_b(insn) || aarch64_insn_is_bl(insn) {
        (*api).handler = simulate_b_bl;
    } else if aarch64_insn_is_br(insn) || aarch64_insn_is_blr(insn) {
        (*api).handler = simulate_br_blr;
    } else if aarch64_insn_is_ret(insn) {
        (*api).handler = simulate_ret;
    } else {
        /*
         * Instruction cannot be stepped out-of-line and we don't
         * (yet) simulate it.
         */
        return INSN_REJECTED;
    }

    INSN_GOOD_NO_SLOT
}

// CONFIG_KPROBES conditional preserved from the C implementation.
#[cfg(CONFIG_KPROBES)]
unsafe fn is_probed_address_atomic(
    mut scan_start: *mut kprobe_opcode_t,
    scan_end: *mut kprobe_opcode_t,
) -> bool {
    while scan_start >= scan_end {
        /*
         * atomic region starts from exclusive load and ends with
         * exclusive store.
         */
        if aarch64_insn_is_store_ex(le32_to_cpu(*scan_start)) {
            return false;
        } else if aarch64_insn_is_load_ex(le32_to_cpu(*scan_start)) {
            return true;
        }
        scan_start = scan_start.sub(1);
    }

    false
}

#[cfg(CONFIG_KPROBES)]
pub unsafe fn arm_kprobe_decode_insn(
    addr: *mut kprobe_opcode_t,
    asi: *mut arch_specific_insn,
) -> probe_insn {
    let decoded: probe_insn;
    let insn = le32_to_cpu(*addr);
    let mut scan_end: *mut kprobe_opcode_t = core::ptr::null_mut();
    let mut size: usize = 0;
    let mut offset: usize = 0;
    let api = &mut (*asi).api as *mut arch_probe_insn;

    if aarch64_insn_is_ldr_lit(insn) {
        (*api).handler = simulate_ldr_literal;
        decoded = INSN_GOOD_NO_SLOT;
    } else if aarch64_insn_is_ldrsw_lit(insn) {
        (*api).handler = simulate_ldrsw_literal;
        decoded = INSN_GOOD_NO_SLOT;
    } else {
        decoded = arm_probe_decode_insn(insn, api);
    }

    /*
     * If there's a symbol defined in front of and near enough to
     * the probe address assume it is the entry point to this
     * code and use it to further limit how far back we search
     * when determining if we're in an atomic sequence. If we could
     * not find any symbol skip the atomic test altogether as we
     * could otherwise end up searching irrelevant text/literals.
     * KPROBES depends on KALLSYMS so this last case should never
     * happen.
     */
    if kallsyms_lookup_size_offset(addr as usize, &mut size, &mut offset) {
        if offset < MAX_ATOMIC_CONTEXT_SIZE * core::mem::size_of::<kprobe_opcode_t>() {
            scan_end = addr.sub(offset / core::mem::size_of::<kprobe_opcode_t>());
        } else {
            scan_end = addr.sub(MAX_ATOMIC_CONTEXT_SIZE);
        }
    }

    if decoded != INSN_REJECTED
        && !scan_end.is_null()
        && is_probed_address_atomic(addr.sub(1), scan_end)
    {
        return INSN_REJECTED;
    }

    decoded
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
