// SPDX-License-Identifier: GPL-2.0-or-later
/* linux/fs/isofs/compress.c -- transparent zisofs decompression */

// Kernel headers and symbols referenced below are supplied by the surrounding
// translation unit/build; C preprocessor configuration is intentionally kept
// as Rust-side external dependencies.

use core::ffi::c_void;

static mut ZISOFS_SINK_PAGE: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
static mut ZISOFS_ZLIB_WORKSPACE: *mut c_void = core::ptr::null_mut();

#[repr(C)]
pub struct z_stream {
    pub next_in: *mut u8,
    pub avail_in: u32,
    pub total_in: u64,
    pub next_out: *mut u8,
    pub avail_out: u32,
    pub total_out: u64,
    pub msg: *mut u8,
    pub state: *mut c_void,
    pub zalloc: *mut c_void,
    pub zfree: *mut c_void,
    pub opaque: *mut c_void,
    pub data_type: i32,
    pub adler: u64,
    pub reserved: u64,
    pub workspace: *mut c_void,
}

extern "C" {
    fn deflateBound(source_len: usize) -> usize;
    fn zlib_inflate_init(stream: *mut z_stream) -> i32;
    fn zlib_inflate(stream: *mut z_stream, flush: i32) -> i32;
    fn zlib_inflate_end(stream: *mut z_stream) -> i32;
    fn zlib_inflate_workspacesize() -> usize;
    fn vmalloc(size: usize) -> *mut c_void;
    fn vfree(ptr: *mut c_void);
    fn kzalloc_objs<T>(count: usize) -> *mut T;
    fn kfree(ptr: *mut c_void);
    fn isofs_get_blocks(inode: *mut inode, blocknum: u64, bhs: *mut *mut buffer_head, count: i32) -> i32;
    fn bh_read_batch(count: i32, bhs: *mut *mut buffer_head);
    fn wait_on_buffer(bh: *mut buffer_head);
    fn buffer_uptodate(bh: *mut buffer_head) -> bool;
    fn isofs_bread(inode: *mut inode, block: u64) -> *mut buffer_head;
    fn brelse(bh: *mut buffer_head);
    fn memzero_page(page: *mut page, offset: usize, length: usize);
    fn set_page_uptodate(page: *mut page);
    fn flush_dcache_page(page: *mut page);
    fn unlock_page(page: *mut page);
    fn put_page(page: *mut page);
    fn grab_cache_page_nowait(mapping: *mut address_space, index: u64) -> *mut page;
    fn kmap_local_page(page: *mut page) -> *mut u8;
    fn kunmap_local(addr: *mut u8);
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn printk_debug(fmt: *const u8, ...);
    fn folio_end_read(folio: *mut folio, error: bool);
    fn folio_unlock(folio: *mut folio);
}

// External kernel types/constants/macros are provided by isofs and the kernel.
extern "C" {
    static mut zisofs_zlib_lock: c_void;
}

#[repr(C)] pub struct inode { pub i_size: i64, pub i_ino: u64, pub i_mapping: *mut address_space }
#[repr(C)] pub struct file;
#[repr(C)] pub struct folio { pub index: u64, pub page: page }
#[repr(C)] pub struct page;
#[repr(C)] pub struct address_space;
#[repr(C)] pub struct buffer_head { pub b_data: *mut u8 }
#[repr(C)] pub struct address_space_operations { pub read_folio: Option<unsafe extern "C" fn(*mut file, *mut folio) -> i32> }

const PAGE_SHIFT: usize = 12;
const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
const PAGE_MASK: u64 = !(PAGE_SIZE as u64 - 1);
const EIO: i32 = 5;
const ENOMEM: i32 = 12;
const Z_OK: i32 = 0;
const Z_MEM_ERROR: i32 = -4;
const Z_BUF_ERROR: i32 = -5;
const Z_STREAM_END: i32 = 1;
const Z_SYNC_FLUSH: i32 = 2;

unsafe fn zisofs_uncompress_block(inode: *mut inode, mut block_start: i64, block_end: i64,
    pcount: i32, pages: *mut *mut page, mut poffset: usize, errp: *mut i32) -> i64 {
    let zisofs_block_shift = isofs_i(inode, 1);
    let bufsize = isofs_buffer_size(inode);
    let bufshift = isofs_buffer_bits(inode);
    let bufmask = bufsize - 1;
    let mut block_size = block_end - block_start;
    let mut stream = z_stream { next_in: core::ptr::null_mut(), avail_in: 0, total_in: 0,
        next_out: core::ptr::null_mut(), avail_out: 0, total_out: 0, msg: core::ptr::null_mut(),
        state: core::ptr::null_mut(), zalloc: core::ptr::null_mut(), zfree: core::ptr::null_mut(),
        opaque: core::ptr::null_mut(), data_type: 0, adler: 0, reserved: 0,
        workspace: ZISOFS_ZLIB_WORKSPACE };
    let needblocks = ((block_size + (block_start & bufmask as i64) + bufmask as i64) >> bufshift) as i32;
    if block_size as usize > deflateBound(1usize << zisofs_block_shift) { *errp = -EIO; return 0; }
    if block_size == 0 {
        for i in 0..pcount { let off = if i != 0 { 0 } else { poffset }; let p = *pages.add(i as usize); if !p.is_null() { memzero_page(p, off, PAGE_SIZE-off); set_page_uptodate(p); } }
        return ((pcount as i64) << PAGE_SHIFT) - poffset as i64;
    }
    let blocknum = (block_start as u64) >> bufshift;
    let bhs = kzalloc_objs::<*mut buffer_head>((needblocks + 1) as usize);
    if bhs.is_null() { *errp = -ENOMEM; return 0; }
    let haveblocks = isofs_get_blocks(inode, blocknum, bhs, needblocks); bh_read_batch(haveblocks, bhs);
    if (*bhs).is_null() { *errp = -EIO; goto_b_eio(bhs, 0, stream.total_out); }
    wait_on_buffer(*bhs); if !buffer_uptodate(*bhs) { *errp = -EIO; goto_b_eio(bhs, haveblocks, stream.total_out); }
    mutex_lock(&mut zisofs_zlib_lock); let mut zerr = zlib_inflate_init(&mut stream);
    if zerr != Z_OK { *errp = if zerr == Z_MEM_ERROR { -ENOMEM } else { -EIO }; mutex_unlock(&mut zisofs_zlib_lock); goto_b_eio(bhs, haveblocks, stream.total_out); }
    let mut curbh = 0; let mut curpage = 0;
    while curpage < pcount && curbh < haveblocks && zerr != Z_STREAM_END {
        if stream.avail_out == 0 { let p = *pages.add(curpage as usize); if !p.is_null() { stream.next_out = kmap_local_page(p).add(poffset); stream.avail_out = (PAGE_SIZE-poffset) as u32; poffset=0; } else { stream.next_out = ZISOFS_SINK_PAGE.as_mut_ptr(); stream.avail_out=PAGE_SIZE as u32; } }
        if stream.avail_in == 0 { let bh=*bhs.add(curbh as usize); wait_on_buffer(bh); if !buffer_uptodate(bh) { *errp=-EIO; break; } stream.next_in=(*bh).b_data.add((block_start as usize)&bufmask); stream.avail_in=core::cmp::min((bufsize-((block_start as usize)&bufmask)) as i64, block_size) as u32; block_size-=stream.avail_in as i64; block_start=0; }
        while stream.avail_out != 0 && stream.avail_in != 0 { zerr=zlib_inflate(&mut stream,Z_SYNC_FLUSH); if zerr==Z_BUF_ERROR && stream.avail_in==0 { break; } if zerr==Z_STREAM_END { break; } if zerr!=Z_OK { *errp=if zerr==Z_MEM_ERROR{-ENOMEM}else{-EIO}; break; } }
        if stream.avail_out==0 { let p=*pages.add(curpage as usize); if !p.is_null(){flush_dcache_page(p);set_page_uptodate(p);} if stream.next_out != ZISOFS_SINK_PAGE.as_mut_ptr(){kunmap_local(stream.next_out);stream.next_out=core::ptr::null_mut();} curpage+=1; } if stream.avail_in==0 {curbh+=1;}
    }
    zlib_inflate_end(&mut stream); if !stream.next_out.is_null() && stream.next_out != ZISOFS_SINK_PAGE.as_mut_ptr(){kunmap_local(stream.next_out);} mutex_unlock(&mut zisofs_zlib_lock); goto_b_eio(bhs,haveblocks,stream.total_out)
}

unsafe fn goto_b_eio(bhs:*mut *mut buffer_head, n:i32, out:u64)->i64 { for i in 0..n {brelse(*bhs.add(i as usize));} kfree(bhs as *mut c_void); out as i64 }

unsafe fn zisofs_fill_pages(inode:*mut inode, mut full_page:i32, mut pcount:i32, mut pages:*mut *mut page)->i32 {
    let header_size=isofs_i(inode,0); let shift=isofs_i(inode,1); let blkbits=isofs_buffer_bits(inode); let blksize=1usize<<blkbits;
    let start_off=page_offset(*pages.add(full_page as usize)); let end_off=core::cmp::min(start_off+PAGE_SIZE as u64,(*inode).i_size as u64);
    let mut cstart=start_off>>shift; let cend=(end_off+(1u64<<shift)-1)>>shift; let mut poffset=0i64; let mut blockptr=((header_size as u64)+cstart)<<2;
    let mut bh=isofs_bread(inode,blockptr>>blkbits); if bh.is_null(){return -EIO;} let mut block_start=read_le32((*bh).b_data.add((blockptr as usize)&(blksize-1)));
    while cstart<cend && pcount>0 { blockptr+=4; if (blockptr as usize)&(blksize-1)==0 {brelse(bh);bh=isofs_bread(inode,blockptr>>blkbits);if bh.is_null(){return -EIO;}} let block_end=read_le32((*bh).b_data.add((blockptr as usize)&(blksize-1))); if block_start>block_end {brelse(bh);return -EIO;}
        let mut err=0; let ret=zisofs_uncompress_block(inode,block_start as i64,block_end as i64,pcount,pages,poffset as usize,&mut err); poffset+=ret; let advance=(poffset>>PAGE_SHIFT) as usize; pages=pages.add(advance); pcount-=advance as i32; full_page-=advance as i32; poffset&=!(PAGE_MASK as i64); if err!=0 {brelse(bh);return if full_page<0{0}else{err};} block_start=block_end;cstart+=1;
    } if poffset!=0 && !(*pages).is_null(){memzero_page(*pages,poffset as usize,PAGE_SIZE-poffset as usize);set_page_uptodate(*pages);} brelse(bh);0
}
unsafe extern "C" fn zisofs_read_folio(file:*mut file, folio:*mut folio)->i32 { let inode=file_inode(file); let mapping=(*inode).i_mapping; let end=((*inode).i_size as u64+PAGE_SIZE as u64-1)>>PAGE_SHIFT; let shift=isofs_i(inode,1); let per=if PAGE_SHIFT as u32<=shift{1usize<<(shift-PAGE_SHIFT as u32)}else{0}; let index=(*folio).index; if index>=end{folio_end_read(folio,true);return 0;} let mut full=if per!=0{(index&(per-1))as i32}else{0}; let count=if per!=0{core::cmp::min(per,end-(index&!(per-1)))as i32}else{1}; let mut pages=kzalloc_objs::<*mut page>(core::cmp::max(per,1));if pages.is_null(){folio_unlock(folio);return -ENOMEM;} *pages.add(full as usize)=&mut (*folio).page; let mut idx=index-full as u64;for i in 0..count{if i!=full{*pages.add(i as usize)=grab_cache_page_nowait(mapping,idx);}idx+=1;} let err=zisofs_fill_pages(inode,full,count,pages);for i in 0..count{let p=*pages.add(i as usize);if !p.is_null(){flush_dcache_page(p);unlock_page(p);if i!=full{put_page(p);}}}kfree(pages as *mut c_void);err }
pub static zisofs_aops: address_space_operations = address_space_operations { read_folio: Some(zisofs_read_folio) };
pub unsafe extern "C" fn zisofs_init()->i32 { ZISOFS_ZLIB_WORKSPACE=vmalloc(zlib_inflate_workspacesize()); if ZISOFS_ZLIB_WORKSPACE.is_null(){-ENOMEM}else{0} }
pub unsafe extern "C" fn zisofs_cleanup(){vfree(ZISOFS_ZLIB_WORKSPACE);}

extern "C" { fn isofs_i(inode:*mut inode, index:usize)->u32; fn isofs_buffer_size(inode:*mut inode)->usize; fn isofs_buffer_bits(inode:*mut inode)->usize; }
extern "C" { fn file_inode(file:*mut file)->*mut inode; fn page_offset(page:*mut page)->u64; fn read_le32(p:*mut u8)->u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
