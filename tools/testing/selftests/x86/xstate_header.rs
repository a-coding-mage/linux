// SPDX-License-Identifier: GPL-2.0-only

use core::arch::asm;
use core::ffi::{c_char, c_int, c_void};

// Original C header included <stdint.h> and "kselftest.h".

pub const XSAVE_HDR_OFFSET: usize = 512;
pub const XSAVE_HDR_SIZE: usize = 64;

/*
 * List of XSAVE features Linux knows about. Copied from
 * arch/x86/include/asm/fpu/types.h
 */
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum xfeature {
    XFEATURE_FP,
    XFEATURE_SSE,
    XFEATURE_YMM,
    XFEATURE_BNDREGS,
    XFEATURE_BNDCSR,
    XFEATURE_OPMASK,
    XFEATURE_ZMM_Hi256,
    XFEATURE_Hi16_ZMM,
    XFEATURE_PT_UNIMPLEMENTED_SO_FAR,
    XFEATURE_PKRU,
    XFEATURE_PASID,
    XFEATURE_CET_USER,
    XFEATURE_CET_KERNEL_UNUSED,
    XFEATURE_RSRVD_COMP_13,
    XFEATURE_RSRVD_COMP_14,
    XFEATURE_LBR,
    XFEATURE_RSRVD_COMP_16,
    XFEATURE_XTILECFG,
    XFEATURE_XTILEDATA,
    XFEATURE_APX,

    XFEATURE_MAX,
}

pub const XFEATURE_MAX: u32 = xfeature::XFEATURE_MAX as u32;

/* Copied from arch/x86/kernel/fpu/xstate.c */
pub static xfeature_names: [*const c_char; 21] = [
    b"x87 floating point registers\0".as_ptr() as *const c_char,
    b"SSE registers\0".as_ptr() as *const c_char,
    b"AVX registers\0".as_ptr() as *const c_char,
    b"MPX bounds registers\0".as_ptr() as *const c_char,
    b"MPX CSR\0".as_ptr() as *const c_char,
    b"AVX-512 opmask\0".as_ptr() as *const c_char,
    b"AVX-512 Hi256\0".as_ptr() as *const c_char,
    b"AVX-512 ZMM_Hi256\0".as_ptr() as *const c_char,
    b"Processor Trace (unused)\0".as_ptr() as *const c_char,
    b"Protection Keys User registers\0".as_ptr() as *const c_char,
    b"PASID state\0".as_ptr() as *const c_char,
    b"Control-flow User registers\0".as_ptr() as *const c_char,
    b"Control-flow Kernel registers (unused)\0".as_ptr() as *const c_char,
    b"unknown xstate feature\0".as_ptr() as *const c_char,
    b"unknown xstate feature\0".as_ptr() as *const c_char,
    b"unknown xstate feature\0".as_ptr() as *const c_char,
    b"unknown xstate feature\0".as_ptr() as *const c_char,
    b"AMX Tile config\0".as_ptr() as *const c_char,
    b"AMX Tile data\0".as_ptr() as *const c_char,
    b"APX registers\0".as_ptr() as *const c_char,
    b"unknown xstate feature\0".as_ptr() as *const c_char,
];

#[repr(C)]
pub struct xsave_buffer_fields {
    pub legacy: [c_char; XSAVE_HDR_OFFSET],
    pub header: [c_char; XSAVE_HDR_SIZE],
    pub extended: [c_char; 0],
}

#[repr(C)]
pub union xsave_buffer_union {
    pub fields: core::mem::ManuallyDrop<xsave_buffer_fields>,
    pub bytes: [c_char; 0],
}

#[repr(C)]
pub struct xsave_buffer {
    pub u: xsave_buffer_union,
}

#[inline]
pub unsafe fn xsave(xbuf: *mut xsave_buffer, rfbm: u64) {
    let rfbm_hi: u32 = (rfbm >> 32) as u32;
    let rfbm_lo: u32 = rfbm as u32;

    unsafe {
        asm!(
            "xsave (rdi)",
            in("rdi") xbuf,
            in("eax") rfbm_lo,
            in("edx") rfbm_hi,
            options(nostack, preserves_flags)
        );
    }
}

#[inline]
pub unsafe fn xrstor(xbuf: *mut xsave_buffer, rfbm: u64) {
    let rfbm_hi: u32 = (rfbm >> 32) as u32;
    let rfbm_lo: u32 = rfbm as u32;

    unsafe {
        asm!(
            "xrstor (rdi)",
            in("rdi") xbuf,
            in("eax") rfbm_lo,
            in("edx") rfbm_hi,
            options(nostack, preserves_flags)
        );
    }
}

pub const CPUID_LEAF_XSTATE: u32 = 0xd;
pub const CPUID_SUBLEAF_XSTATE_USER: u32 = 0x0;

unsafe extern "C" {
    pub fn __cpuid_count(
        leaf: u32,
        subleaf: u32,
        eax: *mut u32,
        ebx: *mut u32,
        ecx: *mut u32,
        edx: *mut u32,
    );
    pub fn ksft_print_msg(fmt: *const c_char, ...);
    pub fn aligned_alloc(alignment: usize, size: usize) -> *mut c_void;
    pub fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    pub fn rand() -> c_int;
    pub fn test_xstate(feature_num: u32);
}

#[inline]
pub unsafe fn get_xbuf_size() -> u32 {
    let mut eax: u32 = 0;
    let mut ebx: u32 = 0;
    let mut ecx: u32 = 0;
    let mut edx: u32 = 0;

    unsafe {
        __cpuid_count(
            CPUID_LEAF_XSTATE,
            CPUID_SUBLEAF_XSTATE_USER,
            &mut eax,
            &mut ebx,
            &mut ecx,
            &mut edx,
        );
    }

    /*
     * EBX enumerates the size (in bytes) required by the XSAVE
     * instruction for an XSAVE area containing all the user state
     * components corresponding to bits currently set in XCR0.
     */
    ebx
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct xstate_info {
    pub name: *const c_char,
    pub num: u32,
    pub mask: u32,
    pub xbuf_offset: u32,
    pub size: u32,
}

#[inline]
pub unsafe fn get_xstate_info(xfeature_num: u32) -> xstate_info {
    let mut xstate: xstate_info = xstate_info {
        name: core::ptr::null(),
        num: 0,
        mask: 0,
        xbuf_offset: 0,
        size: 0,
    };
    let mut eax: u32 = 0;
    let mut ebx: u32 = 0;
    let mut ecx: u32 = 0;
    let mut edx: u32 = 0;

    if xfeature_num >= XFEATURE_MAX {
        unsafe {
            ksft_print_msg(b"unknown state\n\0".as_ptr() as *const c_char);
        }
        return xstate;
    }

    xstate.name = xfeature_names[xfeature_num as usize];
    xstate.num = xfeature_num;
    xstate.mask = 1u32 << xfeature_num;

    unsafe {
        __cpuid_count(
            CPUID_LEAF_XSTATE,
            xfeature_num,
            &mut eax,
            &mut ebx,
            &mut ecx,
            &mut edx,
        );
    }
    xstate.size = eax;
    xstate.xbuf_offset = ebx;
    xstate
}

#[inline]
pub unsafe fn alloc_xbuf() -> *mut xsave_buffer {
    let xbuf_size: u32 = unsafe { get_xbuf_size() };

    /* XSAVE buffer should be 64B-aligned. */
    unsafe { aligned_alloc(64, xbuf_size as usize) as *mut xsave_buffer }
}

#[inline]
pub unsafe fn clear_xstate_header(xbuf: *mut xsave_buffer) {
    unsafe {
        memset(
            (*xbuf).u.fields.header.as_mut_ptr() as *mut c_void,
            0,
            core::mem::size_of_val(&(*xbuf).u.fields.header),
        );
    }
}

#[inline]
pub unsafe fn set_xstatebv(xbuf: *mut xsave_buffer, bv: u64) {
    /* XSTATE_BV is at the beginning of the header: */
    unsafe {
        *((*xbuf).u.fields.header.as_mut_ptr() as *mut u64) = bv;
    }
}

/* See 'struct _fpx_sw_bytes' at sigcontext.h */
pub const SW_BYTES_OFFSET: usize = 464;
/* N.B. The struct's field name varies so read from the offset. */
pub const SW_BYTES_BV_OFFSET: usize = SW_BYTES_OFFSET + 8;

#[repr(C)]
pub struct _fpx_sw_bytes {
    _unused: [u8; 0],
}

#[inline]
pub unsafe fn get_fpx_sw_bytes(xbuf: *mut c_void) -> *mut _fpx_sw_bytes {
    unsafe { (xbuf as *mut u8).add(SW_BYTES_OFFSET) as *mut _fpx_sw_bytes }
}

#[inline]
pub unsafe fn get_fpx_sw_bytes_features(buffer: *mut c_void) -> u64 {
    unsafe { *((buffer as *mut u8).add(SW_BYTES_BV_OFFSET) as *mut u64) }
}

#[inline]
pub unsafe fn set_rand_data(xstate: *mut xstate_info, xbuf: *mut xsave_buffer) {
    let mut ptr: *mut c_int = unsafe {
        ((*xbuf).u.bytes.as_mut_ptr()).add((*xstate).xbuf_offset as usize) as *mut c_int
    };
    let data: c_int;
    let mut i: c_int;

    /*
     * Ensure that 'data' is never 0.  This ensures that
     * the registers are never in their initial configuration
     * and thus never tracked as being in the init state.
     */
    data = unsafe { rand() } | 1;

    i = 0;
    while i < unsafe { (*xstate).size } as c_int / core::mem::size_of::<c_int>() as c_int {
        unsafe {
            *ptr = data;
            ptr = ptr.add(1);
        }
        i += 1;
    }
}

/* Testing kernel's context switching and ABI support for the xstate. */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
