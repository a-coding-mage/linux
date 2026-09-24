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
