/* An interface for efficient virtio implementation, currently for use by KVM,
 * but hopefully others soon.  Do NOT change this since it will
 * break existing servers and clients.
 *
 * This header is BSD licensed so anyone can use the definitions to implement
 * compatible drivers/servers.
 */

/* Dependencies supplied by the Linux type headers. */

pub const VRING_DESC_F_NEXT: u32 = 1;
pub const VRING_DESC_F_WRITE: u32 = 2;
pub const VRING_DESC_F_INDIRECT: u32 = 4;

pub const VRING_PACKED_DESC_F_AVAIL: u32 = 7;
pub const VRING_PACKED_DESC_F_USED: u32 = 15;

pub const VRING_USED_F_NO_NOTIFY: u32 = 1;
pub const VRING_AVAIL_F_NO_INTERRUPT: u32 = 1;

pub const VRING_PACKED_EVENT_FLAG_ENABLE: u32 = 0x0;
pub const VRING_PACKED_EVENT_FLAG_DISABLE: u32 = 0x1;
pub const VRING_PACKED_EVENT_FLAG_DESC: u32 = 0x2;
pub const VRING_PACKED_EVENT_F_WRAP_CTR: u32 = 15;

pub const VIRTIO_RING_F_INDIRECT_DESC: u32 = 28;
pub const VIRTIO_RING_F_EVENT_IDX: u32 = 29;

pub const VRING_AVAIL_ALIGN_SIZE: usize = 2;
pub const VRING_USED_ALIGN_SIZE: usize = 4;
pub const VRING_DESC_ALIGN_SIZE: usize = 16;

#[repr(C)]
pub struct vring_desc {
    pub addr: __virtio64,
    pub len: __virtio32,
    pub flags: __virtio16,
    pub next: __virtio16,
}

#[repr(C)]
pub struct vring_avail {
    pub flags: __virtio16,
    pub idx: __virtio16,
    pub ring: [__virtio16; 0],
}

#[repr(C)]
pub struct vring_used_elem {
    pub id: __virtio32,
    pub len: __virtio32,
}

pub type vring_used_elem_t = vring_used_elem;

#[repr(C)]
pub struct vring_used {
    pub flags: __virtio16,
    pub idx: __virtio16,
    pub ring: [vring_used_elem_t; 0],
}

pub type vring_desc_t = vring_desc;
pub type vring_avail_t = vring_avail;
pub type vring_used_t = vring_used;

#[repr(C)]
pub struct vring {
    pub num: core::ffi::c_uint,
    pub desc: *mut vring_desc_t,
    pub avail: *mut vring_avail_t,
    pub used: *mut vring_used_t,
}

/* The standard layout for the legacy ring is a continuous chunk of memory. */
#[inline]
pub unsafe fn vring_used_event(vr: *mut vring) -> *mut __virtio16 {
    (*(*vr).avail).ring.as_mut_ptr().add((*vr).num as usize)
}

#[inline]
pub unsafe fn vring_avail_event(vr: *mut vring) -> *mut __virtio16 {
    (*(*vr).used).ring.as_mut_ptr().cast::<__virtio16>().add((*vr).num as usize)
}

#[inline]
pub unsafe fn vring_init(vr: *mut vring, num: core::ffi::c_uint, p: *mut core::ffi::c_void, align: usize) {
    (*vr).num = num;
    (*vr).desc = p.cast();
    (*vr).avail = (p.cast::<u8>().add((num as usize) * core::mem::size_of::<vring_desc>())).cast();
    let ring_end = (*vr).avail.cast::<u8>().add(2 * (num as usize + 2));
    let addr = ring_end as usize;
    (*vr).used = ((addr + align - 1) & !(align - 1)) as *mut vring_used_t;
}

#[inline]
pub const fn vring_size(num: core::ffi::c_uint, align: usize) -> usize {
    (((core::mem::size_of::<vring_desc>() * num as usize)
        + (core::mem::size_of::<__virtio16>() * (3 + num as usize))
        + align - 1) & !(align - 1))
        + core::mem::size_of::<__virtio16>() * 3
        + core::mem::size_of::<vring_used_elem>() * num as usize
}

#[inline]
pub fn vring_need_event(event_idx: __u16, new_idx: __u16, old: __u16) -> bool {
    new_idx.wrapping_sub(event_idx).wrapping_sub(1)
        < new_idx.wrapping_sub(old)
}

#[repr(C)]
pub struct vring_packed_desc_event {
    pub off_wrap: __le16,
    pub flags: __le16,
}

#[repr(C)]
pub struct vring_packed_desc {
    pub addr: __le64,
    pub len: __le32,
    pub id: __le16,
    pub flags: __le16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
