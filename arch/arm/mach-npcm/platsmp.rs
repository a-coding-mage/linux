// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Nuvoton Technology corporation.
// Copyright 2018 Google, Inc.

// #define pr_fmt(fmt) "nuvoton,npcm7xx-smp: " fmt
// C header dependencies are supplied by the surrounding kernel translation.

const NPCM7XX_SCRPAD_REG: usize = 0x13c;

extern "C" {
    fn npcm7xx_secondary_startup();
}

// External kernel types and functions referenced by this translation.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const core::ffi::c_char,
        compatible: *const core::ffi::c_char,
    ) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn of_node_put(node: *mut device_node);
    fn iowrite32(value: u32, addr: *mut core::ffi::c_void);
    fn __pa_symbol(symbol: unsafe extern "C" fn()) -> usize;
    fn dsb_sev();
    fn iounmap(addr: *mut core::ffi::c_void);
    fn scu_enable(base: *mut core::ffi::c_void);
}

const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;

unsafe fn npcm7xx_smp_boot_secondary(
    _cpu: u32,
    _idle: *mut task_struct,
) -> i32 {
    let mut gcr_np: *mut device_node;
    let gcr_base: *mut core::ffi::c_void;
    let mut ret: i32 = 0;

    gcr_np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"nuvoton,npcm750-gcr\0".as_ptr() as *const core::ffi::c_char,
    );
    if gcr_np.is_null() {
        // pr_err("no gcr device node\n");
        ret = -ENODEV;
        return ret;
    }
    gcr_base = of_iomap(gcr_np, 0);
    of_node_put(gcr_np);
    if gcr_base.is_null() {
        // pr_err("could not iomap gcr");
        ret = -ENOMEM;
        return ret;
    }

    /* give boot ROM kernel start address. */
    iowrite32(
        __pa_symbol(npcm7xx_secondary_startup),
        gcr_base.add(NPCM7XX_SCRPAD_REG),
    );
    /* make sure the previous write is seen by all observers. */
    dsb_sev();

    iounmap(gcr_base);
    ret
}

unsafe fn npcm7xx_smp_prepare_cpus(_max_cpus: u32) {
    let mut scu_np: *mut device_node;
    let scu_base: *mut core::ffi::c_void;

    scu_np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"arm,cortex-a9-scu\0".as_ptr() as *const core::ffi::c_char,
    );
    if scu_np.is_null() {
        // pr_err("no scu device node\n");
        return;
    }
    scu_base = of_iomap(scu_np, 0);
    of_node_put(scu_np);
    if scu_base.is_null() {
        // pr_err("could not iomap scu");
        return;
    }

    scu_enable(scu_base);

    iounmap(scu_base);
}

#[repr(C)]
struct smp_operations {
    smp_prepare_cpus: unsafe fn(u32),
    smp_boot_secondary: unsafe fn(u32, *mut task_struct) -> i32,
}

static mut npcm7xx_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: npcm7xx_smp_prepare_cpus,
    smp_boot_secondary: npcm7xx_smp_boot_secondary,
};

// CPU_METHOD_OF_DECLARE(npcm7xx_smp, "nuvoton,npcm750-smp", &npcm7xx_smp_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
