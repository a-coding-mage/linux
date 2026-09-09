// SPDX-License-Identifier: GPL-2.0

/* Container for pinned pfns / pages in frame_vector.c */
#[repr(C)]
pub struct frame_vector {
    pub nr_allocated: core::ffi::c_uint, /* Number of frames we have space for */
    pub nr_frames: core::ffi::c_uint,    /* Number of frames stored in ptrs array */
    pub got_ref: bool,                   /* Did we pin pages by getting page ref? */
    pub is_pfns: bool,                   /* Does array contain pages or pfns? */
    pub ptrs: [*mut core::ffi::c_void; 0], /* Array of pinned pfns / pages. Use
                                            * pfns_vector_pages() or pfns_vector_pfns()
                                            * for access */
}

pub struct page;

extern "C" {
    pub fn frame_vector_create(nr_frames: core::ffi::c_uint) -> *mut frame_vector;
    pub fn frame_vector_destroy(vec: *mut frame_vector);
    pub fn get_vaddr_frames(
        start: core::ffi::c_ulong,
        nr_pfns: core::ffi::c_uint,
        write: bool,
        vec: *mut frame_vector,
    ) -> core::ffi::c_int;
    pub fn put_vaddr_frames(vec: *mut frame_vector);
    pub fn frame_vector_to_pages(vec: *mut frame_vector) -> core::ffi::c_int;
    pub fn frame_vector_to_pfns(vec: *mut frame_vector);
}

extern "C" {
    pub fn ERR_PTR(error: core::ffi::c_int) -> *mut page;
}

#[inline]
pub unsafe fn frame_vector_count(vec: *mut frame_vector) -> core::ffi::c_uint {
    (*vec).nr_frames
}

#[inline]
pub unsafe fn frame_vector_pages(vec: *mut frame_vector) -> *mut *mut page {
    if (*vec).is_pfns {
        let err = frame_vector_to_pages(vec);

        if err != 0 {
            return ERR_PTR(err);
        }
    }
    (*vec).ptrs.as_mut_ptr() as *mut *mut page
}

#[inline]
pub unsafe fn frame_vector_pfns(vec: *mut frame_vector) -> *mut core::ffi::c_ulong {
    if !(*vec).is_pfns {
        frame_vector_to_pfns(vec);
    }
    (*vec).ptrs.as_mut_ptr() as *mut core::ffi::c_ulong
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
