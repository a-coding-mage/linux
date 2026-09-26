// SPDX-License-Identifier: GPL-2.0-only
// Included by a wrapper that imports the real production state and functions.
use ffi::{c_char, c_int};
use std::{alloc::{alloc_zeroed, dealloc, Layout}, ffi::CStr, ptr};

unsafe extern "C" {
    fn original_prepare(boot: *mut c_char, extra: *mut c_char, args: *mut c_char, offset: usize);
    fn original_setup(arch: *mut c_char);
    fn original_saved() -> *mut c_char;
    fn original_static() -> *mut c_char;
    fn original_extra() -> *mut c_char;
    fn original_length() -> u32;
    fn original_cmdline_has_extra_options() -> bool;
    fn original_init(value: *mut c_char, ramdisk: bool) -> c_int;
    fn original_arg(index: u32) -> *const c_char;
    fn original_execute() -> *mut c_char;
    fn original_rdinit() -> *mut c_char;
    fn original_rdinit_set() -> bool;
}

// Only the allocation boundary is instrumented. Allocation policy, NUMA and
// physical reservations require later native early-boot execution.
static mut ALLOCATIONS: Vec<(*mut u8, Layout, Vec<u8>)> = Vec::new();
#[no_mangle]
unsafe extern "C" fn __memblock_alloc_or_panic(
    size: bindings::phys_addr_t,
    align: bindings::phys_addr_t,
    function: *const c_char,
) -> *mut ffi::c_void {
    let layout = Layout::from_size_align(size as usize, align as usize).unwrap();
    let memory = unsafe { alloc_zeroed(layout) };
    assert!(!memory.is_null());
    let function = unsafe { CStr::from_ptr(function.cast()) }.to_bytes().to_vec();
    unsafe { (*ptr::addr_of_mut!(ALLOCATIONS)).push((memory, layout, function)); }
    memory.cast()
}

macro_rules! boundary {
    ($($name:ident),*) => {$(
        #[no_mangle] static mut $name: [bindings::initcall_entry_t; 0] = [];
    )*};
}
boundary!(__initcall0_start, __initcall1_start, __initcall2_start, __initcall3_start,
          __initcall4_start, __initcall5_start, __initcall6_start, __initcall7_start,
          __initcall_end);

fn terminated(value: &[u8]) -> Vec<c_char> {
    assert!(!value.contains(&0));
    let mut result = value.to_vec();
    result.push(0);
    result
}

unsafe fn string(pointer: *const c_char) -> Vec<u8> {
    unsafe { CStr::from_ptr(pointer.cast()) }.to_bytes().to_vec()
}

unsafe fn check_case(boot: &[u8], arch: &[u8], extra: Option<&[u8]>, args: Option<&[u8]>, offset: usize) {
    let mut boot = terminated(boot);
    let mut arch = terminated(arch);
    let mut extra = extra.map(terminated);
    let mut original_args = args.map(terminated);
    let mut rust_args = args.map(terminated);
    let extra_ptr = extra.as_mut().map_or(ptr::null_mut(), |v| v.as_mut_ptr());
    let old_args = original_args.as_mut().map_or(ptr::null_mut(), |v| v.as_mut_ptr());
    let new_args = rust_args.as_mut().map_or(ptr::null_mut(), |v| v.as_mut_ptr());
    unsafe {
        original_prepare(boot.as_mut_ptr(), extra_ptr, old_args, offset);
        ptr::copy_nonoverlapping(boot.as_ptr(), ptr::addr_of_mut!(boot_command_line).cast(), boot.len());
        extra_command_line = extra_ptr;
        extra_init_args = new_args;
        #[cfg(CONFIG_BOOT_CONFIG)] { initargs_offs = offset; }
        assert_eq!(original_cmdline_has_extra_options(), main_command_line::cmdline_has_extra_options());
        original_setup(arch.as_mut_ptr());
        main_command_line::setup_command_line(arch.as_mut_ptr());
        assert_eq!(string(original_saved()), string(saved_command_line));
        assert_eq!(string(original_static()), string(static_command_line));
        assert_eq!(original_length(), { saved_command_line_len });
        assert_eq!(original_args, rust_args);
        if !old_args.is_null() {
            assert_eq!(original_extra().offset_from(old_args), extra_init_args.offset_from(new_args));
        }
        let allocations = &mut *ptr::addr_of_mut!(ALLOCATIONS);
        assert_eq!(allocations.len(), 4);
        for index in 0..2 {
            assert_eq!(allocations[index].1, allocations[index + 2].1);
            assert_eq!(allocations[index].2, b"setup_command_line");
            assert_eq!(allocations[index].2, allocations[index + 2].2);
        }
        // A mutable parser copy must not change the saved /proc command line.
        let saved = string(saved_command_line);
        if *static_command_line != 0 { *static_command_line = b'!'; }
        assert_eq!(string(saved_command_line), saved);
        for (memory, layout, _) in allocations.drain(..) { dealloc(memory, layout); }
    }
}

fn main() {
    let mut cases = 0usize;
    for boot in [b"".as_slice(), b"root=/dev/vda console=ttyS0", b"arg -- user=old", b" -- first"] {
        for arch in [boot, b"architecture=rewritten -- untouched"] {
            for extra in [None, Some(b"".as_slice()), Some(b"config=first ")] {
                for args in [None, Some(b"".as_slice()), Some(b"\t \n"), Some(b"new=one"),
                             Some(b"  new=one new=two\t\n"), Some(b"\xa0value\xa0")] {
                    unsafe { check_case(boot, arch, extra, args, 0); }
                    cases += 1;
                    #[cfg(CONFIG_BOOT_CONFIG)]
                    if let Some(index) = boot.windows(4).position(|window| window == b" -- ") {
                        unsafe { check_case(boot, arch, extra, args, index + 4); }
                        cases += 1;
                    }
                }
            }
        }
    }
    let long = vec![b'x'; bindings::RUST_INIT_MAIN_COMMAND_LINE_SIZE as usize - 1];
    unsafe { check_case(&long, b"short", Some(b"prefix "), Some(b" added "), 0); }
    cases += 1;
    for ramdisk in [false, true] {
        let mut value = terminated(b"/custom/init");
        unsafe {
            (*ptr::addr_of_mut!(argv_init)).fill(value.as_ptr());
            execute_command = ptr::null_mut();
            ramdisk_execute_command = c"/init".as_ptr().cast_mut().cast();
            ramdisk_execute_command_set = false;
            let result = if ramdisk { main_command_line::rdinit_setup(value.as_mut_ptr()) }
                         else { main_command_line::init_setup(value.as_mut_ptr()) };
            assert_eq!(result, original_init(value.as_mut_ptr(), ramdisk));
            let argv = &*ptr::addr_of!(argv_init);
            for (index, arg) in argv.iter().enumerate() {
                assert_eq!(*arg, original_arg(index as u32));
                assert_eq!(arg.is_null(), index > 0 && index < bindings::RUST_INIT_MAIN_MAX_INIT_ARGS as usize);
            }
            assert_eq!({ execute_command }, original_execute());
            assert_eq!(string(ramdisk_execute_command), string(original_rdinit()));
            assert_eq!({ ramdisk_execute_command_set }, original_rdinit_set());
        }
        cases += 1;
    }
    println!("INIT_MAIN_COMMAND_LINE_OK cases={cases}");
}
