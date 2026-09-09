/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* apc - Driver definitions for power management functions
 * of Aurora Personality Chip (APC) on SPARCstation-4/5 and
 * derivatives
 *
 * Copyright (c) 2001 Eric Brower (ebrower@usa.net)
 *
 */

// C header dependency: <linux/ioctl.h>

pub const APC_IOC: u32 = 'A' as u32;

// Linux ioctl encoding for an int payload (size 4), equivalent to _IOR/_IOW.
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_DIRBITS: u32 = 2;
const IOC_NRMASK: u32 = (1 << IOC_NRBITS) - 1;
const IOC_TYPEMASK: u32 = (1 << IOC_TYPEBITS) - 1;
const IOC_SIZEMASK: u32 = (1 << IOC_SIZEBITS) - 1;
const IOC_DIRMASK: u32 = (1 << IOC_DIRBITS) - 1;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_NONE: u32 = 0;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    ((dir & IOC_DIRMASK) << IOC_DIRSHIFT)
        | ((ty & IOC_TYPEMASK) << IOC_TYPESHIFT)
        | ((nr & IOC_NRMASK) << IOC_NRSHIFT)
        | ((size & IOC_SIZEMASK) << IOC_SIZESHIFT)
}

const fn ior(ty: u32, nr: u32, size: u32) -> u32 {
    ioc(IOC_READ, ty, nr, size)
}

const fn iow(ty: u32, nr: u32, size: u32) -> u32 {
    ioc(IOC_WRITE, ty, nr, size)
}

pub const APCIOCGFANCTL: u32 = ior(APC_IOC, 0x00, core::mem::size_of::<i32>() as u32); /* Get fan speed */
pub const APCIOCSFANCTL: u32 = iow(APC_IOC, 0x01, core::mem::size_of::<i32>() as u32); /* Set fan speed */

pub const APCIOCGCPWR: u32 = ior(APC_IOC, 0x02, core::mem::size_of::<i32>() as u32); /* Get CPOWER state */
pub const APCIOCSCPWR: u32 = iow(APC_IOC, 0x03, core::mem::size_of::<i32>() as u32); /* Set CPOWER state */

pub const APCIOCGBPORT: u32 = ior(APC_IOC, 0x04, core::mem::size_of::<i32>() as u32); /* Get BPORT state */
pub const APCIOCSBPORT: u32 = iow(APC_IOC, 0x05, core::mem::size_of::<i32>() as u32); /* Set BPORT state */

/*
 * Register offsets
 */
pub const APC_IDLE_REG: u32 = 0x00;
pub const APC_FANCTL_REG: u32 = 0x20;
pub const APC_CPOWER_REG: u32 = 0x24;
pub const APC_BPORT_REG: u32 = 0x30;

pub const APC_REGMASK: u32 = 0x01;
pub const APC_BPMASK: u32 = 0x03;

/*
 * IDLE - CPU standby values (set to initiate standby)
 */
pub const APC_IDLE_ON: u32 = 0x01;

/*
 * FANCTL - Fan speed control state values
 */
pub const APC_FANCTL_HI: u32 = 0x00; /* Fan speed high */
pub const APC_FANCTL_LO: u32 = 0x01; /* Fan speed low */

/*
 * CPWR - Convenience power outlet state values
 */
pub const APC_CPOWER_ON: u32 = 0x00; /* Conv power on */
pub const APC_CPOWER_OFF: u32 = 0x01; /* Conv power off */

/*
 * BPA/BPB - Read-Write "Bit Ports" state values (reset to 0 at power-on)
 *
 * WARNING: Internal usage of bit ports is platform dependent--
 * don't modify BPORT settings unless you know what you are doing.
 *
 * On SS5 BPA seems to toggle onboard ethernet loopback... -E
 */
pub const APC_BPORT_A: u32 = 0x01; /* Bit Port A */
pub const APC_BPORT_B: u32 = 0x02; /* Bit Port B */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
