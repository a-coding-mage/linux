// SPDX-License-Identifier: GPL-2.0-only
/*
 * Written by Pekka Paalanen, 2008-2009 <pq@iki.fi>
 */

// C dependencies supplied by the kernel build environment.

static mut mmio_address: ::core::primitive::c_ulong = 0;
static mut read_far: ::core::primitive::c_ulong = 0x400100;

unsafe extern "C" {
    fn ioremap(addr: ::core::primitive::c_ulong, size: ::core::primitive::c_ulong) -> *mut ::core::ffi::c_void;
    fn iounmap(addr: *mut ::core::ffi::c_void);
    fn ioread8(addr: *mut ::core::ffi::c_void) -> u8;
    fn ioread16(addr: *mut ::core::ffi::c_void) -> u16;
    fn ioread32(addr: *mut ::core::ffi::c_void) -> u32;
    fn iowrite8(value: u8, addr: *mut ::core::ffi::c_void);
    fn iowrite16(value: u16, addr: *mut ::core::ffi::c_void);
    fn iowrite32(value: u32, addr: *mut ::core::ffi::c_void);
    fn security_locked_down(reason: u32) -> i32;
    fn synchronize_rcu();
}

const LOCKDOWN_MMIOTRACE: u32 = 0;
const PAGE_SIZE: usize = 4096;

unsafe fn v16(i: u32) -> u32 {
    i.wrapping_mul(12).wrapping_add(7)
}

unsafe fn v32(i: u32) -> u32 {
    i.wrapping_mul(212371).wrapping_add(13)
}

unsafe fn do_write_test(p: *mut ::core::ffi::c_void) {
    pr_info!("write test.\n");
    mmiotrace_printk!("Write test.\n");

    for i in 0..256u32 {
        iowrite8(i as u8, p.add(i as usize));
    }

    let mut i = 1024u32;
    while i < 5 * 1024 {
        iowrite16(v16(i) as u16, p.add(i as usize));
        i += 2;
    }

    i = 5 * 1024;
    while i < 16 * 1024 {
        iowrite32(v32(i), p.add(i as usize));
        i += 4;
    }
}

unsafe fn do_read_test(p: *mut ::core::ffi::c_void) {
    let mut errs = [0u32; 3];
    pr_info!("read test.\n");
    mmiotrace_printk!("Read test.\n");

    for i in 0..256u32 {
        if ioread8(p.add(i as usize)) != i as u8 {
            errs[0] += 1;
        }
    }

    let mut i = 1024u32;
    while i < 5 * 1024 {
        if ioread16(p.add(i as usize)) != v16(i) as u16 {
            errs[1] += 1;
        }
        i += 2;
    }

    i = 5 * 1024;
    while i < 16 * 1024 {
        if ioread32(p.add(i as usize)) != v32(i) {
            errs[2] += 1;
        }
        i += 4;
    }

    mmiotrace_printk!("Read errors: 8-bit %d, 16-bit %d, 32-bit %d.\n", errs[0], errs[1], errs[2]);
}

unsafe fn do_read_far_test(p: *mut ::core::ffi::c_void) {
    pr_info!("read far test.\n");
    mmiotrace_printk!("Read far test.\n");
    ioread32(p.add(read_far as usize));
}

unsafe fn do_test(size: usize) {
    let p = ioremap(mmio_address, size as ::core::primitive::c_ulong);
    if p.is_null() {
        pr_err!("could not ioremap, aborting.\n");
        return;
    }
    mmiotrace_printk!("ioremap returned %p.\n", p);
    do_write_test(p);
    do_read_test(p);
    if read_far != 0 && read_far < (size - 4) as ::core::primitive::c_ulong {
        do_read_far_test(p);
    }
    iounmap(p);
}

unsafe fn do_test_bulk_ioremapping() {
    for _ in 0..10 {
        let p = ioremap(mmio_address, PAGE_SIZE as ::core::primitive::c_ulong);
        if !p.is_null() {
            iounmap(p);
        }
    }

    // Force freeing. If it will crash we will know why.
    synchronize_rcu();
}

unsafe fn init() -> i32 {
    let size: usize = if read_far != 0 { 8 << 20 } else { 16 << 10 };
    let ret = security_locked_down(LOCKDOWN_MMIOTRACE);

    if ret != 0 {
        return ret;
    }

    if mmio_address == 0 {
        pr_err!("you have to use the module argument mmio_address.\n");
        pr_err!("DO NOT LOAD THIS MODULE UNLESS YOU REALLY KNOW WHAT YOU ARE DOING!\n");
        return -6; // -ENXIO
    }

    pr_warn!("WARNING: mapping %lu kB @ 0x%08lx in PCI address space, and writing 16 kB of rubbish in there.\n", size >> 10, mmio_address);
    do_test(size);
    do_test_bulk_ioremapping();
    pr_info!("All done.\n");
    0
}

unsafe fn cleanup() {
    pr_debug!("unloaded.\n");
}

// module_init(init);
// module_exit(cleanup);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Test module for mmiotrace");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
