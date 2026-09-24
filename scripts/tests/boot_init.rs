// SPDX-License-Identifier: GPL-2.0-only
//! PID 1 fixture for boot-testing images produced by the migrated build tools.

use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::MetadataExt;
use std::time::Duration;

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
compile_error!("the boot fixture supports the x86-64 and AArch64 Linux syscall ABIs");

// Keep module syscalls explicit: the guest libc need not provide wrappers.
#[cfg(target_arch = "x86_64")]
const INIT_MODULE: std::ffi::c_long = 175;
#[cfg(target_arch = "x86_64")]
const DELETE_MODULE: std::ffi::c_long = 176;
// AArch64 uses include/uapi/asm-generic/unistd.h.
#[cfg(target_arch = "aarch64")]
const INIT_MODULE: std::ffi::c_long = 105;
#[cfg(target_arch = "aarch64")]
const DELETE_MODULE: std::ffi::c_long = 106;

fn load_module(bytes: &[u8]) -> io::Result<()> {
    unsafe extern "C" {
        fn syscall(number: std::ffi::c_long, ...) -> std::ffi::c_long;
    }
    // The module buffer and empty NUL-terminated parameters remain valid for
    // the synchronous system call on either supported guest architecture.
    let result = unsafe { syscall(INIT_MODULE, bytes.as_ptr(), bytes.len(), c"".as_ptr()) };
    if result == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

fn unload_module(name: &std::ffi::CStr) -> io::Result<()> {
    unsafe extern "C" {
        fn syscall(number: std::ffi::c_long, ...) -> std::ffi::c_long;
    }
    // O_NONBLOCK avoids waiting forever on a remaining dependency; never use
    // forced module removal in this fixture.
    let result = unsafe { syscall(DELETE_MODULE, name.as_ptr(), 0x800u32) };
    if result == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

fn write_record(writer: &mut impl Write, message: &str) -> io::Result<()> {
    if message
        .bytes()
        .any(|byte| matches!(byte, b'\n' | b'\r' | 0))
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid log record",
        ));
    }
    // /dev/kmsg turns each write into one printk record. Do not use write_all:
    // retrying a short write would create a second, broken marker record.
    let record = format!("<6>{message}\n");
    loop {
        match writer.write(record.as_bytes()) {
            Ok(count) if count == record.len() => return Ok(()),
            Ok(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "short /dev/kmsg record write",
                ))
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        }
    }
}

fn configure_failslab_controls(
    stack_filter: bool,
    mut read: impl FnMut(&str) -> io::Result<String>,
    mut write: impl FnMut(&str, &[u8]) -> io::Result<()>,
) -> io::Result<()> {
    // Probability zero keeps unrelated tasks unaffected. The kernel fixture
    // alone arms current->fail_nth immediately around each tested allocation.
    for (name, value) in [
        ("probability", "0"),
        ("ignore-gfp-wait", "N"),
        ("cache-filter", "N"),
    ] {
        write(name, format!("{value}\n").as_bytes())?;
        if read(name)?.trim() != value {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "failslab control readback differs",
            ));
        }
    }
    // When compiled in, the stack filter also applies to per-task fail_nth.
    // Check the original unrestricted bounds rather than assuming they hold.
    for (name, expected) in [
        ("require-start", 0),
        ("require-end", usize::MAX),
        ("reject-start", 0),
        ("reject-end", 0),
    ] {
        match read(name) {
            Err(error) if !stack_filter && error.kind() == io::ErrorKind::NotFound => (),
            Err(error) => return Err(error),
            Ok(_) if !stack_filter => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "unexpected failslab stack filter control",
                ))
            }
            Ok(text) => {
                let value = usize::from_str_radix(text.trim().trim_start_matches("0x"), 16)
                    .map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "invalid failslab stack bound")
                    })?;
                if value != expected {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "failslab stack filter is restrictive",
                    ));
                }
            }
        }
    }
    Ok(())
}

fn prepare_failslab(stack_filter: bool) -> io::Result<()> {
    unsafe extern "C" {
        fn mount(
            source: *const std::ffi::c_char,
            target: *const std::ffi::c_char,
            filesystem: *const std::ffi::c_char,
            flags: std::ffi::c_ulong,
            data: *const std::ffi::c_void,
        ) -> std::ffi::c_int;
    }
    fs::create_dir_all("/sys/kernel/debug")?;
    // SAFETY: These static strings and NULL data satisfy mount's C interface.
    // This helper is called only by the isolated VM's PID 1, never on the host.
    if unsafe {
        mount(
            c"debugfs".as_ptr(),
            c"/sys/kernel/debug".as_ptr(),
            c"debugfs".as_ptr(),
            0,
            std::ptr::null(),
        )
    } != 0
    {
        return Err(io::Error::last_os_error());
    }
    let root = std::path::Path::new("/sys/kernel/debug/failslab");
    configure_failslab_controls(
        stack_filter,
        |name| fs::read_to_string(root.join(name)),
        |name, value| {
            fs::OpenOptions::new()
                .write(true)
                .open(root.join(name))?
                .write_all(value)
        },
    )
}

fn main() {
    assert_eq!(std::process::id(), 1);
    // Console TTY writes can interleave with a module's printk output even
    // after init_module returns. Use the same record serialization as printk.
    let mut log = fs::OpenOptions::new()
        .write(true)
        .open("/dev/kmsg")
        .unwrap();
    let expected = b"Rust-generated initramfs fixture\n\0with binary data\xff";
    assert_eq!(fs::read("/fixture").unwrap(), expected);
    assert_eq!(fs::read("/hardlink").unwrap(), expected);
    assert_eq!(fs::read("/symlink").unwrap(), expected);
    assert_eq!(
        fs::read_link("/symlink").unwrap().to_str(),
        Some("/fixture")
    );
    let file = fs::metadata("/fixture").unwrap();
    let link = fs::metadata("/hardlink").unwrap();
    assert_eq!(file.ino(), link.ino());
    assert_eq!(file.nlink(), 2);
    assert_eq!(file.uid(), 123);
    assert_eq!(file.gid(), 456);
    assert_eq!(file.mode() & 0o777, 0o640);
    if fs::exists("/failslab-setup").unwrap() {
        let stack_filter = match fs::read("/failslab-setup").unwrap().as_slice() {
            b"failslab-v1\nstacktrace-filter=0\n" => false,
            b"failslab-v1\nstacktrace-filter=1\n" => true,
            _ => panic!("invalid failslab setup plan"),
        };
        prepare_failslab(stack_filter).unwrap();
        write_record(&mut log, "LUPOS_FAILSLAB_SETUP_OK").unwrap();
    }
    // These optional fixtures only exist in the isolated test VM. Check
    // rejected signatures before the valid load, so EEXIST cannot mask them.
    for index in 0.. {
        let path = format!("/reject-module.{index}");
        if !fs::exists(&path).unwrap() {
            break;
        }
        let result = load_module(&fs::read(path).unwrap());
        assert_eq!(
            result.unwrap_err().raw_os_error(),
            Some(129),
            "expected EKEYREJECTED"
        );
        write_record(&mut log, &format!("LUPOS_RUST_MODULE_REJECT_OK {index}")).unwrap();
    }
    let mut loaded_paths = Vec::new();
    for index in 0.. {
        let path = format!("/preload-module.{index}");
        if !fs::exists(&path).unwrap() {
            break;
        }
        load_module(&fs::read(&path).unwrap()).unwrap();
        loaded_paths.push(path);
        write_record(&mut log, &format!("LUPOS_RUST_PRELOAD_OK {index}")).unwrap();
    }
    if fs::exists("/test-module.ko").unwrap() {
        load_module(&fs::read("/test-module.ko").unwrap()).unwrap();
        loaded_paths.push("/test-module.ko".to_owned());
        write_record(&mut log, "LUPOS_RUST_MODULE_LOAD_OK").unwrap();
    }
    if fs::exists("/reload-plan").unwrap() {
        let plan = fs::read_to_string("/reload-plan").unwrap();
        let entries: Vec<_> = plan
            .lines()
            .map(|line| line.split_once('\t').unwrap())
            .collect();
        assert_eq!(entries.len(), loaded_paths.len());
        for ((path, _), loaded) in entries.iter().zip(&loaded_paths) {
            assert_eq!(path, loaded);
        }
        for (index, (_, name)) in entries.iter().enumerate().rev() {
            let name = std::ffi::CString::new(*name).unwrap();
            unload_module(&name).unwrap();
            write_record(&mut log, &format!("LUPOS_RUST_MODULE_UNLOAD_OK {index}")).unwrap();
        }
        for (index, (path, _)) in entries.iter().enumerate() {
            load_module(&fs::read(path).unwrap()).unwrap();
            write_record(&mut log, &format!("LUPOS_RUST_MODULE_RELOAD_OK {index}")).unwrap();
        }
    }
    write_record(&mut log, "LUPOS_RUST_BUILD_BOOT_OK").unwrap();
    // The runner stops the VM after receiving the marker. Keeping PID 1 alive
    // avoids an intentional kernel panic being mistaken for a boot failure.
    loop {
        std::thread::sleep(Duration::from_secs(60));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn failslab_controls_are_written_and_verified_before_loading() {
        use std::cell::RefCell;
        use std::collections::BTreeMap;
        for stack in [false, true] {
            let mut initial = BTreeMap::new();
            for (name, value) in [
                ("probability", "100"),
                ("ignore-gfp-wait", "Y"),
                ("cache-filter", "Y"),
            ] {
                initial.insert(name.to_owned(), value.to_owned());
            }
            if stack {
                for (name, value) in [
                    ("require-start", 0),
                    ("require-end", usize::MAX),
                    ("reject-start", 0),
                    ("reject-end", 0),
                ] {
                    initial.insert(name.to_owned(), format!("0x{value:x}\n"));
                }
            }
            let values = RefCell::new(initial);
            let writes = RefCell::new(Vec::new());
            configure_failslab_controls(
                stack,
                |name| {
                    values
                        .borrow()
                        .get(name)
                        .cloned()
                        .ok_or(io::ErrorKind::NotFound.into())
                },
                |name, value| {
                    writes.borrow_mut().push(name.to_owned());
                    values
                        .borrow_mut()
                        .insert(name.to_owned(), String::from_utf8(value.to_vec()).unwrap());
                    Ok(())
                },
            )
            .unwrap();
            assert_eq!(
                *writes.borrow(),
                ["probability", "ignore-gfp-wait", "cache-filter"]
            );
            assert_eq!(values.borrow()["probability"], "0\n");
        }
    }

    #[test]
    fn failslab_readback_errors_missing_controls_and_filters_are_fatal() {
        for mode in 0..7 {
            let result = configure_failslab_controls(
                true,
                |name| {
                    if mode == 0 {
                        return Err(io::ErrorKind::NotFound.into());
                    }
                    let value = match name {
                        "probability" => {
                            if mode == 1 {
                                "1"
                            } else {
                                "0"
                            }
                        }
                        "ignore-gfp-wait" | "cache-filter" => "N",
                        "require-start" => {
                            if mode == 2 {
                                "0x1"
                            } else {
                                "0x0"
                            }
                        }
                        "require-end" => {
                            if mode == 3 {
                                return Err(io::ErrorKind::NotFound.into());
                            }
                            return Ok(if mode == 4 {
                                "invalid".to_owned()
                            } else {
                                format!("{:#x}", usize::MAX)
                            });
                        }
                        "reject-start" => {
                            if mode == 5 {
                                "0x1"
                            } else {
                                "0x0"
                            }
                        }
                        "reject-end" => "0x0",
                        _ => unreachable!(),
                    };
                    Ok(value.to_owned())
                },
                |_, _| {
                    if mode == 6 {
                        Err(io::ErrorKind::PermissionDenied.into())
                    } else {
                        Ok(())
                    }
                },
            );
            assert!(
                result.is_err(),
                "failure mode {mode} must not proceed to module loading"
            );
        }
    }

    #[test]
    fn partial_stack_filter_controls_never_count_as_disabled() {
        let names = ["require-start", "require-end", "reject-start", "reject-end"];
        for enabled in [false, true] {
            for present in 0..16 {
                let result = configure_failslab_controls(
                    enabled,
                    |name| {
                        if name == "probability" {
                            return Ok("0".to_owned());
                        }
                        if name == "ignore-gfp-wait" || name == "cache-filter" {
                            return Ok("N".to_owned());
                        }
                        let index = names.iter().position(|value| *value == name).unwrap();
                        if present & (1 << index) == 0 {
                            return Err(io::ErrorKind::NotFound.into());
                        }
                        Ok(format!(
                            "0x{:x}",
                            if name == "require-end" { usize::MAX } else { 0 }
                        ))
                    },
                    |_, _| Ok(()),
                );
                assert_eq!(
                    result.is_ok(),
                    if enabled { present == 15 } else { present == 0 },
                    "enabled={enabled}, present={present}"
                );
            }
        }
    }

    #[derive(Default)]
    struct Writer {
        replies: VecDeque<io::Result<usize>>,
        calls: Vec<Vec<u8>>,
    }

    impl Write for Writer {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.calls.push(bytes.to_vec());
            self.replies.pop_front().unwrap_or(Ok(bytes.len()))
        }

        fn flush(&mut self) -> io::Result<()> {
            panic!("record writes must not depend on buffered flushing")
        }
    }

    #[test]
    fn complete_markers_are_individual_info_records() {
        let mut writer = Writer::default();
        for index in 0..40 {
            write_record(&mut writer, &format!("LUPOS_RUST_PRELOAD_OK {index}")).unwrap();
        }
        write_record(&mut writer, "LUPOS_RUST_BUILD_BOOT_OK").unwrap();
        assert_eq!(writer.calls.len(), 41);
        for (index, record) in writer.calls[..40].iter().enumerate() {
            assert_eq!(
                record,
                format!("<6>LUPOS_RUST_PRELOAD_OK {index}\n").as_bytes()
            );
        }
        assert_eq!(writer.calls[40], b"<6>LUPOS_RUST_BUILD_BOOT_OK\n");
    }

    #[test]
    fn interrupted_write_retries_the_complete_record() {
        let mut writer = Writer::default();
        writer
            .replies
            .push_back(Err(io::ErrorKind::Interrupted.into()));
        write_record(&mut writer, "LUPOS_RUST_BUILD_BOOT_OK").unwrap();
        assert_eq!(
            writer.calls,
            vec![b"<6>LUPOS_RUST_BUILD_BOOT_OK\n".to_vec(); 2]
        );
    }

    #[test]
    fn short_writes_fail_without_writing_a_second_record() {
        for count in [0, 1, b"<6>LUPOS_RUST_BUILD_BOOT_OK\n".len() - 1] {
            let mut writer = Writer::default();
            writer.replies.push_back(Ok(count));
            assert_eq!(
                write_record(&mut writer, "LUPOS_RUST_BUILD_BOOT_OK")
                    .unwrap_err()
                    .kind(),
                io::ErrorKind::WriteZero
            );
            assert_eq!(writer.calls.len(), 1);
        }
    }

    #[test]
    fn other_errors_fail_without_retry_or_fallback_console_write() {
        for kind in [
            io::ErrorKind::BrokenPipe,
            io::ErrorKind::WouldBlock,
            io::ErrorKind::PermissionDenied,
        ] {
            let mut writer = Writer::default();
            writer.replies.push_back(Err(kind.into()));
            assert_eq!(
                write_record(&mut writer, "LUPOS_RUST_BUILD_BOOT_OK")
                    .unwrap_err()
                    .kind(),
                kind
            );
            assert_eq!(writer.calls.len(), 1);
        }
    }

    #[test]
    fn multi_line_or_nul_records_are_rejected_before_writing() {
        for message in ["OK\nBAD", "OK\rBAD", "OK\0BAD"] {
            let mut writer = Writer::default();
            assert_eq!(
                write_record(&mut writer, message).unwrap_err().kind(),
                io::ErrorKind::InvalidInput
            );
            assert!(writer.calls.is_empty());
        }
    }
}
