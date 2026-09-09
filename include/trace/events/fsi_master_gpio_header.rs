/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the Linux tracepoint header.  The tracepoint registration
// and formatting machinery is supplied by the surrounding trace subsystem.

#[repr(C)]
pub struct FsiMasterGpio {
    pub master: FsiMaster,
}

#[repr(C)]
pub struct FsiMaster {
    pub idx: ::core::ffi::c_int,
}

#[repr(C)]
pub struct FsiMasterGpioIn {
    pub master_idx: ::core::ffi::c_int,
    pub bits: ::core::ffi::c_int,
    pub msg: u64,
}

#[inline]
pub unsafe fn fsi_master_gpio_in(
    master: *const FsiMasterGpio,
    bits: ::core::ffi::c_int,
    msg: u64,
) -> FsiMasterGpioIn {
    FsiMasterGpioIn {
        master_idx: (*master).master.idx,
        bits,
        msg: msg & (1u64.wrapping_shl(bits as u32).wrapping_sub(1)),
    }
}

#[repr(C)]
pub struct FsiMasterGpioOut {
    pub master_idx: ::core::ffi::c_int,
    pub bits: ::core::ffi::c_int,
    pub msg: u64,
}

#[inline]
pub unsafe fn fsi_master_gpio_out(
    master: *const FsiMasterGpio,
    bits: ::core::ffi::c_int,
    msg: u64,
) -> FsiMasterGpioOut {
    FsiMasterGpioOut {
        master_idx: (*master).master.idx,
        bits,
        msg: msg & (1u64.wrapping_shl(bits as u32).wrapping_sub(1)),
    }
}

#[repr(C)]
pub struct FsiMasterGpioClockZeros {
    pub master_idx: ::core::ffi::c_int,
    pub clocks: ::core::ffi::c_int,
}

#[inline]
pub unsafe fn fsi_master_gpio_clock_zeros(
    master: *const FsiMasterGpio,
    clocks: ::core::ffi::c_int,
) -> FsiMasterGpioClockZeros {
    FsiMasterGpioClockZeros { master_idx: (*master).master.idx, clocks }
}

#[repr(C)]
pub struct FsiMasterGpioMasterEvent {
    pub master_idx: ::core::ffi::c_int,
}

#[inline]
pub unsafe fn fsi_master_gpio_break(master: *const FsiMasterGpio) -> FsiMasterGpioMasterEvent {
    FsiMasterGpioMasterEvent { master_idx: (*master).master.idx }
}

#[inline]
pub unsafe fn fsi_master_gpio_crc_cmd_error(master: *const FsiMasterGpio) -> FsiMasterGpioMasterEvent {
    FsiMasterGpioMasterEvent { master_idx: (*master).master.idx }
}

#[inline]
pub unsafe fn fsi_master_gpio_crc_rsp_error(master: *const FsiMasterGpio) -> FsiMasterGpioMasterEvent {
    FsiMasterGpioMasterEvent { master_idx: (*master).master.idx }
}

#[repr(C)]
pub struct FsiMasterGpioPollResponseBusy {
    pub master_idx: ::core::ffi::c_int,
    pub busy: ::core::ffi::c_int,
}

#[inline]
pub unsafe fn fsi_master_gpio_poll_response_busy(
    master: *const FsiMasterGpio,
    busy: ::core::ffi::c_int,
) -> FsiMasterGpioPollResponseBusy {
    FsiMasterGpioPollResponseBusy { master_idx: (*master).master.idx, busy }
}

#[repr(C)]
pub struct FsiMasterGpioCmdAbsAddr {
    pub master_idx: ::core::ffi::c_int,
    pub addr: u32,
}

#[inline]
pub unsafe fn fsi_master_gpio_cmd_abs_addr(
    master: *const FsiMasterGpio,
    addr: u32,
) -> FsiMasterGpioCmdAbsAddr {
    FsiMasterGpioCmdAbsAddr { master_idx: (*master).master.idx, addr }
}

#[repr(C)]
pub struct FsiMasterGpioCmdRelAddr {
    pub master_idx: ::core::ffi::c_int,
    pub rel_addr: u32,
}

#[inline]
pub unsafe fn fsi_master_gpio_cmd_rel_addr(
    master: *const FsiMasterGpio,
    rel_addr: u32,
) -> FsiMasterGpioCmdRelAddr {
    FsiMasterGpioCmdRelAddr { master_idx: (*master).master.idx, rel_addr }
}

#[inline]
pub unsafe fn fsi_master_gpio_cmd_same_addr(
    master: *const FsiMasterGpio,
) -> FsiMasterGpioMasterEvent {
    FsiMasterGpioMasterEvent { master_idx: (*master).master.idx }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
