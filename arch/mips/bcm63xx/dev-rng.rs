/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2011 Florian Fainelli <florian@openwrt.org>
 */

// Dependencies supplied by the Linux kernel and bcm63xx headers are external
// to this translation unit.

#[repr(C)]
pub struct Resource {
    pub start: isize,
    pub end: isize,
    pub flags: u64,
}

#[repr(C)]
pub struct PlatformDevice {
    pub name: *const u8,
    pub id: i32,
    pub num_resources: usize,
    pub resource: *mut Resource,
}

extern "C" {
    static IORESOURCE_MEM: u64;
    fn BCMCPU_IS_6368() -> bool;
    fn bcm63xx_regset_address(regset: i32) -> isize;
    fn platform_device_register(device: *mut PlatformDevice) -> i32;
}

extern "C" {
    static RSET_RNG: i32;
    static RSET_RNG_SIZE: isize;
}

static mut rng_resources: [Resource; 1] = [Resource {
    start: -1, /* filled at runtime */
    end: -1,   /* filled at runtime */
    flags: 0,   /* IORESOURCE_MEM, supplied by the external kernel headers */
}];

static mut bcm63xx_rng_device: PlatformDevice = PlatformDevice {
    name: b"bcm63xx-rng\0".as_ptr(),
    id: -1,
    num_resources: 1,
    resource: core::ptr::addr_of_mut!(rng_resources) as *mut Resource,
};

unsafe fn bcm63xx_rng_register() -> i32 {
    if !BCMCPU_IS_6368() {
        return -19; // -ENODEV
    }

    rng_resources[0].flags = IORESOURCE_MEM;
    rng_resources[0].start = bcm63xx_regset_address(RSET_RNG);
    rng_resources[0].end = rng_resources[0].start;
    rng_resources[0].end += RSET_RNG_SIZE - 1;

    platform_device_register(core::ptr::addr_of_mut!(bcm63xx_rng_device))
}

// arch_initcall(bcm63xx_rng_register);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
