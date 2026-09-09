use core::ffi::c_void;

// The C `__iomem` address-space qualifier has no direct Rust equivalent.
// These declarations are supplied by the surrounding kernel translation.
extern "C" {
    fn kmsan_unpoison_memory(address: *const c_void, size: usize);
    fn kmsan_check_memory(address: *const c_void, size: usize);
    fn readb(address: *const u8) -> u8;
    fn writeb(value: u8, address: *mut u8);
    fn cc_platform_has(attribute: u32) -> bool;
    fn memset(address: *mut c_void, value: i32, size: usize) -> *mut c_void;
}

// Supplied by the surrounding kernel translation.
const CC_ATTR_GUEST_UNROLL_STRING_IO: u32 = 0;

unsafe fn rep_movs(to: *mut c_void, from: *const c_void, n: usize) {
    // Originally from i386/string.h. The original implementation uses the
    // architecture-specific `rep movsl`, `movsw`, and `movsb` instructions.
    // This preserves the same byte-copy operation and ordering.
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, n);
}

unsafe fn string_memcpy_fromio(to: *mut c_void, from: *const u8, mut n: usize) {
    let orig_to = to;
    let orig_n = n;

    if n == 0 {
        return;
    }

    // Align any unaligned source IO.
    if (from as usize & 1) != 0 {
        core::ptr::write(to as *mut u8, readb(from));
        n -= 1;
    }
    if n > 1 && (from as usize & 2) != 0 {
        core::ptr::write_unaligned(to as *mut u16, core::ptr::read_unaligned(from as *const u16));
        n -= 2;
    }
    rep_movs(to, from as *const c_void, n);
    // KMSAN must treat values read from devices as initialized.
    kmsan_unpoison_memory(orig_to, orig_n);
}

unsafe fn string_memcpy_toio(to: *mut u8, from: *const u8, mut n: usize) {
    if n == 0 {
        return;
    }

    // Make sure uninitialized memory isn't copied to devices.
    kmsan_check_memory(from as *const c_void, n);
    // Align any unaligned destination IO.
    if (to as usize & 1) != 0 {
        writeb(core::ptr::read(from), to);
        n -= 1;
    }
    if n > 1 && (to as usize & 2) != 0 {
        core::ptr::copy_nonoverlapping(from, to, 2);
        n -= 2;
    }
    rep_movs(to as *mut c_void, from as *const c_void, n);
}

unsafe fn unrolled_memcpy_fromio(to: *mut c_void, from: *const u8, n: usize) {
    let input = from;
    let output = to as *mut u8;

    for i in 0..n {
        *output.add(i) = readb(input.add(i));
    }
}

unsafe fn unrolled_memcpy_toio(to: *mut u8, from: *const u8, n: usize) {
    let output = to;
    let input = from;

    for i in 0..n {
        writeb(*input.add(i), output.add(i));
    }
}

unsafe fn unrolled_memset_io(a: *mut u8, b: i32, c: usize) {
    let memory = a;

    for i in 0..c {
        writeb(b as u8, memory.add(i));
    }
}

#[no_mangle]
pub unsafe extern "C" fn memcpy_fromio(to: *mut c_void, from: *const u8, n: usize) {
    if cc_platform_has(CC_ATTR_GUEST_UNROLL_STRING_IO) {
        unrolled_memcpy_fromio(to, from, n);
    } else {
        string_memcpy_fromio(to, from, n);
    }
}

#[no_mangle]
pub unsafe extern "C" fn memcpy_toio(to: *mut u8, from: *const u8, n: usize) {
    if cc_platform_has(CC_ATTR_GUEST_UNROLL_STRING_IO) {
        unrolled_memcpy_toio(to, from, n);
    } else {
        string_memcpy_toio(to, from, n);
    }
}

#[no_mangle]
pub unsafe extern "C" fn memset_io(a: *mut u8, b: i32, c: usize) {
    if cc_platform_has(CC_ATTR_GUEST_UNROLL_STRING_IO) {
        unrolled_memset_io(a, b, c);
    } else {
        /*
         * TODO: memset can mangle the IO patterns quite a bit.
         * perhaps it would be better to use a dumb one:
         */
        memset(a as *mut c_void, b, c);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
