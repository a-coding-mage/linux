// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2018 Linus Torvalds. All rights reserved.
// Copyright(c) 2018 Alexei Starovoitov. All rights reserved.
// Copyright(c) 2018 Intel Corporation. All rights reserved.

// C dependencies: linux/compiler.h and asm/barrier.h provide
// OPTIMIZER_HIDE_VAR, BUILD_BUG_ON, and BITS_PER_LONG.

#[repr(C)]
pub struct task_struct {
    _opaque: [u8; 0],
}

#[macro_export]
macro_rules! barrier_nospec {
    () => {{ }};
}

/**
 * array_index_mask_nospec() - generate a ~0 mask when index < size, 0 otherwise
 * @index: array element index
 * @size: number of elements in array
 *
 * When @index is out of bounds (@index >= @size), the sign bit will be
 * set.  Extend the sign bit to all bits and invert, giving a result of
 * zero for an out of bounds index, or ~0 if within bounds [0, @size).
 */
#[inline]
pub fn array_index_mask_nospec(index: libc::c_ulong, size: libc::c_ulong) -> libc::c_ulong {
    /*
     * Always calculate and emit the mask even if the compiler
     * thinks the mask is not needed. The compiler does not take
     * into account the value of @index under speculation.
     */
    // OPTIMIZER_HIDE_VAR(index);
    let value = index | (size.wrapping_sub(1) .wrapping_sub(index));
    (!(value as libc::c_long) >> (libc::c_ulong::BITS - 1)) as libc::c_ulong
}

/*
 * array_index_nospec - sanitize an array index after a bounds check
 *
 * For a code sequence like:
 *
 *     if (index < size) {
 *         index = array_index_nospec(index, size);
 *         val = array[index];
 *     }
 *
 * ...if the CPU speculates past the bounds check then
 * array_index_nospec() will clamp the index within the range of [0,
 * size).
 */
#[macro_export]
macro_rules! array_index_nospec {
    ($index:expr, $size:expr) => {{
        let _i = $index;
        let _s = $size;
        let _mask = $crate::array_index_mask_nospec(_i as libc::c_ulong, _s as libc::c_ulong);

        // BUILD_BUG_ON(sizeof(_i) > sizeof(long));
        // BUILD_BUG_ON(sizeof(_s) > sizeof(long));

        (_i & (_mask as _))
    }};
}

/* Speculation control prctl */
extern "C" {
    pub fn arch_prctl_spec_ctrl_get(
        task: *mut task_struct,
        which: libc::c_ulong,
    ) -> libc::c_int;
    pub fn arch_prctl_spec_ctrl_set(
        task: *mut task_struct,
        which: libc::c_ulong,
        ctrl: libc::c_ulong,
    ) -> libc::c_int;
    /* Speculation control for seccomp enforced mitigation */
    pub fn arch_seccomp_spec_mitigate(task: *mut task_struct);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
