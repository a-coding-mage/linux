/* SPDX-License-Identifier: BSD-3-Clause */

use core::ffi::{c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

pub const ICMSGHDRFLAG_TRANSACTION: u32 = 1;
pub const ICMSGHDRFLAG_REQUEST: u32 = 2;
pub const ICMSGHDRFLAG_RESPONSE: u32 = 4;

pub const IC_VERSION_NEGOTIATION_MAX_VER_COUNT: u32 = 100;
pub const ICMSG_HDR: usize = size_of::<vmbuspipe_hdr>() + size_of::<icmsg_hdr>();
pub const fn ICMSG_NEGOTIATE_PKT_SIZE(icframe_vercnt: usize, icmsg_vercnt: usize) -> usize {
    ICMSG_HDR
        + size_of::<icmsg_negotiate>()
        + ((icframe_vercnt + icmsg_vercnt) * size_of::<ic_version>())
}

/*
 * Channel packets
 */

/* Channel packet flags */
pub const VMBUS_CHANPKT_TYPE_INBAND: u16 = 0x0006;
pub const VMBUS_CHANPKT_TYPE_RXBUF: u16 = 0x0007;
pub const VMBUS_CHANPKT_TYPE_GPA: u16 = 0x0009;
pub const VMBUS_CHANPKT_TYPE_COMP: u16 = 0x000b;

pub const VMBUS_CHANPKT_FLAG_NONE: u16 = 0;
pub const VMBUS_CHANPKT_FLAG_RC: u16 = 0x0001; /* report completion */

pub const VMBUS_CHANPKT_SIZE_SHIFT: u32 = 3;
pub const VMBUS_CHANPKT_SIZE_ALIGN: u32 = 1u32 << VMBUS_CHANPKT_SIZE_SHIFT;
pub const VMBUS_CHANPKT_HLEN_MIN: usize =
    size_of::<vmbus_chanpkt_hdr>() >> VMBUS_CHANPKT_SIZE_SHIFT;

/*
 * Buffer ring
 */
#[repr(C, packed)]
pub struct vmbus_bufring {
    pub windex: u32,
    pub rindex: u32,

    /*
     * Interrupt mask {0,1}
     *
     * For TX bufring, host set this to 1, when it is processing
     * the TX bufring, so that we can safely skip the TX event
     * notification to host.
     *
     * For RX bufring, once this is set to 1 by us, host will not
     * further dispatch interrupts to us, even if there are data
     * pending on the RX bufring.  This effectively disables the
     * interrupt of the channel to which this RX bufring is attached.
     */
    pub imask: u32,

    /*
     * Win8 uses some of the reserved bits to implement
     * interrupt driven flow management. On the send side
     * we can request that the receiver interrupt the sender
     * when the ring transitions from being full to being able
     * to handle a message of size "pending_send_sz".
     *
     * Add necessary state for this enhancement.
     */
    pub pending_send: u32,
    pub reserved1: [u32; 12],

    pub feature_bits: vmbus_bufring_feature_bits,

    /* Pad it to rte_mem_page_size() so that data starts on page boundary */
    pub reserved2: [u8; 4028],

    /*
     * Ring data starts here + RingDataStartOffset
     * !!! DO NOT place any fields below this !!!
     */
    pub data: [u8; 0],
}

#[repr(C)]
pub union vmbus_bufring_feature_bits {
    pub bits: vmbus_bufring_feature_bits_bits,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_bufring_feature_bits_bits {
    pub _bitfield: u32,
}

impl vmbus_bufring_feature_bits_bits {
    pub const FEAT_PENDING_SEND_SZ_MASK: u32 = 0x1;

    pub fn feat_pending_send_sz(&self) -> u32 {
        self._bitfield & Self::FEAT_PENDING_SEND_SZ_MASK
    }

    pub fn set_feat_pending_send_sz(&mut self, val: u32) {
        self._bitfield =
            (self._bitfield & !Self::FEAT_PENDING_SEND_SZ_MASK)
                | (val & Self::FEAT_PENDING_SEND_SZ_MASK);
    }
}

#[repr(C)]
pub struct vmbus_br {
    pub vbr: *mut vmbus_bufring,
    pub dsize: u32,
    pub windex: u32, /* next available location */
}

#[repr(C, packed)]
pub struct vmbus_chanpkt_hdr {
    pub type_: u16, /* VMBUS_CHANPKT_TYPE_ */
    pub hlen: u16,  /* header len, in 8 bytes */
    pub tlen: u16,  /* total len, in 8 bytes */
    pub flags: u16, /* VMBUS_CHANPKT_FLAG_ */
    pub xactid: u64,
}

#[repr(C, packed)]
pub struct vmbus_chanpkt {
    pub hdr: vmbus_chanpkt_hdr,
}

#[repr(C, packed)]
pub struct vmbuspipe_hdr {
    pub flags: c_uint,
    pub msgsize: c_uint,
}

#[repr(C, packed)]
pub struct ic_version {
    pub major: u16,
    pub minor: u16,
}

#[repr(C, packed)]
pub struct icmsg_negotiate {
    pub icframe_vercnt: u16,
    pub icmsg_vercnt: u16,
    pub reserved: c_uint,
    pub icversion_data: [ic_version; 0], /* any size array */
}

#[repr(C, packed)]
pub struct icmsg_hdr {
    pub icverframe: ic_version,
    pub icmsgtype: u16,
    pub icvermsg: ic_version,
    pub icmsgsize: u16,
    pub status: c_uint,
    pub ictransaction_id: u8,
    pub icflags: u8,
    pub reserved: [u8; 2],
}

unsafe extern "C" {
    pub fn rte_vmbus_chan_recv_raw(
        rxbr: *mut vmbus_br,
        data: *mut c_void,
        len: *mut u32,
    ) -> c_int;
    pub fn rte_vmbus_chan_send(
        txbr: *mut vmbus_br,
        type_: u16,
        data: *mut c_void,
        dlen: u32,
        flags: u32,
    ) -> c_int;
    pub fn vmbus_br_setup(br: *mut vmbus_br, buf: *mut c_void, blen: c_uint);
    pub fn vmbus_uio_map(fd: *mut c_int, size: c_int) -> *mut c_void;
}

/* Amount of space available for write */
pub unsafe fn vmbus_br_availwrite(br: *const vmbus_br, windex: u32) -> u32 {
    let vbr = ptr::addr_of!((*br).vbr).read_unaligned();
    let rindex = ptr::addr_of!((*vbr).rindex).read_volatile();

    if windex >= rindex {
        ptr::addr_of!((*br).dsize)
            .read_unaligned()
            .wrapping_sub(windex.wrapping_sub(rindex))
    } else {
        rindex.wrapping_sub(windex)
    }
}

pub unsafe fn vmbus_br_availread(br: *const vmbus_br) -> u32 {
    let vbr = ptr::addr_of!((*br).vbr).read_unaligned();
    let windex = ptr::addr_of!((*vbr).windex).read_volatile();
    ptr::addr_of!((*br).dsize)
        .read_unaligned()
        .wrapping_sub(vmbus_br_availwrite(br, windex))
}
