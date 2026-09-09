/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Register definitions for the Hexagon architecture */

/* The C header guard and __ASSEMBLY__ conditional are omitted from Rust. */

/* See kernel/entry.S for further documentation. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HvmEventRecord {
    pub vmel: usize,    /* Event Linkage (return address) */
    pub vmest: usize,   /* Event context - pre-event SSR values */
    pub vmpsp: usize,   /* Previous stack pointer */
    pub vmbadva: usize, /* Bad virtual address for addressing events */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PtRegs {
    pub restart_r0: isize, /* R0 checkpoint for syscall restart */
    pub syscall_nr: isize, /* Only used in system calls */
    pub usr_preds: UsrPreds,
    pub m0_m1: M0M1,
    pub sa1_lc1: Sa1Lc1,
    pub sa0_lc0: Sa0Lc0,
    pub ugp_gp: UgpGp,
    pub cs0_cs1: Cs0Cs1,
    /* Be extremely careful with rearranging these, if at all. Some code
     * assumes the 32 registers exist exactly like this in memory;
     * e.g. kernel/ptrace.c and kernel/signal.c (restore_sigcontext). */
    pub r00_r01: R00R01,
    pub r02_r03: R02R03,
    pub r04_r05: R04R05,
    pub r06_r07: R06R07,
    pub r08_r09: R08R09,
    pub r10_r11: R10R11,
    pub r12_r13: R12R13,
    pub r14_r15: R14R15,
    pub r16_r17: R16R17,
    pub r18_r19: R18R19,
    pub r20_r21: R20R21,
    pub r22_r23: R22R23,
    pub r24_r25: R24R25,
    pub r26_r27: R26R27,
    pub r28_r29: R28R29,
    pub r30_r31: R30R31,
    /* VM dispatch pushes event record onto stack - we can build on it */
    pub hvmer: HvmEventRecord,
}

macro_rules! pair_union {
    ($name:ident, $a:ident, $b:ident, $wide:ident) => {
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub union $name {
            pub pair: Pair<$a, $b>,
            pub $wide: i64,
        }
    };
}

#[repr(C)] #[derive(Copy, Clone)] pub struct Pair<A: Copy, B: Copy> { pub first: A, pub second: B }
#[repr(C)] #[derive(Copy, Clone)] pub struct UsrPreds { pub pair: Pair<usize, usize>, pub predsusr: i64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct M0M1 { pub pair: Pair<usize, usize>, pub m1m0: i64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct Sa1Lc1 { pub pair: Pair<usize, usize>, pub lc1sa1: i64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct Sa0Lc0 { pub pair: Pair<usize, usize>, pub lc0sa0: i64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct UgpGp { pub pair: Pair<usize, usize>, pub gpugp: i64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct Cs0Cs1 { pub pair: Pair<usize, usize>, pub cs1cs0: i64 }

macro_rules! reg_union { ($name:ident, $wide:ident) => {
    #[repr(C)] #[derive(Copy, Clone)] pub union $name { pub pair: Pair<usize, usize>, pub $wide: i64 }
} }
reg_union!(R00R01, r0100); reg_union!(R02R03, r0302); reg_union!(R04R05, r0504);
reg_union!(R06R07, r0706); reg_union!(R08R09, r0908); reg_union!(R10R11, r1110);
reg_union!(R12R13, r1312); reg_union!(R14R15, r1514); reg_union!(R16R17, r1716);
reg_union!(R18R19, r1918); reg_union!(R20R21, r2120); reg_union!(R22R23, r2322);
reg_union!(R24R25, r2524); reg_union!(R26R27, r2726);
#[repr(C)] #[derive(Copy, Clone)] pub struct R28R29Regs { pub r28: usize, pub r29: usize }
#[repr(C)] #[derive(Copy, Clone)] pub union R28R29 { pub regs: R28R29Regs, pub r2928: i64 }
reg_union!(R30R31, r3130);

/* These masks and shifts are supplied by the corresponding architecture headers. */
pub unsafe fn pt_elr(regs: *mut PtRegs) -> usize { (*regs).hvmer.vmel }
pub unsafe fn pt_set_elr(regs: *mut PtRegs, val: usize) { (*regs).hvmer.vmel = val; }
pub unsafe fn pt_cause(regs: *mut PtRegs) -> usize { (*regs).hvmer.vmest & HVM_VMEST_CAUSE_MSK }
pub unsafe fn user_mode(regs: *mut PtRegs) -> bool { (*regs).hvmer.vmest & (HVM_VMEST_UM_MSK << HVM_VMEST_UM_SFT) != 0 }
pub unsafe fn ints_enabled(regs: *mut PtRegs) -> bool { (*regs).hvmer.vmest & (HVM_VMEST_IE_MSK << HVM_VMEST_IE_SFT) != 0 }
pub unsafe fn pt_psp(regs: *mut PtRegs) -> usize { (*regs).hvmer.vmpsp }
pub unsafe fn pt_badva(regs: *mut PtRegs) -> usize { (*regs).hvmer.vmbadva }
pub unsafe fn pt_set_singlestep(regs: *mut PtRegs) { (*regs).hvmer.vmest |= 1usize << HVM_VMEST_SS_SFT; }
pub unsafe fn pt_clr_singlestep(regs: *mut PtRegs) { (*regs).hvmer.vmest &= !(1usize << HVM_VMEST_SS_SFT); }
pub unsafe fn pt_set_rte_sp(regs: *mut PtRegs, sp: usize) { (*regs).hvmer.vmpsp = sp; (*regs).r28_r29.regs.r29 = sp; }
pub unsafe fn pt_set_kmode(regs: *mut PtRegs) { (*regs).hvmer.vmest = HVM_VMEST_IE_MSK << HVM_VMEST_IE_SFT; }
pub unsafe fn pt_set_usermode(regs: *mut PtRegs) { (*regs).hvmer.vmest = (HVM_VMEST_UM_MSK << HVM_VMEST_UM_SFT) | (HVM_VMEST_IE_MSK << HVM_VMEST_IE_SFT); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
