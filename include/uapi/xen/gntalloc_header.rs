/******************************************************************************
 * gntalloc.h
 *
 * Interface to /dev/xen/gntalloc.
 *
 * Author: Daniel De Graaf <dgdegra@tycho.nsa.gov>
 *
 * This file is in the public domain.
 */

// Dependency intent: the C header includes <linux/types.h>.

/*
 * Allocates a new page and creates a new grant reference.
 */
pub const IOCTL_GNTALLOC_ALLOC_GREF: _ = _IOC(_IOC_NONE, 'G', 5, core::mem::size_of::<ioctl_gntalloc_alloc_gref>());

#[repr(C)]
pub struct ioctl_gntalloc_alloc_gref {
    /* IN parameters */
    /* The ID of the domain to be given access to the grants. */
    pub domid: u16,
    /* Flags for this mapping */
    pub flags: u16,
    /* Number of pages to map */
    pub count: u32,
    /* OUT parameters */
    /* The offset to be used on a subsequent call to mmap(). */
    pub index: u64,
    /* The grant references of the newly created grant, one per page */
    /* Variable size, depending on count */
    pub gref_ids: ioctl_gntalloc_alloc_gref__bindgen_ty_1,
}

#[repr(C)]
pub union ioctl_gntalloc_alloc_gref__bindgen_ty_1 {
    pub gref_ids: [u32; 1],
    pub gref_ids_flex: [u32; 0],
}

pub const GNTALLOC_FLAG_WRITABLE: u32 = 1;

/*
 * Deallocates the grant reference, allowing the associated page to be freed if
 * no other domains are using it.
 */
pub const IOCTL_GNTALLOC_DEALLOC_GREF: _ = _IOC(_IOC_NONE, 'G', 6, core::mem::size_of::<ioctl_gntalloc_dealloc_gref>());

#[repr(C)]
pub struct ioctl_gntalloc_dealloc_gref {
    /* IN parameters */
    /* The offset returned in the map operation */
    pub index: u64,
    /* Number of references to unmap */
    pub count: u32,
}

/*
 * Sets up an unmap notification within the page, so that the other side can do
 * cleanup if this side crashes. Required to implement cross-domain robust
 * mutexes or close notification on communication channels.
 *
 * Each mapped page only supports one notification; multiple calls referring to
 * the same page overwrite the previous notification. You must clear the
 * notification prior to the IOCTL_GNTALLOC_DEALLOC_GREF if you do not want it
 * to occur.
 */
pub const IOCTL_GNTALLOC_SET_UNMAP_NOTIFY: _ = _IOC(_IOC_NONE, 'G', 7, core::mem::size_of::<ioctl_gntalloc_unmap_notify>());

#[repr(C)]
pub struct ioctl_gntalloc_unmap_notify {
    /* IN parameters */
    /* Offset in the file descriptor for a byte within the page (same as
     * used in mmap). If using UNMAP_NOTIFY_CLEAR_BYTE, this is the byte to
     * be cleared. Otherwise, it can be any byte in the page whose
     * notification we are adjusting.
     */
    pub index: u64,
    /* Action(s) to take on unmap */
    pub action: u32,
    /* Event channel to notify */
    pub event_channel_port: u32,
}

/* Clear (set to zero) the byte specified by index */
pub const UNMAP_NOTIFY_CLEAR_BYTE: u32 = 0x1;
/* Send an interrupt on the indicated event channel */
pub const UNMAP_NOTIFY_SEND_EVENT: u32 = 0x2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
