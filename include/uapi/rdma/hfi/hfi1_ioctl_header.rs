/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * GPL LICENSE SUMMARY
 *
 * Copyright(c) 2015 Intel Corporation.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * BSD LICENSE
 *
 * Copyright(c) 2015 Intel Corporation.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 *  - Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  - Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *  - Neither the name of Intel Corporation nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

/* linux/types.h supplies the original __u* and __aligned_u64 types. */

/*
 * This structure is passed to the driver to tell it where
 * user code buffers are, sizes, etc. The offsets and sizes of the
 * fields must remain unchanged, for binary compatibility. It can
 * be extended, if userversion is changed so user code can tell, if needed.
 */
#[repr(C)]
pub struct hfi1_user_info {
    /* version of user software, to detect compatibility issues.
     * Should be set to HFI1_USER_SWVERSION. */
    pub userversion: u32,
    pub pad: u32,
    /* If two or more processes wish to share a context, each process
     * must set the subcontext_cnt and subcontext_id to the same
     * values. The only restriction on the subcontext_id is that
     * it be unique for a given node. */
    pub subctxt_cnt: u16,
    pub subctxt_id: u16,
    /* 128bit UUID passed in by PSM. */
    pub uuid: [u8; 16],
}

#[repr(C)]
pub struct hfi1_ctxt_info {
    pub runtime_flags: u64,    /* chip/drv runtime flags (HFI1_CAP_*) */
    pub rcvegr_size: u32,      /* size of each eager buffer */
    pub num_active: u16,       /* number of active units */
    pub unit: u16,             /* unit (chip) assigned to caller */
    pub ctxt: u16,             /* ctxt on unit assigned to caller */
    pub subctxt: u16,          /* subctxt on unit assigned to caller */
    pub rcvtids: u16,          /* number of Rcv TIDs for this context */
    pub credits: u16,          /* number of PIO credits for this context */
    pub numa_node: u16,        /* NUMA node of the assigned device */
    pub rec_cpu: u16,          /* cpu # for affinity (0xffff if none) */
    pub send_ctxt: u16,        /* send context in use by this user context */
    pub egrtids: u16,          /* number of RcvArray entries for Eager Rcvs */
    pub rcvhdrq_cnt: u16,      /* number of RcvHdrQ entries */
    pub rcvhdrq_entsize: u16,  /* size (in bytes) for each RcvHdrQ entry */
    pub sdma_ring_size: u16,   /* number of entries in SDMA request ring */
}

#[repr(C)]
pub struct hfi1_tid_info {
    /* virtual address of first page in transfer */
    pub vaddr: u64,
    /* pointer to tid array. this array is big enough */
    pub tidlist: u64,
    /* number of tids programmed by this request */
    pub tidcnt: u32,
    /* length of transfer buffer programmed by this request */
    pub length: u32,
}

/*
 * This structure is returned by the driver immediately after
 * open to get implementation-specific info, and info specific to this
 * instance.
 *
 * This struct must have explicit pad fields where type sizes
 * may result in different alignments between 32 and 64 bit
 * programs, since the 64 bit kernel requires the user code
 * to have matching offsets.
 */
#[repr(C)]
pub struct hfi1_base_info {
    pub hw_version: u32,          /* version of hardware, for feature checking. */
    pub sw_version: u32,          /* version of software, for feature checking. */
    pub jkey: u16,                /* Job key */
    pub padding1: u16,
    /* The special QP (queue pair) value that identifies PSM
     * protocol packet from standard IB packets. */
    pub bthqp: u32,
    pub sc_credits_addr: u64,     /* PIO credit return address, */
    /* Base address of write-only pio buffers for this process.
     * Each buffer has sendpio_credits*64 bytes. */
    pub pio_bufbase_sop: u64,
    /* Base address of write-only pio buffers for this process.
     * Each buffer has sendpio_credits*64 bytes. */
    pub pio_bufbase: u64,
    pub rcvhdr_bufbase: u64,      /* address where receive buffer queue is mapped into */
    pub rcvegr_bufbase: u64,      /* base address of Eager receive buffers. */
    pub sdma_comp_bufbase: u64,   /* base address of SDMA completion ring */
    /* User register base for init code, not to be used directly by
     * protocol or applications. Always maps real chip register space.
     * the register addresses are:
     * ur_rcvhdrhead, ur_rcvhdrtail, ur_rcvegrhead, ur_rcvegrtail,
     * ur_rcvtidflow */
    pub user_regbase: u64,
    pub events_bufbase: u64,      /* notification events */
    pub status_bufbase: u64,      /* status page */
    pub rcvhdrtail_base: u64,     /* rcvhdrtail update */
    /* shared memory pages for subctxts if ctxt is shared; these cover
     * all the processes in the group sharing a single context.
     * all have enough space for the num_subcontexts value on this job. */
    pub subctxt_uregbase: u64,
    pub subctxt_rcvegrbuf: u64,
    pub subctxt_rcvhdrbuf: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
