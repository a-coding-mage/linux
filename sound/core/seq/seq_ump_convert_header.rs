// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ALSA sequencer event conversion between UMP and legacy clients
 */

// C dependencies: "seq_clientmgr.h", "seq_ports.h"

extern "C" {
    pub fn snd_seq_deliver_from_ump(
        source: *mut snd_seq_client,
        dest: *mut snd_seq_client,
        dest_port: *mut snd_seq_client_port,
        event: *mut snd_seq_event,
        atomic: core::ffi::c_int,
        hop: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn snd_seq_deliver_to_ump(
        source: *mut snd_seq_client,
        dest: *mut snd_seq_client,
        dest_port: *mut snd_seq_client_port,
        event: *mut snd_seq_event,
        atomic: core::ffi::c_int,
        hop: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn snd_seq_ump_group_port(event: *const snd_seq_event) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
