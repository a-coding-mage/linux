// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 1996, 1997, 1998, 1999, 2000,
 * Ingo Molnar, Matti Aarnio, Jakub Jelinek, Richard Henderson.
 *
 * Dispatch optimized XOR parity functions.
 */

// Linux kernel dependencies and build-time configuration are supplied externally.

extern "C" {
    static mut template_list: *mut xor_block_template;
    static mut forced_template: *mut xor_block_template;
    static mut xor_block_8regs: xor_block_template;
    static mut xor_block_8regs_p: xor_block_template;
    static mut xor_block_32regs: xor_block_template;
    static mut xor_block_32regs_p: xor_block_template;
}

#[repr(C)]
pub struct xor_block_template {
    pub next: *mut xor_block_template,
    pub xor_gen: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32)>,
    pub speed: u64,
    pub name: *const core::ffi::c_char,
}

// DEFINE_STATIC_CALL_NULL(xor_gen_impl, *xor_block_8regs.xor_gen)
static mut xor_gen_impl: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32)> = None;

/// xor_gen - generate RAID-style XOR information
/// @dest: destination vector
/// @srcs: source vectors
/// @src_cnt: number of source vectors
/// @bytes: length in bytes of each vector
pub unsafe extern "C" fn xor_gen(
    dest: *mut core::ffi::c_void,
    srcs: *mut *mut core::ffi::c_void,
    src_cnt: u32,
    bytes: u32,
) {
    // WARN_ON_ONCE(!in_task() || irqs_disabled() || softirq_count());
    // WARN_ON_ONCE(bytes == 0);
    // WARN_ON_ONCE(bytes & 511);
    if let Some(func) = xor_gen_impl {
        func(dest, srcs, src_cnt, bytes);
    }
}

// EXPORT_SYMBOL(xor_gen);

/// xor_register - register a XOR template
pub unsafe extern "C" fn xor_register(tmpl: *mut xor_block_template) {
    (*tmpl).next = template_list;
    template_list = tmpl;
}

/// xor_force - force use of a XOR template
pub unsafe extern "C" fn xor_force(tmpl: *mut xor_block_template) {
    forced_template = tmpl;
}

const BENCH_SIZE: u32 = 4096;
const NR_SRCS: usize = 4;
const REPS: u32 = 800;

unsafe fn do_xor_speed(
    tmpl: *mut xor_block_template,
    dest: *mut core::ffi::c_void,
    srcs: *mut *mut core::ffi::c_void,
) {
    let mut t: u64;
    // preempt_disable();
    t = ktime_get_ns();
    for _ in 0..REPS {
        // mb(); /* prevent loop optimization */
        if let Some(func) = (*tmpl).xor_gen {
            func(dest, srcs, NR_SRCS as u32, BENCH_SIZE);
        }
        // mb();
    }
    t = core::cmp::max(ktime_get_ns().wrapping_sub(t), 1);
    // preempt_enable();

    (*tmpl).speed = ((BENCH_SIZE as u64)
        .wrapping_mul(REPS as u64)
        .wrapping_mul(NR_SRCS as u64)
        .wrapping_mul(1000)) / t;

    // pr_info("   %-16s: %5d MB/sec\n", tmpl->name, tmpl->speed);
}

unsafe fn calibrate_xor_blocks() -> i32 {
    let mut f: *mut xor_block_template;
    let mut fastest: *mut xor_block_template;
    let mut srcs: [*mut core::ffi::c_void; NR_SRCS] = [core::ptr::null_mut(); NR_SRCS];
    let buf: *mut core::ffi::c_void;
    let dest: *mut core::ffi::c_void;

    if !forced_template.is_null() {
        return 0;
    }

    buf = kmalloc(BENCH_SIZE as usize * (NR_SRCS + 1));
    if buf.is_null() {
        // pr_warn("xor: Yikes!  No memory available.\n");
        return -12;
    }
    get_random_bytes(buf, BENCH_SIZE as usize * (NR_SRCS + 1));
    dest = buf;
    for i in 0..NR_SRCS {
        srcs[i] = (buf as *mut u8).add((i + 1) * BENCH_SIZE as usize) as *mut core::ffi::c_void;
    }

    // pr_info("xor: measuring software checksum speed\n");
    fastest = template_list;
    f = template_list;
    while !f.is_null() {
        do_xor_speed(f, dest, srcs.as_mut_ptr());
        if (*f).speed > (*fastest).speed {
            fastest = f;
        }
        f = (*f).next;
    }
    xor_gen_impl = (*fastest).xor_gen;
    // pr_info("xor: using function: %s (%d MB/sec)\n", fastest->name, fastest->speed);
    kfree(buf);
    0
}

#[cfg(feature = "CONFIG_XOR_BLOCKS_ARCH")]
unsafe fn arch_xor_init() {
    // The architecture-specific xor_arch.h implementation is supplied externally.
}

#[cfg(not(feature = "CONFIG_XOR_BLOCKS_ARCH"))]
unsafe fn arch_xor_init() {
    xor_register(&raw mut xor_block_8regs);
    xor_register(&raw mut xor_block_8regs_p);
    xor_register(&raw mut xor_block_32regs);
    xor_register(&raw mut xor_block_32regs_p);
}

unsafe fn xor_init() -> i32 {
    arch_xor_init();
    if !forced_template.is_null() {
        // pr_info("xor: automatically using best checksumming function   %-10s\n", forced_template->name);
        xor_gen_impl = (*forced_template).xor_gen;
        return 0;
    }

    #[cfg(feature = "MODULE")]
    {
        return calibrate_xor_blocks();
    }
    #[cfg(not(feature = "MODULE"))]
    {
        xor_gen_impl = (*template_list).xor_gen;
        return 0;
    }
}

unsafe fn xor_exit() {}

// MODULE_DESCRIPTION("RAID-5 checksumming functions");
// MODULE_LICENSE("GPL");
// When built-in, calibration is registered separately after the default template.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
