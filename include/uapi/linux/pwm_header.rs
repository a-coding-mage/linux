/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */

// Dependency intent: linux/ioctl.h and linux/types.h.

/// Describe a PWM waveform for a pwm_chip's PWM channel.
///
/// - `hwpwm`: per-chip relative index of the PWM device
/// - `__pad`: padding, must be zero
/// - `period_length_ns`: duration of the repeating period. A value of 0
///   represents a disabled PWM.
/// - `duty_length_ns`: duration of the active part in each period
/// - `duty_offset_ns`: offset of the rising edge from a period's start
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwmchip_waveform {
    pub hwpwm: u32,
    pub __pad: u32,
    pub period_length_ns: u64,
    pub duty_length_ns: u64,
    pub duty_offset_ns: u64,
}

const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_NONE: u32 = 0;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, kind: u32, nr: u32, size: u32) -> u32 {
    (dir << IOC_DIRSHIFT)
        | (size << IOC_SIZESHIFT)
        | (kind << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
}

const fn io(kind: u32, nr: u32) -> u32 {
    ioc(IOC_NONE, kind, nr, 0)
}

const fn iowr<T>(kind: u32, nr: u32) -> u32 {
    ioc(IOC_READ | IOC_WRITE, kind, nr, core::mem::size_of::<T>() as u32)
}

const fn iow<T>(kind: u32, nr: u32) -> u32 {
    ioc(IOC_WRITE, kind, nr, core::mem::size_of::<T>() as u32)
}

/* Reserves the passed hwpwm for exclusive control. */
pub const PWM_IOCTL_REQUEST: u32 = io(0x75, 1);

/* counter part to PWM_IOCTL_REQUEST */
pub const PWM_IOCTL_FREE: u32 = io(0x75, 2);

/*
 * Modifies the passed wf according to hardware constraints. All parameters are
 * rounded down to the next possible value, unless there is no such value, then
 * values are rounded up. Note that zero isn't considered for rounding down
 * period_length_ns.
 */
pub const PWM_IOCTL_ROUNDWF: u32 = iowr::<pwmchip_waveform>(0x75, 3);

/* Get the currently implemented waveform */
pub const PWM_IOCTL_GETWF: u32 = iowr::<pwmchip_waveform>(0x75, 4);

/* Like PWM_IOCTL_ROUNDWF + PWM_IOCTL_SETEXACTWF in one go. */
pub const PWM_IOCTL_SETROUNDEDWF: u32 = iow::<pwmchip_waveform>(0x75, 5);

/*
 * Program the PWM to emit exactly the passed waveform, subject only to rounding
 * down each value less than 1 ns. Returns 0 on success, -EDOM if the waveform
 * cannot be implemented exactly, or other negative error codes.
 */
pub const PWM_IOCTL_SETEXACTWF: u32 = iow::<pwmchip_waveform>(0x75, 6);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
