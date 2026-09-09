// SPDX-License-Identifier: GPL-2.0
/*
 * jump label x86 support
 *
 * Copyright (C) 2009 Jason Baron <jbaron@redhat.com>
 *
 */

#[repr(C)]
pub struct jump_label_patch {
    pub code: *const core::ffi::c_void,
    pub size: core::ffi::c_int,
}

pub unsafe fn arch_jump_entry_size(entry: *mut jump_entry) -> core::ffi::c_int {
    let mut insn: insn = core::mem::zeroed();

    insn_decode_kernel(&mut insn, jump_entry_code(entry) as *mut core::ffi::c_void);
    BUG_ON!(insn.length != 2 && insn.length != 5);

    insn.length
}

unsafe fn __jump_label_patch(
    entry: *mut jump_entry,
    type_: jump_label_type,
) -> jump_label_patch {
    let expect: *const core::ffi::c_void;
    let mut code: *const core::ffi::c_void;
    let nop: *const core::ffi::c_void;
    let addr: *const core::ffi::c_void;
    let dest: *const core::ffi::c_void;
    let size: core::ffi::c_int;

    addr = jump_entry_code(entry) as *const core::ffi::c_void;
    dest = jump_entry_target(entry) as *const core::ffi::c_void;

    size = arch_jump_entry_size(entry);
    match size {
        JMP8_INSN_SIZE => {
            code = text_gen_insn(JMP8_INSN_OPCODE, addr, dest);
            nop = x86_nops[size as usize];
        }
        JMP32_INSN_SIZE => {
            code = text_gen_insn(JMP32_INSN_OPCODE, addr, dest);
            nop = x86_nops[size as usize];
        }
        _ => BUG!(),
    }

    if type_ == JUMP_LABEL_JMP {
        expect = nop;
    } else {
        expect = code;
    }

    if memcmp(addr, expect, size as usize) != 0 {
        /*
         * The location is not an op that we were expecting.
         * Something went wrong. Crash the box, as something could be
         * corrupting the kernel.
         */
        pr_crit!(
            "jump_label: Fatal kernel bug, unexpected op at %pS [%p] (%5ph != %5ph)) size:%d type:%d\n",
            addr, addr, addr, expect, size, type_
        );
        BUG!();
    }

    if type_ == JUMP_LABEL_NOP {
        code = nop;
    }

    jump_label_patch { code, size }
}

#[inline(always)]
unsafe fn __jump_label_transform(
    entry: *mut jump_entry,
    type_: jump_label_type,
    init: core::ffi::c_int,
) {
    let jlp = __jump_label_patch(entry, type_);

    /*
     * As long as only a single processor is running and the code is still
     * not marked as RO, text_poke_early() can be used; Checking that
     * system_state is SYSTEM_BOOTING guarantees it. It will be set to
     * SYSTEM_SCHEDULING before other cores are awaken and before the
     * code is write-protected.
     *
     * At the time the change is being done, just ignore whether we
     * are doing nop -> jump or jump -> nop transition, and assume
     * always nop being the 'currently valid' instruction
     */
    if init != 0 || system_state == SYSTEM_BOOTING {
        text_poke_early(jump_entry_code(entry) as *mut core::ffi::c_void, jlp.code, jlp.size);
        return;
    }

    smp_text_poke_single(
        jump_entry_code(entry) as *mut core::ffi::c_void,
        jlp.code,
        jlp.size,
        core::ptr::null_mut(),
    );
}

unsafe fn jump_label_transform(
    entry: *mut jump_entry,
    type_: jump_label_type,
    init: core::ffi::c_int,
) {
    mutex_lock(&text_mutex);
    __jump_label_transform(entry, type_, init);
    mutex_unlock(&text_mutex);
}

pub unsafe fn arch_jump_label_transform(entry: *mut jump_entry, type_: jump_label_type) {
    jump_label_transform(entry, type_, 0);
}

pub unsafe fn arch_jump_label_transform_queue(
    entry: *mut jump_entry,
    type_: jump_label_type,
) -> bool {
    let jlp: jump_label_patch;

    if system_state == SYSTEM_BOOTING {
        /*
         * Fallback to the non-batching mode.
         */
        arch_jump_label_transform(entry, type_);
        return true;
    }

    mutex_lock(&text_mutex);
    jlp = __jump_label_patch(entry, type_);
    smp_text_poke_batch_add(
        jump_entry_code(entry) as *mut core::ffi::c_void,
        jlp.code,
        jlp.size,
        core::ptr::null_mut(),
    );
    mutex_unlock(&text_mutex);
    true
}

pub unsafe fn arch_jump_label_transform_apply() {
    mutex_lock(&text_mutex);
    smp_text_poke_batch_finish();
    mutex_unlock(&text_mutex);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
