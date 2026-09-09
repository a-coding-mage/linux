// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/machvec.c
 *
 * The SuperH machine vector setup handlers, yanked from setup.c
 *
 *  Copyright (C) 1999  Niibe Yutaka
 *  Copyright (C) 2002 - 2007 Paul Mundt
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    static __machvec_start: u8;
    static __machvec_end: u8;
    static mut sh_mv: sh_machine_vector;
    static mut machvec_selected: core::ffi::c_uint;

    fn strcasecmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> core::ffi::c_int;
    fn strchr(s: *const core::ffi::c_char, c: core::ffi::c_int) -> *mut core::ffi::c_char;
    fn strlen(s: *const core::ffi::c_char) -> usize;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> core::ffi::c_int;
    fn get_system_type() -> *const core::ffi::c_char;
    fn panic(format: *const core::ffi::c_char, ...);
    fn pr_info(format: *const core::ffi::c_char, ...);
    fn pr_cont(format: *const core::ffi::c_char, ...);
    fn pr_notice(format: *const core::ffi::c_char, ...);

    static generic_irq_demux: unsafe extern "C" fn();
    static generic_mode_pins: unsafe extern "C" fn();
    static generic_mem_init: unsafe extern "C" fn();
}

#[repr(C)]
pub struct sh_machine_vector {
    pub mv_name: *const core::ffi::c_char,
    pub mv_irq_demux: Option<unsafe extern "C" fn()>,
    pub mv_mode_pins: Option<unsafe extern "C" fn()>,
    pub mv_mem_init: Option<unsafe extern "C" fn()>,
}

const MV_NAME_SIZE: usize = 32;

unsafe fn get_mv_byname(name: *const core::ffi::c_char) -> *mut sh_machine_vector {
    let mut mv = &__machvec_start as *const u8 as *mut sh_machine_vector;
    let end = &__machvec_end as *const u8 as usize;

    while !mv.is_null() && (mv as usize) < end {
        if strcasecmp(name, (*mv).mv_name) == 0 {
            return mv;
        }
        mv = mv.add(1);
    }

    core::ptr::null_mut()
}

unsafe extern "C" fn early_parse_mv(mut from: *mut core::ffi::c_char) -> core::ffi::c_int {
    let mut mv_name = [0 as core::ffi::c_char; MV_NAME_SIZE];
    let mut mv_end = strchr(from, b' ' as core::ffi::c_int);
    if mv_end.is_null() {
        mv_end = from.add(strlen(from));
    }

    let _mv_comma = strchr(from, b',' as core::ffi::c_int);
    let mut mv_len = mv_end.offset_from(from) as usize;
    if mv_len > MV_NAME_SIZE - 1 {
        mv_len = MV_NAME_SIZE - 1;
    }
    memcpy(
        mv_name.as_mut_ptr() as *mut core::ffi::c_void,
        from as *const core::ffi::c_void,
        mv_len,
    );
    mv_name[mv_len] = 0;
    from = mv_end;

    machvec_selected = 1;

    /* Boot with the generic vector */
    if strcmp(mv_name.as_ptr(), b"generic\0".as_ptr() as *const core::ffi::c_char) == 0 {
        return 0;
    }

    let mvp = get_mv_byname(mv_name.as_ptr());
    if mvp.is_null() {
        pr_info(b"Available vectors:\n\n\t'%s', \0".as_ptr() as *const core::ffi::c_char, sh_mv.mv_name);
        let mut current = &__machvec_start as *const u8 as *mut sh_machine_vector;
        while !current.is_null() && (current as usize) < (&__machvec_end as *const u8 as usize) {
            pr_cont(b"'%s', \0".as_ptr() as *const core::ffi::c_char, (*current).mv_name);
            current = current.add(1);
        }
        pr_cont(b"\n\n\0".as_ptr() as *const core::ffi::c_char);
        panic(
            b"Failed to select machvec '%s' -- halting.\n\0".as_ptr() as *const core::ffi::c_char,
            mv_name.as_ptr(),
        );
    } else {
        sh_mv = *mvp;
    }

    0
}

// early_param("sh_mv", early_parse_mv);

pub unsafe extern "C" fn sh_mv_setup() {
    /*
     * Only overload the machvec if one hasn't been selected on
     * the command line with sh_mv=
     */
    if machvec_selected == 0 {
        let machvec_size = (&__machvec_end as *const u8 as usize)
            .wrapping_sub(&__machvec_start as *const u8 as usize);

        /*
         * Sanity check for machvec section alignment. Ensure
         * __initmv hasn't been misused.
         */
        if machvec_size % core::mem::size_of::<sh_machine_vector>() != 0 {
            panic(b"machvec misaligned, invalid __initmv use?\0".as_ptr() as *const core::ffi::c_char);
        }

        /*
         * If the machvec hasn't been preselected, use the first
         * vector (usually the only one) from .machvec.init.
         */
        if machvec_size >= core::mem::size_of::<sh_machine_vector>() {
            sh_mv = *(&__machvec_start as *const u8 as *const sh_machine_vector);
        }
    }

    pr_notice(
        b"Booting machvec: %s\n\0".as_ptr() as *const core::ffi::c_char,
        get_system_type(),
    );

    /*
     * Manually walk the vec, fill in anything that the board hasn't yet
     * by hand, wrapping to the generic implementation.
     */
    if sh_mv.mv_irq_demux.is_none() {
        sh_mv.mv_irq_demux = Some(generic_irq_demux);
    }
    if sh_mv.mv_mode_pins.is_none() {
        sh_mv.mv_mode_pins = Some(generic_mode_pins);
    }
    if sh_mv.mv_mem_init.is_none() {
        sh_mv.mv_mem_init = Some(generic_mem_init);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
