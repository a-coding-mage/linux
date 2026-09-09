// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2019 Intel Corporation. */

// Dependencies supplied by the surrounding kernel Rust bindings.

/* The BPF dispatcher is a multiway branch code generator. The
 * dispatcher is a mechanism to avoid the performance penalty of an
 * indirect call, which is expensive when retpolines are enabled. A
 * dispatch client registers a BPF program into the dispatcher, and if
 * there is available room in the dispatcher a direct call to the BPF
 * program will be generated. All calls to the BPF programs called via
 * the dispatcher will then be a direct call, instead of an indirect.
 * The dispatcher hijacks a trampoline function it via the __fentry__
 * of the trampoline. The trampoline function has the following
 * signature:
 *
 * unsigned int trampoline(const void *ctx, const struct bpf_insn *insnsi,
 *                         unsigned int (*bpf_func)(const void *,
 *                                                  const struct bpf_insn *));
 */

unsafe fn bpf_dispatcher_find_prog(
    d: *mut bpf_dispatcher,
    prog: *mut bpf_prog,
) -> *mut bpf_dispatcher_prog {
    let mut i: i32 = 0;

    while i < BPF_DISPATCHER_MAX {
        if prog == (*d).progs[i as usize].prog {
            return &mut (*d).progs[i as usize] as *mut bpf_dispatcher_prog;
        }
        i += 1;
    }
    core::ptr::null_mut()
}

unsafe fn bpf_dispatcher_find_free(
    d: *mut bpf_dispatcher,
) -> *mut bpf_dispatcher_prog {
    bpf_dispatcher_find_prog(d, core::ptr::null_mut())
}

unsafe fn bpf_dispatcher_add_prog(
    d: *mut bpf_dispatcher,
    prog: *mut bpf_prog,
) -> bool {
    let mut entry: *mut bpf_dispatcher_prog;

    if prog.is_null() {
        return false;
    }

    entry = bpf_dispatcher_find_prog(d, prog);
    if !entry.is_null() {
        refcount_inc(&mut (*entry).users);
        return false;
    }

    entry = bpf_dispatcher_find_free(d);
    if entry.is_null() {
        return false;
    }

    bpf_prog_inc(prog);
    (*entry).prog = prog;
    refcount_set(&mut (*entry).users, 1);
    (*d).num_progs += 1;
    true
}

unsafe fn bpf_dispatcher_remove_prog(
    d: *mut bpf_dispatcher,
    prog: *mut bpf_prog,
) -> bool {
    let entry: *mut bpf_dispatcher_prog;

    if prog.is_null() {
        return false;
    }

    entry = bpf_dispatcher_find_prog(d, prog);
    if entry.is_null() {
        return false;
    }

    if refcount_dec_and_test(&mut (*entry).users) {
        (*entry).prog = core::ptr::null_mut();
        bpf_prog_put(prog);
        (*d).num_progs -= 1;
        return true;
    }
    false
}

#[unsafe(no_mangle)]
#[linkage = "weak"]
pub unsafe extern "C" fn arch_prepare_bpf_dispatcher(
    image: *mut core::ffi::c_void,
    buf: *mut core::ffi::c_void,
    funcs: *mut i64,
    num_funcs: i32,
) -> i32 {
    let _ = (image, buf, funcs, num_funcs);
    -ENOTSUPP
}

unsafe fn bpf_dispatcher_prepare(
    d: *mut bpf_dispatcher,
    image: *mut core::ffi::c_void,
    buf: *mut core::ffi::c_void,
) -> i32 {
    let mut ips: [i64; BPF_DISPATCHER_MAX as usize] = [0; BPF_DISPATCHER_MAX as usize];
    let mut ipsp: *mut i64 = &mut ips[0];
    let mut i: i32 = 0;

    while i < BPF_DISPATCHER_MAX {
        if !(*d).progs[i as usize].prog.is_null() {
            *ipsp = (*(*d).progs[i as usize].prog).bpf_func as usize as i64;
            ipsp = ipsp.add(1);
        }
        i += 1;
    }
    arch_prepare_bpf_dispatcher(image, buf, &mut ips[0], (*d).num_progs)
}

unsafe fn bpf_dispatcher_update(d: *mut bpf_dispatcher, prev_num_progs: i32) {
    let mut new: *mut core::ffi::c_void;
    let mut tmp: *mut core::ffi::c_void;
    let mut noff: usize = 0;

    if prev_num_progs != 0 {
        noff = (*d).image_off ^ (PAGE_SIZE / 2);
    }

    new = if (*d).num_progs != 0 { (*d).image.add(noff) } else { core::ptr::null_mut() };
    tmp = if (*d).num_progs != 0 { (*d).rw_image.add(noff) } else { core::ptr::null_mut() };
    if !new.is_null() {
        /* Prepare the dispatcher in d->rw_image. Then use
         * bpf_arch_text_copy to update d->image, which is RO+X.
         */
        if bpf_dispatcher_prepare(d, new, tmp) != 0 {
            return;
        }
        if IS_ERR(bpf_arch_text_copy(new, tmp, PAGE_SIZE / 2)) {
            return;
        }
    }

    __BPF_DISPATCHER_UPDATE(if !new.is_null() { new } else { &bpf_dispatcher_nop_func as *const _ as *mut _ });

    /* Make sure all the callers executing the previous/old half of the
     * image leave it, so following update call can modify it safely.
     */
    synchronize_rcu();

    if !new.is_null() {
        (*d).image_off = noff;
    }
}

pub unsafe fn bpf_dispatcher_change_prog(
    d: *mut bpf_dispatcher,
    from: *mut bpf_prog,
    to: *mut bpf_prog,
) {
    let mut changed = false;
    let prev_num_progs: i32;

    if from == to {
        return;
    }

    mutex_lock(&mut (*d).mutex);
    if (*d).image.is_null() {
        (*d).image = bpf_prog_pack_alloc(PAGE_SIZE, bpf_jit_fill_hole_with_zero, false);
        if (*d).image.is_null() {
            mutex_unlock(&mut (*d).mutex);
            return;
        }
        /* d->rw_image doesn't need to be in module memory range, so we
         * can use vzalloc.
         */
        (*d).rw_image = vzalloc(PAGE_SIZE);
        if (*d).rw_image.is_null() {
            bpf_prog_pack_free((*d).image, PAGE_SIZE);
            (*d).image = core::ptr::null_mut();
            mutex_unlock(&mut (*d).mutex);
            return;
        }
        bpf_image_ksym_init((*d).image, PAGE_SIZE, &mut (*d).ksym);
        bpf_image_ksym_add(&mut (*d).ksym);
    }

    prev_num_progs = (*d).num_progs;
    changed |= bpf_dispatcher_remove_prog(d, from);
    changed |= bpf_dispatcher_add_prog(d, to);

    if changed {
        bpf_dispatcher_update(d, prev_num_progs);
    }
    mutex_unlock(&mut (*d).mutex);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
