// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Support code for virtual Ranchu board for MIPS.
 *
 * Author: Miodrag Dinic <miodrag.dinic@mips.com>
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_void};

const GOLDFISH_TIMER_LOW: usize = 0x00;
const GOLDFISH_TIMER_HIGH: usize = 0x04;

const NSEC_PER_SEC: u64 = 1_000_000_000;

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const c_char,
}

extern "C" {
    fn readl(addr: *const c_void) -> u32;
    fn of_find_compatible_node(
        from: *mut DeviceNode,
        type_: *const c_char,
        compatible: *const c_char,
    ) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: i32) -> *mut c_void;
    fn of_node_put(node: *mut DeviceNode);
    fn iounmap(addr: *mut c_void);
    fn read_c0_count() -> u32;
    fn panic(message: *const c_char, ... ) -> !;
}

unsafe fn read_rtc_time(base: *mut c_void) -> u64 {
    let time_low: u32;
    let time_high: u32;

    /*
     * Reading the low address latches the high value
     * as well so there is no fear that we may read
     * inaccurate high value.
     */
    time_low = readl((base as *mut u8).add(GOLDFISH_TIMER_LOW) as *const c_void);
    time_high = readl((base as *mut u8).add(GOLDFISH_TIMER_HIGH) as *const c_void);

    ((time_high as u64) << 32) | time_low as u64
}

unsafe fn ranchu_measure_hpt_freq() -> u32 {
    let rtc_start: u64;
    let mut rtc_current: u64;
    let mut rtc_delta: u64;
    let start: u32;
    let mut count: u32;
    let np: *mut DeviceNode;
    let rtc_base: *mut c_void;

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"google,goldfish-rtc\0".as_ptr() as *const c_char,
    );
    if np.is_null() {
        panic(b"%s(): Failed to find 'google,goldfish-rtc' dt node!\0".as_ptr() as *const c_char);
    }

    rtc_base = of_iomap(np, 0);
    of_node_put(np);
    if rtc_base.is_null() {
        panic(b"%s(): Failed to ioremap Goldfish RTC base!\0".as_ptr() as *const c_char);
    }

    /*
     * Poll the nanosecond resolution RTC for one
     * second to calibrate the CPU frequency.
     */
    rtc_start = read_rtc_time(rtc_base);
    start = read_c0_count();

    loop {
        rtc_current = read_rtc_time(rtc_base);
        rtc_delta = rtc_current.wrapping_sub(rtc_start);
        if rtc_delta >= NSEC_PER_SEC {
            break;
        }
    }

    count = read_c0_count().wrapping_sub(start);

    /*
     * Make sure the frequency will be a round number.
     * Without this correction, the returned value may vary
     * between subsequent emulation executions.
     *
     * TODO: Set this value using device tree.
     */
    count = count.wrapping_add(5000);
    count = count.wrapping_sub(count % 10000);

    iounmap(rtc_base);

    count
}

#[link_section = ".init.rodata"]
pub static ranchu_of_match: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mti,ranchu\0".as_ptr() as *const c_char,
    },
    OfDeviceId {
        compatible: core::ptr::null(),
    },
];

// Corresponds to: MIPS_MACHINE(ranchu) = { .matches = ranchu_of_match,
// .measure_hpt_freq = ranchu_measure_hpt_freq }.
#[repr(C)]
pub struct MipsMachine {
    pub matches: *const OfDeviceId,
    pub measure_hpt_freq: unsafe fn() -> u32,
}

#[no_mangle]
pub static mut ranchu: MipsMachine = MipsMachine {
    matches: ranchu_of_match.as_ptr(),
    measure_hpt_freq: ranchu_measure_hpt_freq,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
