/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/kprobes.h, linux/percpu.h, asm/lowcore.h, asm/ptrace.h,
// and asm/asm-offsets.h.

#[inline(always)]
unsafe fn percpu_entry(regs: *mut pt_regs) {
    let lc: *mut lowcore = get_lowcore();

    if user_mode(regs) {
        return;
    }
    (*regs).cpu = (*lc).cpu_nr;
    (*regs).percpu_register = (*lc).percpu_register;
    (*lc).percpu_register = 0;
}

#[inline(always)]
unsafe fn percpu_code_check(regs: *mut pt_regs) -> bool {
    let mut insn: u32;
    let mut disp: usize;
    let mut p: *mut kprobe;

    if likely(user_mode(regs) || (*regs).percpu_register == 0) {
        return false;
    }
    /*
     * Within a percpu code section - check if the percpu base register
     * needs to be updated. This is the case if the PSW does not point to
     * the ADD instruction within the section.
     * - AG %rx,percpu_offset_in_lowcore(%r0,%r0)
     * which adds the percpu offset to the percpu base register.
    */
    lockdep_assert_preemption_disabled();
    let ia = psw_bits((*regs).psw).ia as *const u8;
    loop {
        insn = core::ptr::read_volatile(ia as *const u16) as u32;
        if unlikely(insn as u16 == BREAKPOINT_INSTRUCTION) {
            p = get_kprobe(ia as *mut core::ffi::c_void);
            /*
             * If the kprobe is concurrently removed on a different CPU
             * it might not be found anymore. However text must have
             * been restored - try again.
             */
            if p.is_null() {
                continue;
            }
            insn = (*p).opcode as u32;
        }
        break;
    }
    if (insn & 0xff0f) != 0xe300 {
        return true;
    }
    disp = core::mem::offset_of!(lowcore, percpu_offset);
    if machine_has_relocated_lowcore() {
        disp += LOWCORE_ALT_ADDRESS as usize;
    }
    insn = ((disp as u32 & 0xff000) >> 4) | ((disp as u32 & 0x00fff) << 16) | 0x8;
    if core::ptr::read_volatile(ia.add(2) as *const u32) != insn {
        return true;
    }
    false
}

#[inline(always)]
unsafe fn percpu_exit(regs: *mut pt_regs, needs_fixup: bool) {
    let lc: *mut lowcore = get_lowcore();
    let reg: u8;

    if user_mode(regs) {
        return;
    }
    reg = (*regs).percpu_register;
    (*lc).percpu_register = reg;
    if likely(!needs_fixup) {
        return;
    }
    /* Check if process has been migrated to a different CPU. */
    if (*regs).cpu == (*lc).cpu_nr {
        return;
    }
    /* Fixup percpu base register */
    (*regs).gprs[reg as usize] -= __per_cpu_offset[(*regs).cpu as usize];
    (*regs).gprs[reg as usize] += (*lc).percpu_offset;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
