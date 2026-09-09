/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * NET		Generic infrastructure for Network protocols.
 *
 * Authors:	Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

/* C header guard: _TIMEWAIT_SOCK_H */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct timewait_sock_ops {
    pub twsk_slab: *mut kmem_cache,
    pub twsk_slab_name: *mut core::ffi::c_char,
    pub twsk_obj_size: core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
