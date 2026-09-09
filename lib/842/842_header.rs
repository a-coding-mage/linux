/* SPDX-License-Identifier: GPL-2.0 */

/* The 842 compressed format is made up of multiple blocks, each of
 * which have the format:
 *
 * <template>[arg1][arg2][arg3][arg4]
 *
 * where there are between 0 and 4 template args, depending on the specific
 * template operation.  For normal operations, each arg is either a specific
 * number of data bytes to add to the output buffer, or an index pointing
 * to a previously-written number of data bytes to copy to the output buffer.
 *
 * The template code is a 5-bit value.  This code indicates what to do with
 * the following data.  Template codes from 0 to 0x19 should use the template
 * table, the static "decomp_ops" table used in decompress.  For each template
 * (table row), there are between 1 and 4 actions; each action corresponds to
 * an arg following the template code bits.  Each action is either a "data"
 * type action, or a "index" type action, and each action results in 2, 4, or 8
 * bytes being written to the output buffer.  Each template (i.e. all actions
 * in the table row) will add up to 8 bytes being written to the output buffer.
 * Any row with less than 4 actions is padded with noop actions, indicated by
 * N0 (for which there is no corresponding arg in the compressed data buffer).
 *
 * "Data" actions, indicated in the table by D2, D4, and D8, mean that the
 * corresponding arg is 2, 4, or 8 bytes, respectively, in the compressed data
 * buffer should be copied directly to the output buffer.
 *
 * "Index" actions, indicated in the table by I2, I4, and I8, mean the
 * corresponding arg is an index parameter that points to, respectively, a 2,
 * 4, or 8 byte value already in the output buffer, that should be copied to
 * the end of the output buffer.  Essentially, the index points to a position
 * in a ring buffer that contains the last N bytes of output buffer data.
 * The number of bits for each index's arg are: 8 bits for I2, 9 bits for I4,
 * and 8 bits for I8.  Since each index points to a 2, 4, or 8 byte section,
 * this means that I2 can reference 512 bytes ((2^8 bits = 256) * 2 bytes), I4
 * can reference 2048 bytes ((2^9 = 512) * 4 bytes), and I8 can reference 2048
 * bytes ((2^8 = 256) * 8 bytes).  Think of it as a kind-of ring buffer for
 * each of I2, I4, and I8 that are updated for each byte written to the output
 * buffer.  In this implementation, the output buffer is directly used for each
 * index; there is no additional memory required.  Note that the index is into
 * a ring buffer, not a sliding window; for example, if there have been 260
 * bytes written to the output buffer, an I2 index of 0 would index to byte 256
 * in the output buffer, while an I2 index of 16 would index to byte 16 in the
 * output buffer.
 *
 * There are also 3 special template codes; 0x1b for "repeat", 0x1c for
 * "zeros", and 0x1e for "end".  The "repeat" operation is followed by a 6 bit
 * arg N indicating how many times to repeat.  The last 8 bytes written to the
 * output buffer are written again to the output buffer, N + 1 times.  The
 * "zeros" operation, which has no arg bits, writes 8 zeros to the output
 * buffer.  The "end" operation, which also has no arg bits, signals the end
 * of the compressed data.  There may be some number of padding (don't care,
 * but usually 0) bits after the "end" operation bits, to fill the buffer
 * length to a specific byte multiple (usually a multiple of 8, 16, or 32
 * bytes).
 *
 * This software implementation also uses one of the undefined template values,
 * 0x1d as a special "short data" template code, to represent less than 8 bytes
 * of uncompressed data.  It is followed by a 3 bit arg N indicating how many
 * data bytes will follow, and then N bytes of data, which should be copied to
 * the output buffer.  This allows the software 842 compressor to accept input
 * buffers that are not an exact multiple of 8 bytes long.  However, those
 * compressed buffers containing this sw-only template will be rejected by the
 * 842 hardware decompressor, and must be decompressed with this software
 * library.  The 842 software compression module includes a parameter to
 * disable using this sw-only "short data" template, and instead simply
 * reject any input buffer that is not a multiple of 8 bytes long.
 *
 * After all actions for each operation code are processed, another template
 * code is in the next 5 bits.  The decompression ends once the "end" template
 * code is detected.
 */

// Dependencies supplied by the surrounding Linux/kernel translation.

/* special templates */
pub const OP_REPEAT: i32 = 0x1B;
pub const OP_ZEROS: i32 = 0x1C;
pub const OP_END: i32 = 0x1E;

/* sw only template - this is not in the hw design; it's used only by this
 * software compressor and decompressor, to allow input buffers that aren't a
 * multiple of 8.
 */
pub const OP_SHORT_DATA: i32 = 0x1D;

/* additional bits of each op param */
pub const OP_BITS: i32 = 5;
pub const REPEAT_BITS: i32 = 6;
pub const SHORT_DATA_BITS: i32 = 3;
pub const I2_BITS: i32 = 8;
pub const I4_BITS: i32 = 9;
pub const I8_BITS: i32 = 8;
pub const CRC_BITS: i32 = 32;

pub const REPEAT_BITS_MAX: i32 = 0x3f;
pub const SHORT_DATA_BITS_MAX: i32 = 0x7;

/* Arbitrary values used to indicate action */
pub const OP_ACTION: i32 = 0x70;
pub const OP_ACTION_INDEX: i32 = 0x10;
pub const OP_ACTION_DATA: i32 = 0x20;
pub const OP_ACTION_NOOP: i32 = 0x40;
pub const OP_AMOUNT: i32 = 0x0f;
pub const OP_AMOUNT_0: i32 = 0x00;
pub const OP_AMOUNT_2: i32 = 0x02;
pub const OP_AMOUNT_4: i32 = 0x04;
pub const OP_AMOUNT_8: i32 = 0x08;

pub const D2: i32 = OP_ACTION_DATA | OP_AMOUNT_2;
pub const D4: i32 = OP_ACTION_DATA | OP_AMOUNT_4;
pub const D8: i32 = OP_ACTION_DATA | OP_AMOUNT_8;
pub const I2: i32 = OP_ACTION_INDEX | OP_AMOUNT_2;
pub const I4: i32 = OP_ACTION_INDEX | OP_AMOUNT_4;
pub const I8: i32 = OP_ACTION_INDEX | OP_AMOUNT_8;
pub const N0: i32 = OP_ACTION_NOOP | OP_AMOUNT_0;

/* the max of the regular templates - not including the special templates */
pub const OPS_MAX: i32 = 0x1a;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
