// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Kernel Probes Jump Optimization (Optprobes)
 *
 * Copyright (C) IBM Corporation, 2002, 2004
 * Copyright (C) Hitachi Ltd., 2012
 * Copyright (C) Huawei Inc., 2014
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

/*
 * See register_usage_flags. If the probed instruction doesn't use PC,
 * we can copy it into template and have it executed directly without
 * simulation or emulation.
 */
pub const ARM_REG_PC: usize = 15;

#[inline]
pub unsafe fn can_kprobe_direct_exec(m: *const usize) -> bool {
    !test_bit(ARM_REG_PC, m)
}

// The following symbols are provided by the ARM assembly template in the
// original implementation. The assembly itself is kept as an external ABI.
extern "C" {
    static optprobe_template_entry: u8;
    static mut optprobe_template_sub_sp: u8;
    static mut optprobe_template_add_sp: u8;
    static mut optprobe_template_restore_begin: u8;
    static mut optprobe_template_restore_orig_insn: u8;
    static mut optprobe_template_restore_end: u8;
    static mut optprobe_template_val: u8;
    static mut optprobe_template_call: u8;
    static mut optprobe_template_end: u8;
}

#[inline]
unsafe fn tmpl_idx(p: *const u8) -> usize {
    (p as usize - (&optprobe_template_entry as *const u8 as usize))
        / core::mem::size_of::<usize>()
}

#[inline]
unsafe fn tmpl_val_idx() -> usize { tmpl_idx(&optprobe_template_val) }
#[inline]
unsafe fn tmpl_call_idx() -> usize { tmpl_idx(&optprobe_template_call) }
#[inline]
unsafe fn tmpl_end_idx() -> usize { tmpl_idx(&optprobe_template_end) }
#[inline]
unsafe fn tmpl_add_sp() -> usize { tmpl_idx(&optprobe_template_add_sp) }
#[inline]
unsafe fn tmpl_sub_sp() -> usize { tmpl_idx(&optprobe_template_sub_sp) }
#[inline]
unsafe fn tmpl_restore_begin() -> usize { tmpl_idx(&optprobe_template_restore_begin) }
#[inline]
unsafe fn tmpl_restore_orign_insn() -> usize { tmpl_idx(&optprobe_template_restore_orig_insn) }
#[inline]
unsafe fn tmpl_restore_end() -> usize { tmpl_idx(&optprobe_template_restore_end) }

pub unsafe fn arch_prepared_optinsn(optinsn: *const arch_optimized_insn) -> i32 {
    if (*optinsn).insn.is_null() { 0 } else { 1 }
}

pub unsafe fn arch_check_optimized_kprobe(_op: *const optimized_kprobe) -> i32 { 0 }

unsafe fn can_optimize(kp: *const kprobe) -> i32 {
    if (*kp).ainsn.stack_space < 0 { return 0; }
    if (*kp).ainsn.stack_space > 255 - core::mem::size_of::<pt_regs>() as i32 { return 0; }
    1
}

unsafe fn __arch_remove_optimized_kprobe(op: *mut optimized_kprobe, dirty: i32) {
    if !(*op).optinsn.insn.is_null() {
        free_optinsn_slot((*op).optinsn.insn, dirty);
        (*op).optinsn.insn = core::ptr::null_mut();
    }
}

unsafe extern "C" fn optimized_callback(op: *mut optimized_kprobe, regs: *mut pt_regs) {
    let p = &mut (*op).kp as *mut kprobe;
    (*regs).ARM_pc = (*op).kp.addr as usize;
    (*regs).ARM_ORIG_r0 = !0usize;
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    let kcb = get_kprobe_ctlblk();
    if kprobe_running() != 0 {
        kprobes_inc_nmissed_count(&mut (*op).kp);
    } else {
        __this_cpu_write_current_kprobe(&mut (*op).kp);
        (*kcb).kprobe_status = KPROBE_HIT_ACTIVE;
        opt_pre_handler(&mut (*op).kp, regs);
        __this_cpu_write_current_kprobe(core::ptr::null_mut());
    }
    if !(*p).ainsn.kprobe_direct_exec {
        ((*p).ainsn.insn_singlestep)((*p).opcode, &mut (*p).ainsn, regs);
    }
    local_irq_restore(flags);
}

pub unsafe fn arch_prepare_optimized_kprobe(op: *mut optimized_kprobe, orig: *mut kprobe) -> i32 {
    if can_optimize(orig) == 0 { return -EILSEQ; }
    let code = get_optinsn_slot();
    if code.is_null() { return -ENOMEM; }
    let rel_chk = ((code as isize - (*orig).addr as isize + 8) as usize) & 0xfe000003;
    if rel_chk != 0 && rel_chk != 0xfe000000 {
        free_optinsn_slot(code, 0);
        return -ERANGE;
    }
    core::ptr::copy_nonoverlapping(
        &optprobe_template_entry as *const u8 as *const usize,
        code as *mut usize,
        tmpl_end_idx(),
    );
    BUG_ON((*orig).ainsn.stack_space < 0);
    let stack_protect = core::mem::size_of::<pt_regs>() as i32 + (*orig).ainsn.stack_space;
    BUG_ON(stack_protect > 255);
    (code as *mut usize).add(tmpl_sub_sp()).write(__opcode_to_mem_arm(0xe24dd000 | stack_protect as u32));
    (code as *mut usize).add(tmpl_add_sp()).write(__opcode_to_mem_arm(0xe28d3000 | stack_protect as u32));
    (code as *mut usize).add(tmpl_val_idx()).write(op as usize);
    (code as *mut usize).add(tmpl_call_idx()).write(optimized_callback as usize);
    (*orig).ainsn.kprobe_direct_exec = false;
    if can_kprobe_direct_exec(&(*orig).ainsn.register_usage_flags) {
        let final_branch = arm_gen_branch((code as usize) + tmpl_restore_end() * core::mem::size_of::<usize>(), (*op).kp.addr as usize + 4);
        if final_branch != 0 {
            (code as *mut usize).add(tmpl_restore_begin()).write(__opcode_to_mem_arm(0xe89d7fff));
            (code as *mut usize).add(tmpl_restore_orign_insn()).write(__opcode_to_mem_arm((*orig).opcode));
            (code as *mut usize).add(tmpl_restore_end()).write(__opcode_to_mem_arm(final_branch));
            (*orig).ainsn.kprobe_direct_exec = true;
        }
    }
    flush_icache_range(code as usize, code as usize + tmpl_end_idx() * core::mem::size_of::<kprobe_opcode_t>());
    (*op).optinsn.insn = code;
    0
}

pub unsafe fn arch_optimize_kprobes(oplist: *mut list_head) {
    let mut op: *mut optimized_kprobe;
    let mut tmp: *mut optimized_kprobe;
    list_for_each_entry_safe!(op, tmp, oplist, list, {
        let mut insn: usize;
        WARN_ON(kprobe_disabled(&mut (*op).kp));
        core::ptr::copy_nonoverlapping((*op).kp.addr, (*op).optinsn.copied_insn.as_mut_ptr(), RELATIVEJUMP_SIZE);
        insn = arm_gen_branch((*op).kp.addr as usize, (*op).optinsn.insn as usize);
        BUG_ON(insn == 0);
        insn = (__mem_to_opcode_arm((*op).optinsn.copied_insn[0]) & 0xf0000000) | (insn & 0x0fffffff);
        kprobes_remove_breakpoint((*op).kp.addr, insn);
        list_del_init(&mut (*op).list);
    });
}

pub unsafe fn arch_unoptimize_kprobe(op: *mut optimized_kprobe) { arch_arm_kprobe(&mut (*op).kp); }

pub unsafe fn arch_unoptimize_kprobes(oplist: *mut list_head, done_list: *mut list_head) {
    let mut op: *mut optimized_kprobe;
    let mut tmp: *mut optimized_kprobe;
    list_for_each_entry_safe!(op, tmp, oplist, list, {
        arch_unoptimize_kprobe(op);
        list_move(&mut (*op).list, done_list);
    });
}

pub unsafe fn arch_within_optimized_kprobe(op: *const optimized_kprobe, addr: *mut kprobe_opcode_t) -> i32 {
    if (*op).kp.addr <= addr && (*op).kp.addr.add(RELATIVEJUMP_SIZE / core::mem::size_of::<kprobe_opcode_t>()) > addr { 1 } else { 0 }
}

pub unsafe fn arch_remove_optimized_kprobe(op: *mut optimized_kprobe) { __arch_remove_optimized_kprobe(op, 1); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
