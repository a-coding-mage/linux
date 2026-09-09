/*
 * Copyright (c) 2015-2016, Linaro Limited
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 * this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 * this list of conditions and the following disclaimer in the documentation
 * and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

// Dependencies supplied by the surrounding kernel bindings provide __u* types
// and the _IOR/_IOWR ioctl encoding macros.

pub const TEE_IOC_MAGIC: u32 = 0xa4;
pub const TEE_IOC_BASE: u32 = 0;
pub const TEE_MAX_ARG_SIZE: u32 = 4096;

pub const TEE_GEN_CAP_GP: u32 = 1 << 0;
pub const TEE_GEN_CAP_PRIVILEGED: u32 = 1 << 1;
pub const TEE_GEN_CAP_REG_MEM: u32 = 1 << 2;
pub const TEE_GEN_CAP_MEMREF_NULL: u32 = 1 << 3;
pub const TEE_GEN_CAP_OBJREF: u32 = 1 << 4;

pub const TEE_MEMREF_NULL: u64 = (-1i64) as u64;
pub const TEE_OBJREF_NULL: u64 = (-1i64) as u64;

pub const TEE_IMPL_ID_OPTEE: u32 = 1;
pub const TEE_IMPL_ID_AMDTEE: u32 = 2;
pub const TEE_IMPL_ID_TSTEE: u32 = 3;
pub const TEE_IMPL_ID_QTEE: u32 = 4;
pub const TEE_OPTEE_CAP_TZ: u32 = 1 << 0;

#[repr(C)]
pub struct tee_ioctl_version_data { pub impl_id: u32, pub impl_caps: u32, pub gen_caps: u32 }
pub const TEE_IOC_VERSION: _ = _IOR!(TEE_IOC_MAGIC, TEE_IOC_BASE + 0, tee_ioctl_version_data);

#[repr(C)]
pub struct tee_ioctl_shm_alloc_data { pub size: u64, pub flags: u32, pub id: i32 }
pub const TEE_IOC_SHM_ALLOC: _ = _IOWR!(TEE_IOC_MAGIC, TEE_IOC_BASE + 1, tee_ioctl_shm_alloc_data);

#[repr(C)]
pub struct tee_ioctl_buf_data { pub buf_ptr: u64, pub buf_len: u64 }

pub const TEE_IOCTL_PARAM_ATTR_TYPE_NONE: u64 = 0;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT: u64 = 1;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_OUTPUT: u64 = 2;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INOUT: u64 = 3;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INPUT: u64 = 5;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_OUTPUT: u64 = 6;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INOUT: u64 = 7;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_UBUF_INPUT: u64 = 8;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_UBUF_OUTPUT: u64 = 9;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_UBUF_INOUT: u64 = 10;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_OBJREF_INPUT: u64 = 11;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_OBJREF_OUTPUT: u64 = 12;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_OBJREF_INOUT: u64 = 13;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_MASK: u64 = 0xff;
pub const TEE_IOCTL_PARAM_ATTR_META: u64 = 0x100;
pub const TEE_IOCTL_PARAM_ATTR_MASK: u64 = TEE_IOCTL_PARAM_ATTR_TYPE_MASK | TEE_IOCTL_PARAM_ATTR_META;

pub const TEE_IOCTL_LOGIN_PUBLIC: u32 = 0;
pub const TEE_IOCTL_LOGIN_USER: u32 = 1;
pub const TEE_IOCTL_LOGIN_GROUP: u32 = 2;
pub const TEE_IOCTL_LOGIN_APPLICATION: u32 = 4;
pub const TEE_IOCTL_LOGIN_USER_APPLICATION: u32 = 5;
pub const TEE_IOCTL_LOGIN_GROUP_APPLICATION: u32 = 6;
pub const TEE_IOCTL_LOGIN_REE_KERNEL_MIN: u32 = 0x80000000;
pub const TEE_IOCTL_LOGIN_REE_KERNEL_MAX: u32 = 0xBFFFFFFF;
pub const TEE_IOCTL_LOGIN_REE_KERNEL: u32 = 0x80000000;

#[repr(C)]
pub struct tee_ioctl_param { pub attr: u64, pub a: u64, pub b: u64, pub c: u64 }

pub const TEE_IOCTL_UUID_LEN: usize = 16;

#[repr(C)]
pub struct tee_ioctl_open_session_arg {
    pub uuid: [u8; TEE_IOCTL_UUID_LEN], pub clnt_uuid: [u8; TEE_IOCTL_UUID_LEN],
    pub clnt_login: u32, pub cancel_id: u32, pub session: u32, pub ret: u32,
    pub ret_origin: u32, pub num_params: u32,
    // C flexible array member; storage follows this header.
    pub params: [tee_ioctl_param; 0],
}
pub const TEE_IOC_OPEN_SESSION: _ = _IOR!(TEE_IOC_MAGIC, TEE_IOC_BASE + 2, tee_ioctl_buf_data);

#[repr(C)]
pub struct tee_ioctl_invoke_arg {
    pub func: u32, pub session: u32, pub cancel_id: u32, pub ret: u32,
    pub ret_origin: u32, pub num_params: u32, pub params: [tee_ioctl_param; 0],
}
pub const TEE_IOC_INVOKE: _ = _IOR!(TEE_IOC_MAGIC, TEE_IOC_BASE + 3, tee_ioctl_buf_data);

#[repr(C)]
pub struct tee_ioctl_cancel_arg { pub cancel_id: u32, pub session: u32 }
pub const TEE_IOC_CANCEL: _ = _IOR!(TEE_IOC_MAGIC, TEE_IOC_BASE + 4, tee_ioctl_cancel_arg);

#[repr(C)]
pub struct tee_ioctl_close_session_arg { pub session: u32 }
pub const TEE_IOC_CLOSE_SESSION: _ = _IOR!(TEE_IOC_MAGIC, TEE_IOC_BASE + 5, tee_ioctl_close_session_arg);

#[repr(C)]
pub struct tee_iocl_supp_recv_arg { pub func: u32, pub num_params: u32, pub params: [tee_ioctl_param; 0] }
pub const TEE_IOC_SUPPL_RECV: _ = _IOR!(TEE_IOC_MAGIC, TEE_IOC_BASE + 6, tee_ioctl_buf_data);

#[repr(C)]
pub struct tee_iocl_supp_send_arg { pub ret: u32, pub num_params: u32, pub params: [tee_ioctl_param; 0] }
pub const TEE_IOC_SUPPL_SEND: _ = _IOR!(TEE_IOC_MAGIC, TEE_IOC_BASE + 7, tee_ioctl_buf_data);

#[repr(C)]
pub struct tee_ioctl_shm_register_data { pub addr: u64, pub length: u64, pub flags: u32, pub id: i32 }

#[repr(C)]
pub struct tee_ioctl_shm_register_fd_data { pub fd: i64, pub size: u64, pub flags: u32, pub id: i32 }
pub const TEE_IOC_SHM_REGISTER_FD: _ = _IOWR!(TEE_IOC_MAGIC, TEE_IOC_BASE + 8, tee_ioctl_shm_register_fd_data);
pub const TEE_IOC_SHM_REGISTER: _ = _IOWR!(TEE_IOC_MAGIC, TEE_IOC_BASE + 9, tee_ioctl_shm_register_data);

#[repr(C)]
pub struct tee_ioctl_object_invoke_arg {
    pub id: u64, pub op: u32, pub ret: u32, pub num_params: u32,
    pub _bindgen_anon_1: u32, pub params: [tee_ioctl_param; 0],
}
pub const TEE_IOC_OBJECT_INVOKE: _ = _IOR!(TEE_IOC_MAGIC, TEE_IOC_BASE + 10, tee_ioctl_buf_data);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
