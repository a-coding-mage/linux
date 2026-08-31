// SPDX-License-Identifier: GPL-2.0
/*
 * Boot config tool for initrd image
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_void};
use std::ptr;

type size_t = usize;
type ssize_t = isize;
type off_t = c_long;

const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const O_APPEND: c_int = 0o2000;
const SEEK_SET: c_int = 0;
const SEEK_END: c_int = 2;

const ENOMEM: c_int = 12;
const E2BIG: c_int = 7;
const EINVAL: c_int = 22;
const ENOSPC: c_int = 28;

const BOOTCONFIG_MAGIC: &[u8; BOOTCONFIG_MAGIC_LEN] = b"#BOOTCONFIG\n";
const BOOTCONFIG_MAGIC_LEN: usize = 12;
const BOOTCONFIG_ALIGN: usize = 4;
const BOOTCONFIG_ALIGN_MASK: usize = BOOTCONFIG_ALIGN - 1;
const XBC_KEYLEN_MAX: usize = 256;
const XBC_DATA_MAX: i64 = 32 * 1024;

/* Bootconfig footer is [size][csum][BOOTCONFIG_MAGIC]. */
const BOOTCONFIG_FOOTER_SIZE: usize = mem::size_of::<u32>() * 2 + BOOTCONFIG_MAGIC_LEN;

#[repr(C)]
pub struct xbc_node {
    _data: [u8; 0],
    next: *mut xbc_node,
}

#[repr(C)]
struct stat {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: c_uint,
    st_uid: c_uint,
    st_gid: c_uint,
    __pad0: c_int,
    st_rdev: u64,
    st_size: off_t,
    st_blksize: off_t,
    st_blocks: off_t,
    st_atime: off_t,
    st_atime_nsec: off_t,
    st_mtime: off_t,
    st_mtime_nsec: off_t,
    st_ctime: off_t,
    st_ctime_nsec: off_t,
    __unused: [c_long; 3],
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut stderr: *mut c_void;
    static mut stdout: *mut c_void;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fputs(s: *const c_char, stream: *mut c_void) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn ftruncate(fd: c_int, length: off_t) -> c_int;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;

    fn xbc_root_node() -> *mut xbc_node;
    fn xbc_node_is_key(node: *mut xbc_node) -> bool;
    fn xbc_node_is_value(node: *mut xbc_node) -> bool;
    fn xbc_node_is_array(node: *mut xbc_node) -> bool;
    fn xbc_node_get_child(node: *mut xbc_node) -> *mut xbc_node;
    fn xbc_node_get_next(node: *mut xbc_node) -> *mut xbc_node;
    fn xbc_node_get_parent(node: *mut xbc_node) -> *mut xbc_node;
    fn xbc_node_get_data(node: *mut xbc_node) -> *const c_char;
    fn xbc_node_compose_key(node: *mut xbc_node, buf: *mut c_char, size: size_t) -> c_int;
    fn xbc_calc_checksum(data: *const c_char, size: u32) -> u32;
    fn xbc_init(data: *mut c_char, size: u32, errmsg: *mut *const c_char, errpos: *mut c_int) -> c_int;
    fn xbc_find_node(key: *const c_char) -> *mut xbc_node;
    fn xbc_snprint_cmdline(buf: *mut c_char, size: size_t, root: *mut xbc_node) -> c_int;
    fn xbc_get_info(nodes: *mut c_int, size: *mut c_int);
    fn xbc_exit();

    /* Rust translation of xbc_array_for_each_value()/xbc_for_each_key_value()
     * requires iterator entry points supplied by the bootconfig dependency. */
    fn xbc_array_value_next(node: *mut xbc_node, prev: *const c_char) -> *const c_char;
    fn xbc_key_value_next(leaf: *mut *mut xbc_node, prev: *const c_char) -> *const c_char;
}

unsafe fn pr_errno(msg: *const c_char, err: c_int) -> c_int {
    fprintf(stderr, c"%s: %d\n".as_ptr(), msg, err);
    err
}

unsafe fn xbc_show_value(node: *mut xbc_node, semicolon: bool) -> c_int {
    let mut val: *const c_char = ptr::null();
    let eol: *const c_char = if semicolon { c";\n".as_ptr() } else { c"\n".as_ptr() };
    let mut q: c_char;
    let mut i: c_int = 0;

    loop {
        val = xbc_array_value_next(node, val);
        if val.is_null() {
            break;
        }
        if !strchr(val, b'"' as c_int).is_null() {
            q = b'\'' as c_char;
        } else {
            q = b'"' as c_char;
        }
        printf(
            c"%c%s%c%s".as_ptr(),
            q as c_int,
            val,
            q as c_int,
            if xbc_node_is_array(node) { c", ".as_ptr() } else { eol },
        );
        i += 1;
    }
    i
}

unsafe fn xbc_show_compact_tree() {
    let mut node: *mut xbc_node;
    let mut cnode: *mut xbc_node = ptr::null_mut();
    let mut vnode: *mut xbc_node;
    let mut depth: c_int = 0;
    let mut i: c_int;

    node = xbc_root_node();
    while !node.is_null() && xbc_node_is_key(node) {
        i = 0;
        while i < depth {
            printf(c"\t".as_ptr());
            i += 1;
        }
        if cnode.is_null() {
            cnode = xbc_node_get_child(node);
        }
        while !cnode.is_null() && xbc_node_is_key(cnode) && (*cnode).next.is_null() {
            vnode = xbc_node_get_child(cnode);
            /*
             * If @cnode has value and subkeys, this
             * should show it as below.
             *
             * key(@node) {
             *      key(@cnode) = value;
             *      key(@cnode) {
             *          subkeys;
             *      }
             * }
             */
            if !vnode.is_null() && xbc_node_is_value(vnode) && !(*vnode).next.is_null() {
                break;
            }
            printf(c"%s.".as_ptr(), xbc_node_get_data(node));
            node = cnode;
            cnode = vnode;
        }
        if !cnode.is_null() && xbc_node_is_key(cnode) {
            printf(c"%s {\n".as_ptr(), xbc_node_get_data(node));
            depth += 1;
            node = cnode;
            cnode = ptr::null_mut();
            continue;
        } else if !cnode.is_null() && xbc_node_is_value(cnode) {
            printf(c"%s = ".as_ptr(), xbc_node_get_data(node));
            xbc_show_value(cnode, true);
            /*
             * If @node has value and subkeys, continue
             * looping on subkeys with same node.
             */
            if !(*cnode).next.is_null() {
                cnode = xbc_node_get_next(cnode);
                continue;
            }
        } else {
            printf(c"%s;\n".as_ptr(), xbc_node_get_data(node));
        }
        cnode = ptr::null_mut();

        if !(*node).next.is_null() {
            node = xbc_node_get_next(node);
            continue;
        }
        while (*node).next.is_null() {
            node = xbc_node_get_parent(node);
            if node.is_null() {
                return;
            }
            if (*xbc_node_get_child(node)).next.is_null() {
                continue;
            }
            if depth != 0 {
                depth -= 1;
                i = 0;
                while i < depth {
                    printf(c"\t".as_ptr());
                    i += 1;
                }
                printf(c"}\n".as_ptr());
            }
        }
        node = xbc_node_get_next(node);
    }
}

unsafe fn xbc_show_list() {
    let mut key = [0 as c_char; XBC_KEYLEN_MAX];
    let mut leaf: *mut xbc_node = ptr::null_mut();
    let mut val: *const c_char = ptr::null();
    let mut ret: c_int;

    loop {
        val = xbc_key_value_next(&mut leaf, val);
        if val.is_null() {
            break;
        }
        ret = xbc_node_compose_key(leaf, key.as_mut_ptr(), XBC_KEYLEN_MAX);
        if ret < 0 {
            fprintf(stderr, c"Failed to compose key %d\n".as_ptr(), ret);
            break;
        }
        printf(c"%s = ".as_ptr(), key.as_ptr());
        if val.is_null() || *val == 0 {
            printf(c"\"\"\n".as_ptr());
            continue;
        }
        xbc_show_value(xbc_node_get_child(leaf), false);
    }
}

const PAGE_SIZE: c_int = 4096;

unsafe fn load_xbc_fd(fd: c_int, buf: *mut *mut c_char, size: c_int) -> c_int {
    let ret: c_int;

    *buf = malloc(size as size_t + 1) as *mut c_char;
    if (*buf).is_null() {
        return -ENOMEM;
    }

    ret = read(fd, *buf as *mut c_void, size as size_t) as c_int;
    if ret < 0 {
        return -errno;
    }
    *(*buf).add(size as usize) = 0;

    ret
}

/* Return the read size or -errno */
unsafe fn load_xbc_file(path: *const c_char, buf: *mut *mut c_char) -> c_int {
    let mut st: stat = mem::zeroed();
    let fd: c_int;
    let mut ret: c_int;

    fd = open(path, O_RDONLY);
    if fd < 0 {
        return -errno;
    }
    ret = fstat(fd, &mut st);
    if ret < 0 {
        ret = -errno;
        close(fd);
        return ret;
    }

    ret = load_xbc_fd(fd, buf, st.st_size as c_int);

    close(fd);

    ret
}

unsafe fn load_xbc_from_initrd(fd: c_int, buf: *mut *mut c_char) -> c_int {
    let mut st: stat = mem::zeroed();
    let mut ret: c_int;
    let mut size: u32 = 0;
    let mut csum: u32 = 0;
    let rcsum: u32;
    let mut magic = [0 as c_char; BOOTCONFIG_MAGIC_LEN];
    let mut msg: *const c_char = ptr::null();

    ret = fstat(fd, &mut st);
    if ret < 0 {
        return -errno;
    }

    if st.st_size < BOOTCONFIG_FOOTER_SIZE as off_t {
        return 0;
    }

    if lseek(fd, -(BOOTCONFIG_MAGIC_LEN as off_t), SEEK_END) < 0 {
        return pr_errno(c"Failed to lseek for magic".as_ptr(), -errno);
    }

    if read(fd, magic.as_mut_ptr() as *mut c_void, BOOTCONFIG_MAGIC_LEN) < 0 {
        return pr_errno(c"Failed to read".as_ptr(), -errno);
    }

    /* Check the bootconfig magic bytes */
    if memcmp(
        magic.as_ptr() as *const c_void,
        BOOTCONFIG_MAGIC.as_ptr() as *const c_void,
        BOOTCONFIG_MAGIC_LEN,
    ) != 0
    {
        return 0;
    }

    if lseek(fd, -(BOOTCONFIG_FOOTER_SIZE as off_t), SEEK_END) < 0 {
        return pr_errno(c"Failed to lseek for size".as_ptr(), -errno);
    }

    if read(
        fd,
        &mut size as *mut u32 as *mut c_void,
        mem::size_of::<u32>(),
    ) < 0
    {
        return pr_errno(c"Failed to read size".as_ptr(), -errno);
    }
    size = u32::from_le(size);

    if read(
        fd,
        &mut csum as *mut u32 as *mut c_void,
        mem::size_of::<u32>(),
    ) < 0
    {
        return pr_errno(c"Failed to read checksum".as_ptr(), -errno);
    }
    csum = u32::from_le(csum);

    /* Wrong size error  */
    if st.st_size < size as off_t + BOOTCONFIG_FOOTER_SIZE as off_t {
        fprintf(stderr, c"bootconfig size is too big\n".as_ptr());
        return -E2BIG;
    }

    if lseek(
        fd,
        st.st_size - (size as off_t + BOOTCONFIG_FOOTER_SIZE as off_t),
        SEEK_SET,
    ) < 0
    {
        return pr_errno(c"Failed to lseek".as_ptr(), -errno);
    }

    ret = load_xbc_fd(fd, buf, size as c_int);
    if ret < 0 {
        return ret;
    }

    /* Wrong Checksum */
    rcsum = xbc_calc_checksum(*buf, size);
    if csum != rcsum {
        fprintf(stderr, c"checksum error: %u != %u\n".as_ptr(), csum, rcsum);
        return -EINVAL;
    }

    ret = xbc_init(*buf, size, &mut msg, ptr::null_mut());
    /* Wrong data */
    if ret < 0 {
        fprintf(stderr, c"parse error: %s.\n".as_ptr(), msg);
        return ret;
    }

    size as c_int
}

unsafe fn show_xbc_error(data: *const c_char, msg: *const c_char, pos: c_int) {
    let mut lin: c_int = 1;
    let mut col: c_int;
    let mut i: c_int;

    if pos < 0 {
        fprintf(stderr, c"Error: %s.\n".as_ptr(), msg);
        return;
    }

    /* Note that pos starts from 0 but lin and col should start from 1. */
    col = pos + 1;
    i = 0;
    while i < pos {
        if *data.add(i as usize) == b'\n' as c_char {
            lin += 1;
            col = pos - i;
        }
        i += 1;
    }
    fprintf(stderr, c"Parse Error: %s at %d:%d\n".as_ptr(), msg, lin, col);
}

unsafe fn init_xbc_with_error(buf: *mut c_char, len: c_int) -> c_int {
    let copy: *mut c_char = strdup(buf);
    let mut msg: *const c_char = ptr::null();
    let ret: c_int;
    let mut pos: c_int = 0;

    if copy.is_null() {
        return -ENOMEM;
    }

    ret = xbc_init(buf, len as u32, &mut msg, &mut pos);
    if ret < 0 {
        show_xbc_error(copy, msg, pos);
    }
    free(copy as *mut c_void);

    ret
}

unsafe fn show_xbc_kernel_cmdline() -> c_int {
    let root: *mut xbc_node;
    let mut buf: *mut c_char = ptr::null_mut();
    let len: c_int;
    let ret: c_int;

    root = xbc_find_node(c"kernel".as_ptr());
    if root.is_null() {
        return 0; /* no kernel.* keys: emit empty output */
    }

    len = xbc_snprint_cmdline(ptr::null_mut(), 0, root);
    if len < 0 {
        fprintf(stderr, c"Failed to size cmdline output: %d\n".as_ptr(), len);
        return len;
    }
    if len == 0 {
        return 0;
    }

    buf = malloc(len as size_t + 1) as *mut c_char;
    if buf.is_null() {
        return -ENOMEM;
    }

    ret = xbc_snprint_cmdline(buf, len as size_t + 1, root);
    if ret < 0 {
        fprintf(stderr, c"Failed to render cmdline output: %d\n".as_ptr(), ret);
        free(buf as *mut c_void);
        return ret;
    }

    fputs(buf, stdout);
    free(buf as *mut c_void);
    0
}

unsafe fn show_xbc(path: *const c_char, list: bool, render_cmdline: bool) -> c_int {
    let mut ret: c_int;
    let fd: c_int;
    let mut buf: *mut c_char = ptr::null_mut();
    let mut st: stat = mem::zeroed();

    ret = stat(path, &mut st);
    if ret < 0 {
        ret = -errno;
        fprintf(stderr, c"Failed to stat %s: %d\n".as_ptr(), path, ret);
        return ret;
    }

    fd = open(path, O_RDONLY);
    if fd < 0 {
        ret = -errno;
        fprintf(stderr, c"Failed to open initrd %s: %d\n".as_ptr(), path, ret);
        return ret;
    }

    ret = load_xbc_from_initrd(fd, &mut buf);
    close(fd);
    if ret < 0 {
        fprintf(stderr, c"Failed to load a boot config from initrd: %d\n".as_ptr(), ret);
        free(buf as *mut c_void);
        return ret;
    }
    /* Assume a bootconfig file if it is enough small */
    if ret == 0 && st.st_size <= XBC_DATA_MAX as off_t {
        ret = load_xbc_file(path, &mut buf);
        if ret < 0 {
            fprintf(stderr, c"Failed to load a boot config: %d\n".as_ptr(), ret);
            free(buf as *mut c_void);
            return ret;
        }
        if init_xbc_with_error(buf, ret) < 0 {
            free(buf as *mut c_void);
            return ret;
        }
    }
    if render_cmdline {
        ret = show_xbc_kernel_cmdline();
    } else if list {
        xbc_show_list();
    } else {
        xbc_show_compact_tree();
    }
    if ret > 0 {
        ret = 0;
    }
    free(buf as *mut c_void);

    ret
}

unsafe fn delete_xbc(path: *const c_char) -> c_int {
    let mut st: stat = mem::zeroed();
    let mut ret: c_int = 0;
    let fd: c_int;
    let size: c_int;
    let mut buf: *mut c_char = ptr::null_mut();

    fd = open(path, O_RDWR);
    if fd < 0 {
        ret = -errno;
        fprintf(stderr, c"Failed to open initrd %s: %d\n".as_ptr(), path, ret);
        return ret;
    }

    size = load_xbc_from_initrd(fd, &mut buf);
    if size < 0 {
        ret = size;
        fprintf(stderr, c"Failed to load a boot config from initrd: %d\n".as_ptr(), ret);
    } else if size > 0 {
        ret = fstat(fd, &mut st);
        if ret == 0 {
            ret = ftruncate(fd, st.st_size - size as off_t - BOOTCONFIG_FOOTER_SIZE as off_t);
        }
        if ret != 0 {
            ret = -errno;
        }
    } /* Ignore if there is no boot config in initrd */

    close(fd);
    free(buf as *mut c_void);

    ret
}

#[repr(C)]
struct bootconfig_footer {
    size: u32,
    csum: u32,
    magic: [c_char; BOOTCONFIG_MAGIC_LEN],
}

unsafe fn apply_xbc(path: *const c_char, xbc_path: *const c_char) -> c_int {
    let mut footer: bootconfig_footer = mem::zeroed();
    let mut buf: *mut c_char = ptr::null_mut();
    let data: *mut c_char;
    let mut total_size: size_t;
    let mut st: stat = mem::zeroed();
    let mut msg: *const c_char = ptr::null();
    let mut size: u32;
    let csum: u32;
    let mut pos: c_int = 0;
    let pad: c_int;
    let mut ret: c_int;
    let fd: c_int;

    ret = load_xbc_file(xbc_path, &mut buf);
    if ret < 0 {
        fprintf(stderr, c"Failed to load %s : %d\n".as_ptr(), xbc_path, ret);
        return ret;
    }
    size = (strlen(buf) + 1) as u32;
    csum = xbc_calc_checksum(buf, size);

    /* Backup the bootconfig data */
    data = calloc(size as size_t + BOOTCONFIG_ALIGN + BOOTCONFIG_FOOTER_SIZE, 1) as *mut c_char;
    if data.is_null() {
        free(buf as *mut c_void);
        return -ENOMEM;
    }
    memcpy(data as *mut c_void, buf as *const c_void, size as size_t);

    /* Check the data format */
    ret = xbc_init(buf, size, &mut msg, &mut pos);
    if ret < 0 {
        show_xbc_error(data, msg, pos);
        free(data as *mut c_void);
        free(buf as *mut c_void);

        return ret;
    }
    printf(c"Apply %s to %s\n".as_ptr(), xbc_path, path);
    xbc_get_info(&mut ret, ptr::null_mut());
    printf(c"\tNumber of nodes: %d\n".as_ptr(), ret);
    printf(c"\tSize: %u bytes\n".as_ptr(), size as c_uint);
    printf(c"\tChecksum: %u\n".as_ptr(), csum as c_uint);

    /* TODO: Check the options by schema */
    xbc_exit();
    free(buf as *mut c_void);

    /* Remove old boot config if exists */
    ret = delete_xbc(path);
    if ret < 0 {
        fprintf(stderr, c"Failed to delete previous boot config: %d\n".as_ptr(), ret);
        free(data as *mut c_void);
        return ret;
    }

    /* Apply new one */
    fd = open(path, O_RDWR | O_APPEND);
    if fd < 0 {
        ret = -errno;
        fprintf(stderr, c"Failed to open %s: %d\n".as_ptr(), path, ret);
        free(data as *mut c_void);
        return ret;
    }
    /* TODO: Ensure the @path is initramfs/initrd image */
    if fstat(fd, &mut st) < 0 {
        ret = -errno;
        fprintf(stderr, c"Failed to get the size of %s\n".as_ptr(), path);
        close(fd);
        free(data as *mut c_void);
        return ret;
    }

    /* To align up the total size to BOOTCONFIG_ALIGN, get padding size */
    total_size = st.st_size as size_t + size as size_t + BOOTCONFIG_FOOTER_SIZE;
    pad = (((total_size + BOOTCONFIG_ALIGN - 1) & !BOOTCONFIG_ALIGN_MASK) - total_size) as c_int;
    size = size.wrapping_add(pad as u32);

    /* Add a footer */
    footer.size = size.to_le();
    footer.csum = csum.to_le();
    memcpy(
        footer.magic.as_mut_ptr() as *mut c_void,
        BOOTCONFIG_MAGIC.as_ptr() as *const c_void,
        BOOTCONFIG_MAGIC_LEN,
    );
    const _: [(); BOOTCONFIG_FOOTER_SIZE] = [(); mem::size_of::<bootconfig_footer>()];
    memcpy(
        data.add(size as usize) as *mut c_void,
        &footer as *const bootconfig_footer as *const c_void,
        BOOTCONFIG_FOOTER_SIZE,
    );

    total_size = size as size_t + BOOTCONFIG_FOOTER_SIZE;

    ret = write(fd, data as *const c_void, total_size) as c_int;
    if (ret as size_t) < total_size {
        if ret < 0 {
            ret = -errno;
        }
        fprintf(stderr, c"Failed to apply a boot config: %d\n".as_ptr(), ret);
        if ret >= 0 {
            /* Map the partial write to -ENOSPC */
            if ret >= 0 {
                ret = -ENOSPC;
            }
            if ftruncate(fd, st.st_size) < 0 {
                ret = -errno;
                fprintf(stderr, c"Failed to rollback the write error: %d\n".as_ptr(), ret);
                fprintf(
                    stderr,
                    c"The initrd %s may be corrupted. Recommend to rebuild.\n".as_ptr(),
                    path,
                );
            }
        }
    } else {
        ret = 0;
    }

    close(fd);
    free(data as *mut c_void);

    ret
}

unsafe fn usage() -> c_int {
    printf(
        c"Usage: bootconfig [OPTIONS] <INITRD>\nOr     bootconfig <CONFIG>\n Apply, delete or show boot config to initrd.\n Options:\n\t\t-a <config>: Apply boot config to initrd\n\t\t-d : Delete boot config file from initrd\n\t\t-l : list boot config in initrd or file\n\t\t-C : render the kernel.* subtree as a flat cmdline\n\t\t     string (suitable for embedding in a kernel image)\n\t\t     and print it to stdout\n\n If no option is given, show the bootconfig in the given file.\n".as_ptr(),
    );
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut path: *mut c_char;
    let mut apply: *mut c_char = ptr::null_mut();
    let mut render_cmdline: bool = false;
    let mut delete: bool = false;
    let mut list: bool = false;
    let mut opt: c_int;

    loop {
        opt = getopt(argc, argv as *const *mut c_char, c"hda:lC".as_ptr());
        if opt == -1 {
            break;
        }
        match opt {
            x if x == b'd' as c_int => {
                delete = true;
            }
            x if x == b'a' as c_int => {
                apply = optarg;
            }
            x if x == b'l' as c_int => {
                list = true;
            }
            x if x == b'C' as c_int => {
                render_cmdline = true;
            }
            x if x == b'h' as c_int => {
                return usage();
            }
            _ => {
                return usage();
            }
        }
    }

    if ((if !apply.is_null() { 1 } else { 0 })
        + (if delete { 1 } else { 0 })
        + (if list { 1 } else { 0 })
        + (if render_cmdline { 1 } else { 0 }))
        > 1
    {
        fprintf(
            stderr,
            c"Error: You can give one of -a, -d, -l or -C at once.\n".as_ptr(),
        );
        return usage();
    }

    if optind >= argc {
        fprintf(stderr, c"Error: No initrd is specified.\n".as_ptr());
        return usage();
    }

    path = *argv.add(optind as usize);

    if !apply.is_null() {
        return apply_xbc(path, apply);
    } else if delete {
        return delete_xbc(path);
    }

    show_xbc(path, list, render_cmdline)
}
