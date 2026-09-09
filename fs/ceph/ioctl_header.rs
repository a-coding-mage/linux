/* SPDX-License-Identifier: GPL-2.0 */

// Equivalent of the Linux ioctl declarations included by the original header.

pub const CEPH_IOCTL_MAGIC: u32 = 0x97;

/*
 * CEPH_IOC_GET_LAYOUT - get file layout or dir layout policy
 * CEPH_IOC_SET_LAYOUT - set file layout
 * CEPH_IOC_SET_LAYOUT_POLICY - set dir layout policy
 *
 * The file layout specifies how file data is striped over objects in
 * the distributed object store, which object pool they belong to (if
 * it differs from the default), and an optional 'preferred osd' to
 * store them on.
 *
 * Files get a new layout based on the policy set on the containing
 * directory or one of its ancestors.  The GET_LAYOUT ioctl will let
 * you examine the layout for a file or the policy on a directory.
 *
 * SET_LAYOUT will let you set a layout on a newly created file.  This
 * only works immediately after the file is created and before any
 * data is written to it.
 *
 * SET_LAYOUT_POLICY will let you set a layout policy (default layout)
 * on a directory that will apply to any new files created in that
 * directory (or any child directory that doesn't specify a layout of
 * its own).
 */

/* use u64 to align sanely on all archs */
#[repr(C)]
pub struct ceph_ioctl_layout {
    pub stripe_unit: u64,
    pub stripe_count: u64,
    pub object_size: u64,
    pub data_pool: u64,

    /* obsolete.  new values ignored, always return -1 */
    pub preferred_osd: i64,
}

/* Linux generic ioctl encoding, retained as a local equivalent of _IOC. */
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> libc::c_ulong {
    ((dir << IOC_DIRSHIFT) | (ty << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)) as libc::c_ulong
}

pub const CEPH_IOC_GET_LAYOUT: libc::c_ulong =
    ioc(IOC_READ, CEPH_IOCTL_MAGIC, 1, core::mem::size_of::<ceph_ioctl_layout>() as u32);
pub const CEPH_IOC_SET_LAYOUT: libc::c_ulong =
    ioc(IOC_WRITE, CEPH_IOCTL_MAGIC, 2, core::mem::size_of::<ceph_ioctl_layout>() as u32);
pub const CEPH_IOC_SET_LAYOUT_POLICY: libc::c_ulong =
    ioc(IOC_WRITE, CEPH_IOCTL_MAGIC, 5, core::mem::size_of::<ceph_ioctl_layout>() as u32);

/*
 * CEPH_IOC_GET_DATALOC - get location of file data in the cluster
 *
 * Extract identity, address of the OSD and object storing a given
 * file offset.
 */
#[repr(C)]
pub struct ceph_ioctl_dataloc {
    pub file_offset: u64,       /* in+out: file offset */
    pub object_offset: u64,     /* out: offset in object */
    pub object_no: u64,         /* out: object # */
    pub object_size: u64,       /* out: object size */
    pub object_name: [libc::c_char; 64], /* out: object name */
    pub block_offset: u64,      /* out: offset in block */
    pub block_size: u64,        /* out: block length */
    pub osd: i64,               /* out: osd # */
    pub osd_addr: libc::sockaddr_storage, /* out: osd address */
}

pub const CEPH_IOC_GET_DATALOC: libc::c_ulong =
    ioc(IOC_READ | IOC_WRITE, CEPH_IOCTL_MAGIC, 3, core::mem::size_of::<ceph_ioctl_dataloc>() as u32);

/*
 * CEPH_IOC_LAZYIO - relax consistency
 *
 * Normally Ceph switches to synchronous IO when multiple clients have
 * the file open (and or more for write).  Reads and writes bypass the
 * page cache and go directly to the OSD.  Setting this flag on a file
 * descriptor will allow buffered IO for this file in cases where the
 * application knows it won't interfere with other nodes (or doesn't
 * care).
 */
pub const CEPH_IOC_LAZYIO: libc::c_ulong = ioc(0, CEPH_IOCTL_MAGIC, 4, 0);

/*
 * CEPH_IOC_SYNCIO - force synchronous IO
 *
 * This ioctl sets a file flag that forces the synchronous IO that
 * bypasses the page cache, even if it is not necessary.  This is
 * essentially the opposite behavior of IOC_LAZYIO.  This forces the
 * same read/write path as a file opened by multiple clients when one
 * or more of those clients is opened for write.
 *
 * Note that this type of sync IO takes a different path than a file
 * opened with O_SYNC/D_SYNC (writes hit the page cache and are
 * immediately flushed on page boundaries).  It is very similar to
 * O_DIRECT (writes bypass the page cache) excep that O_DIRECT writes
 * are not copied (user page must remain stable) and O_DIRECT writes
 * have alignment restrictions (on the buffer and file offset).
 */
pub const CEPH_IOC_SYNCIO: libc::c_ulong = ioc(0, CEPH_IOCTL_MAGIC, 5, 0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
