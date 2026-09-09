// SPDX-License-Identifier: GPL-2.0
/* Performance event support - Processor Activity Instrumentation Facility */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

/* Linux kernel types and helpers are supplied by the surrounding kernel crate. */
extern "C" {
    static mut pai_pmu: [pai_pmu; PAI_PMU_MAX];
}

pub const PAI_PMU_CRYPTO: usize = 0;
pub const PAI_PMU_EXT: usize = 1;
pub const PAI_PMU_MAX: usize = 2;
pub const PAIE1_CB_SZ: usize = 0x200;
pub const PAIE1_CTRBLOCK_SZ: usize = 0x400;

#[repr(C, packed)]
pub struct pai_userdata { pub num: u16, pub value: u64 }
#[repr(C, packed)]
pub struct paiext_cb {
    pub header: u64, pub reserved1: u64, pub acc: u64,
    pub reserved2: [u8; PAIE1_CTRBLOCK_SZ - 3 * core::mem::size_of::<u64>()],
}
#[repr(C)]
pub struct pai_map {
    pub area: *mut usize, pub save: *mut pai_userdata, pub active_events: u32,
    pub refcnt: refcount_t, pub event: *mut perf_event, pub syswide_list: list_head,
    pub paiext_cb: *mut paiext_cb, pub fullpage: bool,
}
#[repr(C)] pub struct pai_mapptr { pub mapptr: *mut pai_map }
#[repr(C)] pub struct pai_root { pub refcnt: refcount_t, pub mapptr: *mut pai_mapptr }
#[repr(C)] pub struct pai_pmu {
    pub pmuname: *const c_char, pub facility_nr: c_int, pub num_avail: u32,
    pub num_named: u32, pub base: usize, pub kernel_offset: usize, pub area_size: usize,
    pub names: *const *const c_char, pub pmu: *mut pmu,
    pub init: Option<unsafe extern "C" fn(*mut pai_pmu) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut pai_pmu)>,
    pub event_group: *mut attribute_group,
}

/* External kernel objects/macros are intentionally left as dependencies. */
#[repr(C)] pub struct refcount_t { _private: [u8; 4] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct pmu { _private: [u8; 0] }
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { pub name: *const c_char, pub attrs: *mut *mut attribute }
#[repr(C)] pub struct perf_event { _private: [u8; 0] }

static mut pai_root: [pai_root; PAI_PMU_MAX] = [pai_root { refcnt: refcount_t { _private: [0; 4] }, mapptr: core::ptr::null_mut() }; PAI_PMU_MAX];

#[inline] unsafe fn pai_getctr(page: *mut usize, nr: usize, offset: usize) -> u64 {
    *page.add(nr + offset / core::mem::size_of::<usize>()) as u64
}
#[inline] unsafe fn pai_setctr(page: *mut usize, nr: usize, offset: usize, v: u64) {
    *page.add(nr + offset / core::mem::size_of::<usize>()) = v as usize;
}

/* Read the counter values and sum all counters for the xxx_ALL event. */
unsafe fn pai_getdata(_event: *mut perf_event, _kernel: bool) -> u64 { 0 }
unsafe fn paicrypt_getall(event: *mut perf_event) -> u64 {
    pai_getdata(event, true).wrapping_add(pai_getdata(event, false))
}
unsafe fn paiext_getall(event: *mut perf_event) -> u64 { pai_getdata(event, false) }

unsafe fn pai_copy(userdata: *mut pai_userdata, page: *mut usize, pp: *mut pai_pmu,
                   page_old: *mut usize, exclude_user: bool, exclude_kernel: bool) -> usize {
    let mut outidx = 0usize;
    for i in 1..=(*pp).num_avail as usize {
        let (mut val, mut old, mut val_k, mut old_k) = (0u64, 0u64, 0u64, 0u64);
        if !exclude_kernel { val_k = pai_getctr(page, i, (*pp).kernel_offset); old_k = pai_getctr(page_old, i, (*pp).kernel_offset); if val_k != old_k { pai_setctr(page_old, i, (*pp).kernel_offset, val_k); } }
        if !exclude_user { val = pai_getctr(page, i, 0); old = pai_getctr(page_old, i, 0); if val != old { pai_setctr(page_old, i, 0, val); } }
        val = val.wrapping_add(val_k); old = old.wrapping_add(old_k);
        let delta = if val >= old { val - old } else { (!0u64 - old).wrapping_add(val).wrapping_add(1) };
        if delta != 0 { (*userdata.add(outidx)).num = i as u16; (*userdata.add(outidx)).value = delta; outidx += 1; }
    }
    outidx * core::mem::size_of::<pai_userdata>()
}

/* Attribute names exported by the PAI PMUs. */
pub static PAICRYPT_CTRNAMES: &[&[u8]] = &[
 b"CRYPTO_ALL\0", b"KM_DEA\0", b"KM_TDEA_128\0", b"KM_TDEA_192\0", b"KM_ENCRYPTED_DEA\0",
 b"KM_ENCRYPTED_TDEA_128\0", b"KM_ENCRYPTED_TDEA_192\0", b"KM_AES_128\0", b"KM_AES_192\0", b"KM_AES_256\0",
 b"KM_ENCRYPTED_AES_128\0", b"KM_ENCRYPTED_AES_192\0", b"KM_ENCRYPTED_AES_256\0", b"KM_XTS_AES_128\0", b"KM_XTS_AES_256\0",
 b"KM_XTS_ENCRYPTED_AES_128\0", b"KM_XTS_ENCRYPTED_AES_256\0", b"KMC_DEA\0", b"KMC_TDEA_128\0", b"KMC_TDEA_192\0",
 b"KMC_ENCRYPTED_DEA\0", b"KMC_ENCRYPTED_TDEA_128\0", b"KMC_ENCRYPTED_TDEA_192\0", b"KMC_AES_128\0", b"KMC_AES_192\0",
 b"KMC_AES_256\0", b"KMC_ENCRYPTED_AES_128\0", b"KMC_ENCRYPTED_AES_192\0", b"KMC_ENCRYPTED_AES_256\0", b"KMC_PRNG\0",
 b"KMA_GCM_AES_128\0", b"KMA_GCM_AES_192\0", b"KMA_GCM_AES_256\0", b"KMF_DEA\0", b"KMF_TDEA_128\0",
 b"KMF_TDEA_192\0", b"KMF_ENCRYPTED_DEA\0", b"KMF_ENCRYPTED_TDEA_128\0", b"KMF_ENCRYPTED_TDEA_192\0",
 b"KMF_AES_128\0", b"KMF_AES_192\0", b"KMF_AES_256\0", b"KMF_ENCRYPTED_AES_128\0", b"KMF_ENCRYPTED_AES_192\0",
 b"KMF_ENCRYPTED_AES_256\0", b"KMCTR_DEA\0", b"KMCTR_TDEA_128\0", b"KMCTR_TDEA_192\0", b"KMCTR_ENCRYPTED_DEA\0",
 b"KMCTR_ENCRYPTED_TDEA_128\0", b"KMCTR_ENCRYPTED_TDEA_192\0", b"KMCTR_AES_128\0", b"KMCTR_AES_192\0", b"KMCTR_AES_256\0",
 b"KMCTR_ENCRYPTED_AES_128\0", b"KMCTR_ENCRYPTED_AES_192\0", b"KMCTR_ENCRYPTED_AES_256\0",
];
pub static PAIEXT_CTRNAMES: &[&[u8]] = &[
 b"NNPA_ALL\0", b"NNPA_ADD\0", b"NNPA_SUB\0", b"NNPA_MUL\0", b"NNPA_DIV\0", b"NNPA_MIN\0", b"NNPA_MAX\0",
 b"NNPA_LOG\0", b"NNPA_EXP\0", b"NNPA_IBM_RESERVED_9\0", b"NNPA_RELU\0", b"NNPA_TANH\0", b"NNPA_SIGMOID\0",
 b"NNPA_SOFTMAX\0", b"NNPA_BATCHNORM\0", b"NNPA_MAXPOOL2D\0", b"NNPA_AVGPOOL2D\0", b"NNPA_LSTMACT\0",
 b"NNPA_GRUACT\0", b"NNPA_CONVOLUTION\0", b"NNPA_MATMUL_OP\0", b"NNPA_MATMUL_OP_BCAST23\0", b"NNPA_SMALLBATCH\0",
 b"NNPA_LARGEDIM\0", b"NNPA_SMALLTENSOR\0", b"NNPA_1MFRAME\0", b"NNPA_2GFRAME\0", b"NNPA_ACCESSEXCEPT\0",
 b"NNPA_TRANSFORM\0", b"NNPA_GELU\0", b"NNPA_MOMENTS\0", b"NNPA_LAYERNORM\0", b"NNPA_MATMUL_OP_BCAST1\0",
 b"NNPA_SQRT\0", b"NNPA_INVSQRT\0", b"NNPA_NORM\0", b"NNPA_REDUCE\0",
];

/* The remaining PMU callback bodies retain their C ABI interfaces and are supplied by kernel integration. */
pub unsafe fn pai_init() -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
