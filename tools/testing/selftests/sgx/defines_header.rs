/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright(c) 2016-20 Intel Corporation.
 */

// C header dependencies preserved for translation context:
// <stdint.h>
// "../../../../arch/x86/include/asm/sgx.h"
// "../../../../arch/x86/include/asm/enclu.h"
// "../../../../arch/x86/include/uapi/asm/sgx.h"

pub const PAGE_SIZE: u64 = 4096;
pub const PAGE_MASK: u64 = !(PAGE_SIZE - 1);

// C attribute helper macros from this header:
// #define __aligned(x) __attribute__((__aligned__(x)))
// #define __packed __attribute__((packed))
// #define __used __attribute__((used))
// #define __section(x)__attribute__((__section__(x)))

#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum encl_op_type {
    ENCL_OP_PUT_TO_BUFFER = 0,
    ENCL_OP_GET_FROM_BUFFER = 1,
    ENCL_OP_PUT_TO_ADDRESS = 2,
    ENCL_OP_GET_FROM_ADDRESS = 3,
    ENCL_OP_NOP = 4,
    ENCL_OP_EACCEPT = 5,
    ENCL_OP_EMODPE = 6,
    ENCL_OP_INIT_TCS_PAGE = 7,
    ENCL_OP_MAX = 8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct encl_op_header {
    pub type_: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct encl_op_put_to_buf {
    pub header: encl_op_header,
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct encl_op_get_from_buf {
    pub header: encl_op_header,
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct encl_op_put_to_addr {
    pub header: encl_op_header,
    pub value: u64,
    pub addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct encl_op_get_from_addr {
    pub header: encl_op_header,
    pub value: u64,
    pub addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct encl_op_eaccept {
    pub header: encl_op_header,
    pub epc_addr: u64,
    pub flags: u64,
    pub ret: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct encl_op_emodpe {
    pub header: encl_op_header,
    pub epc_addr: u64,
    pub flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct encl_op_init_tcs_page {
    pub header: encl_op_header,
    pub tcs_page: u64,
    pub ssa: u64,
    pub entry: u64,
}
