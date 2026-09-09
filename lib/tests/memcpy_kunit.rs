// SPDX-License-Identifier: GPL-2.0
/* Test cases for memcpy(), memmove(), and memset(). */

use core::ffi::c_void;

#[repr(C)]
pub union SomeBytesView {
    pub data: [u8; 32],
    pub fields: SomeBytesFields,
}

#[repr(C)]
pub struct SomeBytesFields {
    pub one: u32,
    pub two: u16,
    pub three: u8,
    pub _hole: u8,
    pub four: [u32; 4],
}

#[repr(C)]
pub struct SomeBytes {
    pub data: [u8; 32],
}

#[allow(non_camel_case_types)]
pub enum kunit {}
extern "C" {
    fn get_random_bytes(buf: *mut c_void, len: usize);
    fn cond_resched();
    fn kunit_assert_eq(test: *mut kunit, a: usize, b: usize);
}

static mut LARGER_ARRAY: [u8; 2048] = [0; 2048];
static mut LARGE_SRC: [u8; 1024] = [0; 1024];
static mut LARGE_DST: [u8; 2048] = [0; 2048];
static LARGE_ZERO: [u8; 2048] = [0; 2048];

unsafe fn check(_test: *mut kunit, instance: &SomeBytes, value: u8) {
    for i in 0..instance.data.len() {
        assert_eq!(instance.data[i], value);
    }
}

unsafe fn compare(_test: *mut kunit, _name: &str, one: &SomeBytes, two: &SomeBytes) {
    for i in 0..one.data.len() {
        assert_eq!(one.data[i], two.data[i]);
    }
}

unsafe fn memcpy_test(test: *mut kunit) {
    let control = SomeBytes { data: [0x20; 32] };
    let zero = SomeBytes { data: [0; 32] };
    let mut dest = SomeBytes { data: [0; 32] };
    let middle = SomeBytes { data: [0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0,0,0,0,0,0,0,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20] };
    let three = SomeBytes { data: [0,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0,0,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20] };
    check(test, &control, 0x20); check(test, &zero, 0); compare(test, "static initializers", &dest, &zero);
    dest = control; compare(test, "direct assignment", &dest, &control);
    dest.data.copy_from_slice(&zero.data); compare(test, "complete overwrite", &dest, &zero);
    dest = control; dest.data[12..19].copy_from_slice(&zero.data[..7]); compare(test, "middle overwrite", &dest, &middle);
    dest = control; let mut count = 1usize; let mut ptr = dest.data.as_mut_ptr(); *ptr = zero.data[0]; ptr = ptr.add(1); count += 1; ptr = ptr.add(8); *ptr = zero.data[0]; let _ = ptr.add(1); count += 1; let _ = count;
    compare(test, "argument side-effects", &dest, &three);
}

unsafe fn memmove_test(test: *mut kunit) {
    let control = SomeBytes { data: [0x99; 32] }; let zero = SomeBytes { data: [0; 32] }; let mut dest = SomeBytes { data: [0; 32] };
    let middle = SomeBytes { data: [0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0,0,0,0,0,0,0,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99] };
    let five = SomeBytes { data: [0,0,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0,0,0,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99] };
    check(test,&control,0x99); check(test,&zero,0); compare(test,"static initializers",&zero,&dest);
    dest=control; compare(test,"direct assignment",&dest,&control); dest.data.copy_from_slice(&zero.data); compare(test,"complete overwrite",&dest,&zero);
    dest=control; dest.data[12..19].copy_from_slice(&zero.data[..7]); compare(test,"middle overwrite",&dest,&middle);
    dest=control; dest.data[0..2].copy_from_slice(&zero.data[..2]); dest.data[11..13].copy_from_slice(&zero.data[..2]); compare(test,"argument side-effects",&dest,&five);
    let mut overlap = SomeBytes { data: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99,0x99] };
    overlap.data.copy_within(0..5,2); assert_eq!(&overlap.data[..7], &[0,1,0,1,2,3,4]);
    LARGER_ARRAY[256]=0xaa; core::ptr::copy(LARGER_ARRAY.as_ptr().add(256),LARGER_ARRAY.as_mut_ptr(),1024); assert_eq!(LARGER_ARRAY[0],0xaa); assert_eq!(LARGER_ARRAY[256],0);
    LARGER_ARRAY[0]=0xbb; core::ptr::copy(LARGER_ARRAY.as_ptr(),LARGER_ARRAY.as_mut_ptr().add(256),1024); assert_eq!(LARGER_ARRAY[0],0xbb); assert_eq!(LARGER_ARRAY[256],0xbb);
}

unsafe fn memset_test(test: *mut kunit) {
    let control=SomeBytes{data:[0x30;32]}; let mut dest=SomeBytes{data:[0;32]}; let complete=SomeBytes{data:[0xff;32]};
    check(test,&control,0x30); check(test,&dest,0); dest=control; compare(test,"direct assignment",&dest,&control); dest.data.fill(0xff); compare(test,"complete overwrite",&dest,&complete);
    dest=control; dest.data[4..20].fill(0x31); compare(test,"middle overwrite",&dest,&SomeBytes{data:[0x30,0x30,0x30,0x30,0x31,0x31,0x31,0x31,0x31,0x31,0x31,0x31,0x31,0x31,0x31,0x31,0x31,0x31,0x31,0x31,0x30,0x30,0x30,0x30,0x30,0x30,0x30,0x30,0x30,0x30,0x30,0x30]});
    dest=control; dest.data[0]=0x60; dest.data[9..11].fill(0x61); compare(test,"argument side-effects",&dest,&dest);
}

unsafe fn set_random_nonzero(_test:*mut kunit, byte:*mut u8) { while *byte==0 { get_random_bytes(byte as *mut c_void,1); } }
unsafe fn init_large(test:*mut kunit) { get_random_bytes(LARGE_SRC.as_mut_ptr() as *mut c_void, LARGE_SRC.len()); set_random_nonzero(test,LARGE_SRC.as_mut_ptr()); set_random_nonzero(test,LARGE_SRC.as_mut_ptr().add(1023)); LARGE_DST.fill(0); }
unsafe fn copy_large_test(test:*mut kunit, use_memmove:bool) { init_large(test); for bytes in 1..=1024 { for offset in 0..1024 { if use_memmove { core::ptr::copy(LARGE_SRC.as_ptr(),LARGE_DST.as_mut_ptr().add(offset),bytes); } else { core::ptr::copy_nonoverlapping(LARGE_SRC.as_ptr(),LARGE_DST.as_mut_ptr().add(offset),bytes); } assert_eq!(&LARGE_DST[offset..offset+bytes],&LARGE_SRC[..bytes]); LARGE_DST[offset..offset+bytes].fill(0); } cond_resched(); } }
unsafe fn memcpy_large_test(t:*mut kunit){copy_large_test(t,false)} unsafe fn memmove_large_test(t:*mut kunit){copy_large_test(t,true)}
fn next_step(mut idx:i32,start:i32,end:i32,mut inc:i32)->i32 { let start=start+inc; let end=end-inc; if idx<start || idx+inc>end {inc=1;} idx+inc }
unsafe fn memmove_overlap_test(test:*mut kunit){init_large(test); for bytes in 1..=1024 { for d_off in 0..128 { let s_start=(d_off-bytes as i32).max(0) as usize; let s_end=(d_off+bytes as i32).min(1024) as usize; for s_off in s_start..s_end { core::ptr::copy(LARGE_DST.as_ptr().add(s_off),LARGE_DST.as_mut_ptr().add(d_off),bytes); } } cond_resched(); } }

// KUnit registration and module metadata are supplied by the kernel integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
