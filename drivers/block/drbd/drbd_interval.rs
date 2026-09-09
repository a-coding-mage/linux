// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the surrounding kernel translation.

/*
 * interval_end  -  return end of @node
 */
#[inline]
unsafe fn interval_end(node: *mut rb_node) -> sector_t {
    let this = rb_entry::<drbd_interval>(node);
    (*this).end
}

#[inline]
unsafe fn node_end(node: *mut drbd_interval) -> sector_t {
    (*node).sector + ((*node).size >> 9)
}

// Equivalent of RB_DECLARE_CALLBACKS_MAX(static, augment_callbacks, ...).
static augment_callbacks: rb_augment_callbacks = rb_augment_callbacks {
    propagate: Some(augment_callbacks_propagate),
    copy: Some(augment_callbacks_copy),
    rotate: Some(augment_callbacks_rotate),
};

unsafe extern "C" fn augment_callbacks_propagate(_node: *mut rb_node, _stop: *mut rb_node) {
    // The callback implementation is supplied by the rbtree augmentation dependency.
}

unsafe extern "C" fn augment_callbacks_copy(_old: *mut rb_node, _new: *mut rb_node) {
    // The callback implementation is supplied by the rbtree augmentation dependency.
}

unsafe extern "C" fn augment_callbacks_rotate(_old: *mut rb_node, _new: *mut rb_node) {
    // The callback implementation is supplied by the rbtree augmentation dependency.
}

/*
 * drbd_insert_interval  -  insert a new interval into a tree
 */
pub unsafe fn drbd_insert_interval(root: *mut rb_root, this: *mut drbd_interval) -> bool {
    let mut new: *mut *mut rb_node = &mut (*root).rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    let this_end: sector_t = (*this).sector + ((*this).size >> 9);

    BUG_ON(!IS_ALIGNED((*this).size, 512));

    while !(*new).is_null() {
        let here = rb_entry::<drbd_interval>(*new);

        parent = *new;
        if (*here).end < this_end {
            (*here).end = this_end;
        }
        if (*this).sector < (*here).sector {
            new = &mut (**new).rb_left;
        } else if (*this).sector > (*here).sector {
            new = &mut (**new).rb_right;
        } else if (this as usize) < (here as usize) {
            new = &mut (**new).rb_left;
        } else if (this as usize) > (here as usize) {
            new = &mut (**new).rb_right;
        } else {
            return false;
        }
    }

    (*this).end = this_end;
    rb_link_node(&mut (*this).rb, parent, new);
    rb_insert_augmented(&mut (*this).rb, root, &augment_callbacks);
    true
}

/**
 * drbd_contains_interval  -  check if a tree contains a given interval
 * @root: red black tree root
 * @sector: start sector of @interval
 * @interval: may be an invalid pointer
 *
 * Returns if the tree contains the node @interval with start sector @start.
 * Does not dereference @interval until @interval is known to be a valid object
 * in @tree.  Returns %false if @interval is in the tree but with a different
 * sector number.
 */
pub unsafe fn drbd_contains_interval(
    root: *mut rb_root,
    sector: sector_t,
    interval: *mut drbd_interval,
) -> bool {
    let mut node = (*root).rb_node;

    while !node.is_null() {
        let here = rb_entry::<drbd_interval>(node);

        if sector < (*here).sector {
            node = (*node).rb_left;
        } else if sector > (*here).sector {
            node = (*node).rb_right;
        } else if (interval as usize) < (here as usize) {
            node = (*node).rb_left;
        } else if (interval as usize) > (here as usize) {
            node = (*node).rb_right;
        } else {
            return true;
        }
    }
    false
}

/*
 * drbd_remove_interval  -  remove an interval from a tree
 */
pub unsafe fn drbd_remove_interval(root: *mut rb_root, this: *mut drbd_interval) {
    /* avoid endless loop */
    if drbd_interval_empty(this) {
        return;
    }

    rb_erase_augmented(&mut (*this).rb, root, &augment_callbacks);
}

/**
 * drbd_find_overlap  - search for an interval overlapping with [sector, sector + size)
 * @root: red black tree root
 * @sector: start sector
 * @size: size, aligned to 512 bytes
 *
 * Returns an interval overlapping with [sector, sector + size), or NULL if
 * there is none.  When there is more than one overlapping interval in the
 * tree, the interval with the lowest start sector is returned, and all other
 * overlapping intervals will be on the right side of the tree, reachable with
 * rb_next().
 */
pub unsafe fn drbd_find_overlap(
    root: *mut rb_root,
    sector: sector_t,
    size: u32,
) -> *mut drbd_interval {
    let mut node = (*root).rb_node;
    let mut overlap: *mut drbd_interval = core::ptr::null_mut();
    let end: sector_t = sector + ((size as sector_t) >> 9);

    BUG_ON(!IS_ALIGNED(size, 512));

    while !node.is_null() {
        let here = rb_entry::<drbd_interval>(node);

        if !(*node).rb_left.is_null() && sector < interval_end((*node).rb_left) {
            /* Overlap if any must be on left side */
            node = (*node).rb_left;
        } else if (*here).sector < end
            && sector < (*here).sector + (((*here).size as sector_t) >> 9)
        {
            overlap = here;
            break;
        } else if sector >= (*here).sector {
            /* Overlap if any must be on right side */
            node = (*node).rb_right;
        } else {
            break;
        }
    }
    overlap
}

pub unsafe fn drbd_next_overlap(
    mut i: *mut drbd_interval,
    sector: sector_t,
    size: u32,
) -> *mut drbd_interval {
    let end: sector_t = sector + ((size as sector_t) >> 9);
    let mut node: *mut rb_node;

    loop {
        node = rb_next(&mut (*i).rb);
        if node.is_null() {
            return core::ptr::null_mut();
        }
        i = rb_entry::<drbd_interval>(node);
        if (*i).sector >= end {
            return core::ptr::null_mut();
        }
        if sector < (*i).sector + (((*i).size as sector_t) >> 9) {
            return i;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
