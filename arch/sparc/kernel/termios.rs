// SPDX-License-Identifier: GPL-2.0
// Translated from linux/termios_internal.h-dependent C source.

/*
 * c_cc characters in the termio structure.  Oh, how I love being
 * backwardly compatible.  Notice that character 4 and 5 are
 * interpreted differently depending on whether ICANON is set in
 * c_lflag.  If it's set, they are used as _VEOF and _VEOL, otherwise
 * as _VMIN and V_TIME.  This is for compatibility with OSF/1 (which
 * is compatible with sysV)...
 */
pub const _VMIN: usize = 4;
pub const _VTIME: usize = 5;

pub unsafe fn kernel_termios_to_user_termio(
    termio: *mut termio,
    termios: *mut ktermios,
) -> i32 {
    let mut v: termio = core::mem::zeroed();
    (*(&mut v)).c_iflag = (*termios).c_iflag;
    (*(&mut v)).c_oflag = (*termios).c_oflag;
    (*(&mut v)).c_cflag = (*termios).c_cflag;
    (*(&mut v)).c_lflag = (*termios).c_lflag;
    (*(&mut v)).c_line = (*termios).c_line;
    core::ptr::copy_nonoverlapping((*termios).c_cc.as_ptr(), v.c_cc.as_mut_ptr(), NCC);
    if (v.c_lflag & ICANON) == 0 {
        v.c_cc[_VMIN] = (*termios).c_cc[VMIN];
        v.c_cc[_VTIME] = (*termios).c_cc[VTIME];
    }
    copy_to_user(termio, &v, core::mem::size_of::<termio>())
}

pub unsafe fn user_termios_to_kernel_termios(
    k: *mut ktermios,
    u: *mut termios2,
) -> i32 {
    let mut err: i32;
    err = get_user(&mut (*k).c_iflag, &(*u).c_iflag);
    err |= get_user(&mut (*k).c_oflag, &(*u).c_oflag);
    err |= get_user(&mut (*k).c_cflag, &(*u).c_cflag);
    err |= get_user(&mut (*k).c_lflag, &(*u).c_lflag);
    err |= get_user(&mut (*k).c_line, &(*u).c_line);
    err |= copy_from_user((*k).c_cc.as_mut_ptr(), (*u).c_cc.as_ptr(), NCCS);
    if ((*k).c_lflag & ICANON) != 0 {
        err |= get_user(&mut (*k).c_cc[VEOF], &(*u).c_cc[VEOF]);
        err |= get_user(&mut (*k).c_cc[VEOL], &(*u).c_cc[VEOL]);
    } else {
        err |= get_user(&mut (*k).c_cc[VMIN], &(*u).c_cc[_VMIN]);
        err |= get_user(&mut (*k).c_cc[VTIME], &(*u).c_cc[_VTIME]);
    }
    err |= get_user(&mut (*k).c_ispeed, &(*u).c_ispeed);
    err |= get_user(&mut (*k).c_ospeed, &(*u).c_ospeed);
    err
}

pub unsafe fn kernel_termios_to_user_termios(
    u: *mut termios2,
    k: *mut ktermios,
) -> i32 {
    let mut err: i32;
    err = put_user((*k).c_iflag, &mut (*u).c_iflag);
    err |= put_user((*k).c_oflag, &mut (*u).c_oflag);
    err |= put_user((*k).c_cflag, &mut (*u).c_cflag);
    err |= put_user((*k).c_lflag, &mut (*u).c_lflag);
    err |= put_user((*k).c_line, &mut (*u).c_line);
    err |= copy_to_user((*u).c_cc.as_mut_ptr(), (*k).c_cc.as_ptr(), NCCS);
    if ((*k).c_lflag & ICANON) == 0 {
        err |= put_user((*k).c_cc[VMIN], &mut (*u).c_cc[_VMIN]);
        err |= put_user((*k).c_cc[VTIME], &mut (*u).c_cc[_VTIME]);
    } else {
        err |= put_user((*k).c_cc[VEOF], &mut (*u).c_cc[VEOF]);
        err |= put_user((*k).c_cc[VEOL], &mut (*u).c_cc[VEOL]);
    }
    err |= put_user((*k).c_ispeed, &mut (*u).c_ispeed);
    err |= put_user((*k).c_ospeed, &mut (*u).c_ospeed);
    err
}

pub unsafe fn user_termios_to_kernel_termios_1(
    k: *mut ktermios,
    u: *mut termios,
) -> i32 {
    let mut err: i32;
    err = get_user(&mut (*k).c_iflag, &(*u).c_iflag);
    err |= get_user(&mut (*k).c_oflag, &(*u).c_oflag);
    err |= get_user(&mut (*k).c_cflag, &(*u).c_cflag);
    err |= get_user(&mut (*k).c_lflag, &(*u).c_lflag);
    err |= get_user(&mut (*k).c_line, &(*u).c_line);
    err |= copy_from_user((*k).c_cc.as_mut_ptr(), (*u).c_cc.as_ptr(), NCCS);
    if ((*k).c_lflag & ICANON) != 0 {
        err |= get_user(&mut (*k).c_cc[VEOF], &(*u).c_cc[VEOF]);
        err |= get_user(&mut (*k).c_cc[VEOL], &(*u).c_cc[VEOL]);
    } else {
        err |= get_user(&mut (*k).c_cc[VMIN], &(*u).c_cc[_VMIN]);
        err |= get_user(&mut (*k).c_cc[VTIME], &(*u).c_cc[_VTIME]);
    }
    err
}

pub unsafe fn kernel_termios_to_user_termios_1(
    u: *mut termios,
    k: *mut ktermios,
) -> i32 {
    let mut err: i32;
    err = put_user((*k).c_iflag, &mut (*u).c_iflag);
    err |= put_user((*k).c_oflag, &mut (*u).c_oflag);
    err |= put_user((*k).c_cflag, &mut (*u).c_cflag);
    err |= put_user((*k).c_lflag, &mut (*u).c_lflag);
    err |= put_user((*k).c_line, &mut (*u).c_line);
    err |= copy_to_user((*u).c_cc.as_mut_ptr(), (*k).c_cc.as_ptr(), NCCS);
    if ((*k).c_lflag & ICANON) == 0 {
        err |= put_user((*k).c_cc[VMIN], &mut (*u).c_cc[_VMIN]);
        err |= put_user((*k).c_cc[VTIME], &mut (*u).c_cc[_VTIME]);
    } else {
        err |= put_user((*k).c_cc[VEOF], &mut (*u).c_cc[VEOF]);
        err |= put_user((*k).c_cc[VEOL], &mut (*u).c_cc[VEOL]);
    }
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
