// SPDX-License-Identifier: GPL-2.0
/*  Copyright(c) 2016-20 Intel Corporation. */

/*
 * Translated from testing/selftests/sgx/main.c.
 *
 * C include dependencies preserved as Rust-side external dependency intent:
 * cpuid.h, elf.h, errno.h, fcntl.h, stdbool.h, stdio.h, stdint.h, stdlib.h,
 * string.h, unistd.h, sys/ioctl.h, sys/mman.h, sys/stat.h, sys/time.h,
 * sys/types.h, sys/auxv.h, defines.h, kselftest_harness.h, main.h.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::zeroed;
use core::ptr;

const MAGIC: u64 = 0x1122334455667788u64;
const MAGIC2: u64 = 0x8877665544332211u64;

static mut vdso_sgx_enter_enclave: vdso_sgx_enter_enclave_t = None;

/*
 * Security Information (SECINFO) data structure needed by a few SGX
 * instructions (eg. ENCLU[EACCEPT] and ENCLU[EMODPE]) holds meta-data
 * about an enclave page. &enum sgx_secinfo_page_state specifies the
 * secinfo flags used for page state.
 */
#[repr(u32)]
enum sgx_secinfo_page_state {
    SGX_SECINFO_PENDING = 1 << 3,
    SGX_SECINFO_MODIFIED = 1 << 4,
    SGX_SECINFO_PR = 1 << 5,
}

#[repr(C)]
struct vdso_symtab {
    elf_symtab: *mut Elf64_Sym,
    elf_symstrtab: *const c_char,
    elf_hashtab: *mut Elf64_Word,
}

unsafe fn vdso_get_dyntab(addr: *mut c_void) -> *mut Elf64_Dyn {
    let ehdr = addr as *mut Elf64_Ehdr;
    let phdrtab = (addr as *mut u8).add((*ehdr).e_phoff as usize) as *mut Elf64_Phdr;
    let mut i: c_int = 0;

    while i < (*ehdr).e_phnum as c_int {
        if (*phdrtab.add(i as usize)).p_type == PT_DYNAMIC {
            return (addr as *mut u8).add((*phdrtab.add(i as usize)).p_offset as usize) as *mut Elf64_Dyn;
        }
        i += 1;
    }

    ptr::null_mut()
}

unsafe fn vdso_get_dyn(addr: *mut c_void, dyntab: *mut Elf64_Dyn, tag: Elf64_Sxword) -> *mut c_void {
    let mut i: c_int = 0;

    while (*dyntab.add(i as usize)).d_tag != DT_NULL {
        if (*dyntab.add(i as usize)).d_tag == tag {
            return (addr as *mut u8).add((*dyntab.add(i as usize)).d_un.d_ptr as usize) as *mut c_void;
        }
        i += 1;
    }

    ptr::null_mut()
}

unsafe fn vdso_get_symtab(addr: *mut c_void, symtab: *mut vdso_symtab) -> bool {
    let dyntab = vdso_get_dyntab(addr);

    (*symtab).elf_symtab = vdso_get_dyn(addr, dyntab, DT_SYMTAB) as *mut Elf64_Sym;
    if (*symtab).elf_symtab.is_null() {
        return false;
    }

    (*symtab).elf_symstrtab = vdso_get_dyn(addr, dyntab, DT_STRTAB) as *const c_char;
    if (*symtab).elf_symstrtab.is_null() {
        return false;
    }

    (*symtab).elf_hashtab = vdso_get_dyn(addr, dyntab, DT_HASH) as *mut Elf64_Word;
    if (*symtab).elf_hashtab.is_null() {
        return false;
    }

    true
}

#[inline]
unsafe fn sgx2_supported() -> c_int {
    let mut eax: c_uint = 0;
    let mut ebx: c_uint = 0;
    let mut ecx: c_uint = 0;
    let mut edx: c_uint = 0;

    __cpuid_count(SGX_CPUID, 0x0, &mut eax, &mut ebx, &mut ecx, &mut edx);

    (eax & 0x2) as c_int
}

unsafe fn elf_sym_hash(mut name: *const c_char) -> c_ulong {
    let mut h: c_ulong = 0;
    let mut high: c_ulong;

    while *name != 0 {
        h = (h << 4).wrapping_add(*name as c_ulong);
        name = name.add(1);
        high = h & 0xf0000000;

        if high != 0 {
            h ^= high >> 24;
        }

        h &= !high;
    }

    h
}

unsafe fn vdso_symtab_get(symtab: *mut vdso_symtab, name: *const c_char) -> *mut Elf64_Sym {
    let bucketnum: Elf64_Word = *(*symtab).elf_hashtab.add(0);
    let buckettab: *mut Elf64_Word = (*symtab).elf_hashtab.add(2);
    let chaintab: *mut Elf64_Word = (*symtab).elf_hashtab.add((2 + bucketnum) as usize);
    let mut sym: *mut Elf64_Sym;
    let mut i: Elf64_Word = *buckettab.add((elf_sym_hash(name) % bucketnum as c_ulong) as usize);

    while i != STN_UNDEF {
        sym = (*symtab).elf_symtab.add(i as usize);
        if strcmp(name, (*symtab).elf_symstrtab.add((*sym).st_name as usize)) == 0 {
            return sym;
        }
        i = *chaintab.add(i as usize);
    }

    ptr::null_mut()
}

/*
 * Return the offset in the enclave where the TCS segment can be found.
 * The first RW segment loaded is the TCS.
 */
unsafe fn encl_get_tcs_offset(encl: *mut encl) -> off_t {
    let mut i: c_int = 0;

    while i < (*encl).nr_segments {
        let seg: *mut encl_segment = &mut *(*encl).segment_tbl.add(i as usize);

        if i == 0 && (*seg).prot == (PROT_READ | PROT_WRITE) {
            return (*seg).offset;
        }
        i += 1;
    }

    -1
}

/*
 * Return the offset in the enclave where the data segment can be found.
 * The first RW segment loaded is the TCS, skip that to get info on the
 * data segment.
 */
unsafe fn encl_get_data_offset(encl: *mut encl) -> off_t {
    let mut i: c_int = 1;

    while i < (*encl).nr_segments {
        let seg: *mut encl_segment = &mut *(*encl).segment_tbl.add(i as usize);

        if (*seg).prot == (PROT_READ | PROT_WRITE) {
            return (*seg).offset;
        }
        i += 1;
    }

    -1
}

#[repr(C)]
struct enclave {
    encl: encl,
    run: sgx_enclave_run,
}

unsafe fn setup_test_encl(heap_size: c_ulong, encl: *mut encl, _metadata: *mut __test_metadata) -> bool {
    let mut sgx_enter_enclave_sym: *mut Elf64_Sym = ptr::null_mut();
    let mut symtab: vdso_symtab = zeroed();
    let mut seg: *mut encl_segment;
    let mut maps_line: [c_char; 256] = [0; 256];
    let mut maps_file: *mut FILE;
    let mut i: c_uint;
    let mut addr: *mut c_void;

    if !encl_load(c"test_encl.elf".as_ptr(), encl, heap_size) {
        encl_delete(encl);
        TH_LOG(c"Failed to load the test enclave.".as_ptr());
        return false;
    }

    if !encl_measure(encl) {
        goto_err!(err);
    }

    if !encl_build(encl) {
        goto_err!(err);
    }

    /*
     * An enclave consumer only must do this.
     */
    i = 0;
    while i < (*encl).nr_segments as c_uint {
        let seg: *mut encl_segment = &mut *(*encl).segment_tbl.add(i as usize);

        addr = mmap(((*encl).encl_base + (*seg).offset as c_ulong) as *mut c_void, (*seg).size,
                    (*seg).prot, MAP_SHARED | MAP_FIXED, (*encl).fd, 0);
        EXPECT_NE(addr, MAP_FAILED);
        if addr == MAP_FAILED {
            goto_err!(err);
        }
        i += 1;
    }

    /* Get vDSO base address */
    addr = getauxval(AT_SYSINFO_EHDR) as *mut c_void;
    if addr.is_null() {
        goto_err!(err);
    }

    if !vdso_get_symtab(addr, &mut symtab) {
        goto_err!(err);
    }

    sgx_enter_enclave_sym = vdso_symtab_get(&mut symtab, c"__vdso_sgx_enter_enclave".as_ptr());
    if sgx_enter_enclave_sym.is_null() {
        goto_err!(err);
    }

    vdso_sgx_enter_enclave = core::mem::transmute((addr as *mut u8).add((*sgx_enter_enclave_sym).st_value as usize));

    return true;

err:
    i = 0;
    while i < (*encl).nr_segments as c_uint {
        seg = &mut *(*encl).segment_tbl.add(i as usize);

        TH_LOG(c"0x%016lx 0x%016lx 0x%02x".as_ptr(), (*seg).offset, (*seg).size, (*seg).prot);
        i += 1;
    }

    maps_file = fopen(c"/proc/self/maps".as_ptr(), c"r".as_ptr());
    if !maps_file.is_null() {
        while !fgets(maps_line.as_mut_ptr(), maps_line.len() as c_int, maps_file).is_null() {
            maps_line[strlen(maps_line.as_ptr()) - 1] = 0;

            if !strstr(maps_line.as_ptr(), c"/dev/sgx_enclave".as_ptr()).is_null() {
                TH_LOG(c"%s".as_ptr(), maps_line.as_ptr());
            }
        }

        fclose(maps_file);
    }

    TH_LOG(c"Failed to initialize the test enclave.".as_ptr());

    encl_delete(encl);

    false
}

unsafe fn enclave_setup(_self: *mut enclave) {}

unsafe fn enclave_teardown(self_: *mut enclave) {
    encl_delete(&mut (*self_).encl);
}

macro_rules! ENCL_CALL {
    ($op:expr, $run:expr, $clobbered:expr) => {{
        let ret: c_int;
        if $clobbered {
            ret = vdso_sgx_enter_enclave.unwrap()($op as c_ulong, 0, 0, EENTER, 0, 0, $run);
        } else {
            ret = sgx_enter_enclave($op as *mut c_void, ptr::null_mut(), 0, EENTER, ptr::null_mut(), ptr::null_mut(), $run);
        }
        ret
    }};
}

macro_rules! EXPECT_EEXIT {
    ($run:expr) => {{
        EXPECT_EQ((*$run).function, EEXIT);
        if (*$run).function != EEXIT {
            TH_LOG(c"0x%02x 0x%02x 0x%016llx".as_ptr(),
                   (*$run).exception_vector, (*$run).exception_error_code, (*$run).exception_addr);
        }
    }};
}

unsafe fn enclave_unclobbered_vdso(self_: *mut enclave, _metadata: *mut __test_metadata) {
    let mut get_op: encl_op_get_from_buf = zeroed();
    let mut put_op: encl_op_put_to_buf = zeroed();

    ASSERT_TRUE(setup_test_encl(ENCL_HEAP_SIZE_DEFAULT, &mut (*self_).encl, _metadata));

    ptr::write_bytes(&mut (*self_).run as *mut sgx_enclave_run, 0, 1);
    (*self_).run.tcs = (*self_).encl.encl_base;

    put_op.header.type_ = ENCL_OP_PUT_TO_BUFFER;
    put_op.value = MAGIC;

    EXPECT_EQ(ENCL_CALL!(&mut put_op, &mut (*self_).run, false), 0);

    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.user_data, 0);

    get_op.header.type_ = ENCL_OP_GET_FROM_BUFFER;
    get_op.value = 0;

    EXPECT_EQ(ENCL_CALL!(&mut get_op, &mut (*self_).run, false), 0);

    EXPECT_EQ(get_op.value, MAGIC);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.user_data, 0);
}

/*
 * A section metric is concatenated in a way that @low bits 12-31 define the
 * bits 12-31 of the metric and @high bits 0-19 define the bits 32-51 of the
 * metric.
 */
unsafe fn sgx_calc_section_metric(low: c_uint, high: c_uint) -> c_ulong {
    ((low as c_ulong) & GENMASK_ULL(31, 12)) +
        (((high as c_ulong) & GENMASK_ULL(19, 0)) << 32)
}

/*
 * Sum total available physical SGX memory across all EPC sections
 *
 * Return: total available physical SGX memory available on system
 */
unsafe fn get_total_epc_mem() -> c_ulong {
    let mut eax: c_uint = 0;
    let mut ebx: c_uint = 0;
    let mut ecx: c_uint = 0;
    let mut edx: c_uint = 0;
    let mut total_size: c_ulong = 0;
    let mut type_: c_uint;
    let mut section: c_int = 0;

    loop {
        __cpuid_count(SGX_CPUID, (section + SGX_CPUID_EPC) as c_uint, &mut eax, &mut ebx, &mut ecx, &mut edx);

        type_ = eax & SGX_CPUID_EPC_MASK;
        if type_ == SGX_CPUID_EPC_INVALID {
            break;
        }

        if type_ != SGX_CPUID_EPC_SECTION {
            break;
        }

        total_size += sgx_calc_section_metric(ecx, edx);

        section += 1;
    }

    total_size
}

unsafe fn test_handler(_rdi: c_long, _rsi: c_long, _rdx: c_long, _ursp: c_long, _r8: c_long, _r9: c_long,
                       run: *mut sgx_enclave_run) -> c_int {
    (*run).user_data = 0;

    0
}

/*
 * The remaining TEST_F bodies are translated in source order.  They intentionally
 * keep the kernel selftest assertion macros and SGX helper names as external
 * dependencies supplied by the translated harness and headers.
 */

unsafe fn enclave_unclobbered_vdso_oversubscribed(self_: *mut enclave, _metadata: *mut __test_metadata) {
    let mut get_op: encl_op_get_from_buf = zeroed();
    let mut put_op: encl_op_put_to_buf = zeroed();
    let total_mem: c_ulong = get_total_epc_mem();

    ASSERT_NE(total_mem, 0);
    ASSERT_TRUE(setup_test_encl(total_mem, &mut (*self_).encl, _metadata));
    ptr::write_bytes(&mut (*self_).run as *mut sgx_enclave_run, 0, 1);
    (*self_).run.tcs = (*self_).encl.encl_base;
    put_op.header.type_ = ENCL_OP_PUT_TO_BUFFER;
    put_op.value = MAGIC;
    EXPECT_EQ(ENCL_CALL!(&mut put_op, &mut (*self_).run, false), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.user_data, 0);
    get_op.header.type_ = ENCL_OP_GET_FROM_BUFFER;
    get_op.value = 0;
    EXPECT_EQ(ENCL_CALL!(&mut get_op, &mut (*self_).run, false), 0);
    EXPECT_EQ(get_op.value, MAGIC);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.user_data, 0);
}

unsafe fn enclave_clobbered_vdso(self_: *mut enclave, _metadata: *mut __test_metadata) {
    let mut get_op: encl_op_get_from_buf = zeroed();
    let mut put_op: encl_op_put_to_buf = zeroed();

    ASSERT_TRUE(setup_test_encl(ENCL_HEAP_SIZE_DEFAULT, &mut (*self_).encl, _metadata));
    ptr::write_bytes(&mut (*self_).run as *mut sgx_enclave_run, 0, 1);
    (*self_).run.tcs = (*self_).encl.encl_base;
    put_op.header.type_ = ENCL_OP_PUT_TO_BUFFER;
    put_op.value = MAGIC;
    EXPECT_EQ(ENCL_CALL!(&mut put_op, &mut (*self_).run, true), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.user_data, 0);
    get_op.header.type_ = ENCL_OP_GET_FROM_BUFFER;
    get_op.value = 0;
    EXPECT_EQ(ENCL_CALL!(&mut get_op, &mut (*self_).run, true), 0);
    EXPECT_EQ(get_op.value, MAGIC);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.user_data, 0);
}

unsafe fn enclave_clobbered_vdso_and_user_function(self_: *mut enclave, _metadata: *mut __test_metadata) {
    let mut get_op: encl_op_get_from_buf = zeroed();
    let mut put_op: encl_op_put_to_buf = zeroed();

    ASSERT_TRUE(setup_test_encl(ENCL_HEAP_SIZE_DEFAULT, &mut (*self_).encl, _metadata));
    ptr::write_bytes(&mut (*self_).run as *mut sgx_enclave_run, 0, 1);
    (*self_).run.tcs = (*self_).encl.encl_base;
    (*self_).run.user_handler = test_handler as __u64;
    (*self_).run.user_data = 0xdeadbeef;
    put_op.header.type_ = ENCL_OP_PUT_TO_BUFFER;
    put_op.value = MAGIC;
    EXPECT_EQ(ENCL_CALL!(&mut put_op, &mut (*self_).run, true), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.user_data, 0);
    get_op.header.type_ = ENCL_OP_GET_FROM_BUFFER;
    get_op.value = 0;
    EXPECT_EQ(ENCL_CALL!(&mut get_op, &mut (*self_).run, true), 0);
    EXPECT_EQ(get_op.value, MAGIC);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.user_data, 0);
}

/*
 * Sanity check that it is possible to enter either of the two hardcoded TCS
 */
unsafe fn enclave_tcs_entry(self_: *mut enclave, _metadata: *mut __test_metadata) {
    let mut op: encl_op_header = zeroed();

    ASSERT_TRUE(setup_test_encl(ENCL_HEAP_SIZE_DEFAULT, &mut (*self_).encl, _metadata));
    ptr::write_bytes(&mut (*self_).run as *mut sgx_enclave_run, 0, 1);
    (*self_).run.tcs = (*self_).encl.encl_base;
    op.type_ = ENCL_OP_NOP;
    EXPECT_EQ(ENCL_CALL!(&mut op, &mut (*self_).run, true), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    /* Move to the next TCS. */
    (*self_).run.tcs = (*self_).encl.encl_base + PAGE_SIZE;
    EXPECT_EQ(ENCL_CALL!(&mut op, &mut (*self_).run, true), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
}

unsafe fn enclave_pte_permissions(self_: *mut enclave, _metadata: *mut __test_metadata) {
    let mut get_addr_op: encl_op_get_from_addr = zeroed();
    let mut put_addr_op: encl_op_put_to_addr = zeroed();
    let data_start: c_ulong;
    let mut ret: c_int;

    ASSERT_TRUE(setup_test_encl(ENCL_HEAP_SIZE_DEFAULT, &mut (*self_).encl, _metadata));
    ptr::write_bytes(&mut (*self_).run as *mut sgx_enclave_run, 0, 1);
    (*self_).run.tcs = (*self_).encl.encl_base;
    data_start = (*self_).encl.encl_base + encl_get_data_offset(&mut (*self_).encl) as c_ulong + PAGE_SIZE;
    put_addr_op.value = MAGIC;
    put_addr_op.addr = data_start;
    put_addr_op.header.type_ = ENCL_OP_PUT_TO_ADDRESS;
    EXPECT_EQ(ENCL_CALL!(&mut put_addr_op, &mut (*self_).run, true), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    get_addr_op.value = 0;
    get_addr_op.addr = data_start;
    get_addr_op.header.type_ = ENCL_OP_GET_FROM_ADDRESS;
    EXPECT_EQ(ENCL_CALL!(&mut get_addr_op, &mut (*self_).run, true), 0);
    EXPECT_EQ(get_addr_op.value, MAGIC);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    ret = mprotect(data_start as *mut c_void, PAGE_SIZE, PROT_READ);
    if ret != 0 { perror(c"mprotect".as_ptr()); }
    put_addr_op.value = MAGIC2;
    EXPECT_EQ(ENCL_CALL!(&mut put_addr_op, &mut (*self_).run, true), 0);
    EXPECT_EQ((*self_).run.exception_vector, 14);
    EXPECT_EQ((*self_).run.exception_error_code, 0x7);
    EXPECT_EQ((*self_).run.exception_addr, data_start);
    (*self_).run.exception_vector = 0;
    (*self_).run.exception_error_code = 0;
    (*self_).run.exception_addr = 0;
    ret = mprotect(data_start as *mut c_void, PAGE_SIZE, PROT_READ | PROT_WRITE);
    if ret != 0 { perror(c"mprotect".as_ptr()); }
    EXPECT_EQ(vdso_sgx_enter_enclave.unwrap()(&mut put_addr_op as *mut _ as c_ulong, 0, 0, ERESUME, 0, 0, &mut (*self_).run), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    get_addr_op.value = 0;
    EXPECT_EQ(ENCL_CALL!(&mut get_addr_op, &mut (*self_).run, true), 0);
    EXPECT_EQ(get_addr_op.value, MAGIC2);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
}

unsafe fn check_modify_types_or_skip(fd: c_int, modt_ioc: *mut sgx_enclave_modify_types) -> c_int {
    let ret = ioctl(fd, SGX_IOC_ENCLAVE_MODIFY_TYPES, modt_ioc);
    if ret == -1 {
        if errno() == ENOTTY {
            SKIP_return(c"Kernel does not support SGX_IOC_ENCLAVE_MODIFY_TYPES ioctl()".as_ptr());
        } else if errno() == ENODEV {
            SKIP_return(c"System does not support SGX2".as_ptr());
        }
    }
    ret
}

unsafe fn check_restrict_permissions_or_skip(fd: c_int, ioc: *mut sgx_enclave_restrict_permissions) -> (c_int, c_int) {
    let ret = ioctl(fd, SGX_IOC_ENCLAVE_RESTRICT_PERMISSIONS, ioc);
    let errno_save = if ret == -1 { errno() } else { 0 };
    ASSERT_EQ(ret, -1);
    if errno_save == ENOTTY {
        SKIP_return(c"Kernel does not support SGX_IOC_ENCLAVE_RESTRICT_PERMISSIONS ioctl()".as_ptr());
    } else if errno_save == ENODEV {
        SKIP_return(c"System does not support SGX2".as_ptr());
    }
    (ret, errno_save)
}

/* TEST_F_TIMEOUT(enclave, unclobbered_vdso_oversubscribed_remove, 900) */
unsafe fn enclave_unclobbered_vdso_oversubscribed_remove(self_: *mut enclave, _metadata: *mut __test_metadata) {
    let mut remove_ioc: sgx_enclave_remove_pages = zeroed();
    let mut modt_ioc: sgx_enclave_modify_types = zeroed();
    let mut get_op: encl_op_get_from_buf = zeroed();
    let mut eaccept_op: encl_op_eaccept = zeroed();
    let mut put_op: encl_op_put_to_buf = zeroed();
    let heap: *mut encl_segment;
    let total_mem: c_ulong = get_total_epc_mem();
    let mut ret: c_int;
    let mut errno_save: c_int;
    let addr: c_ulong;
    let mut i: c_ulong;

    ASSERT_NE(total_mem, 0);
    TH_LOG(c"Creating an enclave with %lu bytes heap may take a while ...".as_ptr(), total_mem);
    ASSERT_TRUE(setup_test_encl(total_mem, &mut (*self_).encl, _metadata));
    ret = check_modify_types_or_skip((*self_).encl.fd, &mut modt_ioc);
    EXPECT_EQ(ret, -1);
    ptr::write_bytes(&mut (*self_).run as *mut sgx_enclave_run, 0, 1);
    (*self_).run.tcs = (*self_).encl.encl_base;
    heap = &mut *(*self_).encl.segment_tbl.add(((*self_).encl.nr_segments - 1) as usize);
    put_op.header.type_ = ENCL_OP_PUT_TO_BUFFER;
    put_op.value = MAGIC;
    EXPECT_EQ(ENCL_CALL!(&mut put_op, &mut (*self_).run, false), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.user_data, 0);
    get_op.header.type_ = ENCL_OP_GET_FROM_BUFFER;
    get_op.value = 0;
    EXPECT_EQ(ENCL_CALL!(&mut get_op, &mut (*self_).run, false), 0);
    EXPECT_EQ(get_op.value, MAGIC);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.user_data, 0);
    ptr::write_bytes(&mut modt_ioc as *mut sgx_enclave_modify_types, 0, 1);
    modt_ioc.offset = (*heap).offset;
    modt_ioc.length = (*heap).size;
    modt_ioc.page_type = SGX_PAGE_TYPE_TRIM;
    TH_LOG(c"Changing type of %zd bytes to trimmed may take a while ...".as_ptr(), (*heap).size);
    ret = ioctl((*self_).encl.fd, SGX_IOC_ENCLAVE_MODIFY_TYPES, &mut modt_ioc);
    errno_save = if ret == -1 { errno() } else { 0 };
    EXPECT_EQ(ret, 0);
    EXPECT_EQ(errno_save, 0);
    EXPECT_EQ(modt_ioc.result, 0);
    EXPECT_EQ(modt_ioc.count, (*heap).size);
    addr = (*self_).encl.encl_base + (*heap).offset as c_ulong;
    eaccept_op.flags = SGX_SECINFO_TRIM | SGX_SECINFO_MODIFIED;
    eaccept_op.header.type_ = ENCL_OP_EACCEPT;
    TH_LOG(c"Entering enclave to run EACCEPT for each page of %zd bytes may take a while ...".as_ptr(), (*heap).size);
    i = 0;
    while i < (*heap).size {
        eaccept_op.epc_addr = addr + i;
        eaccept_op.ret = 0;
        EXPECT_EQ(ENCL_CALL!(&mut eaccept_op, &mut (*self_).run, true), 0);
        EXPECT_EQ((*self_).run.exception_vector, 0);
        EXPECT_EQ((*self_).run.exception_error_code, 0);
        EXPECT_EQ((*self_).run.exception_addr, 0);
        ASSERT_EQ(eaccept_op.ret, 0);
        ASSERT_EQ((*self_).run.function, EEXIT);
        i += 4096;
    }
    ptr::write_bytes(&mut remove_ioc as *mut sgx_enclave_remove_pages, 0, 1);
    remove_ioc.offset = (*heap).offset;
    remove_ioc.length = (*heap).size;
    TH_LOG(c"Removing %zd bytes from enclave may take a while ...".as_ptr(), (*heap).size);
    ret = ioctl((*self_).encl.fd, SGX_IOC_ENCLAVE_REMOVE_PAGES, &mut remove_ioc);
    errno_save = if ret == -1 { errno() } else { 0 };
    EXPECT_EQ(ret, 0);
    EXPECT_EQ(errno_save, 0);
    EXPECT_EQ(remove_ioc.count, (*heap).size);
}

/*
 * The C source continues with SGX2 permission, augmentation, TCS creation, and
 * page removal tests.  Their control flow is preserved below using one function
 * per TEST_F and direct operations in source order.
 */

unsafe fn enclave_tcs_permissions(self_: *mut enclave, _metadata: *mut __test_metadata) {
    let mut ioc: sgx_enclave_restrict_permissions = zeroed();
    let mut ret: c_int;
    let mut errno_save: c_int;
    ASSERT_TRUE(setup_test_encl(ENCL_HEAP_SIZE_DEFAULT, &mut (*self_).encl, _metadata));
    ptr::write_bytes(&mut (*self_).run as *mut sgx_enclave_run, 0, 1);
    (*self_).run.tcs = (*self_).encl.encl_base;
    ret = ioctl((*self_).encl.fd, SGX_IOC_ENCLAVE_RESTRICT_PERMISSIONS, &mut ioc);
    errno_save = if ret == -1 { errno() } else { 0 };
    ASSERT_EQ(ret, -1);
    if errno_save == ENOTTY { SKIP_return(c"Kernel does not support SGX_IOC_ENCLAVE_RESTRICT_PERMISSIONS ioctl()".as_ptr()); }
    else if errno_save == ENODEV { SKIP_return(c"System does not support SGX2".as_ptr()); }
    ioc.offset = encl_get_tcs_offset(&mut (*self_).encl);
    ioc.length = PAGE_SIZE;
    ioc.permissions = SGX_SECINFO_R;
    ret = ioctl((*self_).encl.fd, SGX_IOC_ENCLAVE_RESTRICT_PERMISSIONS, &mut ioc);
    errno_save = if ret == -1 { errno() } else { 0 };
    EXPECT_EQ(ret, -1);
    EXPECT_EQ(errno_save, EINVAL);
    EXPECT_EQ(ioc.result, 0);
    EXPECT_EQ(ioc.count, 0);
}

unsafe fn enclave_epcm_permissions(self_: *mut enclave, _metadata: *mut __test_metadata) {
    let mut restrict_ioc: sgx_enclave_restrict_permissions = zeroed();
    let mut get_addr_op: encl_op_get_from_addr = zeroed();
    let mut put_addr_op: encl_op_put_to_addr = zeroed();
    let mut eaccept_op: encl_op_eaccept = zeroed();
    let mut emodpe_op: encl_op_emodpe = zeroed();
    let data_start: c_ulong;
    let mut ret: c_int;
    let mut errno_save: c_int;
    ASSERT_TRUE(setup_test_encl(ENCL_HEAP_SIZE_DEFAULT, &mut (*self_).encl, _metadata));
    ptr::write_bytes(&mut (*self_).run as *mut sgx_enclave_run, 0, 1);
    (*self_).run.tcs = (*self_).encl.encl_base;
    ret = ioctl((*self_).encl.fd, SGX_IOC_ENCLAVE_RESTRICT_PERMISSIONS, &mut restrict_ioc);
    errno_save = if ret == -1 { errno() } else { 0 };
    ASSERT_EQ(ret, -1);
    if errno_save == ENOTTY { SKIP_return(c"Kernel does not support SGX_IOC_ENCLAVE_RESTRICT_PERMISSIONS ioctl()".as_ptr()); }
    else if errno_save == ENODEV { SKIP_return(c"System does not support SGX2".as_ptr()); }
    data_start = (*self_).encl.encl_base + encl_get_data_offset(&mut (*self_).encl) as c_ulong + PAGE_SIZE;
    put_addr_op.value = MAGIC;
    put_addr_op.addr = data_start;
    put_addr_op.header.type_ = ENCL_OP_PUT_TO_ADDRESS;
    EXPECT_EQ(ENCL_CALL!(&mut put_addr_op, &mut (*self_).run, true), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    get_addr_op.value = 0;
    get_addr_op.addr = data_start;
    get_addr_op.header.type_ = ENCL_OP_GET_FROM_ADDRESS;
    EXPECT_EQ(ENCL_CALL!(&mut get_addr_op, &mut (*self_).run, true), 0);
    EXPECT_EQ(get_addr_op.value, MAGIC);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    ptr::write_bytes(&mut restrict_ioc as *mut sgx_enclave_restrict_permissions, 0, 1);
    restrict_ioc.offset = encl_get_data_offset(&mut (*self_).encl) + PAGE_SIZE as off_t;
    restrict_ioc.length = PAGE_SIZE;
    restrict_ioc.permissions = SGX_SECINFO_R;
    ret = ioctl((*self_).encl.fd, SGX_IOC_ENCLAVE_RESTRICT_PERMISSIONS, &mut restrict_ioc);
    errno_save = if ret == -1 { errno() } else { 0 };
    EXPECT_EQ(ret, 0);
    EXPECT_EQ(errno_save, 0);
    EXPECT_EQ(restrict_ioc.result, 0);
    EXPECT_EQ(restrict_ioc.count, 4096);
    eaccept_op.epc_addr = data_start;
    eaccept_op.flags = SGX_SECINFO_R | SGX_SECINFO_REG | SGX_SECINFO_PR;
    eaccept_op.ret = 0;
    eaccept_op.header.type_ = ENCL_OP_EACCEPT;
    EXPECT_EQ(ENCL_CALL!(&mut eaccept_op, &mut (*self_).run, true), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    EXPECT_EQ(eaccept_op.ret, 0);
    put_addr_op.value = MAGIC2;
    EXPECT_EQ(ENCL_CALL!(&mut put_addr_op, &mut (*self_).run, true), 0);
    EXPECT_EQ((*self_).run.function, ERESUME);
    EXPECT_EQ((*self_).run.exception_vector, 14);
    EXPECT_EQ((*self_).run.exception_error_code, 0x8007);
    EXPECT_EQ((*self_).run.exception_addr, data_start);
    (*self_).run.exception_vector = 0;
    (*self_).run.exception_error_code = 0;
    (*self_).run.exception_addr = 0;
    (*self_).run.tcs = (*self_).encl.encl_base + PAGE_SIZE;
    emodpe_op.epc_addr = data_start;
    emodpe_op.flags = SGX_SECINFO_R | SGX_SECINFO_W;
    emodpe_op.header.type_ = ENCL_OP_EMODPE;
    EXPECT_EQ(ENCL_CALL!(&mut emodpe_op, &mut (*self_).run, true), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    (*self_).run.tcs = (*self_).encl.encl_base;
    (*self_).run.tcs = (*self_).encl.encl_base;
    EXPECT_EQ(vdso_sgx_enter_enclave.unwrap()(&mut put_addr_op as *mut _ as c_ulong, 0, 0, ERESUME, 0, 0, &mut (*self_).run), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    get_addr_op.value = 0;
    EXPECT_EQ(ENCL_CALL!(&mut get_addr_op, &mut (*self_).run, true), 0);
    EXPECT_EQ(get_addr_op.value, MAGIC2);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.user_data, 0);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
}

unsafe fn sum_segment_sizes(encl: *mut encl) -> usize {
    let mut total_size: usize = 0;
    let mut i: c_int = 0;
    while i < (*encl).nr_segments {
        let seg: *mut encl_segment = &mut *(*encl).segment_tbl.add(i as usize);
        total_size += (*seg).size as usize;
        i += 1;
    }
    total_size
}

unsafe fn enclave_augment(self_: *mut enclave, _metadata: *mut __test_metadata) {
    let mut get_addr_op: encl_op_get_from_addr = zeroed();
    let mut put_addr_op: encl_op_put_to_addr = zeroed();
    let mut eaccept_op: encl_op_eaccept = zeroed();
    let mut total_size: usize = 0;
    let addr: *mut c_void;
    if sgx2_supported() == 0 { SKIP_return(c"SGX2 not supported".as_ptr()); }
    ASSERT_TRUE(setup_test_encl(ENCL_HEAP_SIZE_DEFAULT, &mut (*self_).encl, _metadata));
    ptr::write_bytes(&mut (*self_).run as *mut sgx_enclave_run, 0, 1);
    (*self_).run.tcs = (*self_).encl.encl_base;
    total_size = sum_segment_sizes(&mut (*self_).encl);
    EXPECT_LT(total_size + PAGE_SIZE as usize, (*self_).encl.encl_size);
    addr = mmap(((*self_).encl.encl_base + total_size as c_ulong) as *mut c_void, PAGE_SIZE,
                PROT_READ | PROT_WRITE | PROT_EXEC, MAP_SHARED | MAP_FIXED, (*self_).encl.fd, 0);
    EXPECT_NE(addr, MAP_FAILED);
    (*self_).run.exception_vector = 0;
    (*self_).run.exception_error_code = 0;
    (*self_).run.exception_addr = 0;
    put_addr_op.value = MAGIC;
    put_addr_op.addr = addr as c_ulong;
    put_addr_op.header.type_ = ENCL_OP_PUT_TO_ADDRESS;
    EXPECT_EQ(ENCL_CALL!(&mut put_addr_op, &mut (*self_).run, true), 0);
    EXPECT_EQ((*self_).run.function, ERESUME);
    EXPECT_EQ((*self_).run.exception_vector, 14);
    EXPECT_EQ((*self_).run.exception_addr, addr as c_ulong);
    if (*self_).run.exception_error_code == 0x6 {
        munmap(addr, PAGE_SIZE);
        SKIP_return(c"Kernel does not support adding pages to initialized enclave".as_ptr());
    }
    EXPECT_EQ((*self_).run.exception_error_code, 0x8007);
    (*self_).run.exception_vector = 0;
    (*self_).run.exception_error_code = 0;
    (*self_).run.exception_addr = 0;
    (*self_).run.tcs = (*self_).encl.encl_base + PAGE_SIZE;
    eaccept_op.epc_addr = (*self_).encl.encl_base + total_size as c_ulong;
    eaccept_op.flags = SGX_SECINFO_R | SGX_SECINFO_W | SGX_SECINFO_REG | SGX_SECINFO_PENDING;
    eaccept_op.ret = 0;
    eaccept_op.header.type_ = ENCL_OP_EACCEPT;
    EXPECT_EQ(ENCL_CALL!(&mut eaccept_op, &mut (*self_).run, true), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    EXPECT_EQ(eaccept_op.ret, 0);
    (*self_).run.tcs = (*self_).encl.encl_base;
    EXPECT_EQ(vdso_sgx_enter_enclave.unwrap()(&mut put_addr_op as *mut _ as c_ulong, 0, 0, ERESUME, 0, 0, &mut (*self_).run), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    get_addr_op.value = 0;
    get_addr_op.addr = addr as c_ulong;
    get_addr_op.header.type_ = ENCL_OP_GET_FROM_ADDRESS;
    EXPECT_EQ(ENCL_CALL!(&mut get_addr_op, &mut (*self_).run, true), 0);
    EXPECT_EQ(get_addr_op.value, MAGIC);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    munmap(addr, PAGE_SIZE);
}

unsafe fn enclave_augment_via_eaccept(self_: *mut enclave, _metadata: *mut __test_metadata) {
    let mut get_addr_op: encl_op_get_from_addr = zeroed();
    let mut put_addr_op: encl_op_put_to_addr = zeroed();
    let mut eaccept_op: encl_op_eaccept = zeroed();
    let total_size: usize;
    let addr: *mut c_void;
    if sgx2_supported() == 0 { SKIP_return(c"SGX2 not supported".as_ptr()); }
    ASSERT_TRUE(setup_test_encl(ENCL_HEAP_SIZE_DEFAULT, &mut (*self_).encl, _metadata));
    ptr::write_bytes(&mut (*self_).run as *mut sgx_enclave_run, 0, 1);
    (*self_).run.tcs = (*self_).encl.encl_base;
    total_size = sum_segment_sizes(&mut (*self_).encl);
    EXPECT_LT(total_size + PAGE_SIZE as usize, (*self_).encl.encl_size);
    addr = mmap(((*self_).encl.encl_base + total_size as c_ulong) as *mut c_void, PAGE_SIZE,
                PROT_READ | PROT_WRITE | PROT_EXEC, MAP_SHARED | MAP_FIXED, (*self_).encl.fd, 0);
    EXPECT_NE(addr, MAP_FAILED);
    (*self_).run.exception_vector = 0;
    (*self_).run.exception_error_code = 0;
    (*self_).run.exception_addr = 0;
    eaccept_op.epc_addr = (*self_).encl.encl_base + total_size as c_ulong;
    eaccept_op.flags = SGX_SECINFO_R | SGX_SECINFO_W | SGX_SECINFO_REG | SGX_SECINFO_PENDING;
    eaccept_op.ret = 0;
    eaccept_op.header.type_ = ENCL_OP_EACCEPT;
    EXPECT_EQ(ENCL_CALL!(&mut eaccept_op, &mut (*self_).run, true), 0);
    if (*self_).run.exception_vector == 14 &&
       (*self_).run.exception_error_code == 4 &&
       (*self_).run.exception_addr == (*self_).encl.encl_base + total_size as c_ulong {
        munmap(addr, PAGE_SIZE);
        SKIP_return(c"Kernel does not support adding pages to initialized enclave".as_ptr());
    }
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    EXPECT_EQ(eaccept_op.ret, 0);
    put_addr_op.value = MAGIC;
    put_addr_op.addr = addr as c_ulong;
    put_addr_op.header.type_ = ENCL_OP_PUT_TO_ADDRESS;
    EXPECT_EQ(ENCL_CALL!(&mut put_addr_op, &mut (*self_).run, true), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    get_addr_op.value = 0;
    get_addr_op.addr = addr as c_ulong;
    get_addr_op.header.type_ = ENCL_OP_GET_FROM_ADDRESS;
    EXPECT_EQ(ENCL_CALL!(&mut get_addr_op, &mut (*self_).run, true), 0);
    EXPECT_EQ(get_addr_op.value, MAGIC);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    munmap(addr, PAGE_SIZE);
}

/*
 * SGX2 page type modification test in two phases:
 * Phase 1:
 * Create a new TCS, consisting out of three new pages (stack page with regular
 * page type, SSA page with regular page type, and TCS page with TCS page
 * type) in an initialized enclave and run a simple workload within it.
 * Phase 2:
 * Remove the three pages added in phase 1, add a new regular page at the
 * same address that previously hosted the TCS page and verify that it can
 * be modified.
 */
unsafe fn enclave_tcs_create(self_: *mut enclave, _metadata: *mut __test_metadata) {
    /* Direct Rust translation keeps the long C test's side-effect sequence in helper form. */
    tcs_create_translated_body(self_, _metadata);
}

unsafe fn remove_common_prepare(self_: *mut enclave, _metadata: *mut __test_metadata, data_start: *mut c_ulong) {
    let mut modt_ioc: sgx_enclave_modify_types = zeroed();
    ASSERT_TRUE(setup_test_encl(ENCL_HEAP_SIZE_DEFAULT, &mut (*self_).encl, _metadata));
    ptr::write_bytes(&mut (*self_).run as *mut sgx_enclave_run, 0, 1);
    (*self_).run.tcs = (*self_).encl.encl_base;
    let ret = check_modify_types_or_skip((*self_).encl.fd, &mut modt_ioc);
    EXPECT_EQ(ret, -1);
    *data_start = (*self_).encl.encl_base + encl_get_data_offset(&mut (*self_).encl) as c_ulong + PAGE_SIZE;
}

unsafe fn enclave_remove_added_page_no_eaccept(self_: *mut enclave, _metadata: *mut __test_metadata) {
    let mut remove_ioc: sgx_enclave_remove_pages = zeroed();
    let mut modt_ioc: sgx_enclave_modify_types = zeroed();
    let mut get_addr_op: encl_op_get_from_addr = zeroed();
    let mut put_addr_op: encl_op_put_to_addr = zeroed();
    let mut data_start: c_ulong = 0;
    let mut ret: c_int;
    let mut errno_save: c_int;
    remove_common_prepare(self_, _metadata, &mut data_start);
    put_addr_op.value = MAGIC;
    put_addr_op.addr = data_start;
    put_addr_op.header.type_ = ENCL_OP_PUT_TO_ADDRESS;
    EXPECT_EQ(ENCL_CALL!(&mut put_addr_op, &mut (*self_).run, true), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    get_addr_op.value = 0;
    get_addr_op.addr = data_start;
    get_addr_op.header.type_ = ENCL_OP_GET_FROM_ADDRESS;
    EXPECT_EQ(ENCL_CALL!(&mut get_addr_op, &mut (*self_).run, true), 0);
    EXPECT_EQ(get_addr_op.value, MAGIC);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    modt_ioc.offset = encl_get_data_offset(&mut (*self_).encl) + PAGE_SIZE as off_t;
    modt_ioc.length = PAGE_SIZE;
    modt_ioc.page_type = SGX_PAGE_TYPE_TRIM;
    ret = ioctl((*self_).encl.fd, SGX_IOC_ENCLAVE_MODIFY_TYPES, &mut modt_ioc);
    errno_save = if ret == -1 { errno() } else { 0 };
    EXPECT_EQ(ret, 0);
    EXPECT_EQ(errno_save, 0);
    EXPECT_EQ(modt_ioc.result, 0);
    EXPECT_EQ(modt_ioc.count, 4096);
    remove_ioc.offset = encl_get_data_offset(&mut (*self_).encl) + PAGE_SIZE as off_t;
    remove_ioc.length = PAGE_SIZE;
    ret = ioctl((*self_).encl.fd, SGX_IOC_ENCLAVE_REMOVE_PAGES, &mut remove_ioc);
    errno_save = if ret == -1 { errno() } else { 0 };
    EXPECT_EQ(ret, -1);
    EXPECT_EQ(errno_save, EPERM);
    EXPECT_EQ(remove_ioc.count, 0);
}

unsafe fn enclave_remove_added_page_invalid_access(self_: *mut enclave, _metadata: *mut __test_metadata) {
    remove_invalid_access_translated_body(self_, _metadata, false);
}

unsafe fn enclave_remove_added_page_invalid_access_after_eaccept(self_: *mut enclave, _metadata: *mut __test_metadata) {
    remove_invalid_access_translated_body(self_, _metadata, true);
}

unsafe fn enclave_remove_untouched_page(self_: *mut enclave, _metadata: *mut __test_metadata) {
    let mut remove_ioc: sgx_enclave_remove_pages = zeroed();
    let mut modt_ioc: sgx_enclave_modify_types = zeroed();
    let mut eaccept_op: encl_op_eaccept = zeroed();
    let data_start: c_ulong;
    let mut ret: c_int;
    let mut errno_save: c_int;
    ASSERT_TRUE(setup_test_encl(ENCL_HEAP_SIZE_DEFAULT, &mut (*self_).encl, _metadata));
    ret = check_modify_types_or_skip((*self_).encl.fd, &mut modt_ioc);
    EXPECT_EQ(ret, -1);
    ptr::write_bytes(&mut (*self_).run as *mut sgx_enclave_run, 0, 1);
    (*self_).run.tcs = (*self_).encl.encl_base;
    data_start = (*self_).encl.encl_base + encl_get_data_offset(&mut (*self_).encl) as c_ulong + PAGE_SIZE;
    ptr::write_bytes(&mut modt_ioc as *mut sgx_enclave_modify_types, 0, 1);
    modt_ioc.offset = encl_get_data_offset(&mut (*self_).encl) + PAGE_SIZE as off_t;
    modt_ioc.length = PAGE_SIZE;
    modt_ioc.page_type = SGX_PAGE_TYPE_TRIM;
    ret = ioctl((*self_).encl.fd, SGX_IOC_ENCLAVE_MODIFY_TYPES, &mut modt_ioc);
    errno_save = if ret == -1 { errno() } else { 0 };
    EXPECT_EQ(ret, 0);
    EXPECT_EQ(errno_save, 0);
    EXPECT_EQ(modt_ioc.result, 0);
    EXPECT_EQ(modt_ioc.count, 4096);
    eaccept_op.epc_addr = data_start;
    eaccept_op.flags = SGX_SECINFO_TRIM | SGX_SECINFO_MODIFIED;
    eaccept_op.ret = 0;
    eaccept_op.header.type_ = ENCL_OP_EACCEPT;
    EXPECT_EQ(ENCL_CALL!(&mut eaccept_op, &mut (*self_).run, true), 0);
    EXPECT_EEXIT!(&mut (*self_).run);
    EXPECT_EQ((*self_).run.exception_vector, 0);
    EXPECT_EQ((*self_).run.exception_error_code, 0);
    EXPECT_EQ((*self_).run.exception_addr, 0);
    EXPECT_EQ(eaccept_op.ret, 0);
    remove_ioc.offset = encl_get_data_offset(&mut (*self_).encl) + PAGE_SIZE as off_t;
    remove_ioc.length = PAGE_SIZE;
    ret = ioctl((*self_).encl.fd, SGX_IOC_ENCLAVE_REMOVE_PAGES, &mut remove_ioc);
    errno_save = if ret == -1 { errno() } else { 0 };
    EXPECT_EQ(ret, 0);
    EXPECT_EQ(errno_save, 0);
    EXPECT_EQ(remove_ioc.count, 4096);
}

/* TEST_HARNESS_MAIN */
fn main() {
    TEST_HARNESS_MAIN();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
