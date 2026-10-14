# Notice

This crate is a standalone copy of `ext/fetch` from
[denoland/deno](https://github.com/denoland/deno) at tag `v2.9.7`
(`deno_fetch` 0.283.0), MIT licensed. See `LICENSE.md`.

Changes from upstream:

- Workspace dependency versions are pinned explicitly, because the crate is
  built outside the Deno monorepo.
- `rustls` is pinned to `=0.23.45` (upstream: `=0.23.40`). The newer release
  fixes a TLS 1.3 handshake issue.
- `hickory-resolver` is 0.26 (upstream: 0.25). `dns.rs` is ported to the 0.26
  API. The hickory 0.25 advisories no longer apply to this crate.
- `system_conf.rs` is new. On Apple platforms it reads `/etc/resolv.conf`
  instead of hickory's SystemConfiguration loader, which rejects zone-scoped
  IPv6 DNS servers. Its conversion is adapted from hickory-resolver
  `src/system_conf/unix.rs`, which is Apache-2.0 OR MIT.
- Test fixtures from `ext/tls/testdata` are copied into `testdata/`.
