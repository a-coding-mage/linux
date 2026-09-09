// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Author: Xianghua Xiao <x.xiao@freescale.com>
 *         Zhang Wei <wei.zhang@freescale.com>
 *
 * Copyright 2006 Freescale Semiconductor Inc.
 */

// Linux and architecture dependencies supplied by other translation units.

unsafe extern "C" {
    fn __secondary_start_mpc86xx();
}

const MCM_PORT_CONFIG_OFFSET: usize = 0x10;

/* Offset from CCSRBAR */
const MPC86XX_MCM_OFFSET: usize = 0x1000;
const MPC86XX_MCM_SIZE: usize = 0x1000;

unsafe extern "C" {
    static mut __secondary_hold_acknowledge: ::core::ffi::c_int;

    fn get_immrbase() -> usize;
    fn ioremap(addr: usize, size: usize) -> *mut u32;
    fn iounmap(addr: *mut u32);
    fn in_be32(addr: *const u32) -> ::core::ffi::c_ulong;
    fn out_be32(addr: *mut u32, value: ::core::ffi::c_ulong);
    fn patch_branch(addr: *mut u32, target: ::core::ffi::c_ulong, flags: ::core::ffi::c_ulong);
    fn patch_instruction(addr: *mut u32, instruction: u32);
    fn ppc_inst(value: u32) -> u32;
    fn local_irq_save(flags: *mut ::core::ffi::c_ulong);
    fn local_irq_restore(flags: ::core::ffi::c_ulong);
    fn mdelay(milliseconds: ::core::ffi::c_uint);
    fn mpic_setup_this_cpu();
    fn smp_mpic_message_pass();
    fn smp_mpic_probe();
    fn smp_generic_take_timebase();
    fn smp_generic_give_timebase();
    fn pr_debug(format: *const ::core::ffi::c_char, ...);
}

const NR_CPUS: ::core::ffi::c_int = 0; // Supplied by the kernel configuration.
const KERNELBASE: usize = 0; // Supplied by the platform headers.
const BRANCH_SET_LINK: ::core::ffi::c_ulong = 0; // Supplied by the instruction headers.
const ENOENT: ::core::ffi::c_int = 2;

#[repr(C)]
pub struct smp_ops_t {
    pub cause_nmi_ipi: *mut ::core::ffi::c_void,
    pub message_pass: Option<unsafe extern "C" fn()>,
    pub probe: Option<unsafe extern "C" fn()>,
    pub kick_cpu: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub setup_cpu: Option<unsafe extern "C" fn(::core::ffi::c_int)>,
    pub take_timebase: Option<unsafe extern "C" fn()>,
    pub give_timebase: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    static mut smp_ops: *mut smp_ops_t;
}

unsafe fn smp_86xx_release_core(nr: ::core::ffi::c_int) {
    let mcm_vaddr: *mut u32;
    let mut pcr: ::core::ffi::c_ulong;

    if nr < 0 || nr >= NR_CPUS {
        return;
    }

    /*
     * Startup Core #nr.
     */
    mcm_vaddr = ioremap(get_immrbase() + MPC86XX_MCM_OFFSET, MPC86XX_MCM_SIZE);
    pcr = in_be32(mcm_vaddr.add(MCM_PORT_CONFIG_OFFSET >> 2));
    pcr |= 1 << (nr + 24);
    out_be32(mcm_vaddr.add(MCM_PORT_CONFIG_OFFSET >> 2), pcr);

    iounmap(mcm_vaddr);
}

unsafe fn smp_86xx_kick_cpu(nr: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let save_vector: u32;
    let target: ::core::ffi::c_ulong;
    let mut flags: ::core::ffi::c_ulong = 0;
    let mut n: ::core::ffi::c_int = 0;
    let vector = (KERNELBASE + 0x100) as *mut u32;

    if nr < 0 || nr >= NR_CPUS {
        return -ENOENT;
    }

    pr_debug(b"smp_86xx_kick_cpu: kick CPU #%d\n\0".as_ptr() as *const _, nr);

    local_irq_save(&mut flags);

    /* Save reset vector */
    save_vector = *vector;

    /* Setup fake reset vector to call __secondary_start_mpc86xx. */
    target = __secondary_start_mpc86xx as *const () as usize as ::core::ffi::c_ulong;
    patch_branch(vector, target, BRANCH_SET_LINK);

    /* Kick that CPU */
    smp_86xx_release_core(nr);

    /* Wait a bit for the CPU to take the exception. */
    while __secondary_hold_acknowledge != nr {
        n += 1;
        if n >= 1000 {
            break;
        }
        mdelay(1);
    }

    /* Restore the exception vector */
    patch_instruction(vector, ppc_inst(save_vector));

    local_irq_restore(flags);

    pr_debug(b"wait CPU #%d for %d msecs.\n\0".as_ptr() as *const _, nr, n);

    0
}

unsafe fn smp_86xx_setup_cpu(_cpu_nr: ::core::ffi::c_int) {
    mpic_setup_this_cpu();
}

#[no_mangle]
pub static mut smp_86xx_ops: smp_ops_t = smp_ops_t {
    cause_nmi_ipi: ::core::ptr::null_mut(),
    message_pass: Some(smp_mpic_message_pass),
    probe: Some(smp_mpic_probe),
    kick_cpu: Some(smp_86xx_kick_cpu),
    setup_cpu: Some(smp_86xx_setup_cpu),
    take_timebase: Some(smp_generic_take_timebase),
    give_timebase: Some(smp_generic_give_timebase),
};

pub unsafe fn mpc86xx_smp_init() {
    smp_ops = &raw mut smp_86xx_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
