// SPDX-License-Identifier: GPL-2.0
/*
 * Entropy functions used on early boot for KASLR base and memory
 * randomization. The base randomization is done in the compressed
 * kernel and memory randomization is done early when the regular
 * kernel starts. This file is included in the compressed kernel and
 * normally linked in the regular.
 */

// The declarations below are supplied by the architecture and boot-time
// support code represented by the C includes in the original source.
extern "C" {
    fn outb(value: u8, port: u16);
    fn inb(port: u16) -> u8;
    fn rdrand_long(value: *mut libc_ulong) -> bool;
    fn rdtsc() -> libc_ulong;
    fn early_printk(format: *const u8, ...);
    fn cpu_feature_enabled(feature: u32) -> bool;
    fn kaslr_offset() -> libc_ulong;
    fn get_boot_seed() -> libc_ulong;
    fn debug_putstr(value: *const u8);
}

type libc_ulong = usize;

const I8254_PORT_CONTROL: u16 = 0x43;
const I8254_PORT_COUNTER0: u16 = 0x40;
const I8254_CMD_READBACK: u8 = 0xC0;
const I8254_SELECT_COUNTER0: u8 = 0x02;
const I8254_STATUS_NOTREADY: u8 = 0x40;

unsafe fn i8254() -> u16 {
    let mut status: u8;
    let mut timer: u16;

    loop {
        outb(
            I8254_CMD_READBACK | I8254_SELECT_COUNTER0,
            I8254_PORT_CONTROL,
        );
        status = inb(I8254_PORT_COUNTER0);
        timer = inb(I8254_PORT_COUNTER0) as u16;
        timer |= (inb(I8254_PORT_COUNTER0) as u16) << 8;
        if status & I8254_STATUS_NOTREADY == 0 {
            break;
        }
    }

    timer
}

#[cfg(not(KASLR_COMPRESSED_BOOT))]
unsafe fn putstr(value: *const u8) {
    early_printk(b"%s\0".as_ptr(), value);
}

#[cfg(KASLR_COMPRESSED_BOOT)]
unsafe fn putstr(value: *const u8) {
    debug_putstr(value);
}

#[cfg(not(KASLR_COMPRESSED_BOOT))]
unsafe fn has_cpuflag(feature: u32) -> bool {
    cpu_feature_enabled(feature)
}

#[cfg(KASLR_COMPRESSED_BOOT)]
unsafe fn has_cpuflag(feature: u32) -> bool {
    // Supplied by the compressed-kernel architecture headers.
    cpu_feature_enabled(feature)
}

#[cfg(not(KASLR_COMPRESSED_BOOT))]
unsafe fn boot_seed() -> libc_ulong {
    kaslr_offset()
}

#[cfg(KASLR_COMPRESSED_BOOT)]
unsafe fn boot_seed() -> libc_ulong {
    get_boot_seed()
}

pub unsafe fn kaslr_get_random_long(purpose: *const u8) -> libc_ulong {
    #[cfg(target_arch = "x86_64")]
    let mix_const: libc_ulong = 0x5d6008cbf3848dd3usize;
    #[cfg(not(target_arch = "x86_64"))]
    let mix_const: libc_ulong = 0x3f39e593usize;

    let mut raw: libc_ulong;
    let mut random: libc_ulong = boot_seed();
    let mut use_i8254 = true;

    if !purpose.is_null() {
        putstr(purpose);
        putstr(b" KASLR using\0".as_ptr());
    }

    if has_cpuflag(X86_FEATURE_RDRAND) {
        if !purpose.is_null() {
            putstr(b" RDRAND\0".as_ptr());
        }
        raw = 0;
        if rdrand_long(&mut raw) {
            random ^= raw;
            use_i8254 = false;
        }
    }

    if has_cpuflag(X86_FEATURE_TSC) {
        if !purpose.is_null() {
            putstr(b" RDTSC\0".as_ptr());
        }
        raw = rdtsc();
        random ^= raw;
        use_i8254 = false;
    }

    if use_i8254 {
        if !purpose.is_null() {
            putstr(b" i8254\0".as_ptr());
        }
        random ^= i8254() as libc_ulong;
    }

    /* Circular multiply for better bit diffusion */
    let product = (random as u128) * (mix_const as u128);
    random = product as libc_ulong;
    raw = (product >> (core::mem::size_of::<libc_ulong>() * 8)) as libc_ulong;
    random = random.wrapping_add(raw);

    if !purpose.is_null() {
        putstr(b"...\n\0".as_ptr());
    }

    random
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
