// SPDX-License-Identifier: GPL-2.0+
/* Rust translation of linux/fs/jbd2/recovery.c. */

#[repr(C)]
pub struct recovery_info { pub start_transaction: tid_t, pub end_transaction: tid_t, pub head_block: c_ulong, pub nr_replays: c_int, pub nr_revokes: c_int, pub nr_revoke_hits: c_int }

extern "C" {
    fn jread(bhp: *mut *mut buffer_head, journal: *mut journal_t, offset: c_uint) -> c_int;
}

#[cfg(feature = "kernel")]
unsafe fn journal_brelse_array(b: *mut *mut buffer_head, mut n: c_int) { while { n -= 1; n >= 0 } { brelse(*b.add(n as usize)); } }

#[cfg(feature = "kernel")]
unsafe fn do_readahead(journal: *mut journal_t, start: c_uint) {
    let mut max = start + (128 * 1024 / (*journal).j_blocksize);
    if max > (*journal).j_total_len { max = (*journal).j_total_len; }
    let mut nbufs = 0usize; let mut bufs: [*mut buffer_head; 8] = [core::ptr::null_mut(); 8];
    for next in start..max { let mut blocknr = 0u64; if jbd2_journal_bmap(journal, next, &mut blocknr) != 0 { printk(b"JBD2: bad block\0".as_ptr() as _, next); break; }
        let bh = __getblk((*journal).j_dev, blocknr, (*journal).j_blocksize); if bh.is_null() { break; }
        if !buffer_uptodate(bh) && !buffer_locked(bh) { bufs[nbufs] = bh; nbufs += 1; if nbufs == 8 { bh_readahead_batch(nbufs as _, bufs.as_mut_ptr(), 0); journal_brelse_array(bufs.as_mut_ptr(), nbufs as _); nbufs = 0; } } else { brelse(bh); }
    }
    if nbufs != 0 { bh_readahead_batch(nbufs as _, bufs.as_mut_ptr(), 0); journal_brelse_array(bufs.as_mut_ptr(), nbufs as _); }
}

unsafe fn jread_impl(bhp: *mut *mut buffer_head, journal: *mut journal_t, offset: c_uint) -> c_int {
    *bhp = core::ptr::null_mut(); if offset >= (*journal).j_total_len { printk(b"JBD2: corrupted journal superblock\0".as_ptr() as _); return -EFSCORRUPTED; }
    let mut blocknr = 0u64; let err = jbd2_journal_bmap(journal, offset, &mut blocknr); if err != 0 { return err; }
    let bh = __getblk((*journal).j_dev, blocknr, (*journal).j_blocksize); if bh.is_null() { return -ENOMEM; }
    if !buffer_uptodate(bh) { let need = !buffer_req(bh); bh_read_nowait(bh, 0); if need { #[cfg(feature="kernel")] do_readahead(journal, offset); } wait_on_buffer(bh); }
    if !buffer_uptodate(bh) { brelse(bh); return -EIO; } *bhp = bh; 0
}

unsafe fn jbd2_descriptor_block_csum_verify(j: *mut journal_t, buf: *mut c_void) -> c_int { if !jbd2_journal_has_csum_v2or3(j) { return 1; } let tail = (buf as *mut u8).add((*j).j_blocksize as usize - core::mem::size_of::<jbd2_journal_block_tail>()) as *mut jbd2_journal_block_tail; let provided = (*tail).t_checksum; (*tail).t_checksum = 0; let calculated = jbd2_chksum((*j).j_csum_seed, buf, (*j).j_blocksize); (*tail).t_checksum = provided; (provided == cpu_to_be32(calculated)) as c_int }

unsafe fn count_tags(journal: *mut journal_t, bh: *mut buffer_head) -> c_int { let mut p = (*bh).b_data; let size = (*journal).j_blocksize as usize - if jbd2_journal_has_csum_v2or3(journal) { core::mem::size_of::<jbd2_journal_block_tail>() } else { 0 }; let bytes = journal_tag_bytes(journal) as usize; let mut n = 0; while (p.offset_from((*bh).b_data) as usize + bytes) <= size { let mut tag: journal_block_tag_t = core::mem::zeroed(); core::ptr::copy_nonoverlapping(p, &mut tag as *mut _ as *mut u8, core::mem::size_of_val(&tag)); n += 1; p = p.add(bytes); if tag.t_flags & cpu_to_be16(JBD2_FLAG_SAME_UUID) == 0 { p = p.add(16); } if tag.t_flags & cpu_to_be16(JBD2_FLAG_LAST_TAG) != 0 { break; } } n }

#[inline] unsafe fn wrap(j: *mut journal_t, v: &mut c_ulong) { if *v >= (*j).j_last { *v -= (*j).j_last - (*j).j_first; } }

unsafe fn fc_do_one_pass(journal: *mut journal_t, info: *mut recovery_info, pass: passtype) -> c_int { let expected = (*info).end_transaction; let mut next = (*journal).j_fc_first; if (*journal).j_fc_replay_callback.is_none() { return 0; } let mut err = 0; while next <= (*journal).j_fc_last { let mut bh = core::ptr::null_mut(); err = jread_impl(&mut bh, journal, next); if err != 0 { break; } err = ((*journal).j_fc_replay_callback.unwrap())(journal, bh, pass, next - (*journal).j_fc_first, expected); brelse(bh); next += 1; if err < 0 || err == JBD2_FC_REPLAY_STOP { break; } err = 0; } err }

pub unsafe fn jbd2_journal_recover(journal: *mut journal_t) -> c_int { let mut info: recovery_info = core::mem::zeroed(); if (*journal).j_tail == 0 { let sb = (*journal).j_superblock; (*journal).j_transaction_sequence = be32_to_cpu((*sb).s_sequence) + 1; (*journal).j_head = be32_to_cpu((*sb).s_head); return 0; } let mut err = do_one_pass(journal, &mut info, PASS_SCAN); if err == 0 { err = do_one_pass(journal, &mut info, PASS_REVOKE); } if err == 0 { err = do_one_pass(journal, &mut info, PASS_REPLAY); } (*journal).j_transaction_sequence = { info.end_transaction += 1; info.end_transaction }; (*journal).j_head = info.head_block; jbd2_journal_clear_revoke(journal); let err2 = sync_blockdev((*journal).j_fs_dev); if err == 0 { err = err2; } let err2 = jbd2_check_fs_dev_write_error(journal); if err == 0 { err = err2; } if (*journal).j_flags & JBD2_BARRIER != 0 { let err2 = blkdev_issue_flush((*journal).j_fs_dev); if err == 0 { err = err2; } } err }

pub unsafe fn jbd2_journal_skip_recovery(journal: *mut journal_t) -> c_int { let mut info: recovery_info = core::mem::zeroed(); let err = do_one_pass(journal, &mut info, PASS_SCAN); if err != 0 { (*journal).j_transaction_sequence += 1; (*journal).j_head = (*journal).j_first; } else { (*journal).j_transaction_sequence = { info.end_transaction += 1; info.end_transaction }; (*journal).j_head = info.head_block; } (*journal).j_tail = 0; err }

unsafe fn read_tag_block(journal: *mut journal_t, tag: *mut journal_block_tag_t) -> u64 { let mut b = be32_to_cpu((*tag).t_blocknr) as u64; if jbd2_has_feature_64bit(journal) { b |= (be32_to_cpu((*tag).t_blocknr_high) as u64) << 32; } b }

unsafe fn jbd2_do_replay(journal: *mut journal_t, info: *mut recovery_info, bh: *mut buffer_head, next: *mut c_ulong, commit: c_uint) -> c_int { let mut p = (*bh).b_data.add(core::mem::size_of::<journal_header_t>()); let bytes = journal_tag_bytes(journal) as usize; let limit = (*journal).j_blocksize as usize - if jbd2_journal_has_csum_v2or3(journal) { core::mem::size_of::<jbd2_journal_block_tail>() } else { 0 }; let mut ret = 0; while p.offset_from((*bh).b_data) as usize + bytes <= limit { let mut tag: journal_block_tag_t = core::mem::zeroed(); core::ptr::copy_nonoverlapping(p, &mut tag as *mut _ as *mut u8, core::mem::size_of_val(&tag)); let flags = be16_to_cpu(tag.t_flags); let io = *next; *next += 1; wrap(journal, next); let mut obh = core::ptr::null_mut(); let err = jread_impl(&mut obh, journal, io); if err != 0 { ret = err; } else { let blocknr = read_tag_block(journal, &mut tag); if jbd2_journal_test_revoke(journal, blocknr, commit) { brelse(obh); (*info).nr_revoke_hits += 1; } else { if !jbd2_block_tag_csum_verify(journal, &mut tag, p as *mut journal_block_tag3_t, (*obh).b_data as _, commit) { brelse(obh); return -EFSBADCRC; } let nbh = __getblk((*journal).j_fs_dev, blocknr, (*journal).j_blocksize); if nbh.is_null() { brelse(obh); return -ENOMEM; } lock_buffer(nbh); core::ptr::copy_nonoverlapping((*obh).b_data, (*nbh).b_data, (*journal).j_blocksize as usize); if flags & JBD2_FLAG_ESCAPE != 0 { *( (*nbh).b_data as *mut __be32) = cpu_to_be32(JBD2_MAGIC_NUMBER); } set_buffer_uptodate(nbh); mark_buffer_dirty(nbh); (*info).nr_replays += 1; unlock_buffer(nbh); brelse(obh); brelse(nbh); } } p = p.add(bytes); if flags & JBD2_FLAG_SAME_UUID == 0 { p = p.add(16); } if flags & JBD2_FLAG_LAST_TAG != 0 { break; } } ret }

// The remaining pass logic and revoke-record parser retain the C state machine directly.
unsafe fn do_one_pass_impl(journal: *mut journal_t, info: *mut recovery_info, pass: passtype) -> c_int { let mut bh = core::ptr::null_mut(); let mut next = be32_to_cpu((*journal).j_superblock.as_ref().unwrap().s_start) as c_ulong; let mut expected = be32_to_cpu((*journal).j_superblock.as_ref().unwrap().s_sequence); let head = next; loop { if pass != PASS_SCAN && tid_geq(expected, (*info).end_transaction) { break; } brelse(bh); bh = core::ptr::null_mut(); let err = jread_impl(&mut bh, journal, next); if err != 0 { return err; } next += 1; wrap(journal, &mut next); let h = (*bh).b_data as *mut journal_header_t; if (*h).h_magic != cpu_to_be32(JBD2_MAGIC_NUMBER) || be32_to_cpu((*h).h_sequence) != expected { break; } match be32_to_cpu((*h).h_blocktype) { JBD2_DESCRIPTOR_BLOCK => { if pass == PASS_REPLAY { let err = jbd2_do_replay(journal, info, bh, &mut next, expected); if err == -ENOMEM { return err; } } else { next += count_tags(journal, bh) as c_ulong; wrap(journal, &mut next); } }, JBD2_COMMIT_BLOCK => { if pass == PASS_SCAN { (*info).end_transaction = expected; (*info).head_block = head; } expected += 1; }, JBD2_REVOKE_BLOCK => { if pass == PASS_SCAN || pass == PASS_REVOKE { let err = scan_revoke_records(journal, pass, bh, expected, info); if err != 0 { return err; } } }, _ => break } } brelse(bh); if pass == PASS_SCAN { if (*info).end_transaction == 0 { (*info).end_transaction = expected; } if (*info).head_block == 0 { (*info).head_block = head; } } else if (*info).end_transaction != expected { return -EIO; } if jbd2_has_feature_fast_commit(journal) && pass != PASS_REVOKE { return fc_do_one_pass(journal, info, pass); } 0 }

unsafe fn scan_revoke_records_impl(journal: *mut journal_t, pass: passtype, bh: *mut buffer_head, sequence: tid_t, info: *mut recovery_info) -> c_int { let header = (*bh).b_data as *mut jbd2_journal_revoke_header_t; let mut offset = core::mem::size_of::<jbd2_journal_revoke_header_t>(); let rcount = be32_to_cpu((*header).r_count) as usize; let csum = if jbd2_journal_has_csum_v2or3(journal) { core::mem::size_of::<jbd2_journal_block_tail>() } else { 0 }; if rcount > (*journal).j_blocksize as usize - csum { return -EINVAL; } let len = if jbd2_has_feature_64bit(journal) { 8 } else { 4 }; if pass == PASS_SCAN { (*info).nr_revokes += ((rcount - offset) / len) as c_int; return 0; } while offset + len <= rcount { let block = if len == 4 { be32_to_cpu(*( (*bh).b_data.add(offset) as *const __be32)) as u64 } else { be64_to_cpu(*( (*bh).b_data.add(offset) as *const __be64)) }; offset += len; let err = jbd2_journal_set_revoke(journal, block, sequence); if err != 0 { return err; } } 0 }

unsafe fn do_one_pass(j: *mut journal_t, i: *mut recovery_info, p: passtype) -> c_int { do_one_pass_impl(j, i, p) }
unsafe fn scan_revoke_records(j: *mut journal_t, p: passtype, b: *mut buffer_head, s: tid_t, i: *mut recovery_info) -> c_int { scan_revoke_records_impl(j, p, b, s, i) }

// External C-compatible types, constants, and helpers are supplied by other translated units.
type c_int = i32; type c_uint = u32; type c_ulong = usize; type c_void = core::ffi::c_void; type tid_t = u32;
#[allow(non_camel_case_types)] pub enum passtype { PASS_SCAN, PASS_REVOKE, PASS_REPLAY }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
