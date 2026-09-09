/*
 * inode.h
 *
 */

extern "C" {
    pub fn befs_check_inode(
        sb: *mut super_block,
        raw_inode: *mut befs_inode,
        inode: befs_blocknr_t,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
