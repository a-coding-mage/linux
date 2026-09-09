// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-tegra/io.c
 *
 * Copyright (C) 2010 Google, Inc.
 *
 * Author:
 *	Colin Cross <ccross@google.com>
 *	Erik Gilling <konkers@google.com>
 */

// Linux kernel and architecture headers supplied by the surrounding tree.

#[repr(C)]
pub struct map_desc {
    pub virtual_: ::core::ffi::c_ulong,
    pub pfn: ::core::ffi::c_ulong,
    pub length: ::core::ffi::c_ulong,
    pub type_: ::core::ffi::c_int,
}

extern "C" {
    fn debug_ll_io_init();
    fn iotable_init(io_desc: *mut map_desc, nr: usize);
}

// These constants and conversion function are provided by board/iomap and
// the architecture headers in the surrounding kernel tree.
extern "C" {
    static IO_PPSB_VIRT: ::core::ffi::c_ulong;
    static IO_PPSB_PHYS: ::core::ffi::c_ulong;
    static IO_PPSB_SIZE: ::core::ffi::c_ulong;
    static IO_APB_VIRT: ::core::ffi::c_ulong;
    static IO_APB_PHYS: ::core::ffi::c_ulong;
    static IO_APB_SIZE: ::core::ffi::c_ulong;
    static IO_CPU_VIRT: ::core::ffi::c_ulong;
    static IO_CPU_PHYS: ::core::ffi::c_ulong;
    static IO_CPU_SIZE: ::core::ffi::c_ulong;
    static IO_IRAM_VIRT: ::core::ffi::c_ulong;
    static IO_IRAM_PHYS: ::core::ffi::c_ulong;
    static IO_IRAM_SIZE: ::core::ffi::c_ulong;
    static MT_DEVICE: ::core::ffi::c_int;
}

// `__phys_to_pfn` is a build-time architecture macro in the C source; its
// equivalent is retained here as an external dependency.
extern "C" {
    fn __phys_to_pfn(phys: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
}

// C `__initdata`; initialization is expressed through the corresponding
// low-level values supplied by the surrounding kernel.
#[allow(non_upper_case_globals)]
static mut tegra_io_desc: [map_desc; 4] = [
    map_desc {
        virtual_: unsafe { IO_PPSB_VIRT },
        pfn: unsafe { __phys_to_pfn(IO_PPSB_PHYS) },
        length: unsafe { IO_PPSB_SIZE },
        type_: unsafe { MT_DEVICE },
    },
    map_desc {
        virtual_: unsafe { IO_APB_VIRT },
        pfn: unsafe { __phys_to_pfn(IO_APB_PHYS) },
        length: unsafe { IO_APB_SIZE },
        type_: unsafe { MT_DEVICE },
    },
    map_desc {
        virtual_: unsafe { IO_CPU_VIRT },
        pfn: unsafe { __phys_to_pfn(IO_CPU_PHYS) },
        length: unsafe { IO_CPU_SIZE },
        type_: unsafe { MT_DEVICE },
    },
    map_desc {
        virtual_: unsafe { IO_IRAM_VIRT },
        pfn: unsafe { __phys_to_pfn(IO_IRAM_PHYS) },
        length: unsafe { IO_IRAM_SIZE },
        type_: unsafe { MT_DEVICE },
    },
];

pub unsafe extern "C" fn tegra_map_common_io() {
    debug_ll_io_init();
    iotable_init(tegra_io_desc.as_mut_ptr(), tegra_io_desc.len());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
