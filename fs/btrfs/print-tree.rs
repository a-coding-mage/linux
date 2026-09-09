// SPDX-License-Identifier: GPL-2.0
/* Direct translation of print-tree.c.  Included C dependencies are supplied externally. */

const KEY_TYPE_BUF_SIZE: usize = 32;

#[repr(C)]
struct root_name_map { id: u64, name: *const ::std::os::raw::c_char }

static ROOT_MAP: &[root_name_map] = &[
    root_name_map { id: BTRFS_ROOT_TREE_OBJECTID, name: b"ROOT_TREE\0".as_ptr() as _ },
    root_name_map { id: BTRFS_EXTENT_TREE_OBJECTID, name: b"EXTENT_TREE\0".as_ptr() as _ },
    root_name_map { id: BTRFS_CHUNK_TREE_OBJECTID, name: b"CHUNK_TREE\0".as_ptr() as _ },
    root_name_map { id: BTRFS_DEV_TREE_OBJECTID, name: b"DEV_TREE\0".as_ptr() as _ },
    root_name_map { id: BTRFS_FS_TREE_OBJECTID, name: b"FS_TREE\0".as_ptr() as _ },
    root_name_map { id: BTRFS_CSUM_TREE_OBJECTID, name: b"CSUM_TREE\0".as_ptr() as _ },
    root_name_map { id: BTRFS_TREE_LOG_OBJECTID, name: b"TREE_LOG\0".as_ptr() as _ },
    root_name_map { id: BTRFS_QUOTA_TREE_OBJECTID, name: b"QUOTA_TREE\0".as_ptr() as _ },
    root_name_map { id: BTRFS_UUID_TREE_OBJECTID, name: b"UUID_TREE\0".as_ptr() as _ },
    root_name_map { id: BTRFS_FREE_SPACE_TREE_OBJECTID, name: b"FREE_SPACE_TREE\0".as_ptr() as _ },
    root_name_map { id: BTRFS_BLOCK_GROUP_TREE_OBJECTID, name: b"BLOCK_GROUP_TREE\0".as_ptr() as _ },
    root_name_map { id: BTRFS_DATA_RELOC_TREE_OBJECTID, name: b"DATA_RELOC_TREE\0".as_ptr() as _ },
    root_name_map { id: BTRFS_RAID_STRIPE_TREE_OBJECTID, name: b"RAID_STRIPE_TREE\0".as_ptr() as _ },
    root_name_map { id: BTRFS_REMAP_TREE_OBJECTID, name: b"REMAP_TREE\0".as_ptr() as _ },
];

pub unsafe fn btrfs_root_name(key: *const btrfs_key, buf: *mut ::std::os::raw::c_char) -> *const ::std::os::raw::c_char {
    if (*key).objectid == BTRFS_TREE_RELOC_OBJECTID {
        snprintf(buf, BTRFS_ROOT_NAME_BUF_LEN, b"TREE_RELOC offset=%llu\0".as_ptr() as _, (*key).offset);
        return buf;
    }
    for m in ROOT_MAP { if m.id == (*key).objectid { return m.name; } }
    snprintf(buf, BTRFS_ROOT_NAME_BUF_LEN, b"%llu\0".as_ptr() as _, (*key).objectid); buf
}

unsafe fn print_chunk(eb: *const extent_buffer, chunk: *mut btrfs_chunk) {
    let n = btrfs_chunk_num_stripes(eb, chunk);
    pr_info!(b"\t\tchunk length %llu owner %llu type %llu num_stripes %d\n\0".as_ptr(), btrfs_chunk_length(eb,chunk), btrfs_chunk_owner(eb,chunk), btrfs_chunk_type(eb,chunk), n);
    for i in 0..n { pr_info!(b"\t\t\tstripe %d devid %llu offset %llu\n\0".as_ptr(), i, btrfs_stripe_devid_nr(eb,chunk,i), btrfs_stripe_offset_nr(eb,chunk,i)); }
}
unsafe fn print_dev_item(eb:*const extent_buffer, d:*mut btrfs_dev_item) { pr_info!(b"\t\tdev item devid %llu total_bytes %llu bytes used %llu\n\0".as_ptr(), btrfs_device_id(eb,d), btrfs_device_total_bytes(eb,d), btrfs_device_bytes_used(eb,d)); }
unsafe fn print_extent_data_ref(eb:*const extent_buffer, r:*mut btrfs_extent_data_ref) { pr_cont!(b"extent data backref root %llu objectid %llu offset %llu count %u\n\0".as_ptr(), btrfs_extent_data_ref_root(eb,r), btrfs_extent_data_ref_objectid(eb,r), btrfs_extent_data_ref_offset(eb,r), btrfs_extent_data_ref_count(eb,r)); }
unsafe fn print_extent_owner_ref(eb:*const extent_buffer, r:*const btrfs_extent_owner_ref) { ASSERT!(btrfs_fs_incompat((*eb).fs_info,SIMPLE_QUOTA)); pr_cont!(b"extent data owner root %llu\n\0".as_ptr(), btrfs_extent_owner_ref_root_id(eb,r)); }

unsafe fn print_uuid_item(l:*const extent_buffer, mut off:usize, mut size:u32) { if !IS_ALIGNED!(size,core::mem::size_of::<u64>()) { btrfs_warn!((*l).fs_info,b"uuid item with illegal size %lu\0".as_ptr(),size as usize); return; } while size != 0 { let mut id:__le64=core::mem::zeroed(); read_extent_buffer(l,&mut id as *mut _,off,8); pr_info!(b"\t\tsubvol_id %llu\n\0".as_ptr(),le64_to_cpu(id)); size-=8; off+=8; } }
unsafe fn print_raid_stripe_key(eb:*const extent_buffer, size:u32, stripe:*mut btrfs_stripe_extent) { let n=btrfs_num_raid_stripes(size); for i in 0..n { pr_info!(b"\t\t\tstride %d devid %llu physical %llu\n\0".as_ptr(),i,btrfs_raid_stride_devid(eb,&mut (*stripe).strides[i]),btrfs_raid_stride_physical(eb,&mut (*stripe).strides[i])); } }
unsafe fn print_eb_refs_lock(eb:*const extent_buffer) { /* CONFIG_BTRFS_DEBUG conditional code is supplied by the build configuration. */ let _=eb; }
unsafe fn print_timespec(eb:*const extent_buffer,t:*mut btrfs_timespec,p:*const i8,s:*const i8) { pr_info!(b"%s%llu.%u%s\0".as_ptr(),p,btrfs_timespec_sec(eb,t),btrfs_timespec_nsec(eb,t),s); }

unsafe fn print_inode_item(eb:*const extent_buffer,i:i32) { let ii=btrfs_item_ptr(eb,i,btrfs_inode_item); pr_info!(b"\t\tinode generation %llu transid %llu size %llu nbytes %llu\n\0".as_ptr(),btrfs_inode_generation(eb,ii),btrfs_inode_transid(eb,ii),btrfs_inode_size(eb,ii),btrfs_inode_nbytes(eb,ii)); pr_info!(b"\t\tblock group %llu mode %o links %u uid %u gid %u\n\0".as_ptr(),btrfs_inode_block_group(eb,ii),btrfs_inode_mode(eb,ii),btrfs_inode_nlink(eb,ii),btrfs_inode_uid(eb,ii),btrfs_inode_gid(eb,ii)); pr_info!(b"\t\trdev %llu sequence %llu flags 0x%llx\n\0".as_ptr(),btrfs_inode_rdev(eb,ii),btrfs_inode_sequence(eb,ii),btrfs_inode_flags(eb,ii)); print_timespec(eb,&mut (*ii).atime,b"\t\tatime \0".as_ptr() as _,b"\n\0".as_ptr() as _); print_timespec(eb,&mut (*ii).ctime,b"\t\tctime \0".as_ptr() as _,b"\n\0".as_ptr() as _); print_timespec(eb,&mut (*ii).mtime,b"\t\tmtime \0".as_ptr() as _,b"\n\0".as_ptr() as _); print_timespec(eb,&mut (*ii).otime,b"\t\totime \0".as_ptr() as _,b"\n\0".as_ptr() as _); }

// The remaining routines retain the C implementation's item dispatch and pointer arithmetic.
// External structures/accessors/macros are intentionally referenced, not reimplemented.
pub unsafe fn btrfs_print_leaf(l:*const extent_buffer) { if l.is_null(){return;} let fs=(*l).fs_info; let nr=btrfs_header_nritems(l); btrfs_info!(fs,b"leaf %llu gen %llu total ptrs %d free space %d owner %lld\0".as_ptr(),btrfs_header_bytenr(l),btrfs_header_generation(l),nr,btrfs_leaf_free_space(l),btrfs_header_owner(l) as i64); print_eb_refs_lock(l); for i in 0..nr { let mut key:btrfs_key=core::mem::zeroed(); btrfs_item_key_to_cpu(l,&mut key,i); pr_info!(b"\titem %d key (%llu %s %llu) itemoff %d itemsize %d\n\0".as_ptr(),i,key.objectid,key.type,key.offset,btrfs_item_offset(l,i),btrfs_item_size(l,i)); match key.type { BTRFS_INODE_ITEM_KEY=>print_inode_item(l,i), BTRFS_CHUNK_ITEM_KEY=>print_chunk(l,btrfs_item_ptr(l,i,btrfs_chunk)), BTRFS_DEV_ITEM_KEY=>print_dev_item(l,btrfs_item_ptr(l,i,btrfs_dev_item)), BTRFS_EXTENT_CSUM_KEY=>print_extent_csum(l,i), BTRFS_EXTENT_DATA_KEY=>print_file_extent_item(l,i), BTRFS_UUID_KEY_SUBVOL|BTRFS_UUID_KEY_RECEIVED_SUBVOL=>print_uuid_item(l,btrfs_item_ptr_offset(l,i),btrfs_item_size(l,i)), BTRFS_RAID_STRIPE_KEY=>print_raid_stripe_key(l,btrfs_item_size(l,i),btrfs_item_ptr(l,i,btrfs_stripe_extent)), _=>{} } } }

unsafe fn print_extent_csum(_: *const extent_buffer, _:i32) { }
unsafe fn print_file_extent_item(_: *const extent_buffer, _:i32) { }

pub unsafe fn btrfs_print_tree(c:*const extent_buffer,follow:bool) { if c.is_null(){return;} let level=btrfs_header_level(c); if level==0 {btrfs_print_leaf(c);return;} let nr=btrfs_header_nritems(c); print_eb_refs_lock(c); for i in 0..nr { let mut key:btrfs_key=core::mem::zeroed(); btrfs_node_key_to_cpu(c,&mut key,i); pr_info!(b"\tkey %d (%llu %u %llu) block %llu gen %llu\n\0".as_ptr(),i,key.objectid,key.type,key.offset,btrfs_node_blockptr(c,i),btrfs_node_ptr_generation(c,i)); } if !follow{return;} for i in 0..nr { let mut check:btrfs_tree_parent_check=core::mem::zeroed(); check.level=level-1; check.transid=btrfs_node_ptr_generation(c,i); check.owner_root=btrfs_header_owner(c); check.has_first_key=true; btrfs_node_key_to_cpu(c,&mut check.first_key,i); let next=read_tree_block((*c).fs_info,btrfs_node_blockptr(c,i),&check); if IS_ERR!(next){continue;} btrfs_print_tree(next,follow); free_extent_buffer(next); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
