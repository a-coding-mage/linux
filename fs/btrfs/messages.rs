// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by other translation units are intentionally left external.

#[cfg(feature = "config_printk")]
const STATE_STRING_PREFACE: &[u8] = b" state ";

#[cfg(feature = "config_printk")]
const STATE_STRING_BUF_LEN: usize = STATE_STRING_PREFACE.len() + BTRFS_FS_STATE_COUNT + 1;

#[cfg(feature = "config_printk")]
static FS_STATE_CHARS: [u8; BTRFS_FS_STATE_COUNT] = [
    0, // designated entries supplied by the C enum layout
];

#[cfg(feature = "config_printk")]
unsafe fn btrfs_state_to_string(info: *const btrfs_fs_info, buf: *mut i8) {
    let mut states_printed = false;
    let fs_state = core::ptr::read_volatile(core::ptr::addr_of!((*info).fs_state));
    let mut curr = buf as *mut u8;

    core::ptr::copy_nonoverlapping(
        STATE_STRING_PREFACE.as_ptr(),
        curr,
        STATE_STRING_PREFACE.len(),
    );
    curr = curr.add(STATE_STRING_PREFACE.len() - 1);

    if BTRFS_FS_ERROR(info) {
        *curr = b'E';
        curr = curr.add(1);
        states_printed = true;
    }

    let mut bit = 0usize;
    while bit < core::mem::size_of::<usize>() * 8 {
        if (fs_state & (1usize << bit)) != 0 {
            if bit >= BTRFS_FS_STATE_COUNT {
                WARN_ON_ONCE(true);
            } else if FS_STATE_CHARS[bit] != 0 {
                *curr = FS_STATE_CHARS[bit];
                curr = curr.add(1);
                states_printed = true;
            }
        }
        bit += 1;
    }

    if !states_printed {
        curr = buf as *mut u8;
    }
    *curr = 0;
}

pub unsafe fn btrfs_decode_error(error: i32) -> *const i8 {
    match error {
        -ENOENT => b"No such entry\0".as_ptr() as *const i8,
        -EIO => b"IO failure\0".as_ptr() as *const i8,
        -ENOMEM => b"Out of memory\0".as_ptr() as *const i8,
        -EEXIST => b"Object already exists\0".as_ptr() as *const i8,
        -ENOSPC => b"No space left\0".as_ptr() as *const i8,
        -EROFS => b"Readonly filesystem\0".as_ptr() as *const i8,
        -EOPNOTSUPP => b"Operation not supported\0".as_ptr() as *const i8,
        -EUCLEAN => b"Filesystem corrupted\0".as_ptr() as *const i8,
        -EDQUOT => b"Quota exceeded\0".as_ptr() as *const i8,
        _ => b"unknown\0".as_ptr() as *const i8,
    }
}

pub unsafe extern "C" fn __btrfs_handle_fs_error(
    fs_info: *mut btrfs_fs_info,
    function: *const i8,
    line: u32,
    error: i32,
    fmt: *const i8,
    mut args: ...,
) {
    let sb = (*fs_info).sb;

    if error == -EROFS && sb_rdonly(sb) {
        return;
    }

    #[cfg(feature = "config_printk")]
    {
        let mut statestr = [0i8; STATE_STRING_BUF_LEN];
        let _errstr = btrfs_decode_error(error);
        btrfs_state_to_string(fs_info, statestr.as_mut_ptr());
        if !fmt.is_null() {
            let mut vaf = va_format { fmt, va: &mut args };
            pr_crit(/* format and arguments supplied externally */ &mut vaf);
        } else {
            pr_crit(/* format and arguments supplied externally */);
        }
    }

    core::ptr::write_volatile(core::ptr::addr_of_mut!((*fs_info).fs_error), error);
    if ((*sb).s_flags & SB_BORN) == 0 || sb_rdonly(sb) {
        return;
    }
    btrfs_discard_stop(fs_info);
    btrfs_set_sb_rdonly(sb);
    btrfs_info(fs_info, b"forced readonly\0".as_ptr() as *const i8);
}

#[cfg(feature = "config_printk")]
static LOGTYPES: [&[u8]; 8] = [
    b"emergency\0", b"alert\0", b"critical\0", b"error\0",
    b"warning\0", b"notice\0", b"info\0", b"debug\0",
];

#[cfg(feature = "config_printk")]
static mut PRINTK_LIMITS: [ratelimit_state; 8] = [
    ratelimit_state::INIT(), ratelimit_state::INIT(), ratelimit_state::INIT(),
    ratelimit_state::INIT(), ratelimit_state::INIT(), ratelimit_state::INIT(),
    ratelimit_state::INIT(), ratelimit_state::INIT(),
];

#[cfg(feature = "config_printk")]
pub unsafe extern "C" fn _btrfs_printk(
    fs_info: *const btrfs_fs_info,
    level: u32,
    fmt: *const i8,
    mut args: ...,
) {
    let ratelimit = &mut PRINTK_LIMITS[level as usize];
    if IS_ENABLED(CONFIG_BTRFS_DEBUG) || __ratelimit(ratelimit) {
        let mut vaf = va_format { fmt, va: &mut args };
        if !fs_info.is_null() {
            let mut statestr = [0i8; STATE_STRING_BUF_LEN];
            btrfs_state_to_string(fs_info, statestr.as_mut_ptr());
            _printk(&mut vaf);
        } else {
            _printk(&mut vaf);
        }
    }
}

#[cfg(target_pointer_width = "32")]
pub unsafe extern "C" fn btrfs_warn_32bit_limit(fs_info: *mut btrfs_fs_info) {
    if !test_and_set_bit(BTRFS_FS_32BIT_WARN, &mut (*fs_info).flags) {
        btrfs_warn(fs_info, b"reaching 32bit limit for logical addresses\0".as_ptr() as *const i8);
        btrfs_warn(fs_info, b"due to page cache limit on 32bit systems, btrfs can't access metadata at or beyond %lluT\0".as_ptr() as *const i8, BTRFS_32BIT_MAX_FILE_SIZE >> 40);
        btrfs_warn(fs_info, b"please consider upgrading to 64bit kernel/hardware\0".as_ptr() as *const i8);
    }
}

#[cfg(target_pointer_width = "32")]
pub unsafe extern "C" fn btrfs_err_32bit_limit(fs_info: *mut btrfs_fs_info) {
    if !test_and_set_bit(BTRFS_FS_32BIT_ERROR, &mut (*fs_info).flags) {
        btrfs_err(fs_info, b"reached 32bit limit for logical addresses\0".as_ptr() as *const i8);
        btrfs_err(fs_info, b"due to page cache limit on 32bit systems, metadata beyond %lluT can't be accessed\0".as_ptr() as *const i8, BTRFS_32BIT_MAX_FILE_SIZE >> 40);
        btrfs_err(fs_info, b"please consider upgrading to 64bit kernel/hardware\0".as_ptr() as *const i8);
    }
}

pub unsafe extern "C" fn __btrfs_panic(
    fs_info: *const btrfs_fs_info,
    function: *const i8,
    line: u32,
    error: i32,
    fmt: *const i8,
    mut args: ...,
) {
    let mut s_id = b"<unknown>\0".as_ptr() as *mut i8;
    let mut vaf = va_format { fmt, va: &mut args };
    if !fs_info.is_null() {
        s_id = (*(*fs_info).sb).s_id.as_mut_ptr();
    }
    if !fs_info.is_null() && btrfs_test_opt(fs_info, PANIC_ON_FATAL_ERROR) {
        panic(&mut vaf, error);
    }
    btrfs_crit(fs_info, &mut vaf, function, line, error);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
