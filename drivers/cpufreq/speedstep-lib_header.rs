/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * (C) 2002 - 2003 Dominik Brodowski <linux@brodo.de>
 *
 *  Library for common functions for Intel SpeedStep v.1 and v.2 support
 *
 *  BIG FAT DISCLAIMER: Work in progress code. Possibly *dangerous*
 */

/* processors */
#[repr(u32)]
pub enum speedstep_processor {
    SPEEDSTEP_CPU_PIII_C_EARLY = 0x00000001, /* Coppermine core */
    SPEEDSTEP_CPU_PIII_C = 0x00000002,       /* Coppermine core */
    SPEEDSTEP_CPU_PIII_T = 0x00000003,       /* Tualatin core */
    SPEEDSTEP_CPU_P4M = 0x00000004,          /* P4-M  */
    /* the following processors are not speedstep-capable and are not auto-detected
     * in speedstep_detect_processor(). However, their speed can be detected using
     * the speedstep_get_frequency() call. */
    SPEEDSTEP_CPU_PM = 0xFFFFFF03,   /* Pentium M  */
    SPEEDSTEP_CPU_P4D = 0xFFFFFF04,  /* desktop P4  */
    SPEEDSTEP_CPU_PCORE = 0xFFFFFF05, /* Core */
}

/* speedstep states -- only two of them */
pub const SPEEDSTEP_HIGH: u32 = 0x00000000;
pub const SPEEDSTEP_LOW: u32 = 0x00000001;

/* detect a speedstep-capable processor */
unsafe extern "C" {
    pub fn speedstep_detect_processor() -> speedstep_processor;

    /* detect the current speed (in khz) of the processor */
    pub fn speedstep_get_frequency(processor: speedstep_processor) -> u32;

    /* detect the low and high speeds of the processor. The callback
     * set_state"'s first argument is either SPEEDSTEP_HIGH or SPEEDSTEP_LOW; the
     * second argument is zero so that no cpufreq_notify_transition calls are
     * initiated.
     */
    pub fn speedstep_get_freqs(
        processor: speedstep_processor,
        low_speed: *mut u32,
        high_speed: *mut u32,
        transition_latency: *mut u32,
        set_state: Option<unsafe extern "C" fn(state: u32)>,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
