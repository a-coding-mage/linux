// SPDX-License-Identifier: GPL-2.0 OR MIT

//
// Xen para-virtual sound device
//
// Copyright (C) 2016-2018 EPAM Systems Inc.
//
// Author: Oleksandr Andrushchenko <oleksandr_andrushchenko@epam.com>
//

// Forward declaration of struct defined elsewhere
#[repr(C)]
pub struct xen_snd_front_info;

// Forward declaration of struct defined elsewhere
#[repr(C)]
pub struct xen_snd_front_evtchnl;

extern "C" {
    pub fn xen_snd_front_alsa_init(front_info: *mut xen_snd_front_info) -> i32;

    pub fn xen_snd_front_alsa_fini(front_info: *mut xen_snd_front_info);

    pub fn xen_snd_front_alsa_handle_cur_pos(evtchnl: *mut xen_snd_front_evtchnl, pos_bytes: u64);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
