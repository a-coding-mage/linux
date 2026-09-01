/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * cx20442.h  --  audio driver for CX20442
 *
 * Copyright 2009 Janusz Krzysztofik <jkrzyszt@tis.icnet.pl>
 */

// Header guard _CX20442_CODEC_H omitted in Rust.

#[repr(C)]
pub struct cx20442_codec {
    pub component: *mut snd_soc_component,
    pub ready: bool,
}

unsafe extern "C" {
    pub static mut v253_ops: tty_ldisc_ops;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
