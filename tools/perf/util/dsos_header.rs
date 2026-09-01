/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from perf/util/dsos.h.
 *
 * C include dependencies:
 * - <stdbool.h>
 * - <stdio.h>
 * - <linux/list.h>
 * - <linux/rbtree.h>
 * - "rwsem.h"
 */

use core::ffi::{c_char, c_int, c_uint};

pub enum dso {}
pub enum dso_id {}
pub enum kmod_path {}
pub enum machine {}
pub enum FILE {}

/*
 * Collection of DSOs as an array for iteration speed, but sorted for O(n)
 * lookup.
 */
#[repr(C)]
pub struct dsos {
    pub lock: rw_semaphore,
    pub dsos: *mut *mut dso,
    pub cnt: c_uint,
    pub allocated: c_uint,
    pub sorted: bool,
}

unsafe extern "C" {
    pub fn dsos__init(dsos: *mut dsos);
    pub fn dsos__exit(dsos: *mut dsos);

    pub fn __dsos__add(dsos: *mut dsos, dso: *mut dso) -> c_int;
    pub fn dsos__add(dsos: *mut dsos, dso: *mut dso) -> c_int;
    pub fn dsos__find(dsos: *mut dsos, name: *const c_char, cmp_short: bool) -> *mut dso;

    pub fn dsos__findnew_id(dsos: *mut dsos, name: *const c_char, id: *const dso_id) -> *mut dso;

    pub fn dsos__read_build_ids(dsos: *mut dsos, with_hits: bool) -> bool;

    pub fn dsos__fprintf_buildid(
        dsos: *mut dsos,
        fp: *mut FILE,
        skip: Option<unsafe extern "C" fn(dso: *mut dso, parm: c_int) -> bool>,
        parm: c_int,
    ) -> usize;
    pub fn dsos__fprintf(dsos: *mut dsos, fp: *mut FILE) -> usize;

    pub fn dsos__hit_all(dsos: *mut dsos) -> c_int;

    pub fn dsos__findnew_module_dso(
        dsos: *mut dsos,
        machine: *mut machine,
        m: *mut kmod_path,
        filename: *const c_char,
    ) -> *mut dso;

    pub fn dsos__find_kernel_dso(dsos: *mut dsos) -> *mut dso;

    pub fn dsos__for_each_dso(
        dsos: *mut dsos,
        cb: Option<unsafe extern "C" fn(dso: *mut dso, data: *mut core::ffi::c_void) -> c_int>,
        data: *mut core::ffi::c_void,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
