// SPDX-License-Identifier: GPL-2.0-only
use ffi::{c_char, c_int};

#[no_mangle]
static mut console_printk: [c_int; 4] = [0; 4];

unsafe extern "C" {
    fn original_parameter(callback: u32, value: *mut c_char, level: c_int, reset: u32) -> c_int;
    fn original_console(index: u32) -> c_int;
    fn original_reset() -> u32;
}

fn main() {
    let mut inputs = vec![None];
    for value in ["", "-", "+", "+3", " 5", "- 5", "--1", "0x", "0xF", "0Xabcdef", "09",
                  "-0x80000000", "2147483647", "2147483648", "4294967295", "4294967296",
                  "18446744073709551615", "18446744073709551616", "-18446744073709551616",
                  "999999999999999999999999999999999999", "-999999999999999999999999999999999999",
                  "7,", "2-5", ",4", "0,3", "5word", "4\n"] {
        inputs.push(Some(value.to_owned()));
    }
    for value in -20..256 { inputs.push(Some(value.to_string())); }
    let mut cases = 0;
    for input in inputs {
        let mut input = input.map(|value| { let mut bytes = value.into_bytes(); bytes.push(0); bytes });
        let value = input.as_mut().map_or(core::ptr::null_mut(), |bytes| bytes.as_mut_ptr());
        for callback in 0..4 {
            for level in [-1, 0, 4, 10] {
                unsafe {
                    console_printk = [level, 11, 22, 33];
                    main_globals::reset_devices = 0xa5a5;
                    let result = match callback {
                        0 => main_parameters::set_reset_devices(value),
                        1 => main_parameters::debug_kernel(value),
                        2 => main_parameters::quiet_kernel(value),
                        _ => main_parameters::loglevel(value),
                    };
                    assert_eq!(result, original_parameter(callback, value, level, 0xa5a5));
                    for index in 0..4 {
                        assert_eq!(console_printk[index], original_console(index as u32));
                    }
                    assert_eq!({ main_globals::reset_devices }, original_reset());
                }
                cases += 1;
            }
        }
    }
    println!("INIT_MAIN_PARAMETERS_OK cases={cases}");
}
