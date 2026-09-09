/**
 * css_get - obtain a reference on the specified css
 * @css: target css
 *
 * The caller must already have a reference.
 */
pub unsafe fn css_get(css: *mut cgroup_subsys_state) {
    if ((*css).flags & CSS_NO_REF) == 0 {
        percpu_ref_get(&mut (*css).refcnt);
    }
}

/**
 * css_get_many - obtain references on the specified css
 * @css: target css
 * @n: number of references to get
 *
 * The caller must already have a reference.
 */
pub unsafe fn css_get_many(css: *mut cgroup_subsys_state, n: u32) {
    if ((*css).flags & CSS_NO_REF) == 0 {
        percpu_ref_get_many(&mut (*css).refcnt, n);
    }
}

/**
 * css_tryget - try to obtain a reference on the specified css
 * @css: target css
 *
 * Obtain a reference on @css unless it already has reached zero and is
 * being released.  This function doesn't care whether @css is on or
 * offline.  The caller naturally needs to ensure that @css is accessible
 * but doesn't have to be holding a reference on it - IOW, RCU protected
 * access is good enough for this function.  Returns %true if a reference
 * count was successfully obtained; %false otherwise.
 */
pub unsafe fn css_tryget(css: *mut cgroup_subsys_state) -> bool {
    if ((*css).flags & CSS_NO_REF) == 0 {
        return percpu_ref_tryget(&mut (*css).refcnt);
    }
    true
}

/**
 * css_tryget_online - try to obtain a reference on the specified css if online
 * @css: target css
 *
 * Obtain a reference on @css if it's online.  The caller naturally needs
 * to ensure that @css is accessible but doesn't have to be holding a
 * reference on it - IOW, RCU protected access is good enough for this
 * function.  Returns %true if a reference count was successfully obtained;
 * %false otherwise.
 */
pub unsafe fn css_tryget_online(css: *mut cgroup_subsys_state) -> bool {
    if ((*css).flags & CSS_NO_REF) == 0 {
        return percpu_ref_tryget_live(&mut (*css).refcnt);
    }
    true
}

/**
 * css_put - put a css reference
 * @css: target css
 *
 * Put a reference obtained via css_get() and css_tryget_online().
 */
pub unsafe fn css_put(css: *mut cgroup_subsys_state) {
    if ((*css).flags & CSS_NO_REF) == 0 {
        percpu_ref_put(&mut (*css).refcnt);
    }
}

/**
 * css_put_many - put css references
 * @css: target css
 * @n: number of references to put
 *
 * Put references obtained via css_get() and css_tryget_online().
 */
pub unsafe fn css_put_many(css: *mut cgroup_subsys_state, n: u32) {
    if ((*css).flags & CSS_NO_REF) == 0 {
        percpu_ref_put_many(&mut (*css).refcnt, n);
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
