// SPDX-License-Identifier: GPL-2.0-or-later
/* Windows System Compression (WOF) decompression glue. */

// Kernel and NTFS dependencies are supplied by the surrounding translation.

const WOF_NAME: [u16; 17] = [
    'W' as u16, 'o' as u16, 'f' as u16, 'C' as u16, 'o' as u16, 'm' as u16,
    'p' as u16, 'r' as u16, 'e' as u16, 's' as u16, 's' as u16, 'e' as u16,
    'd' as u16, 'D' as u16, 'a' as u16, 't' as u16, 'a' as u16,
];
const WOF_NAME_LEN: usize = 17;
const NTFS_WOF_MAX_COMP_UNIT: usize = 1usize << 15;
const NTFS_WOF_MAX_PAGES: usize = (NTFS_WOF_MAX_COMP_UNIT + PAGE_SIZE - 1) / PAGE_SIZE;

#[repr(C)]
struct NtfsWofWorkspace { lock: *mut Mutex, codec: *const NtfsCodecOps, comp_unit: u32, input: *mut c_void, input_size: usize, output: *mut c_void, scratch: *mut c_void }

static mut NTFS_WOF_XPRESS4K_WORKSPACE: NtfsWofWorkspace = NtfsWofWorkspace { lock: core::ptr::null_mut(), codec: core::ptr::null(), comp_unit: 1 << 12, input: core::ptr::null_mut(), input_size: 0, output: core::ptr::null_mut(), scratch: core::ptr::null_mut() };
static mut NTFS_WOF_XPRESS8K_WORKSPACE: NtfsWofWorkspace = NtfsWofWorkspace { lock: core::ptr::null_mut(), codec: core::ptr::null(), comp_unit: 1 << 13, input: core::ptr::null_mut(), input_size: 0, output: core::ptr::null_mut(), scratch: core::ptr::null_mut() };
static mut NTFS_WOF_XPRESS16K_WORKSPACE: NtfsWofWorkspace = NtfsWofWorkspace { lock: core::ptr::null_mut(), codec: core::ptr::null(), comp_unit: 1 << 14, input: core::ptr::null_mut(), input_size: 0, output: core::ptr::null_mut(), scratch: core::ptr::null_mut() };
static mut NTFS_WOF_LZX32K_WORKSPACE: NtfsWofWorkspace = NtfsWofWorkspace { lock: core::ptr::null_mut(), codec: core::ptr::null(), comp_unit: 1 << 15, input: core::ptr::null_mut(), input_size: 0, output: core::ptr::null_mut(), scratch: core::ptr::null_mut() };

unsafe fn ntfs_wof_workspace(bits: u8) -> *mut NtfsWofWorkspace {
    match bits { 12 => &mut NTFS_WOF_XPRESS4K_WORKSPACE, 13 => &mut NTFS_WOF_XPRESS8K_WORKSPACE, 14 => &mut NTFS_WOF_XPRESS16K_WORKSPACE, 15 => &mut NTFS_WOF_LZX32K_WORKSPACE, _ => core::ptr::null_mut() }
}

unsafe fn ntfs_wof_workspace_prepare(ws: *mut NtfsWofWorkspace) -> i32 {
    if !(*ws).input.is_null() { return 0; }
    (*ws).input_size = ((*ws).comp_unit as usize + 511) & !511;
    let scratch_size = ((*(*ws).codec).scratch_size)((*ws).comp_unit);
    if scratch_size == 0 { return -22; }
    let input = kvmalloc((*ws).input_size, GFP_NOFS);
    let output = kvmalloc((*ws).comp_unit as usize, GFP_NOFS);
    let scratch = kvzalloc(scratch_size, GFP_NOFS);
    if input.is_null() || output.is_null() || scratch.is_null() { kvfree(input); kvfree(output); kvfree(scratch); return -12; }
    (*ws).input = input; (*ws).output = output; (*ws).scratch = scratch; 0
}

pub unsafe fn ntfs_wof_free_workspaces() {
    for ws in [&mut NTFS_WOF_XPRESS4K_WORKSPACE, &mut NTFS_WOF_XPRESS8K_WORKSPACE, &mut NTFS_WOF_XPRESS16K_WORKSPACE, &mut NTFS_WOF_LZX32K_WORKSPACE] {
        mutex_lock((*ws).lock); kvfree((*ws).input); kvfree((*ws).output); kvfree((*ws).scratch);
        (*ws).input = core::ptr::null_mut(); (*ws).output = core::ptr::null_mut(); (*ws).scratch = core::ptr::null_mut(); mutex_unlock((*ws).lock);
    }
}

unsafe fn ntfs_wof_decode(ws: *mut NtfsWofWorkspace, src: *const c_void, src_len: u32, dst: *mut c_void, dst_len: u32) -> i32 {
    if src_len == dst_len { core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, dst_len as usize); return 0; }
    ((*(*ws).codec).decompress_chunk)((*ws).scratch, src, src_len, dst, dst_len, (*ws).comp_unit)
}

// The following routines retain the source control flow and call the corresponding
// kernel/NTFS primitives supplied by the surrounding Rust translation.
pub unsafe fn ntfs_read_wof_compressed_block(folio: *mut Folio) -> i32 {
    let mapping = (*folio).mapping;
    let ni = NTFS_I((*mapping).host);
    let ws = ntfs_wof_workspace((*ni).itype.compressed.block_size_bits);
    if ws.is_null() { folio_clear_uptodate(folio); folio_unlock(folio); return -95; }
    let size = i_size_read(VFS_I(ni));
    let start = folio_pos(folio);
    if start >= size { folio_zero_segment(folio, 0, folio_size(folio)); flush_dcache_folio(folio); folio_mark_uptodate(folio); folio_unlock(folio); return 0; }
    mutex_lock((*ws).lock);
    let mut err = ntfs_wof_workspace_prepare(ws);
    if err == 0 {
        // Full chunk-table parsing, resident/nonresident reads, direct folio mapping,
        // decompression, and tail zeroing are delegated to the translated helpers.
        err = ntfs_wof_read_and_decode_chunks(ni, mapping, folio, ws, size);
    }
    mutex_unlock((*ws).lock);
    if err == 0 { flush_dcache_folio(folio); folio_mark_uptodate(folio); } else { folio_clear_uptodate(folio); }
    folio_unlock(folio); err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
