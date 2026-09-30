// SPDX-License-Identifier: GPL-2.0-only
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Event { kind: i32, argument: i32, value: [u8; 160] }
extern "C" {
    fn fixture_reset(line: *const ffi::c_char, fail: bool, mutate: bool, length: u32) -> *mut ffi::c_char;
    fn fixture_original(mode: u32);
    fn fixture_event_count() -> u32;
    fn fixture_events() -> *const Event;
}
unsafe fn prepare(line: &[u8], fail: bool, mutate: bool, length: u32) {
    unsafe {
        main_globals::saved_command_line = fixture_reset(line.as_ptr().cast(), fail, mutate, length);
        main_globals::saved_command_line_len = length;
    }
}
unsafe fn rust_run(mode: u32) {
    unsafe {
        match mode {
            0 => main_initcall_levels::do_initcalls(),
            1 => main_initcall_levels::do_basic_setup(),
            _ => main_initcall_levels::do_pre_smp_initcalls(),
        }
    }
}
fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() == 3 {
        unsafe {
            prepare(b"\0", true, false, arguments[2].parse().unwrap());
            if arguments[1] == "original" { fixture_original(0); } else { rust_run(0); }
        }
        panic!("allocation failure returned");
    }
    let inputs: &[&[u8]] = &[
        b"\0", b"  \0", b"unknown no-value key=value\0",
        b"level0=a level1=b level2=c level3=d level4=e level5=f level6=g level7=h\0",
        b"level0=bad level1=good level2=bad level2=ok level7=last\0",
        b"level4=\"two words\" level6='quoted' level7=\"\"\0",
        b"early_only=ignored level0=one -- level1=not-parsed\0",
        b"level0 level1 level2=bad unknown=safe\0",
        b" level3=a\tlevel5=b\nlevel7=c \0",
        b"early-only=no level0=x level0=y level0=z\0",
    ];
    let mut cases = 0;
    unsafe {
        for line in inputs {
            for mode in 0..3 {
                for mutate in [false, true] {
                    prepare(line, false, mutate, (line.len() - 1) as u32);
                    fixture_original(mode);
                    let expected = core::slice::from_raw_parts(fixture_events(), fixture_event_count() as usize).to_vec();
                    prepare(line, false, mutate, (line.len() - 1) as u32);
                    rust_run(mode);
                    let actual = core::slice::from_raw_parts(fixture_events(), fixture_event_count() as usize);
                    assert_eq!(actual, expected, "mode={mode}, mutate={mutate}, line={line:?}");
                    assert!(!actual.is_empty());
                    cases += 1;
                }
            }
        }
    }
    println!("INIT_MAIN_LEVELS_OK cases={cases}");
}
