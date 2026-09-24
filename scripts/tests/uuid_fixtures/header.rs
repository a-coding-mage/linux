// SPDX-License-Identifier: GPL-2.0-only
//! Compile the complete translated header against genuine kernel dependencies.
#[macro_use]
#[path = "../include/linux/uuid_header.rs"]
pub mod declarations;
use declarations::{guid_t, uuid_t};

/// Verify both initializer macros against the real public types.
pub const VALUES: (guid_t, uuid_t) = (
    GUID_INIT!(0x01234567u32, 0x89abu16, 0xcdefu16, 0, 1, 2, 3, 4, 5, 6, 7),
    UUID_INIT!(0x01234567u32, 0x89abu16, 0xcdefu16, 0, 1, 2, 3, 4, 5, 6, 7),
);

/// Exercises every inline header helper with disjoint local objects.
#[no_mangle]
pub extern "C" fn uuid_header_check() -> bool {
    use declarations::*;
    let (g, u) = VALUES;
    let mut gc = guid_t { b: [0; 16] };
    let mut uc = uuid_t { b: [0; 16] };
    let mut bytes = [0; 16];
    // SAFETY: Each pointer addresses a local fully initialized object or the
    // genuine immutable static. All copy regions are disjoint and 16 bytes.
    unsafe {
        if !guid_is_null(&gc) || !uuid_is_null(&uc) { return false; }
        guid_copy(&mut gc, &g);
        uuid_copy(&mut uc, &u);
        if !guid_equal(&gc, &g) || !uuid_equal(&uc, &u) { return false; }
        if guid_is_null(&gc) || uuid_is_null(&uc) { return false; }
        export_guid(bytes.as_mut_ptr(), &gc);
        import_uuid(&mut uc, bytes.as_ptr());
        export_uuid(bytes.as_mut_ptr(), &uc);
        import_guid(&mut gc, bytes.as_ptr());
        if !guid_equal(&gc, &g) { return false; }
        if uuid_equal(&uc, &u) { return false; }
    }
    g.b[..8] == [0x67, 0x45, 0x23, 0x01, 0xab, 0x89, 0xef, 0xcd]
        && u.b[..8] == [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]
}

/// Constants exercise promoted narrow, signed and wide operands.
pub const INITIALIZERS: [[u8; 16]; 6] = [
 GUID_INIT!(0x12u8, 0x34u8, 0x56u8, 0,1,2,3,4,5,6,7).b,
 UUID_INIT!(0x12u8, 0x34u8, 0x56u8, 0,1,2,3,4,5,6,7).b,
 GUID_INIT!(-18i8, -52i8, -86i8, -1i8,1,2,3,4,5,6,7).b,
 UUID_INIT!(-18i8, -52i8, -86i8, -1i8,1,2,3,4,5,6,7).b,
 GUID_INIT!(0x123456789abcdef0u64, 0x12345678u64, 0x9abcdef0u64, 0,1,2,3,4,5,6,7).b,
 UUID_INIT!(0x123456789abcdef0u64, 0x12345678u64, 0x9abcdef0u64, 0,1,2,3,4,5,6,7).b,
];
/// Signed wide constants retain the same low bytes as arithmetic C shifts.
pub const SIGNED_WIDE: [[u8; 16]; 2] = [
 GUID_INIT!(-0x123456789i64,-0x123456789i64,-0x123456789i64,0,1,2,3,4,5,6,7).b,
 UUID_INIT!(-0x123456789i64,-0x123456789i64,-0x123456789i64,0,1,2,3,4,5,6,7).b,
];
/// Return const/runtime initializers and repeated-expression counts to C.
/// # Safety
/// Output points to 16 writable bytes.
#[no_mangle]
pub unsafe extern "C" fn uuid_initializer_check(which: u32, value: u64, out: *mut u8) -> u32 {
 let mut counts = [0u32; 11];
 let mut next = |index: usize| { counts[index] += 1; value };
 let bytes = match which {
  0..=5 => INITIALIZERS[which as usize],
  6 => GUID_INIT!(value as u8,value as u8,value as u8,0,1,2,3,4,5,6,7).b,
  7 => UUID_INIT!(value as u8,value as u8,value as u8,0,1,2,3,4,5,6,7).b,
  8 => GUID_INIT!(value as i8,value as i8,value as i8,0,1,2,3,4,5,6,7).b,
  9 => UUID_INIT!(value as i8,value as i8,value as i8,0,1,2,3,4,5,6,7).b,
  10 => GUID_INIT!(value,value,value,0,1,2,3,4,5,6,7).b,
  11 => UUID_INIT!(value,value,value,0,1,2,3,4,5,6,7).b,
  12 => GUID_INIT!(next(0),next(1),next(2),next(3),next(4),next(5),next(6),next(7),next(8),next(9),next(10)).b,
  14..=15 => SIGNED_WIDE[(which - 14) as usize],
  16 => GUID_INIT!(value as i64,value as i64,value as i64,0,1,2,3,4,5,6,7).b,
  17 => UUID_INIT!(value as i64,value as i64,value as i64,0,1,2,3,4,5,6,7).b,
  _ => UUID_INIT!(next(0),next(1),next(2),next(3),next(4),next(5),next(6),next(7),next(8),next(9),next(10)).b,
 };
 // SAFETY: Caller supplies a disjoint writable output array of 16 bytes.
 unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), out, 16); }
 if (which != 12 && which != 13) || counts == [4,2,2,1,1,1,1,1,1,1,1] { 1 } else { 0 }
}
