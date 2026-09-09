/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from vdso/helpers.h; external VDSO types, constants, and barriers
 * are supplied by the surrounding translation unit. */

#[inline(always)]
pub unsafe fn vdso_is_timens_clock(vc: *const vdso_clock) -> bool {
    // CONFIG_TIME_NS is a build-time configuration condition.
    cfg!(feature = "time_ns") && (*vc).clock_mode == VDSO_CLOCKMODE_TIMENS
}

#[inline(always)]
pub unsafe fn vdso_read_begin(vc: *const vdso_clock) -> u32 {
    let mut seq: u32;

    loop {
        seq = core::ptr::read_volatile(core::ptr::addr_of!((*vc).seq));
        if (seq & 1) == 0 {
            break;
        }
        cpu_relax();
    }

    smp_rmb();
    seq
}

/*
 * Variant of vdso_read_begin() to handle VDSO_CLOCKMODE_TIMENS.
 *
 * Time namespace enabled tasks have a special VVAR page installed which has
 * vc->seq set to 1 and vc->clock_mode set to VDSO_CLOCKMODE_TIMENS. For non
 * time namespace affected tasks this does not affect performance because if
 * vc->seq is odd, i.e. a concurrent update is in progress the extra check for
 * vc->clock_mode is just a few extra instructions while spin waiting for
 * vc->seq to become even again.
 */
#[inline(always)]
pub unsafe fn vdso_read_begin_timens(vc: *const vdso_clock, seq: *mut u32) -> bool {
    loop {
        *seq = core::ptr::read_volatile(core::ptr::addr_of!((*vc).seq));
        if (*seq & 1) == 0 {
            break;
        }
        if vdso_is_timens_clock(vc) {
            return true;
        }
        cpu_relax();
    }
    smp_rmb();

    false
}

#[inline(always)]
pub unsafe fn vdso_read_retry(vc: *const vdso_clock, start: u32) -> u32 {
    let seq: u32;

    smp_rmb();
    seq = core::ptr::read_volatile(core::ptr::addr_of!((*vc).seq));
    (seq != start) as u32
}

#[inline(always)]
pub unsafe fn vdso_write_seq_begin(vc: *mut vdso_clock) {
    /* Volatile access prevents the compiler from tearing the sequence update. */
    let seq = core::ptr::read_volatile(core::ptr::addr_of!((*vc).seq));
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*vc).seq), seq.wrapping_add(1));
}

#[inline(always)]
pub unsafe fn vdso_write_seq_end(vc: *mut vdso_clock) {
    /* Volatile access prevents the compiler from tearing the sequence update. */
    let seq = core::ptr::read_volatile(core::ptr::addr_of!((*vc).seq));
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*vc).seq), seq.wrapping_add(1));
}

#[inline(always)]
pub unsafe fn vdso_write_begin_clock(vc: *mut vdso_clock) {
    vdso_write_seq_begin(vc);
    /* Ensure the sequence invalidation is visible before data is modified */
    smp_wmb();
}

#[inline(always)]
pub unsafe fn vdso_write_end_clock(vc: *mut vdso_clock) {
    /* Ensure the data update is visible before the sequence is set valid again */
    smp_wmb();
    vdso_write_seq_end(vc);
}

#[inline(always)]
pub unsafe fn vdso_write_begin(vd: *mut vdso_time_data) {
    let vc = (*vd).clock_data;

    vdso_write_seq_begin(vc.add(CS_HRES_COARSE));
    vdso_write_seq_begin(vc.add(CS_RAW));
    /* Ensure the sequence invalidation is visible before data is modified */
    smp_wmb();
}

#[inline(always)]
pub unsafe fn vdso_write_end(vd: *mut vdso_time_data) {
    let vc = (*vd).clock_data;

    /* Ensure the data update is visible before the sequence is set valid again */
    smp_wmb();
    vdso_write_seq_end(vc.add(CS_HRES_COARSE));
    vdso_write_seq_end(vc.add(CS_RAW));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
