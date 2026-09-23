.. SPDX-License-Identifier: GPL-2.0-only

C to Rust translation integration
================================

The migration objective is to replace the translated C sources throughout the
kernel, build tools, architecture support, and tests with working Rust, with
equivalent build, runtime, and test behavior. This objective is not yet complete.
The presence of an adjacent ``.rs`` file does not mean that its definitions are
complete or that Kbuild uses it.

Host tools
----------

The following normal Kbuild targets now select Rust sources:

* ``scripts/basic/fixdep``
* ``scripts/kallsyms``
* ``scripts/tracepoint-update``
* ``scripts/gen_packed_field_checks``
* ``scripts/asn1_compiler``
* ``scripts/unifdef``
* ``scripts/sorttable``
* ``scripts/recordmcount``
* ``scripts/insert-sys-cert``
* ``scripts/sign-file``
* ``certs/extract-cert``
* ``scripts/mod/mk_elfconfig``
* ``scripts/mod/modpost``
* ``scripts/selinux/mdp/mdp``
* ``scripts/kconfig/conf``
* ``scripts/genksyms/genksyms``
* ``scripts/dtc/dtc``
* ``scripts/dtc/fdtoverlay``
* ``scripts/dtc/fdtget`` and ``scripts/dtc/fdtput`` (explicit host targets)
* ``scripts/ipe/polgen/polgen``
* ``arch/x86/tools/vdso2c``
* ``arch/x86/tools/relocs``
* ``arch/x86/boot/compressed/mkpiggy``
* ``arch/x86/boot/mkcpustr``
* ``arch/powerpc/boot/addnote``
* ``arch/powerpc/boot/hack-coff``
* ``arch/powerpc/boot/mktree``
* ``arch/arm/vdso/vdsomunge`` (also used by ARM64's 32-bit compatibility vDSO)
* ``arch/arm64/kernel/pi/relacheck``
* ``arch/arm64/kvm/hyp/nvhe/gen-hyprel``
* ``arch/alpha/boot/tools/mkbb``
* ``arch/alpha/boot/tools/objstrip``
* ``arch/mips/tools/elf-entry``
* ``arch/mips/boot/elf2ecoff`` (also used by compressed boot)
* ``arch/mips/boot/compressed/calc_vmlinuz_load_addr``
* ``arch/mips/vdso/genvdso``
* ``arch/s390/tools/gen_facilities``
* ``arch/s390/tools/gen_opcode_table``
* ``usr/gen_init_cpio``

These tools use the Rust standard library without third-party Rust crates or
unresolved declarations for functions that were C macros or inline helpers.
The signing and certificate-extraction tools additionally link the existing
OpenSSL ``libcrypto`` host library through an audited, opaque-handle ownership
boundary. Their common Rust module replaces the translated C stdio and macro
declarations. A small Python helper probes the installed OpenSSL headers using
the host C compiler to select the correct library ABI; it compiles no C shim.
The host OpenSSL development headers/library and Python 3 are therefore needed
when building these tools, including the combined ``rust-host-tools`` target.
The tracepoint checker and table sorter import ``scripts/elf-parse.rs`` as a
checked Rust module. Kbuild tracks that module through rustc's dependency output.
The ASN.1 compiler imports translated tag definitions, and the SELinux policy
generator imports shared immutable class, permission, SID, and capability
tables. Its filesystem configuration flags preserve the distinction between
built-in and modular features.

The module postprocessor uses owned module/export records and the shared checked
ELF reader. Its source-version digest, all 57 device-table alias families,
namespaces, symbol versions, export trimming, section-reference diagnostics,
and generated files are tested against C, including ELF32/ELF64 and both byte
orders. Normal Kbuild embeds the target compiler's generated device-table
offsets; the target C structure definitions and their layout emitter remain
part of this mixed build. The no-config ``rust-host-tools`` executable does not
embed target-specific offsets. Set ``MODPOST_DEVICETABLE_OFFSETS`` to an existing
generated ``scripts/mod/devicetable-offsets.h`` when using that executable on
device tables; missing metadata is an error, not silently omitted aliases.
Command-line paths, symbol dumps, and ELF text metadata currently require UTF-8;
source-version dependency filenames are byte-preserving. Malformed ELF extents,
links, symbol indices, and device-table data are checked before access.
To compare complete generated files against C using an existing kernel and
its modules, including saved CRCs when ``CONFIG_MODVERSIONS=y``::

    MODPOST_KERNEL_BUILD=/tmp/lupos-build \
        python3 -m unittest discover -s scripts/tests -p test_modpost.py

The IPE boot-policy generator preserves every input byte and the original C
string escaping, including binary policy data and non-UTF-8 pathnames. Tests
compile the generated C source and verify its embedded bytes. Buffered write
failures are reported instead of silently producing incomplete policy source.

The Alpha boot helpers decode ELF and ECOFF files and construct SRM boot blocks
using explicit target-endian fields. They preserve disk labels, checksums,
entry-point offsets, BSS padding, primary-loader block counts, and non-UTF-8
filenames. Differential tests cover the original C output and diagnostics,
including sparse primary images larger than 32 bits. Malformed header extents
and arithmetic are checked, and partial writes advance by the bytes written.
These checks do not substitute for an Alpha target-kernel build or boot.

Build the migrated tools without creating a kernel configuration::

    make O=/tmp/lupos-build rust-host-tools

Build and run the regression suite::

    make O=/tmp/lupos-build rust-host-tests

The suite compiles the unchanged C implementations as test references and
compares their output, diagnostics, and exit codes against Rust. It also checks
Kbuild source selection, incremental rebuilds, and imported-module
dependencies. This requires a host C compiler/linker, ``HOSTRUSTC``, and
Python 3. Kconfig, genksyms, and DTC differential tests additionally require Flex
and Bison to build the original C references; the Rust tools themselves need
neither. Tests use temporary directories. Normal builds use the Rust tools;
the C implementations remain available as behavioral references.

The device-tree compiler uses owned tree nodes, ordered properties and labels,
byte-valued data with reference markers, and checked blob input/output. Its full
source grammar includes overlays, includes, binary includes, deletion, labels,
and unsigned integer expressions. All registered checks, phandle allocation,
symbol/fixup generation, filesystem input, source annotations, blob versions
1/2/3/16/17, DTS, and assembly output are tested against C. As with the existing
Kbuild configuration, YAML output is not enabled. Source parsing and tree
serialization use explicit stacks, including deep nodes and expressions.
The parser no longer inherits Bison's fixed parser-stack allocation limit;
buffered output failures and malformed blob bounds are reported safely.
Differential fixtures include real ARM, ARM64, RISC-V, and PowerPC boards and
the kernel's OF unit-test overlays.

The device-tree overlay, query, and editing tools share safe byte-slice libfdt
modules. Header/tag traversal, old-format property alignment, aliases, phandles,
reservation maps, string lists, in-place mutation, sequential construction,
packing/reordering, and overlay fixups and symbol propagation are compared
against C. Tests compare entire buffers, including both damaged blobs after
an overlay failure, and apply the kernel's OF overlay fixtures and chained
compiled DTS overlays. Names, values, and filesystem paths retain their bytes.
Overlay traversal uses explicit stacks. Malformed local-fixup offsets and
sequential-write headers are bounds checked, and truncated input is detected
using actual bytes read rather than allocation capacity. The legacy unbuilt
query/edit applets had undefined behavior for 16-bit writes, invalid integers,
and raw formatting: Rust writes complete big-endian halfwords, diagnoses invalid
numbers, and implements raw byte values without invalid format strings.

The tracepoint checker additionally rejects truncated ELF data and checks
multiple files without retaining section pointers from a previous file.
Its cross-target cases cover ELF32, ELF64, both byte orders, and extended
section numbering.

The table sorter supports all exception-table architectures from the C tool,
both ELF classes and byte orders, extended symbol indices, x86 and LoongArch
ORC layouts, and address- or RELA-based mcount tables. Kbuild passes the ORC
and mcount configuration to rustc. Differential tests compare the complete
modified ELF files, including function-list filtering and relative fixups.
The Rust sorter validates all edits before writing, processes the final symbol
table entry, and keeps architecture state separate for each input file.

The ftrace call-site recorder implements the original architecture-specific
relocation filters, section-base selection, MIPS call pairs, and x86, ARM,
Thumb, and AArch64 instruction replacements. Tests compare output files for
every supported architecture, both ELF classes and byte orders, extended
indices, and real ``-pg`` compiler objects that are linked after rewriting.
It supports byte-preserving paths and section names. ELF accesses are checked,
failed edits are not committed, and architecture state is reset for each file.
Big-endian extended indices are decoded correctly instead of following the
original unchecked host-endian reads. New-section alignment bytes are zeroed;
the C implementation leaves those padding bytes uninitialized. Mixed REL/RELA
call-site tables are rejected instead of emitting inconsistent entry sizes.
An optional check recompiles a saved kernel compilation unit with profiling
into a temporary directory, leaving the kernel output tree unchanged::

    RECORDMCOUNT_KERNEL_BUILD=/tmp/lupos-build \
    RECORDMCOUNT_KERNEL_OBJECT=lib/test_hexdump.o \
        python3 -m unittest discover -s scripts/tests -p test_recordmcount.py

The x86 vDSO image generator uses the same checked ELF reader for both 32-bit
and 64-bit inputs. Its generated C image data and metadata match the original
tool; malformed input is rejected before opening the output, and write errors
are checked. Generated target-kernel C image data remains part of the mixed
build until those consumers are migrated.

The x86 relocation extractor shares the checked ELF reader and preserves
kernel and real-mode symbol classifications, relocation filtering, duplicate
offsets, sorted binary tables, assembler output, and the absolute-symbol and
relocation-inspection modes. Tests cover both ELF classes, extended indices,
all relocation types, and over 100,000 randomized relocations per class.
Additional comparisons against retained kernel relocations can be run with::

    RELOCS_TEST_VMLINUX=/tmp/lupos-build/vmlinux.unstripped \
    RELOCS_TEST_REALMODE=/tmp/lupos-build/arch/x86/realmode/rm/realmode.elf \
        python3 -m unittest discover -s scripts/tests -p test_relocs.py

Malformed section links, string references, and table strides are rejected
with checked errors rather than accessing memory outside the ELF image.

The compressed x86 image wrapper is also generated by Rust, preserving the
embedded file name, compressed length, and little-endian uncompressed-size
footer. Its tests assemble the wrapper and check the embedded bytes, including
non-UTF-8 file names and sparse input files larger than 4 GiB.

The extra-certificate insertion tool shares the checked ELF reader and supports
both symbol tables and a supplied ``System.map``. It preserves native-class
and native-endian restrictions, original inode and permissions, replacement
accounting, reserve zeroing, and no-op behavior. Tests compare complete modified
images with C, including real linked ELF input, and check that malformed images
are rejected before writing. Binary certificate comparison and exact map-symbol
matching intentionally fix the C tool's NUL-terminated and substring comparisons.
An optional comparison modifies only temporary copies of a real kernel and
its stripped image (with ``CONFIG_SYSTEM_EXTRA_CERTIFICATE=y``)::

    INSERT_CERT_KERNEL_BUILD=/tmp/lupos-build \
        python3 -m unittest discover -s scripts/tests -p test_insert_sys_cert.py

Certificate extraction preserves PEM bundle ordering, DER output, verbose
subject descriptions, empty key lists, and byte-preserving paths. Tests compare
outputs and errors against C and check partial-output behavior after a malformed
later certificate. Explicit BIO flushing reports buffered output failures that
the original ignored. Signing tests compare deterministic RSA CMS signatures
and module trailers, independently verify RSA and EC signatures, and cover
encrypted keys, raw signatures, detached signatures, and certificate formats.
The shared wrapper implements provider and legacy-engine PKCS#11 paths; real
hardware-token success and older OpenSSL library versions still require
environment-specific validation. The installed OpenSSL 3 library and missing
provider failures are covered by the automated tests.

The x86 CPU-string generator imports the translated feature-bit definitions
and immutable name table from ``cpufeatures_header.rs``. Tests verify every
translated feature bit, compare the generated header against C for both
kernel widths, and compile the header under multiple required-feature masks.

The PowerPC Open Firmware helpers preserve ELF32/ELF64 notes in both byte
orders, XCOFF optional-header section indices, and tree-boot image headers,
checksums, and sector padding. Differential tests compare complete modified
images and errors with C, including non-UTF-8 paths and 65,535 XCOFF sections.
The existing ELF32 RPA note-size field, host-endian tree-image checksum, and
retained final-sector bytes are deliberately preserved. Malformed overflowing
ELF offsets and oversized XCOFF optional headers are rejected with checked
errors instead of accessing memory outside their buffers. These host tools
are validated through Kbuild; a PowerPC target-kernel boot is not yet validated.

The ARM vDSO helper preserves EABI validation, soft-float flag clearing,
output permissions, and failed-output cleanup. The ARM64 early-relocation
checker uses the shared checked ELF reader, preserves data-section filtering,
converts opted-in ABS64 entries to PREL64, and removes rejected outputs.
Tests compare complete images, diagnostics, and link behavior with C, including
100,001 randomized relocations per byte order. When Clang and LLD are available,
the suite also builds real ARM and AArch64 fixtures in both byte orders.
Truncated headers, invalid encodings, and out-of-bounds section references are
rejected safely; extended ELF section numbering is supported. Kbuild tests
exercise both the ARM helper rule and ARM64's borrowed helper path. ARM and
ARM64 target-kernel boots are not yet validated.

The ARM64 hypervisor relocation generator uses the same checked ELF reader,
with byte order selected from the target kernel configuration rather than the
host. It preserves generated assembly, section filtering, relocation ordering,
and partial output on errors. Tests cover every accepted relocation type,
rejected types, both byte orders, raw-byte paths and section names, and real
AArch64 assembler objects. Malformed extents and indices are rejected safely;
extended section numbering is supported. Assertion diagnostics retain the
original decoded values and source locations but use portable expressions
instead of host-libc macro expansions. Delayed output errors are reported.
Set ``HYPREL_KERNEL_BUILD`` to an ARM64 build containing
``arch/arm64/kvm/hyp/nvhe/kvm_nvhe.tmp.o`` to compare an actual hypervisor object
against the unchanged C tool and Kbuild's saved ``hyp-reloc.S`` output.
The normal little-endian ARM64 hypervisor-object build is verified. This tree
marks big-endian ARM64 kernels ``BROKEN``; big-endian coverage is limited to
tool fixtures and explicit host-rule configuration, not a target-kernel build.

The MIPS boot helpers preserve ELF32/ELF64 entry-address sign extension in both
byte orders, compressed-load address rounding and hexadecimal argument parsing,
and complete ELF32-to-ECOFF conversion. ECOFF coverage includes all header fields,
the optional six-section form, segment ordering and gaps, PROM padding, retained
existing-file tails and permissions, and partial output on errors. Tests compare
randomized images and real big- and little-endian MIPS compiler/linker output.
Malformed ELF magic, class, and encoding are rejected before opening an ECOFF
output instead of interpreting arbitrary bytes as an ELF32 header. Raw-byte
paths and checked output errors are supported. Normal and compressed-boot Kbuild
host rules are verified; a MIPS target-kernel boot is not yet validated.

The MIPS vDSO generator repairs ABI sections in both debug and stripped ELF
images and emits byte-identical C image descriptions for O32, N32, and N64 in
both byte orders. It preserves in-place inode/permission behavior, ordered
partial repairs on semantic errors, and required-symbol output cleanup.
Checked shared ELF access replaces native-layout casts and unresolved helper
declarations. Extended section numbering and large symbol tables are supported;
input/output inode aliases and mismatched image formats are rejected before
modification, and output failures are reported. Tests compile and execute the
generated C to compare its embedded bytes. Normal 32-bit little-endian Malta
and 64-bit big-endian Malta vDSO builds, including both compatibility ABIs,
are verified through target compilation. To compare their raw inputs, repaired
images and generated source against C, set ``MIPS_VDSO_KERNEL_BUILD`` to the
corresponding build directory when running ``test_mips_genvdso.py``.

The s390 facility and disassembler generators preserve all three facility masks,
configuration-dependent architecture bits, instruction formats, long names,
opcode grouping, and table ordering. Differential tests cover all 64 feature
combinations and the complete checked-in opcode table, and compile the generated
disassembler header as C. Normal ``kapi`` rules generate byte-identical headers,
rebuild on configuration changes, and preserve unchanged header timestamps.
Opcode input is byte-preserving, including ASCII case conversion and whitespace.
Overlong fields and incomplete multi-byte opcode fragments are rejected instead
of overflowing C's fixed arrays or reading uninitialized data; input/output
errors are reported. An s390 target-kernel boot is not yet validated.

The line-oriented Kconfig front end uses owned Rust modules for preprocessing,
lexing, parsing, expression evaluation, menus, symbols, and configuration I/O.
It implements the existing configuration modes, preserves choice priority,
tristate dependencies, select/imply behavior, configuration backups, and
generated-header and dependency-file ordering. Full-tree x86 tests compare
complete output files and diagnostics with the original C implementation.
The existing Kconfig tests can also be run with::

    make O=/tmp/lupos-build testconfig

That target requires Python's ``pytest`` package. The interactive graphical
and terminal front ends (``menuconfig``, ``nconfig``, ``xconfig``, and
``gconfig``) still use the C Kconfig core and are not yet migrated.
The Rust Kconfig model currently requires UTF-8 source and configuration
strings. Non-UTF-8 configuration input is rejected without replacing the
existing file; byte-preserving support remains outstanding. Output write
errors and failed configuration-backup renames are also reported instead of
being silently ignored.

The symbol-version generator uses the complete declaration grammar with
checked-in LR tables, stable token-arena indices, and owned symbol definitions.
It preserves typedef and compound expansion, enum values, CRCs, source-file
boundaries, reference modversions, type dumps, and parser/lexer diagnostics.
Tests compare real exported kernel declarations, generated declarations,
malformed-input recovery, deep type graphs, and binary input bytes with C.
The table-maintenance helper ``scripts/genksyms/gen_parser_tables.py`` emits
an ``apply_patch`` patch; it is not needed during normal builds. Grammar changes
require regenerating the tables and reviewing the corresponding Rust actions.
Malformed reference-file read failures are reported instead of reproducing
the C implementation's nonterminating read-error loop.
After completing a kernel build with ``CONFIG_MODVERSIONS=y``, the optional
integration check compares saved compilation-unit CRCs and ``Module.symvers``
against the source-built C reference::

    GENKSYMS_KERNEL_BUILD=/tmp/lupos-build \
        python3 -m unittest discover -s scripts/tests -p test_genksyms.py

Kernel pipeline validation
--------------------------

To exercise the migrated generators through an x86-64 kernel build, header
installation, and an emulated boot::

    make O=/tmp/lupos-build defconfig
    make O=/tmp/lupos-build -j8 bzImage
    make O=/tmp/lupos-build headers_install INSTALL_HDR_PATH=/tmp/lupos-headers
    make O=/tmp/lupos-build rust-boot-test

The boot test requires QEMU's x86 system emulator and firmware. Set ``QEMU``
to an alternate emulator command and ``QEMU_DATA`` to an alternate firmware
directory if necessary. It uses software emulation without networking or
root privileges. It builds a static Rust PID 1, packs an initramfs with the
migrated generator, and boots the image. PID 1 verifies binary file contents,
hardlinks, symlinks, ownership, and permissions before printing a success
marker. The runner then stops QEMU. Artifacts and the console log are kept in
``rust-boot-test/`` under the kernel output directory.

The ownership checks require ``CONFIG_MULTIUSER=y``. The runner also accepts
``--module /path/to/module.ko`` to load a module inside the isolated VM and
repeatable ``--reject-module /path/to/unsigned.ko`` arguments to check signature
enforcement. Rejection tests require ``CONFIG_MODULE_SIG_FORCE=y`` and expect
``EKEYREJECTED`` before attempting the valid load. For a signing-pipeline test,
enable ``CONFIG_TEST_HEXDUMP=m``, ``CONFIG_MODULE_SIG_ALL=y`` and signature
enforcement, install modules into a temporary ``INSTALL_MOD_PATH``, pass the
installed signed ``test_hexdump.ko`` as ``--module``, and pass the original
unsigned build artifact as ``--reject-module``. This exercises generated symbol
versions, certificate extraction, signing, kernel verification, and the loaded
module's own hexdump tests without loading anything into the host kernel.

This currently validates a mixed C/Rust kernel build; it does not establish
that the target-kernel translation is complete.

Remaining integration
---------------------

Most of the migration is still outstanding. In particular:

* The remaining Kconfig front ends, device-tree tools,
  architecture tools, and other host utilities still need complete Rust
  implementations and corresponding build changes.
* Target-kernel C objects still take precedence over adjacent Rust files.
  Their Rust definitions, shared types, configuration handling, exported
  symbols, and module boundaries must be repaired before selecting them.
* Some translated files omit core behavior. For example,
  ``init/main.rs::start_kernel`` currently panics instead of initializing
  and starting the kernel.
* Architecture build files and translated selftests need integration and
  behavioral verification.
* A successful mixed C/Rust build validates the migrated components only.
  Full Rust kernel linking, booting, and the applicable kernel/selftest
  suites remain required before declaring the translation complete.
