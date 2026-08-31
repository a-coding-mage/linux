// SPDX-License-Identifier: GPL-2.0
/* Copyright Amazon.com Inc. or its affiliates */
// Converted from includes:
// <linux/types.h>
// <linux/bpf.h>
// <bpf/bpf_helpers.h>
// <bpf/bpf_tracing.h>

pub const BUFF_SZ: usize = 512;

unsafe extern "C" {
    fn bpf_csum_diff(
        from: *mut core::ffi::c_void,
        from_size: u32,
        to: *mut core::ffi::c_void,
        to_size: u32,
        seed: u32,
    ) -> i64;
}

/* Will be updated by benchmark before program loading */
#[no_mangle]
pub static mut to_buff: [i8; BUFF_SZ] = [0; BUFF_SZ];
#[no_mangle]
pub static to_buff_len: u32 = 0;
#[no_mangle]
pub static mut from_buff: [i8; BUFF_SZ] = [0; BUFF_SZ];
#[no_mangle]
pub static from_buff_len: u32 = 0;
#[no_mangle]
pub static mut seed: u16 = 0;

#[no_mangle]
pub static mut result: i16 = 0;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn compute_checksum(ctx: *mut core::ffi::c_void) -> i32 {
    let to_len_half: i32 = to_buff_len as i32 / 2;
    let from_len_half: i32 = from_buff_len as i32 / 2;
    let mut result2: i16;

    /* Calculate checksum in one go */
    result2 = bpf_csum_diff(
        from_buff.as_mut_ptr() as *mut core::ffi::c_void,
        from_buff_len,
        to_buff.as_mut_ptr() as *mut core::ffi::c_void,
        to_buff_len,
        seed as u32,
    ) as i16;

    /* Calculate checksum by concatenating bpf_csum_diff()*/
    result = bpf_csum_diff(
        from_buff.as_mut_ptr() as *mut core::ffi::c_void,
        from_buff_len.wrapping_sub(from_len_half as u32),
        to_buff.as_mut_ptr() as *mut core::ffi::c_void,
        to_buff_len.wrapping_sub(to_len_half as u32),
        seed as u32,
    ) as i16;

    result = bpf_csum_diff(
        (from_buff.as_mut_ptr() as *mut core::ffi::c_void)
            .wrapping_add(from_buff_len.wrapping_sub(from_len_half as u32) as usize),
        from_len_half as u32,
        (to_buff.as_mut_ptr() as *mut core::ffi::c_void)
            .wrapping_add(to_buff_len.wrapping_sub(to_len_half as u32) as usize),
        to_len_half as u32,
        result as u32,
    ) as i16;

    result = if result == result2 { result } else { 0 };

    0
}
