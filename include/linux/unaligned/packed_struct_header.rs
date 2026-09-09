// Dependency: `u16`, `u32`, and `u64` correspond to the Linux kernel types
// supplied by `<linux/types.h>`.

#[repr(C, packed)]
pub struct __una_u16 {
    pub x: u16,
}

#[repr(C, packed)]
pub struct __una_u32 {
    pub x: u32,
}

#[repr(C, packed)]
pub struct __una_u64 {
    pub x: u64,
}

pub unsafe fn __get_unaligned_cpu16(p: *const core::ffi::c_void) -> u16 {
    let ptr = p as *const __una_u16;
    core::ptr::read_unaligned(core::ptr::addr_of!((*ptr).x))
}

pub unsafe fn __get_unaligned_cpu32(p: *const core::ffi::c_void) -> u32 {
    let ptr = p as *const __una_u32;
    core::ptr::read_unaligned(core::ptr::addr_of!((*ptr).x))
}

pub unsafe fn __get_unaligned_cpu64(p: *const core::ffi::c_void) -> u64 {
    let ptr = p as *const __una_u64;
    core::ptr::read_unaligned(core::ptr::addr_of!((*ptr).x))
}

pub unsafe fn __put_unaligned_cpu16(val: u16, p: *mut core::ffi::c_void) {
    let ptr = p as *mut __una_u16;
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*ptr).x), val);
}

pub unsafe fn __put_unaligned_cpu32(val: u32, p: *mut core::ffi::c_void) {
    let ptr = p as *mut __una_u32;
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*ptr).x), val);
}

pub unsafe fn __put_unaligned_cpu64(val: u64, p: *mut core::ffi::c_void) {
    let ptr = p as *mut __una_u64;
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*ptr).x), val);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
