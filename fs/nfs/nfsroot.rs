// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful Rust translation of nfsroot.c. Kernel-provided types, globals,
 * functions, and macros are intentionally referenced as external dependencies.
 */

// #define NFSDBG_FACILITY NFSDBG_ROOT
pub const NFS_ROOT: &str = "/tftpboot/%s";
// The selected default depends on the kernel configuration:
pub const NFS_DEF_OPTIONS: &str = "vers=4,tcp,rsize=4096,wsize=4096";

static mut NFS_ROOT_PARMS: [u8; NFS_MAXPATHLEN + 1] = [0; NFS_MAXPATHLEN + 1];
static mut NFS_ROOT_OPTIONS: [u8; 256] = [0; 256];
static mut SERVADDR: __be32 = htonl(INADDR_NONE);
static mut NFS_EXPORT_PATH: [u8; NFS_MAXPATHLEN + 1] = [0; NFS_MAXPATHLEN + 1];
static mut NFS_ROOT_DEVICE: [u8; NFS_MAXPATHLEN + 1] = [0; NFS_MAXPATHLEN + 1];

#[cfg(feature = "NFS_DEBUG")]
unsafe extern "C" fn nfs_root_debug(_unused: *mut c_char) -> c_int {
    nfs_debug |= NFSDBG_ROOT | NFSDBG_MOUNT;
    1
}

// __setup("nfsrootdebug", nfs_root_debug);

unsafe extern "C" fn nfs_root_setup(line: *mut c_char) -> c_int {
    ROOT_DEV = Root_NFS;
    if *line == b'/' as c_char || *line == b',' as c_char
        || (*line >= b'0' as c_char && *line <= b'9' as c_char)
    {
        strscpy(NFS_ROOT_PARMS.as_mut_ptr(), line, NFS_ROOT_PARMS.len());
    } else {
        let n = strlen(line) + NFS_ROOT.len() - 1;
        if n >= NFS_ROOT_PARMS.len() {
            *line.add(NFS_ROOT_PARMS.len() - NFS_ROOT.len() - 2) = 0;
        }
        sprintf(NFS_ROOT_PARMS.as_mut_ptr(), NFS_ROOT.as_ptr() as *const c_char, line);
    }
    root_server_addr = root_nfs_parse_addr(NFS_ROOT_PARMS.as_mut_ptr());
    1
}

// __setup("nfsroot=", nfs_root_setup);

unsafe extern "C" fn root_nfs_copy(dest: *mut c_char, src: *const c_char, destlen: usize) -> c_int {
    if strscpy(dest, src, destlen) == -E2BIG { -1 } else { 0 }
}

unsafe extern "C" fn root_nfs_cat(dest: *mut c_char, src: *const c_char, destlen: usize) -> c_int {
    let len = strlen(dest);
    if len != 0 && *dest.add(len - 1) != b',' as c_char {
        if strlcat(dest, b",".as_ptr() as *const c_char, destlen) >= destlen { return -1; }
    }
    if strlcat(dest, src, destlen) >= destlen { return -1; }
    0
}

unsafe extern "C" fn root_nfs_parse_options(mut incoming: *mut c_char, exppath: *mut c_char, exppathlen: usize) -> c_int {
    let p = strsep(&mut incoming, b",".as_ptr() as *const c_char);
    if *p != 0 && strcmp(p, b"default\0".as_ptr() as *const c_char) != 0 {
        if root_nfs_copy(exppath, p, exppathlen) != 0 { return -1; }
    }
    if !incoming.is_null() && *incoming != 0 {
        if root_nfs_cat(NFS_ROOT_OPTIONS.as_mut_ptr(), incoming, NFS_ROOT_OPTIONS.len()) != 0 { return -1; }
    }
    0
}

unsafe extern "C" fn root_nfs_data(cmdline: *mut c_char) -> c_int {
    let mut mand_options = [0 as c_char; 32 + INET_ADDRSTRLEN + 1];
    let mut retval = -1;
    let tmplen = NFS_EXPORT_PATH.len();
    let tmp = kzalloc(tmplen, GFP_KERNEL);
    if tmp.is_null() { printk(KERN_ERR, b"Root-NFS: could not allocate memory\n\0".as_ptr() as *const c_char); return retval; }
    strcpy(tmp, NFS_ROOT.as_ptr() as *const c_char);
    if *root_server_path != 0 && root_nfs_parse_options(root_server_path, tmp, tmplen) != 0 { printk(KERN_ERR, b"Root-NFS: mount options string too long\n\0".as_ptr() as *const c_char); kfree(tmp); return retval; }
    if *cmdline != 0 && root_nfs_parse_options(cmdline, tmp, tmplen) != 0 { printk(KERN_ERR, b"Root-NFS: mount options string too long\n\0".as_ptr() as *const c_char); kfree(tmp); return retval; }
    snprintf(mand_options.as_mut_ptr(), mand_options.len(), b"nolock,addr=%pI4\0".as_ptr() as *const c_char, &SERVADDR);
    if root_nfs_cat(NFS_ROOT_OPTIONS.as_mut_ptr(), mand_options.as_ptr(), NFS_ROOT_OPTIONS.len()) != 0 { printk(KERN_ERR, b"Root-NFS: mount options string too long\n\0".as_ptr() as *const c_char); kfree(tmp); return retval; }
    let len = snprintf(NFS_EXPORT_PATH.as_mut_ptr(), NFS_EXPORT_PATH.len(), tmp, (*utsname()).nodename.as_ptr());
    if len >= NFS_EXPORT_PATH.len() as c_int { printk(KERN_ERR, b"Root-NFS: root device name too long.\n\0".as_ptr() as *const c_char); kfree(tmp); return retval; }
    let len = snprintf(NFS_ROOT_DEVICE.as_mut_ptr(), NFS_ROOT_DEVICE.len(), b"%pI4:%s\0".as_ptr() as *const c_char, &SERVADDR, NFS_EXPORT_PATH.as_ptr());
    if len >= NFS_ROOT_DEVICE.len() as c_int { printk(KERN_ERR, b"Root-NFS: root device name too long.\n\0".as_ptr() as *const c_char); kfree(tmp); return retval; }
    retval = 0;
    kfree(tmp);
    retval
}

pub unsafe extern "C" fn nfs_root_data(root_device: *mut *mut c_char, root_data: *mut *mut c_char) -> c_int {
    SERVADDR = root_server_addr;
    if SERVADDR == htonl(INADDR_NONE) { printk(KERN_ERR, b"Root-NFS: no NFS server address\n\0".as_ptr() as *const c_char); return -1; }
    if root_nfs_data(NFS_ROOT_PARMS.as_mut_ptr()) < 0 { return -1; }
    *root_device = NFS_ROOT_DEVICE.as_mut_ptr();
    *root_data = NFS_ROOT_OPTIONS.as_mut_ptr();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
