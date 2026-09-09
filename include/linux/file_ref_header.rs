/* SPDX-License-Identifier: GPL-2.0-only */

/* Dependencies supplied by the surrounding kernel translation. */

#[cfg(target_pointer_width = "64")]
pub const FILE_REF_ONEREF: u64 = 0x0000_0000_0000_0000;
#[cfg(target_pointer_width = "64")]
pub const FILE_REF_MAXREF: u64 = 0x7fff_ffff_ffff_ffff;
#[cfg(target_pointer_width = "64")]
pub const FILE_REF_SATURATED: u64 = 0xa000_0000_0000_0000;
#[cfg(target_pointer_width = "64")]
pub const FILE_REF_RELEASED: u64 = 0xc000_0000_0000_0000;
#[cfg(target_pointer_width = "64")]
pub const FILE_REF_DEAD: u64 = 0xe000_0000_0000_0000;
#[cfg(target_pointer_width = "64")]
pub const FILE_REF_NOREF: u64 = 0xffff_ffff_ffff_ffff;

#[cfg(target_pointer_width = "32")]
pub const FILE_REF_ONEREF: u32 = 0x0000_0000;
#[cfg(target_pointer_width = "32")]
pub const FILE_REF_MAXREF: u32 = 0x7fff_ffff;
#[cfg(target_pointer_width = "32")]
pub const FILE_REF_SATURATED: u32 = 0xa000_0000;
#[cfg(target_pointer_width = "32")]
pub const FILE_REF_RELEASED: u32 = 0xc000_0000;
#[cfg(target_pointer_width = "32")]
pub const FILE_REF_DEAD: u32 = 0xe000_0000;
#[cfg(target_pointer_width = "32")]
pub const FILE_REF_NOREF: u32 = 0xffff_ffff;

#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub struct file_ref_t {
    pub refcnt: atomic64_t,
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
pub struct file_ref_t {
    pub refcnt: atomic_t,
}

extern "C" {
    pub fn __file_ref_put(ref_: *mut file_ref_t, cnt: ::core::ffi::c_ulong) -> bool;
}

#[inline]
pub unsafe fn file_ref_init(ref_: *mut file_ref_t, cnt: ::core::ffi::c_ulong) {
    atomic_long_set(&mut (*ref_).refcnt, cnt.wrapping_sub(1));
}

#[inline]
pub unsafe fn file_ref_get(ref_: *mut file_ref_t) -> bool {
    !atomic_long_add_negative(1, &mut (*ref_).refcnt)
}

#[inline]
pub unsafe fn file_ref_inc(ref_: *mut file_ref_t) {
    let prior: ::core::ffi::c_long = atomic_long_fetch_inc_relaxed(&mut (*ref_).refcnt);
    WARN_ONCE(prior < 0, "file_ref_inc() on a released file reference");
}

#[inline]
pub unsafe fn file_ref_put(ref_: *mut file_ref_t) -> bool {
    let _preempt_guard = guard(preempt());
    let cnt: ::core::ffi::c_long = atomic_long_dec_return(&mut (*ref_).refcnt);
    if cnt >= 0 {
        return false;
    }
    __file_ref_put(ref_, cnt as ::core::ffi::c_ulong)
}

#[inline]
pub unsafe fn file_ref_put_close(ref_: *mut file_ref_t) -> bool {
    let mut old: ::core::ffi::c_long = atomic_long_read(&(*ref_).refcnt);
    if likely(old == FILE_REF_ONEREF as ::core::ffi::c_long) {
        if likely(atomic_long_try_cmpxchg(
            &mut (*ref_).refcnt,
            &mut old,
            FILE_REF_DEAD as ::core::ffi::c_long,
        )) {
            return true;
        }
    }
    file_ref_put(ref_)
}

#[inline]
pub unsafe fn file_ref_read(ref_: *mut file_ref_t) -> ::core::ffi::c_ulong {
    let c: ::core::ffi::c_ulong = atomic_long_read(&(*ref_).refcnt) as ::core::ffi::c_ulong;
    if c >= FILE_REF_RELEASED as ::core::ffi::c_ulong {
        0
    } else {
        c.wrapping_add(1)
    }
}

#[inline]
pub unsafe fn __file_ref_read_raw(ref_: *mut file_ref_t) -> ::core::ffi::c_ulong {
    atomic_long_read(&(*ref_).refcnt) as ::core::ffi::c_ulong
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
