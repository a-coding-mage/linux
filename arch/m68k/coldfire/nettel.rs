// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/

/*
 *	nettel.c -- startup code support for the NETtel boards
 *
 *	Copyright (C) 2009, Greg Ungerer (gerg@snapgear.com)
 */

/***************************************************************************/

// Linux and architecture dependencies are supplied by the surrounding tree.

/***************************************************************************/

/* Define the IO and interrupt resources of the 2 SMC9196 interfaces. */
const NETTEL_SMC0_ADDR: u64 = 0x30600300;
const NETTEL_SMC0_IRQ: u64 = 29;

const NETTEL_SMC1_ADDR: u64 = 0x30600000;
const NETTEL_SMC1_IRQ: u64 = 27;

/*
 * We need some access into the SMC9196 registers. Define those registers
 * we will need here (including the smc91x.h doesn't seem to give us these
 * in a simple form).
 */
const SMC91XX_BANKSELECT: u64 = 14;
const SMC91XX_BASEADDR: u64 = 2;
const SMC91XX_BASEMAC: u64 = 4;

/***************************************************************************/

static mut nettel_smc91x_0_resources: [resource; 2] = [
    resource {
        start: NETTEL_SMC0_ADDR,
        end: NETTEL_SMC0_ADDR + 0x20,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: NETTEL_SMC0_IRQ,
        end: NETTEL_SMC0_IRQ,
        flags: IORESOURCE_IRQ,
    },
];

static mut nettel_smc91x_1_resources: [resource; 2] = [
    resource {
        start: NETTEL_SMC1_ADDR,
        end: NETTEL_SMC1_ADDR + 0x20,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: NETTEL_SMC1_IRQ,
        end: NETTEL_SMC1_IRQ,
        flags: IORESOURCE_IRQ,
    },
];

static mut nettel_smc91x: [platform_device; 2] = [
    platform_device {
        name: "smc91x",
        id: 0,
        num_resources: nettel_smc91x_0_resources.len(),
        resource: nettel_smc91x_0_resources.as_mut_ptr(),
    },
    platform_device {
        name: "smc91x",
        id: 1,
        num_resources: nettel_smc91x_1_resources.len(),
        resource: nettel_smc91x_1_resources.as_mut_ptr(),
    },
];

static mut nettel_devices: [*mut platform_device; 2] = unsafe {
    [
        &mut nettel_smc91x[0] as *mut platform_device,
        &mut nettel_smc91x[1] as *mut platform_device,
    ]
};

/***************************************************************************/

static mut nettel_macdefault: [u8; 6] = [
    0x00, 0xd0, 0xcf, 0x00, 0x00, 0x01,
];

/*
 * Set flash contained MAC address into SMC9196 core. Make sure the flash
 * MAC address is sane, and not an empty flash. If no good use the Moreton
 * Bay default MAC address instead.
 */

unsafe fn nettel_smc91x_setmac(ioaddr: u32, flashaddr: u32) {
    let mut macp = flashaddr as *mut u16;
    if (*macp.add(0) == 0xffff) && (*macp.add(1) == 0xffff) && (*macp.add(2) == 0xffff) {
        macp = nettel_macdefault.as_mut_ptr() as *mut u16;
    }

    mcf_write16(1, (NETTEL_SMC0_ADDR + SMC91XX_BANKSELECT) as u32);
    mcf_write16(*macp.add(0), ioaddr + SMC91XX_BASEMAC as u32);
    mcf_write16(*macp.add(1), ioaddr + (SMC91XX_BASEMAC + 2) as u32);
    mcf_write16(*macp.add(2), ioaddr + (SMC91XX_BASEMAC + 4) as u32);
}

/***************************************************************************/

/*
 * Re-map the address space of at least one of the SMC ethernet
 * parts. Both parts power up decoding the same address, so we
 * need to move one of them first, before doing anything else.
 */

unsafe fn nettel_smc91x_init() {
    mcf_write16(0x00ec, MCFSIM_PADDR);
    mcf_setppdata(0, 0x0080);
    mcf_write16(1, (NETTEL_SMC0_ADDR + SMC91XX_BANKSELECT) as u32);
    mcf_write16(0x0067, (NETTEL_SMC0_ADDR + SMC91XX_BASEADDR) as u32);
    mcf_setppdata(0x0080, 0);

    /* Set correct chip select timing for SMC9196 accesses */
    mcf_write16(0x1180, MCFSIM_CSCR3);

    /* Set the SMC interrupts to be auto-vectored */
    mcf_autovector(NETTEL_SMC0_IRQ as u32);
    mcf_autovector(NETTEL_SMC1_IRQ as u32);

    /* Set MAC addresses from flash for both interfaces */
    nettel_smc91x_setmac(NETTEL_SMC0_ADDR as u32, 0xf0006000);
    nettel_smc91x_setmac(NETTEL_SMC1_ADDR as u32, 0xf0006006);
}

/***************************************************************************/

unsafe fn init_nettel() -> i32 {
    nettel_smc91x_init();
    platform_add_devices(nettel_devices.as_mut_ptr(), nettel_devices.len());
    0
}

// arch_initcall(init_nettel);

/***************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
