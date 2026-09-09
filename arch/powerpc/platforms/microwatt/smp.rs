// SPDX-License-Identifier: GPL-2.0-or-later

/*
 * SMP support functions for Microwatt
 * Copyright 2025 Paul Mackerras <paulus@ozlabs.org>
 */

// C dependencies: linux/kernel.h, linux/smp.h, linux/io.h,
// asm/early_ioremap.h, asm/ppc-opcode.h, asm/reg.h, asm/smp.h,
// asm/xics.h, and microwatt.h.

extern "C" {
    fn xics_smp_probe();
    fn xics_setup_cpu();
    fn smp_generic_kick_cpu();
    fn early_ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn early_iounmap(addr: *mut core::ffi::c_void, size: usize);
    fn readl(addr: *const u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn barrier();
    fn pr_err(format: *const u8, ...);

    static mut smp_ops: *mut smp_ops_t;
    static mut __secondary_hold_acknowledge: bool;
}

#[repr(C)]
struct smp_ops_t {
    probe: Option<unsafe extern "C" fn()>,
    message_pass: Option<unsafe extern "C" fn()>,
    kick_cpu: Option<unsafe extern "C" fn()>,
    setup_cpu: Option<unsafe extern "C" fn(i32)>,
}

unsafe extern "C" fn microwatt_smp_probe() {
    xics_smp_probe();
}

unsafe extern "C" fn microwatt_smp_setup_cpu(cpu: i32) {
    if cpu != 0 {
        xics_setup_cpu();
    }
}

static mut microwatt_smp_ops: smp_ops_t = smp_ops_t {
    probe: Some(microwatt_smp_probe),
    message_pass: None, // Use smp_muxed_ipi_message_pass
    kick_cpu: Some(smp_generic_kick_cpu),
    setup_cpu: Some(microwatt_smp_setup_cpu),
};

/* XXX get from device tree */
const SYSCON_BASE: usize = 0xc0000000;
const SYSCON_LENGTH: usize = 0x100;

const SYSCON_CPU_CTRL: usize = 0x58;

unsafe extern "C" fn microwatt_init_smp() {
    let syscon: *mut u8;
    let ncpus: i32;
    let mut timeout: i32;

    syscon = early_ioremap(SYSCON_BASE, SYSCON_LENGTH) as *mut u8;
    if syscon.is_null() {
        pr_err(b"Failed to map SYSCON\0".as_ptr());
        return;
    }
    ncpus = ((readl(syscon.add(SYSCON_CPU_CTRL)) >> 8) & 0xff) as i32;
    if ncpus < 2 {
        early_iounmap(syscon as *mut core::ffi::c_void, SYSCON_LENGTH);
        return;
    }

    smp_ops = &raw mut microwatt_smp_ops;

    /*
     * Write two instructions at location 0:
     * mfspr r3, PIR
     * b __secondary_hold
     */
    *(0xc0000000usize as *mut u32) = PPC_RAW_MFSPR(3, SPRN_PIR);
    *((0xc0000000usize + 4) as *mut u32) =
        PPC_RAW_BRANCH(&__secondary_hold as *const _ as isize - (0xc0000000usize + 4) as isize);

    // enable the other CPUs, they start at location 0
    writel((1u32 << ncpus) - 1, syscon.add(SYSCON_CPU_CTRL));

    timeout = 10000;
    while !__secondary_hold_acknowledge {
        timeout -= 1;
        if timeout == 0 {
            break;
        }
        barrier();
    }

    early_iounmap(syscon as *mut core::ffi::c_void, SYSCON_LENGTH);
}

extern "C" {
    fn PPC_RAW_MFSPR(reg: i32, spr: i32) -> u32;
    fn PPC_RAW_BRANCH(offset: isize) -> u32;
    static __secondary_hold: u8;
    const SPRN_PIR: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
