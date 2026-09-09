/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Definitions for building a device tree by calling into the
 * Open Firmware PROM.
 *
 * Copyright (C) 2010  Andres Salomon <dilinger@queued.net>
 */

/* C header guard: _LINUX_OF_PDT_H */

/* overridable operations for calling into the PROM */
#[repr(C)]
pub struct of_pdt_ops {
    /*
     * buf should be 32 bytes; return 0 on success.
     * If prev is NULL, the first property will be returned.
     */
    pub nextprop: Option<unsafe extern "C" fn(node: phandle, prev: *mut ::core::ffi::c_char, buf: *mut ::core::ffi::c_char) -> ::core::ffi::c_int>,

    /* for both functions, return proplen on success; -1 on error */
    pub getproplen: Option<unsafe extern "C" fn(node: phandle, prop: *const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub getproperty: Option<unsafe extern "C" fn(node: phandle, prop: *const ::core::ffi::c_char, buf: *mut ::core::ffi::c_char, bufsize: ::core::ffi::c_int) -> ::core::ffi::c_int>,

    /* phandles are 0 if no child or sibling exists */
    pub getchild: Option<unsafe extern "C" fn(parent: phandle) -> phandle>,
    pub getsibling: Option<unsafe extern "C" fn(node: phandle) -> phandle>,

    /* return 0 on success; fill in 'len' with number of bytes in path */
    pub pkg2path: Option<unsafe extern "C" fn(node: phandle, buf: *mut ::core::ffi::c_char, buflen: ::core::ffi::c_int, len: *mut ::core::ffi::c_int) -> ::core::ffi::c_int>,
}

unsafe extern "C" {
    pub fn prom_early_alloc(size: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;

    /* for building the device tree */
    pub fn of_pdt_build_devicetree(root_node: phandle, ops: *mut of_pdt_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
