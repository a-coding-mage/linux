// SPDX-License-Identifier: GPL-2.0-only
use ffi::c_char;

unsafe extern "C" {
    fn original_parameter() -> *const bindings::kernel_param;
    fn original_reset(value: bool);
    fn original_value() -> bool;
}

fn main() {
    unsafe {
        let original = original_parameter();
        let rust = core::ptr::addr_of!(main_core_param::INITCALL_DEBUG_PARAMETER.0);
        assert_eq!((*rust).mod_, (*original).mod_);
        assert_eq!((*rust).ops, (*original).ops);
        assert_eq!((*rust).perm, (*original).perm);
        assert_eq!((*rust).level, (*original).level);
        assert_eq!((*rust).flags, (*original).flags);
        assert_eq!((*rust).__bindgen_anon_1.arg,
                   core::ptr::addr_of_mut!(main_globals::initcall_debug).cast());
        assert_eq!(core::slice::from_raw_parts((*rust).name, b"initcall_debug\0".len()), b"initcall_debug\0");
        let mut cases = 0;
        for byte in 0..258 {
            let mut value = [byte as c_char, b'n', 0];
            if byte == 256 { value = [b'o', b'f', 0]; }
            let value = if byte == 257 { core::ptr::null() } else { value.as_ptr() };
            for initial in [false, true] {
                original_reset(initial);
                main_globals::initcall_debug = initial;
                let c_result = ((*(*original).ops).set.unwrap())(value, original);
                let rust_result = ((*(*rust).ops).set.unwrap())(value, rust);
                assert_eq!(rust_result, c_result);
                assert_eq!({ main_globals::initcall_debug }, original_value());
                let mut c_buffer = [0xa5; 8];
                let mut rust_buffer = c_buffer;
                assert_eq!(((*(*rust).ops).get.unwrap())(rust_buffer.as_mut_ptr(), rust),
                           ((*(*original).ops).get.unwrap())(c_buffer.as_mut_ptr(), original));
                assert_eq!(rust_buffer, c_buffer);
                cases += 1;
            }
        }
        println!("INIT_MAIN_CORE_PARAM_OK cases={cases}");
    }
}
