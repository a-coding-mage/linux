// SPDX-License-Identifier: GPL-2.0-only
/*
 * KVM L1 hypervisor optimizations on Hyper-V.
 */

// Dependency intent from the original source:
// linux/kvm_host.h, asm/mshyperv.h, hyperv.h, and kvm_onhyperv.h

#[repr(C)]
pub struct kvm_hv_tlb_range {
    pub start_gfn: u64,
    pub pages: u64,
}

unsafe fn kvm_fill_hv_flush_list_func(
    flush: *mut hv_guest_mapping_flush_list,
    data: *mut core::ffi::c_void,
) -> i32 {
    let range = data as *mut kvm_hv_tlb_range;

    hyperv_fill_flush_guest_mapping_list(
        flush,
        (*range).start_gfn,
        (*range).pages,
    )
}

#[inline]
unsafe fn hv_remote_flush_root_tdp(
    root_tdp: hpa_t,
    range: *mut kvm_hv_tlb_range,
) -> i32 {
    if !range.is_null() {
        hyperv_flush_guest_mapping_range(
            root_tdp,
            kvm_fill_hv_flush_list_func,
            range as *mut core::ffi::c_void,
        )
    } else {
        hyperv_flush_guest_mapping(root_tdp)
    }
}

unsafe fn __hv_flush_remote_tlbs_range(
    kvm: *mut kvm,
    range: *mut kvm_hv_tlb_range,
) -> i32 {
    let kvm_arch: *mut kvm_arch = &mut (*kvm).arch;
    let mut ret: i32 = 0;
    let mut nr_unique_valid_roots: i32;
    let mut i: usize;
    let mut root: hpa_t;

    spin_lock(&mut (*kvm_arch).hv_root_tdp_lock);

    if !VALID_PAGE((*kvm_arch).hv_root_tdp) {
        nr_unique_valid_roots = 0;

        /*
         * Flush all valid roots, and see if all vCPUs have converged
         * on a common root, in which case future flushes can skip the
         * loop and flush the common root.
         */
        kvm_for_each_vcpu!(i, vcpu, kvm, {
            root = (*vcpu).arch.hv_root_tdp;
            if !VALID_PAGE(root) || root == (*kvm_arch).hv_root_tdp {
                continue;
            }

            /*
             * Set the tracked root to the first valid root.  Keep
             * this root for the entirety of the loop even if more
             * roots are encountered as a low effort optimization
             * to avoid flushing the same (first) root again.
             */
            nr_unique_valid_roots += 1;
            if nr_unique_valid_roots == 1 {
                (*kvm_arch).hv_root_tdp = root;
            }

            if ret == 0 {
                ret = hv_remote_flush_root_tdp(root, range);
            }

            /*
             * Stop processing roots if a failure occurred and
             * multiple valid roots have already been detected.
             */
            if ret != 0 && nr_unique_valid_roots > 1 {
                break;
            }
        });

        /*
         * The optimized flush of a single root can't be used if there
         * are multiple valid roots (obviously).
         */
        if nr_unique_valid_roots > 1 {
            (*kvm_arch).hv_root_tdp = INVALID_PAGE;
        }
    } else {
        ret = hv_remote_flush_root_tdp((*kvm_arch).hv_root_tdp, range);
    }

    spin_unlock(&mut (*kvm_arch).hv_root_tdp_lock);
    ret
}

pub unsafe fn hv_flush_remote_tlbs_range(
    kvm: *mut kvm,
    start_gfn: gfn_t,
    nr_pages: gfn_t,
) -> i32 {
    let mut range = kvm_hv_tlb_range {
        start_gfn,
        pages: nr_pages,
    };

    __hv_flush_remote_tlbs_range(kvm, &mut range)
}

// EXPORT_SYMBOL_FOR_KVM_INTERNAL(hv_flush_remote_tlbs_range);

pub unsafe fn hv_flush_remote_tlbs(kvm: *mut kvm) -> i32 {
    __hv_flush_remote_tlbs_range(kvm, core::ptr::null_mut())
}

// EXPORT_SYMBOL_FOR_KVM_INTERNAL(hv_flush_remote_tlbs);

pub unsafe fn hv_track_root_tdp(vcpu: *mut kvm_vcpu, root_tdp: hpa_t) {
    let kvm_arch: *mut kvm_arch = &mut (*(*vcpu).kvm).arch;

    if kvm_x86_ops.flush_remote_tlbs == hv_flush_remote_tlbs {
        spin_lock(&mut (*kvm_arch).hv_root_tdp_lock);
        (*vcpu).arch.hv_root_tdp = root_tdp;
        if root_tdp != (*kvm_arch).hv_root_tdp {
            (*kvm_arch).hv_root_tdp = INVALID_PAGE;
        }
        spin_unlock(&mut (*kvm_arch).hv_root_tdp_lock);
    }
}

// EXPORT_SYMBOL_FOR_KVM_INTERNAL(hv_track_root_tdp);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
