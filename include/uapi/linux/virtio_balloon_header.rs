/* This header is BSD licensed so anyone can use the definitions to implement
 * compatible drivers/servers.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of IBM nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL IBM OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE. */

/* The following types correspond to linux/types.h and linux/virtio_types.h. */
pub type __le32 = u32;
pub type __virtio16 = u16;
pub type __virtio64 = u64;

/* The feature bitmap for virtio balloon */
pub const VIRTIO_BALLOON_F_MUST_TELL_HOST: u32 = 0; /* Tell before reclaiming pages */
pub const VIRTIO_BALLOON_F_STATS_VQ: u32 = 1; /* Memory Stats virtqueue */
pub const VIRTIO_BALLOON_F_DEFLATE_ON_OOM: u32 = 2; /* Deflate balloon on OOM */
pub const VIRTIO_BALLOON_F_FREE_PAGE_HINT: u32 = 3; /* VQ to report free pages */
pub const VIRTIO_BALLOON_F_PAGE_POISON: u32 = 4; /* Guest is using page poisoning */
pub const VIRTIO_BALLOON_F_REPORTING: u32 = 5; /* Page reporting virtqueue */

/* Size of a PFN in the balloon interface. */
pub const VIRTIO_BALLOON_PFN_SHIFT: u32 = 12;

pub const VIRTIO_BALLOON_CMD_ID_STOP: u32 = 0;
pub const VIRTIO_BALLOON_CMD_ID_DONE: u32 = 1;

#[repr(C)]
pub union virtio_balloon_config__bindgen_ty_1 {
    pub free_page_hint_cmd_id: __le32,
    pub free_page_report_cmd_id: __le32, /* deprecated */
}

#[repr(C)]
pub struct virtio_balloon_config {
    /* Number of pages host wants Guest to give up. */
    pub num_pages: __le32,
    /* Number of pages we've actually got in balloon. */
    pub actual: __le32,
    /*
     * Free page hint command id, readonly by guest.
     * Was previously named free_page_report_cmd_id so we
     * need to carry that name for legacy support.
     */
    pub free_page_hint_cmd_id: virtio_balloon_config__bindgen_ty_1,
    /* Stores PAGE_POISON if page poisoning is in use */
    pub poison_val: __le32,
}

pub const VIRTIO_BALLOON_S_SWAP_IN: u32 = 0;   /* Amount of memory swapped in */
pub const VIRTIO_BALLOON_S_SWAP_OUT: u32 = 1;   /* Amount of memory swapped out */
pub const VIRTIO_BALLOON_S_MAJFLT: u32 = 2;   /* Number of major faults */
pub const VIRTIO_BALLOON_S_MINFLT: u32 = 3;   /* Number of minor faults */
pub const VIRTIO_BALLOON_S_MEMFREE: u32 = 4;   /* Total amount of free memory */
pub const VIRTIO_BALLOON_S_MEMTOT: u32 = 5;   /* Total amount of memory */
pub const VIRTIO_BALLOON_S_AVAIL: u32 = 6;   /* Available memory as in /proc */
pub const VIRTIO_BALLOON_S_CACHES: u32 = 7;   /* Disk caches */
pub const VIRTIO_BALLOON_S_HTLB_PGALLOC: u32 = 8;  /* Hugetlb page allocations */
pub const VIRTIO_BALLOON_S_HTLB_PGFAIL: u32 = 9;  /* Hugetlb page allocation failures */
pub const VIRTIO_BALLOON_S_OOM_KILL: u32 = 10; /* OOM killer invocations */
pub const VIRTIO_BALLOON_S_ALLOC_STALL: u32 = 11; /* Stall count of memory allocatoin */
pub const VIRTIO_BALLOON_S_ASYNC_SCAN: u32 = 12; /* Amount of memory scanned asynchronously */
pub const VIRTIO_BALLOON_S_DIRECT_SCAN: u32 = 13; /* Amount of memory scanned directly */
pub const VIRTIO_BALLOON_S_ASYNC_RECLAIM: u32 = 14; /* Amount of memory reclaimed asynchronously */
pub const VIRTIO_BALLOON_S_DIRECT_RECLAIM: u32 = 15; /* Amount of memory reclaimed directly */
pub const VIRTIO_BALLOON_S_NR: u32 = 16;

macro_rules! VIRTIO_BALLOON_S_NAMES_WITH_PREFIX {
    ($prefix:expr) => {
        [
            concat!($prefix, "swap-in"), concat!($prefix, "swap-out"),
            concat!($prefix, "major-faults"), concat!($prefix, "minor-faults"),
            concat!($prefix, "free-memory"), concat!($prefix, "total-memory"),
            concat!($prefix, "available-memory"), concat!($prefix, "disk-caches"),
            concat!($prefix, "hugetlb-allocations"), concat!($prefix, "hugetlb-failures"),
            concat!($prefix, "oom-kills"), concat!($prefix, "alloc-stalls"),
            concat!($prefix, "async-scans"), concat!($prefix, "direct-scans"),
            concat!($prefix, "async-reclaims"), concat!($prefix, "direct-reclaims"),
        ]
    };
}

pub const VIRTIO_BALLOON_S_NAMES: [&str; 16] = VIRTIO_BALLOON_S_NAMES_WITH_PREFIX!("");

#[repr(C, packed)]
pub struct virtio_balloon_stat {
    pub tag: __virtio16,
    pub val: __virtio64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
