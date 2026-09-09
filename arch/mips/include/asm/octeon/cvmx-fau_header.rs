/* Interface to the hardware Fetch and Add Unit. */

pub const CVMX_FAU_LOAD_IO_ADDRESS: u64 = cvmx_build_io_address(0x1e, 0);
pub const CVMX_FAU_BITS_SCRADDR: (u32, u32) = (63, 56);
pub const CVMX_FAU_BITS_LEN: (u32, u32) = (55, 48);
pub const CVMX_FAU_BITS_INEVAL: (u32, u32) = (35, 14);
pub const CVMX_FAU_BITS_TAGWAIT: (u32, u32) = (13, 13);
pub const CVMX_FAU_BITS_NOADD: (u32, u32) = (13, 13);
pub const CVMX_FAU_BITS_SIZE: (u32, u32) = (12, 11);
pub const CVMX_FAU_BITS_REGISTER: (u32, u32) = (10, 0);

#[repr(u32)]
pub enum cvmx_fau_op_size_t {
    CVMX_FAU_OP_SIZE_8 = 0,
    CVMX_FAU_OP_SIZE_16 = 1,
    CVMX_FAU_OP_SIZE_32 = 2,
    CVMX_FAU_OP_SIZE_64 = 3,
}

#[repr(C)]
pub struct cvmx_fau_tagwait64_t { pub error: u64, pub value: i64 }
#[repr(C)]
pub struct cvmx_fau_tagwait32_t { pub error: u64, pub value: i32 }
#[repr(C)]
pub struct cvmx_fau_tagwait16_t { pub error: u64, pub value: i16 }
#[repr(C)]
pub struct cvmx_fau_tagwait8_t { pub error: u64, pub value: i8 }

#[repr(C)]
pub struct cvmx_fau_async_tagwait_result_s { pub invalid: u64, pub data: u64 }
#[repr(C)]
pub union cvmx_fau_async_tagwait_result_t {
    pub u64_: u64,
    pub s: cvmx_fau_async_tagwait_result_s,
}

#[cfg(__BIG_ENDIAN_BITFIELD)]
const SWIZZLE_8: u64 = 0;
#[cfg(not(__BIG_ENDIAN_BITFIELD))]
const SWIZZLE_8: u64 = 0x7;
#[cfg(__BIG_ENDIAN_BITFIELD)]
const SWIZZLE_16: u64 = 0;
#[cfg(not(__BIG_ENDIAN_BITFIELD))]
const SWIZZLE_16: u64 = 0x6;
#[cfg(__BIG_ENDIAN_BITFIELD)]
const SWIZZLE_32: u64 = 0;
#[cfg(not(__BIG_ENDIAN_BITFIELD))]
const SWIZZLE_32: u64 = 0x4;

#[inline]
pub unsafe fn __cvmx_fau_store_address(noadd: u64, reg: u64) -> u64 {
    CVMX_ADD_IO_SEG(CVMX_FAU_LOAD_IO_ADDRESS)
        | cvmx_build_bits(CVMX_FAU_BITS_NOADD, noadd)
        | cvmx_build_bits(CVMX_FAU_BITS_REGISTER, reg)
}

#[inline]
pub unsafe fn __cvmx_fau_atomic_address(tagwait: u64, reg: u64, value: i64) -> u64 {
    CVMX_ADD_IO_SEG(CVMX_FAU_LOAD_IO_ADDRESS)
        | cvmx_build_bits(CVMX_FAU_BITS_INEVAL, value as u64)
        | cvmx_build_bits(CVMX_FAU_BITS_TAGWAIT, tagwait)
        | cvmx_build_bits(CVMX_FAU_BITS_REGISTER, reg)
}

#[inline] pub unsafe fn cvmx_fau_fetch_and_add64(reg: cvmx_fau_reg_64_t, value: i64) -> i64 { cvmx_read64_int64(__cvmx_fau_atomic_address(0, reg as u64, value)) }
#[inline] pub unsafe fn cvmx_fau_fetch_and_add32(mut reg: cvmx_fau_reg_32_t, value: i32) -> i32 { reg ^= SWIZZLE_32 as _; cvmx_read64_int32(__cvmx_fau_atomic_address(0, reg as u64, value as i64)) }
#[inline] pub unsafe fn cvmx_fau_fetch_and_add16(mut reg: cvmx_fau_reg_16_t, value: i16) -> i16 { reg ^= SWIZZLE_16 as _; cvmx_read64_int16(__cvmx_fau_atomic_address(0, reg as u64, value as i64)) }
#[inline] pub unsafe fn cvmx_fau_fetch_and_add8(mut reg: cvmx_fau_reg_8_t, value: i8) -> i8 { reg ^= SWIZZLE_8 as _; cvmx_read64_int8(__cvmx_fau_atomic_address(0, reg as u64, value as i64)) }

#[inline] pub unsafe fn cvmx_fau_tagwait_fetch_and_add64(reg: cvmx_fau_reg_64_t, value: i64) -> cvmx_fau_tagwait64_t { let i = cvmx_read64_int64(__cvmx_fau_atomic_address(1, reg as u64, value)); core::mem::transmute(i) }
#[inline] pub unsafe fn cvmx_fau_tagwait_fetch_and_add32(mut reg: cvmx_fau_reg_32_t, value: i32) -> cvmx_fau_tagwait32_t { reg ^= SWIZZLE_32 as _; let i = cvmx_read64_int32(__cvmx_fau_atomic_address(1, reg as u64, value as i64)); core::mem::transmute(i as u64) }
#[inline] pub unsafe fn cvmx_fau_tagwait_fetch_and_add16(mut reg: cvmx_fau_reg_16_t, value: i16) -> cvmx_fau_tagwait16_t { reg ^= SWIZZLE_16 as _; let i = cvmx_read64_int16(__cvmx_fau_atomic_address(1, reg as u64, value as i64)); core::mem::transmute(i as u64) }
#[inline] pub unsafe fn cvmx_fau_tagwait_fetch_and_add8(mut reg: cvmx_fau_reg_8_t, value: i8) -> cvmx_fau_tagwait8_t { reg ^= SWIZZLE_8 as _; let i = cvmx_read64_int8(__cvmx_fau_atomic_address(1, reg as u64, value as i64)); core::mem::transmute(i as u64) }

#[inline]
pub unsafe fn __cvmx_fau_iobdma_data(scraddr: u64, value: i64, tagwait: u64, size: cvmx_fau_op_size_t, reg: u64) -> u64 {
    CVMX_FAU_LOAD_IO_ADDRESS
        | cvmx_build_bits(CVMX_FAU_BITS_SCRADDR, scraddr >> 3)
        | cvmx_build_bits(CVMX_FAU_BITS_LEN, 1)
        | cvmx_build_bits(CVMX_FAU_BITS_INEVAL, value as u64)
        | cvmx_build_bits(CVMX_FAU_BITS_TAGWAIT, tagwait)
        | cvmx_build_bits(CVMX_FAU_BITS_SIZE, size as u64)
        | cvmx_build_bits(CVMX_FAU_BITS_REGISTER, reg)
}

#[inline] pub unsafe fn cvmx_fau_async_fetch_and_add64(s: u64, r: cvmx_fau_reg_64_t, v: i64) { cvmx_send_single(__cvmx_fau_iobdma_data(s,v,0,cvmx_fau_op_size_t::CVMX_FAU_OP_SIZE_64,r as u64)); }
#[inline] pub unsafe fn cvmx_fau_async_fetch_and_add32(s: u64, r: cvmx_fau_reg_32_t, v: i32) { cvmx_send_single(__cvmx_fau_iobdma_data(s,v as i64,0,cvmx_fau_op_size_t::CVMX_FAU_OP_SIZE_32,r as u64)); }
#[inline] pub unsafe fn cvmx_fau_async_fetch_and_add16(s: u64, r: cvmx_fau_reg_16_t, v: i16) { cvmx_send_single(__cvmx_fau_iobdma_data(s,v as i64,0,cvmx_fau_op_size_t::CVMX_FAU_OP_SIZE_16,r as u64)); }
#[inline] pub unsafe fn cvmx_fau_async_fetch_and_add8(s: u64, r: cvmx_fau_reg_8_t, v: i8) { cvmx_send_single(__cvmx_fau_iobdma_data(s,v as i64,0,cvmx_fau_op_size_t::CVMX_FAU_OP_SIZE_8,r as u64)); }
#[inline] pub unsafe fn cvmx_fau_async_tagwait_fetch_and_add64(s:u64,r:cvmx_fau_reg_64_t,v:i64){cvmx_send_single(__cvmx_fau_iobdma_data(s,v,1,cvmx_fau_op_size_t::CVMX_FAU_OP_SIZE_64,r as u64));}
#[inline] pub unsafe fn cvmx_fau_async_tagwait_fetch_and_add32(s:u64,r:cvmx_fau_reg_32_t,v:i32){cvmx_send_single(__cvmx_fau_iobdma_data(s,v as i64,1,cvmx_fau_op_size_t::CVMX_FAU_OP_SIZE_32,r as u64));}
#[inline] pub unsafe fn cvmx_fau_async_tagwait_fetch_and_add16(s:u64,r:cvmx_fau_reg_16_t,v:i16){cvmx_send_single(__cvmx_fau_iobdma_data(s,v as i64,1,cvmx_fau_op_size_t::CVMX_FAU_OP_SIZE_16,r as u64));}
#[inline] pub unsafe fn cvmx_fau_async_tagwait_fetch_and_add8(s:u64,r:cvmx_fau_reg_8_t,v:i8){cvmx_send_single(__cvmx_fau_iobdma_data(s,v as i64,1,cvmx_fau_op_size_t::CVMX_FAU_OP_SIZE_8,r as u64));}

#[inline] pub unsafe fn cvmx_fau_atomic_add64(r:cvmx_fau_reg_64_t,v:i64){cvmx_write64_int64(__cvmx_fau_store_address(0,r as u64),v)}
#[inline] pub unsafe fn cvmx_fau_atomic_add32(mut r:cvmx_fau_reg_32_t,v:i32){r^=SWIZZLE_32 as _;cvmx_write64_int32(__cvmx_fau_store_address(0,r as u64),v)}
#[inline] pub unsafe fn cvmx_fau_atomic_add16(mut r:cvmx_fau_reg_16_t,v:i16){r^=SWIZZLE_16 as _;cvmx_write64_int16(__cvmx_fau_store_address(0,r as u64),v)}
#[inline] pub unsafe fn cvmx_fau_atomic_add8(mut r:cvmx_fau_reg_8_t,v:i8){r^=SWIZZLE_8 as _;cvmx_write64_int8(__cvmx_fau_store_address(0,r as u64),v)}
#[inline] pub unsafe fn cvmx_fau_atomic_write64(r:cvmx_fau_reg_64_t,v:i64){cvmx_write64_int64(__cvmx_fau_store_address(1,r as u64),v)}
#[inline] pub unsafe fn cvmx_fau_atomic_write32(mut r:cvmx_fau_reg_32_t,v:i32){r^=SWIZZLE_32 as _;cvmx_write64_int32(__cvmx_fau_store_address(1,r as u64),v)}
#[inline] pub unsafe fn cvmx_fau_atomic_write16(mut r:cvmx_fau_reg_16_t,v:i16){r^=SWIZZLE_16 as _;cvmx_write64_int16(__cvmx_fau_store_address(1,r as u64),v)}
#[inline] pub unsafe fn cvmx_fau_atomic_write8(mut r:cvmx_fau_reg_8_t,v:i8){r^=SWIZZLE_8 as _;cvmx_write64_int8(__cvmx_fau_store_address(1,r as u64),v)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
