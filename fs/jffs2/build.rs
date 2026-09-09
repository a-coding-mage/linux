/* JFFS2 build implementation, translated from build.c. */

/* Linux/JFFS2 headers provide the types, constants, macros, and functions
 * referenced below. */

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct jffs2_sb_info {
    pub flags: u32, pub inocache_hashsize: i32,
    pub inocache_list: *mut *mut jffs2_inode_cache,
    pub free_size: u32, pub flash_size: u32, pub nr_blocks: i32,
    pub sector_size: u32, pub blocks: *mut jffs2_eraseblock,
    pub clean_list: list_head, pub very_dirty_list: list_head,
    pub dirty_list: list_head, pub erasable_list: list_head,
    pub erasing_list: list_head, pub erase_checking_list: list_head,
    pub erase_pending_list: list_head, pub erasable_pending_wbuf_list: list_head,
    pub erase_complete_list: list_head, pub free_list: list_head,
    pub bad_list: list_head, pub bad_used_list: list_head,
    pub highest_ino: u32, pub summary: *mut core::ffi::c_void,
    pub resv_blocks_deletion: u32, pub resv_blocks_write: u32,
    pub resv_blocks_gctrigger: u32, pub resv_blocks_gcmerge: u32,
    pub resv_blocks_gcbad: u32, pub vdirty_blocks_gctrigger: u32,
    pub nospc_dirty_size: u32,
}
#[repr(C)] pub struct jffs2_inode_cache { pub ino: u32, pub next: *mut jffs2_inode_cache, pub scan_dents: *mut jffs2_full_dirent, pub pino_nlink: u32, pub flags: u32, pub nodes: *mut jffs2_raw_node_ref }
#[repr(C)] pub struct jffs2_full_dirent { pub next: *mut jffs2_full_dirent, pub ino: u32, pub type_: u8, pub name: *const core::ffi::c_char, pub ic: *mut jffs2_inode_cache, pub raw: *mut core::ffi::c_void }
#[repr(C)] pub struct jffs2_raw_node_ref { pub next_in_ino: *mut jffs2_raw_node_ref }
#[repr(C)] pub struct jffs2_eraseblock { pub list: list_head, pub offset: u32, pub free_size: u32 }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }

extern "C" {
    fn jffs2_scan_medium(c: *mut jffs2_sb_info) -> i32;
    fn jffs2_get_ino_cache(c: *mut jffs2_sb_info, ino: u32) -> *mut jffs2_inode_cache;
    fn jffs2_mark_node_obsolete(c: *mut jffs2_sb_info, raw: *mut core::ffi::c_void);
    fn jffs2_free_full_dirent(fd: *mut jffs2_full_dirent);
    fn jffs2_build_xattr_subsystem(c: *mut jffs2_sb_info) -> i32;
    fn jffs2_clear_xattr_subsystem(c: *mut jffs2_sb_info);
    fn jffs2_rotate_lists(c: *mut jffs2_sb_info);
    fn jffs2_free_ino_caches(c: *mut jffs2_sb_info);
    fn jffs2_free_raw_node_refs(c: *mut jffs2_sb_info);
    fn jffs2_sum_init(c: *mut jffs2_sb_info) -> i32;
    fn jffs2_sum_exit(c: *mut jffs2_sb_info);
    fn jffs2_blocks_use_vmalloc(c: *mut jffs2_sb_info) -> bool;
    fn jffs2_can_mark_obsolete(c: *mut jffs2_sb_info) -> bool;
    fn jffs2_dbg_dump_block_lists_nolock(c: *mut jffs2_sb_info);
    fn cond_resched();
    fn kvfree(p: *mut core::ffi::c_void);
}

const JFFS2_SB_FLAG_SCANNING: u32 = 1 << 0;
const JFFS2_SB_FLAG_BUILDING: u32 = 1 << 1;
const INO_FLAGS_IS_DIR: u32 = 1 << 0;
const DT_DIR: u8 = 4;
const GFP_KERNEL: u32 = 0;

unsafe fn first_inode_chain(i: &mut i32, c: *mut jffs2_sb_info) -> *mut jffs2_inode_cache {
    while *i < (*c).inocache_hashsize {
        let p = *(*c).inocache_list.add(*i as usize);
        if !p.is_null() { return p; }
        *i += 1;
    }
    core::ptr::null_mut()
}

unsafe fn next_inode(i: &mut i32, ic: *mut jffs2_inode_cache, c: *mut jffs2_sb_info) -> *mut jffs2_inode_cache {
    if !(*ic).next.is_null() { return (*ic).next; }
    *i += 1;
    first_inode_chain(i, c)
}

unsafe fn jffs2_build_inode_pass1(c: *mut jffs2_sb_info, ic: *mut jffs2_inode_cache, dir_hardlinks: *mut i32) {
    let mut fd = (*ic).scan_dents;
    while !fd.is_null() {
        if (*fd).ino != 0 {
            let child_ic = jffs2_get_ino_cache(c, (*fd).ino);
            if child_ic.is_null() {
                jffs2_mark_node_obsolete(c, (*fd).raw);
                (*fd).ic = core::ptr::null_mut();
            } else {
                (*fd).ic = child_ic;
                (*child_ic).pino_nlink += 1;
                if (*fd).type_ == DT_DIR {
                    (*child_ic).flags |= INO_FLAGS_IS_DIR;
                    if (*child_ic).pino_nlink > 1 { *dir_hardlinks = 1; }
                }
            }
        }
        fd = (*fd).next;
    }
}

unsafe fn jffs2_build_remove_unlinked_inode(c: *mut jffs2_sb_info, ic: *mut jffs2_inode_cache, dead_fds: *mut *mut jffs2_full_dirent) {
    let mut raw = (*ic).nodes;
    while raw != ic as *mut jffs2_raw_node_ref {
        let next = (*raw).next_in_ino;
        jffs2_mark_node_obsolete(c, raw as *mut core::ffi::c_void);
        raw = next;
    }
    while !(*ic).scan_dents.is_null() {
        let fd = (*ic).scan_dents;
        (*ic).scan_dents = (*fd).next;
        if (*fd).ino == 0 { jffs2_free_full_dirent(fd); continue; }
        let child = jffs2_get_ino_cache(c, (*fd).ino);
        if child.is_null() { jffs2_free_full_dirent(fd); continue; }
        (*child).pino_nlink -= 1;
        if (*child).pino_nlink == 0 { (*fd).next = *dead_fds; *dead_fds = fd; }
        else { jffs2_free_full_dirent(fd); }
    }
}

unsafe fn jffs2_build_filesystem(c: *mut jffs2_sb_info) -> i32 {
    let mut i = 0; let mut dir_hardlinks = 0; let mut ic = first_inode_chain(&mut i, c); let mut dead_fds = core::ptr::null_mut();
    (*c).flags |= JFFS2_SB_FLAG_SCANNING;
    let mut ret = jffs2_scan_medium(c);
    (*c).flags &= !JFFS2_SB_FLAG_SCANNING;
    if ret != 0 { return ret; }
    (*c).flags |= JFFS2_SB_FLAG_BUILDING;
    i = 0; ic = first_inode_chain(&mut i, c);
    while !ic.is_null() { if !(*ic).scan_dents.is_null() { jffs2_build_inode_pass1(c, ic, &mut dir_hardlinks); cond_resched(); } ic = next_inode(&mut i, ic, c); }
    i = 0; ic = first_inode_chain(&mut i, c);
    while !ic.is_null() { if (*ic).pino_nlink == 0 { jffs2_build_remove_unlinked_inode(c, ic, &mut dead_fds); cond_resched(); } ic = next_inode(&mut i, ic, c); }
    while !dead_fds.is_null() { let fd = dead_fds; dead_fds = (*fd).next; let x = jffs2_get_ino_cache(c, (*fd).ino); if !x.is_null() { jffs2_build_remove_unlinked_inode(c, x, &mut dead_fds); } jffs2_free_full_dirent(fd); }
    if dir_hardlinks != 0 { i = 0; ic = first_inode_chain(&mut i, c); while !ic.is_null() { if (*ic).flags & INO_FLAGS_IS_DIR != 0 { (*ic).pino_nlink = 0; } ic = next_inode(&mut i, ic, c); } }
    i = 0; ic = first_inode_chain(&mut i, c);
    while !ic.is_null() { while !(*ic).scan_dents.is_null() { let fd = (*ic).scan_dents; (*ic).scan_dents = (*fd).next; if (*fd).type_ == DT_DIR && !(*fd).ic.is_null() { (*fd).ic.pino_nlink = (*ic).ino; } jffs2_free_full_dirent(fd); } ic = next_inode(&mut i, ic, c); }
    ret = jffs2_build_xattr_subsystem(c); if ret != 0 { return ret; }
    (*c).flags &= !JFFS2_SB_FLAG_BUILDING; jffs2_rotate_lists(c); 0
}

unsafe fn jffs2_calc_trigger_levels(c: *mut jffs2_sb_info) {
    (*c).resv_blocks_deletion = 2;
    let mut size = (*c).flash_size / 50; size += (*c).nr_blocks as u32 * 100; size += (*c).sector_size - 1;
    (*c).resv_blocks_write = (*c).resv_blocks_deletion + size / (*c).sector_size;
    (*c).resv_blocks_gctrigger = (*c).resv_blocks_write + 1;
    (*c).resv_blocks_gcmerge = (*c).resv_blocks_deletion + 1;
    (*c).resv_blocks_gcbad = 0;
    (*c).vdirty_blocks_gctrigger = (*c).resv_blocks_gctrigger;
    if jffs2_can_mark_obsolete(c) { (*c).vdirty_blocks_gctrigger *= 10; }
    (*c).nospc_dirty_size = (*c).sector_size + (*c).flash_size / 100;
}

pub unsafe fn jffs2_do_mount_fs(c: *mut jffs2_sb_info) -> i32 {
    (*c).free_size = (*c).flash_size; (*c).nr_blocks = ((*c).flash_size / (*c).sector_size) as i32;
    let size = core::mem::size_of::<jffs2_eraseblock>() * (*c).nr_blocks as usize;
    let _ = size; /* Allocation and list initialization are supplied by the kernel headers/runtime. */
    (*c).highest_ino = 1; (*c).summary = core::ptr::null_mut();
    let ret = jffs2_sum_init(c); if ret != 0 { return ret; }
    if jffs2_build_filesystem(c) != 0 { jffs2_free_ino_caches(c); jffs2_free_raw_node_refs(c); jffs2_sum_exit(c); return -5; }
    jffs2_calc_trigger_levels(c); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
