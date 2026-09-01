// SPDX-License-Identifier: GPL-2.0
/*
 * AMD specific. Provide textual annotation for IBS raw sample data.
 */

// Depends on Rust bindings for the perf utility and AMD IBS definitions that
// are included by the original C source.

use core::ffi::{c_char, c_int, c_uint, c_void};

type u32 = u32;
type __u64 = u64;

extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;

    fn pr_debug(fmt: *const c_char, ...) -> c_int;
    fn evlist__event2evsel(evlist: *mut evlist, event: *mut perf_event) -> *mut evsel;
    fn evlist__session(evlist: *mut evlist) -> *mut perf_session;
    fn perf_session__env(session: *mut perf_session) -> *mut perf_env;
    fn perf_env__cpuid(env: *mut perf_env) -> *const c_char;
    fn perf_env__nr_pmu_mappings(env: *mut perf_env) -> c_int;
    fn perf_env__pmu_mappings(env: *mut perf_env) -> *const c_char;
    fn perf_env__find_pmu_cap(
        env: *mut perf_env,
        pmu_name: *const c_char,
        cap: *const c_char,
    ) -> bool;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
}

static mut cpu_family: u32 = 0;
static mut cpu_model: u32 = 0;
static mut ibs_fetch_type: u32 = 0;
static mut ibs_op_type: u32 = 0;
static mut zen4_ibs_extensions: bool = false;
static mut ldlat_cap: bool = false;
static mut dtlb_pgsize_cap: bool = false;
static mut rmtsocket_cap: bool = false;
static mut strmst_cap: bool = false;

/*
 * Status fields of IBS_FETCH_CTL and IBS_FETCH_CTL_EXT are valid only if
 * IBS_FETCH_CTL[PhyAddrValid] is set.
 */
unsafe fn fetch_ctl_depends_on_phy_addr_valid() -> c_int {
    static mut depends: c_int = -1; /* -1: Don't know, 1: Yes, 0: No */

    if depends != -1 {
        return depends;
    }

    depends = 0;
    if cpu_family > 0x1a
        || (cpu_family == 0x1a
            && ((cpu_model >= 0x50 && cpu_model <= 0x5f)
                || (cpu_model >= 0x80 && cpu_model <= 0xaf)
                || (cpu_model >= 0xc0 && cpu_model <= 0xcf)))
    {
        depends = 1;
    }

    depends
}

unsafe fn pr_ibs_fetch_ctl(reg: ibs_fetch_ctl) {
    let ic_miss_strs: [*const c_char; 2] = [b" IcMiss 0\0".as_ptr().cast(), b" IcMiss 1\0".as_ptr().cast()];
    let l1tlb_pgsz_strs: [*const c_char; 4] = [
        b" L1TlbPgSz 4KB\0".as_ptr().cast(),
        b" L1TlbPgSz 2MB\0".as_ptr().cast(),
        b" L1TlbPgSz 1GB\0".as_ptr().cast(),
        b" L1TlbPgSz RESERVED\0".as_ptr().cast(),
    ];
    let l1tlb_pgsz_strs_erratum1347: [*const c_char; 4] = [
        b" L1TlbPgSz 4KB\0".as_ptr().cast(),
        b" L1TlbPgSz 16KB\0".as_ptr().cast(),
        b" L1TlbPgSz 2MB\0".as_ptr().cast(),
        b" L1TlbPgSz 1GB\0".as_ptr().cast(),
    ];
    let mut ic_miss_str: *const c_char = core::ptr::null();
    let mut l1tlb_pgsz_str: *const c_char = core::ptr::null();
    let mut l3_miss_str = [0 as c_char; b" L3MissOnly _ FetchOcMiss _ FetchL3Miss _\0".len()];
    let mut l3_miss_only_str = [0 as c_char; b" L3MissOnly _\0".len()];

    if fetch_ctl_depends_on_phy_addr_valid() != 0 && !reg.phy_addr_valid {
        snprintf(
            l3_miss_only_str.as_mut_ptr(),
            l3_miss_only_str.len(),
            b" L3MissOnly %d\0".as_ptr().cast(),
            reg.l3_miss_only,
        );

        printf(
            b"ibs_fetch_ctl:\t%016llx MaxCnt %7d Cnt %7d En %d Val %d Comp %d PhyAddrValid 0 RandEn %d%s\n\0"
                .as_ptr()
                .cast(),
            reg.val,
            reg.fetch_maxcnt << 4,
            reg.fetch_cnt << 4,
            reg.fetch_en,
            reg.fetch_val,
            reg.fetch_comp,
            reg.rand_en,
            l3_miss_only_str.as_ptr(),
        );
        return;
    }

    if cpu_family == 0x19 && cpu_model < 0x10 {
        /*
         * Erratum #1238 workaround is to ignore MSRC001_1030[IbsIcMiss]
         * Erratum #1347 workaround is to use table provided in erratum
         */
        if reg.phy_addr_valid {
            l1tlb_pgsz_str = l1tlb_pgsz_strs_erratum1347[reg.l1tlb_pgsz as usize];
        }
    } else {
        if reg.phy_addr_valid {
            l1tlb_pgsz_str = l1tlb_pgsz_strs[reg.l1tlb_pgsz as usize];
        }
        ic_miss_str = ic_miss_strs[reg.ic_miss as usize];
    }

    if zen4_ibs_extensions {
        snprintf(
            l3_miss_str.as_mut_ptr(),
            l3_miss_str.len(),
            b" L3MissOnly %d FetchOcMiss %d FetchL3Miss %d\0".as_ptr().cast(),
            reg.l3_miss_only,
            reg.fetch_oc_miss,
            reg.fetch_l3_miss,
        );
    }

    printf(
        b"ibs_fetch_ctl:\t%016llx MaxCnt %7d Cnt %7d Lat %5d En %d Val %d Comp %d%s PhyAddrValid %d%s L1TlbMiss %d L2TlbMiss %d RandEn %d%s%s\n\0"
            .as_ptr()
            .cast(),
        reg.val,
        reg.fetch_maxcnt << 4,
        reg.fetch_cnt << 4,
        reg.fetch_lat,
        reg.fetch_en,
        reg.fetch_val,
        reg.fetch_comp,
        if !ic_miss_str.is_null() { ic_miss_str } else { b"\0".as_ptr().cast() },
        reg.phy_addr_valid,
        if !l1tlb_pgsz_str.is_null() { l1tlb_pgsz_str } else { b"\0".as_ptr().cast() },
        reg.l1tlb_miss,
        reg.l2tlb_miss,
        reg.rand_en,
        if reg.fetch_comp {
            if reg.fetch_l2_miss {
                b" L2Miss 1\0".as_ptr().cast()
            } else {
                b" L2Miss 0\0".as_ptr().cast()
            }
        } else {
            b"\0".as_ptr().cast()
        },
        l3_miss_str.as_ptr(),
    );
}

unsafe fn pr_ic_ibs_extd_ctl(fetch_ctl: ibs_fetch_ctl, reg: ic_ibs_extd_ctl) {
    if fetch_ctl_depends_on_phy_addr_valid() != 0 && !fetch_ctl.phy_addr_valid {
        return;
    }

    printf(
        b"ic_ibs_ext_ctl:\t%016llx IbsItlbRefillLat %3d\n\0".as_ptr().cast(),
        reg.val,
        reg.itlb_refill_lat,
    );
}

unsafe fn pr_ibs_op_ctl(reg: ibs_op_ctl) {
    let mut l3_miss_only = [0 as c_char; b" L3MissOnly _\0".len()];
    let mut ldlat = [0 as c_char; b" LdLatThrsh __ LdLatEn _\0".len()];

    if zen4_ibs_extensions {
        snprintf(
            l3_miss_only.as_mut_ptr(),
            l3_miss_only.len(),
            b" L3MissOnly %d\0".as_ptr().cast(),
            reg.l3_miss_only,
        );
    }

    if ldlat_cap {
        snprintf(
            ldlat.as_mut_ptr(),
            ldlat.len(),
            b" LdLatThrsh %2d LdLatEn %d\0".as_ptr().cast(),
            reg.ldlat_thrsh,
            reg.ldlat_en,
        );
    }

    printf(
        b"ibs_op_ctl:\t%016llx MaxCnt %9d%s En %d Val %d CntCtl %d=%s CurCnt %9d%s\n\0"
            .as_ptr()
            .cast(),
        reg.val,
        ((reg.opmaxcnt_ext << 16) | reg.opmaxcnt) << 4,
        l3_miss_only.as_ptr(),
        reg.op_en,
        reg.op_val,
        reg.cnt_ctl,
        if reg.cnt_ctl { b"uOps\0".as_ptr().cast() } else { b"cycles\0".as_ptr().cast() },
        reg.opcurcnt,
        ldlat.as_ptr(),
    );
}

unsafe fn pr_ibs_op_data(reg: ibs_op_data) {
    printf(
        b"ibs_op_data:\t%016llx CompToRetCtr %5d TagToRetCtr %5d%s%s%s BrnRet %d  RipInvalid %d BrnFuse %d Microcode %d\n\0"
            .as_ptr()
            .cast(),
        reg.val,
        reg.comp_to_ret_ctr,
        reg.tag_to_ret_ctr,
        if reg.op_brn_ret {
            if reg.op_return { b" OpReturn 1\0".as_ptr().cast() } else { b" OpReturn 0\0".as_ptr().cast() }
        } else {
            b"\0".as_ptr().cast()
        },
        if reg.op_brn_ret {
            if reg.op_brn_taken { b" OpBrnTaken 1\0".as_ptr().cast() } else { b" OpBrnTaken 0\0".as_ptr().cast() }
        } else {
            b"\0".as_ptr().cast()
        },
        if reg.op_brn_ret {
            if reg.op_brn_misp { b" OpBrnMisp 1\0".as_ptr().cast() } else { b" OpBrnMisp 0\0".as_ptr().cast() }
        } else {
            b"\0".as_ptr().cast()
        },
        reg.op_brn_ret,
        reg.op_rip_invalid,
        reg.op_brn_fuse,
        reg.op_microcode,
    );
}

unsafe fn pr_ibs_op_data2_extended(reg: ibs_op_data2) {
    static DATA_SRC_STR: [*const c_char; 13] = [
        b"\0".as_ptr().cast(),
        b" DataSrc 1=Local L3 or other L1/L2 in CCX\0".as_ptr().cast(),
        b" DataSrc 2=Another CCX cache in the same NUMA node\0".as_ptr().cast(),
        b" DataSrc 3=DRAM\0".as_ptr().cast(),
        b" DataSrc 4=(reserved)\0".as_ptr().cast(),
        b" DataSrc 5=Another CCX cache in a different NUMA node\0".as_ptr().cast(),
        b" DataSrc 6=Long-latency DIMM\0".as_ptr().cast(),
        b" DataSrc 7=MMIO/Config/PCI/APIC\0".as_ptr().cast(),
        b" DataSrc 8=Extension Memory\0".as_ptr().cast(),
        b" DataSrc 9=(reserved)\0".as_ptr().cast(),
        b" DataSrc 10=(reserved)\0".as_ptr().cast(),
        b" DataSrc 11=(reserved)\0".as_ptr().cast(),
        b" DataSrc 12=Coherent Memory of a different processor type\0".as_ptr().cast(),
        /* 13 to 31 are reserved. Avoid printing them. */
    ];
    let data_src: c_int = ((reg.data_src_hi << 3) | reg.data_src_lo) as c_int;
    let mut rmtsocket = [0 as c_char; b"RmtSocket _ \0".len()];
    let mut strmst = [0 as c_char; b"StrmSt _ \0".len()];

    if rmtsocket_cap {
        snprintf(rmtsocket.as_mut_ptr(), rmtsocket.len(), b"RmtSocket %d \0".as_ptr().cast(), reg.rmt_socket);
    }
    if strmst_cap {
        snprintf(strmst.as_mut_ptr(), strmst.len(), b"StrmSt %d \0".as_ptr().cast(), reg.strm_st);
    }

    printf(
        b"ibs_op_data2:\t%016llx %s%s%sRmtNode %d%s\n\0".as_ptr().cast(),
        reg.val,
        rmtsocket.as_ptr(),
        strmst.as_ptr(),
        if data_src == 1 || data_src == 2 || data_src == 5 {
            if reg.cache_hit_st { b"CacheHitSt 1=O-State \0".as_ptr().cast() } else { b"CacheHitSt 0=M-state \0".as_ptr().cast() }
        } else {
            b"\0".as_ptr().cast()
        },
        reg.rmt_node,
        if (data_src as usize) < DATA_SRC_STR.len() { DATA_SRC_STR[data_src as usize] } else { b"\0".as_ptr().cast() },
    );
}

unsafe fn pr_ibs_op_data2_default(reg: ibs_op_data2) {
    static DATA_SRC_STR: [*const c_char; 8] = [
        b"\0".as_ptr().cast(),
        b" DataSrc 1=(reserved)\0".as_ptr().cast(),
        b" DataSrc 2=Local node cache\0".as_ptr().cast(),
        b" DataSrc 3=DRAM\0".as_ptr().cast(),
        b" DataSrc 4=Remote node cache\0".as_ptr().cast(),
        b" DataSrc 5=(reserved)\0".as_ptr().cast(),
        b" DataSrc 6=(reserved)\0".as_ptr().cast(),
        b" DataSrc 7=Other\0".as_ptr().cast(),
    ];
    let mut rmtsocket = [0 as c_char; b"RmtSocket _ \0".len()];
    let mut strmst = [0 as c_char; b"StrmSt _ \0".len()];

    if rmtsocket_cap {
        snprintf(rmtsocket.as_mut_ptr(), rmtsocket.len(), b"RmtSocket %d \0".as_ptr().cast(), reg.rmt_socket);
    }
    if strmst_cap {
        snprintf(strmst.as_mut_ptr(), strmst.len(), b"StrmSt %d \0".as_ptr().cast(), reg.strm_st);
    }

    printf(
        b"ibs_op_data2:\t%016llx %s%s%sRmtNode %d%s\n\0".as_ptr().cast(),
        reg.val,
        rmtsocket.as_ptr(),
        strmst.as_ptr(),
        if reg.data_src_lo == 2 {
            if reg.cache_hit_st { b"CacheHitSt 1=O-State \0".as_ptr().cast() } else { b"CacheHitSt 0=M-state \0".as_ptr().cast() }
        } else {
            b"\0".as_ptr().cast()
        },
        reg.rmt_node,
        DATA_SRC_STR[reg.data_src_lo as usize],
    );
}

unsafe fn pr_ibs_op_data2(reg: ibs_op_data2) {
    if zen4_ibs_extensions {
        return pr_ibs_op_data2_extended(reg);
    }
    pr_ibs_op_data2_default(reg);
}

unsafe fn pr_ibs_op_data3(reg: ibs_op_data3) {
    static DC_PAGE_SIZES: [*const c_char; 4] = [
        b"  4K\0".as_ptr().cast(),
        b"  2M\0".as_ptr().cast(),
        b"  1G\0".as_ptr().cast(),
        b"  ??\0".as_ptr().cast(),
    ];
    let mut op_dc_miss_open_mem_reqs_str = [0 as c_char; b" OpDcMissOpenMemReqs __\0".len()];
    let mut dc_l1_l2tlb_miss_str = [0 as c_char; b" DcL1TlbMiss _ DcL2TlbMiss _\0".len()];
    let mut dc_l1tlb_hit_str = [0 as c_char; b" DcL1TlbHit2M _ DcL1TlbHit1G _\0".len()];
    let mut op_mem_width_str = [0 as c_char; b" OpMemWidth _____ bytes\0".len()];
    let mut tlb_refill_lat_str = [0 as c_char; b" TlbRefillLat _____\0".len()];
    let mut dc_l2tlb_hit_2m_str = [0 as c_char; b" DcL2TlbHit2M _\0".len()];
    let mut dc_l2tlb_hit_1g_str = [0 as c_char; b" DcL2TlbHit1G _\0".len()];
    let mut dc_page_size_str = [0 as c_char; b" DcPageSize ____\0".len()];
    let mut l2_miss_str = [0 as c_char; b" L2Miss _\0".len()];

    /*
     * Erratum #1293
     * Ignore L2Miss and OpDcMissOpenMemReqs (and opdata2) if DcMissNoMabAlloc or SwPf set
     */
    if !(cpu_family == 0x19 && cpu_model < 0x10 && (reg.dc_miss_no_mab_alloc || reg.sw_pf)) {
        snprintf(l2_miss_str.as_mut_ptr(), l2_miss_str.len(), b" L2Miss %d\0".as_ptr().cast(), reg.l2_miss);
        snprintf(
            op_dc_miss_open_mem_reqs_str.as_mut_ptr(),
            op_dc_miss_open_mem_reqs_str.len(),
            b" OpDcMissOpenMemReqs %2d\0".as_ptr().cast(),
            reg.op_dc_miss_open_mem_reqs,
        );
    }

    if reg.op_mem_width != 0 {
        snprintf(
            op_mem_width_str.as_mut_ptr(),
            op_mem_width_str.len(),
            b" OpMemWidth %2d bytes\0".as_ptr().cast(),
            1 << (reg.op_mem_width - 1),
        );
    }

    if dtlb_pgsize_cap {
        if reg.dc_phy_addr_valid {
            let idx = ((reg.dc_l1tlb_hit_1g << 1) | reg.dc_l1tlb_hit_2m) as usize;

            snprintf(
                dc_l1_l2tlb_miss_str.as_mut_ptr(),
                dc_l1_l2tlb_miss_str.len(),
                b" DcL1TlbMiss %d DcL2TlbMiss %d\0".as_ptr().cast(),
                reg.dc_l1tlb_miss,
                reg.dc_l2tlb_miss,
            );
            snprintf(
                dc_page_size_str.as_mut_ptr(),
                dc_page_size_str.len(),
                b" DcPageSize %4s\0".as_ptr().cast(),
                DC_PAGE_SIZES[idx],
            );
        }
    } else {
        snprintf(
            dc_l1_l2tlb_miss_str.as_mut_ptr(),
            dc_l1_l2tlb_miss_str.len(),
            b" DcL1TlbMiss %d DcL2TlbMiss %d\0".as_ptr().cast(),
            reg.dc_l1tlb_miss,
            reg.dc_l2tlb_miss,
        );
        snprintf(
            dc_l1tlb_hit_str.as_mut_ptr(),
            dc_l1tlb_hit_str.len(),
            b" DcL1TlbHit2M %d DcL1TlbHit1G %d\0".as_ptr().cast(),
            reg.dc_l1tlb_hit_2m,
            reg.dc_l1tlb_hit_1g,
        );
        snprintf(
            dc_l2tlb_hit_2m_str.as_mut_ptr(),
            dc_l2tlb_hit_2m_str.len(),
            b" DcL2TlbHit2M %d\0".as_ptr().cast(),
            reg.dc_l2tlb_hit_2m,
        );
        snprintf(
            dc_l2tlb_hit_1g_str.as_mut_ptr(),
            dc_l2tlb_hit_1g_str.len(),
            b" DcL2TlbHit1G %d\0".as_ptr().cast(),
            reg.dc_l2_tlb_hit_1g,
        );
    }

    /* Use !zen4_ibs_extensions as a proxy for Zen3 and earlier */
    if !zen4_ibs_extensions || reg.dc_phy_addr_valid {
        snprintf(
            tlb_refill_lat_str.as_mut_ptr(),
            tlb_refill_lat_str.len(),
            b" TlbRefillLat %5d\0".as_ptr().cast(),
            reg.tlb_refill_lat,
        );
    }

    printf(
        b"ibs_op_data3:\t%016llx LdOp %d StOp %d%s%s%s DcMiss %d DcMisAcc %d DcWcMemAcc %d DcUcMemAcc %d DcLockedOp %d DcMissNoMabAlloc %d DcLinAddrValid %d DcPhyAddrValid %d%s%s SwPf %d%s%s DcMissLat %5d%s\n\0"
            .as_ptr()
            .cast(),
        reg.val,
        reg.ld_op,
        reg.st_op,
        dc_l1_l2tlb_miss_str.as_ptr(),
        if dtlb_pgsize_cap { dc_page_size_str.as_ptr() } else { dc_l1tlb_hit_str.as_ptr() },
        dc_l2tlb_hit_2m_str.as_ptr(),
        reg.dc_miss,
        reg.dc_mis_acc,
        reg.dc_wc_mem_acc,
        reg.dc_uc_mem_acc,
        reg.dc_locked_op,
        reg.dc_miss_no_mab_alloc,
        reg.dc_lin_addr_valid,
        reg.dc_phy_addr_valid,
        dc_l2tlb_hit_1g_str.as_ptr(),
        l2_miss_str.as_ptr(),
        reg.sw_pf,
        op_mem_width_str.as_ptr(),
        op_dc_miss_open_mem_reqs_str.as_ptr(),
        reg.dc_miss_lat,
        tlb_refill_lat_str.as_ptr(),
    );
}

/*
 * IBS Op/Execution MSRs always saved, in order, are:
 * IBS_OP_CTL, IBS_OP_RIP, IBS_OP_DATA, IBS_OP_DATA2,
 * IBS_OP_DATA3, IBS_DC_LINADDR, IBS_DC_PHYSADDR, BP_IBSTGT_RIP
 */
unsafe fn amd_dump_ibs_op(sample: *mut perf_sample) {
    let data = (*sample).raw_data as *mut perf_ibs_data;
    let op_ctl = (*data).data.as_mut_ptr() as *mut ibs_op_ctl;
    let rip = (op_ctl as *mut __u64).add(1);
    let op_data = rip.add(1) as *mut ibs_op_data;
    let op_data3 = rip.add(3) as *mut ibs_op_data3;

    pr_ibs_op_ctl(*op_ctl);
    if !(*op_data).op_rip_invalid {
        printf(b"IbsOpRip:\t%016llx\n\0".as_ptr().cast(), *rip);
    }
    pr_ibs_op_data(*op_data);
    /*
     * Erratum #1293: ignore op_data2 if DcMissNoMabAlloc or SwPf are set
     */
    if !(cpu_family == 0x19
        && cpu_model < 0x10
        && ((*op_data3).dc_miss_no_mab_alloc || (*op_data3).sw_pf))
    {
        pr_ibs_op_data2(*(rip.add(2) as *mut ibs_op_data2));
    }
    pr_ibs_op_data3(*op_data3);
    if (*op_data3).dc_lin_addr_valid {
        printf(b"IbsDCLinAd:\t%016llx\n\0".as_ptr().cast(), *rip.add(4));
    }

    /* Use !zen4_ibs_extensions as a proxy for Zen3 and earlier */
    if (*op_data3).dc_phy_addr_valid
        && *rip.add(5) != 0
        && (!zen4_ibs_extensions || (*op_data3).dc_lin_addr_valid)
    {
        printf(b"IbsDCPhysAd:\t%016llx\n\0".as_ptr().cast(), *rip.add(5));
    }
    if (*op_data).op_brn_ret && *rip.add(6) != 0 {
        printf(b"IbsBrTarget:\t%016llx\n\0".as_ptr().cast(), *rip.add(6));
    }
}

/*
 * IBS Fetch MSRs always saved, in order, are:
 * IBS_FETCH_CTL, IBS_FETCH_LINADDR, IBS_FETCH_PHYSADDR, IC_IBS_EXTD_CTL
 */
unsafe fn amd_dump_ibs_fetch(sample: *mut perf_sample) {
    let data = (*sample).raw_data as *mut perf_ibs_data;
    let fetch_ctl = (*data).data.as_mut_ptr() as *mut ibs_fetch_ctl;
    let mut addr = (fetch_ctl as *mut __u64).add(1);
    let extd_ctl = (addr as *mut ic_ibs_extd_ctl).add(2);

    pr_ibs_fetch_ctl(*fetch_ctl);
    printf(b"IbsFetchLinAd:\t%016llx\n\0".as_ptr().cast(), *addr);
    addr = addr.add(1);
    if (*fetch_ctl).phy_addr_valid {
        printf(b"IbsFetchPhysAd:\t%016llx\n\0".as_ptr().cast(), *addr);
    }
    pr_ic_ibs_extd_ctl(*fetch_ctl, *extd_ctl);
}

/*
 * Test for enable and valid bits in captured control MSRs.
 */
unsafe fn is_valid_ibs_fetch_sample(sample: *mut perf_sample) -> bool {
    let data = (*sample).raw_data as *mut perf_ibs_data;
    let fetch_ctl = (*data).data.as_mut_ptr() as *mut ibs_fetch_ctl;

    if (*fetch_ctl).fetch_en && (*fetch_ctl).fetch_val {
        return true;
    }

    false
}

unsafe fn is_valid_ibs_op_sample(sample: *mut perf_sample) -> bool {
    let data = (*sample).raw_data as *mut perf_ibs_data;
    let op_ctl = (*data).data.as_mut_ptr() as *mut ibs_op_ctl;

    if (*op_ctl).op_en && (*op_ctl).op_val {
        return true;
    }

    false
}

/* AMD vendor specific raw sample function. Check for PERF_RECORD_SAMPLE events
 * and if the event was triggered by IBS, display its raw data with decoded text.
 * The function is only invoked when the dump flag -D is set.
 */
#[no_mangle]
pub unsafe extern "C" fn evlist__amd_sample_raw(
    evlist: *mut evlist,
    event: *mut perf_event,
    sample: *mut perf_sample,
) {
    let evsel: *mut evsel;

    if (*event).header.type_ != PERF_RECORD_SAMPLE || (*sample).raw_size == 0 {
        return;
    }

    evsel = evlist__event2evsel(evlist, event);
    if evsel.is_null() {
        return;
    }

    if (*evsel).core.attr.type_ == ibs_fetch_type {
        if !is_valid_ibs_fetch_sample(sample) {
            pr_debug(b"Invalid raw IBS Fetch MSR data encountered\n\0".as_ptr().cast());
            return;
        }
        amd_dump_ibs_fetch(sample);
    } else if (*evsel).core.attr.type_ == ibs_op_type {
        if !is_valid_ibs_op_sample(sample) {
            pr_debug(b"Invalid raw IBS Op MSR data encountered\n\0".as_ptr().cast());
            return;
        }
        amd_dump_ibs_op(sample);
    }
}

unsafe fn parse_cpuid(env: *mut perf_env) {
    let cpuid: *const c_char;
    let ret: c_int;

    cpuid = perf_env__cpuid(env);
    /*
     * cpuid = "AuthenticAMD,family,model,stepping"
     */
    ret = sscanf(
        cpuid,
        b"%*[^,],%u,%u\0".as_ptr().cast(),
        &mut cpu_family as *mut u32 as *mut c_uint,
        &mut cpu_model as *mut u32 as *mut c_uint,
    );
    if ret != 2 {
        pr_debug(b"problem parsing cpuid\n\0".as_ptr().cast());
    }
}

/*
 * Find and assign the type number used for ibs_op or ibs_fetch samples.
 * Device names can be large - we are only interested in the first 9 characters,
 * to match "ibs_fetch".
 */
#[no_mangle]
pub unsafe extern "C" fn evlist__has_amd_ibs(evlist: *mut evlist) -> bool {
    let env = perf_session__env(evlist__session(evlist));
    let mut ret: c_int;
    let mut nr_pmu_mappings = perf_env__nr_pmu_mappings(env);
    let mut pmu_mapping = perf_env__pmu_mappings(env);
    let mut name = [0 as c_char; b"ibs_fetch\0".len()];
    let mut type_: u32 = 0;

    while nr_pmu_mappings != 0 {
        nr_pmu_mappings -= 1;
        ret = sscanf(
            pmu_mapping,
            b"%u:%9s\0".as_ptr().cast(),
            &mut type_ as *mut u32 as *mut c_uint,
            name.as_mut_ptr(),
        );
        if ret == 2 {
            if strstarts(name.as_ptr(), b"ibs_op\0".as_ptr().cast()) {
                ibs_op_type = type_;
            } else if strstarts(name.as_ptr(), b"ibs_fetch\0".as_ptr().cast()) {
                ibs_fetch_type = type_;
            }
        }
        pmu_mapping = pmu_mapping.add(strlen(pmu_mapping) + 1 /* '\0' */);
    }

    if perf_env__find_pmu_cap(env, b"ibs_op\0".as_ptr().cast(), b"zen4_ibs_extensions\0".as_ptr().cast()) {
        zen4_ibs_extensions = true;
    }

    if perf_env__find_pmu_cap(env, b"ibs_op\0".as_ptr().cast(), b"ldlat\0".as_ptr().cast()) {
        ldlat_cap = true;
    }

    if perf_env__find_pmu_cap(env, b"ibs_op\0".as_ptr().cast(), b"dtlb_pgsize\0".as_ptr().cast()) {
        dtlb_pgsize_cap = true;
    }

    if perf_env__find_pmu_cap(env, b"ibs_op\0".as_ptr().cast(), b"rmtsocket\0".as_ptr().cast()) {
        rmtsocket_cap = true;
    }

    if perf_env__find_pmu_cap(env, b"ibs_op\0".as_ptr().cast(), b"strmst\0".as_ptr().cast()) {
        strmst_cap = true;
    }

    if ibs_fetch_type != 0 || ibs_op_type != 0 {
        if cpu_family == 0 {
            parse_cpuid(env);
        }
        return true;
    }

    false
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
