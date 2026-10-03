Lupos io_uring zero-copy receive source checkpoint
================================================

This is a reviewed, UNBUILT reconstruction of ``io_uring/zcrx.c``. All 84
original function definitions have Rust implementations. It is not native ABI,
link, runtime, DMA or concurrency acceptance. ``CONFIG_RUST_IO_URING_ZCRX``
is default-off; the original C provider, header and maintained tests remain
unchanged. Kbuild selects exactly one ``zcrx.o`` implementation.

Source and implementation
-------------------------

The reconstruction uses base commit
``298abda59725d491c10be678618ac76300d1aba1``. The complete reviewed revision-02
source archive has SHA-256
``2890cd5aabbfd825423e6a15d930b95d3c7d2c55a273c5903efbd7001abf4571``.
At that checkpoint, all seven provider/integration files were byte-identical
to that seal. The shared
``rust/Makefile`` include is composed separately with existing provider rules.

Configured bindgen reads the original kernel, io_uring and networking headers;
Rust uses ``kernel::ffi`` and no guessed external structure layouts. The C
companion contains narrow macro/inline/field access and callback-table
initialization. No provider algorithm falls back to the original C unit.

Rust owns pinned-memory and DMA-BUF import/unwind, area publication and DMA
unmapping, device and context lifetimes, registration and fd export/import,
refill-ring acquire/release ordering, user and page-pool references, notification
task work, control operations and TCP/skb copy/zero-copy receive. Distinct
warning-once sites and spinlock initialization classes are preserved.

Independent source review covered all 84 original bodies. Corrective review
closed the identified anonymous ``io_ring_ctx`` member-access and kernel FFI
scalar/pointer conversion issues. Function inventories, original-source hashes
and source-preserving patch applicability were checked. Native ABI, KCFI,
symbol closure and execution remain unverified; source review does not
establish behavioral parity.

First native attempt and correction
-----------------------------------

At commit ``bc846d0c26f566e6a586592c2a15d5a0aa9f5f94``, the configured C
provider and all four original callers compiled, and their repeated builds
passed no-op checks. The C/R configurations differed only in the Rust provider
selector. Configured bindgen succeeded; the first Rust compile failed with
seven diagnostics: three missing compound macro constants, two missing enum
constants used at three sites, and one untyped private-data pointer.

The correction adds typed C-derived constant aliases and explicitly types the
``io_zcrx_ifq`` pointer. Its reviewed source archive has SHA-256
``9daf8f50736297cf7c61bfc830fc1f6615d867bb835b0f43b2ee81ef449c7b59``.
It changes no algorithms, tests or compiler flags. Corrected native replay
remains pending; the failed first attempt does not establish Rust build or
behavioral acceptance.

Required native checks and original C tests
------------------------------------------

The first native targets are paired configured C/R ``io_uring/zcrx.o`` builds
and the Rust ``io_uring/zcrx_helpers.o``. Compile unchanged ``memmap.o``,
``register.o``, ``net.o`` and ``io_uring.o`` callers, compare their provider
relocations and verify unique ownership of all six public zcrx entry points.
The bounded ``io_uring/built-in.a`` target can check canonical provider selection;
an archive check does not establish final kernel linking or runtime behavior.

The maintained test authority is
``tools/testing/selftests/drivers/net/hw/iou-zcrx.c``, its original ``Makefile``
and unchanged ``iou-zcrx.py`` orchestrator. The Makefile requires a liburing
header/library pair exposing ``io_uring_register_ifq`` and links with ``-luring``.
Retain its feature gate, C payload assertions, failures and skips. Do not
substitute a translated Rust test harness.

Required original cases are:

* ``test_zcrx.single`` and ``test_zcrx.rss``
* ``test_zcrx_oneshot.single`` and ``test_zcrx_oneshot.rss``
* ``test_zcrx_large_chunks``

Run matched C/R kernels on an approved isolated receive NIC and remote endpoint
with IPv6/TCP, two combined channels, data split, ntuple/RSS steering and the
page-pool provider features. The large-chunk case needs hugepage-backed memory
and verifies the receive-buffer size through netlink. Unsupported-feature skips
remain coverage gaps. These device tests have not run.

NODEV fallback, DMA-BUF error unwinds, imported-fd/context teardown, additional
areas, invalid refill entries, notifications/statistics, allocation/user-copy
faults, CQ saturation, concurrent recycling, other configurations/architectures
and sanitizer/lockdep coverage also remain outstanding. Driver implementation
and native device operations are deferred.
