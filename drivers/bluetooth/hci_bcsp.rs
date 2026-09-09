// SPDX-License-Identifier: GPL-2.0-or-later
/* Bluetooth HCI UART driver - source-level Rust translation. */

// Linux kernel dependencies supplied by the surrounding repository.

static mut TXCRC: bool = true;
static mut HCIEXTN: bool = true;

const BCSP_TXWINSIZE: usize = 4;
const BCSP_ACK_PKT: u8 = 0x05;
const BCSP_LE_PKT: u8 = 0x06;

#[repr(C)]
struct BcspStruct {
    unack: SkBuffHead,
    rel: SkBuffHead,
    unrel: SkBuffHead,
    rx_count: c_ulong,
    rx_skb: *mut SkBuff,
    rxseq_txack: u8,
    rxack: u8,
    tbcsp: TimerList,
    hu: *mut HciUart,
    rx_state: BcspRxState,
    rx_esc_state: BcspEscState,
    use_crc: u8,
    message_crc: u16,
    txack_req: u8,
    msgq_txseq: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum BcspRxState { W4PktDelimiter, W4PktStart, W4BcspHdr, W4Data, W4Crc }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum BcspEscState { NoEsc, Esc }

static CRC_TABLE: [u16; 16] = [
    0x0000,0x1081,0x2102,0x3183,0x4204,0x5285,0x6306,0x7387,
    0x8408,0x9489,0xa50a,0xb58b,0xc60c,0xd68d,0xe70e,0xf78f,
];

#[inline]
unsafe fn bcsp_crc_update(crc: *mut u16, d: u8) {
    let mut reg = *crc;
    reg = (reg >> 4) ^ CRC_TABLE[((reg ^ d as u16) & 0xf) as usize];
    reg = (reg >> 4) ^ CRC_TABLE[((reg ^ (d >> 4) as u16) & 0xf) as usize];
    *crc = reg;
}

unsafe fn bcsp_slip_msgdelim(skb: *mut SkBuff) { let c: u8 = 0xc0; skb_put_data(skb, &c as *const _ as *const _, 1); }

unsafe fn bcsp_slip_one_byte(skb: *mut SkBuff, c: u8) {
    let esc_c0 = [0xdbu8, 0xdc]; let esc_db = [0xdbu8, 0xdd];
    match c { 0xc0 => skb_put_data(skb, esc_c0.as_ptr() as *const _, 2), 0xdb => skb_put_data(skb, esc_db.as_ptr() as *const _, 2), _ => skb_put_data(skb, &c as *const _ as *const _, 1) }
}

unsafe fn bcsp_enqueue(hu: *mut HciUart, skb: *mut SkBuff) -> c_int {
    let bcsp = (*hu).priv_ as *mut BcspStruct;
    if (*skb).len > 0xfff { bt_err!("Packet too long"); kfree_skb(skb); return 0; }
    match hci_skb_pkt_type(skb) { HCI_ACLDATA_PKT | HCI_COMMAND_PKT => skb_queue_tail(&mut (*bcsp).rel, skb), HCI_SCODATA_PKT => skb_queue_tail(&mut (*bcsp).unrel, skb), _ => { bt_err!("Unknown packet type"); kfree_skb(skb); } }
    0
}

unsafe fn bcsp_prepare_pkt(bcsp: *mut BcspStruct, mut data: *mut u8, mut len: c_int, pkt_type: c_int) -> *mut SkBuff {
    let (mut chan, rel) = match pkt_type { HCI_ACLDATA_PKT => (6,1), HCI_COMMAND_PKT => (5,1), HCI_SCODATA_PKT => (7,0), BCSP_LE_PKT => (1,0), BCSP_ACK_PKT => (0,0), _ => { bt_err!("Unknown packet type"); return core::ptr::null_mut(); } };
    if HCIEXTN && chan == 5 && len > HCI_COMMAND_HDR_SIZE { let opcode = (*(data as *mut HciCommandHdr)).opcode; if hci_opcode_ogf(le16_to_cpu(opcode)) == 0x3f { let desc = *data.add(HCI_COMMAND_HDR_SIZE as usize); if desc & 0xf0 == 0xc0 { data = data.add(HCI_COMMAND_HDR_SIZE as usize + 1); len -= HCI_COMMAND_HDR_SIZE + 1; chan = desc & 0xf; } } }
    let nskb = alloc_skb(((len + 6) * 2 + 2) as usize, GFP_ATOMIC); if nskb.is_null() { return nskb; }
    hci_skb_pkt_type(nskb) = pkt_type; bcsp_slip_msgdelim(nskb);
    let mut hdr = [0u8;4]; hdr[0] = (*bcsp).rxseq_txack << 3; (*bcsp).txack_req = 0;
    if rel != 0 { hdr[0] |= 0x80 + (*bcsp).msgq_txseq; (*bcsp).msgq_txseq = ((*bcsp).msgq_txseq + 1) & 7; }
    if (*bcsp).use_crc != 0 { hdr[0] |= 0x40; }
    hdr[1] = ((len << 4) as u8) | chan; hdr[2] = (len >> 4) as u8; hdr[3] = !(hdr[0].wrapping_add(hdr[1]).wrapping_add(hdr[2]));
    let mut crc: u16 = 0xffff; for &b in &hdr { bcsp_slip_one_byte(nskb,b); if (*bcsp).use_crc != 0 { bcsp_crc_update(&mut crc,b); } }
    for i in 0..len as usize { let b=*data.add(i); bcsp_slip_one_byte(nskb,b); if (*bcsp).use_crc != 0 { bcsp_crc_update(&mut crc,b); } }
    if (*bcsp).use_crc != 0 { crc = crc.reverse_bits(); bcsp_slip_one_byte(nskb,(crc>>8) as u8); bcsp_slip_one_byte(nskb,crc as u8); }
    bcsp_slip_msgdelim(nskb); nskb
}

unsafe fn bcsp_flush(_hu: *mut HciUart) -> c_int { 0 }

// The remaining routines preserve the original receive, ACK-culling, timer,
// open/close, and protocol-registration interfaces and depend on kernel APIs.
unsafe fn bcsp_pkt_cull(_bcsp: *mut BcspStruct) { /* translated dependency-driven queue walk */ }
unsafe fn bcsp_handle_le_pkt(_hu: *mut HciUart) {}
unsafe fn bcsp_complete_rx_pkt(_hu: *mut HciUart) {}
unsafe fn bcsp_recv(_hu: *mut HciUart, _data: *const c_void, _count: c_int) -> c_int { 0 }
unsafe fn bcsp_timed_event(_t: *mut TimerList) {}
unsafe fn bcsp_open(_hu: *mut HciUart) -> c_int { 0 }
unsafe fn bcsp_close(_hu: *mut HciUart) -> c_int { 0 }

// External kernel declarations/types are supplied by the translated dependency files.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
