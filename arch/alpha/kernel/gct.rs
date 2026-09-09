// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/gct.c
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/types.h, linux/errno.h, asm/hwrpb.h, and asm/gct.h.

pub unsafe fn gct6_find_nodes(
    mut node: *mut gct6_node,
    search: *mut gct6_search_struct,
) -> i32 {
    let mut wanted: *mut gct6_search_struct;
    let mut status: i32 = 0;

    /* First check the magic number.  */
    if (*node).magic != GCT_NODE_MAGIC {
        printk(KERN_ERR "GCT Node MAGIC incorrect - GCT invalid\n");
        return -EINVAL;
    }

    /* Check against the search struct.  */
    wanted = search;
    while !wanted.is_null() && ((*wanted).type_ | (*wanted).subtype) != 0 {
        if (*node).type_ != (*wanted).type_ {
            wanted = wanted.add(1);
            continue;
        }
        if (*node).subtype != (*wanted).subtype {
            wanted = wanted.add(1);
            continue;
        }

        /* Found it -- call out.  */
        if let Some(callout) = (*wanted).callout {
            callout(node);
        }

        wanted = wanted.add(1);
    }

    /* Now walk the tree, siblings first.  */
    if (*node).next != 0 {
        status |= gct6_find_nodes(GCT_NODE_PTR((*node).next), search);
    }

    /* Then the children.  */
    if (*node).child != 0 {
        status |= gct6_find_nodes(GCT_NODE_PTR((*node).child), search);
    }

    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
