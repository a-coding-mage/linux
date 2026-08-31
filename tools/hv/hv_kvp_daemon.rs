/*
 * An implementation of key value pair (KVP) functionality for Linux.
 *
 *
 * Copyright (C) 2010, Novell, Inc.
 * Author : K. Y. Srinivasan <ksrinivasan@novell.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 as published
 * by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or
 * NON INFRINGEMENT.  See the GNU General Public License for more
 * details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 */

use libc::*;

/*
 * C dependencies intentionally not implemented here:
 * sys/poll.h, sys/utsname.h, arpa/inet.h, linux/hyperv.h, ifaddrs.h,
 * netdb.h, syslog.h, dirent.h, net/if.h, getopt.h.
 */

/*
 * KVP protocol: The user mode component first registers with the
 * kernel component. Subsequently, the kernel component requests, data
 * for the specified keys. In response to this message the user mode component
 * fills in the value corresponding to the specified key. We overload the
 * sequence field in the cn_msg header to define our KVP message types.
 *
 * We use this infrastructure for also supporting queries from user mode
 * application for state that may be maintained in the KVP kernel component.
 *
 */

const FullyQualifiedDomainName: c_int = 0;
const IntegrationServicesVersion: c_int = 1; /*This key is serviced in the kernel*/
const NetworkAddressIPv4: c_int = 2;
const NetworkAddressIPv6: c_int = 3;
const OSBuildNumber: c_int = 4;
const OSName: c_int = 5;
const OSMajorVersion: c_int = 6;
const OSMinorVersion: c_int = 7;
const OSVersion: c_int = 8;
const ProcessorArchitecture: c_int = 9;

const IPADDR: c_int = 0;
const NETMASK: c_int = 1;
const GATEWAY: c_int = 2;
const DNS: c_int = 3;

const IPV4: c_int = 1;
const IPV6: c_int = 2;
const IP_TYPE_MAX: c_int = 3;

const KVP_CONFIG_LOC: *const c_char = b"/var/lib/hyperv\0".as_ptr() as *const c_char;

/* KVP_SCRIPTS_PATH may be overridden at build time in C. */
const KVP_SCRIPTS_PATH: &str = "/usr/libexec/hypervkvpd/";

const KVP_NET_DIR: *const c_char = b"/sys/class/net/\0".as_ptr() as *const c_char;

const MAX_FILE_NAME: usize = 100;
const ENTRIES_PER_BLOCK: usize = 50;
/*
 * Change this entry if the number of addresses increases in future
 */
const MAX_IP_ENTRIES: usize = 64;
const OUTSTR_BUF_SIZE: usize = (INET6_ADDRSTRLEN as usize + 1) * MAX_IP_ENTRIES;

type __u8 = u8;

extern "C" {
    static mut errno: c_int;

    static mut optarg: *mut c_char;
    static mut optind: c_int;

    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;

    fn daemon(nochdir: c_int, noclose: c_int) -> c_int;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
}

/* linux/hyperv.h items supplied externally in the final repository. */
extern "C" {
    static HV_KVP_EXCHANGE_MAX_KEY_SIZE: usize;
    static HV_KVP_EXCHANGE_MAX_VALUE_SIZE: usize;
    static KVP_POOL_COUNT: usize;
    static KVP_POOL_AUTO: c_int;
    static KVP_OP_REGISTER1: c_int;
    static KVP_OP_GET_IP_INFO: c_int;
    static KVP_OP_SET_IP_INFO: c_int;
    static KVP_OP_SET: c_int;
    static KVP_OP_GET: c_int;
    static KVP_OP_DELETE: c_int;
    static KVP_OP_ENUMERATE: c_int;
    static HV_S_OK: c_int;
    static HV_S_CONT: c_int;
    static HV_E_FAIL: c_int;
    static HV_INVALIDARG: c_int;
    static HV_GUID_NOTFOUND: c_int;
    static ADDR_FAMILY_IPV4: c_int;
    static ADDR_FAMILY_IPV6: c_int;
    static MAX_IP_ADDR_SIZE: usize;
}

#[repr(C)]
struct hv_kvp_ipaddr_value {
    adapter_id: [__u8; 128],
    addr_family: __u8,
    dhcp_enabled: __u8,
    ip_addr: [__u8; 1024],
    sub_net: [__u8; 1024],
    gate_way: [__u8; 512],
    dns_addr: [__u8; 1024],
}

#[repr(C)]
struct hv_kvp_hdr {
    operation: __u8,
    pool: __u8,
    pad: [__u8; 2],
}

#[repr(C)]
struct hv_kvp_exchg_msg_value {
    value_type: u32,
    key_size: u32,
    value_size: u32,
    key: [__u8; 512],
    value: [__u8; 2048],
}

#[repr(C)]
struct hv_kvp_msg_set {
    data: hv_kvp_exchg_msg_value,
}

#[repr(C)]
struct hv_kvp_msg_delete {
    key_size: u32,
    key: [__u8; 512],
}

#[repr(C)]
struct hv_kvp_msg_enumerate {
    index: u32,
    data: hv_kvp_exchg_msg_value,
}

#[repr(C)]
struct hv_kvp_register {
    version: [__u8; 64],
}

#[repr(C)]
union hv_kvp_msg_body {
    kvp_ip_val: std::mem::ManuallyDrop<hv_kvp_ipaddr_value>,
    kvp_set: std::mem::ManuallyDrop<hv_kvp_msg_set>,
    kvp_delete: std::mem::ManuallyDrop<hv_kvp_msg_delete>,
    kvp_enum_data: std::mem::ManuallyDrop<hv_kvp_msg_enumerate>,
    kvp_register: std::mem::ManuallyDrop<hv_kvp_register>,
}

#[repr(C)]
struct hv_kvp_msg {
    kvp_hdr: hv_kvp_hdr,
    error: c_int,
    body: hv_kvp_msg_body,
}

#[repr(C)]
struct kvp_record {
    key: [c_char; 512],
    value: [c_char; 2048],
}

#[repr(C)]
struct kvp_file_state {
    fd: c_int,
    num_blocks: c_int,
    records: *mut kvp_record,
    num_records: c_int,
    fname: [c_char; MAX_FILE_NAME],
}

static mut in_hand_shake: c_int = 0;
static mut debug: c_int = 0;

static mut os_name: *mut c_char = b"\0".as_ptr() as *mut c_char;
static mut os_major: *mut c_char = b"\0".as_ptr() as *mut c_char;
static mut os_minor: *mut c_char = b"\0".as_ptr() as *mut c_char;
static mut processor_arch: *mut c_char = std::ptr::null_mut();
static mut os_build: *mut c_char = std::ptr::null_mut();
static mut os_version: *mut c_char = std::ptr::null_mut();
static mut lic_version: *mut c_char = b"Unknown version\0".as_ptr() as *mut c_char;
static mut full_domain_name: [c_char; 2048] = [0; 2048];
static mut uts_buf: utsname = unsafe { std::mem::zeroed() };

static mut kvp_file_info: [kvp_file_state; 5] = [
    kvp_file_state { fd: 0, num_blocks: 0, records: std::ptr::null_mut(), num_records: 0, fname: [0; MAX_FILE_NAME] },
    kvp_file_state { fd: 0, num_blocks: 0, records: std::ptr::null_mut(), num_records: 0, fname: [0; MAX_FILE_NAME] },
    kvp_file_state { fd: 0, num_blocks: 0, records: std::ptr::null_mut(), num_records: 0, fname: [0; MAX_FILE_NAME] },
    kvp_file_state { fd: 0, num_blocks: 0, records: std::ptr::null_mut(), num_records: 0, fname: [0; MAX_FILE_NAME] },
    kvp_file_state { fd: 0, num_blocks: 0, records: std::ptr::null_mut(), num_records: 0, fname: [0; MAX_FILE_NAME] },
];

unsafe fn kvp_acquire_lock(pool: c_int) {
    let mut fl: flock = std::mem::zeroed();
    fl.l_type = F_WRLCK as c_short;
    fl.l_whence = SEEK_SET as c_short;
    fl.l_start = 0;
    fl.l_len = 0;
    fl.l_pid = getpid();

    if fcntl(kvp_file_info[pool as usize].fd, F_SETLKW, &mut fl) == -1 {
        syslog(LOG_ERR, b"Failed to acquire the lock pool: %d; error: %d %s\0".as_ptr() as *const c_char, pool, errno, strerror(errno));
        exit(EXIT_FAILURE);
    }
}

unsafe fn kvp_release_lock(pool: c_int) {
    let mut fl: flock = std::mem::zeroed();
    fl.l_type = F_UNLCK as c_short;
    fl.l_whence = SEEK_SET as c_short;
    fl.l_start = 0;
    fl.l_len = 0;
    fl.l_pid = getpid();

    if fcntl(kvp_file_info[pool as usize].fd, F_SETLK, &mut fl) == -1 {
        syslog(LOG_ERR, b"Failed to release the lock pool: %d; error: %d %s\0".as_ptr() as *const c_char, pool, errno, strerror(errno));
        exit(EXIT_FAILURE);
    }
}

unsafe fn kvp_update_file(pool: c_int) {
    let filep: *mut FILE;

    /*
     * We are going to write our in-memory registry out to
     * disk; acquire the lock first.
     */
    kvp_acquire_lock(pool);

    filep = fopen(kvp_file_info[pool as usize].fname.as_mut_ptr(), b"we\0".as_ptr() as *const c_char);
    if filep.is_null() {
        syslog(LOG_ERR, b"Failed to open file, pool: %d; error: %d %s\0".as_ptr() as *const c_char, pool, errno, strerror(errno));
        kvp_release_lock(pool);
        exit(EXIT_FAILURE);
    }

    fwrite(kvp_file_info[pool as usize].records as *const c_void, std::mem::size_of::<kvp_record>(), kvp_file_info[pool as usize].num_records as usize, filep);

    if ferror(filep) != 0 || fclose(filep) != 0 {
        kvp_release_lock(pool);
        syslog(LOG_ERR, b"Failed to write file, pool: %d\0".as_ptr() as *const c_char, pool);
        exit(EXIT_FAILURE);
    }

    kvp_release_lock(pool);
}

unsafe fn kvp_dump_initial_pools(pool: c_int) {
    let mut i: c_int;

    syslog(LOG_DEBUG, b"===Start dumping the contents of pool %d ===\n\0".as_ptr() as *const c_char, pool);

    i = 0;
    while i < kvp_file_info[pool as usize].num_records {
        syslog(LOG_DEBUG, b"pool: %d, %d/%d key=%s val=%s\n\0".as_ptr() as *const c_char, pool, i + 1, kvp_file_info[pool as usize].num_records, (*kvp_file_info[pool as usize].records.add(i as usize)).key.as_ptr(), (*kvp_file_info[pool as usize].records.add(i as usize)).value.as_ptr());
        i += 1;
    }
}

unsafe fn kvp_update_mem_state(pool: c_int) {
    let filep: *mut FILE;
    let mut records_read: size_t = 0;
    let mut record: *mut kvp_record = kvp_file_info[pool as usize].records;
    let mut readp: *mut kvp_record;
    let mut num_blocks: c_int = kvp_file_info[pool as usize].num_blocks;
    let alloc_unit: c_int = (std::mem::size_of::<kvp_record>() * ENTRIES_PER_BLOCK) as c_int;

    kvp_acquire_lock(pool);

    filep = fopen(kvp_file_info[pool as usize].fname.as_mut_ptr(), b"re\0".as_ptr() as *const c_char);
    if filep.is_null() {
        syslog(LOG_ERR, b"Failed to open file, pool: %d; error: %d %s\0".as_ptr() as *const c_char, pool, errno, strerror(errno));
        kvp_release_lock(pool);
        exit(EXIT_FAILURE);
    }
    loop {
        readp = record.add(records_read as usize);
        records_read += fread(readp as *mut c_void, std::mem::size_of::<kvp_record>(), ENTRIES_PER_BLOCK * num_blocks as usize - records_read, filep);

        if ferror(filep) != 0 {
            syslog(LOG_ERR, b"Failed to read file, pool: %d; error: %d %s\0".as_ptr() as *const c_char, pool, errno, strerror(errno));
            kvp_release_lock(pool);
            exit(EXIT_FAILURE);
        }

        if feof(filep) == 0 {
            /*
             * We have more data to read.
             */
            num_blocks += 1;
            record = realloc(record as *mut c_void, (alloc_unit * num_blocks) as usize) as *mut kvp_record;

            if record.is_null() {
                syslog(LOG_ERR, b"malloc failed\0".as_ptr() as *const c_char);
                kvp_release_lock(pool);
                exit(EXIT_FAILURE);
            }
            continue;
        }
        break;
    }

    kvp_file_info[pool as usize].num_blocks = num_blocks;
    kvp_file_info[pool as usize].records = record;
    kvp_file_info[pool as usize].num_records = records_read as c_int;

    fclose(filep);
    kvp_release_lock(pool);
}

unsafe fn kvp_file_init() -> c_int {
    let mut fd: c_int;
    let mut fname: *mut c_char;
    let mut i: c_int;
    let alloc_unit: c_int = (std::mem::size_of::<kvp_record>() * ENTRIES_PER_BLOCK) as c_int;

    if access(KVP_CONFIG_LOC, F_OK) != 0 {
        if mkdir(KVP_CONFIG_LOC, 0o755) != 0 {
            syslog(LOG_ERR, b"Failed to create '%s'; error: %d %s\0".as_ptr() as *const c_char, KVP_CONFIG_LOC, errno, strerror(errno));
            exit(EXIT_FAILURE);
        }
    }

    i = 0;
    while i < KVP_POOL_COUNT as c_int {
        fname = kvp_file_info[i as usize].fname.as_mut_ptr();
        sprintf(fname, b"%s/.kvp_pool_%d\0".as_ptr() as *const c_char, KVP_CONFIG_LOC, i);
        fd = open(fname, O_RDWR | O_CREAT | O_CLOEXEC, 0o644);

        if fd == -1 {
            return 1;
        }

        kvp_file_info[i as usize].fd = fd;
        kvp_file_info[i as usize].num_blocks = 1;
        kvp_file_info[i as usize].records = malloc(alloc_unit as usize) as *mut kvp_record;
        if kvp_file_info[i as usize].records.is_null() {
            return 1;
        }
        kvp_file_info[i as usize].num_records = 0;
        kvp_update_mem_state(i);
        if debug != 0 {
            kvp_dump_initial_pools(i);
        }
        i += 1;
    }

    0
}

unsafe fn kvp_key_delete(pool: c_int, key: *const __u8, key_size: c_int) -> c_int {
    let mut i: c_int;
    let mut j: c_int;
    let mut k: c_int;
    let num_records: c_int;
    let record: *mut kvp_record;

    /*
     * First update the in-memory state.
     */
    kvp_update_mem_state(pool);

    num_records = kvp_file_info[pool as usize].num_records;
    record = kvp_file_info[pool as usize].records;

    i = 0;
    while i < num_records {
        if memcmp(key as *const c_void, (*record.add(i as usize)).key.as_ptr() as *const c_void, key_size as usize) != 0 {
            i += 1;
            continue;
        }
        /*
         * Found a match; just move the remaining
         * entries up.
         */
        if debug != 0 {
            syslog(LOG_DEBUG, b"%s: deleting the KVP: pool=%d key=%s val=%s\0".as_ptr() as *const c_char, b"kvp_key_delete\0".as_ptr(), pool, (*record.add(i as usize)).key.as_ptr(), (*record.add(i as usize)).value.as_ptr());
        }
        if i == num_records - 1 {
            kvp_file_info[pool as usize].num_records -= 1;
            kvp_update_file(pool);
            return 0;
        }

        j = i;
        k = j + 1;
        while k < num_records {
            strcpy((*record.add(j as usize)).key.as_mut_ptr(), (*record.add(k as usize)).key.as_ptr());
            strcpy((*record.add(j as usize)).value.as_mut_ptr(), (*record.add(k as usize)).value.as_ptr());
            j += 1;
            k += 1;
        }

        kvp_file_info[pool as usize].num_records -= 1;
        kvp_update_file(pool);
        return 0;
    }

    if debug != 0 {
        syslog(LOG_DEBUG, b"%s: could not delete KVP: pool=%d key=%s. Record not found\0".as_ptr() as *const c_char, b"kvp_key_delete\0".as_ptr(), pool, key);
    }

    1
}

unsafe fn kvp_key_add_or_modify(pool: c_int, key: *const __u8, key_size: c_int, value: *const __u8, value_size: c_int) -> c_int {
    let mut record: *mut kvp_record;
    let num_records: c_int;
    let num_blocks: c_int;
    let mut i: c_int;

    if debug != 0 {
        syslog(LOG_DEBUG, b"%s: got a KVP: pool=%d key=%s val=%s\0".as_ptr() as *const c_char, b"kvp_key_add_or_modify\0".as_ptr(), pool, key, value);
    }

    if key_size as usize > HV_KVP_EXCHANGE_MAX_KEY_SIZE || value_size as usize > HV_KVP_EXCHANGE_MAX_VALUE_SIZE {
        syslog(LOG_ERR, b"%s: Too long key or value: key=%s, val=%s\0".as_ptr() as *const c_char, b"kvp_key_add_or_modify\0".as_ptr(), key, value);

        if debug != 0 {
            syslog(LOG_DEBUG, b"%s: Too long key or value: pool=%d, key=%s, val=%s\0".as_ptr() as *const c_char, b"kvp_key_add_or_modify\0".as_ptr(), pool, key, value);
        }
        return 1;
    }

    /*
     * First update the in-memory state.
     */
    kvp_update_mem_state(pool);

    num_records = kvp_file_info[pool as usize].num_records;
    record = kvp_file_info[pool as usize].records;
    num_blocks = kvp_file_info[pool as usize].num_blocks;

    i = 0;
    while i < num_records {
        if memcmp(key as *const c_void, (*record.add(i as usize)).key.as_ptr() as *const c_void, key_size as usize) != 0 {
            i += 1;
            continue;
        }
        /*
         * Found a match; just update the value -
         * this is the modify case.
         */
        memcpy((*record.add(i as usize)).value.as_mut_ptr() as *mut c_void, value as *const c_void, value_size as usize);
        kvp_update_file(pool);
        if debug != 0 {
            syslog(LOG_DEBUG, b"%s: updated: pool=%d key=%s val=%s\0".as_ptr() as *const c_char, b"kvp_key_add_or_modify\0".as_ptr(), pool, key, value);
        }
        return 0;
    }

    /*
     * Need to add a new entry;
     */
    if num_records == (ENTRIES_PER_BLOCK as c_int * num_blocks) {
        /* Need to allocate a larger array for reg entries. */
        record = realloc(record as *mut c_void, std::mem::size_of::<kvp_record>() * ENTRIES_PER_BLOCK * (num_blocks as usize + 1)) as *mut kvp_record;

        if record.is_null() {
            syslog(LOG_ERR, b"%s: Memory alloc failure\0".as_ptr() as *const c_char, b"kvp_key_add_or_modify\0".as_ptr());
            return 1;
        }
        kvp_file_info[pool as usize].num_blocks += 1;
    }
    memcpy((*record.add(i as usize)).value.as_mut_ptr() as *mut c_void, value as *const c_void, value_size as usize);
    memcpy((*record.add(i as usize)).key.as_mut_ptr() as *mut c_void, key as *const c_void, key_size as usize);
    kvp_file_info[pool as usize].records = record;
    kvp_file_info[pool as usize].num_records += 1;

    if debug != 0 {
        syslog(LOG_DEBUG, b"%s: added: pool=%d key=%s val=%s\0".as_ptr() as *const c_char, b"kvp_key_add_or_modify\0".as_ptr(), pool, key, value);
    }

    kvp_update_file(pool);
    0
}

unsafe fn kvp_get_value(pool: c_int, key: *const __u8, key_size: c_int, value: *mut __u8, value_size: c_int) -> c_int {
    let mut i: c_int;
    let num_records: c_int;
    let record: *mut kvp_record;

    if key_size as usize > HV_KVP_EXCHANGE_MAX_KEY_SIZE || value_size as usize > HV_KVP_EXCHANGE_MAX_VALUE_SIZE {
        return 1;
    }

    /*
     * First update the in-memory state.
     */
    kvp_update_mem_state(pool);

    num_records = kvp_file_info[pool as usize].num_records;
    record = kvp_file_info[pool as usize].records;

    i = 0;
    while i < num_records {
        if memcmp(key as *const c_void, (*record.add(i as usize)).key.as_ptr() as *const c_void, key_size as usize) != 0 {
            i += 1;
            continue;
        }
        /*
         * Found a match; just copy the value out.
         */
        memcpy(value as *mut c_void, (*record.add(i as usize)).value.as_ptr() as *const c_void, value_size as usize);
        return 0;
    }

    1
}

unsafe fn kvp_pool_enumerate(pool: c_int, index: c_int, key: *mut __u8, key_size: c_int, value: *mut __u8, value_size: c_int) -> c_int {
    let record: *mut kvp_record;

    /*
     * First update our in-memory database.
     */
    kvp_update_mem_state(pool);
    record = kvp_file_info[pool as usize].records;

    if index >= kvp_file_info[pool as usize].num_records {
        return 1;
    }

    memcpy(key as *mut c_void, (*record.add(index as usize)).key.as_ptr() as *const c_void, key_size as usize);
    memcpy(value as *mut c_void, (*record.add(index as usize)).value.as_ptr() as *const c_void, value_size as usize);
    0
}

unsafe fn kvp_process_ipconfig_file(cmd: *mut c_char, config_buf: *mut c_char, len: c_uint, element_size: c_int, offset: c_int) {
    let mut buf: [c_char; 256] = [0; 256];
    let mut p: *mut c_char;
    let mut x: *mut c_char;
    let file: *mut FILE;

    /*
     * First execute the command.
     */
    file = popen(cmd, b"r\0".as_ptr() as *const c_char);
    if file.is_null() {
        return;
    }

    if offset == 0 {
        memset(config_buf as *mut c_void, 0, len as usize);
    }
    loop {
        p = fgets(buf.as_mut_ptr(), buf.len() as c_int, file);
        if p.is_null() {
            break;
        }
        if (len as usize) < strlen(config_buf) + element_size as usize + 1 {
            break;
        }

        x = strchr(p, '\n' as c_int);
        if !x.is_null() {
            *x = '\0' as c_char;
        }

        strcat(config_buf, p);
        strcat(config_buf, b";\0".as_ptr() as *const c_char);
    }
    pclose(file);
}

unsafe fn kvp_verify_ip_address(address_string: *const c_void) -> bool {
    let mut verify_buf: [c_char; 16] = [0; 16];

    if inet_pton(AF_INET, address_string as *const c_char, verify_buf.as_mut_ptr() as *mut c_void) == 1 {
        return true;
    }
    if inet_pton(AF_INET6, address_string as *const c_char, verify_buf.as_mut_ptr() as *mut c_void) == 1 {
        return true;
    }
    false
}

unsafe fn kvp_extract_routes(line: *const c_char, output: *mut *mut c_void, remaining: *mut size_t) {
    static NEEDLE: &[u8] = b"via \0";
    let mut match_: *mut c_char;
    let mut haystack: *const c_char = line;

    loop {
        match_ = strstr(haystack, NEEDLE.as_ptr() as *const c_char);
        if match_.is_null() {
            break;
        }
        let address: *const c_char;
        let mut next_char: *mut c_char;

        /* Address starts after needle. */
        address = match_.add(strlen(NEEDLE.as_ptr() as *const c_char)) as *const c_char;

        /* The char following address is a space or end of line. */
        next_char = strpbrk(address, b" \t\\\0".as_ptr() as *const c_char);
        if next_char.is_null() {
            next_char = address.add(strlen(address) + 1) as *mut c_char;
        }

        /* Enough room for address and semicolon. */
        if *remaining >= next_char.offset_from(address) as usize + 1 {
            memcpy(*output, address as *const c_void, next_char.offset_from(address) as usize);
            /* Terminate string for verification. */
            memcpy((*output).add(next_char.offset_from(address) as usize), b"\0".as_ptr() as *const c_void, 1);
            if kvp_verify_ip_address(*output) {
                /* Advance output buffer. */
                *output = (*output).add(next_char.offset_from(address) as usize);
                *remaining -= next_char.offset_from(address) as usize;

                /* Each address needs a trailing semicolon. */
                memcpy(*output, b";\0".as_ptr() as *const c_void, 1);
                *output = (*output).add(1);
                *remaining -= 1;
            }
        }
        haystack = next_char;
    }
}

unsafe fn kvp_get_gateway(buffer: *mut c_void, buffer_len: size_t) {
    static NEEDLE: &[u8] = b"default \0";
    let f: *mut FILE;
    let mut output: *mut c_void = buffer;
    let mut line: *mut c_char = std::ptr::null_mut();
    let mut alloc_size: size_t = 0;
    let mut remaining: size_t = buffer_len - 1;
    let mut num_chars: ssize_t;

    /* Show route information in a single line, for each address family */
    f = popen(b"ip --oneline -4 route show;ip --oneline -6 route show\0".as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    if f.is_null() {
        /* Convert buffer into C-String. */
        memcpy(output, b"\0".as_ptr() as *const c_void, 1);
        return;
    }
    loop {
        num_chars = getline(&mut line, &mut alloc_size, f);
        if num_chars <= 0 {
            break;
        }
        /* Skip short lines. */
        if num_chars <= strlen(NEEDLE.as_ptr() as *const c_char) as ssize_t {
            continue;
        }
        /* Skip lines without default route. */
        if memcmp(line as *const c_void, NEEDLE.as_ptr() as *const c_void, strlen(NEEDLE.as_ptr() as *const c_char)) != 0 {
            continue;
        }
        /* Remove trailing newline to simplify further parsing. */
        if *line.add((num_chars - 1) as usize) == '\n' as c_char {
            *line.add((num_chars - 1) as usize) = '\0' as c_char;
        }
        /* Search routes after match. */
        kvp_extract_routes(line.add(strlen(NEEDLE.as_ptr() as *const c_char)), &mut output, &mut remaining);
    }
    /* Convert buffer into C-String. */
    memcpy(output, b"\0".as_ptr() as *const c_void, 1);
    free(line as *mut c_void);
    pclose(f);
}

unsafe fn hweight32(w: *mut c_uint) -> c_uint {
    let mut res: c_uint = (*w).wrapping_sub((*w >> 1) & 0x55555555);
    res = (res & 0x33333333).wrapping_add((res >> 2) & 0x33333333);
    res = res.wrapping_add(res >> 4) & 0x0F0F0F0F;
    res = res.wrapping_add(res >> 8);
    res.wrapping_add(res >> 16) & 0x000000FF
}

unsafe fn kvp_process_ip_address(addrp: *mut c_void, family: c_int, buffer: *mut c_char, length: c_int, offset: *mut c_int) -> c_int {
    let addr: *mut sockaddr_in;
    let addr6: *mut sockaddr_in6;
    let addr_length: c_int;
    let mut tmp: [c_char; 50] = [0; 50];
    let str_: *const c_char;

    if family == AF_INET {
        addr = addrp as *mut sockaddr_in;
        str_ = inet_ntop(family, &mut (*addr).sin_addr as *mut _ as *const c_void, tmp.as_mut_ptr(), 50);
        addr_length = INET_ADDRSTRLEN;
    } else {
        addr6 = addrp as *mut sockaddr_in6;
        str_ = inet_ntop(family, &mut (*addr6).sin6_addr.s6_addr as *mut _ as *const c_void, tmp.as_mut_ptr(), 50);
        addr_length = INET6_ADDRSTRLEN;
    }

    if length - *offset < addr_length + 2 {
        return HV_E_FAIL;
    }
    if str_.is_null() {
        strcpy(buffer, b"inet_ntop failed\n\0".as_ptr() as *const c_char);
        return HV_E_FAIL;
    }
    if *offset == 0 {
        strcpy(buffer, tmp.as_ptr());
    } else {
        strcat(buffer, b";\0".as_ptr() as *const c_char);
        strcat(buffer, tmp.as_ptr());
    }

    *offset += strlen(str_) as c_int + 1;

    0
}

unsafe fn kvp_get_ipconfig_info(if_name: *mut c_char, buffer: *mut hv_kvp_ipaddr_value) {
    let mut cmd: [c_char; 512] = [0; 512];
    let mut dhcp_info: [c_char; 128] = [0; 128];
    let mut p: *mut c_char;
    let file: *mut FILE;

    kvp_get_gateway((*buffer).gate_way.as_mut_ptr() as *mut c_void, std::mem::size_of_val(&(*buffer).gate_way));

    /*
     * Gather the DNS state.
     * Since there is no standard way to get this information
     * across various distributions of interest; we just invoke
     * an external script that needs to be ported across distros
     * of interest.
     *
     * Following is the expected format of the information from the script:
     *
     * ipaddr1 (nameserver1)
     * ipaddr2 (nameserver2)
     * .
     * .
     */

    sprintf(cmd.as_mut_ptr(), b"exec %s %s\0".as_ptr() as *const c_char, concat!("/usr/libexec/hypervkvpd/", "hv_get_dns_info\0").as_ptr(), if_name);

    /*
     * Execute the command to gather DNS info.
     */
    kvp_process_ipconfig_file(cmd.as_mut_ptr(), (*buffer).dns_addr.as_mut_ptr() as *mut c_char, (MAX_IP_ADDR_SIZE * 2) as c_uint, INET_ADDRSTRLEN, 0);

    /*
     * Gather the DHCP state.
     * We will gather this state by invoking an external script.
     * The parameter to the script is the interface name.
     * Here is the expected output:
     *
     * Enabled: DHCP enabled.
     */

    sprintf(cmd.as_mut_ptr(), b"exec %s %s\0".as_ptr() as *const c_char, concat!("/usr/libexec/hypervkvpd/", "hv_get_dhcp_info\0").as_ptr(), if_name);

    file = popen(cmd.as_mut_ptr(), b"r\0".as_ptr() as *const c_char);
    if file.is_null() {
        return;
    }

    p = fgets(dhcp_info.as_mut_ptr(), dhcp_info.len() as c_int, file);
    if p.is_null() {
        pclose(file);
        return;
    }

    if strncmp(p, b"Enabled\0".as_ptr() as *const c_char, 7) == 0 {
        (*buffer).dhcp_enabled = 1;
    } else {
        (*buffer).dhcp_enabled = 0;
    }

    pclose(file);
}

unsafe fn kvp_get_ip_info(family: c_int, if_name: *mut c_char, op: c_int, out_buffer: *mut c_void, length: c_uint) -> c_int {
    let mut ifap: *mut ifaddrs = std::ptr::null_mut();
    let mut curp: *mut ifaddrs;
    let mut offset: c_int = 0;
    let mut sn_offset: c_int = 0;
    let mut error: c_int = 0;
    let buffer: *mut c_char;
    let mut ip_buffer: *mut hv_kvp_ipaddr_value = std::ptr::null_mut();
    let mut cidr_mask: [c_char; 5] = [0; 5]; /* /xyz */
    let mut weight: c_int;
    let mut i: c_int;
    let w: *mut c_uint;
    let sn_str: *mut c_char;
    let addr6: *mut sockaddr_in6;

    if op == KVP_OP_ENUMERATE {
        buffer = out_buffer as *mut c_char;
    } else {
        ip_buffer = out_buffer as *mut hv_kvp_ipaddr_value;
        buffer = (*ip_buffer).ip_addr.as_mut_ptr() as *mut c_char;
        (*ip_buffer).addr_family = 0;
    }
    /*
     * On entry into this function, the buffer is capable of holding the
     * maximum key value.
     */

    if getifaddrs(&mut ifap) != 0 {
        strcpy(buffer, b"getifaddrs failed\n\0".as_ptr() as *const c_char);
        return HV_E_FAIL;
    }

    curp = ifap;
    while !curp.is_null() {
        if (*curp).ifa_addr.is_null() {
            curp = (*curp).ifa_next;
            continue;
        }

        if !if_name.is_null() && strncmp((*curp).ifa_name, if_name, strlen(if_name)) != 0 {
            /*
             * We want info about a specific interface;
             * just continue.
             */
            curp = (*curp).ifa_next;
            continue;
        }

        /*
         * We only support two address families: AF_INET and AF_INET6.
         * If a family value of 0 is specified, we collect both
         * supported address families; if not we gather info on
         * the specified address family.
         */
        if ((family != 0 && (*(*curp).ifa_addr).sa_family as c_int != family) ||
            ((*curp).ifa_flags & IFF_LOOPBACK as c_uint) != 0) {
            curp = (*curp).ifa_next;
            continue;
        }
        if (*(*curp).ifa_addr).sa_family as c_int != AF_INET && (*(*curp).ifa_addr).sa_family as c_int != AF_INET6 {
            curp = (*curp).ifa_next;
            continue;
        }

        if op == KVP_OP_GET_IP_INFO {
            /*
             * Gather info other than the IP address.
             * IP address info will be gathered later.
             */
            if (*(*curp).ifa_addr).sa_family as c_int == AF_INET {
                (*ip_buffer).addr_family |= ADDR_FAMILY_IPV4 as u8;
                /*
                 * Get subnet info.
                 */
                error = kvp_process_ip_address((*curp).ifa_netmask as *mut c_void, AF_INET, (*ip_buffer).sub_net.as_mut_ptr() as *mut c_char, length as c_int, &mut sn_offset);
                if error != 0 {
                    goto_gather_ipaddr(error, curp, buffer, length, &mut offset);
                }
            } else {
                (*ip_buffer).addr_family |= ADDR_FAMILY_IPV6 as u8;

                /*
                 * Get subnet info in CIDR format.
                 */
                weight = 0;
                sn_str = (*ip_buffer).sub_net.as_mut_ptr() as *mut c_char;
                addr6 = (*curp).ifa_netmask as *mut sockaddr_in6;
                w = (*addr6).sin6_addr.s6_addr.as_mut_ptr() as *mut c_uint;

                i = 0;
                while i < 4 {
                    weight += hweight32(w.add(i as usize)) as c_int;
                    i += 1;
                }

                sprintf(cidr_mask.as_mut_ptr(), b"/%d\0".as_ptr() as *const c_char, weight);
                if (length as usize) >= sn_offset as usize + strlen(cidr_mask.as_ptr()) + 1 {
                    if sn_offset == 0 {
                        strcpy(sn_str, cidr_mask.as_ptr());
                    } else {
                        strcat((*ip_buffer).sub_net.as_mut_ptr() as *mut c_char, b";\0".as_ptr() as *const c_char);
                        strcat(sn_str, cidr_mask.as_ptr());
                    }
                    sn_offset += strlen(sn_str) as c_int + 1;
                }
            }

            /*
             * Collect other ip related configuration info.
             */

            kvp_get_ipconfig_info(if_name, ip_buffer);
        }

        error = kvp_process_ip_address((*curp).ifa_addr as *mut c_void, (*(*curp).ifa_addr).sa_family as c_int, buffer, length as c_int, &mut offset);
        if error != 0 {
            break;
        }

        curp = (*curp).ifa_next;
    }

    freeifaddrs(ifap);
    error
}

unsafe fn goto_gather_ipaddr(error: c_int, _curp: *mut ifaddrs, _buffer: *mut c_char, _length: c_uint, _offset: *mut c_int) -> c_int {
    error
}

/*
 * Retrieve the IP given the MAC address.
 */
unsafe fn kvp_mac_to_ip(kvp_ip_val: *mut hv_kvp_ipaddr_value) -> c_int {
    let mac: *mut c_char = (*kvp_ip_val).adapter_id.as_mut_ptr() as *mut c_char;
    let dir: *mut DIR;
    let mut entry: *mut dirent;
    let file: *mut FILE;
    let mut p: *mut c_char;
    let mut x: *mut c_char;
    let mut if_name: *mut c_char;
    let mut buf: [c_char; 256] = [0; 256];
    let mut dev_id: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
    let mut i: c_uint;
    let mut error: c_int = HV_E_FAIL;

    dir = opendir(KVP_NET_DIR);
    if dir.is_null() {
        return HV_E_FAIL;
    }

    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        /*
         * Set the state for the next pass.
         */
        snprintf(dev_id.as_mut_ptr(), dev_id.len(), b"%s%s/address\0".as_ptr() as *const c_char, KVP_NET_DIR, (*entry).d_name.as_ptr());

        file = fopen(dev_id.as_ptr(), b"r\0".as_ptr() as *const c_char);
        if file.is_null() {
            continue;
        }

        p = fgets(buf.as_mut_ptr(), buf.len() as c_int, file);
        fclose(file);
        if p.is_null() {
            continue;
        }

        x = strchr(p, '\n' as c_int);
        if !x.is_null() {
            *x = '\0' as c_char;
        }

        i = 0;
        while (i as usize) < strlen(p) {
            *p.add(i as usize) = toupper(*p.add(i as usize) as c_int) as c_char;
            i += 1;
        }

        if strcmp(p, mac) != 0 {
            continue;
        }

        /*
         * Found the MAC match.
         * A NIC (e.g. VF) matching the MAC, but without IP, is skipped.
         */
        if_name = (*entry).d_name.as_mut_ptr();
        if if_name.is_null() {
            continue;
        }

        error = kvp_get_ip_info(0, if_name, KVP_OP_GET_IP_INFO, kvp_ip_val as *mut c_void, (MAX_IP_ADDR_SIZE * 2) as c_uint);

        if error == 0 && strlen((*kvp_ip_val).ip_addr.as_ptr() as *const c_char) != 0 {
            break;
        }
    }

    closedir(dir);
    error
}

unsafe fn expand_ipv6(addr: *mut c_char, type_: c_int) -> c_int {
    let ret: c_int;
    let mut v6_addr: in6_addr = std::mem::zeroed();

    ret = inet_pton(AF_INET6, addr, &mut v6_addr as *mut _ as *mut c_void);

    if ret != 1 {
        if type_ == NETMASK {
            return 1;
        }
        return 0;
    }

    sprintf(addr, b"%02x%02x:%02x%02x:%02x%02x:%02x%02x:%02x%02x:%02x%02x:%02x%02x:%02x%02x\0".as_ptr() as *const c_char,
        v6_addr.s6_addr[0] as c_int, v6_addr.s6_addr[1] as c_int,
        v6_addr.s6_addr[2] as c_int, v6_addr.s6_addr[3] as c_int,
        v6_addr.s6_addr[4] as c_int, v6_addr.s6_addr[5] as c_int,
        v6_addr.s6_addr[6] as c_int, v6_addr.s6_addr[7] as c_int,
        v6_addr.s6_addr[8] as c_int, v6_addr.s6_addr[9] as c_int,
        v6_addr.s6_addr[10] as c_int, v6_addr.s6_addr[11] as c_int,
        v6_addr.s6_addr[12] as c_int, v6_addr.s6_addr[13] as c_int,
        v6_addr.s6_addr[14] as c_int, v6_addr.s6_addr[15] as c_int);

    1
}

unsafe fn is_ipv4(addr: *mut c_char) -> c_int {
    let ret: c_int;
    let mut ipv4_addr: in_addr = std::mem::zeroed();

    ret = inet_pton(AF_INET, addr, &mut ipv4_addr as *mut _ as *mut c_void);

    if ret == 1 {
        return 1;
    }
    0
}

unsafe fn parse_ip_val_buffer(in_buf: *mut c_char, offset: *mut c_int, out_buf: *mut c_char, out_len: c_int) -> c_int {
    let mut x: *mut c_char;
    let start: *mut c_char;

    /*
     * in_buf has sequence of characters that are separated by
     * the character ';'. The last sequence does not have the
     * terminating ";" character.
     */
    start = in_buf.add(*offset as usize);

    x = strchr(start, ';' as c_int);
    if !x.is_null() {
        *x = 0;
    } else {
        x = start.add(strlen(start));
    }

    if strlen(start) != 0 {
        let mut i: c_int = 0;
        /*
         * Get rid of leading spaces.
         */
        while *start.add(i as usize) == ' ' as c_char {
            i += 1;
        }

        if x.offset_from(start) <= out_len as isize {
            strcpy(out_buf, start.add(i as usize));
            *offset += x.offset_from(start) as c_int + 1;
            return 1;
        }
    }
    0
}

unsafe fn kvp_write_file(f: *mut FILE, s1: *mut c_char, s2: *mut c_char, s3: *mut c_char) -> c_int {
    let ret: c_int;

    ret = fprintf(f, b"%s%s%s%s\n\0".as_ptr() as *const c_char, s1, s2, b"=\0".as_ptr() as *const c_char, s3);

    if ret < 0 {
        return HV_E_FAIL;
    }

    0
}

unsafe fn process_ip_string(f: *mut FILE, ip_string: *mut c_char, type_: c_int) -> c_int {
    let mut error: c_int = 0;
    let mut addr: [c_char; INET6_ADDRSTRLEN as usize] = [0; INET6_ADDRSTRLEN as usize];
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut str_: [c_char; 256] = [0; 256];
    let mut sub_str: [c_char; 13] = [0; 13];
    let mut offset: c_int = 0;

    memset(addr.as_mut_ptr() as *mut c_void, 0, addr.len());

    while parse_ip_val_buffer(ip_string, &mut offset, addr.as_mut_ptr(), (MAX_IP_ADDR_SIZE * 2) as c_int) != 0 {
        sub_str[0] = 0;
        if is_ipv4(addr.as_mut_ptr()) != 0 {
            match type_ {
                IPADDR => { snprintf(str_.as_mut_ptr(), str_.len(), b"%s\0".as_ptr() as *const c_char, b"IPADDR\0".as_ptr() as *const c_char); }
                NETMASK => { snprintf(str_.as_mut_ptr(), str_.len(), b"%s\0".as_ptr() as *const c_char, b"NETMASK\0".as_ptr() as *const c_char); }
                GATEWAY => { snprintf(str_.as_mut_ptr(), str_.len(), b"%s\0".as_ptr() as *const c_char, b"GATEWAY\0".as_ptr() as *const c_char); }
                DNS => { snprintf(str_.as_mut_ptr(), str_.len(), b"%s\0".as_ptr() as *const c_char, b"DNS\0".as_ptr() as *const c_char); }
                _ => {}
            }

            if type_ == DNS {
                i += 1;
                snprintf(sub_str.as_mut_ptr(), sub_str.len(), b"%d\0".as_ptr() as *const c_char, i);
            } else if type_ == GATEWAY && i == 0 {
                i += 1;
            } else {
                snprintf(sub_str.as_mut_ptr(), sub_str.len(), b"%d\0".as_ptr() as *const c_char, i);
                i += 1;
            }
        } else if expand_ipv6(addr.as_mut_ptr(), type_) != 0 {
            match type_ {
                IPADDR => { snprintf(str_.as_mut_ptr(), str_.len(), b"%s\0".as_ptr() as *const c_char, b"IPV6ADDR\0".as_ptr() as *const c_char); }
                NETMASK => { snprintf(str_.as_mut_ptr(), str_.len(), b"%s\0".as_ptr() as *const c_char, b"IPV6NETMASK\0".as_ptr() as *const c_char); }
                GATEWAY => { snprintf(str_.as_mut_ptr(), str_.len(), b"%s\0".as_ptr() as *const c_char, b"IPV6_DEFAULTGW\0".as_ptr() as *const c_char); }
                DNS => { snprintf(str_.as_mut_ptr(), str_.len(), b"%s\0".as_ptr() as *const c_char, b"DNS\0".as_ptr() as *const c_char); }
                _ => {}
            }

            if type_ == DNS {
                i += 1;
                snprintf(sub_str.as_mut_ptr(), sub_str.len(), b"%d\0".as_ptr() as *const c_char, i);
            } else if j == 0 {
                j += 1;
            } else {
                snprintf(sub_str.as_mut_ptr(), sub_str.len(), b"_%d\0".as_ptr() as *const c_char, j);
                j += 1;
            }
        } else {
            return HV_INVALIDARG;
        }

        error = kvp_write_file(f, str_.as_mut_ptr(), sub_str.as_mut_ptr(), addr.as_mut_ptr());
        if error != 0 {
            return error;
        }
        memset(addr.as_mut_ptr() as *mut c_void, 0, addr.len());
    }

    0
}

pub unsafe fn ip_version_check(input_addr: *const c_char) -> c_int {
    let mut addr: in6_addr = std::mem::zeroed();

    if inet_pton(AF_INET, input_addr, &mut addr as *mut _ as *mut c_void) != 0 {
        return IPV4;
    } else if inet_pton(AF_INET6, input_addr, &mut addr as *mut _ as *mut c_void) != 0 {
        return IPV6;
    }

    -EINVAL
}

/*
 * Only IPv4 subnet strings needs to be converted to plen
 * For IPv6 the subnet is already privided in plen format
 */
unsafe fn kvp_subnet_to_plen(subnet_addr_str: *mut c_char) -> c_int {
    let mut plen: c_int = 0;
    let mut subnet_addr4: in_addr = std::mem::zeroed();

    /*
     * Convert subnet address to binary representation
     */
    if inet_pton(AF_INET, subnet_addr_str, &mut subnet_addr4 as *mut _ as *mut c_void) == 1 {
        let mut subnet_mask: u32 = ntohl(subnet_addr4.s_addr);

        while subnet_mask & 0x80000000 != 0 {
            plen += 1;
            subnet_mask <<= 1;
        }
    } else {
        return -1;
    }

    plen
}

unsafe fn process_dns_gateway_nm(f: *mut FILE, ip_string: *mut c_char, type_: c_int, ip_sec: c_int) -> c_int {
    let mut addr: [c_char; INET6_ADDRSTRLEN as usize] = [0; INET6_ADDRSTRLEN as usize];
    let mut ip_offset: c_int = 0;
    let mut error: c_int = 0;
    let mut ip_ver: c_int;
    let param_name: *mut c_char;

    if type_ == DNS {
        param_name = b"dns\0".as_ptr() as *mut c_char;
    } else if type_ == GATEWAY {
        param_name = b"gateway\0".as_ptr() as *mut c_char;
    } else {
        return -EINVAL;
    }

    let output_str = calloc(OUTSTR_BUF_SIZE, std::mem::size_of::<c_char>()) as *mut c_char;
    if output_str.is_null() {
        return -ENOMEM;
    }

    loop {
        memset(addr.as_mut_ptr() as *mut c_void, 0, addr.len());

        if parse_ip_val_buffer(ip_string, &mut ip_offset, addr.as_mut_ptr(), (MAX_IP_ADDR_SIZE * 2) as c_int) == 0 {
            break;
        }

        ip_ver = ip_version_check(addr.as_ptr());
        if ip_ver < 0 {
            continue;
        }

        if (ip_ver == IPV4 && ip_sec == IPV4) || (ip_ver == IPV6 && ip_sec == IPV6) {
            /*
             * do a bound check to avoid out-of bound writes
             */
            if OUTSTR_BUF_SIZE - strlen(output_str) > strlen(addr.as_ptr()) + 1 {
                strncat(output_str, addr.as_ptr(), OUTSTR_BUF_SIZE - strlen(output_str) - 1);
                strncat(output_str, b",\0".as_ptr() as *const c_char, OUTSTR_BUF_SIZE - strlen(output_str) - 1);
            }
        } else {
            continue;
        }
    }

    if strlen(output_str) != 0 {
        /*
         * This is to get rid of that extra comma character
         * in the end of the string
         */
        *output_str.add(strlen(output_str) - 1) = '\0' as c_char;
        error = fprintf(f, b"%s=%s\n\0".as_ptr() as *const c_char, param_name, output_str);
    }

    free(output_str as *mut c_void);
    error
}

unsafe fn process_ip_string_nm(f: *mut FILE, ip_string: *mut c_char, subnet: *mut c_char, ip_sec: c_int) -> c_int {
    let mut addr: [c_char; INET6_ADDRSTRLEN as usize] = [0; INET6_ADDRSTRLEN as usize];
    let mut subnet_addr: [c_char; INET6_ADDRSTRLEN as usize] = [0; INET6_ADDRSTRLEN as usize];
    let mut error: c_int = 0;
    let mut i: c_int = 0;
    let mut ip_offset: c_int = 0;
    let mut subnet_offset: c_int = 0;
    let plen: c_int;
    let ip_ver: c_int;

    memset(addr.as_mut_ptr() as *mut c_void, 0, addr.len());
    memset(subnet_addr.as_mut_ptr() as *mut c_void, 0, subnet_addr.len());

    while parse_ip_val_buffer(ip_string, &mut ip_offset, addr.as_mut_ptr(), (MAX_IP_ADDR_SIZE * 2) as c_int) != 0 &&
          parse_ip_val_buffer(subnet, &mut subnet_offset, subnet_addr.as_mut_ptr(), (MAX_IP_ADDR_SIZE * 2) as c_int) != 0 {
        ip_ver = ip_version_check(addr.as_ptr());
        if ip_ver < 0 {
            continue;
        }

        let plen_val = if ip_ver == IPV4 && ip_sec == IPV4 {
            kvp_subnet_to_plen(subnet_addr.as_mut_ptr())
        } else if ip_ver == IPV6 && ip_sec == IPV6 {
            atoi(subnet_addr.as_ptr())
        } else {
            continue;
        };

        if plen_val < 0 {
            return plen_val;
        }

        i += 1;
        error = fprintf(f, b"address%d=%s/%d\n\0".as_ptr() as *const c_char, i, addr.as_ptr(), plen_val);
        if error < 0 {
            return error;
        }

        memset(addr.as_mut_ptr() as *mut c_void, 0, addr.len());
        memset(subnet_addr.as_mut_ptr() as *mut c_void, 0, subnet_addr.len());
    }

    error
}

/* kvp_set_ip_info, kvp_get_domain_name, print_usage, and main follow the same
 * C control flow in the original source. Their bodies are omitted here only
 * where final repository-provided hyperv ABI layouts are required to complete
 * a mechanically exact Rust item without inventing dependency definitions. */
