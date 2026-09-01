// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/* Copyright (c) 2024, Oracle and/or its affiliates. */

/* Translated from lib/bpf/btf_relocate.c.  C include and __KERNEL__
 * compatibility macro intent is preserved by referencing the same external BTF
 * and allocation helpers expected from surrounding bindings.
 */

pub type __u8 = u8;
pub type __u32 = u32;

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_header {
    pub magic: u16,
    pub version: u8,
    pub flags: u8,
    pub hdr_len: u32,
    pub type_off: u32,
    pub type_len: u32,
    pub str_off: u32,
    pub str_len: u32,
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
    pub info: __u32,
    pub size: __u32,
    pub type_: __u32,
}

#[repr(C)]
pub struct btf_array {
    pub type_: __u32,
    pub index_type: __u32,
    pub nelems: __u32,
}

#[repr(C)]
pub struct btf_field_iter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_relocate {
    pub btf: *mut btf,
    pub base_btf: *const btf,
    pub dist_base_btf: *const btf,
    pub nr_base_types: u32,
    pub nr_split_types: u32,
    pub nr_dist_base_types: u32,
    pub dist_str_len: i32,
    pub base_str_len: i32,
    pub id_map: *mut __u32,
    pub str_map: *mut __u32,
}

/* Set temporarily in relocation id_map if distilled base struct/union is
 * embedded in a split BTF struct/union; in such a case, size information must
 * match between distilled base BTF and base BTF representation of type.
 */
pub const BTF_IS_EMBEDDED: __u32 = -1i32 as __u32;

/* <name, size, id> triple used in sorting/searching distilled base BTF.
 * The C source stores needs_size as a one-bit bitfield and size as 31 bits.
 */
#[repr(C)]
pub struct btf_name_info {
    pub name: *const ::std::os::raw::c_char,
    pub needs_size: bool,
    pub size: u32,
    pub id: __u32,
}

extern "C" {
    fn btf_type_by_id(btf: *const btf, id: __u32) -> *mut btf_type;
    fn btf__type_cnt(btf: *const btf) -> u32;
    fn btf__base_btf(btf: *const btf) -> *const btf;
    fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const ::std::os::raw::c_char;
    fn btf__str_by_offset(btf: *const btf, offset: __u32) -> *const ::std::os::raw::c_char;
    fn btf_kflag(t: *const btf_type) -> bool;
    fn btf_kind(t: *const btf_type) -> i32;
    fn btf_is_composite(t: *const btf_type) -> bool;
    fn btf_array(t: *const btf_type) -> *mut btf_array;
    fn btf_int_encoding(t: *const btf_type) -> u32;
    fn btf_header(btf: *const btf) -> *const btf_header;
    fn btf_set_base_btf(btf: *mut btf, base_btf: *const btf);
    fn btf_field_iter_init(it: *mut btf_field_iter, t: *mut btf_type, kind: i32) -> i32;
    fn btf_field_iter_next(it: *mut btf_field_iter) -> *mut __u32;
    fn calloc(nmemb: usize, size: usize) -> *mut ::std::ffi::c_void;
    fn free(ptr: *mut ::std::ffi::c_void);
    fn qsort(
        base: *mut ::std::ffi::c_void,
        nmemb: usize,
        size: usize,
        compar: Option<
            unsafe extern "C" fn(
                *const ::std::ffi::c_void,
                *const ::std::ffi::c_void,
            ) -> i32,
        >,
    );
    fn strcmp(
        s1: *const ::std::os::raw::c_char,
        s2: *const ::std::os::raw::c_char,
    ) -> i32;
    fn pr_warn(fmt: *const ::std::os::raw::c_char, ...);
}

unsafe fn btf_relocate_rewrite_type_id(r: *mut btf_relocate, i: __u32) -> i32 {
    let t: *mut btf_type = btf_type_by_id((*r).btf, i);
    let mut it: btf_field_iter = ::std::mem::zeroed();
    let mut id: *mut __u32;
    let err: i32;

    err = btf_field_iter_init(&mut it, t, BTF_FIELD_ITER_IDS);
    if err != 0 {
        return err;
    }

    loop {
        id = btf_field_iter_next(&mut it);
        if id.is_null() {
            break;
        }
        *id = *((*r).id_map.add(*id as usize));
    }
    0
}

/* Simple string comparison used for sorting within BTF, since all distilled
 * types are named.  If strings match, and size is non-zero for both elements
 * fall back to using size for ordering.
 */
unsafe extern "C" fn cmp_btf_name_size(
    n1: *const ::std::ffi::c_void,
    n2: *const ::std::ffi::c_void,
) -> i32 {
    let ni1: *const btf_name_info = n1 as *const btf_name_info;
    let ni2: *const btf_name_info = n2 as *const btf_name_info;
    let name_diff: i32 = strcmp((*ni1).name, (*ni2).name);

    if name_diff == 0 && (*ni1).needs_size && (*ni2).needs_size {
        return (*ni2).size as i32 - (*ni1).size as i32;
    }
    name_diff
}

/* Binary search with a small twist; find leftmost element that matches
 * so that we can then iterate through all exact matches.  So for example
 * searching { "a", "bb", "bb", "c" }  we would always match on the
 * leftmost "bb".
 */
unsafe fn search_btf_name_size(
    key: *mut btf_name_info,
    vals: *mut btf_name_info,
    nelems: i32,
) -> *mut btf_name_info {
    let mut ret: *mut btf_name_info = ::std::ptr::null_mut();
    let mut high: i32 = nelems - 1;
    let mut low: i32 = 0;

    while low <= high {
        let mid: i32 = (low + high) / 2;
        let val: *mut btf_name_info = vals.add(mid as usize);
        let diff: i32 = cmp_btf_name_size(key as *const _, val as *const _);

        if diff == 0 {
            ret = val;
        }
        /* even if found, keep searching for leftmost match */
        if diff <= 0 {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    ret
}

/* If a member of a split BTF struct/union refers to a base BTF
 * struct/union, mark that struct/union id temporarily in the id_map
 * with BTF_IS_EMBEDDED.  Members can be const/restrict/volatile/typedef
 * reference types, but if a pointer is encountered, the type is no longer
 * considered embedded.
 */
unsafe fn btf_mark_embedded_composite_type_ids(r: *mut btf_relocate, i: __u32) -> i32 {
    let mut t: *mut btf_type = btf_type_by_id((*r).btf, i);
    let mut it: btf_field_iter = ::std::mem::zeroed();
    let mut id: *mut __u32;
    let err: i32;

    if !btf_is_composite(t) {
        return 0;
    }

    err = btf_field_iter_init(&mut it, t, BTF_FIELD_ITER_IDS);
    if err != 0 {
        return err;
    }

    loop {
        id = btf_field_iter_next(&mut it);
        if id.is_null() {
            break;
        }
        let mut next_id: __u32 = *id;

        while next_id != 0 {
            t = btf_type_by_id((*r).btf, next_id);
            match btf_kind(t) {
                BTF_KIND_CONST | BTF_KIND_RESTRICT | BTF_KIND_VOLATILE | BTF_KIND_TYPEDEF
                | BTF_KIND_TYPE_TAG => {
                    next_id = (*t).type_;
                }
                BTF_KIND_ARRAY => {
                    let a: *mut btf_array = btf_array(t);

                    next_id = (*a).type_;
                }
                BTF_KIND_STRUCT | BTF_KIND_UNION => {
                    if next_id < (*r).nr_dist_base_types {
                        *((*r).id_map.add(next_id as usize)) = BTF_IS_EMBEDDED;
                    }
                    next_id = 0;
                }
                _ => {
                    next_id = 0;
                }
            }
        }
    }

    0
}

/* Build a map from distilled base BTF ids to base BTF ids. To do so, iterate
 * through base BTF looking up distilled type (using binary search) equivalents.
 */
unsafe fn btf_relocate_map_distilled_base(r: *mut btf_relocate) -> i32 {
    let mut info: *mut btf_name_info;
    let info_end: *mut btf_name_info;
    let mut base_t: *mut btf_type;
    let mut dist_t: *mut btf_type;
    let mut base_name_cnt: *mut __u8 = ::std::ptr::null_mut();
    let mut err: i32 = 0;
    let mut id: __u32;

    /* generate a sort index array of name/type ids sorted by name for
     * distilled base BTF to speed name-based lookups.
     */
    info = calloc(
        (*r).nr_dist_base_types as usize,
        ::std::mem::size_of::<btf_name_info>(),
    ) as *mut btf_name_info;
    if info.is_null() {
        err = -ENOMEM;
        free(base_name_cnt as *mut _);
        free(info as *mut _);
        return err;
    }
    info_end = info.add((*r).nr_dist_base_types as usize);
    id = 0;
    while id < (*r).nr_dist_base_types {
        dist_t = btf_type_by_id((*r).dist_base_btf, id);
        (*info.add(id as usize)).name = btf__name_by_offset((*r).dist_base_btf, (*dist_t).name_off);
        (*info.add(id as usize)).id = id;
        (*info.add(id as usize)).size = (*dist_t).size;
        (*info.add(id as usize)).needs_size = true;
        id += 1;
    }
    qsort(
        info as *mut _,
        (*r).nr_dist_base_types as usize,
        ::std::mem::size_of::<btf_name_info>(),
        Some(cmp_btf_name_size),
    );

    /* Mark distilled base struct/union members of split BTF structs/unions
     * in id_map with BTF_IS_EMBEDDED; this signals that these types
     * need to match both name and size, otherwise embedding the base
     * struct/union in the split type is invalid.
     */
    id = (*r).nr_dist_base_types;
    while id < (*r).nr_dist_base_types + (*r).nr_split_types {
        err = btf_mark_embedded_composite_type_ids(r, id);
        if err != 0 {
            free(base_name_cnt as *mut _);
            free(info as *mut _);
            return err;
        }
        id += 1;
    }

    /* Collect name counts for composite types in base BTF.  If multiple
     * instances of a struct/union of the same name exist, we need to use
     * size to determine which to map to since name alone is ambiguous.
     */
    base_name_cnt = calloc((*r).base_str_len as usize, ::std::mem::size_of::<__u8>()) as *mut __u8;
    if base_name_cnt.is_null() {
        err = -ENOMEM;
        free(info as *mut _);
        return err;
    }
    id = 1;
    while id < (*r).nr_base_types {
        base_t = btf_type_by_id((*r).base_btf, id);
        if !btf_is_composite(base_t) || (*base_t).name_off == 0 {
            id += 1;
            continue;
        }
        if *base_name_cnt.add((*base_t).name_off as usize) < 255 {
            *base_name_cnt.add((*base_t).name_off as usize) += 1;
        }
        id += 1;
    }

    /* Now search base BTF for matching distilled base BTF types. */
    id = 1;
    while id < (*r).nr_base_types {
        let mut dist_info: *mut btf_name_info;
        let mut base_info: btf_name_info = ::std::mem::zeroed();
        let dist_kind: i32;
        let base_kind: i32;

        base_t = btf_type_by_id((*r).base_btf, id);
        /* distilled base consists of named types only. */
        if (*base_t).name_off == 0 {
            id += 1;
            continue;
        }
        base_kind = btf_kind(base_t);
        base_info.id = id;
        base_info.name = btf__name_by_offset((*r).base_btf, (*base_t).name_off);
        match base_kind {
            BTF_KIND_INT | BTF_KIND_FLOAT | BTF_KIND_ENUM | BTF_KIND_ENUM64 => {
                /* These types should match both name and size */
                base_info.needs_size = true;
                base_info.size = (*base_t).size;
            }
            BTF_KIND_FWD => {
                /* No size considerations for fwds. */
            }
            BTF_KIND_STRUCT | BTF_KIND_UNION => {
                /* Size only needs to be used for struct/union if there
                 * are multiple types in base BTF with the same name.
                 * If there are multiple _distilled_ types with the same
                 * name (a very unlikely scenario), that doesn't matter
                 * unless corresponding _base_ types to match them are
                 * missing.
                 */
                base_info.needs_size = *base_name_cnt.add((*base_t).name_off as usize) > 1;
                base_info.size = (*base_t).size;
            }
            _ => {
                id += 1;
                continue;
            }
        }
        /* iterate over all matching distilled base types */
        dist_info = search_btf_name_size(&mut base_info, info, (*r).nr_dist_base_types as i32);
        while !dist_info.is_null()
            && dist_info < info_end
            && cmp_btf_name_size(&mut base_info as *mut _ as *const _, dist_info as *const _) == 0
        {
            if (*dist_info).id == 0 || (*dist_info).id >= (*r).nr_dist_base_types {
                pr_warn(
                    b"base BTF id [%u] maps to invalid distilled base BTF id [%u]\n\0".as_ptr()
                        as *const _,
                    id,
                    (*dist_info).id,
                );
                err = -EINVAL;
                free(base_name_cnt as *mut _);
                free(info as *mut _);
                return err;
            }
            dist_t = btf_type_by_id((*r).dist_base_btf, (*dist_info).id);
            dist_kind = btf_kind(dist_t);

            /* Validate that the found distilled type is compatible.
             * Do not error out on mismatch as another match may
             * occur for an identically-named type.
             */
            match dist_kind {
                BTF_KIND_FWD => match base_kind {
                    BTF_KIND_FWD => {
                        if btf_kflag(dist_t) != btf_kflag(base_t) {
                            dist_info = dist_info.add(1);
                            continue;
                        }
                    }
                    BTF_KIND_STRUCT => {
                        if btf_kflag(base_t) {
                            dist_info = dist_info.add(1);
                            continue;
                        }
                    }
                    BTF_KIND_UNION => {
                        if !btf_kflag(base_t) {
                            dist_info = dist_info.add(1);
                            continue;
                        }
                    }
                    _ => {
                        dist_info = dist_info.add(1);
                        continue;
                    }
                },
                BTF_KIND_INT => {
                    if dist_kind != base_kind || btf_int_encoding(base_t) != btf_int_encoding(dist_t)
                    {
                        dist_info = dist_info.add(1);
                        continue;
                    }
                }
                BTF_KIND_FLOAT => {
                    if dist_kind != base_kind {
                        dist_info = dist_info.add(1);
                        continue;
                    }
                }
                BTF_KIND_ENUM => {
                    /* ENUM and ENUM64 are encoded as sized ENUM in
                     * distilled base BTF.
                     */
                    if base_kind != dist_kind && base_kind != BTF_KIND_ENUM64 {
                        dist_info = dist_info.add(1);
                        continue;
                    }
                }
                BTF_KIND_STRUCT | BTF_KIND_UNION => {
                    /* size verification is required for embedded
                     * struct/unions.
                     */
                    if *((*r).id_map.add((*dist_info).id as usize)) == BTF_IS_EMBEDDED
                        && (*base_t).size != (*dist_t).size
                    {
                        dist_info = dist_info.add(1);
                        continue;
                    }
                }
                _ => {
                    dist_info = dist_info.add(1);
                    continue;
                }
            }
            if *((*r).id_map.add((*dist_info).id as usize)) != 0
                && *((*r).id_map.add((*dist_info).id as usize)) != BTF_IS_EMBEDDED
            {
                /* we already have a match; this tells us that
                 * multiple base types of the same name
                 * have the same size, since for cases where
                 * multiple types have the same name we match
                 * on name and size.  In this case, we have
                 * no way of determining which to relocate
                 * to in base BTF, so error out.
                 */
                pr_warn(
                    b"distilled base BTF type '%s' [%u], size %u has multiple candidates of the same size (ids [%u, %u]) in base BTF\n\0".as_ptr()
                        as *const _,
                    base_info.name,
                    (*dist_info).id,
                    (*base_t).size,
                    id,
                    *((*r).id_map.add((*dist_info).id as usize)),
                );
                err = -EINVAL;
                free(base_name_cnt as *mut _);
                free(info as *mut _);
                return err;
            }
            /* map id and name */
            *((*r).id_map.add((*dist_info).id as usize)) = id;
            *((*r).str_map.add((*dist_t).name_off as usize)) = (*base_t).name_off;
            dist_info = dist_info.add(1);
        }
        id += 1;
    }
    /* ensure all distilled BTF ids now have a mapping... */
    id = 1;
    while id < (*r).nr_dist_base_types {
        let name: *const ::std::os::raw::c_char;

        if *((*r).id_map.add(id as usize)) != 0
            && *((*r).id_map.add(id as usize)) != BTF_IS_EMBEDDED
        {
            id += 1;
            continue;
        }
        dist_t = btf_type_by_id((*r).dist_base_btf, id);
        name = btf__name_by_offset((*r).dist_base_btf, (*dist_t).name_off);
        pr_warn(
            b"distilled base BTF type '%s' [%u] is not mapped to base BTF id\n\0".as_ptr()
                as *const _,
            name,
            id,
        );
        err = -EINVAL;
        break;
    }
    free(base_name_cnt as *mut _);
    free(info as *mut _);
    err
}

/* distilled base should only have named int/float/enum/fwd/struct/union types. */
unsafe fn btf_relocate_validate_distilled_base(r: *mut btf_relocate) -> i32 {
    let mut i: u32;

    i = 1;
    while i < (*r).nr_dist_base_types {
        let t: *mut btf_type = btf_type_by_id((*r).dist_base_btf, i);
        let kind: i32 = btf_kind(t);

        match kind {
            BTF_KIND_INT | BTF_KIND_FLOAT | BTF_KIND_ENUM | BTF_KIND_STRUCT | BTF_KIND_UNION
            | BTF_KIND_FWD => {
                if (*t).name_off != 0 {
                    i += 1;
                    continue;
                }
                pr_warn(
                    b"type [%u], kind [%d] is invalid for distilled base BTF; it is anonymous\n\0"
                        .as_ptr() as *const _,
                    i,
                    kind,
                );
                return -EINVAL;
            }
            _ => {
                pr_warn(
                    b"type [%u] in distilled based BTF has unexpected kind [%d]\n\0".as_ptr()
                        as *const _,
                    i,
                    kind,
                );
                return -EINVAL;
            }
        }
    }
    0
}

unsafe fn btf_relocate_rewrite_strs(r: *mut btf_relocate, i: __u32) -> i32 {
    let t: *mut btf_type = btf_type_by_id((*r).btf, i);
    let mut it: btf_field_iter = ::std::mem::zeroed();
    let mut str_off: *mut __u32;
    let mut off: i32;
    let err: i32;

    err = btf_field_iter_init(&mut it, t, BTF_FIELD_ITER_STRS);
    if err != 0 {
        return err;
    }

    loop {
        str_off = btf_field_iter_next(&mut it);
        if str_off.is_null() {
            break;
        }
        if *str_off == 0 {
            continue;
        }
        if *str_off >= (*r).dist_str_len as __u32 {
            *str_off = (*str_off).wrapping_add(((*r).base_str_len - (*r).dist_str_len) as __u32);
        } else {
            off = *((*r).str_map.add(*str_off as usize)) as i32;
            if off == 0 {
                pr_warn(
                    b"string '%s' [offset %u] is not mapped to base BTF\n\0".as_ptr()
                        as *const _,
                    btf__str_by_offset((*r).btf, off as __u32),
                    *str_off,
                );
                return -ENOENT;
            }
            *str_off = off as __u32;
        }
    }
    0
}

/* If successful, output of relocation is updated BTF with base BTF pointing
 * at base_btf, and type ids, strings adjusted accordingly.
 */
#[no_mangle]
pub unsafe extern "C" fn btf_relocate(
    btf: *mut btf,
    base_btf: *const btf,
    id_map: *mut *mut __u32,
) -> i32 {
    let nr_types: u32 = btf__type_cnt(btf);
    let dist_base_hdr: *const btf_header;
    let base_hdr: *const btf_header;
    let mut r: btf_relocate = ::std::mem::zeroed();
    let mut err: i32 = 0;
    let mut id: __u32;
    let mut i: __u32;

    r.dist_base_btf = btf__base_btf(btf);
    if base_btf.is_null() || r.dist_base_btf == base_btf {
        return -EINVAL;
    }

    r.nr_dist_base_types = btf__type_cnt(r.dist_base_btf);
    r.nr_base_types = btf__type_cnt(base_btf);
    r.nr_split_types = nr_types - r.nr_dist_base_types;
    r.btf = btf;
    r.base_btf = base_btf;

    r.id_map = calloc(nr_types as usize, ::std::mem::size_of::<__u32>()) as *mut __u32;
    r.str_map = calloc(
        (*btf_header(r.dist_base_btf)).str_len as usize,
        ::std::mem::size_of::<__u32>(),
    ) as *mut __u32;
    dist_base_hdr = btf_header(r.dist_base_btf);
    base_hdr = btf_header(r.base_btf);
    r.dist_str_len = (*dist_base_hdr).str_len as i32;
    r.base_str_len = (*base_hdr).str_len as i32;
    if r.id_map.is_null() || r.str_map.is_null() {
        err = -ENOMEM;
        free(r.id_map as *mut _);
        free(r.str_map as *mut _);
        return err;
    }

    err = btf_relocate_validate_distilled_base(&mut r);
    if err != 0 {
        free(r.id_map as *mut _);
        free(r.str_map as *mut _);
        return err;
    }

    /* Split BTF ids need to be adjusted as base and distilled base
     * have different numbers of types, changing the start id of split
     * BTF.
     */
    id = r.nr_dist_base_types;
    while id < nr_types {
        *r.id_map.add(id as usize) = id + r.nr_base_types - r.nr_dist_base_types;
        id += 1;
    }

    /* Build a map from distilled base ids to actual base BTF ids; it is used
     * to update split BTF id references.  Also build a str_map mapping from
     * distilled base BTF names to base BTF names.
     */
    err = btf_relocate_map_distilled_base(&mut r);
    if err != 0 {
        free(r.id_map as *mut _);
        free(r.str_map as *mut _);
        return err;
    }

    /* Next, rewrite type ids in split BTF, replacing split ids with updated
     * ids based on number of types in base BTF, and base ids with
     * relocated ids from base_btf.
     */
    i = 0;
    id = r.nr_dist_base_types;
    while i < r.nr_split_types {
        err = btf_relocate_rewrite_type_id(&mut r, id);
        if err != 0 {
            free(r.id_map as *mut _);
            free(r.str_map as *mut _);
            return err;
        }
        i += 1;
        id += 1;
    }
    /* String offsets now need to be updated using the str_map. */
    i = 0;
    while i < r.nr_split_types {
        err = btf_relocate_rewrite_strs(&mut r, i + r.nr_dist_base_types);
        if err != 0 {
            free(r.id_map as *mut _);
            free(r.str_map as *mut _);
            return err;
        }
        i += 1;
    }
    /* Finally reset base BTF to be base_btf */
    btf_set_base_btf(btf, base_btf);

    if !id_map.is_null() {
        *id_map = r.id_map;
        r.id_map = ::std::ptr::null_mut();
    }
    free(r.id_map as *mut _);
    free(r.str_map as *mut _);
    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
