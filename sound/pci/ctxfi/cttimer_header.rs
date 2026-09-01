/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Timer handling
 */

/* Dependencies from the original header:
 * <linux/spinlock.h>
 * <linux/timer.h>
 * <linux/list.h>
 */

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ct_atc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ct_atc_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ct_timer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ct_timer_instance {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn ct_timer_new(atc: *mut ct_atc) -> *mut ct_timer;
    pub fn ct_timer_free(atimer: *mut ct_timer);

    pub fn ct_timer_instance_new(
        atimer: *mut ct_timer,
        apcm: *mut ct_atc_pcm,
    ) -> *mut ct_timer_instance;
    pub fn ct_timer_instance_free(ti: *mut ct_timer_instance);
    pub fn ct_timer_start(ti: *mut ct_timer_instance);
    pub fn ct_timer_stop(ti: *mut ct_timer_instance);
    pub fn ct_timer_prepare(ti: *mut ct_timer_instance);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
