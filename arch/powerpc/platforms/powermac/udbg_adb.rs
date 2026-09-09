// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/string.h, linux/kernel.h, linux/errno.h, linux/bitops.h,
// linux/ptrace.h, linux/adb.h, linux/pmu.h, linux/cuda.h, linux/of.h,
// asm/machdep.h, asm/io.h, asm/page.h, asm/xmon.h, asm/bootx.h, asm/errno.h,
// asm/pmac_feature.h, asm/processor.h, asm/delay.h, asm/btext.h, asm/time.h,
// asm/udbg.h

/*
 * This implementation is "special", it can "patch" the current
 * udbg implementation and work on top of it. It must thus be
 * initialized last
 */

static mut udbg_adb_old_putc: Option<unsafe extern "C" fn(c: core::ffi::c_char)> = None;
static mut udbg_adb_old_getc: Option<unsafe extern "C" fn() -> core::ffi::c_int> = None;
static mut udbg_adb_old_getc_poll: Option<unsafe extern "C" fn() -> core::ffi::c_int> = None;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum InputAdb {
    InputAdbNone,
    InputAdbPmu,
    InputAdbCuda,
}

static mut input_type: InputAdb = InputAdb::InputAdbNone;

static mut xmon_wants_key: core::ffi::c_int = 0;
static mut xmon_adb_keycode: core::ffi::c_int = 0;

#[inline]
unsafe fn udbg_adb_poll() {
    // CONFIG_ADB_PMU condition from the C source.
    #[cfg(feature = "CONFIG_ADB_PMU")]
    if input_type == InputAdb::InputAdbPmu {
        pmu_poll_adb();
    }
    // CONFIG_ADB_CUDA condition from the C source.
    #[cfg(feature = "CONFIG_ADB_CUDA")]
    if input_type == InputAdb::InputAdbCuda {
        cuda_poll();
    }
}

// The following declarations and definitions are present when CONFIG_BOOTX_TEXT is enabled.
#[cfg(feature = "CONFIG_BOOTX_TEXT")]
static mut udbg_adb_use_btext: core::ffi::c_int = 0;
#[cfg(feature = "CONFIG_BOOTX_TEXT")]
static mut xmon_adb_shiftstate: core::ffi::c_int = 0;

#[cfg(feature = "CONFIG_BOOTX_TEXT")]
static xmon_keytab: [u8; 128] = *b"asdfhgzxcv\0bqwerty123465=97-80]o u[ip\rlj'k;\\,/nm.\t `\x7f\0\x1b\0\0\0\0\0\0\0\0\0\0\0.\0*\0+\0\0\0\0\0/\r\0-\0\0\0123456789\0\0\0";
#[cfg(feature = "CONFIG_BOOTX_TEXT")]
static xmon_shift_keytab: [u8; 128] = *b"ASDFHGZXCV\0BQWERYT!@#$^%+(&_*)}OU{IP\rLJ\"K:|<?NM>\t ~\x7f\0\x1b\0\0\0\0\0\0\0\0\0\0\0.\0*\0+\0\0\0\0\0/\r\0-\0\0\0123456789\0\0\0";

#[cfg(feature = "CONFIG_BOOTX_TEXT")]
unsafe fn udbg_adb_local_getc() -> core::ffi::c_int {
    let (mut k, mut t, mut on): (core::ffi::c_int, core::ffi::c_int, core::ffi::c_int);
    xmon_wants_key = 1;
    loop {
        xmon_adb_keycode = -1;
        t = 0;
        on = 0;
        k = -1;
        loop {
            t -= 1;
            if t < 0 {
                on = 1 - on;
                btext_drawchar(if on != 0 { 0xdb } else { 0x20 });
                btext_drawchar(b'\b' as core::ffi::c_int);
                t = 200000;
            }
            udbg_adb_poll();
            if let Some(f) = udbg_adb_old_getc_poll { k = f(); }
            if !(k == -1 && xmon_adb_keycode == -1) { break; }
        }
        if on != 0 { btext_drawstring(b" \0\x08\0".as_ptr() as *const core::ffi::c_char); }
        if k != -1 { return k; }
        k = xmon_adb_keycode;
        if (k & 0x7f) == 0x38 || (k & 0x7f) == 0x7b {
            xmon_adb_shiftstate = if (k & 0x80) == 0 { 1 } else { 0 };
            continue;
        }
        if k >= 0x80 { continue; }
        k = if xmon_adb_shiftstate != 0 { xmon_shift_keytab[k as usize] } else { xmon_keytab[k as usize] } as core::ffi::c_int;
        if k != 0 { break; }
    }
    xmon_wants_key = 0;
    k
}

unsafe fn udbg_adb_getc() -> core::ffi::c_int {
    #[cfg(feature = "CONFIG_BOOTX_TEXT")]
    if udbg_adb_use_btext != 0 && input_type != InputAdb::InputAdbNone { return udbg_adb_local_getc(); }
    if let Some(f) = udbg_adb_old_getc { return f(); }
    -1
}

unsafe fn udbg_adb_getc_poll() -> core::ffi::c_int {
    udbg_adb_poll();
    if let Some(f) = udbg_adb_old_getc_poll { return f(); }
    -1
}

unsafe fn udbg_adb_putc(c: core::ffi::c_char) {
    #[cfg(feature = "CONFIG_BOOTX_TEXT")]
    if udbg_adb_use_btext != 0 { btext_drawchar(c as core::ffi::c_int); }
    if let Some(f) = udbg_adb_old_putc { f(c); }
}

pub unsafe extern "C" fn udbg_adb_init_early() {
    #[cfg(feature = "CONFIG_BOOTX_TEXT")]
    if btext_find_display(1) == 0 {
        udbg_adb_use_btext = 1;
        udbg_putc = Some(udbg_adb_putc);
    }
}

pub unsafe extern "C" fn udbg_adb_init(force_btext: core::ffi::c_int) -> core::ffi::c_int {
    let mut np: *mut device_node;
    udbg_adb_old_putc = udbg_putc;
    udbg_adb_old_getc = udbg_getc;
    udbg_adb_old_getc_poll = udbg_getc_poll;
    if udbg_adb_old_putc.map(|f| f as usize) == Some(udbg_adb_putc as usize) { udbg_adb_old_putc = None; }
    #[cfg(feature = "CONFIG_BOOTX_TEXT")]
    if udbg_adb_old_putc.map(|f| f as usize) == Some(btext_drawchar as usize) { udbg_adb_old_putc = None; }
    udbg_putc = Some(udbg_adb_putc);
    udbg_getc = Some(udbg_adb_getc);
    udbg_getc_poll = Some(udbg_adb_getc_poll);
    #[cfg(feature = "CONFIG_BOOTX_TEXT")]
    if btext_find_display(force_btext) == 0 { udbg_adb_use_btext = 1; }
    // Find a keyboard whose parent is of type "adb".
    np = core::ptr::null_mut();
    for_each_node_by_name!(np, "keyboard", {
        let parent = of_get_parent(np);
        let found = of_node_is_type(parent, "adb");
        of_node_put(parent);
        if found != 0 { break; }
    });
    if np.is_null() { return -ENODEV; }
    of_node_put(np);
    #[cfg(feature = "CONFIG_ADB_PMU")]
    if find_via_pmu() != 0 { input_type = InputAdb::InputAdbPmu; }
    #[cfg(feature = "CONFIG_ADB_CUDA")]
    if find_via_cuda() != 0 { input_type = InputAdb::InputAdbCuda; }
    if input_type == InputAdb::InputAdbNone { return -ENODEV; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
