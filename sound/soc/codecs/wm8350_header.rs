/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * wm8350.h - WM8903 audio codec interface
 *
 * Copyright 2008 Wolfson Microelectronics PLC.
 */

/* Dependencies from the original C header:
 * #include <sound/soc.h>
 * #include <linux/mfd/wm8350/audio.h>
 */

#[repr(C)]
pub struct snd_soc_component {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum wm8350_jack {
    WM8350_JDL = 1,
    WM8350_JDR = 2,
}

unsafe extern "C" {
    pub fn wm8350_hp_jack_detect(
        component: *mut snd_soc_component,
        which: wm8350_jack,
        jack: *mut snd_soc_jack,
        report: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn wm8350_mic_jack_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        detect_report: ::std::os::raw::c_int,
        short_report: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
