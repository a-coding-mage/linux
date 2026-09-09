// SPDX-License-Identifier: GPL-2.0
/*
 * (C) 2001 Clemson University and The University of Chicago
 *
 * See COPYING in top-level directory.
 */

// Translated from the C dependencies:
// #include "protocol.h"
// #include "orangefs-kernel.h"
// #include "orangefs-bufmap.h"

extern "C" {
    fn simple_get_link();
    fn orangefs_setattr();
    fn orangefs_getattr();
    fn orangefs_listxattr();
    fn orangefs_permission();
    fn orangefs_update_time();
}

pub static orangefs_symlink_inode_operations: inode_operations = inode_operations {
    get_link: Some(simple_get_link),
    setattr: Some(orangefs_setattr),
    getattr: Some(orangefs_getattr),
    listxattr: Some(orangefs_listxattr),
    permission: Some(orangefs_permission),
    update_time: Some(orangefs_update_time),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
