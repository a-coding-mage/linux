/*
 * Copyright (c) 2018, Mellanox Technologies inc.  All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// Documentation/userspace-api/ioctl/ioctl-number.rst
pub const RDMA_IOCTL_MAGIC: u32 = 0x1b;

// _IOWR(RDMA_IOCTL_MAGIC, 1, struct ib_uverbs_ioctl_hdr)
// The exact ioctl encoding is platform-dependent and supplied by the target
// platform's ioctl definitions.
pub const RDMA_VERBS_IOCTL: u32 = 0;

pub const UVERBS_ATTR_F_MANDATORY: u32 = 1u32 << 0;
/*
 * Valid output bit should be ignored and considered set in
 * mandatory fields. This bit is kernel output.
 */
pub const UVERBS_ATTR_F_VALID_OUTPUT: u32 = 1u32 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_attr__bindgen_ty_1__bindgen_ty_1 {
    pub elem_id: u8,
    pub reserved: u8,
}

#[repr(C)]
pub union ib_uverbs_attr__bindgen_ty_1 {
    pub enum_data: ib_uverbs_attr__bindgen_ty_1__bindgen_ty_1,
    pub reserved: u16,
}

#[repr(C)]
pub union ib_uverbs_attr__bindgen_ty_2 {
    /* ptr to command, inline data, idr/fd or ptr to __u32 array of IDRs */
    pub data: u64,
    /* Used by FD_IN and FD_OUT */
    pub data_s64: i64,
}

#[repr(C)]
pub struct ib_uverbs_attr {
    pub attr_id: u16,
    pub len: u16,
    pub flags: u16,
    pub attr_data: ib_uverbs_attr__bindgen_ty_1,
    pub data: ib_uverbs_attr__bindgen_ty_2,
}

#[repr(C)]
pub struct ib_uverbs_ioctl_hdr {
    pub length: u16,
    pub object_id: u16,
    pub method_id: u16,
    pub num_attrs: u16,
    pub reserved1: u64,
    pub driver_id: u32,
    pub reserved2: u32,
    pub attrs: [ib_uverbs_attr; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
