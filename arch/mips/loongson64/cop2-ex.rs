/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2014 Lemote Corporation.
 *   written by Huacai Chen <chenhc@loongson.com>
 *
 * based on arch/mips/cavium-octeon/cpu.c
 * Copyright (C) 2009 Wind River Systems,
 *   written by Ralf Baechle <ralf@linux-mips.org>
 */

unsafe fn loongson_cu2_call(
    _nfb: *mut notifier_block,
    action: ::core::ffi::c_ulong,
    data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut res: u32;
    let mut fpu_owned: u32;
    let mut value: ::core::ffi::c_ulong;
    let mut value_next: ::core::ffi::c_ulong;
    let mut insn: mips_instruction;
    let fr = !test_thread_flag(TIF_32BIT_FPREGS);
    let regs = data as *mut pt_regs;
    let addr = (*regs).cp0_badvaddr as *mut ::core::ffi::c_void;
    let pc = exception_epc(regs) as *mut u32;

    let ra = (*regs).regs[31];
    __get_user(insn.word, pc);

    match action {
        CU2_EXCEPTION => {
            preempt_disable();
            fpu_owned = __is_fpu_owner();
            if !fr {
                set_c0_status(ST0_CU1 | ST0_CU2);
            } else {
                set_c0_status(ST0_CU1 | ST0_CU2 | ST0_FR);
            }
            enable_fpu_hazard();
            KSTK_STATUS(current) |= ST0_CU1 | ST0_CU2;
            if fr {
                KSTK_STATUS(current) |= ST0_FR;
            } else {
                KSTK_STATUS(current) &= !ST0_FR;
            }
            /* If FPU is owned, we needn't init or restore fp */
            if fpu_owned == 0 {
                set_thread_flag(TIF_USEDFPU);
                init_fp_ctx(current);
                _restore_fp(current);
            }
            preempt_enable();
            return NOTIFY_STOP; /* Don't call default notifier */
        }
        CU2_LWC2_OP => {
            if insn.loongson3_lswc2_format.ls == 0 { goto sigbus; }
            if insn.loongson3_lswc2_format.fr == 0 { /* gslq */
                if !access_ok(addr, 16) { goto sigbus; }
                LoadDW(addr, value, res);
                if res != 0 { goto fault; }
                LoadDW(addr.add(8), value_next, res);
                if res != 0 { goto fault; }
                (*regs).regs[insn.loongson3_lswc2_format.rt] = value;
                (*regs).regs[insn.loongson3_lswc2_format.rq] = value_next;
                compute_return_epc(regs);
            } else { /* gslqc1 */
                if !access_ok(addr, 16) { goto sigbus; }
                lose_fpu(1);
                LoadDW(addr, value, res);
                if res != 0 { goto fault; }
                LoadDW(addr.add(8), value_next, res);
                if res != 0 { goto fault; }
                set_fpr64(&mut (*current).thread.fpu.fpr[insn.loongson3_lswc2_format.rt], 0, value);
                set_fpr64(&mut (*current).thread.fpu.fpr[insn.loongson3_lswc2_format.rq], 0, value_next);
                compute_return_epc(regs);
                own_fpu(1);
            }
            return NOTIFY_STOP;
        }
        CU2_SWC2_OP => {
            if insn.loongson3_lswc2_format.ls == 0 { goto sigbus; }
            if insn.loongson3_lswc2_format.fr == 0 { /* gssq */
                if !access_ok(addr, 16) { goto sigbus; }
                /* write upper 8 bytes first */
                value_next = (*regs).regs[insn.loongson3_lswc2_format.rq];
                StoreDW(addr.add(8), value_next, res);
                if res != 0 { goto fault; }
                value = (*regs).regs[insn.loongson3_lswc2_format.rt];
                StoreDW(addr, value, res);
                if res != 0 { goto fault; }
                compute_return_epc(regs);
            } else { /* gssqc1 */
                if !access_ok(addr, 16) { goto sigbus; }
                lose_fpu(1);
                value_next = get_fpr64(&(*current).thread.fpu.fpr[insn.loongson3_lswc2_format.rq], 0);
                StoreDW(addr.add(8), value_next, res);
                if res != 0 { goto fault; }
                value = get_fpr64(&(*current).thread.fpu.fpr[insn.loongson3_lswc2_format.rt], 0);
                StoreDW(addr, value, res);
                if res != 0 { goto fault; }
                compute_return_epc(regs);
                own_fpu(1);
            }
            return NOTIFY_STOP;
        }
        CU2_LDC2_OP => {
            match insn.loongson3_lsdc2_format.opcode1 {
                0x1 => { if !access_ok(addr, 2) { goto sigbus; } LoadHW(addr, value, res); if res != 0 { goto fault; } compute_return_epc(regs); (*regs).regs[insn.loongson3_lsdc2_format.rt] = value; }
                0x2 => { if !access_ok(addr, 4) { goto sigbus; } LoadW(addr, value, res); if res != 0 { goto fault; } compute_return_epc(regs); (*regs).regs[insn.loongson3_lsdc2_format.rt] = value; }
                0x3 => { if !access_ok(addr, 8) { goto sigbus; } LoadDW(addr, value, res); if res != 0 { goto fault; } compute_return_epc(regs); (*regs).regs[insn.loongson3_lsdc2_format.rt] = value; }
                0x6 => { die_if_kernel("Unaligned FP access in kernel code", regs); BUG_ON(!used_math()); if !access_ok(addr, 4) { goto sigbus; } lose_fpu(1); LoadW(addr, value, res); if res != 0 { goto fault; } set_fpr64(&mut (*current).thread.fpu.fpr[insn.loongson3_lsdc2_format.rt], 0, value); compute_return_epc(regs); own_fpu(1); }
                0x7 => { die_if_kernel("Unaligned FP access in kernel code", regs); BUG_ON(!used_math()); if !access_ok(addr, 8) { goto sigbus; } lose_fpu(1); LoadDW(addr, value, res); if res != 0 { goto fault; } set_fpr64(&mut (*current).thread.fpu.fpr[insn.loongson3_lsdc2_format.rt], 0, value); compute_return_epc(regs); own_fpu(1); }
                _ => {}
            }
            return NOTIFY_STOP;
        }
        CU2_SDC2_OP => {
            match insn.loongson3_lsdc2_format.opcode1 {
                0x1 => { if !access_ok(addr, 2) { goto sigbus; } compute_return_epc(regs); value = (*regs).regs[insn.loongson3_lsdc2_format.rt]; StoreHW(addr, value, res); if res != 0 { goto fault; } }
                0x2 => { if !access_ok(addr, 4) { goto sigbus; } compute_return_epc(regs); value = (*regs).regs[insn.loongson3_lsdc2_format.rt]; StoreW(addr, value, res); if res != 0 { goto fault; } }
                0x3 => { if !access_ok(addr, 8) { goto sigbus; } compute_return_epc(regs); value = (*regs).regs[insn.loongson3_lsdc2_format.rt]; StoreDW(addr, value, res); if res != 0 { goto fault; } }
                0x6 => { die_if_kernel("Unaligned FP access in kernel code", regs); BUG_ON(!used_math()); if !access_ok(addr, 4) { goto sigbus; } lose_fpu(1); value = get_fpr64(&(*current).thread.fpu.fpr[insn.loongson3_lsdc2_format.rt], 0); StoreW(addr, value, res); if res != 0 { goto fault; } compute_return_epc(regs); own_fpu(1); }
                0x7 => { die_if_kernel("Unaligned FP access in kernel code", regs); BUG_ON(!used_math()); if !access_ok(addr, 8) { goto sigbus; } lose_fpu(1); value = get_fpr64(&(*current).thread.fpu.fpr[insn.loongson3_lsdc2_format.rt], 0); StoreDW(addr, value, res); if res != 0 { goto fault; } compute_return_epc(regs); own_fpu(1); }
                _ => {}
            }
            return NOTIFY_STOP;
        }
        _ => {}
    }
    return NOTIFY_OK;

fault:
    (*regs).regs[31] = ra;
    (*regs).cp0_epc = pc as ::core::ffi::c_ulong;
    if fixup_exception(regs) { return NOTIFY_STOP; }
    die_if_kernel("Unhandled kernel unaligned access", regs);
    force_sig(SIGSEGV);
    return NOTIFY_STOP;

sigbus:
    die_if_kernel("Unhandled kernel unaligned access", regs);
    force_sig(SIGBUS);
    return NOTIFY_STOP;
}

unsafe fn loongson_cu2_setup() -> ::core::ffi::c_int {
    cu2_notifier(loongson_cu2_call, 0)
}

early_initcall!(loongson_cu2_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
