// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OSS compatible sequencer driver
 *
 * midi device information
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

// C header dependencies:
// #include "seq_oss_device.h"
// #include <sound/seq_oss_legacy.h>

use core::ffi::{c_int, c_void};

unsafe extern "C" {
    pub fn snd_seq_oss_midi_lookup_ports(client: c_int) -> c_int;
    pub fn snd_seq_oss_midi_check_new_port(pinfo: *mut snd_seq_port_info) -> c_int;
    pub fn snd_seq_oss_midi_check_exit_port(client: c_int, port: c_int) -> c_int;
    pub fn snd_seq_oss_midi_clear_all();

    pub fn snd_seq_oss_midi_setup(dp: *mut seq_oss_devinfo);
    pub fn snd_seq_oss_midi_cleanup(dp: *mut seq_oss_devinfo);

    pub fn snd_seq_oss_midi_open(dp: *mut seq_oss_devinfo, dev: c_int, file_mode: c_int) -> c_int;
    pub fn snd_seq_oss_midi_open_all(dp: *mut seq_oss_devinfo, file_mode: c_int);
    pub fn snd_seq_oss_midi_close(dp: *mut seq_oss_devinfo, dev: c_int) -> c_int;
    pub fn snd_seq_oss_midi_reset(dp: *mut seq_oss_devinfo, dev: c_int);
    pub fn snd_seq_oss_midi_putc(
        dp: *mut seq_oss_devinfo,
        dev: c_int,
        c: u8,
        ev: *mut snd_seq_event,
        lockp: *mut *mut snd_use_lock_t,
    ) -> c_int;
    pub fn snd_seq_oss_midi_input(
        ev: *mut snd_seq_event,
        direct: c_int,
        private: *mut c_void,
    ) -> c_int;
    pub fn snd_seq_oss_midi_filemode(dp: *mut seq_oss_devinfo, dev: c_int) -> c_int;
    pub fn snd_seq_oss_midi_make_info(
        dp: *mut seq_oss_devinfo,
        dev: c_int,
        inf: *mut midi_info,
    ) -> c_int;
    pub fn snd_seq_oss_midi_get_addr(
        dp: *mut seq_oss_devinfo,
        dev: c_int,
        addr: *mut snd_seq_addr,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
