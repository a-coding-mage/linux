// SPDX-License-Identifier: LGPL-2.1
/*
 * trace/beauty/ioctl.c
 *
 *  Copyright (C) 2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

type size_t = usize;

#[repr(C)]
pub struct strarray {
    pub nr_entries: c_int,
    pub entries: *const *const c_char,
}

#[repr(C)]
pub struct file {
    pub dev_maj: c_int,
}

#[repr(C)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub thread: *mut c_void,
    pub show_string_prefix: bool,
}

const _IOC_NRBITS: c_int = 8;
const _IOC_TYPEBITS: c_int = 8;
const _IOC_SIZEBITS: c_int = 14;
const _IOC_DIRBITS: c_int = 2;

const _IOC_NRSHIFT: c_int = 0;
const _IOC_TYPESHIFT: c_int = _IOC_NRSHIFT + _IOC_NRBITS;
const _IOC_SIZESHIFT: c_int = _IOC_TYPESHIFT + _IOC_TYPEBITS;
const _IOC_DIRSHIFT: c_int = _IOC_SIZESHIFT + _IOC_SIZEBITS;

const _IOC_NRMASK: c_ulong = ((1u64 << _IOC_NRBITS) - 1) as c_ulong;
const _IOC_TYPEMASK: c_ulong = ((1u64 << _IOC_TYPEBITS) - 1) as c_ulong;
const _IOC_SIZEMASK: c_ulong = ((1u64 << _IOC_SIZEBITS) - 1) as c_ulong;
const _IOC_DIRMASK: c_ulong = ((1u64 << _IOC_DIRBITS) - 1) as c_ulong;

const _IOC_NONE: c_int = 0;
const _IOC_WRITE: c_int = 1;
const _IOC_READ: c_int = 2;

const USB_DEVICE_MAJOR: c_int = 189;

#[inline]
const fn _IOC_DIR(nr: c_ulong) -> c_int {
    ((nr >> _IOC_DIRSHIFT) & _IOC_DIRMASK) as c_int
}

#[inline]
const fn _IOC_TYPE(nr: c_ulong) -> c_int {
    ((nr >> _IOC_TYPESHIFT) & _IOC_TYPEMASK) as c_int
}

#[inline]
const fn _IOC_NR(nr: c_ulong) -> c_int {
    ((nr >> _IOC_NRSHIFT) & _IOC_NRMASK) as c_int
}

#[inline]
const fn _IOC_SIZE(nr: c_ulong) -> c_int {
    ((nr >> _IOC_SIZESHIFT) & _IOC_SIZEMASK) as c_int
}

unsafe extern "C" {
    static TCGETS: c_ulong;
    static TIOCSBRK: c_ulong;
    static TIOCGEXCL: c_ulong;
    static FIONCLEX: c_ulong;

    static strarray__drm_ioctl_cmds: strarray;
    static strarray__sndrv_pcm_ioctl_cmds: strarray;
    static strarray__sndrv_ctl_ioctl_cmds: strarray;
    static strarray__kvm_ioctl_cmds: strarray;
    static strarray__vhost_virtio_ioctl_cmds: strarray;
    static strarray__vhost_virtio_ioctl_read_cmds: strarray;
    static strarray__perf_ioctl_cmds: strarray;
    static strarray__usbdevfs_ioctl_cmds: strarray;

    fn scnprintf(bf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn syscall_arg__val(arg: *mut syscall_arg, idx: c_int) -> c_ulong;
    fn thread__files_entry(thread: *mut c_void, fd: c_int) -> *mut file;
}

static IOCTL_TTY_CMD_1: [&[u8]; 37] = [
    b"TCGETS\0",
    b"TCSETS\0",
    b"TCSETSW\0",
    b"TCSETSF\0",
    b"TCGETA\0",
    b"TCSETA\0",
    b"TCSETAW\0",
    b"TCSETAF\0",
    b"TCSBRK\0",
    b"TCXONC\0",
    b"TCFLSH\0",
    b"TIOCEXCL\0",
    b"TIOCNXCL\0",
    b"TIOCSCTTY\0",
    b"TIOCGPGRP\0",
    b"TIOCSPGRP\0",
    b"TIOCOUTQ\0",
    b"TIOCSTI\0",
    b"TIOCGWINSZ\0",
    b"TIOCSWINSZ\0",
    b"TIOCMGET\0",
    b"TIOCMBIS\0",
    b"TIOCMBIC\0",
    b"TIOCMSET\0",
    b"TIOCGSOFTCAR\0",
    b"TIOCSSOFTCAR\0",
    b"FIONREAD\0",
    b"TIOCLINUX\0",
    b"TIOCCONS\0",
    b"TIOCGSERIAL\0",
    b"TIOCSSERIAL\0",
    b"TIOCPKT\0",
    b"FIONBIO\0",
    b"TIOCNOTTY\0",
    b"TIOCSETD\0",
    b"TIOCGETD\0",
    b"TCSBRKP\0",
];

static IOCTL_TTY_CMD_2: [&[u8]; 19] = [
    b"TIOCSBRK\0",
    b"TIOCCBRK\0",
    b"TIOCGSID\0",
    b"TCGETS2\0",
    b"TCSETS2\0",
    b"TCSETSW2\0",
    b"TCSETSF2\0",
    b"TIOCGRS48\0",
    b"TIOCSRS485\0",
    b"TIOCGPTN\0",
    b"TIOCSPTLCK\0",
    b"TIOCGDEV\0",
    b"TCSETX\0",
    b"TCSETXF\0",
    b"TCSETXW\0",
    b"TIOCSIG\0",
    b"TIOCVHANGUP\0",
    b"TIOCGPKT\0",
    b"TIOCGPTLCK\0",
];

static IOCTL_TTY_CMD_3: [&[u8]; 3] = [b"TIOCGEXCL\0", b"TIOCGPTPEER\0", b"TIOCGISO7816\0"];
static IOCTL_TTY_CMD_4: [&[u8]; 14] = [
    b"FIONCLEX\0",
    b"FIOCLEX\0",
    b"FIOASYNC\0",
    b"TIOCSERCONFIG\0",
    b"TIOCSERGWILD\0",
    b"TIOCSERSWILD\0",
    b"TIOCGLCKTRMIOS\0",
    b"TIOCSLCKTRMIOS\0",
    b"TIOCSERGSTRUCT\0",
    b"TIOCSERGETLSR\0",
    b"TIOCSERGETMULTI\0",
    b"TIOCSERSETMULTI\0",
    b"TIOCMIWAIT\0",
    b"TIOCGICOUNT\0",
];

unsafe fn ioctl_tty_cmd_entry(nr: c_int) -> *const c_char {
    let tcgets = _IOC_NR(TCGETS);
    let tiocsbrk = _IOC_NR(TIOCSBRK);
    let tiocgexcl = _IOC_NR(TIOCGEXCL);
    let fionclex = _IOC_NR(FIONCLEX);

    if nr >= tcgets && nr < tcgets + IOCTL_TTY_CMD_1.len() as c_int {
        return IOCTL_TTY_CMD_1[(nr - tcgets) as usize].as_ptr() as *const c_char;
    }
    if nr >= tiocsbrk && nr < tiocsbrk + IOCTL_TTY_CMD_2.len() as c_int {
        return IOCTL_TTY_CMD_2[(nr - tiocsbrk) as usize].as_ptr() as *const c_char;
    }
    if nr >= tiocgexcl && nr < tiocgexcl + IOCTL_TTY_CMD_3.len() as c_int {
        return IOCTL_TTY_CMD_3[(nr - tiocgexcl) as usize].as_ptr() as *const c_char;
    }
    if nr >= fionclex && nr < fionclex + IOCTL_TTY_CMD_4.len() as c_int {
        return IOCTL_TTY_CMD_4[(nr - fionclex) as usize].as_ptr() as *const c_char;
    }

    core::ptr::null()
}

unsafe fn strarray_entry(s: *const strarray, nr: c_int) -> *const c_char {
    if nr >= 0 && nr < (*s).nr_entries {
        let entry = *(*s).entries.add(nr as usize);
        if !entry.is_null() {
            return entry;
        }
    }
    core::ptr::null()
}

unsafe fn ioctl__scnprintf_tty_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: size_t) -> size_t {
    let entry = ioctl_tty_cmd_entry(nr);

    if !entry.is_null() {
        return scnprintf(bf, size, c"%s".as_ptr(), entry) as size_t;
    }

    scnprintf(bf, size, c"(%#x, %#x, %#x)".as_ptr(), 'T' as c_int, nr, dir) as size_t
}

unsafe fn ioctl__scnprintf_drm_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: size_t) -> size_t {
    /* trace/beauty/generated/ioctl/drm_ioctl_array.c supplies strarray__drm_ioctl_cmds. */
    let entry = strarray_entry(&raw const strarray__drm_ioctl_cmds, nr);

    if !entry.is_null() {
        return scnprintf(bf, size, c"DRM_%s".as_ptr(), entry) as size_t;
    }

    scnprintf(bf, size, c"(%#x, %#x, %#x)".as_ptr(), 'd' as c_int, nr, dir) as size_t
}

unsafe fn ioctl__scnprintf_sndrv_pcm_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: size_t) -> size_t {
    /* trace/beauty/generated/ioctl/sndrv_pcm_ioctl_array.c supplies strarray__sndrv_pcm_ioctl_cmds. */
    let entry = strarray_entry(&raw const strarray__sndrv_pcm_ioctl_cmds, nr);

    if !entry.is_null() {
        return scnprintf(bf, size, c"SNDRV_PCM_%s".as_ptr(), entry) as size_t;
    }

    scnprintf(bf, size, c"(%#x, %#x, %#x)".as_ptr(), 'A' as c_int, nr, dir) as size_t
}

unsafe fn ioctl__scnprintf_sndrv_ctl_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: size_t) -> size_t {
    /* trace/beauty/generated/ioctl/sndrv_ctl_ioctl_array.c supplies strarray__sndrv_ctl_ioctl_cmds. */
    let entry = strarray_entry(&raw const strarray__sndrv_ctl_ioctl_cmds, nr);

    if !entry.is_null() {
        return scnprintf(bf, size, c"SNDRV_CTL_%s".as_ptr(), entry) as size_t;
    }

    scnprintf(bf, size, c"(%#x, %#x, %#x)".as_ptr(), 'U' as c_int, nr, dir) as size_t
}

unsafe fn ioctl__scnprintf_kvm_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: size_t) -> size_t {
    /* trace/beauty/generated/ioctl/kvm_ioctl_array.c supplies strarray__kvm_ioctl_cmds. */
    let entry = strarray_entry(&raw const strarray__kvm_ioctl_cmds, nr);

    if !entry.is_null() {
        return scnprintf(bf, size, c"KVM_%s".as_ptr(), entry) as size_t;
    }

    scnprintf(bf, size, c"(%#x, %#x, %#x)".as_ptr(), 0xAE as c_int, nr, dir) as size_t
}

unsafe fn ioctl__scnprintf_vhost_virtio_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: size_t) -> size_t {
    /* trace/beauty/generated/ioctl/vhost_virtio_ioctl_array.c supplies both vhost strarrays. */
    let s = if (dir & _IOC_READ) != 0 {
        &raw const strarray__vhost_virtio_ioctl_read_cmds
    } else {
        &raw const strarray__vhost_virtio_ioctl_cmds
    };
    let entry = strarray_entry(s, nr);

    if !entry.is_null() {
        return scnprintf(bf, size, c"VHOST_%s".as_ptr(), entry) as size_t;
    }

    scnprintf(bf, size, c"(%#x, %#x, %#x)".as_ptr(), 0xAF as c_int, nr, dir) as size_t
}

unsafe fn ioctl__scnprintf_perf_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: size_t) -> size_t {
    /* trace/beauty/generated/ioctl/perf_ioctl_array.c supplies strarray__perf_ioctl_cmds. */
    let entry = strarray_entry(&raw const strarray__perf_ioctl_cmds, nr);

    if !entry.is_null() {
        return scnprintf(bf, size, c"PERF_%s".as_ptr(), entry) as size_t;
    }

    scnprintf(bf, size, c"(%#x, %#x, %#x)".as_ptr(), 0xAE as c_int, nr, dir) as size_t
}

unsafe fn ioctl__scnprintf_usbdevfs_cmd(nr: c_int, dir: c_int, bf: *mut c_char, size: size_t) -> size_t {
    /* trace/beauty/generated/ioctl/usbdevfs_ioctl_array.c supplies strarray__usbdevfs_ioctl_cmds. */
    let entry = strarray_entry(&raw const strarray__usbdevfs_ioctl_cmds, nr);

    if !entry.is_null() {
        return scnprintf(bf, size, c"USBDEVFS_%s".as_ptr(), entry) as size_t;
    }

    scnprintf(bf, size, c"(%c, %#x, %#x)".as_ptr(), 'U' as c_int, nr, dir) as size_t
}

#[repr(C)]
struct ioctl_type {
    type_: c_int,
    scnprintf: Option<unsafe fn(c_int, c_int, *mut c_char, size_t) -> size_t>,
}

static IOCTL_TYPES: [ioctl_type; 140] = {
    let mut ioctl_types = [ioctl_type { type_: 0, scnprintf: None }; 140];
    ioctl_types[0] = ioctl_type { type_: '$' as c_int, scnprintf: Some(ioctl__scnprintf_perf_cmd) };
    ioctl_types[('A' as usize) - ('$' as usize)] = ioctl_type { type_: 'A' as c_int, scnprintf: Some(ioctl__scnprintf_sndrv_pcm_cmd) };
    ioctl_types[('T' as usize) - ('$' as usize)] = ioctl_type { type_: 'T' as c_int, scnprintf: Some(ioctl__scnprintf_tty_cmd) };
    ioctl_types[('U' as usize) - ('$' as usize)] = ioctl_type { type_: 'U' as c_int, scnprintf: Some(ioctl__scnprintf_sndrv_ctl_cmd) };
    ioctl_types[('d' as usize) - ('$' as usize)] = ioctl_type { type_: 'd' as c_int, scnprintf: Some(ioctl__scnprintf_drm_cmd) };
    ioctl_types[(0xAE as usize) - ('$' as usize)] = ioctl_type { type_: 0xAE, scnprintf: Some(ioctl__scnprintf_kvm_cmd) };
    ioctl_types[(0xAF as usize) - ('$' as usize)] = ioctl_type { type_: 0xAF, scnprintf: Some(ioctl__scnprintf_vhost_virtio_cmd) };
    ioctl_types
};

unsafe fn ioctl__scnprintf_cmd(cmd: c_ulong, bf: *mut c_char, size: size_t, show_prefix: bool) -> size_t {
    let prefix = c"_IOC_".as_ptr();
    let dir = _IOC_DIR(cmd);
    let type_ = _IOC_TYPE(cmd);
    let nr = _IOC_NR(cmd);
    let sz = _IOC_SIZE(cmd);
    let mut printed: c_int = 0;
    let nr_types = IOCTL_TYPES.len() as c_int;

    if type_ >= IOCTL_TYPES[0].type_ && type_ <= IOCTL_TYPES[(nr_types - 1) as usize].type_ {
        let index = type_ - IOCTL_TYPES[0].type_;

        if let Some(scnprintf_fn) = IOCTL_TYPES[index as usize].scnprintf {
            return scnprintf_fn(nr, dir, bf, size);
        }
    }

    printed += scnprintf(bf.add(printed as usize), size.wrapping_sub(printed as usize), c"%c".as_ptr(), '(' as c_int);

    if dir == _IOC_NONE {
        printed += scnprintf(
            bf.add(printed as usize),
            size.wrapping_sub(printed as usize),
            c"%s%s".as_ptr(),
            if show_prefix { prefix } else { c"".as_ptr() },
            c"NONE".as_ptr(),
        );
    } else {
        if (dir & _IOC_READ) != 0 {
            printed += scnprintf(
                bf.add(printed as usize),
                size.wrapping_sub(printed as usize),
                c"%s%s".as_ptr(),
                if show_prefix { prefix } else { c"".as_ptr() },
                c"READ".as_ptr(),
            );
        }
        if (dir & _IOC_WRITE) != 0 {
            printed += scnprintf(
                bf.add(printed as usize),
                size.wrapping_sub(printed as usize),
                c"%s%s%s".as_ptr(),
                if (dir & _IOC_READ) != 0 { c"|".as_ptr() } else { c"".as_ptr() },
                if show_prefix { prefix } else { c"".as_ptr() },
                c"WRITE".as_ptr(),
            );
        }
    }

    (printed
        + scnprintf(
            bf.add(printed as usize),
            size.wrapping_sub(printed as usize),
            c", %#x, %#x, %#x)".as_ptr(),
            type_,
            nr,
            sz,
        )) as size_t
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_ioctl_cmd(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let cmd = (*arg).val;
    let fd = syscall_arg__val(arg, 0) as c_int;
    let file = thread__files_entry((*arg).thread, fd);

    if !file.is_null() {
        if (*file).dev_maj == USB_DEVICE_MAJOR {
            return ioctl__scnprintf_usbdevfs_cmd(_IOC_NR(cmd), _IOC_DIR(cmd), bf, size);
        }
    }

    ioctl__scnprintf_cmd(cmd, bf, size, (*arg).show_string_prefix)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
