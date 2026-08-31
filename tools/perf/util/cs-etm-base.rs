// SPDX-License-Identifier: GPL-2.0
/*
 * File for any parts of the Coresight decoding that don't require
 * OpenCSD.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

type u64 = u64;

#[repr(C)]
pub struct perf_event_header {
    pub size: u16,
}

#[repr(C)]
pub struct perf_record_auxtrace_info {
    pub header: perf_event_header,
    pub priv_: [u64; 0],
}

#[repr(C)]
pub union perf_event {
    pub auxtrace_info: core::mem::ManuallyDrop<perf_record_auxtrace_info>,
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stdout: *mut c_void;
    static __perf_cs_etmv3_magic: u64;
    static __perf_cs_etmv4_magic: u64;
    static __perf_cs_ete_magic: u64;
    static dump_trace: bool;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn pr_err(format: *const c_char, ...) -> c_int;
    fn cs_etm__process_auxtrace_info_full(
        event: *mut perf_event,
        session: *mut perf_session,
    ) -> c_int;
}

const EINVAL: c_int = 22;

/* Constants supplied by cs-etm.h in the original C translation unit. */
unsafe extern "C" {
    static CS_HEADER_VERSION: c_int;
    static CS_PMU_TYPE_CPUS: c_int;
    static CS_ETM_SNAPSHOT: c_int;
    static CS_HEADER_VERSION_MAX: c_int;
    static CS_HEADER_CURRENT_VERSION: c_uint;
    static INFO_HEADER_SIZE: usize;
    static CS_ETM_HEADER_SIZE: usize;

    static CS_ETM_MAGIC: c_int;
    static CS_ETM_CPU: c_int;
    static CS_ETM_NR_TRC_PARAMS: c_int;
    static CS_ETM_ETMCR: c_int;
    static CS_ETM_ETMTRACEIDR: c_int;
    static CS_ETM_ETMCCER: c_int;
    static CS_ETM_ETMIDR: c_int;
    static CS_ETM_NR_TRC_PARAMS_V0: c_int;
    static CS_ETM_COMMON_BLK_MAX_V1: c_int;
    static CS_ETM_PRIV_MAX: c_int;

    static CS_ETMV4_TRCCONFIGR: c_int;
    static CS_ETMV4_TRCTRACEIDR: c_int;
    static CS_ETMV4_TRCIDR0: c_int;
    static CS_ETMV4_TRCIDR1: c_int;
    static CS_ETMV4_TRCIDR2: c_int;
    static CS_ETMV4_TRCIDR8: c_int;
    static CS_ETMV4_TRCAUTHSTATUS: c_int;
    static CS_ETMV4_TS_SOURCE: c_int;
    static CS_ETMV4_NR_TRC_PARAMS_V0: c_int;
    static CS_ETMV4_PRIV_MAX: c_int;

    static CS_ETE_TRCCONFIGR: c_int;
    static CS_ETE_TRCTRACEIDR: c_int;
    static CS_ETE_TRCIDR0: c_int;
    static CS_ETE_TRCIDR1: c_int;
    static CS_ETE_TRCIDR2: c_int;
    static CS_ETE_TRCIDR8: c_int;
    static CS_ETE_TRCAUTHSTATUS: c_int;
    static CS_ETE_TRCDEVARCH: c_int;
    static CS_ETE_TS_SOURCE: c_int;
    static CS_ETE_PRIV_MAX: c_int;
}

unsafe fn cs_etm_global_header_fmt(idx: c_int) -> *const c_char {
    if idx == CS_HEADER_VERSION {
        b"\tHeader version\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_PMU_TYPE_CPUS {
        b"\tPMU type/num cpus\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETM_SNAPSHOT {
        b"\tSnapshot\t\t       %llx\n\0".as_ptr() as *const c_char
    } else {
        core::ptr::null()
    }
}

unsafe fn cs_etm_priv_fmt(idx: c_int) -> *const c_char {
    if idx == CS_ETM_MAGIC {
        b"\tMagic number\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETM_CPU {
        b"\tCPU\t\t\t       %lld\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETM_NR_TRC_PARAMS {
        b"\tNR_TRC_PARAMS\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETM_ETMCR {
        b"\tETMCR\t\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETM_ETMTRACEIDR {
        b"\tETMTRACEIDR\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETM_ETMCCER {
        b"\tETMCCER\t\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETM_ETMIDR {
        b"\tETMIDR\t\t\t       %llx\n\0".as_ptr() as *const c_char
    } else {
        core::ptr::null()
    }
}

unsafe fn cs_etmv4_priv_fmt(idx: c_int) -> *const c_char {
    if idx == CS_ETM_MAGIC {
        b"\tMagic number\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETM_CPU {
        b"\tCPU\t\t\t       %lld\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETM_NR_TRC_PARAMS {
        b"\tNR_TRC_PARAMS\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETMV4_TRCCONFIGR {
        b"\tTRCCONFIGR\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETMV4_TRCTRACEIDR {
        b"\tTRCTRACEIDR\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETMV4_TRCIDR0 {
        b"\tTRCIDR0\t\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETMV4_TRCIDR1 {
        b"\tTRCIDR1\t\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETMV4_TRCIDR2 {
        b"\tTRCIDR2\t\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETMV4_TRCIDR8 {
        b"\tTRCIDR8\t\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETMV4_TRCAUTHSTATUS {
        b"\tTRCAUTHSTATUS\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETMV4_TS_SOURCE {
        b"\tTS_SOURCE\t\t       %lld\n\0".as_ptr() as *const c_char
    } else {
        core::ptr::null()
    }
}

unsafe fn cs_ete_priv_fmt(idx: c_int) -> *const c_char {
    if idx == CS_ETM_MAGIC {
        b"\tMagic number\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETM_CPU {
        b"\tCPU\t\t\t       %lld\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETM_NR_TRC_PARAMS {
        b"\tNR_TRC_PARAMS\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETE_TRCCONFIGR {
        b"\tTRCCONFIGR\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETE_TRCTRACEIDR {
        b"\tTRCTRACEIDR\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETE_TRCIDR0 {
        b"\tTRCIDR0\t\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETE_TRCIDR1 {
        b"\tTRCIDR1\t\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETE_TRCIDR2 {
        b"\tTRCIDR2\t\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETE_TRCIDR8 {
        b"\tTRCIDR8\t\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETE_TRCAUTHSTATUS {
        b"\tTRCAUTHSTATUS\t\t       %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETE_TRCDEVARCH {
        b"\tTRCDEVARCH                     %llx\n\0".as_ptr() as *const c_char
    } else if idx == CS_ETE_TS_SOURCE {
        b"\tTS_SOURCE                      %lld\n\0".as_ptr() as *const c_char
    } else {
        core::ptr::null()
    }
}

static PARAM_UNK_FMT: &[u8] = b"\tUnknown parameter [%d]\t       %lx\n\0";
static MAGIC_UNK_FMT: &[u8] = b"\tMagic number Unknown\t       %lx\n\0";

unsafe fn cs_etm__print_cpu_metadata_v0(val: *mut u64, offset: *mut c_int) -> c_int {
    let mut i: c_int = *offset;
    let mut j: c_int;
    let mut nr_params: c_int = 0;
    let fmt_offset: c_int;
    let magic: u64;

    /* check magic value */
    magic = *val.offset((i + CS_ETM_MAGIC) as isize);
    if magic != __perf_cs_etmv3_magic && magic != __perf_cs_etmv4_magic {
        /* failure - note bad magic value */
        fprintf(stdout, MAGIC_UNK_FMT.as_ptr() as *const c_char, magic);
        return -EINVAL;
    }

    /* print common header block */
    fprintf(stdout, cs_etm_priv_fmt(CS_ETM_MAGIC), *val.offset(i as isize));
    i += 1;
    fprintf(stdout, cs_etm_priv_fmt(CS_ETM_CPU), *val.offset(i as isize));
    i += 1;

    if magic == __perf_cs_etmv3_magic {
        nr_params = CS_ETM_NR_TRC_PARAMS_V0;
        fmt_offset = CS_ETM_ETMCR;
        /* after common block, offset format index past NR_PARAMS */
        j = fmt_offset;
        while j < nr_params + fmt_offset {
            fprintf(stdout, cs_etm_priv_fmt(j), *val.offset(i as isize));
            j += 1;
            i += 1;
        }
    } else if magic == __perf_cs_etmv4_magic {
        nr_params = CS_ETMV4_NR_TRC_PARAMS_V0;
        fmt_offset = CS_ETMV4_TRCCONFIGR;
        /* after common block, offset format index past NR_PARAMS */
        j = fmt_offset;
        while j < nr_params + fmt_offset {
            fprintf(stdout, cs_etmv4_priv_fmt(j), *val.offset(i as isize));
            j += 1;
            i += 1;
        }
    }
    *offset = i;
    0
}

unsafe fn cs_etm__print_cpu_metadata_v1(val: *mut u64, offset: *mut c_int) -> c_int {
    let mut i: c_int = *offset;
    let mut j: c_int;
    let total_params: c_int;
    let magic: u64;

    magic = *val.offset((i + CS_ETM_MAGIC) as isize);
    /* total params to print is NR_PARAMS + common block size for v1 */
    total_params =
        *val.offset((i + CS_ETM_NR_TRC_PARAMS) as isize) as c_int + CS_ETM_COMMON_BLK_MAX_V1;

    if magic == __perf_cs_etmv3_magic {
        j = 0;
        while j < total_params {
            /* if newer record - could be excess params */
            if j >= CS_ETM_PRIV_MAX {
                fprintf(
                    stdout,
                    PARAM_UNK_FMT.as_ptr() as *const c_char,
                    j,
                    *val.offset(i as isize),
                );
            } else {
                fprintf(stdout, cs_etm_priv_fmt(j), *val.offset(i as isize));
            }
            j += 1;
            i += 1;
        }
    } else if magic == __perf_cs_etmv4_magic {
        j = 0;
        while j < total_params {
            /* if newer record - could be excess params */
            if j >= CS_ETMV4_PRIV_MAX {
                fprintf(
                    stdout,
                    PARAM_UNK_FMT.as_ptr() as *const c_char,
                    j,
                    *val.offset(i as isize),
                );
            } else {
                fprintf(stdout, cs_etmv4_priv_fmt(j), *val.offset(i as isize));
            }
            j += 1;
            i += 1;
        }
    } else if magic == __perf_cs_ete_magic {
        j = 0;
        while j < total_params {
            /* if newer record - could be excess params */
            if j >= CS_ETE_PRIV_MAX {
                fprintf(
                    stdout,
                    PARAM_UNK_FMT.as_ptr() as *const c_char,
                    j,
                    *val.offset(i as isize),
                );
            } else {
                fprintf(stdout, cs_ete_priv_fmt(j), *val.offset(i as isize));
            }
            j += 1;
            i += 1;
        }
    } else {
        /* failure - note bad magic value and error out */
        fprintf(stdout, MAGIC_UNK_FMT.as_ptr() as *const c_char, magic);
        return -EINVAL;
    }
    *offset = i;
    0
}

unsafe fn cs_etm__print_auxtrace_info(val: *mut u64, num: c_int) {
    let mut i: c_int;
    let mut cpu: c_int = 0;
    let version: u64;
    let mut err: c_int = 0;

    version = *val.offset(0);

    i = 0;
    while i < CS_HEADER_VERSION_MAX {
        fprintf(
            stdout,
            cs_etm_global_header_fmt(i),
            *val.offset(i as isize),
        );
        i += 1;
    }

    i = CS_HEADER_VERSION_MAX;
    while cpu < num {
        if version == 0 {
            err = cs_etm__print_cpu_metadata_v0(val, &mut i);
        } else if version == 1 || version == 2 {
            /* printing same for both, but value bit flags added on v2 */
            err = cs_etm__print_cpu_metadata_v1(val, &mut i);
        }
        if err != 0 {
            return;
        }
        cpu += 1;
    }
}

/*
 * Do some basic checks and print the auxtrace info header before calling
 * into cs_etm__process_auxtrace_info_full() which requires OpenCSD to be
 * linked in. This allows some basic debugging if OpenCSD is missing.
 */
#[no_mangle]
pub unsafe extern "C" fn cs_etm__process_auxtrace_info(
    event: *mut perf_event,
    session: *mut perf_session,
) -> c_int {
    let auxtrace_info: *mut perf_record_auxtrace_info = &mut (*event).auxtrace_info;
    let event_header_size: c_int = core::mem::size_of::<perf_event_header>() as c_int;
    let num_cpu: c_int;
    let ptr: *mut u64;
    let hdr_version: u64;

    /* Ensure priv[] is large enough for the global header entries */
    if (*auxtrace_info).header.size as usize
        < event_header_size as usize + INFO_HEADER_SIZE + CS_ETM_HEADER_SIZE
    {
        return -EINVAL;
    }

    /* First the global part */
    ptr = (*auxtrace_info).priv_.as_mut_ptr();

    /* Look for version of the header */
    hdr_version = *ptr.offset(0);
    if hdr_version > CS_HEADER_CURRENT_VERSION as u64 {
        pr_err(
            b"\nCS ETM Trace: Unknown Header Version = %#lx\0".as_ptr() as *const c_char,
            hdr_version,
        );
        pr_err(
            b", version supported <= %x\n\0".as_ptr() as *const c_char,
            CS_HEADER_CURRENT_VERSION,
        );
        return -EINVAL;
    }

    if dump_trace {
        num_cpu = (*ptr.offset(CS_PMU_TYPE_CPUS as isize) & 0xffffffff) as c_int;
        cs_etm__print_auxtrace_info(ptr, num_cpu);
    }

    cs_etm__process_auxtrace_info_full(event, session)
}
