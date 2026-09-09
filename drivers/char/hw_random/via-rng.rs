/*
 * RNG driver for VIA RNGs
 *
 * Copyright 2005 (c) MontaVista Software, Inc.
 *
 * with the majority of the code coming from:
 *
 * Hardware driver for the Intel/AMD/VIA Random Number Generators (RNG)
 * (c) Copyright 2003 Red Hat Inc <jgarzik@redhat.com>
 *
 * derived from
 *
 * Hardware driver for the AMD 768 Random Number Generator (RNG)
 * (c) Copyright 2001 Red Hat Inc
 *
 * derived from
 *
 * Hardware driver for Intel i810 Random Number Generator (RNG)
 * Copyright 2000,2001 Jeff Garzik <jgarzik@pobox.com>
 * Copyright 2000,2001 Philipp Rumpf <prumpf@mandrakesoft.com>
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.
 */

// C dependencies: crypto/padlock.h, linux/module.h, linux/kernel.h,
// linux/hw_random.h, linux/delay.h, asm/cpu_device_id.h, asm/io.h,
// asm/msr.h, asm/cpufeature.h, and asm/fpu/api.h.

enum {
    VIA_STRFILT_CNT_SHIFT: u32 = 16,
    VIA_STRFILT_FAIL: u32 = 1 << 15,
    VIA_STRFILT_ENABLE: u32 = 1 << 14,
    VIA_RAWBITS_ENABLE: u32 = 1 << 13,
    VIA_RNG_ENABLE: u32 = 1 << 6,
    VIA_NOISESRC1: u32 = 1 << 8,
    VIA_NOISESRC2: u32 = 1 << 9,
    VIA_XSTORE_CNT_MASK: u32 = 0x0f,
    VIA_RNG_CHUNK_8: u32 = 0x00,
    VIA_RNG_CHUNK_4: u32 = 0x01,
    VIA_RNG_CHUNK_4_MASK: u32 = 0xffff_ffff,
    VIA_RNG_CHUNK_2: u32 = 0x02,
    VIA_RNG_CHUNK_2_MASK: u32 = 0xffff,
    VIA_RNG_CHUNK_1: u32 = 0x03,
    VIA_RNG_CHUNK_1_MASK: u32 = 0xff,
}

/* See the original C source for the rationale behind the one-byte rate. */
#[inline]
unsafe fn xstore(addr: *mut u32, edx_in: u32) -> u32 {
    let eax_out: u32;
    core::arch::asm!(
        ".byte 0x0F, 0xA7, 0xC0",
        inout("eax") 0u32 => eax_out,
        inout("edx") edx_in => _,
        inout("edi") addr => _,
        inout("memory") *addr => _,
    );
    eax_out
}

unsafe fn via_rng_data_present(rng: *mut hwrng, wait: i32) -> i32 {
    let mut buf = [0u8; 16 + PADLOCK_ALIGNMENT - STACK_ALIGN];
    let via_rng_datum = ptr_align(buf.as_mut_ptr(), PADLOCK_ALIGNMENT) as *mut u32;
    let mut bytes_out: u32 = 0;
    let mut i = 0;

    while i < 20 {
        *via_rng_datum = 0;
        bytes_out = xstore(via_rng_datum, VIA_RNG_CHUNK_1) & VIA_XSTORE_CNT_MASK;
        if bytes_out != 0 || wait == 0 {
            break;
        }
        udelay(10);
        i += 1;
    }
    (*rng).priv_ = *via_rng_datum as u64;
    if bytes_out != 0 { 1 } else { 0 }
}

unsafe fn via_rng_data_read(rng: *mut hwrng, data: *mut u32) -> i32 {
    *data = (*rng).priv_ as u32;
    1
}

unsafe fn via_rng_init(rng: *mut hwrng) -> i32 {
    let c = &cpu_data(0);
    let (mut lo, mut hi, old_lo): (u32, u32, u32);

    if ((c.x86 == 6 && c.x86_model >= 0x0f) || c.x86 > 6) {
        if !boot_cpu_has(X86_FEATURE_XSTORE_EN) {
            pr_err!(PFX "can't enable hardware RNG if XSTORE is not enabled\n");
            return -ENODEV;
        }
        return 0;
    }

    rdmsr(MSR_VIA_RNG, &mut lo, &mut hi);
    old_lo = lo;
    lo &= !(0x7f << VIA_STRFILT_CNT_SHIFT);
    lo &= !VIA_XSTORE_CNT_MASK;
    lo &= !(VIA_STRFILT_ENABLE | VIA_STRFILT_FAIL | VIA_RAWBITS_ENABLE);
    lo |= VIA_RNG_ENABLE | VIA_NOISESRC1;
    if c.x86_model == 9 && c.x86_stepping > 7 { lo |= VIA_NOISESRC2; }
    if c.x86_model >= 10 { lo |= VIA_NOISESRC2; }
    if lo != old_lo { wrmsr(MSR_VIA_RNG, lo, hi); }
    rdmsr(MSR_VIA_RNG, &mut lo, &mut hi);
    if lo & VIA_RNG_ENABLE == 0 {
        pr_err!(PFX "cannot enable VIA C3 RNG, aborting\n");
        return -ENODEV;
    }
    0
}

static mut via_rng: hwrng = hwrng {
    name: b"via\0".as_ptr() as *const _,
    init: Some(via_rng_init),
    data_present: Some(via_rng_data_present),
    data_read: Some(via_rng_data_read),
};

unsafe fn via_rng_mod_init() -> i32 {
    if !boot_cpu_has(X86_FEATURE_XSTORE) { return -ENODEV; }
    pr_info!("VIA RNG detected\n");
    let err = hwrng_register(&mut via_rng);
    if err != 0 { pr_err!(PFX "RNG registering failed ({})\n", err); }
    err
}

unsafe fn via_rng_mod_exit() {
    hwrng_unregister(&mut via_rng);
}

static mut via_rng_cpu_id: [x86_cpu_id; 2] = [
    X86_MATCH_FEATURE(X86_FEATURE_XSTORE, core::ptr::null()),
    x86_cpu_id::default(),
];

module_init!(via_rng_mod_init);
module_exit!(via_rng_mod_exit);
MODULE_DEVICE_TABLE!(x86cpu, via_rng_cpu_id);
MODULE_DESCRIPTION!("H/W RNG driver for VIA CPU with PadLock");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
