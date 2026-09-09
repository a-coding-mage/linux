// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit userspace memory allocation resource management.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct kunit_vm_mmap_resource {
    pub addr: ::core::ffi::c_ulong,
    pub size: usize,
}

/* vm_mmap() arguments */
#[repr(C)]
pub struct kunit_vm_mmap_params {
    pub file: *mut file,
    pub addr: ::core::ffi::c_ulong,
    pub len: ::core::ffi::c_ulong,
    pub prot: ::core::ffi::c_ulong,
    pub flag: ::core::ffi::c_ulong,
    pub offset: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mm_struct {
    pub task_size: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct signal_struct {
    pub rlim: [::core::ffi::c_ulong; 16],
}

#[repr(C)]
pub struct task_struct {
    pub mm: *mut mm_struct,
    pub signal: *mut signal_struct,
}

#[repr(C)]
pub struct kunit_resource {
    pub data: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut current: *mut task_struct;
    static TASK_SIZE: ::core::ffi::c_ulong;

    fn mm_alloc() -> *mut mm_struct;
    fn arch_pick_mmap_layout(mm: *mut mm_struct, rlim: *const ::core::ffi::c_ulong);
    fn kthread_use_mm(mm: *mut mm_struct);
    fn vm_mmap(
        file: *mut file,
        addr: ::core::ffi::c_ulong,
        len: ::core::ffi::c_ulong,
        prot: ::core::ffi::c_ulong,
        flag: ::core::ffi::c_ulong,
        offset: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong;
    fn vm_munmap(addr: ::core::ffi::c_ulong, size: ::core::ffi::c_ulong) -> ::core::ffi::c_long;
    fn kmemdup(
        src: *const ::core::ffi::c_void,
        size: usize,
        flags: ::core::ffi::c_ulong,
    ) -> *mut ::core::ffi::c_void;
    fn kfree(ptr: *mut ::core::ffi::c_void);
    fn kunit_alloc_resource(
        test: *mut kunit,
        init: unsafe extern "C" fn(*mut kunit_resource, *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        free: unsafe extern "C" fn(*mut kunit_resource),
        flags: ::core::ffi::c_ulong,
        context: *mut ::core::ffi::c_void,
    ) -> *mut kunit_resource;
}

const EINVAL: ::core::ffi::c_int = 22;
const ENOMEM: ::core::ffi::c_int = 12;
const RLIMIT_STACK: usize = 3;
const GFP_KERNEL: ::core::ffi::c_ulong = 0;

pub unsafe extern "C" fn kunit_attach_mm() -> ::core::ffi::c_int {
    let task = current;

    if !(*task).mm.is_null() {
        return 0;
    }

    /* arch_pick_mmap_layout() is only sane with MMU systems. */
    // CONFIG_MMU is a build-time configuration condition.
    if !cfg!(feature = "CONFIG_MMU") {
        return -EINVAL;
    }

    let mm = mm_alloc();
    if mm.is_null() {
        return -ENOMEM;
    }

    /* Define the task size. */
    (*mm).task_size = TASK_SIZE;

    /* Make sure we can allocate new VMAs. */
    arch_pick_mmap_layout(mm, &(*(*task).signal).rlim[RLIMIT_STACK]);

    /* Attach the mm. It will be cleaned up when the process dies. */
    kthread_use_mm(mm);

    0
}

unsafe extern "C" fn kunit_vm_mmap_init(
    res: *mut kunit_resource,
    context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let p = context as *mut kunit_vm_mmap_params;
    let mut vres = kunit_vm_mmap_resource {
        addr: 0,
        size: 0,
    };
    let ret = kunit_attach_mm();
    if ret != 0 {
        return ret;
    }

    vres.size = (*p).len as usize;
    vres.addr = vm_mmap((*p).file, (*p).addr, (*p).len, (*p).prot, (*p).flag, (*p).offset);
    if vres.addr == 0 {
        return -ENOMEM;
    }
    (*res).data = kmemdup(
        &vres as *const kunit_vm_mmap_resource as *const ::core::ffi::c_void,
        core::mem::size_of::<kunit_vm_mmap_resource>(),
        GFP_KERNEL,
    );
    if (*res).data.is_null() {
        vm_munmap(vres.addr, vres.size as ::core::ffi::c_ulong);
        return -ENOMEM;
    }

    0
}

unsafe extern "C" fn kunit_vm_mmap_free(res: *mut kunit_resource) {
    let vres = (*res).data;

    /*
     * Since this is executed from the test monitoring process,
     * the test's mm has already been torn down. We don't need
     * to run vm_munmap(vres->addr, vres->size), only clean up
     * the vres.
     */

    kfree(vres);
    (*res).data = core::ptr::null_mut();
}

pub unsafe extern "C" fn kunit_vm_mmap(
    test: *mut kunit,
    file: *mut file,
    addr: ::core::ffi::c_ulong,
    len: ::core::ffi::c_ulong,
    prot: ::core::ffi::c_ulong,
    flag: ::core::ffi::c_ulong,
    offset: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    let mut params = kunit_vm_mmap_params {
        file,
        addr,
        len,
        prot,
        flag,
        offset,
    };

    let vres = kunit_alloc_resource(
        test,
        kunit_vm_mmap_init,
        kunit_vm_mmap_free,
        GFP_KERNEL,
        &mut params as *mut kunit_vm_mmap_params as *mut ::core::ffi::c_void,
    );
    if !vres.is_null() {
        return (*(vres as *mut kunit_vm_mmap_resource)).addr;
    }
    0
}

// MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
