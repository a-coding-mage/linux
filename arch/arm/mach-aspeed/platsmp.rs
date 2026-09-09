// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) ASPEED Technology Inc.
// Copyright IBM Corp.

// Translated dependencies: linux/of_address.h, linux/io.h, linux/of.h,
// linux/smp.h

const BOOT_ADDR: usize = 0x00;
const BOOT_SIG: usize = 0x04;

static mut secboot_node: *mut device_node = core::ptr::null_mut();

unsafe extern "C" {
    type device_node;
    type task_struct;
    type smp_operations;

    fn of_iomap(node: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const core::ffi::c_char,
        compatible: *const core::ffi::c_char,
    ) -> *mut device_node;
    fn pr_err(format: *const core::ffi::c_char, ...);
    fn writel_relaxed(value: u32, address: *mut core::ffi::c_void);
    fn __raw_writel(value: u32, address: *mut core::ffi::c_void);
    fn __pa_symbol(symbol: unsafe extern "C" fn()) -> usize;
    fn secondary_startup_arm();
    fn dsb_sev();
    fn iounmap(address: *mut core::ffi::c_void);
}

unsafe fn aspeed_g6_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    let base: *mut core::ffi::c_void;

    base = of_iomap(secboot_node, 0);
    if base.is_null() {
        pr_err(c"could not map the secondary boot base!".as_ptr());
        return -19; // -ENODEV
    }

    writel_relaxed(0, base.add(BOOT_ADDR));
    writel_relaxed(
        __pa_symbol(secondary_startup_arm as unsafe extern "C" fn()) as u32,
        base.add(BOOT_ADDR),
    );
    writel_relaxed(0xABBAAB00u32 | (cpu & 0xff), base.add(BOOT_SIG));

    dsb_sev();

    iounmap(base);

    0
}

unsafe fn aspeed_g6_smp_prepare_cpus(_max_cpus: u32) {
    let base: *mut core::ffi::c_void;

    secboot_node = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        c"aspeed,ast2600-smpmem".as_ptr(),
    );
    if secboot_node.is_null() {
        pr_err(c"secboot device node found!!\n".as_ptr());
        return;
    }

    base = of_iomap(secboot_node, 0);
    if base.is_null() {
        pr_err(c"could not map the secondary boot base!".as_ptr());
        return;
    }
    __raw_writel(0xBADABABA, base.add(BOOT_SIG));

    iounmap(base);
}

static aspeed_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: Some(aspeed_g6_smp_prepare_cpus),
    smp_boot_secondary: Some(aspeed_g6_boot_secondary),
};

// CPU_METHOD_OF_DECLARE(aspeed_smp, "aspeed,ast2600-smp", &aspeed_smp_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
