# Changelog

All notable changes to the `omegon-extension` Rust SDK are documented here.

This crate uses its own SemVer release line. `SDK_CONTRACT_VERSION` is the
cross-language wire compatibility line and may intentionally differ from the
crate version.

## [Unreleased]

## [0.25.2] - 2026-06-03

### Added

- Add `resource.open@1` to the canonical HostAction SDK contract alongside `terminal.create@1` and `package.install@1`.
- Add typed Rust `resource.open@1` helper types and public exports.
- Add `ResourceOpenPermissions` manifest policy fields for host-side `resource.open@1` validation.
- Add cross-SDK contract lockstep checking and downstream issue automation for Python and TypeScript SDK ports.

### Changed

- Report SDK contract version `0.25` from the Rust SDK contract artifact and constants.

## [0.25.0] - 2026-05-27

### Added

- Extracted the Rust `omegon-extension` SDK into a standalone crate layout.
- Added explicit SDK contract constants and embedded `SDK_CONTRACT_JSON`.
- Added `schema/sdk-contract.json` for cross-language SDK lockstep validation.
- Added protocol smoke and contract drift tests for the standalone crate.
