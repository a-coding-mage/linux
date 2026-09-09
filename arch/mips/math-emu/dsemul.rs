// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct emuframe {
    pub emul: mips_instruction,
    pub badinst: mips_instruction,
}

static EMUPAGE_FRAME_COUNT: usize = PAGE_SIZE / core::mem::size_of::<emuframe>();

#[inline]
unsafe fn dsemul_page() -> *mut emuframe {
    STACK_TOP as *mut emuframe
}

unsafe fn alloc_emuframe() -> i32 {
    let mm_ctx = &mut (*(*current).mm).context;
    let mut idx: i32;

    loop {
        spin_lock(&mut mm_ctx.bd_emupage_lock);

        if mm_ctx.bd_emupage_allocmap.is_null() {
            mm_ctx.bd_emupage_allocmap = bitmap_zalloc(EMUPAGE_FRAME_COUNT, GFP_ATOMIC);
            if mm_ctx.bd_emupage_allocmap.is_null() {
                idx = BD_EMUFRAME_NONE;
                spin_unlock(&mut mm_ctx.bd_emupage_lock);
                return idx;
            }
        }

        idx = bitmap_find_free_region(
            mm_ctx.bd_emupage_allocmap,
            EMUPAGE_FRAME_COUNT,
            0,
        );
        if idx < 0 {
            spin_unlock(&mut mm_ctx.bd_emupage_lock);
            if !wait_event_killable(
                &mut mm_ctx.bd_emupage_queue,
                !bitmap_full(mm_ctx.bd_emupage_allocmap, EMUPAGE_FRAME_COUNT),
            ) {
                continue;
            }
            return BD_EMUFRAME_NONE;
        }

        pr_debug!("allocate emuframe {} to {}\n", idx, (*current).pid);
        spin_unlock(&mut mm_ctx.bd_emupage_lock);
        return idx;
    }
}

unsafe fn free_emuframe(idx: i32, mm: *mut mm_struct) {
    let mm_ctx = &mut (*mm).context;

    spin_lock(&mut mm_ctx.bd_emupage_lock);
    pr_debug!("free emuframe {} from {}\n", idx, (*current).pid);
    bitmap_clear(mm_ctx.bd_emupage_allocmap, idx, 1);
    wake_up(&mut mm_ctx.bd_emupage_queue);
    spin_unlock(&mut mm_ctx.bd_emupage_lock);
}

unsafe fn within_emuframe(regs: *mut pt_regs) -> bool {
    let base = dsemul_page() as usize;
    if (*regs).cp0_epc < base {
        return false;
    }
    if (*regs).cp0_epc >= base + PAGE_SIZE {
        return false;
    }
    true
}

pub unsafe fn dsemul_thread_cleanup(tsk: *mut task_struct) -> bool {
    let fr_idx = atomic_xchg(&mut (*tsk).thread.bd_emu_frame, BD_EMUFRAME_NONE);
    if fr_idx == BD_EMUFRAME_NONE {
        return false;
    }

    task_lock(tsk);
    if !(*tsk).mm.is_null() {
        free_emuframe(fr_idx, (*tsk).mm);
    }
    task_unlock(tsk);
    true
}

pub unsafe fn dsemul_thread_rollback(regs: *mut pt_regs) -> bool {
    if !within_emuframe(regs) {
        return false;
    }

    let fr_idx = atomic_read(&(*current).thread.bd_emu_frame);
    if fr_idx == BD_EMUFRAME_NONE {
        return false;
    }
    let fr = &*dsemul_page().add(fr_idx as usize);

    if msk_isa16_mode((*regs).cp0_epc) == (&fr.emul as *const _ as usize) {
        (*regs).cp0_epc = (*current).thread.bd_emu_branch_pc;
    } else if msk_isa16_mode((*regs).cp0_epc) == (&fr.badinst as *const _ as usize) {
        (*regs).cp0_epc = (*current).thread.bd_emu_cont_pc;
    }

    atomic_set(&mut (*current).thread.bd_emu_frame, BD_EMUFRAME_NONE);
    free_emuframe(fr_idx, (*current).mm);
    true
}

pub unsafe fn dsemul_mm_cleanup(mm: *mut mm_struct) {
    bitmap_free((*mm).context.bd_emupage_allocmap);
}

pub unsafe fn mips_dsemul(
    regs: *mut pt_regs,
    ir: mips_instruction,
    branch_pc: usize,
    cont_pc: usize,
) -> i32 {
    let isa16 = get_isa16_mode((*regs).cp0_epc);
    let break_math: mips_instruction;
    let fr_uaddr: usize;
    let mut fr: emuframe = core::mem::zeroed();
    let mut fr_idx: i32;

    if ir == 0 {
        return -1;
    }

    if isa16 != 0 {
        let insn = mips_instruction_union { word: ir };
        if (ir >> 16) == MM_NOP16 {
            return -1;
        }
        if insn.mm_a_format.opcode == mm_addiupc_op {
            let rs = (((insn.mm_a_format.rs + 0xe) & 0xf) + 2) as usize;
            let mut v = ((*regs).cp0_epc & !3) as i32;
            v = v.wrapping_add((insn.mm_a_format.simmediate << 2) as i32);
            (*regs).regs[rs] = v as isize;
            return -1;
        }
    }

    pr_debug!("dsemul 0x{:08lx} cont at 0x{:08lx}\n", (*regs).cp0_epc, cont_pc);
    fr_idx = atomic_read(&(*current).thread.bd_emu_frame);
    if fr_idx == BD_EMUFRAME_NONE {
        fr_idx = alloc_emuframe();
    }
    if fr_idx == BD_EMUFRAME_NONE {
        return SIGBUS;
    }

    break_math = BREAK_MATH(isa16);
    if isa16 != 0 {
        fr.emul = mips_instruction_union { halfword: [ir >> 16, ir] }.word;
        fr.badinst = mips_instruction_union { halfword: [break_math >> 16, break_math] }.word;
    } else {
        fr.emul = ir;
        fr.badinst = break_math;
    }

    fr_uaddr = dsemul_page().add(fr_idx as usize) as usize;
    let ret = access_process_vm(current, fr_uaddr, &mut fr as *mut _ as *mut _, core::mem::size_of::<emuframe>(), FOLL_FORCE | FOLL_WRITE);
    if ret != core::mem::size_of::<emuframe>() as i32 {
        MIPS_FPU_EMU_INC_STATS(errors);
        free_emuframe(fr_idx, (*current).mm);
        return SIGBUS;
    }

    (*current).thread.bd_emu_branch_pc = branch_pc;
    (*current).thread.bd_emu_cont_pc = cont_pc;
    atomic_set(&mut (*current).thread.bd_emu_frame, fr_idx);
    (*regs).cp0_epc = fr_uaddr | isa16 as usize;
    0
}

pub unsafe fn do_dsemulret(xcp: *mut pt_regs) -> bool {
    if !dsemul_thread_cleanup(current) {
        MIPS_FPU_EMU_INC_STATS(errors);
        return false;
    }
    (*xcp).cp0_epc = (*current).thread.bd_emu_cont_pc;
    pr_debug!("dsemulret to 0x{:08lx}\n", (*xcp).cp0_epc);
    MIPS_FPU_EMU_INC_STATS(ds_emul);
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
