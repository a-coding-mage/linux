// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;

extern "C" {
    fn ceph_start_decoding(
        p: *mut *mut c_void,
        end: *mut c_void,
        version: u32,
        name: *const u8,
        struct_v: *mut u8,
        struct_len: *mut u32,
    ) -> i32;
    fn ceph_pr_addr(addr: *const ceph_entity_addr) -> *const u8;
    fn dout(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn memchr_inv(s: *const c_void, c: i32, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn ceph_encode_8(p: *mut *mut c_void, v: u8);
    fn ceph_start_encoding(p: *mut *mut c_void, version: u32, compat: u32, len: u32);
    fn ceph_encode_copy(p: *mut *mut c_void, src: *const c_void, len: usize);
    fn ceph_encode_32(p: *mut *mut c_void, v: u32);
    fn ceph_encode_16(p: *mut *mut c_void, v: u16);
}

#[repr(C)]
pub struct ceph_entity_addr {
    pub type_: u32,
    pub nonce: u32,
    pub in_addr: sockaddr_storage,
}

#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: u16,
    pub __data: [u8; 126],
}

const EINVAL: i32 = 22;
const ENOENT: i32 = 2;
const AF_INET: i32 = 2;
const AF_INET6: i32 = 10;
const CEPH_ENTITY_ADDR_TYPE_LEGACY: u32 = 1;
const CEPH_ENTITY_ADDR_TYPE_MSGR2: u32 = 2;
const CEPH_ENCODING_START_BLK_LEN: i32 = 1;

unsafe fn ceph_decode_entity_addr_versioned(
    p: *mut *mut c_void,
    end: *mut c_void,
    addr: *mut ceph_entity_addr,
) -> i32 {
    let mut struct_v: u8 = 0;
    let mut struct_len: u32 = 0;
    let ret = ceph_start_decoding(
        p, end, 1, b"entity_addr_t\0".as_ptr(), &mut struct_v, &mut struct_len,
    );
    if ret != 0 {
        return ret;
    }

    let struct_end = (*p as *mut u8).add(struct_len as usize) as *mut c_void;
    if (*p as usize).saturating_add(4) > end as usize {
        return -EINVAL;
    }
    (*addr).type_ = u32::from_le_bytes((*(*p as *const [u8; 4])).to_owned());
    *p = (*p as *mut u8).add(4) as *mut c_void;
    if (*p as usize).saturating_add(4) > end as usize {
        return -EINVAL;
    }
    (*addr).nonce = u32::from_le_bytes((*(*p as *const [u8; 4])).to_owned());
    *p = (*p as *mut u8).add(4) as *mut c_void;
    if (*p as usize).saturating_add(4) > end as usize {
        return -EINVAL;
    }
    let addr_len = u32::from_le_bytes((*(*p as *const [u8; 4])).to_owned()) as usize;
    *p = (*p as *mut u8).add(4) as *mut c_void;
    if addr_len > core::mem::size_of::<sockaddr_storage>() {
        return -EINVAL;
    }
    memset(
        &mut (*addr).in_addr as *mut sockaddr_storage as *mut c_void,
        0,
        core::mem::size_of::<sockaddr_storage>(),
    );
    if addr_len != 0 {
        if (*p as usize).saturating_add(addr_len) > end as usize {
            return -EINVAL;
        }
        memcpy(
            &mut (*addr).in_addr as *mut sockaddr_storage as *mut c_void,
            *p,
            addr_len,
        );
        (*p as *mut u8).write_bytes(0, 0);
        (*addr).in_addr.ss_family = (*addr).in_addr.ss_family.to_le();
        *p = (*p as *mut u8).add(addr_len) as *mut c_void;
    }
    *p = struct_end;
    0
}

unsafe fn ceph_decode_entity_addr_legacy(
    p: *mut *mut c_void,
    end: *mut c_void,
    addr: *mut ceph_entity_addr,
) -> i32 {
    if (*p as usize).saturating_add(3) > end as usize {
        return -EINVAL;
    }
    *p = (*p as *mut u8).add(3) as *mut c_void;
    (*addr).type_ = CEPH_ENTITY_ADDR_TYPE_LEGACY;
    if (*p as usize).saturating_add(4) > end as usize {
        return -EINVAL;
    }
    (*addr).nonce = u32::from_ne_bytes((*(*p as *const [u8; 4])).to_owned());
    *p = (*p as *mut u8).add(4) as *mut c_void;
    memset(&mut (*addr).in_addr as *mut _ as *mut c_void, 0, core::mem::size_of::<sockaddr_storage>());
    if (*p as usize).saturating_add(core::mem::size_of::<sockaddr_storage>()) > end as usize {
        return -EINVAL;
    }
    memcpy(&mut (*addr).in_addr as *mut _ as *mut c_void, *p, core::mem::size_of::<sockaddr_storage>());
    *p = (*p as *mut u8).add(core::mem::size_of::<sockaddr_storage>()) as *mut c_void;
    (*addr).in_addr.ss_family = (*addr).in_addr.ss_family.to_be();
    0
}

#[no_mangle]
pub unsafe extern "C" fn ceph_decode_entity_addr(
    p: *mut *mut c_void,
    end: *mut c_void,
    addr: *mut ceph_entity_addr,
) -> i32 {
    if (*p as usize) >= end as usize {
        return -EINVAL;
    }
    let marker = (*(p as *mut *mut u8)).read();
    *p = (*p as *mut u8).add(1) as *mut c_void;
    if marker == 1 {
        ceph_decode_entity_addr_versioned(p, end, addr)
    } else if marker == 0 {
        ceph_decode_entity_addr_legacy(p, end, addr)
    } else {
        -EINVAL
    }
}

unsafe fn get_sockaddr_encoding_len(family: i32) -> i32 {
    match family {
        AF_INET => 16,
        AF_INET6 => 28,
        _ => 128,
    }
}

#[no_mangle]
pub unsafe extern "C" fn ceph_entity_addr_encoding_len(addr: *const ceph_entity_addr) -> i32 {
    let family = (*addr).in_addr.ss_family;
    let addr_len = get_sockaddr_encoding_len(family as i32);
    1 + CEPH_ENCODING_START_BLK_LEN + 4 + 4 + 4 + addr_len
}

#[no_mangle]
pub unsafe extern "C" fn ceph_encode_entity_addr(
    p: *mut *mut c_void,
    addr: *const ceph_entity_addr,
) {
    let family = (*addr).in_addr.ss_family;
    let addr_len = get_sockaddr_encoding_len(family as i32);
    ceph_encode_8(p, 1);
    ceph_start_encoding(p, 1, 1, 4 + 4 + 4 + addr_len as u32);
    ceph_encode_copy(p, &(*addr).type_ as *const _ as *const c_void, 4);
    ceph_encode_copy(p, &(*addr).nonce as *const _ as *const c_void, 4);
    ceph_encode_32(p, addr_len as u32);
    ceph_encode_16(p, family);
    ceph_encode_copy(p, (*addr).in_addr.__data.as_ptr() as *const c_void, (addr_len - 2) as usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
