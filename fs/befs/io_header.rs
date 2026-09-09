/*
 * io.h
 */

extern "C" {
    pub fn befs_bread_iaddr(
        sb: *mut super_block,
        iaddr: befs_inode_addr,
    ) -> *mut buffer_head;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
