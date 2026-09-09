/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 * Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
 *
 * Created by David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 *
 */

// Dependencies supplied by the surrounding translation unit.

pub const JFFS2_SB_FLAG_RO: u32 = 1;
pub const JFFS2_SB_FLAG_SCANNING: u32 = 2; /* Flash scanning is in progress */
pub const JFFS2_SB_FLAG_BUILDING: u32 = 4; /* File system building is in progress */

pub enum jffs2_inodirty {}

#[repr(C)]
pub struct jffs2_mount_opts {
    pub override_compr: bool,
    pub compr: u32,

    /* The size of the reserved pool. The reserved pool is the JFFS2 flash
     * space which may only be used by root cannot be used by the other
     * users. This is implemented simply by means of not allowing the
     * latter users to write to the file system if the amount if the
     * available space is less then 'rp_size'. */
    pub set_rp_size: bool,
    pub rp_size: u32,
}

/* A struct for the overall file system control.  Pointers to
   jffs2_sb_info structs are named `c' in the source code.
   Nee jffs_control
*/
#[repr(C)]
pub struct jffs2_sb_info {
    pub mtd: *mut mtd_info,

    pub highest_ino: u32,
    pub check_ino: u32, /* *NEXT* inode to be checked */

    pub flags: u32,

    pub gc_task: *mut task_struct, /* GC task struct */
    pub gc_thread_start: completion, /* GC thread start completion */
    pub gc_thread_exit: completion, /* GC thread exit completion port */

    pub alloc_sem: mutex, /* Used to protect all the following
                            fields, and also to protect against
                            out-of-order writing of nodes. And GC. */
    pub cleanmarker_size: u32, /* Size of an _inline_ CLEANMARKER
                                  (i.e. zero for OOB CLEANMARKER */

    pub flash_size: u32,
    pub used_size: u32,
    pub dirty_size: u32,
    pub wasted_size: u32,
    pub free_size: u32,
    pub erasing_size: u32,
    pub bad_size: u32,
    pub sector_size: u32,
    pub unchecked_size: u32,

    pub nr_free_blocks: u32,
    pub nr_erasing_blocks: u32,

    /* Number of free blocks there must be before we... */
    pub resv_blocks_write: u8, /* ... allow a normal filesystem write */
    pub resv_blocks_deletion: u8, /* ... allow a normal filesystem deletion */
    pub resv_blocks_gctrigger: u8, /* ... wake up the GC thread */
    pub resv_blocks_gcbad: u8, /* ... pick a block from the bad_list to GC */
    pub resv_blocks_gcmerge: u8, /* ... merge pages when garbage collecting */
    /* Number of 'very dirty' blocks before we trigger immediate GC */
    pub vdirty_blocks_gctrigger: u8,

    pub nospc_dirty_size: u32,

    pub nr_blocks: u32,
    pub blocks: *mut jffs2_eraseblock, /* The whole array of blocks. Used for getting blocks
                                          * from the offset (blocks[ofs / sector_size]) */
    pub nextblock: *mut jffs2_eraseblock, /* The block we're currently filling */

    pub gcblock: *mut jffs2_eraseblock, /* The block we're currently garbage-collecting */

    pub clean_list: list_head, /* Blocks 100% full of clean data */
    pub very_dirty_list: list_head, /* Blocks with lots of dirty space */
    pub dirty_list: list_head, /* Blocks with some dirty space */
    pub erasable_list: list_head, /* Blocks which are completely dirty, and need erasing */
    pub erasable_pending_wbuf_list: list_head, /* Blocks which need erasing but only after the current wbuf is flushed */
    pub erasing_list: list_head, /* Blocks which are currently erasing */
    pub erase_checking_list: list_head, /* Blocks which are being checked and marked */
    pub erase_pending_list: list_head, /* Blocks which need erasing now */
    pub erase_complete_list: list_head, /* Blocks which are erased and need the clean marker written to them */
    pub free_list: list_head, /* Blocks which are free and ready to be used */
    pub bad_list: list_head, /* Bad blocks. */
    pub bad_used_list: list_head, /* Bad blocks with valid data in. */

    pub erase_completion_lock: spinlock_t, /* Protect free_list and erasing_list
                                              against erase completion handler */
    pub erase_wait: wait_queue_head_t, /* For waiting for erases to complete */

    pub inocache_wq: wait_queue_head_t,
    pub inocache_hashsize: i32,
    pub inocache_list: *mut *mut jffs2_inode_cache,
    pub inocache_lock: spinlock_t,

    /* Sem to allow jffs2_garbage_collect_deletion_dirent to
       drop the erase_completion_lock while it's holding a pointer
       to an obsoleted node. I don't like this. Alternatives welcomed. */
    pub erase_free_sem: mutex,

    pub wbuf_pagesize: u32, /* 0 for NOR and other flashes with no wbuf */

    #[cfg(CONFIG_JFFS2_FS_WBUF_VERIFY)]
    pub wbuf_verify: *mut u8, /* read-back buffer for verification */
    #[cfg(CONFIG_JFFS2_FS_WRITEBUFFER)]
    pub wbuf: *mut u8, /* Write-behind buffer for NAND flash */
    #[cfg(CONFIG_JFFS2_FS_WRITEBUFFER)]
    pub wbuf_ofs: u32,
    #[cfg(CONFIG_JFFS2_FS_WRITEBUFFER)]
    pub wbuf_len: u32,
    #[cfg(CONFIG_JFFS2_FS_WRITEBUFFER)]
    pub wbuf_inodes: *mut jffs2_inodirty,
    #[cfg(CONFIG_JFFS2_FS_WRITEBUFFER)]
    pub wbuf_sem: rw_semaphore, /* Protects the write buffer */

    #[cfg(CONFIG_JFFS2_FS_WRITEBUFFER)]
    pub wbuf_dwork: delayed_work, /* write-buffer write-out work */

    #[cfg(CONFIG_JFFS2_FS_WRITEBUFFER)]
    pub oobbuf: *mut u8,
    #[cfg(CONFIG_JFFS2_FS_WRITEBUFFER)]
    pub oobavail: i32, /* How many bytes are available for JFFS2 in OOB */

    pub summary: *mut jffs2_summary, /* Summary information */
    pub mount_opts: jffs2_mount_opts,

    #[cfg(CONFIG_JFFS2_FS_XATTR)]
    pub highest_xid: u32,
    #[cfg(CONFIG_JFFS2_FS_XATTR)]
    pub highest_xseqno: u32,
    #[cfg(CONFIG_JFFS2_FS_XATTR)]
    pub xattrindex: [list_head; 57],
    #[cfg(CONFIG_JFFS2_FS_XATTR)]
    pub xattr_unchecked: list_head,
    #[cfg(CONFIG_JFFS2_FS_XATTR)]
    pub xattr_dead_list: list_head,
    #[cfg(CONFIG_JFFS2_FS_XATTR)]
    pub xref_dead_list: *mut jffs2_xattr_ref,
    #[cfg(CONFIG_JFFS2_FS_XATTR)]
    pub xref_temp: *mut jffs2_xattr_ref,
    #[cfg(CONFIG_JFFS2_FS_XATTR)]
    pub xattr_sem: rw_semaphore,
    #[cfg(CONFIG_JFFS2_FS_XATTR)]
    pub xdatum_mem_usage: u32,
    #[cfg(CONFIG_JFFS2_FS_XATTR)]
    pub xdatum_mem_threshold: u32,

    /* OS-private pointer for getting back to master superblock info */
    pub os_priv: *mut core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
