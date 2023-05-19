# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Expose the `IOC_SET_BPF` ioctl as `Counter::set_bpf`.
- Add `KProbe` and `UProbe` events.
- Add `Event::update_attrs_with_data` to allow events to store references to
  owned data within `Builder`'s `perf_event_attr` struct.
- Add `Tracepoint` event type.

## [0.6.0] - 2023-05-17
### Added
- Expose the `perf_event_data` crate as the `data` module.
- Add `Record::parse_record` to parse records to `data::Record`.
- Add `Software::CGROUP_SWITCHES` and `Software::BPF_OUTPUT` events (#9). @Phantomical

### Changed
- `Hardware` is no longer a rust enum. The constants remain the same.
- `Software` is no longer a rust enum. The constants remain the same.
- The same applies for `WhichCache`, `CacheOp`, and `CacheResult`.
- `WhichCache` has been renamed to `CacheId`.

## [0.5.0] - 2023-04-20
### Added
- Add `Sampler` - a `Counter` which also reads sample events emitted by the kernel.
- Group leaders can now be a `Group`, `Counter`, or `Sampler`.
- Add `Builder::build_group` to build a group with a non-default config.
- Add all missing config options for `Builder`.

### Changed
- The `Event` enum has been replaced with an `Event` trait.
- Constructing a `Builder` now requires that you specify an event type up front
  instead of having a default of `Hardware::INSTRUCTIONS`.