/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2009-2011 Florian Fainelli <florian@openwrt.org>
 * Copyright (C) 2010 Tanguy Bouzeloc <tanguy.bouzeloc@efixo.com>
 */

// Dependencies supplied by the corresponding Linux and BCM63xx headers.

#[repr(C)]
struct Resource {
    start: isize,
    end: isize,
    flags: u64,
}

#[repr(C)]
struct PlatformDevice {
    name: *const core::ffi::c_char,
    id: i32,
    num_resources: usize,
    resource: *mut Resource,
}

extern "C" {
    fn bcm63xx_regset_address(regset: i32) -> isize;
    fn bcm63xx_get_irq_number(irq: i32) -> isize;
    fn platform_device_register(device: *mut PlatformDevice) -> i32;
}

extern "Rust" {
    fn BCMCPU_IS_6328() -> bool;
    fn BCMCPU_IS_6345() -> bool;
    fn BCMCPU_IS_6338() -> bool;
    fn BCMCPU_IS_6348() -> bool;
    fn BCMCPU_IS_3368() -> bool;
    fn BCMCPU_IS_6358() -> bool;
    fn BCMCPU_IS_6362() -> bool;
    fn BCMCPU_IS_6368() -> bool;
}

const RSET_SPI: i32 = 0;
const IRQ_SPI: i32 = 0;
const IORESOURCE_MEM: u64 = 0;
const IORESOURCE_IRQ: u64 = 0;
const BCM_6348_RSET_SPI_SIZE: isize = 0;
const BCM_6358_RSET_SPI_SIZE: isize = 0;

static mut SPI_RESOURCES: [Resource; 2] = [
    Resource {
        start: -1,
        end: -1,
        flags: IORESOURCE_MEM,
    },
    Resource {
        start: -1,
        end: 0,
        flags: IORESOURCE_IRQ,
    },
];

static mut BCM63XX_SPI_DEVICE: PlatformDevice = PlatformDevice {
    name: core::ptr::null(),
    id: -1,
    num_resources: 2,
    resource: core::ptr::null_mut(),
};

// __init
pub unsafe extern "C" fn bcm63xx_spi_register() -> i32 {
    if BCMCPU_IS_6328() || BCMCPU_IS_6345() {
        return -19; // -ENODEV
    }

    SPI_RESOURCES[0].start = bcm63xx_regset_address(RSET_SPI);
    SPI_RESOURCES[0].end = SPI_RESOURCES[0].start;
    SPI_RESOURCES[1].start = bcm63xx_get_irq_number(IRQ_SPI);

    if BCMCPU_IS_6338() || BCMCPU_IS_6348() {
        BCM63XX_SPI_DEVICE.name = b"bcm6348-spi\0".as_ptr() as *const core::ffi::c_char;
        SPI_RESOURCES[0].end = SPI_RESOURCES[0]
            .end
            .wrapping_add(BCM_6348_RSET_SPI_SIZE)
            .wrapping_sub(1);
    }

    if BCMCPU_IS_3368() || BCMCPU_IS_6358() || BCMCPU_IS_6362() || BCMCPU_IS_6368() {
        BCM63XX_SPI_DEVICE.name = b"bcm6358-spi\0".as_ptr() as *const core::ffi::c_char;
        SPI_RESOURCES[0].end = SPI_RESOURCES[0]
            .end
            .wrapping_add(BCM_6358_RSET_SPI_SIZE)
            .wrapping_sub(1);
    }

    BCM63XX_SPI_DEVICE.resource = SPI_RESOURCES.as_mut_ptr();
    platform_device_register(&mut BCM63XX_SPI_DEVICE)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
