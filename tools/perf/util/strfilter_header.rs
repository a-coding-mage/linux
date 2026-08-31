/* SPDX-License-Identifier: GPL-2.0 */
/* General purpose glob matching filter */

use core::ffi::{c_char, c_int};

/* A node of string filter */
#[repr(C)]
pub struct strfilter_node {
    pub l: *mut strfilter_node, /* Tree left branch (for &,|) */
    pub r: *mut strfilter_node, /* Tree right branch (for !,&,|) */
    pub p: *const c_char,       /* Operator or rule */
}

/* String filter */
#[repr(C)]
pub struct strfilter {
    pub root: *mut strfilter_node,
}

unsafe extern "C" {
    /**
     * strfilter__new - Create a new string filter
     * @rules: Filter rule, which is a combination of glob expressions.
     * @err: Pointer which points an error detected on @rules
     *
     * Parse @rules and return new strfilter. Return NULL if an error detected.
     * In that case, *@err will indicate where it is detected, and *@err is NULL
     * if a memory allocation is failed.
     */
    pub fn strfilter__new(rules: *const c_char, err: *mut *const c_char) -> *mut strfilter;

    /**
     * strfilter__or - Append an additional rule by logical-or
     * @filter: Original string filter
     * @rules: Filter rule to be appended at left of the root of
     *         @filter by using logical-or.
     * @err: Pointer which points an error detected on @rules
     *
     * Parse @rules and join it to the @filter by using logical-or.
     * Return 0 if success, or return the error code.
     */
    pub fn strfilter__or(
        filter: *mut strfilter,
        rules: *const c_char,
        err: *mut *const c_char,
    ) -> c_int;

    /**
     * strfilter__add - Append an additional rule by logical-and
     * @filter: Original string filter
     * @rules: Filter rule to be appended at left of the root of
     *         @filter by using logical-and.
     * @err: Pointer which points an error detected on @rules
     *
     * Parse @rules and join it to the @filter by using logical-and.
     * Return 0 if success, or return the error code.
     */
    pub fn strfilter__and(
        filter: *mut strfilter,
        rules: *const c_char,
        err: *mut *const c_char,
    ) -> c_int;

    /**
     * strfilter__compare - compare given string and a string filter
     * @filter: String filter
     * @str: target string
     *
     * Compare @str and @filter. Return true if the str match the rule
     */
    pub fn strfilter__compare(filter: *mut strfilter, str: *const c_char) -> bool;

    /**
     * strfilter__delete - delete a string filter
     * @filter: String filter to delete
     *
     * Delete @filter.
     */
    pub fn strfilter__delete(filter: *mut strfilter);

    /**
     * strfilter__string - Reconstruct a rule string from filter
     * @filter: String filter to reconstruct
     *
     * Reconstruct a rule string from @filter. This will be good for
     * debug messages. Note that returning string must be freed afterward.
     */
    pub fn strfilter__string(filter: *mut strfilter) -> *mut c_char;
}
