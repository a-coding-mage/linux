/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_int, c_uint, c_ulong, c_void};

/* Dependencies from Linux/firewire/sound headers included by the original C header. */
pub type u32 = u32;
pub type __be32 = u32;
pub type snd_pcm_uframes_t = c_ulong;

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_iso_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iso_packets_buffer {
    _private: [u8; 0],
}

/**
 * enum cip_flags - describes details of the streaming protocol
 * @CIP_NONBLOCKING: In non-blocking mode, each packet contains
 *	sample_rate/8000 samples, with rounding up or down to adjust
 *	for clock skew and left-over fractional samples.  This should
 *	be used if supported by the device.
 * @CIP_BLOCKING: In blocking mode, each packet contains either zero or
 *	SYT_INTERVAL samples, with these two types alternating so that
 *	the overall sample rate comes out right.
 * @CIP_EMPTY_WITH_TAG0: Only for in-stream. Empty in-packets have TAG0.
 * @CIP_DBC_IS_END_EVENT: The value of dbc in an packet corresponds to the end
 * of event in the packet. Out of IEC 61883.
 * @CIP_WRONG_DBS: Only for in-stream. The value of dbs is wrong in in-packets.
 *	The value of data_block_quadlets is used instead of reported value.
 * @CIP_SKIP_DBC_ZERO_CHECK: Only for in-stream.  Packets with zero in dbc is
 *	skipped for detecting discontinuity.
 * @CIP_EMPTY_HAS_WRONG_DBC: Only for in-stream. The value of dbc in empty
 *	packet is wrong but the others are correct.
 * @CIP_JUMBO_PAYLOAD: Only for in-stream. The number of data blocks in an
 *	packet is larger than IEC 61883-6 defines. Current implementation
 *	allows 5 times as large as IEC 61883-6 defines.
 * @CIP_HEADER_WITHOUT_EOH: Only for in-stream. CIP Header doesn't include
 *	valid EOH.
 * @CIP_NO_HEADER: a lack of headers in packets
 * @CIP_UNALIGHED_DBC: Only for in-stream. The value of dbc is not alighed to
 *	the value of current SYT_INTERVAL; e.g. initial value is not zero.
 * @CIP_UNAWARE_SYT: For outgoing packet, the value in SYT field of CIP is 0xffff.
 *	For incoming packet, the value in SYT field of CIP is not handled.
 * @CIP_DBC_IS_PAYLOAD_QUADLETS: Available for incoming packet, and only effective with
 *	CIP_DBC_IS_END_EVENT flag. The value of dbc field is the number of accumulated quadlets
 *	in CIP payload, instead of the number of accumulated data blocks.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cip_flags {
    CIP_NONBLOCKING = 0x00,
    CIP_BLOCKING = 0x01,
    CIP_EMPTY_WITH_TAG0 = 0x02,
    CIP_DBC_IS_END_EVENT = 0x04,
    CIP_WRONG_DBS = 0x08,
    CIP_SKIP_DBC_ZERO_CHECK = 0x10,
    CIP_EMPTY_HAS_WRONG_DBC = 0x20,
    CIP_JUMBO_PAYLOAD = 0x40,
    CIP_HEADER_WITHOUT_EOH = 0x80,
    CIP_NO_HEADER = 0x100,
    CIP_UNALIGHED_DBC = 0x200,
    CIP_UNAWARE_SYT = 0x400,
    CIP_DBC_IS_PAYLOAD_QUADLETS = 0x800,
}

/**
 * enum cip_sfc - supported Sampling Frequency Codes (SFCs)
 * @CIP_SFC_32000:   32,000 data blocks
 * @CIP_SFC_44100:   44,100 data blocks
 * @CIP_SFC_48000:   48,000 data blocks
 * @CIP_SFC_88200:   88,200 data blocks
 * @CIP_SFC_96000:   96,000 data blocks
 * @CIP_SFC_176400: 176,400 data blocks
 * @CIP_SFC_192000: 192,000 data blocks
 * @CIP_SFC_COUNT: the number of supported SFCs
 *
 * These values are used to show nominal Sampling Frequency Code in
 * Format Dependent Field (FDF) of AMDTP packet header. In IEC 61883-6:2002,
 * this code means the number of events per second. Actually the code
 * represents the number of data blocks transferred per second in an AMDTP
 * stream.
 *
 * In IEC 61883-6:2005, some extensions were added to support more types of
 * data such as 'One Bit LInear Audio', therefore the meaning of SFC became
 * different depending on the types.
 *
 * Currently our implementation is compatible with IEC 61883-6:2002.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cip_sfc {
    CIP_SFC_32000 = 0,
    CIP_SFC_44100 = 1,
    CIP_SFC_48000 = 2,
    CIP_SFC_88200 = 3,
    CIP_SFC_96000 = 4,
    CIP_SFC_176400 = 5,
    CIP_SFC_192000 = 6,
    CIP_SFC_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amdtp_stream_direction {
    AMDTP_OUT_STREAM = 0,
    AMDTP_IN_STREAM,
}

#[repr(C)]
pub struct pkt_desc {
    pub cycle: u32,
    pub syt: u32,
    pub data_blocks: c_uint,
    pub data_block_counter: c_uint,
    pub ctx_payload: *mut __be32,
    pub link: list_head,
}

pub type amdtp_stream_process_ctx_payloads_t = Option<
    unsafe extern "C" fn(
        s: *mut amdtp_stream,
        desc: *const pkt_desc,
        count: c_uint,
        pcm: *mut snd_pcm_substream,
    ),
>;

#[repr(C)]
pub struct seq_desc {
    pub syt_offset: c_uint,
    pub data_blocks: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdtp_stream_ctx_cache {
    pub descs: *mut seq_desc,
    pub size: c_uint,
    pub pos: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdtp_stream_tx {
    pub ctx_header_size: c_uint,

    /* limit for payload of iso packet. */
    pub max_ctx_payload_length: c_uint,

    /*
     * For quirks of CIP headers.
     * Fixed interval of dbc between previos/current packets.
     */
    pub dbc_interval: c_uint,

    /* The device starts multiplexing events to the packet. */
    pub event_starts: bool,

    pub cache: amdtp_stream_ctx_cache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdtp_stream_rx {
    /* To generate CIP header. */
    pub fdf: c_uint,

    /* To generate constant hardware IRQ. */
    pub event_count: c_uint,

    /* To calculate CIP data blocks and tstamp. */
    pub seq: amdtp_stream_ctx_cache,

    pub data_block_state: c_uint,
    pub syt_offset_state: c_uint,
    pub last_syt_offset: c_uint,

    pub replay_target: *mut amdtp_stream,
    pub cache_pos: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union amdtp_stream_ctx_data {
    pub tx: amdtp_stream_tx,
    pub rx: amdtp_stream_rx,
}

#[repr(C)]
pub struct amdtp_stream {
    pub unit: *mut fw_unit,
    /* The combination of cip_flags enumeration-constants. */
    pub flags: c_uint,
    pub direction: amdtp_stream_direction,
    pub mutex: mutex,

    /* For packet processing. */
    pub context: *mut fw_iso_context,
    pub buffer: iso_packets_buffer,
    pub queue_size: c_uint,
    pub packet_index: c_int,
    pub packet_descs: *mut pkt_desc,
    pub packet_descs_list: list_head,
    pub packet_descs_cursor: *mut pkt_desc,
    pub tag: c_int,
    pub ctx_data: amdtp_stream_ctx_data,

    /* For CIP headers. */
    pub source_node_id_field: c_uint,
    pub data_block_quadlets: c_uint,
    pub data_block_counter: c_uint,
    pub sph: c_uint,
    pub fmt: c_uint,

    /* Internal flags. */
    pub transfer_delay: c_uint,
    pub sfc: cip_sfc,
    pub syt_interval: c_uint,

    /* For a PCM substream processing. */
    pub pcm: *mut snd_pcm_substream,
    pub period_work: work_struct,
    pub pcm_buffer_pointer: snd_pcm_uframes_t,
    pub pcm_period_pointer: c_uint,
    pub pcm_frame_multiplier: c_uint,

    /*
     * To start processing content of packets at the same cycle in several contexts for
     * each direction.
     */
    pub ready_processing: bool,
    pub ready_wait: wait_queue_head_t,
    pub next_cycle: c_uint,

    /* For backends to process data blocks. */
    pub protocol: *mut c_void,
    pub process_ctx_payloads: amdtp_stream_process_ctx_payloads_t,

    /* For domain. */
    pub channel: c_int,
    pub speed: c_int,
    pub list: list_head,
    pub domain: *mut amdtp_domain,
}

#[repr(C)]
pub struct amdtp_domain_processing_cycle {
    pub tx_init_skip: c_uint,
    pub tx_start: c_uint,
    pub rx_start: c_uint,
}

#[repr(C)]
pub struct amdtp_domain_replay {
    /* C bitfields: bool enable:1; bool on_the_fly:1; */
    pub _bitfield: u8,
}

#[repr(C)]
pub struct amdtp_domain {
    pub streams: list_head,

    pub events_per_period: c_uint,
    pub events_per_buffer: c_uint,

    pub irq_target: *mut amdtp_stream,

    pub processing_cycle: amdtp_domain_processing_cycle,

    pub replay: amdtp_domain_replay,
}

unsafe extern "C" {
    pub fn amdtp_stream_init(
        s: *mut amdtp_stream,
        unit: *mut fw_unit,
        dir: amdtp_stream_direction,
        flags: c_uint,
        fmt: c_uint,
        process_ctx_payloads: amdtp_stream_process_ctx_payloads_t,
        protocol_size: c_uint,
    ) -> c_int;
    pub fn amdtp_stream_destroy(s: *mut amdtp_stream);

    pub fn amdtp_stream_set_parameters(
        s: *mut amdtp_stream,
        rate: c_uint,
        data_block_quadlets: c_uint,
        pcm_frame_multiplier: c_uint,
    ) -> c_int;
    pub fn amdtp_stream_get_max_payload(s: *mut amdtp_stream) -> c_uint;

    pub fn amdtp_stream_update(s: *mut amdtp_stream);

    pub fn amdtp_stream_add_pcm_hw_constraints(
        s: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> c_int;

    pub fn amdtp_stream_pcm_prepare(s: *mut amdtp_stream);
    pub fn amdtp_stream_pcm_abort(s: *mut amdtp_stream);

    pub static amdtp_syt_intervals: [c_uint; cip_sfc::CIP_SFC_COUNT as usize];
    pub static amdtp_rate_table: [c_uint; cip_sfc::CIP_SFC_COUNT as usize];

    pub fn amdtp_domain_init(d: *mut amdtp_domain) -> c_int;
    pub fn amdtp_domain_destroy(d: *mut amdtp_domain);

    pub fn amdtp_domain_add_stream(
        d: *mut amdtp_domain,
        s: *mut amdtp_stream,
        channel: c_int,
        speed: c_int,
    ) -> c_int;

    pub fn amdtp_domain_start(
        d: *mut amdtp_domain,
        tx_init_skip_cycles: c_uint,
        replay_seq: bool,
        replay_on_the_fly: bool,
    ) -> c_int;
    pub fn amdtp_domain_stop(d: *mut amdtp_domain);

    pub fn amdtp_domain_stream_pcm_pointer(
        d: *mut amdtp_domain,
        s: *mut amdtp_stream,
    ) -> c_ulong;
    pub fn amdtp_domain_stream_pcm_ack(d: *mut amdtp_domain, s: *mut amdtp_stream) -> c_int;

    fn IS_ERR(ptr: *const c_void) -> bool;
}

/**
 * amdtp_stream_running - check stream is running or not
 * @s: the AMDTP stream
 *
 * If this function returns true, the stream is running.
 */
pub unsafe fn amdtp_stream_running(s: *mut amdtp_stream) -> bool {
    unsafe { !IS_ERR((*s).context.cast::<c_void>()) }
}

/**
 * amdtp_streaming_error - check for streaming error
 * @s: the AMDTP stream
 *
 * If this function returns true, the stream's packet queue has stopped due to
 * an asynchronous error.
 */
pub unsafe fn amdtp_streaming_error(s: *mut amdtp_stream) -> bool {
    unsafe { (*s).packet_index < 0 }
}

/**
 * amdtp_stream_pcm_running - check PCM substream is running or not
 * @s: the AMDTP stream
 *
 * If this function returns true, PCM substream in the AMDTP stream is running.
 */
pub unsafe fn amdtp_stream_pcm_running(s: *mut amdtp_stream) -> bool {
    unsafe { !(*s).pcm.is_null() }
}

/**
 * amdtp_stream_pcm_trigger - start/stop playback from a PCM device
 * @s: the AMDTP stream
 * @pcm: the PCM device to be started, or %NULL to stop the current device
 *
 * Call this function on a running isochronous stream to enable the actual
 * transmission of PCM data.  This function should be called from the PCM
 * device's .trigger callback.
 */
pub unsafe fn amdtp_stream_pcm_trigger(s: *mut amdtp_stream, pcm: *mut snd_pcm_substream) {
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*s).pcm), pcm);
    }
}

/**
 * amdtp_stream_next_packet_desc - retrieve next descriptor for amdtp packet.
 * @s: the AMDTP stream
 * @desc: the descriptor of packet
 *
 * This macro computes next descriptor so that the list of descriptors behaves circular queue.
 */
pub unsafe fn amdtp_stream_next_packet_desc(
    _s: *mut amdtp_stream,
    _desc: *mut pkt_desc,
) -> *mut pkt_desc {
    /*
     * Original macro:
     * list_next_entry_circular(desc, &s->packet_descs_list, link)
     *
     * The list_next_entry_circular/list_head implementation is supplied by
     * Linux list macros outside this isolated header.
     */
    todo!("requires Linux list_next_entry_circular macro semantics")
}

pub fn cip_sfc_is_base_44100(sfc: cip_sfc) -> bool {
    (sfc as c_uint & 1) != 0
}

pub unsafe fn amdtp_domain_set_events_per_period(
    d: *mut amdtp_domain,
    events_per_period: c_uint,
    events_per_buffer: c_uint,
) -> c_int {
    unsafe {
        (*d).events_per_period = events_per_period;
        (*d).events_per_buffer = events_per_buffer;
    }

    0
}

/**
 * amdtp_domain_wait_ready - sleep till being ready to process packets or timeout
 * @d: the AMDTP domain
 * @timeout_ms: msec till timeout
 *
 * If this function return false, the AMDTP domain should be stopped.
 */
pub unsafe fn amdtp_domain_wait_ready(_d: *mut amdtp_domain, _timeout_ms: c_uint) -> bool {
    /*
     * Original inline function iterates with:
     * list_for_each_entry(s, &d->streams, list)
     * and waits with:
     * wait_event_interruptible_timeout(s->ready_wait, s->ready_processing, msecs_to_jiffies(timeout_ms))
     *
     * These are Linux list/waitqueue macros outside this isolated header.
     */
    todo!("requires Linux list_for_each_entry, msecs_to_jiffies, and wait_event_interruptible_timeout semantics")
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
