// SPDX-License-Identifier: GPL-2.0-only
use ffi::{c_char, c_int};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
struct Event { kind: c_int, null: c_int, value: [c_char; 256] }

unsafe extern "C" {
    fn original_early_options(value: *mut c_char);
    fn original_early_param();
    fn original_early_callback(parameter: *mut c_char, value: *mut c_char) -> c_int;
    fn original_early_reset();
    fn original_event_count() -> u32;
    fn original_parser_calls() -> u32;
    fn original_events() -> *const Event;
    static mut original_boot_command_line: [c_char; bindings::RUST_INIT_MAIN_COMMAND_LINE_SIZE as usize];
}

fn terminated(text: &str) -> Vec<c_char> {
    text.bytes().chain(core::iter::once(0)).collect()
}

unsafe fn snapshot() -> (u32, Vec<Event>) {
    unsafe {
        (original_parser_calls(), core::slice::from_raw_parts(original_events(),
            original_event_count() as usize).to_vec())
    }
}

fn main() {
    unsafe {
        for name in ["early", "earlyx", "bad", "dash_name", "dash-name", "other", "", "obsolete", "\u{80}"] {
            for value in [None, Some(""), Some("alpha beta"), Some("\u{80}=x")] {
                let mut parameter = terminated(name);
                let mut value = value.map(terminated);
                let pointer = value.as_mut().map_or(core::ptr::null_mut(), |v| v.as_mut_ptr());
                original_early_reset();
                let result = original_early_callback(parameter.as_mut_ptr(), pointer);
                let expected = snapshot();
                original_early_reset();
                assert_eq!(main_early::do_early_param(parameter.as_mut_ptr(), pointer,
                    core::ptr::null(), core::ptr::null_mut()), result);
                assert_eq!(snapshot(), expected);
            }
        }
        let mut inputs = vec![String::new(), "  \t\n".into(), "-- early=x".into(),
            "early -- bad=x".into(), "early=\"two words\" bad='literal'".into(),
            "\"early=quoted value\" dash-name=word".into(), "early=\"unfinished".into()];
        for left in ["early", "early=", "bad=7", "dash-name=x", "unknown=ignored", "other", "=empty"] {
            for right in ["", " early=again", " -- bad=no", "\tearly=\"a b\"", "\nobsolete"] {
                inputs.push(format!("{left}{right}"));
            }
        }
        for input in inputs {
            let mut c = terminated(&input);
            let mut rust = c.clone();
            original_early_reset();
            original_early_options(c.as_mut_ptr());
            let expected = snapshot();
            original_early_reset();
            main_early::parse_early_options(rust.as_mut_ptr());
            assert_eq!(snapshot(), expected, "{input:?}");
            assert_eq!(rust, c, "tokenizer mutations {input:?}");
        }

        // Exact-size terminated boot storage, retained copy mutations and the
        // second call after a changed boot source exercise the true once flag.
        let length = bindings::RUST_INIT_MAIN_COMMAND_LINE_SIZE as usize;
        let mut boot = vec![b' '; length];
        let prefix = b"early=first dash-name=next -- bad=unparsed";
        boot[..prefix.len()].copy_from_slice(prefix);
        boot[length - 1] = 0;
        core::ptr::addr_of_mut!(original_boot_command_line).cast::<c_char>()
            .copy_from_nonoverlapping(boot.as_ptr(), length);
        core::ptr::addr_of_mut!(main_globals::boot_command_line).cast::<c_char>()
            .copy_from_nonoverlapping(boot.as_ptr(), length);
        original_early_reset();
        original_early_param();
        let expected = snapshot();
        original_early_reset();
        main_early::parse_early_param();
        assert_eq!(snapshot(), expected);
        assert_eq!(expected.0, 1);
        assert_eq!(core::slice::from_raw_parts(core::ptr::addr_of!(main_globals::boot_command_line)
            .cast::<c_char>(), length), boot);
        assert_eq!({ main_globals::parse_early_param_done }, 1);
        core::ptr::addr_of_mut!(original_boot_command_line).cast::<c_char>().write(b'x');
        core::ptr::addr_of_mut!(main_globals::boot_command_line).cast::<c_char>().write(b'x');
        original_early_reset();
        original_early_param();
        let expected = snapshot();
        main_early::parse_early_param();
        assert_eq!(snapshot(), expected);
        assert_eq!(expected, (0, vec![]));
    }
    println!("INIT_MAIN_EARLY_OK");
}
