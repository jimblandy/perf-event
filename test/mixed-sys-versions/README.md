# Mixing major versions of perf-event-open-sys

This is a little hierarchy of toy crates that depend on two different
major versions of `perf-event-open-sys`. A successful build shows that
different major versions can co-exist in the same dependency graph,
without causing link-time errors.

It's been suggested that this crate should have bindgen mark the
struct types used by `perf_event_open` as `#[non_exhaustive]`, so that
when Linux simply adds new fields to the end of the struct (the common
case), we can treat that as a non-breaking change, and release a crate
with the new Linux functionality under the same major version number.

In #34, @jimblandy decided against this, for a few reasons:

- A `sys` crate should expose bindings in as neutral a way as
  possible, so that they can be used by anyone who needs the
  underyling functionality. Placing the `#[non_exhaustive]` attribute
  on a struct would prevent users from writing out literals of that
  struct. That probably isn't common---but it's not a `sys` crate's
  place to make that sort of decision.

- Unlike some (most?) `sys` crates, it is fine to have multiple copies
  of `perf-event-open-sys` in a dependency graph. Nothing forces
  different crates to agree on a particular version of
  `perf-event-open-sys`.

- Because Linux promises not to break userspace programs, there is no
  reason to upgrade to a new version of `perf-event-open-sys` unless
  one needs new functionality. And if one does need new functionality,
  one is revising one's code anyway, and the source compatibilty
  provided by `#[non_exhaustive]` isn't critical.

Overall, there isn't much cost to changing the major version number
frequently, so we can adhere strictly to the principle that `sys`
crates should not restrict what users can do with the bindings.
