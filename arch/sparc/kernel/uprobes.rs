// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * User-space Probes (UProbes) for sparc
 *
 * Copyright (C) 2013 Oracle Inc.
 *
 * Authors:
 *	Jose E. Marchesi <jose.marchesi@oracle.com>
 *	Eric Saint Etienne <eric.saint.etienne@oracle.com>
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub unsafe fn uprobe_get_swbp_addr(regs: *mut pt_regs) -> c_ulong {
    instruction_pointer(regs)
}

unsafe fn copy_to_page(page: *mut page, vaddr: c_ulong, src: *const c_void, len: c_int) {
    let kaddr = kmap_atomic(page);
    memcpy(
        (kaddr as *mut u8).offset((vaddr & !PAGE_MASK) as isize) as *mut c_void,
        src,
        len as usize,
    );
    kunmap_atomic(kaddr);
}

pub unsafe fn arch_uprobe_copy_ixol(
    page: *mut page,
    vaddr: c_ulong,
    src: *mut c_void,
    len: c_ulong,
) {
    let stp_insn: u32 = UPROBE_STP_INSN;
    let mut insn: u32 = *(src as *const u32);

    /* Branches annulling their delay slot must be fixed to not do so. */
    let op = (insn >> 30) & 0x3;
    let op2 = (insn >> 22) & 0x7;

    if op == 0
        && (op2 == 1 || op2 == 2 || op2 == 3 || op2 == 5 || op2 == 6)
        && (insn & ANNUL_BIT) == ANNUL_BIT
    {
        insn &= !ANNUL_BIT;
    }

    copy_to_page(page, vaddr, &insn as *const u32 as *const c_void, len as c_int);
    copy_to_page(
        page,
        vaddr.wrapping_add(len),
        &stp_insn as *const u32 as *const c_void,
        4,
    );
}

pub unsafe fn arch_uprobe_analyze_insn(
    _auprobe: *mut arch_uprobe,
    _mm: *mut mm_struct,
    _addr: c_ulong,
) -> c_int {
    0
}

unsafe fn relbranch_fixup(
    insn: u32,
    utask: *mut uprobe_task,
    regs: *mut pt_regs,
) -> c_ulong {
    if (*regs).tnpc == (*regs).tpc.wrapping_add(0x4) {
        return (*utask).autask.saved_tnpc.wrapping_add(0x4);
    }

    if (insn & 0xc0000000) == 0x40000000
        || (insn & 0xc1c00000) == 0x00400000
        || (insn & 0xc1c00000) == 0x00800000
    {
        let real_pc = (*utask).vaddr as c_ulong;
        let ixol_addr = (*utask).xol_vaddr;
        return real_pc.wrapping_add((*regs).tnpc.wrapping_sub(ixol_addr));
    }

    (*regs).tnpc
}

unsafe fn retpc_fixup(regs: *mut pt_regs, insn: u32, real_pc: c_ulong) -> c_int {
    let mut slot: *mut c_ulong = core::ptr::null_mut();
    let mut rc: c_int = 0;

    if (insn & 0xc0000000) == 0x40000000 {
        slot = &mut (*regs).u_regs[UREG_I7 as usize];
    }

    if (insn & 0xc1f80000) == 0x81c00000 {
        let mut rd = ((insn >> 25) & 0x1f) as c_ulong;

        if rd <= 15 {
            slot = &mut (*regs).u_regs[rd as usize];
        } else {
            let fp = (*regs).u_regs[UREG_FP as usize];
            flushw_all();
            rd -= 16;
            if test_thread_64bit_stack(fp) {
                let uslot = (fp.wrapping_add(STACK_BIAS) as *mut c_ulong).add(rd as usize);
                rc = __put_user(real_pc, uslot);
            } else {
                let uslot = (fp as *mut u32).add(rd as usize);
                rc = __put_user(real_pc as u32, uslot);
            }
        }
    }
    if !slot.is_null() {
        *slot = real_pc;
    }
    rc
}

pub unsafe fn arch_uprobe_skip_sstep(
    auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> bool {
    if (*auprobe).ixol == (1 << 24) {
        (*regs).tnpc = (*regs).tnpc.wrapping_add(4);
        (*regs).tpc = (*regs).tpc.wrapping_add(4);
        return true;
    }
    false
}

pub unsafe fn arch_uprobe_pre_xol(
    _auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> c_int {
    let utask = (*current).utask;
    let autask = &mut (*utask).autask;
    autask.saved_tpc = (*regs).tpc;
    autask.saved_tnpc = (*regs).tnpc;
    instruction_pointer_set(regs, (*utask).xol_vaddr);
    0
}

pub unsafe fn arch_uprobe_post_xol(
    auprobe: *mut arch_uprobe,
    regs: *mut pt_regs,
) -> c_int {
    let utask = (*current).utask;
    let autask = &mut (*utask).autask;
    let insn = (*auprobe).ixol;
    let mut rc: c_int = 0;

    if (*utask).state == UTASK_SSTEP_ACK {
        (*regs).tnpc = relbranch_fixup(insn, utask, regs);
        (*regs).tpc = autask.saved_tnpc;
        rc = retpc_fixup(regs, insn, (*utask).vaddr as c_ulong);
    } else {
        (*regs).tnpc = (*utask).vaddr.wrapping_add(4);
        (*regs).tpc = autask.saved_tnpc.wrapping_add(4);
    }
    rc
}

pub unsafe extern "C" fn uprobe_trap(regs: *mut pt_regs, trap_level: c_ulong) {
    BUG_ON(trap_level != 0x173 && trap_level != 0x174);

    if !user_mode(regs) {
        local_irq_enable();
        bad_trap(regs, trap_level);
        return;
    }

    if notify_die(
        if trap_level == 0x173 { DIE_BPT } else { DIE_SSTEP },
        if trap_level == 0x173 { b"bpt\0".as_ptr() as *const c_char } else { b"sstep\0".as_ptr() as *const c_char },
        regs,
        0,
        trap_level,
        SIGTRAP,
    ) != NOTIFY_STOP
    {
        bad_trap(regs, trap_level);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
