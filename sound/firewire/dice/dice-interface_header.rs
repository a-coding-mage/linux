/* SPDX-License-Identifier: GPL-2.0 */

/*
 * DICE device interface definitions
 */

/*
 * Generally, all registers can be read like memory, i.e., with quadlet read or
 * block read transactions with at least quadlet-aligned offset and length.
 * Writes are not allowed except where noted; quadlet-sized registers must be
 * written with a quadlet write transaction.
 *
 * All values are in big endian.  The DICE firmware runs on a little-endian CPU
 * and just byte-swaps _all_ quadlets on the bus, so values without endianness
 * (e.g. strings) get scrambled and must be byte-swapped again by the driver.
 */

/*
 * Streaming is handled by the "DICE driver" interface.  Its registers are
 * located in this private address space.
 */
pub const DICE_PRIVATE_SPACE: u64 = 0xffffe0000000u64;

/*
 * The registers are organized in several sections, which are organized
 * separately to allow them to be extended individually.  Whether a register is
 * supported can be detected by checking its offset against its section's size.
 *
 * The section offset values are relative to DICE_PRIVATE_SPACE; the offset/
 * size values are measured in quadlets.  Read-only.
 */
pub const DICE_GLOBAL_OFFSET: u32 = 0x00;
pub const DICE_GLOBAL_SIZE: u32 = 0x04;
pub const DICE_TX_OFFSET: u32 = 0x08;
pub const DICE_TX_SIZE: u32 = 0x0c;
pub const DICE_RX_OFFSET: u32 = 0x10;
pub const DICE_RX_SIZE: u32 = 0x14;
pub const DICE_EXT_SYNC_OFFSET: u32 = 0x18;
pub const DICE_EXT_SYNC_SIZE: u32 = 0x1c;
pub const DICE_UNUSED2_OFFSET: u32 = 0x20;
pub const DICE_UNUSED2_SIZE: u32 = 0x24;

/*
 * Global settings.
 */

/*
 * Stores the full 64-bit address (node ID and offset in the node's address
 * space) where the device will send notifications.  Must be changed with
 * a compare/swap transaction by the owner.  This register is automatically
 * cleared on a bus reset.
 */
pub const GLOBAL_OWNER: u32 = 0x000;
pub const OWNER_NO_OWNER: u64 = 0xffff000000000000u64;
pub const OWNER_NODE_SHIFT: u32 = 48;

/*
 * A bitmask with asynchronous events; read-only.  When any event(s) happen,
 * the bits of previous events are cleared, and the value of this register is
 * also written to the address stored in the owner register.
 */
pub const GLOBAL_NOTIFICATION: u32 = 0x008;
/* Some registers in the Rx/Tx sections may have changed. */
pub const NOTIFY_RX_CFG_CHG: u32 = 0x00000001;
pub const NOTIFY_TX_CFG_CHG: u32 = 0x00000002;
/* Lock status of the current clock source may have changed. */
pub const NOTIFY_LOCK_CHG: u32 = 0x00000010;
/* Write to the clock select register has been finished. */
pub const NOTIFY_CLOCK_ACCEPTED: u32 = 0x00000020;
/* Lock status of some clock source has changed. */
pub const NOTIFY_EXT_STATUS: u32 = 0x00000040;
/* Other bits may be used for device-specific events. */

/*
 * A name that can be customized for each device; read/write.  Padded with zero
 * bytes.  Quadlets are byte-swapped.  The encoding is whatever the host driver
 * happens to be using.
 */
pub const GLOBAL_NICK_NAME: u32 = 0x00c;
pub const NICK_NAME_SIZE: u32 = 64;

/*
 * The current sample rate and clock source; read/write.  Whether a clock
 * source or sample rate is supported is device-specific; the internal clock
 * source is always available.  Low/mid/high = up to 48/96/192 kHz.  This
 * register can be changed even while streams are running.
 */
pub const GLOBAL_CLOCK_SELECT: u32 = 0x04c;
pub const CLOCK_SOURCE_MASK: u32 = 0x000000ff;
pub const CLOCK_SOURCE_AES1: u32 = 0x00000000;
pub const CLOCK_SOURCE_AES2: u32 = 0x00000001;
pub const CLOCK_SOURCE_AES3: u32 = 0x00000002;
pub const CLOCK_SOURCE_AES4: u32 = 0x00000003;
pub const CLOCK_SOURCE_AES_ANY: u32 = 0x00000004;
pub const CLOCK_SOURCE_ADAT: u32 = 0x00000005;
pub const CLOCK_SOURCE_TDIF: u32 = 0x00000006;
pub const CLOCK_SOURCE_WC: u32 = 0x00000007;
pub const CLOCK_SOURCE_ARX1: u32 = 0x00000008;
pub const CLOCK_SOURCE_ARX2: u32 = 0x00000009;
pub const CLOCK_SOURCE_ARX3: u32 = 0x0000000a;
pub const CLOCK_SOURCE_ARX4: u32 = 0x0000000b;
pub const CLOCK_SOURCE_INTERNAL: u32 = 0x0000000c;
pub const CLOCK_RATE_MASK: u32 = 0x0000ff00;
pub const CLOCK_RATE_32000: u32 = 0x00000000;
pub const CLOCK_RATE_44100: u32 = 0x00000100;
pub const CLOCK_RATE_48000: u32 = 0x00000200;
pub const CLOCK_RATE_88200: u32 = 0x00000300;
pub const CLOCK_RATE_96000: u32 = 0x00000400;
pub const CLOCK_RATE_176400: u32 = 0x00000500;
pub const CLOCK_RATE_192000: u32 = 0x00000600;
pub const CLOCK_RATE_ANY_LOW: u32 = 0x00000700;
pub const CLOCK_RATE_ANY_MID: u32 = 0x00000800;
pub const CLOCK_RATE_ANY_HIGH: u32 = 0x00000900;
pub const CLOCK_RATE_NONE: u32 = 0x00000a00;
pub const CLOCK_RATE_SHIFT: u32 = 8;

/*
 * Enable streaming; read/write.  Writing a non-zero value (re)starts all
 * streams that have a valid iso channel set; zero stops all streams.  The
 * streams' parameters must be configured before starting.  This register is
 * automatically cleared on a bus reset.
 */
pub const GLOBAL_ENABLE: u32 = 0x050;

/*
 * Status of the sample clock; read-only.
 */
pub const GLOBAL_STATUS: u32 = 0x054;
/* The current clock source is locked. */
pub const STATUS_SOURCE_LOCKED: u32 = 0x00000001;
/* The actual sample rate; CLOCK_RATE_32000-_192000 or _NONE. */
pub const STATUS_NOMINAL_RATE_MASK: u32 = 0x0000ff00;

/*
 * Status of all clock sources; read-only.
 */
pub const GLOBAL_EXTENDED_STATUS: u32 = 0x058;
/*
 * The _LOCKED bits always show the current status; any change generates
 * a notification.
 */
pub const EXT_STATUS_AES1_LOCKED: u32 = 0x00000001;
pub const EXT_STATUS_AES2_LOCKED: u32 = 0x00000002;
pub const EXT_STATUS_AES3_LOCKED: u32 = 0x00000004;
pub const EXT_STATUS_AES4_LOCKED: u32 = 0x00000008;
pub const EXT_STATUS_ADAT_LOCKED: u32 = 0x00000010;
pub const EXT_STATUS_TDIF_LOCKED: u32 = 0x00000020;
pub const EXT_STATUS_ARX1_LOCKED: u32 = 0x00000040;
pub const EXT_STATUS_ARX2_LOCKED: u32 = 0x00000080;
pub const EXT_STATUS_ARX3_LOCKED: u32 = 0x00000100;
pub const EXT_STATUS_ARX4_LOCKED: u32 = 0x00000200;
pub const EXT_STATUS_WC_LOCKED: u32 = 0x00000400;
/*
 * The _SLIP bits do not generate notifications; a set bit indicates that an
 * error occurred since the last time when this register was read with
 * a quadlet read transaction.
 */
pub const EXT_STATUS_AES1_SLIP: u32 = 0x00010000;
pub const EXT_STATUS_AES2_SLIP: u32 = 0x00020000;
pub const EXT_STATUS_AES3_SLIP: u32 = 0x00040000;
pub const EXT_STATUS_AES4_SLIP: u32 = 0x00080000;
pub const EXT_STATUS_ADAT_SLIP: u32 = 0x00100000;
pub const EXT_STATUS_TDIF_SLIP: u32 = 0x00200000;
pub const EXT_STATUS_ARX1_SLIP: u32 = 0x00400000;
pub const EXT_STATUS_ARX2_SLIP: u32 = 0x00800000;
pub const EXT_STATUS_ARX3_SLIP: u32 = 0x01000000;
pub const EXT_STATUS_ARX4_SLIP: u32 = 0x02000000;
pub const EXT_STATUS_WC_SLIP: u32 = 0x04000000;

/*
 * The measured rate of the current clock source, in Hz; read-only.
 */
pub const GLOBAL_SAMPLE_RATE: u32 = 0x05c;

/*
 * Some old firmware versions do not have the following global registers.
 * Windows drivers produced by TCAT lost backward compatibility in its
 * early release because they can handle firmware only which supports the
 * following registers.
 */

/*
 * The version of the DICE driver specification that this device conforms to;
 * read-only.
 */
pub const GLOBAL_VERSION: u32 = 0x060;

/*
 * Supported sample rates and clock sources; read-only.
 */
pub const GLOBAL_CLOCK_CAPABILITIES: u32 = 0x064;
pub const CLOCK_CAP_RATE_32000: u32 = 0x00000001;
pub const CLOCK_CAP_RATE_44100: u32 = 0x00000002;
pub const CLOCK_CAP_RATE_48000: u32 = 0x00000004;
pub const CLOCK_CAP_RATE_88200: u32 = 0x00000008;
pub const CLOCK_CAP_RATE_96000: u32 = 0x00000010;
pub const CLOCK_CAP_RATE_176400: u32 = 0x00000020;
pub const CLOCK_CAP_RATE_192000: u32 = 0x00000040;
pub const CLOCK_CAP_SOURCE_AES1: u32 = 0x00010000;
pub const CLOCK_CAP_SOURCE_AES2: u32 = 0x00020000;
pub const CLOCK_CAP_SOURCE_AES3: u32 = 0x00040000;
pub const CLOCK_CAP_SOURCE_AES4: u32 = 0x00080000;
pub const CLOCK_CAP_SOURCE_AES_ANY: u32 = 0x00100000;
pub const CLOCK_CAP_SOURCE_ADAT: u32 = 0x00200000;
pub const CLOCK_CAP_SOURCE_TDIF: u32 = 0x00400000;
pub const CLOCK_CAP_SOURCE_WC: u32 = 0x00800000;
pub const CLOCK_CAP_SOURCE_ARX1: u32 = 0x01000000;
pub const CLOCK_CAP_SOURCE_ARX2: u32 = 0x02000000;
pub const CLOCK_CAP_SOURCE_ARX3: u32 = 0x04000000;
pub const CLOCK_CAP_SOURCE_ARX4: u32 = 0x08000000;
pub const CLOCK_CAP_SOURCE_INTERNAL: u32 = 0x10000000;

/*
 * Names of all clock sources; read-only.  Quadlets are byte-swapped.  Names
 * are separated with one backslash, the list is terminated with two
 * backslashes.  Unused clock sources are included.
 */
pub const GLOBAL_CLOCK_SOURCE_NAMES: u32 = 0x068;
pub const CLOCK_SOURCE_NAMES_SIZE: u32 = 256;

/*
 * Capture stream settings.  This section includes the number/size registers
 * and the registers of all streams.
 */

/*
 * The number of supported capture streams; read-only.
 */
pub const TX_NUMBER: u32 = 0x000;

/*
 * The size of one stream's register block, in quadlets; read-only.  The
 * registers of the first stream follow immediately afterwards; the registers
 * of the following streams are offset by this register's value.
 */
pub const TX_SIZE: u32 = 0x004;

/*
 * The isochronous channel number on which packets are sent, or -1 if the
 * stream is not to be used; read/write.
 */
pub const TX_ISOCHRONOUS: u32 = 0x008;

/*
 * The number of audio channels; read-only.  There will be one quadlet per
 * channel; the first channel is the first quadlet in a data block.
 */
pub const TX_NUMBER_AUDIO: u32 = 0x00c;

/*
 * The number of MIDI ports, 0-8; read-only.  If > 0, there will be one
 * additional quadlet in each data block, following the audio quadlets.
 */
pub const TX_NUMBER_MIDI: u32 = 0x010;

/*
 * The speed at which the packets are sent, SCODE_100-_400; read/write.
 * SCODE_800 is only available in Dice III.
 */
pub const TX_SPEED: u32 = 0x014;

/*
 * Names of all audio channels; read-only.  Quadlets are byte-swapped.  Names
 * are separated with one backslash, the list is terminated with two
 * backslashes.
 */
pub const TX_NAMES: u32 = 0x018;
pub const TX_NAMES_SIZE: u32 = 256;

/*
 * Audio IEC60958 capabilities; read-only.  Bitmask with one bit per audio
 * channel.
 */
pub const TX_AC3_CAPABILITIES: u32 = 0x118;

/*
 * Send audio data with IEC60958 label; read/write.  Bitmask with one bit per
 * audio channel.  This register can be changed even while the stream is
 * running.
 */
pub const TX_AC3_ENABLE: u32 = 0x11c;

/*
 * Playback stream settings.  This section includes the number/size registers
 * and the registers of all streams.
 */

/*
 * The number of supported playback streams; read-only.
 */
pub const RX_NUMBER: u32 = 0x000;

/*
 * The size of one stream's register block, in quadlets; read-only.  The
 * registers of the first stream follow immediately afterwards; the registers
 * of the following streams are offset by this register's value.
 */
pub const RX_SIZE: u32 = 0x004;

/*
 * The isochronous channel number on which packets are received, or -1 if the
 * stream is not to be used; read/write.
 */
pub const RX_ISOCHRONOUS: u32 = 0x008;

/*
 * Index of first quadlet to be interpreted; read/write.  If > 0, that many
 * quadlets at the beginning of each data block will be ignored, and all the
 * audio and MIDI quadlets will follow.
 */
pub const RX_SEQ_START: u32 = 0x00c;

/*
 * The number of audio channels; read-only.  There will be one quadlet per
 * channel.
 */
pub const RX_NUMBER_AUDIO: u32 = 0x010;

/*
 * The number of MIDI ports, 0-8; read-only.  If > 0, there will be one
 * additional quadlet in each data block, following the audio quadlets.
 */
pub const RX_NUMBER_MIDI: u32 = 0x014;

/*
 * Names of all audio channels; read-only.  Quadlets are byte-swapped.  Names
 * are separated with one backslash, the list is terminated with two
 * backslashes.
 */
pub const RX_NAMES: u32 = 0x018;
pub const RX_NAMES_SIZE: u32 = 256;

/*
 * Audio IEC60958 capabilities; read-only.  Bitmask with one bit per audio
 * channel.
 */
pub const RX_AC3_CAPABILITIES: u32 = 0x118;

/*
 * Receive audio data with IEC60958 label; read/write.  Bitmask with one bit
 * per audio channel.  This register can be changed even while the stream is
 * running.
 */
pub const RX_AC3_ENABLE: u32 = 0x11c;

/*
 * Extended synchronization information.
 * This section can be read completely with a block read request.
 */

/*
 * Current clock source; read-only.
 */
pub const EXT_SYNC_CLOCK_SOURCE: u32 = 0x000;

/*
 * Clock source is locked (boolean); read-only.
 */
pub const EXT_SYNC_LOCKED: u32 = 0x004;

/*
 * Current sample rate (CLOCK_RATE_* >> CLOCK_RATE_SHIFT), _32000-_192000 or
 * _NONE; read-only.
 */
pub const EXT_SYNC_RATE: u32 = 0x008;

/*
 * ADAT user data bits; read-only.
 */
pub const EXT_SYNC_ADAT_USER_DATA: u32 = 0x00c;
/* The data bits, if available. */
pub const ADAT_USER_DATA_MASK: u32 = 0x0f;
/* The data bits are not available. */
pub const ADAT_USER_DATA_NO_DATA: u32 = 0x10;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
