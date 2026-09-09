/* Copyright (c) 2016 Thomas Graf <tgraf@tgraf.ch>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 */

// C dependencies: vmlinux.h and <bpf/bpf_helpers.h>.

#[repr(C)]
pub struct LwtLenHistMap {
    _private: [u8; 0],
}

// Original SEC(".maps") map declaration:
// type = BPF_MAP_TYPE_PERCPU_HASH, key = u64, value = u64,
// pinning = LIBBPF_PIN_BY_NAME, max_entries = 1024.
#[no_mangle]
pub static mut lwt_len_hist_map: LwtLenHistMap = LwtLenHistMap { _private: [] };

unsafe fn log2(mut v: u32) -> u32 {
    let mut r: u32;
    let mut shift: u32;

    r = ((v > 0xFFFF) as u32) << 4;
    v >>= r;
    shift = ((v > 0xFF) as u32) << 3;
    v >>= shift;
    r |= shift;
    shift = ((v > 0xF) as u32) << 2;
    v >>= shift;
    r |= shift;
    shift = ((v > 0x3) as u32) << 1;
    v >>= shift;
    r |= shift;
    r |= v >> 1;
    r
}

unsafe fn log2l(v: u64) -> u32 {
    let hi: u32 = (v >> 32) as u32;
    if hi != 0 {
        log2(hi).wrapping_add(32)
    } else {
        log2(v as u32)
    }
}

// __sk_buff is supplied by vmlinux.h; BPF helpers and constants are supplied
// by <bpf/bpf_helpers.h>.
extern "C" {
    fn bpf_map_lookup_elem(map: *mut LwtLenHistMap, key: *const u64) -> *mut u64;
    fn bpf_map_update_elem(
        map: *mut LwtLenHistMap,
        key: *const u64,
        value: *const u64,
        flags: u64,
    ) -> i64;
}

#[no_mangle]
pub unsafe extern "C" fn do_len_hist(skb: *mut __sk_buff) -> i32 {
    let value: *mut u64;
    let key: u64;
    let init_val: u64 = 1;

    key = log2l((*skb).len as u64);

    value = bpf_map_lookup_elem(&mut lwt_len_hist_map, &key);
    if !value.is_null() {
        (*value) = (*value).wrapping_add(1);
    } else {
        bpf_map_update_elem(&mut lwt_len_hist_map, &key, &init_val, BPF_ANY as u64);
    }

    BPF_OK
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
