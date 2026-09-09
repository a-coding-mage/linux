// SPDX-License-Identifier: GPL-2.0-only
// Dependency supplied externally: <kunit/test.h>

unsafe fn total_mapping_size(phdr: *const elf_phdr, count: usize) -> usize;

unsafe fn total_mapping_size_test(test: *mut kunit) {
    let empty = [
        elf_phdr { p_type: PT_LOAD, p_vaddr: 0, p_memsz: 0 },
        elf_phdr { p_type: PT_INTERP, p_vaddr: 10, p_memsz: 999999 },
    ];
    /*
     * readelf -lW /bin/mount | grep '^  .*0x0' | awk '{print "\t\t{ .p_type = PT_" \
     *                         $1 ", .p_vaddr = " $3 ", .p_memsz = " $6 ", },"}'
     */
    let mount = [
        elf_phdr { p_type: PT_PHDR, p_vaddr: 0x00000040, p_memsz: 0x0002d8 },
        elf_phdr { p_type: PT_INTERP, p_vaddr: 0x00000318, p_memsz: 0x00001c },
        elf_phdr { p_type: PT_LOAD, p_vaddr: 0x00000000, p_memsz: 0x0033a8 },
        elf_phdr { p_type: PT_LOAD, p_vaddr: 0x00004000, p_memsz: 0x005c91 },
        elf_phdr { p_type: PT_LOAD, p_vaddr: 0x0000a000, p_memsz: 0x0022f8 },
        elf_phdr { p_type: PT_LOAD, p_vaddr: 0x0000d330, p_memsz: 0x000d40 },
        elf_phdr { p_type: PT_DYNAMIC, p_vaddr: 0x0000d928, p_memsz: 0x000200 },
        elf_phdr { p_type: PT_NOTE, p_vaddr: 0x00000338, p_memsz: 0x000030 },
        elf_phdr { p_type: PT_NOTE, p_vaddr: 0x00000368, p_memsz: 0x000044 },
        elf_phdr { p_type: PT_GNU_PROPERTY, p_vaddr: 0x00000338, p_memsz: 0x000030 },
        elf_phdr { p_type: PT_GNU_EH_FRAME, p_vaddr: 0x0000b490, p_memsz: 0x0001ec },
        elf_phdr { p_type: PT_GNU_STACK, p_vaddr: 0x00000000, p_memsz: 0x000000 },
        elf_phdr { p_type: PT_GNU_RELRO, p_vaddr: 0x0000d330, p_memsz: 0x000cd0 },
    ];
    let mount_size: usize = 0xE070;
    /* https://lore.kernel.org/linux-fsdevel/YfF18Dy85mCntXrx@fractal.localdomain */
    let unordered = [
        elf_phdr { p_type: PT_LOAD, p_vaddr: 0x00000000, p_memsz: 0x0033a8 },
        elf_phdr { p_type: PT_LOAD, p_vaddr: 0x0000d330, p_memsz: 0x000d40 },
        elf_phdr { p_type: PT_LOAD, p_vaddr: 0x00004000, p_memsz: 0x005c91 },
        elf_phdr { p_type: PT_LOAD, p_vaddr: 0x0000a000, p_memsz: 0x0022f8 },
    ];

    KUNIT_EXPECT_EQ!(test, total_mapping_size(core::ptr::null(), 0), 0);
    KUNIT_EXPECT_EQ!(test, total_mapping_size(empty.as_ptr(), 0), 0);
    KUNIT_EXPECT_EQ!(test, total_mapping_size(empty.as_ptr(), 1), 0);
    KUNIT_EXPECT_EQ!(test, total_mapping_size(unsafe { empty.as_ptr().add(1) }, 1), 0);
    KUNIT_EXPECT_EQ!(test, total_mapping_size(empty.as_ptr(), 2), 0);
    KUNIT_EXPECT_EQ!(test, total_mapping_size(mount.as_ptr(), mount.len()), mount_size);
    KUNIT_EXPECT_EQ!(test, total_mapping_size(unordered.as_ptr(), unordered.len()), mount_size);
}

static mut binfmt_elf_test_cases: [kunit_case; 2] = [
    KUNIT_CASE!(total_mapping_size_test),
    kunit_case { ..unsafe { core::mem::zeroed() } },
];

static mut binfmt_elf_test_suite: kunit_suite = kunit_suite {
    name: KBUILD_MODNAME,
    test_cases: binfmt_elf_test_cases.as_mut_ptr(),
};

kunit_test_suite!(binfmt_elf_test_suite);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
