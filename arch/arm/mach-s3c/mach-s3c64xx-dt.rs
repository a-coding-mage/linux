// SPDX-License-Identifier: GPL-2.0
//
// Samsung's S3C64XX flattened device tree enabled machine
//
// Copyright (c) 2013 Tomasz Figa <tomasz.figa@gmail.com>

// Dependencies supplied by the corresponding architecture headers and source files.

extern "C" {
    fn debug_ll_io_init();
    fn iotable_init(io_desc: *mut map_desc, size: usize);
    fn s3c64xx_init_cpu();
    fn soc_is_s3c64xx() -> bool;
    fn panic(message: *const u8) -> !;
}

#[repr(C)]
pub struct map_desc {
    pub virtual_: usize,
    pub pfn: usize,
    pub length: usize,
    pub type_: usize,
}

// IO mapping for shared system controller IP.
//
// FIXME: Make remaining drivers use dynamic mapping.
static mut S3C64XX_DT_IODESC: [map_desc; 1] = [map_desc {
    virtual_: S3C_VA_SYS as usize,
    pfn: __phys_to_pfn(S3C64XX_PA_SYSCON),
    length: SZ_4K,
    type_: MT_DEVICE,
}];

extern "C" {
    static S3C_VA_SYS: usize;
    static S3C64XX_PA_SYSCON: usize;
    static SZ_4K: usize;
    static MT_DEVICE: usize;
    fn __phys_to_pfn(address: usize) -> usize;
}

unsafe fn s3c64xx_dt_map_io() {
    unsafe {
        debug_ll_io_init();
        iotable_init(
            S3C64XX_DT_IODESC.as_mut_ptr(),
            S3C64XX_DT_IODESC.len(),
        );

        s3c64xx_init_cpu();

        if !soc_is_s3c64xx() {
            panic(b"SoC is not S3C64xx!\0".as_ptr());
        }
    }
}

static S3C64XX_DT_COMPAT: [&[u8]; 3] = [
    b"samsung,s3c6400\0",
    b"samsung,s3c6410\0",
    b"\0",
];

// DT_MACHINE_START(S3C6400_DT, "Samsung S3C64xx (Flattened Device Tree)")
// Maintainer: Tomasz Figa <tomasz.figa@gmail.com>
// .dt_compat = s3c64xx_dt_compat,
// .map_io = s3c64xx_dt_map_io,
// MACHINE_END
// The surrounding architecture supplies the machine-registration macro/object.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
