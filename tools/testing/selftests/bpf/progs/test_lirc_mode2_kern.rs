// SPDX-License-Identifier: GPL-2.0
// test ir decoder
//
// Copyright (C) 2018 Sean Young <sean@mess.org>

// C dependencies: <linux/bpf.h>, <linux/lirc.h>, <bpf/bpf_helpers.h>

extern "C" {
    fn LIRC_IS_PULSE(sample: u32) -> bool;
    fn LIRC_VALUE(sample: u32) -> u32;
    fn bpf_rc_keydown(ctx: *mut u32, protocol: u32, scancode: u32, toggle: u32);
    fn bpf_rc_pointer_rel(ctx: *mut u32, rel_x: u32, rel_y: u32);
}

#[no_mangle]
#[link_section = "lirc_mode2"]
pub unsafe extern "C" fn bpf_decoder(sample: *mut u32) -> i32 {
    if LIRC_IS_PULSE(*sample) {
        let duration: u32 = LIRC_VALUE(*sample);

        if duration & 0x1000 != 0 {
            bpf_rc_keydown(sample, 0x40, duration & 0xffff, 0);
        }
        if duration & 0x2000 != 0 {
            bpf_rc_pointer_rel(sample, (duration >> 8) & 0xff, duration & 0xff);
        }
    }

    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
