AFS address preferences source checkpoint
==========================================

Provenance and scope
--------------------

This default-off, unbuilt checkpoint reconstructs ``fs/afs/addr_prefs.c`` using
the original C and headers at Linux commit
``07b2f9d28c455307d0f626c3673f6a8aa1f3968d``. It combines the initial source
checkpoint with the pointer-offset conversion and composite-object identity
corrections. The final source delta archive SHA-256 is
``0f984044bb5088ef8c1f23313fb587e56ea5f746ede3dbeeabd77f95dc7eb67d``.

Rust implements command splitting, address parsing/comparison, insertion,
addition/deletion, proc writes, RCU publication and both preference lookup
entry points. Configured C declarations supply layouts; adapters retain kernel
allocation, inline, RCU and memory-access primitives. Original lookup,
comparison, error and ownership behavior is retained. The C provider and
existing tests remain unchanged.

``CONFIG_RUST_AFS_ADDR_PREFS`` defaults to ``n``. The enabled proposal retains
the ``kafs`` aggregate and original ``addr_prefs.o`` slot, selects its Rust
source and adds the ordinary C helper. Its explicit Rust object rule derives
composite membership from the actual target stem so the intended module
identity remains ``fs/afs/kafs`` for builtin and module configurations.

Original tests and pending gates
-------------------------------

No maintained in-tree C suite directly exercising address preferences was
identified. RxRPC KUnit covers crypto; ``lib/test-kstrtox.c`` covers numeric
conversion; statmount's filesystem list is not address-preference coverage.
These neighboring tests cannot establish acceptance of this provider.

An applicable maintained external C suite or separately approved native
qualification route is still needed. Qualification must cover proc command
parsing/readback, IPv4/IPv6 ordering and priority replacement, bounded growth,
all error paths, RCU publication/retirement and production client lookups.
Proc-only checks do not establish full AFS/RxRPC lookup behavior. Controlled
server address lists and the required configured client dependencies are
needed for that portion.

Configured bindings, actual native builtin/module commands and compilation,
selector/inspection lifecycle, symbol/link/module/BTF/ABI checks and isolated
runtime comparison remain pending. A suspected original private-allocation
growth leak remains a separate, runtime-unconfirmed baseline concern; no
semantic repair is included. No build or test execution is claimed.
