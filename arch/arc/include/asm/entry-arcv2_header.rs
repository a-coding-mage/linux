/* SPDX-License-Identifier: GPL-2.0 */

// Rust translation of the ARCv2 entry assembly header.
// The symbols supplied by asm-offsets.h, dsp-impl.h, irqflags-arcv2.h,
// and thread_info.h remain external dependencies.

/* Interrupt/Exception stack layout (pt_regs) for ARCv2. */

#[macro_export]
macro_rules! INTERRUPT_PROLOGUE {
    () => {{
        // Hardware has switched the stack and saved PC/STAT32; optionally it
        // has also saved r0-r11 and the loop registers.
        #[cfg(CONFIG_ARC_IRQ_NO_AUTOSAVE)]
        unsafe { core::arch::asm!("sub sp, sp, {sz} - 8", sz = const SZ_PT_REGS); }
        #[cfg(CONFIG_ARC_IRQ_NO_AUTOSAVE)]
        __SAVE_REGFILE_HARD!();
        #[cfg(not(CONFIG_ARC_IRQ_NO_AUTOSAVE))]
        unsafe { core::arch::asm!("sub sp, sp, {r0}", r0 = const PT_r0); }
        __SAVE_REGFILE_SOFT!();
    }};
}

#[macro_export]
macro_rules! EXCEPTION_PROLOGUE_KEEP_AE {
    () => {{
        unsafe {
            core::arch::asm!(
                "sub sp, sp, {sz}", "/* __SAVE_REGFILE_HARD */",
                sz = const SZ_PT_REGS,
            );
        }
        __SAVE_REGFILE_HARD!();
        __SAVE_REGFILE_SOFT!();
        unsafe {
            core::arch::asm!(
                "st r0, [sp]", "lr r10, [eret]", "lr r11, [erstatus]",
                "st2 r10, r11, [{ret}]", "lr r10, [ecr]", "lr r11, [erbta]",
                "st2 r10, r11, [{event}]", ret = const PT_ret,
                event = const PT_event,
            );
        }
    }};
}

#[macro_export]
macro_rules! EXCEPTION_PROLOGUE {
    () => {{
        EXCEPTION_PROLOGUE_KEEP_AE!();
        unsafe { core::arch::asm!("lr r0, [efa]", "mov r1, sp", "lr r9, [status32]"); }
        unsafe { core::arch::asm!("bclr r9, r9, {ae}", "bset r9, r9, {ie}", "kflag r9", ae = const STATUS_AE_BIT, ie = const STATUS_IE_BIT); }
    }};
}

#[macro_export]
macro_rules! __SAVE_REGFILE_HARD {
    () => {{
        unsafe { core::arch::asm!(
            "st2 r0, r1, [{r0}]", "st2 r2, r3, [{r2}]", "st2 r4, r5, [{r4}]",
            "st2 r6, r7, [{r6}]", "st2 r8, r9, [{r8}]", "st2 r10, r11, [{r10}]",
            "st blink, [sp, {blink}]", "lr r10, [lp_end]", "lr r11, [lp_start]",
            "st2 r10, r11, [{lpe}]", "st lp_count, [sp, {lpc}]",
            r0 = const PT_r0, r2 = const PT_r2, r4 = const PT_r4, r6 = const PT_r6,
            r8 = const PT_r8, r10 = const PT_r10, blink = const PT_blink,
            lpe = const PT_lpe, lpc = const PT_lpc,
        ); }
    }};
}

#[macro_export]
macro_rules! __SAVE_REGFILE_SOFT {
    () => {{
        unsafe { core::arch::asm!(
            "st fp, [sp, {fp}]", "st r30, [sp, {r30}]", "st r12, [sp, {r12}]",
            "st r26, [sp, {r26}]", "lr r10, [AUX_USER_SP]", "mov.nz r10, sp",
            "add2.nz r10, r10, {sz4}", "st r10, [sp, {sp}]",
            fp = const PT_fp, r30 = const PT_r30, r12 = const PT_r12,
            r26 = const PT_r26, sz4 = const (SZ_PT_REGS / 4), sp = const PT_sp,
        ); }
        #[cfg(CONFIG_ARC_HAS_ACCL_REGS)]
        unsafe { core::arch::asm!("st2 r58, r59, [sp, {r58}]", r58 = const PT_r58); }
        // DSP_SAVE_REGFILE_IRQ
        #[cfg(CONFIG_ARC_CURR_IN_REG)]
        unsafe { core::arch::asm!("/* GET_CURR_TASK_ON_CPU gp */"); }
    }};
}

#[macro_export]
macro_rules! __RESTORE_REGFILE_SOFT {
    () => {{
        unsafe { core::arch::asm!(
            "ld fp, [sp, {fp}]", "ld r30, [sp, {r30}]", "ld r12, [sp, {r12}]",
            "ld r26, [sp, {r26}]", "bz 1f", "ld r10, [sp, {sp}]",
            "sr r10, [AUX_USER_SP]", "1:", fp = const PT_fp, r30 = const PT_r30,
            r12 = const PT_r12, r26 = const PT_r26, sp = const PT_sp,
        ); }
        // DSP_RESTORE_REGFILE_IRQ
        #[cfg(CONFIG_ARC_HAS_ACCL_REGS)]
        unsafe { core::arch::asm!("ld2 r58, r59, [sp, {r58}]", r58 = const PT_r58); }
    }};
}

#[macro_export]
macro_rules! __RESTORE_REGFILE_HARD {
    () => {{ unsafe { core::arch::asm!(
        "ld blink, [sp, {blink}]", "ld2 r10, r11, [sp, {lpe}]",
        "sr r10, [lp_end]", "sr r11, [lp_start]", "ld r10, [sp, {lpc}]",
        "mov lp_count, r10", "ld2 r0, r1, [sp, {r0}]", "ld2 r2, r3, [sp, {r2}]",
        "ld2 r4, r5, [sp, {r4}]", "ld2 r6, r7, [sp, {r6}]",
        "ld2 r8, r9, [sp, {r8}]", "ld2 r10, r11, [sp, {r10}]",
        blink = const PT_blink, lpe = const PT_lpe, lpc = const PT_lpc,
        r0 = const PT_r0, r2 = const PT_r2, r4 = const PT_r4, r6 = const PT_r6,
        r8 = const PT_r8, r10 = const PT_r10,
    ); }}; }
}

#[macro_export]
macro_rules! INTERRUPT_EPILOGUE {
    () => {{
        __RESTORE_REGFILE_SOFT!();
        #[cfg(CONFIG_ARC_IRQ_NO_AUTOSAVE)]
        { __RESTORE_REGFILE_HARD!(); unsafe { core::arch::asm!("add sp, sp, {n}", n = const (SZ_PT_REGS - 8)); } }
        #[cfg(not(CONFIG_ARC_IRQ_NO_AUTOSAVE))]
        unsafe { core::arch::asm!("add sp, sp, {r0}", r0 = const PT_r0); }
    }};
}

#[macro_export]
macro_rules! EXCEPTION_EPILOGUE {
    () => {{
        unsafe { core::arch::asm!("btst r0, {u}", "ld r10, [sp, {bta}]", "sr r10, [erbta]", u = const STATUS_U_BIT, bta = const PT_bta); }
        unsafe { core::arch::asm!("ld2 r10, r11, [sp, {ret}]", "sr r10, [eret]", "sr r11, [erstatus]", ret = const PT_ret); }
        __RESTORE_REGFILE_SOFT!(); __RESTORE_REGFILE_HARD!();
        unsafe { core::arch::asm!("add sp, sp, {sz}", sz = const SZ_PT_REGS); }
    }};
}

#[macro_export]
macro_rules! FAKE_RET_FROM_EXCPN { () => {{ unsafe { core::arch::asm!("lr r9, [status32]", "bclr r9, r9, {ae}", "bset r9, r9, {ie}", "kflag r9", ae = const STATUS_AE_BIT, ie = const STATUS_IE_BIT); } }}; }

#[macro_export]
macro_rules! GET_CURR_THR_INFO_FROM_SP { ($reg:tt) => {{ unsafe { core::arch::asm!("bmskn {r}, sp, {shift}", r = lateout(reg) _, shift = const (THREAD_SHIFT - 1)); } }}; }

#[macro_export]
macro_rules! GET_CPU_ID { ($reg:tt) => {{ unsafe { core::arch::asm!("lr {r}, [identity]", "xbfu {r}, {r}, 0xE8", r = lateout(reg) _); } }}; }

#[macro_export]
macro_rules! SAVE_ABI_CALLEE_REGS { () => {{ unsafe { core::arch::asm!("push r13", "push r14", "push r15", "push r16", "push r17", "push r18", "push r19", "push r20", "push r21", "push r22", "push r23", "push r24", "push r25"); }}; }}

#[macro_export]
macro_rules! RESTORE_ABI_CALLEE_REGS { () => {{ unsafe { core::arch::asm!("pop r25", "pop r24", "pop r23", "pop r22", "pop r21", "pop r20", "pop r19", "pop r18", "pop r17", "pop r16", "pop r15", "pop r14", "pop r13"); }}; }}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
