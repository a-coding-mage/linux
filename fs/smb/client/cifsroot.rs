// SPDX-License-Identifier: GPL-2.0
/*
 * SMB root file system support
 *
 * Copyright (c) 2019 Paulo Alcantara <palcantara@suse.de>
 */

// Linux kernel dependencies supplied by other translation units.
extern "C" {
    static mut ROOT_DEV: u32;
    static mut root_server_addr: u32;
    static Root_CIFS: u32;
    fn in_aton(addr: *const u8) -> u32;
    fn htonl(value: u32) -> u32;
    fn strlen(s: *const u8) -> usize;
    fn strchr(s: *const u8, c: i32) -> *mut u8;
    fn strchrnul(s: *const u8, c: i32) -> *mut u8;
    fn isdigit(c: i32) -> i32;
    fn pr_err(fmt: *const u8, ...);
}

const DEFAULT_MNT_OPTS: &[u8] =
    b"vers=1.0,cifsacl,mfsymlinks,rsize=1048576,wsize=65536,uid=0,gid=0,hard,rootfs\0";

static mut root_dev: [u8; 2048] = [0; 2048];
static mut root_opts: [u8; 1024] = [0; 1024];

unsafe fn parse_srvaddr(mut start: *mut u8, end: *mut u8) -> u32 {
    /* TODO: ipv6 support */
    let mut addr = [0u8; 16];
    let mut i = 0usize;

    while start < end && i < addr.len() - 1 {
        let ch = *start;
        if isdigit(ch as i32) != 0 || ch == b'.' {
            addr[i] = ch;
            i += 1;
        }
        start = start.add(1);
    }
    addr[i] = 0;
    in_aton(addr.as_ptr())
}

/* cifsroot=//<server-ip>/<share>[,options] */
unsafe extern "C" fn cifs_root_setup(line: *mut u8) -> i32 {
    let mut s: *mut u8;
    let len: usize;
    let mut srvaddr = htonl(0xffff_ffff);

    ROOT_DEV = Root_CIFS;

    if strlen(line) > 3 && *line == b'/' && *line.add(1) == b'/' {
        s = strchr(line.add(2), b'/' as i32);
        if s.is_null() || *s.add(1) == 0 {
            return 1;
        }

        /* make s point to ',' or '\0' at end of line */
        s = strchrnul(s, b',' as i32);
        /* len is strlen(unc) + '\0' */
        len = s.offset_from(line) as usize + 1;
        if len > root_dev.len() {
            pr_err(b"Root-CIFS: UNC path too long\n\0".as_ptr());
            return 1;
        }
        let mut n = 0usize;
        while n < len {
            root_dev[n] = *line.add(n);
            n += 1;
        }
        srvaddr = parse_srvaddr(line.add(2), s);
        if *s != 0 {
            let mut opt_len = 0usize;
            while DEFAULT_MNT_OPTS[opt_len] != 0 {
                opt_len += 1;
            }
            let mut extra_len = 0usize;
            while *s.add(1 + extra_len) != 0 {
                extra_len += 1;
            }
            let n = opt_len + 1 + extra_len;
            if n >= root_opts.len() {
                pr_err(b"Root-CIFS: mount options string too long\n\0".as_ptr());
                root_opts[root_opts.len() - 1] = 0;
                return 1;
            }
            root_opts[..opt_len].copy_from_slice(&DEFAULT_MNT_OPTS[..opt_len]);
            root_opts[opt_len] = b',';
            for j in 0..extra_len {
                root_opts[opt_len + 1 + j] = *s.add(1 + j);
            }
            root_opts[n] = 0;
        }
    }

    root_server_addr = srvaddr;
    1
}

// __setup("cifsroot=", cifs_root_setup);

pub unsafe extern "C" fn cifs_root_data(dev: *mut *mut u8, opts: *mut *mut u8) -> i32 {
    if root_dev[0] == 0 || root_server_addr == htonl(0xffff_ffff) {
        pr_err(b"Root-CIFS: no SMB server address\n\0".as_ptr());
        return -1;
    }

    *dev = root_dev.as_mut_ptr();
    *opts = root_opts.as_mut_ptr();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
