// SPDX-License-Identifier: GPL-2.0-only
/* UBSAN error reporting functions */

// Linux dependencies supplied by the surrounding kernel translation.

#[cfg(any(feature = "CONFIG_UBSAN_TRAP", feature = "CONFIG_UBSAN_KVM_EL2"))]
pub unsafe fn report_ubsan_failure(check_type: u32) -> *const core::ffi::c_char {
    match check_type {
        #[cfg(feature = "CONFIG_UBSAN_BOUNDS")]
        ubsan_out_of_bounds => c"UBSAN: array index out of bounds".as_ptr(),
        #[cfg(feature = "CONFIG_UBSAN_SHIFT")]
        ubsan_shift_out_of_bounds => c"UBSAN: shift out of bounds".as_ptr(),
        #[cfg(any(feature = "CONFIG_UBSAN_DIV_ZERO", feature = "CONFIG_UBSAN_INTEGER_WRAP"))]
        ubsan_divrem_overflow => c"UBSAN: divide/remainder overflow".as_ptr(),
        #[cfg(feature = "CONFIG_UBSAN_UNREACHABLE")]
        ubsan_builtin_unreachable => c"UBSAN: unreachable code".as_ptr(),
        #[cfg(any(feature = "CONFIG_UBSAN_BOOL", feature = "CONFIG_UBSAN_ENUM"))]
        ubsan_load_invalid_value => c"UBSAN: loading invalid value".as_ptr(),
        #[cfg(feature = "CONFIG_UBSAN_ALIGNMENT")]
        ubsan_alignment_assumption => c"UBSAN: alignment assumption".as_ptr(),
        #[cfg(feature = "CONFIG_UBSAN_ALIGNMENT")]
        ubsan_type_mismatch => c"UBSAN: type mismatch".as_ptr(),
        #[cfg(feature = "CONFIG_UBSAN_INTEGER_WRAP")]
        ubsan_add_overflow => c"UBSAN: integer addition overflow".as_ptr(),
        #[cfg(feature = "CONFIG_UBSAN_INTEGER_WRAP")]
        ubsan_sub_overflow => c"UBSAN: integer subtraction overflow".as_ptr(),
        #[cfg(feature = "CONFIG_UBSAN_INTEGER_WRAP")]
        ubsan_mul_overflow => c"UBSAN: integer multiplication overflow".as_ptr(),
        _ => c"UBSAN: unrecognized failure code".as_ptr(),
    }
}

#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
const TYPE_CHECK_KINDS: [&[u8]; 8] = [
    b"load of", b"store to", b"reference binding to", b"member access within",
    b"member call on", b"constructor call on", b"downcast of", b"downcast of",
];
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
const REPORTED_BIT: u32 = 31;
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
const VALUE_LENGTH: usize = 40;

#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
unsafe fn was_reported(location: *mut source_location) -> bool { test_and_set_bit(REPORTED_BIT, &mut (*location).reported) }
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
unsafe fn suppress_report(location: *mut source_location) -> bool { (*current).in_ubsan != 0 || was_reported(location) }
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
unsafe fn type_is_int(ty: *mut type_descriptor) -> bool { (*ty).type_kind == type_kind_int }
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
unsafe fn type_is_signed(ty: *mut type_descriptor) -> bool { WARN_ON(!type_is_int(ty)); (*ty).type_info & 1 != 0 }
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
unsafe fn type_bit_width(ty: *mut type_descriptor) -> u32 { 1 << ((*ty).type_info >> 1) }
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
unsafe fn is_inline_int(ty: *mut type_descriptor) -> bool { WARN_ON(!type_is_int(ty)); type_bit_width(ty) <= (core::mem::size_of::<usize>() as u32 * 8) }

#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
unsafe fn get_signed_val(ty: *mut type_descriptor, val: *mut core::ffi::c_void) -> s_max {
    if is_inline_int(ty) { let extra = (core::mem::size_of::<s_max>() as u32 * 8) - type_bit_width(ty); return (((val as usize) as s_max) << extra) >> extra; }
    if type_bit_width(ty) == 64 { return *(val as *const i64) as s_max; }
    *(val as *const s_max)
}
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
unsafe fn val_is_negative(ty: *mut type_descriptor, val: *mut core::ffi::c_void) -> bool { type_is_signed(ty) && get_signed_val(ty, val) < 0 }
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
unsafe fn get_unsigned_val(ty: *mut type_descriptor, val: *mut core::ffi::c_void) -> u_max { if is_inline_int(ty) { val as usize as u_max } else if type_bit_width(ty) == 64 { *(val as *const u64) as u_max } else { *(val as *const u_max) } }

// Kernel formatting/logging helpers and UBSAN data structures are declared by ubsan.h.
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
extern "C" {
    static mut current: *mut task_struct;
    fn test_and_set_bit(bit: u32, addr: *mut u32) -> bool;
    fn WARN_ON(condition: bool);
    fn user_access_save() -> usize;
    fn user_access_restore(flags: usize);
    fn dump_stack();
    fn check_panic_on_warn(name: *const core::ffi::c_char);
    fn panic(msg: *const core::ffi::c_char) -> !;
}

#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
unsafe fn val_to_string(out: &mut String, ty: *mut type_descriptor, value: *mut core::ffi::c_void) {
    if !type_is_int(ty) { return; }
    if type_bit_width(ty) == 128 { out.push_str(&format!("0x{:08x}{:08x}{:08x}{:08x}", (get_unsigned_val(ty,value)>>96) as u32, (get_unsigned_val(ty,value)>>64) as u32, (get_unsigned_val(ty,value)>>32) as u32, get_unsigned_val(ty,value) as u32)); }
    else if type_is_signed(ty) { out.push_str(&format!("{}", get_signed_val(ty,value) as i64)); }
    else { out.push_str(&format!("{}", get_unsigned_val(ty,value) as u64)); }
}

#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
unsafe fn handle_overflow(data:*mut core::ffi::c_void, lhs:*mut core::ffi::c_void, rhs:*mut core::ffi::c_void, _op:u8) {
    let d = data as *mut overflow_data;
    if suppress_report((*d).location) { return; }
    ubsan_prologue((*d).location, if type_is_signed((*d).ty) { c"signed-integer-overflow".as_ptr() } else { c"unsigned-integer-overflow".as_ptr() });
    let mut l=String::new(); let mut r=String::new(); val_to_string(&mut l,(*d).ty,lhs); val_to_string(&mut r,(*d).ty,rhs);
    ubsan_epilogue();
}
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
unsafe fn ubsan_prologue(_loc:*mut source_location,_reason:*const core::ffi::c_char){(*current).in_ubsan+=1;}
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
unsafe fn ubsan_epilogue(){dump_stack();(*current).in_ubsan-=1;check_panic_on_warn(c"UBSAN".as_ptr());}

#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
#[no_mangle] pub unsafe extern "C" fn __ubsan_handle_add_overflow(data:*mut core::ffi::c_void,lhs:*mut core::ffi::c_void,rhs:*mut core::ffi::c_void){ handle_overflow(data,lhs,rhs,b'+'); }
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
#[no_mangle] pub unsafe extern "C" fn __ubsan_handle_sub_overflow(data:*mut core::ffi::c_void,lhs:*mut core::ffi::c_void,rhs:*mut core::ffi::c_void){ handle_overflow(data,lhs,rhs,b'-'); }
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
#[no_mangle] pub unsafe extern "C" fn __ubsan_handle_mul_overflow(data:*mut core::ffi::c_void,lhs:*mut core::ffi::c_void,rhs:*mut core::ffi::c_void){ handle_overflow(data,lhs,rhs,b'*'); }
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
#[no_mangle] pub unsafe extern "C" fn __ubsan_handle_negate_overflow(data:*mut core::ffi::c_void,old:*mut core::ffi::c_void){let d=data as *mut overflow_data;if suppress_report((*d).location){return}ubsan_prologue((*d).location,c"negation-overflow".as_ptr());let mut s=String::new();val_to_string(&mut s,(*d).ty,old);ubsan_epilogue();}
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
#[no_mangle] pub unsafe extern "C" fn __ubsan_handle_divrem_overflow(data:*mut core::ffi::c_void,lhs:*mut core::ffi::c_void,_rhs:*mut core::ffi::c_void){let d=data as *mut overflow_data;if suppress_report((*d).location){return}ubsan_prologue((*d).location,c"division-overflow".as_ptr());let mut s=String::new();val_to_string(&mut s,(*d).ty,lhs);ubsan_epilogue();}
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
#[no_mangle] pub unsafe extern "C" fn __ubsan_handle_out_of_bounds(data:*mut core::ffi::c_void,_index:*mut core::ffi::c_void){let d=data as *mut out_of_bounds_data;if suppress_report((*d).location){return}ubsan_prologue((*d).location,c"array-index-out-of-bounds".as_ptr());ubsan_epilogue();}
#[cfg(not(feature = "CONFIG_UBSAN_TRAP"))]
#[no_mangle] pub unsafe extern "C" fn __ubsan_handle_builtin_unreachable(data:*mut core::ffi::c_void){let d=data as *mut unreachable_data;ubsan_prologue((*d).location,c"unreachable".as_ptr());ubsan_epilogue();panic(c"can't return from __builtin_unreachable()".as_ptr());}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
