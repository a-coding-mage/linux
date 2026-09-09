/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2009 Freescale Semiconductor, Inc.
 *
 * provides masks and opcode images for use by code generation, emulation
 * and for instructions that older assemblers might not know about
 */

// Dependencies supplied by the surrounding PowerPC translation.

pub const PPC_DBELL_MSG_BRDCAST: u32 = 0x0400_0000;

#[inline]
pub const fn PPC_DBELL_TYPE(x: u32) -> u32 {
    (x & 0xf) << (63 - 36)
}

pub const PPC_DBELL_TYPE_MASK: u32 = PPC_DBELL_TYPE(0xf);

#[inline]
pub const fn PPC_DBELL_LPID(x: u64) -> u64 {
    x << (63 - 49)
}

pub const PPC_DBELL_PIR_MASK: u32 = 0x3fff;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ppc_dbell {
    PPC_DBELL = 0,         /* doorbell */
    PPC_DBELL_CRIT = 1,    /* critical doorbell */
    PPC_G_DBELL = 2,       /* guest doorbell */
    PPC_G_DBELL_CRIT = 3,  /* guest critical doorbell */
    PPC_G_DBELL_MC = 4,    /* guest mcheck doorbell */
    PPC_DBELL_SERVER = 5,  /* doorbell on server */
}

#[cfg(feature = "CONFIG_PPC_BOOK3S")]
pub const PPC_DBELL_MSGTYPE: ppc_dbell = ppc_dbell::PPC_DBELL_SERVER;

#[cfg(feature = "CONFIG_PPC_BOOK3S")]
#[inline]
pub unsafe fn _ppc_msgsnd(msg: u32) {
    // ASM_FTR_IFSET(PPC_MSGSND(msg), PPC_MSGSNDP(msg), CPU_FTR_HVMODE)
    core::arch::asm!("msgsnd {0}", in(reg) msg);
}

#[cfg(feature = "CONFIG_PPC_BOOK3S")]
#[inline]
pub unsafe fn ppc_msgsync() {
    // ASM_FTR_IFSET(PPC_MSGSYNC ; lwsync, "", CPU_FTR_HVMODE|CPU_FTR_ARCH_300)
    // sync is not required when taking messages from the same core
    core::arch::asm!("msgsync; lwsync");
}

#[cfg(feature = "CONFIG_PPC_BOOK3S")]
#[inline]
pub unsafe fn _ppc_msgclr(msg: u32) {
    // ASM_FTR_IFSET(PPC_MSGCLR(msg), PPC_MSGCLRP(msg), CPU_FTR_HVMODE)
    core::arch::asm!("msgclr {0}", in(reg) msg);
}

#[cfg(feature = "CONFIG_PPC_BOOK3S")]
#[inline]
pub unsafe fn ppc_msgclr(ty: ppc_dbell) {
    let msg = PPC_DBELL_TYPE(ty as u32);
    _ppc_msgclr(msg);
}

#[cfg(not(feature = "CONFIG_PPC_BOOK3S"))]
pub const PPC_DBELL_MSGTYPE: ppc_dbell = ppc_dbell::PPC_DBELL;

#[cfg(not(feature = "CONFIG_PPC_BOOK3S"))]
#[inline]
pub unsafe fn _ppc_msgsnd(msg: u32) {
    core::arch::asm!("msgsnd {0}", in(reg) msg);
}

#[cfg(not(feature = "CONFIG_PPC_BOOK3S"))]
#[inline]
pub unsafe fn ppc_msgsync() {}

extern "C" {
    pub fn doorbell_exception(regs: *mut pt_regs);
}

#[inline]
pub unsafe fn ppc_msgsnd_sync() {
    core::arch::asm!("sync", options(nostack));
}

#[inline]
pub unsafe fn ppc_msgsnd(ty: ppc_dbell, flags: u32, tag: u32) {
    let msg = PPC_DBELL_TYPE(ty as u32)
        | (flags & PPC_DBELL_MSG_BRDCAST)
        | (tag & 0x07ff_ffff);
    _ppc_msgsnd(msg);
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn doorbell_global_ipi(cpu: i32) {
    let tag = get_hard_smp_processor_id(cpu);
    kvmppc_set_host_ipi(cpu);
    /* Order previous accesses vs. msgsnd, which is treated as a store */
    ppc_msgsnd_sync();
    ppc_msgsnd(PPC_DBELL_MSGTYPE, 0, tag);
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn doorbell_core_ipi(cpu: i32) {
    let tag = cpu_thread_in_core(cpu);
    kvmppc_set_host_ipi(cpu);
    /* Order previous accesses vs. msgsnd, which is treated as a store */
    ppc_msgsnd_sync();
    ppc_msgsnd(PPC_DBELL_MSGTYPE, 0, tag);
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn doorbell_try_core_ipi(cpu: i32) -> i32 {
    let this_cpu = get_cpu();
    let mut ret = 0;

    if cpumask_test_cpu(cpu, cpu_sibling_mask(this_cpu)) {
        doorbell_core_ipi(cpu);
        ret = 1;
    }

    put_cpu();
    ret
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
