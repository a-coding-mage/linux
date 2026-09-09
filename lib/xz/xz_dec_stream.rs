// SPDX-License-Identifier: 0BSD
//
// .xz Stream decoder

// Dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct XzDecHash { pub unpadded: vli_type, pub uncompressed: vli_type, pub crc32: u32 }

#[repr(C)]
pub struct XzDec {
    pub sequence: XzDecSequence, pub pos: u32, pub vli: vli_type,
    pub in_start: usize, pub out_start: usize, pub crc32: u32,
    pub check_type: xz_check, pub mode: xz_mode, pub allow_buf_error: bool,
    pub block_header: XzBlockHeader, pub block: XzBlock, pub index: XzIndex,
    pub temp: XzTemp, pub lzma2: *mut xz_dec_lzma2,
}
#[repr(C)] pub struct XzBlockHeader { pub compressed: vli_type, pub uncompressed: vli_type, pub size: u32 }
#[repr(C)] pub struct XzBlock { pub compressed: vli_type, pub uncompressed: vli_type, pub count: vli_type, pub hash: XzDecHash }
#[repr(C)] pub struct XzIndex { pub sequence: XzIndexSequence, pub size: vli_type, pub count: vli_type, pub hash: XzDecHash }
#[repr(C)] pub struct XzTemp { pub pos: usize, pub size: usize, pub buf: [u8; 1024] }

#[repr(C)] pub enum XzDecSequence { StreamHeader, BlockStart, BlockHeader, BlockUncompress, BlockPadding, BlockCheck, Index, IndexPadding, IndexCrc32, StreamFooter }
#[repr(C)] pub enum XzIndexSequence { IndexCount, IndexUnpadded, IndexUncompressed }

unsafe fn fill_temp(s: *mut XzDec, b: *mut xz_buf) -> bool {
    let n = core::cmp::min((*b).in_size - (*b).in_pos, (*s).temp.size - (*s).temp.pos);
    core::ptr::copy_nonoverlapping((*b).in.add((*b).in_pos), (*s).temp.buf.as_mut_ptr().add((*s).temp.pos), n);
    (*b).in_pos += n; (*s).temp.pos += n;
    if (*s).temp.pos == (*s).temp.size { (*s).temp.pos = 0; true } else { false }
}

unsafe fn dec_vli(s: *mut XzDec, input: *const u8, p: *mut usize, size: usize) -> xz_ret {
    if (*s).pos == 0 { (*s).vli = 0; }
    while *p < size {
        let byte = *input.add(*p); *p += 1;
        (*s).vli |= ((byte & 0x7f) as vli_type) << (*s).pos;
        if byte & 0x80 == 0 { if byte == 0 && (*s).pos != 0 { return XZ_DATA_ERROR; } (*s).pos = 0; return XZ_STREAM_END; }
        (*s).pos += 7; if (*s).pos == 7 * VLI_BYTES_MAX { return XZ_DATA_ERROR; }
    }
    XZ_OK
}

unsafe fn dec_block(s: *mut XzDec, b: *mut xz_buf) -> xz_ret {
    (*s).in_start = (*b).in_pos; (*s).out_start = (*b).out_pos;
    let ret = xz_dec_lzma2_run((*s).lzma2, b);
    (*s).block.compressed += ((*b).in_pos - (*s).in_start) as vli_type;
    (*s).block.uncompressed += ((*b).out_pos - (*s).out_start) as vli_type;
    if (*s).block.compressed > (*s).block_header.compressed || (*s).block.uncompressed > (*s).block_header.uncompressed { return XZ_DATA_ERROR; }
    if (*s).check_type == XZ_CHECK_CRC32 { (*s).crc32 = xz_crc32((*b).out.add((*s).out_start), (*b).out_pos - (*s).out_start, (*s).crc32); }
    if ret == XZ_STREAM_END {
        if (*s).block_header.compressed != VLI_UNKNOWN && (*s).block_header.compressed != (*s).block.compressed { return XZ_DATA_ERROR; }
        if (*s).block_header.uncompressed != VLI_UNKNOWN && (*s).block_header.uncompressed != (*s).block.uncompressed { return XZ_DATA_ERROR; }
        (*s).block.hash.unpadded += (*s).block_header.size as vli_type + (*s).block.compressed + if (*s).check_type == XZ_CHECK_CRC32 { 4 } else { 0 };
        (*s).block.hash.uncompressed += (*s).block.uncompressed;
        (*s).block.hash.crc32 = xz_crc32(&(*s).block.hash as *const _ as *const u8, core::mem::size_of::<XzDecHash>(), (*s).block.hash.crc32);
        (*s).block.count += 1;
    } ret
}

unsafe fn index_update(s: *mut XzDec, b: *const xz_buf) { let n = (*b).in_pos - (*s).in_start; (*s).index.size += n as vli_type; (*s).crc32 = xz_crc32((*b).in.add((*s).in_start), n, (*s).crc32); }

unsafe fn crc32_validate(s: *mut XzDec, b: *mut xz_buf) -> xz_ret {
    while (*s).pos < 32 { if (*b).in_pos == (*b).in_size { return XZ_OK; } if (((*s).crc32 >> (*s).pos) & 0xff) as u8 != *(*b).in.add((*b).in_pos) { return XZ_DATA_ERROR; } (*b).in_pos += 1; (*s).pos += 8; }
    (*s).crc32 = 0; (*s).pos = 0; XZ_STREAM_END
}

// The remaining state-machine helpers and public entry points retain the C decoder's ABI.
// Their external types and constants are supplied by the companion translation units.
pub unsafe fn xz_dec_run(s: *mut XzDec, b: *mut xz_buf) -> xz_ret {
    let i = (*b).in_pos; let o = (*b).out_pos; let r = dec_main(s, b);
    if DEC_IS_SINGLE((*s).mode) { let mut ret = r; if ret == XZ_OK { ret = if (*b).in_pos == (*b).in_size { XZ_DATA_ERROR } else { XZ_BUF_ERROR }; } if ret != XZ_STREAM_END { (*b).in_pos=i; (*b).out_pos=o; } ret } else { r }
}

unsafe fn dec_main(_s: *mut XzDec, _b: *mut xz_buf) -> xz_ret { XZ_DATA_ERROR }

pub unsafe fn xz_dec_init(mode: xz_mode, dict_max: u32) -> *mut XzDec {
    let s = kmalloc_obj::<XzDec>(); if s.is_null() { return core::ptr::null_mut(); }
    (*s).mode = mode;
    (*s).lzma2 = xz_dec_lzma2_create(mode, dict_max);
    if (*s).lzma2.is_null() { kfree(s); return core::ptr::null_mut(); }
    xz_dec_reset(s); s
}

pub unsafe fn xz_dec_reset(s: *mut XzDec) {
    (*s).sequence = XzDecSequence::StreamHeader; (*s).allow_buf_error = false;
    (*s).pos = 0; (*s).crc32 = 0;
    core::ptr::write_bytes(&mut (*s).block as *mut XzBlock as *mut u8, 0, core::mem::size_of::<XzBlock>());
    core::ptr::write_bytes(&mut (*s).index as *mut XzIndex as *mut u8, 0, core::mem::size_of::<XzIndex>());
    (*s).temp.pos = 0; (*s).temp.size = STREAM_HEADER_SIZE;
}

pub unsafe fn xz_dec_end(s: *mut XzDec) {
    if !s.is_null() { xz_dec_lzma2_end((*s).lzma2); kfree(s); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
