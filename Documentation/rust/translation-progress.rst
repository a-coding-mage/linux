.. SPDX-License-Identifier: GPL-2.0-only

C to Rust translation integration
================================

The migration objective is to replace the translated C sources throughout the
kernel, build tools, architecture support, and tests with working Rust, with
equivalent build, runtime, and test behavior. This objective is not yet complete.
The presence of an adjacent ``.rs`` file does not mean that its definitions are
complete or that Kbuild uses it.

Translated files retain their original ``SOURCE-COMMIT`` comments so the C
revision used for translation remains identifiable. Refactoring or integrating
a translation must not delete that provenance marker. Update it only when the
translation has actually been reconciled with a newer C source revision.
The provenance regression test records each migrated file's exact expected
revision in ``scripts/tests/translated_sources.txt``. An intentional revision
update must update that manifest as well; removing or replacing a marker
accidentally fails the test.

Host tools
----------

Migrated host tools default to Rust. Both implementations remain selectable::

    make O=/tmp/lupos-build HOST_TOOLS_LANG=rust defconfig
    make O=/tmp/lupos-build HOST_TOOLS_LANG=rust -j8

    make O=/tmp/lupos-build HOST_TOOLS_LANG=c defconfig
    make O=/tmp/lupos-build HOST_TOOLS_LANG=c -j8

Pass the same choice to each build invocation. Kbuild records the selected
compiler command and rebuilds affected tools when switching languages, so a
separate output directory is optional. The C branches retain their original
source files, object lists, flags and generated-header dependencies. This
selection applies to migrated host tools, not the target kernel's
``CONFIG_RUST`` setting or tools which were originally Rust-only.
``rust-host-tools`` and ``rust-host-tests`` require ``HOST_TOOLS_LANG=rust``;
the retained C implementations are available through normal Kbuild targets.

The following normal Kbuild targets have Rust implementations:

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
* ``scripts/gendwarfksyms/gendwarfksyms``
* ``scripts/dtc/dtc``
* ``scripts/dtc/fdtoverlay``
* ``scripts/dtc/fdtget`` and ``scripts/dtc/fdtput`` (explicit host targets)
* ``scripts/ipe/polgen/polgen``
* ``arch/x86/tools/vdso2c``
* ``arch/x86/tools/relocs``
* ``arch/x86/tools/insn_decoder_test`` and ``arch/x86/tools/insn_sanity``
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
* ``arch/mips/tools/loongson3-llsc-check``
* ``arch/mips/boot/elf2ecoff`` (also used by compressed boot)
* ``arch/mips/boot/tools/relocs``
* ``arch/mips/boot/compressed/calc_vmlinuz_load_addr``
* ``arch/mips/vdso/genvdso``
* ``arch/s390/tools/gen_facilities``
* ``arch/s390/tools/gen_opcode_table``
* ``arch/s390/tools/relocs``
* ``arch/sparc/boot/piggyback``
* ``arch/sparc/vdso/vdso2c``
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
The DWARF symbol-version generator links the existing elfutils ``libdw``
library, which also requires libelf and zlib. The combined host-tools target
therefore needs these host libraries as well. Its C-reference tests additionally
need the elfutils development headers; no C shim or generated Rust binding is
compiled for the Rust implementation.
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
the C implementations remain selectable build implementations and behavioral
references.

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

The x86 instruction-test tools share one allocation-free, core-only Rust decoder
and immutable attribute tables with the translated architecture sources. Their
tools-side modules import that canonical implementation. Byte slices and checked
cursors replace instruction pointers; C union aliases share one stored field.
Tests compare every intermediate field and failure state with C, including
legacy, REX/REX2, VEX/EVEX/XOP, addressing and immediate forms, truncation, retries,
and manually selected operand widths. Real kernel disassembly and seeded random
streams also match. Target-kernel selection is independent: C remains the
default, with the opt-in ``CONFIG_RUST_X86_INSN`` integration described below.

``arch/x86/tools/gen_inat_tables.py`` emits an ``apply_patch`` patch for updating
the checked-in Rust tables from the original opcode map. Its ``--check`` mode
verifies reproducibility. A compile-time map fingerprint rejects stale tables,
and rustc tracks the map as a dependency; normal Rust builds run no C table
generator. The original AWK/C path remains available with ``HOST_TOOLS_LANG=c``.
CLI tests preserve option parsing, field dumps, raw-byte input, and glibc's
seeded random sequence. Excess disassembly bytes are rejected instead of
overflowing a stack buffer, the first short sanity-input tail is initialized
instead of reading uninitialized bytes, and buffered output errors are reported.
After a configured x86 kernel build, the normal decoder post-test can be enabled
with ``CONFIG_X86_DECODER_SELFTEST=y``. An optional C/Rust comparison of its
complete disassembly stream is available with::

    INSN_KERNEL_BUILD=/tmp/lupos-build \
        python3 -m unittest discover -s scripts/tests -p test_insn_decoder.py

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

The Loongson LL/SC checker preserves executable-section filtering, instruction
classification, synchronization checks, branch displacements, the original
loop-boundary rule, and diagnostic ordering. Its byte-oriented ELF access also
handles empty sections and extended numbering safely and checks branch targets
before reading. Tests compare all opcode families, randomized streams, large
sections, and actual MIPS64 little-endian assembler objects with C; malformed
headers and output failures are reported without unchecked memory access.

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

The MIPS relocation processor uses checked ELF32/ELF64 access in both byte
orders, including MIPS64's mixed-width relocation records. It preserves
relocation order, symbol and section filtering, offset encoding, all output
modes, and the original in-place section-size updates, including their effect
with ``--text`` and ``--bin``. Differential tests compare complete modified
images, all relocation types and flag combinations, partial table records,
large relocation streams, and real compiler/linker output. Invalid extents,
links, indices and overlapping writes are rejected safely. A relocation table
overflow is diagnosed before modifying the file, unlike C's destructive
write-before-check behavior; capacity uses an unsigned byte count instead of
C's signed ``int`` truncation, and output failures are also reported. The
original C implementation remains available with ``HOST_TOOLS_LANG=c``.
The normal 32-bit little-endian and 64-bit big-endian Malta kernel builds are
verified through final linking and relocation insertion. All 16 command-line
flag combinations also match C on copies of both linked kernels. Set
``MIPS_RELOCS_KERNEL_BUILD`` to a
build directory retaining ``.tmp_vmlinux2`` (or ``MIPS_RELOCS_VMLINUX`` to an
unprocessed linked image) to run that comparison. MIPS kernel boot validation
is still outstanding.

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

The s390 relocation extractor uses checked ELF64 big-endian access, including
extended section and symbol indices. It preserves allocated-section filtering,
all supported relocation types, KCFI absolute-symbol exceptions, sorted 32-bit
offsets, duplicates, ignored entry-size fields, and original diagnostics.
Differential tests cover over 100,000 relocations, more than 65,536 sections,
randomized images, and a real s390 compiler/linker executable. Invalid table
links, symbol indices, and unterminated strings are rejected safely; delayed
output failures are reported instead of being ignored. The retained C tool
remains selectable with ``HOST_TOOLS_LANG=c``.

The SPARC boot helper preserves a.out header updates, 32-bit and 64-bit page
alignment, fixed-column System.map parsing, and in-place ramdisk appending,
including existing trailing bytes and permissions. It rejects input inode
aliases that would endlessly append a file to itself and reports ramdisk read
errors. The SPARC vDSO converter preserves big-endian ELF32/ELF64 load and dynamic
table validation and byte-identical raw or generated-C outputs. Checked offsets
replace unchecked pointers; invalid input preserves an existing destination,
and buffered output failures are reported. Tests cover randomized images, raw
byte filenames, a real linked SPARC64 vDSO, and compilation and execution of
generated C to verify embedded bytes and padding. Both host rules pass C/Rust/C
selection tests. A full SPARC target build and boot are not yet validated: the
available Clang rejects the tree's existing ``-Wa,--undeclared-regs`` option
before compiling the kernel vDSO inputs.

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

The DWARF symbol-version generator uses owned symbol and type records,
byte-preserving names, checked ELF metadata, and explicit traversal stacks.
Its type expansion, CRC calculation, symbol aliases, export-pointer fallback,
symtypes output, and kABI stability rules are compared with the original C
implementation. Fixtures include GCC, Clang and Rust debugging information,
DWARF versions 2 through 5, both ELF classes and byte orders, compressed and
separate debug information, and linked compilation units. Anonymous type
cycles and malformed ELF metadata are rejected without unchecked accesses.
Buffered output failures are reported instead of silently succeeding.

Only the public elfutils DWARF APIs cross an unsafe boundary. Library handles,
file descriptors, DIEs, and borrowed strings have checked ownership and
lifetimes. Regression tests verify the C ABI layouts and function signatures
against installed headers, reject escaping or cross-thread handles at compile
time, and check descriptor cleanup. These checks do not replace the external
library's own input validation. Run this family's focused tests with::

    python3 -m unittest discover -s scripts/tests -p 'test_gendwarf*.py'

For development headers extracted outside the system, set ``GENDWARF_PREFIX``
to their ``usr`` directory. ``GENDWARF_CFLAGS`` and ``GENDWARF_LIBS`` can override
the differential tests' compiler and linker flags. Tests do not download or
install dependencies, and explicitly skip C comparisons when the required
development headers are absent.

An x86-64 kernel with ``CONFIG_GENDWARFKSYMS=y``, ``CONFIG_MODVERSIONS=y``
and DWARF 5 debugging information is verified through the normal build,
emulated boot, and module loading. Its saved compilation-unit CRCs, symtypes,
``Module.symvers``, linked kernel images, and an external module are compared
against C. For a completed build made with ``KBUILD_SYMTYPES=1``, run the
read-only integration audit with::

    python3 scripts/tests/check_gendwarf_kernel.py /tmp/lupos-build \
        --module /tmp/lupos-external-module

The optional ``--module`` argument may be repeated or omitted. The checker
writes its reference tools and comparison outputs only in temporary directories.
Native Rust compilation units use their actual Kbuild export ordering and
filtering, including the C-built Rust helpers. A native Rust kernel with the
translated hexadecimal helpers has exact per-unit C/Rust/Kbuild parity for
423 compilation units and 6,664 CRC records, including their symtypes files.
The complete ``vmlinux.o`` and final ``vmlinux`` also match C for all 6,664
exports, diagnostics and symtypes.

The combined native build with Rust BCD, character classification, hexadecimal
helpers and x86 instruction decoding also passes the same read-only audit:
439 compilation units and 6,834 CRC records match C/Rust/Kbuild, including
every symtypes file. Both complete kernel images match the C version tool for
all 6,834 exported symbols, diagnostics and symtypes.

Translated target-kernel code
-----------------------------

``CONFIG_RUST_INT_MATH=y`` selects translated integer exponentiation and square
roots. It requires native Rust support, defaults to disabled and is independent
of ``HOST_TOOLS_LANG``. The Makefile retains ``int_pow.o`` and ``int_sqrt.o`` for
the C choice, or selects ``int_math_rust.o`` and export-only metadata for Rust.
``int_pow`` remains GPL-only; the square-root exports retain their original
license classification. As in C, ``int_sqrt64`` is an exported symbol only on
32-bit kernels; 64-bit C callers retain the original inline helper.

Independent Rust consumers import ``kernel::math``. Its pure, safe ``const``
functions share the translated algorithms without defining duplicate C exports.
``int_pow`` preserves wrapping 64-bit multiplication, including ``0**0 == 1``.
``int_sqrt`` uses the native word width, ``int_sqrt32`` offers an explicit 32-bit
input, and ``int_sqrt64`` consumes all 64 input bits on every architecture.
Core integer operations replace the translation's fictitious external bit-scan
functions. These functions need no allocation or foreign calls.

Only the integer-function boundary of ``include/linux/math_header.rs`` has been
repaired to reuse these algorithms. Its other translated macros and fraction
types remain pending work and are not exposed through ``kernel::math``.

``CONFIG_RUST_BCD=y`` selects the translated binary-coded decimal conversions.
It requires native Rust support, defaults to disabled and is independent of
``HOST_TOOLS_LANG``. Leaving it disabled retains ``bcd.o``; the Rust choice
uses ``bcd_rust.o`` plus export-only C metadata. The implementation is pure,
allocation-free and retains the two exported C functions, including unsigned
overflow and invalid-digit behavior.

Independent Rust consumers import ``kernel::bcd``. The shared safe helpers do
not introduce duplicate C exports or foreign calls and work with either build
selection. Rust cannot reproduce C's ``__builtin_constant_p`` dispatch, so the
API names distinguish its two arithmetic paths explicitly: ``bcd2bin(u8)``
and ``bin2bcd(u32)`` match the runtime functions, while ``const_bcd2bin(u32)``
and ``const_bin2bcd(u32)`` match the full-width unsigned constant expressions.
For example, ``bin2bcd(1024)`` returns ``0xfa``, whereas
``const_bin2bcd(1024)`` returns ``0x664`` before any caller-requested narrowing.
All helpers support Rust constant evaluation without changing these semantics.
``bcd_is_valid`` and ``const_bcd_is_valid`` check the entire unsigned argument;
conversion itself does not reject invalid digits.

Rebuild modules after changing this option when using DWARF symbol versioning.
GCC and Clang omit the original parameter names from the declaration-only
export metadata, changing both function CRCs without changing their C ABI.
The C selection restores the original metadata; genksyms versions are retained.

After a completed x86-64 build, check the selected implementation in QEMU::

    python3 scripts/tests/check_bcd_kernel.py /tmp/lupos-build
    python3 scripts/tests/check_bcd_kernel.py /tmp/lupos-build --caller rust

The first command checks both C exports and the header's constant/runtime
dispatch. The second builds an independent native ``kernel::bcd`` consumer.
Each compares 69,646 inputs, including the complete 16-bit range, against the
unchanged C source and header. Use ``--allow-c-baseline`` for the original C
selection and ``--make-arg``, ``--qemu`` or ``--qemu-data`` for isolated tools.
The runner requires modules, printk and multiuser support, plus native Rust
for the Rust caller, and rejects forced module signatures for its unsigned
fixtures. It validates linked objects and image freshness before booting and
never loads modules into the host. Artifacts remain under ``rust-bcd-test/``
and ``rust-boot-test/``. Focused host tests are in ``test_bcd*.py``.

Native x86-64 builds with Rust 1.85 and extended DWARF module versions pass
both callers with C and Rust implementations. Switching C to Rust and back to
C in one output tree restores the original C CRCs and permits the original
C test module to load unchanged. The independent Rust API consumer also
loads unchanged when switching from C to Rust because it shares pure
helpers rather than depending on either C export's version.

``CONFIG_RUST_CTYPE=y`` selects the translated, immutable 256-byte character
classification table. It requires native Rust support, defaults to disabled,
and is independent of ``HOST_TOOLS_LANG``. The original ``ctype.o`` remains
available; the Rust selection uses ``ctype_rust.o`` and export-only C metadata.
Boot, firmware and host-tool copies are unchanged. The table preserves all
historical Latin-1 entries, including NBSP whitespace and punctuation at
``0xd7`` and ``0xf7``; this is neither ASCII-only nor Unicode classification.

Native Rust consumers import ``kernel::ctype``. That API reuses the translated
safe helper definitions with a private, bounded lookup into the selected C or
Rust table. It introduces no second table, allocation or initialization work.
Only the table-owning crate includes ``lib/ctype.rs``; independent modules must
not include that source again. Table-independent helpers remain usable in
constant expressions. Calls through ``kernel::ctype`` that read the foreign
table require runtime access, because its contents are resolved at link time.

The helpers retain unsigned-byte truncation where C performs it, the full
integer comparison used by ``isdigit``, and the original case-conversion
quirks. Differential tests compare every table byte and helper with the
unchanged C sources under signed/unsigned ``char``. Independent-crate tests
check that consumers share exactly one immutable table. Export metadata keeps
the original symbol versions under GCC and Clang, using both genksyms and
DWARF versioning; a complete array redeclaration after the public header is
needed to preserve Clang's original debug type.

After a completed x86-64 build, test C callers inside QEMU with::

    python3 scripts/tests/check_ctype_kernel.py /tmp/lupos-build

Use ``--caller rust`` to test a separately compiled native Rust module using
``kernel::ctype``, and ``--allow-c-baseline`` to exercise the original C table.
These checks require ``CONFIG_MODULES=y``, ``CONFIG_PRINTK=y`` and
``CONFIG_MULTIUSER=y``; the Rust caller also requires ``CONFIG_RUST=y``.
Forced module signatures must be disabled for the unsigned fixtures. The
runner checks actual archive membership and image freshness before booting,
and never loads a module into the host. ``--make-arg``, ``--qemu`` and
``--qemu-data`` support isolated toolchains and emulators. Test artifacts stay
under ``rust-ctype-test/`` and ``rust-boot-test/`` in the output directory.

Native x86-64 builds with Rust 1.85 and extended DWARF module versions pass
all 256 table bytes and 8,194 inputs per caller with both table selections.
A C module built against the original table also loads unchanged with the
Rust table, and a separately compiled ``kernel::ctype`` Rust module loads
unchanged after switching back to C. Both preserve the original ``_ctype``
symbol version. Host differential, build-selection and runtime-runner checks
are available through ``test_ctype*.py``.

``CONFIG_RUST_HEXDUMP=y`` selects the translated ``lib/hexdump.rs`` for the
kernel's hexadecimal conversion and dump functions. This option requires
native ``CONFIG_RUST`` support and is disabled by default; leaving it disabled
keeps the original C implementation. It is independent of ``HOST_TOOLS_LANG``.
``lib/hexdump_rust.rs`` supplies the kernel crate boundary without changing the
adjacent C source's build rule. The export-only ``hexdump_exports.c`` retains
the standard C export machinery and prototypes; it contains no conversion or
formatting implementation.

The translation needs no allocation. It preserves forward overlapping-buffer
operations, partial writes, native-endian groups, empty-buffer behavior, the
branch-free hexadecimal-digit conversion, and the kernel's printk formatting
and pointer policy. ``CONFIG_PRINTK_INDEX`` emits three records using the real
packed kernel binding and Rust call-site locations. Signed and unsigned host
``char`` bindings are tested separately from their common pointer ABI.

Rebuild modules when changing this option. In DWARF module-versioning mode,
GCC omits formal parameter names from the declaration-only export boundary,
which changes function CRCs despite preserving the machine-level C ABI.
The C selection retains its original metadata; the Rust selection and modules
built against it use matching generated versions.

The normal x86-64 build with Rust 1.85, extended module versions and printk
indexing is verified through QEMU. The unchanged C ``test_hexdump`` module
passes all 1,184 tests against the Rust implementation. An additional native
C-caller module checks all seven exported symbols, including conversion errors,
overlap, null/zero-length calls, and all printk prefix modes. After a completed
build with ``CONFIG_RUST_HEXDUMP=y``, ``CONFIG_MODULES=y``, ``CONFIG_PRINTK=y``
and ``CONFIG_MULTIUSER=y``, run it with::

    python3 scripts/tests/check_hexdump_kernel.py /tmp/lupos-build

This runner builds a temporary test module and loads it only inside QEMU,
never into the host kernel. It checks the linked implementation and rejects
stale images before booting. Use ``--allow-c-baseline`` to run the same checks
against the retained C selection, ``--make-arg`` for nonstandard toolchain
assignments, and ``--qemu``/``--qemu-data`` for an isolated emulator installation.
Forced module signatures must be disabled for this unsigned test fixture.
Artifacts remain in ``rust-hexdump-test/`` and ``rust-boot-test/`` under the
output directory. Host differential, guarded-memory, ABI and build-selection
regressions are available through ``test_hexdump*.py``.

``CONFIG_RUST_X86_INSN=y`` selects the translated x86 instruction decoder
and attribute tables for the running kernel. It requires ``CONFIG_RUST`` and
``CONFIG_INSTRUCTION_DECODER`` and defaults to disabled. The original C
``insn.o`` and ``inat.o`` remain selected when it is disabled; the C instruction
evaluator, compressed-boot decoder and objtool decoder are unchanged.

The native boundary uses the actual generated ``asm/insn.h`` bindings and
provides all ten decoder and six attribute-table C entry points. Field-wise
updates preserve union aliases, padding, noncanonical cached flags, manually
selected operand/address widths, and partial state after errors. Bounded reads
visit only requested bytes; cached stages need not access instruction memory.
Rebased cursors and input overlapping the state structure are covered. The
safe, allocation-free decoding core remains shared with the host tools.

Host regressions in ``test_x86_decoder_abi.py`` compare native layouts and
complete state against the original architecture C headers and implementation,
including guarded pages and self-aliasing input. Build-selection and runtime
checker regressions are in ``test_x86_decoder_build.py`` and
``test_x86_decoder_runtime.py``. To run the unchanged kernel kprobe/kretprobe
KUnit suite inside QEMU, build with ``CONFIG_KPROBES=y``, ``CONFIG_KUNIT=y``,
``CONFIG_KPROBES_SANITY_TEST=y``, ``CONFIG_MULTIUSER=y``, and a reliable unwinder
such as ``CONFIG_UNWINDER_ORC=y``, then run::

    python3 scripts/tests/check_x86_decoder_kernel.py /tmp/lupos-build

The checker verifies linked decoder selection and a current boot image, then
requires every configured kprobe case to pass without skips. Use
``--allow-c-baseline`` to run the same tests with the retained C decoder.
``--qemu`` and ``--qemu-data`` select an isolated emulator installation.
The minimal ``UNWINDER_GUESS`` configuration fails the original nested-kretprobe
stacktrace test with both implementations; it is not a decoder regression.
With ORC, all seven original kprobe/kretprobe cases pass on both C and Rust
x86-64 kernels built with Rust 1.85. Switching C to Rust and back to C in the
same output directory also passes all seven cases after each rebuild; stale
objects from the other selection remain harmless and are not linked.

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
* Apart from the opt-in integer-math, BCD, ctype, hexadecimal-helper and x86-decoder
  integrations above,
  target-kernel C objects still take precedence over adjacent Rust files.
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
