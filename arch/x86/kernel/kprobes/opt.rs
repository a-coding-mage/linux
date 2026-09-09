// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Kernel Probes Jump Optimization (Optprobes)
 *
 * Rust translation of the architecture implementation source.
 * C includes and the assembly template are supplied by the surrounding kernel build.
 */

// The original file emits optprobe_template_entry/clac/val/call/end in .rodata
// using architecture-specific inline assembly. These symbols are external here.
extern "C" {
    static optprobe_template_entry: u8;
    static optprobe_template_clac: u8;
    static optprobe_template_val: u8;
    static optprobe_template_call: u8;
    static optprobe_template_end: u8;
}

const TMPL_CLAC_IDX: isize = unsafe { &optprobe_template_clac as *const u8 as isize - &optprobe_template_entry as *const u8 as isize };
const TMPL_MOVE_IDX: isize = unsafe { &optprobe_template_val as *const u8 as isize - &optprobe_template_entry as *const u8 as isize };
const TMPL_CALL_IDX: isize = unsafe { &optprobe_template_call as *const u8 as isize - &optprobe_template_entry as *const u8 as isize };
const TMPL_END_IDX: isize = unsafe { &optprobe_template_end as *const u8 as isize - &optprobe_template_entry as *const u8 as isize };

pub unsafe extern "C" fn __recover_optprobed_insn(buf: *mut kprobe_opcode_t, addr: c_ulong) -> c_ulong {
    let mut op: *mut optimized_kprobe;
    let mut kp: *mut kprobe;
    let mut offs: c_long;
    let mut i = 0;
    while i < JMP32_INSN_SIZE {
        kp = get_kprobe((addr as *mut c_void).offset(-(i as isize)));
        if !kp.is_null() && kprobe_optimized(kp) {
            op = container_of_optimized(kp);
            if list_empty(&(*op).list) || optprobe_queued_unopt(op) { break; }
        }
        i += 1;
    }
    if i == JMP32_INSN_SIZE { return addr; }
    if copy_from_kernel_nofault(buf as *mut c_void, addr as *const c_void, MAX_INSN_SIZE * core::mem::size_of::<kprobe_opcode_t>()) != 0 { return 0; }
    if addr == (*kp).addr as c_ulong {
        *buf = (*kp).opcode;
        core::ptr::copy_nonoverlapping((*op).optinsn.copied_insn.as_ptr(), buf.add(1), DISP32_SIZE);
    } else {
        offs = addr as c_long - (*kp).addr as c_long - 1;
        core::ptr::copy_nonoverlapping((*op).optinsn.copied_insn.as_ptr().offset(offs as isize), buf, (DISP32_SIZE as c_long - offs) as usize);
    }
    buf as c_ulong
}

unsafe fn synthesize_clac(addr: *mut kprobe_opcode_t) {
    if !boot_cpu_has(X86_FEATURE_SMAP) { return; }
    *addr = 0x0f; *addr.add(1) = 0x01; *addr.add(2) = 0xca;
}

unsafe fn synthesize_set_arg1(mut addr: *mut kprobe_opcode_t, val: c_ulong) {
    #[cfg(target_arch = "x86_64")] { *addr = 0x48; *addr.add(1) = 0xbf; addr = addr.add(2); }
    #[cfg(target_arch = "x86")] { *addr = 0xb8; addr = addr.add(1); }
    *(addr as *mut c_ulong) = val;
}

unsafe extern "C" fn optimized_callback(op: *mut optimized_kprobe, regs: *mut pt_regs) {
    if kprobe_disabled(&(*op).kp) { return; }
    preempt_disable();
    if kprobe_running() { kprobes_inc_nmissed_count(&mut (*op).kp); } else {
        let kcb = get_kprobe_ctlblk();
        (*regs).sp += core::mem::size_of::<c_long>() as _;
        (*regs).cs = __KERNEL_CS;
        #[cfg(target_arch = "x86")] { (*regs).gs = 0; }
        (*regs).ip = (*op).kp.addr as c_ulong + INT3_INSN_SIZE as c_ulong;
        (*regs).orig_ax = !0;
        this_cpu_write_current_kprobe(&mut (*op).kp);
        (*kcb).kprobe_status = KPROBE_HIT_ACTIVE;
        opt_pre_handler(&mut (*op).kp, regs);
        this_cpu_write_current_kprobe(core::ptr::null_mut());
    }
    preempt_enable();
}

unsafe fn copy_optimized_instructions(dest: *mut u8, src: *mut u8, real: *mut u8) -> c_int {
    let mut insn = core::mem::MaybeUninit::<insn>::uninit(); let mut len = 0;
    while len < JMP32_INSN_SIZE { let ret = __copy_instruction(dest.add(len), src.add(len), real.add(len), insn.as_mut_ptr()); if ret == 0 || !can_boost(insn.as_ptr(), src.add(len)) { return -EINVAL; } len += ret as usize; }
    if ftrace_text_reserved(src, src.add(len-1)) || jump_label_text_reserved(src, src.add(len-1)) || static_call_text_reserved(src, src.add(len-1)) { return -EBUSY; } len as c_int
}

unsafe fn insn_is_indirect_jump(insn: *mut insn) -> c_int { ((((*insn).opcode.bytes[0] == 0xff) && (X86_MODRM_REG((*insn).modrm.value) & 6) == 4) || (*insn).opcode.bytes[0] == 0xea) as c_int }

unsafe fn insn_jump_into_range(insn: *mut insn, start: c_ulong, len: c_int) -> c_int {
    let op = (*insn).opcode.bytes[0]; let valid = op == 0xe0 || op == 0xe1 || op == 0xe2 || op == 0xe3 || op == 0xe9 || op == 0xeb || (op == 0x0f && ((*insn).opcode.bytes[1] & 0xf0) == 0x80) || (op & 0xf0) == 0x70;
    if !valid { return 0; } let target = (*insn).next_byte as c_ulong + (*insn).immediate.value as c_ulong; (start <= target && target <= start + len as c_ulong) as c_int
}

// Remaining declarations and operations retain the original externally supplied kernel types/helpers.
unsafe extern "C" {
    fn container_of_optimized(kp: *mut kprobe) -> *mut optimized_kprobe;
    fn this_cpu_write_current_kprobe(kp: *mut kprobe);
}

pub unsafe extern "C" fn arch_check_optimized_kprobe(op: *mut optimized_kprobe) -> c_int {
    let mut i = 1; while i < (*op).optinsn.size { let p = get_kprobe((*op).kp.addr.add(i)); if !p.is_null() && !kprobe_disarmed(p) { return -EEXIST; } i += 1; } 0
}
pub unsafe extern "C" fn arch_within_optimized_kprobe(op: *mut optimized_kprobe, addr: *mut kprobe_opcode_t) -> c_int { ((*op).kp.addr <= addr && (*op).kp.addr.add((*op).optinsn.size) > addr) as c_int }

unsafe fn __arch_remove_optimized_kprobe(op: *mut optimized_kprobe, dirty: c_int) {
    let slot = (*op).optinsn.insn; if !slot.is_null() { let len = TMPL_END_IDX as usize + (*op).optinsn.size + JMP32_INSN_SIZE; if dirty != 0 { perf_event_text_poke(slot, slot, len, core::ptr::null(), 0); } free_optinsn_slot(slot, dirty); (*op).optinsn.insn = core::ptr::null_mut(); (*op).optinsn.size = 0; }
}
pub unsafe extern "C" fn arch_remove_optimized_kprobe(op: *mut optimized_kprobe) { __arch_remove_optimized_kprobe(op, 1); }

pub unsafe extern "C" fn arch_prepare_optimized_kprobe(op: *mut optimized_kprobe, _unused: *mut kprobe) -> c_int {
    let buf = kzalloc(MAX_OPTINSN_SIZE, GFP_KERNEL); if buf.is_null() { return -ENOMEM; }
    let slot = get_optinsn_slot(); (*op).optinsn.insn = slot; if slot.is_null() { kfree(buf); return -ENOMEM; }
    let rel = slot as c_long - (*op).kp.addr as c_long + JMP32_INSN_SIZE as c_long; if rel.abs() > 0x7fffffff { __arch_remove_optimized_kprobe(op, 0); kfree(buf); return -ERANGE; }
    core::ptr::copy_nonoverlapping(&optprobe_template_entry, buf, TMPL_END_IDX as usize);
    let ret = copy_optimized_instructions(buf.add(TMPL_END_IDX as usize), (*op).kp.addr, slot.add(TMPL_END_IDX as usize)); if ret < 0 { __arch_remove_optimized_kprobe(op, 0); kfree(buf); return ret; }
    (*op).optinsn.size = ret as usize; let len = TMPL_END_IDX as usize + ret as usize;
    synthesize_clac(buf.add(TMPL_CLAC_IDX as usize)); synthesize_set_arg1(buf.add(TMPL_MOVE_IDX as usize), op as c_ulong);
    synthesize_relcall(buf.add(TMPL_CALL_IDX as usize), slot.add(TMPL_CALL_IDX as usize), optimized_callback as *const c_void);
    synthesize_reljump(buf.add(len), slot.add(len), (*op).kp.addr.add((*op).optinsn.size));
    perf_event_text_poke(slot, core::ptr::null_mut(), 0, buf, len + JMP32_INSN_SIZE); text_poke(slot, buf, len + JMP32_INSN_SIZE); kfree(buf); 0
}

pub unsafe extern "C" fn setup_detour_execution(p: *mut kprobe, regs: *mut pt_regs, reenter: c_int) -> c_int {
    if (*p).flags & KPROBE_FLAG_OPTIMIZED != 0 { let op = container_of_optimized(p); (*regs).ip = (*op).optinsn.insn as c_ulong + TMPL_END_IDX as c_ulong; if reenter == 0 { reset_current_kprobe(); } return 1; } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
