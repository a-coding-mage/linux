use std::arch::global_asm;
use std::ffi::{c_char, c_int, c_long, c_void};

// C dependencies:
// #include <unistd.h>
// #include <errno.h>
// #include <stdio.h>
// #include <stdlib.h>
// #include <linux/kexec.h>
// #include <linux/reboot.h>
// #include <sys/reboot.h>
// #include <sys/syscall.h>

const KEXEC_PRESERVE_CONTEXT: c_long = 0x00000002;
const LINUX_REBOOT_MAGIC1: c_long = 0xfee1dead_u32 as c_long;
const LINUX_REBOOT_MAGIC2: c_long = 672274793;
const LINUX_REBOOT_CMD_KEXEC: c_long = 0x45584543;
const __NR_REBOOT: c_long = 169;
const __NR_KEXEC_LOAD: c_long = 246;

#[repr(C)]
struct kexec_segment {
    buf: *mut c_void,
    bufsz: usize,
    mem: *mut c_void,
    memsz: usize,
}

global_asm!(
    "  .code64",
    "  .data",
    "purgatory_start:",
    // Trigger kexec debug exception handling
    "  int3",
    // Set load address for next time
    "  leaq purgatory_start_b(%rip), %r11",
    "  movq %r11, 8(%rsp)",
    // Back to Linux
    "  ret",
    // Same again
    "purgatory_start_b:",
    // Trigger kexec debug exception handling
    "  int3",
    // Set load address for next time
    "  leaq purgatory_start(%rip), %r11",
    "  movq %r11, 8(%rsp)",
    // Back to Linux
    "  ret",
    "purgatory_end:",
    ".previous",
);

unsafe extern "C" {
    static mut purgatory_start: c_char;
    static mut purgatory_end: c_char;

    fn syscall(num: c_long, ...) -> c_long;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn printf(format: *const c_char, ...) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    let mut segment: kexec_segment = unsafe { std::mem::zeroed() };
    let mut ret: c_long;

    segment.buf = (&raw mut purgatory_start).cast::<c_void>();
    segment.bufsz =
        (&raw mut purgatory_end as *mut c_char as isize - &raw mut purgatory_start as *mut c_char as isize)
            as usize;
    segment.mem = 0x400000usize as *mut c_void;
    segment.memsz = 0x1000;
    ret = unsafe {
        syscall(
            __NR_KEXEC_LOAD,
            0x400000 as c_long,
            1 as c_long,
            &mut segment as *mut kexec_segment,
            KEXEC_PRESERVE_CONTEXT,
        )
    };
    if ret != 0 {
        unsafe {
            perror(c"kexec_load".as_ptr());
            exit(1);
        }
    }

    ret = unsafe {
        syscall(
            __NR_REBOOT,
            LINUX_REBOOT_MAGIC1,
            LINUX_REBOOT_MAGIC2,
            LINUX_REBOOT_CMD_KEXEC,
        )
    };
    if ret != 0 {
        unsafe {
            perror(c"kexec reboot".as_ptr());
            exit(1);
        }
    }

    ret = unsafe {
        syscall(
            __NR_REBOOT,
            LINUX_REBOOT_MAGIC1,
            LINUX_REBOOT_MAGIC2,
            LINUX_REBOOT_CMD_KEXEC,
        )
    };
    if ret != 0 {
        unsafe {
            perror(c"kexec reboot".as_ptr());
            exit(1);
        }
    }
    unsafe {
        printf(c"Success\n".as_ptr());
    }
    0
}
