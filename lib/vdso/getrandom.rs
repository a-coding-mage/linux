// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022-2024 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 */

use core::{mem, ptr};

/* Types and constants are supplied by the corresponding kernel vDSO headers. */
use crate::{
    vdso_rng_data, vgetrandom_opaque_params, vgetrandom_state, CHACHA_BLOCK_SIZE,
    GRND_INSECURE, GRND_NONBLOCK, GRND_RANDOM, MAP_ANONYMOUS, MAP_DROPPABLE, PAGE_MASK,
    PAGE_SIZE, PROT_READ, PROT_WRITE,
};

extern "C" {
    fn getrandom_syscall(buf: *mut u8, len: usize, flags: u32) -> isize;
    fn __arch_chacha20_blocks_nostack(
        dst: *mut u8,
        key: *const u8,
        counter: *mut u32,
        nblocks: usize,
    );
    fn __arch_get_vdso_u_rng_data() -> *const vdso_rng_data;
}

#[inline(always)]
unsafe fn memcpy_and_zero_src(mut dst: *mut u8, mut src: *mut u8, mut len: usize) {
    #[inline(always)]
    unsafe fn copy_zero<T: Copy>(dst: &mut *mut u8, src: &mut *mut u8, len: &mut usize) {
        while *len >= mem::size_of::<T>() {
            let value = ptr::read_unaligned((*src).cast::<T>());
            ptr::write_unaligned((*dst).cast::<T>(), value);
            ptr::write_unaligned((*src).cast::<T>(), unsafe { mem::zeroed() });
            *dst = (*dst).add(mem::size_of::<T>());
            *src = (*src).add(mem::size_of::<T>());
            *len -= mem::size_of::<T>();
        }
    }

    /* CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS and CONFIG_64BIT select these in C. */
    copy_zero::<u64>(&mut dst, &mut src, &mut len);
    copy_zero::<u32>(&mut dst, &mut src, &mut len);
    copy_zero::<u16>(&mut dst, &mut src, &mut len);
    copy_zero::<u8>(&mut dst, &mut src, &mut len);
}

/// Generic vDSO implementation of the getrandom() syscall.
#[inline(always)]
pub unsafe fn __cvdso_getrandom_data(
    rng_info: *const vdso_rng_data,
    mut buffer: *mut u8,
    mut len: usize,
    flags: u32,
    opaque_state: *mut u8,
    opaque_len: usize,
) -> isize {
    let ret = core::cmp::min((i32::MAX as usize) & PAGE_MASK, len) as isize;
    let state = opaque_state.cast::<vgetrandom_state>();
    let orig_len = len;
    let orig_buffer = buffer;
    let mut have_retried = false;
    let mut counter = [0u32; 2];

    if opaque_len == usize::MAX && buffer.is_null() && len == 0 && flags == 0 {
        let params = opaque_state.cast::<vgetrandom_opaque_params>();
        (*params).size_of_opaque_state = mem::size_of::<vgetrandom_state>();
        (*params).mmap_prot = PROT_READ | PROT_WRITE;
        (*params).mmap_flags = MAP_DROPPABLE | MAP_ANONYMOUS;
        for item in (*params).reserved.iter_mut() { *item = 0; }
        return 0;
    }

    if ((opaque_state as usize & !PAGE_MASK) + mem::size_of::<vgetrandom_state>() > PAGE_SIZE) {
        return -14; // EFAULT
    }
    if flags & !(GRND_NONBLOCK | GRND_RANDOM | GRND_INSECURE) != 0 { return fallback_syscall(orig_buffer, orig_len, flags); }
    if opaque_len != mem::size_of::<vgetrandom_state>() { return fallback_syscall(orig_buffer, orig_len, flags); }
    if ptr::read_volatile(&(*rng_info).is_ready) == 0 { return fallback_syscall(orig_buffer, orig_len, flags); }
    if len == 0 { return 0; }
    if ptr::read_volatile(&(*state).in_use) { return fallback_syscall(orig_buffer, orig_len, flags); }
    ptr::write_volatile(&mut (*state).in_use, true);

    'retry_generation: loop {
        let current_generation = ptr::read_volatile(&(*rng_info).generation);
        if (*state).generation != current_generation {
            ptr::write_volatile(&mut (*state).generation, current_generation);
            core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
            if getrandom_syscall((*state).key.as_mut_ptr(), (*state).key.len(), 0) != (*state).key.len() as isize {
                ptr::write_volatile(&mut (*state).generation, 0);
                ptr::write_volatile(&mut (*state).in_use, false);
                return fallback_syscall(orig_buffer, orig_len, flags);
            }
            (*state).pos = (*state).batch.len();
        }

        len = ret as usize;
        loop {
            let batch_len = core::cmp::min((*state).batch.len() - (*state).pos, len);
            if batch_len != 0 {
                memcpy_and_zero_src(buffer, (*state).batch.as_mut_ptr().add((*state).pos), batch_len);
                (*state).pos += batch_len; buffer = buffer.add(batch_len); len -= batch_len;
            }
            if len == 0 {
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
                if ptr::read_volatile(&(*state).generation) != ptr::read_volatile(&(*rng_info).generation) {
                    if have_retried { ptr::write_volatile(&mut (*state).in_use, false); return fallback_syscall(orig_buffer, orig_len, flags); }
                    have_retried = true; buffer = orig_buffer; continue 'retry_generation;
                }
                ptr::write_volatile(&mut (*state).in_use, false); return ret;
            }
            let nblocks = len / CHACHA_BLOCK_SIZE;
            if nblocks != 0 {
                __arch_chacha20_blocks_nostack(buffer, (*state).key.as_ptr(), counter.as_mut_ptr(), nblocks);
                buffer = buffer.add(nblocks * CHACHA_BLOCK_SIZE); len -= nblocks * CHACHA_BLOCK_SIZE;
            }
            __arch_chacha20_blocks_nostack((*state).batch.as_mut_ptr(), (*state).key.as_ptr(), counter.as_mut_ptr(), (*state).batch.len() / CHACHA_BLOCK_SIZE);
            (*state).pos = 0;
        }
    }
}

#[inline(always)]
pub unsafe fn __cvdso_getrandom(buffer: *mut u8, len: usize, flags: u32, opaque_state: *mut u8, opaque_len: usize) -> isize {
    __cvdso_getrandom_data(__arch_get_vdso_u_rng_data(), buffer, len, flags, opaque_state, opaque_len)
}

#[inline(always)]
unsafe fn fallback_syscall(buffer: *mut u8, len: usize, flags: u32) -> isize {
    getrandom_syscall(buffer, len, flags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
