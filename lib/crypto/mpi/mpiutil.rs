/* mpiutil.ac  -  Utility functions for MPI
 * Copyright (C) 1998, 1999 Free Software Foundation, Inc.
 *
 * This file is part of GnuPG.
 *
 * GnuPG is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * GnuPG is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA
 */

/* Dependencies are supplied by mpi-internal and the kernel environment. */

/*
 * Note: It was a bad idea to use the number of limbs to allocate
 *       because on a alpha the limbs are large but we normally need
 *       integers of n bits - So we should change this to bits (or bytes).
 *
 *       But mpi_alloc is used in a lot of places :-)
 */
#[no_mangle]
pub unsafe extern "C" fn mpi_alloc(nlimbs: ::core::ffi::c_uint) -> MPI {
    let mut a: MPI;

    a = kmalloc_obj::<mpi_t>();
    if a.is_null() {
        return a;
    }

    if nlimbs != 0 {
        (*a).d = mpi_alloc_limb_space(nlimbs);
        if (*a).d.is_null() {
            kfree(a as *mut ::core::ffi::c_void);
            return core::ptr::null_mut();
        }
    } else {
        (*a).d = core::ptr::null_mut();
    }

    (*a).alloced = nlimbs;
    (*a).nlimbs = 0;
    (*a).sign = 0;
    (*a).flags = 0;
    (*a).nbits = 0;
    a
}

pub unsafe extern "C" fn mpi_alloc_limb_space(nlimbs: ::core::ffi::c_uint) -> mpi_ptr_t {
    let len: usize = (nlimbs as usize).wrapping_mul(::core::mem::size_of::<mpi_limb_t>());

    if len == 0 {
        return core::ptr::null_mut();
    }

    kmalloc(len, GFP_KERNEL)
}

pub unsafe extern "C" fn mpi_free_limb_space(a: mpi_ptr_t) {
    if a.is_null() {
        return;
    }

    kfree_sensitive(a);
}

pub unsafe extern "C" fn mpi_assign_limb_space(a: MPI, ap: mpi_ptr_t, nlimbs: ::core::ffi::c_uint) {
    mpi_free_limb_space((*a).d);
    (*a).d = ap;
    (*a).alloced = nlimbs;
}

/*
 * Resize the array of A to NLIMBS. the additional space is cleared
 * (set to 0) [done by m_realloc()]
 */
pub unsafe extern "C" fn mpi_resize(a: MPI, nlimbs: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_void;

    if nlimbs <= (*a).alloced {
        return 0; /* no need to do it */
    }

    if !(*a).d.is_null() {
        p = kzalloc_objs::<mpi_limb_t>(nlimbs);
        if p.is_null() {
            return -12; /* -ENOMEM */
        }
        core::ptr::copy_nonoverlapping(
            (*a).d as *const mpi_limb_t,
            p as *mut mpi_limb_t,
            ((*a).alloced as usize).wrapping_mul(core::mem::size_of::<mpi_limb_t>()) / core::mem::size_of::<mpi_limb_t>(),
        );
        kfree_sensitive((*a).d);
        (*a).d = p as mpi_ptr_t;
    } else {
        (*a).d = kzalloc_objs::<mpi_limb_t>(nlimbs) as mpi_ptr_t;
        if (*a).d.is_null() {
            return -12; /* -ENOMEM */
        }
    }
    (*a).alloced = nlimbs;
    0
}

pub unsafe extern "C" fn mpi_free(a: MPI) {
    if a.is_null() {
        return;
    }

    if (*a).flags & 4 != 0 {
        kfree_sensitive((*a).d);
    } else {
        mpi_free_limb_space((*a).d);
    }

    if (*a).flags & !7 != 0 {
        pr_info("invalid flag value in mpi\n");
    }
    kfree(a as *mut ::core::ffi::c_void);
}

/*
 * Note: This copy function should not interpret the MPI
 *       but copy it transparently.
 */
pub unsafe extern "C" fn mpi_copy(a: MPI) -> MPI {
    let mut b: MPI;

    if !a.is_null() {
        b = mpi_alloc((*a).nlimbs);
        if b.is_null() {
            return core::ptr::null_mut();
        }
        (*b).nlimbs = (*a).nlimbs;
        (*b).sign = (*a).sign;
        (*b).flags = (*a).flags;
        (*b).flags &= !(16 | 32); /* Reset the immutable and constant flags. */
        for i in 0..(*b).nlimbs as usize {
            (*b).d.add(i).write((*a).d.add(i).read());
        }
    } else {
        b = core::ptr::null_mut();
    }
    b
}

/* MODULE_DESCRIPTION("Multiprecision maths library"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
