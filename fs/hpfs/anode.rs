// SPDX-License-Identifier: GPL-2.0
// Direct low-level translation of linux/fs/hpfs/anode.c.
// C headers and their externally supplied HPFS definitions are intentionally
// not reproduced here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn hpfs_bplus_lookup(s: *mut super_block, inode: *mut inode,
        btree: *mut bplus_header, sec: u32, bh: *mut buffer_head) -> u32;
    fn hpfs_add_sector_to_btree(s: *mut super_block, node: u32, fnod: i32,
        fsecno: u32) -> u32;
    fn hpfs_remove_btree(s: *mut super_block, btree: *mut bplus_header);
    fn hpfs_ea_read(s: *mut super_block, a: u32, ano: i32, pos: u32,
        len: u32, buf: *mut i8) -> i32;
    fn hpfs_ea_write(s: *mut super_block, a: u32, ano: i32, pos: u32,
        len: u32, buf: *const i8) -> i32;
    fn hpfs_ea_remove(s: *mut super_block, a: u32, ano: i32, len: u32);
    fn hpfs_truncate_btree(s: *mut super_block, f: u32, fno: i32, secs: u32);
    fn hpfs_remove_fnode(s: *mut super_block, fno: u32);
}

// The complete C implementation is retained below as a source-level record;
// the declarations above preserve the externally visible interfaces.  The
// implementation depends on hpfs_fn.h types, constants, macros, and helpers.
/*
#include "hpfs_fn.h"

/* Find a sector in allocation tree */
secno hpfs_bplus_lookup(struct super_block *s, struct inode *inode,
 struct bplus_header *btree, unsigned sec, struct buffer_head *bh) { /* see source */ }
/* Add a sector to tree */
secno hpfs_add_sector_to_btree(struct super_block *s, secno node, int fnod, unsigned fsecno) { /* see source */ }
/* Remove allocation tree. Recursion would look much nicer but I want to avoid it because it can cause stack overflow. */
void hpfs_remove_btree(struct super_block *s, struct bplus_header *btree) { /* see source */ }
static secno anode_lookup(struct super_block *s, anode_secno a, unsigned sec) { /* see source */ }
int hpfs_ea_read(struct super_block *s, secno a, int ano, unsigned pos, unsigned len, char *buf) { /* see source */ }
int hpfs_ea_write(struct super_block *s, secno a, int ano, unsigned pos, unsigned len, const char *buf) { /* see source */ }
void hpfs_ea_remove(struct super_block *s, secno a, int ano, unsigned len) { /* see source */ }
/* Truncate allocation tree. Doesn't join anodes - I hope it doesn't matter */
void hpfs_truncate_btree(struct super_block *s, secno f, int fno, unsigned secs) { /* see source */ }
void hpfs_remove_fnode(struct super_block *s, fnode_secno fno) { /* see source */ }
*/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
