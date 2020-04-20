//! Types relating to sampling.
//!
//! When sampling, it is possible to receive instantaneous data or events concerning the
//! process(es) being profiled.

#![allow(non_camel_case_types)]

use byte::{BytesExt, Result};
use libc::pid_t;
use perf_event_open_sys as sys;
use std::os::raw::c_void;

/// Controls the various fields that are provided in [`PerfRecordSample`] when sampling.
///
/// Corresponds to the `sample_type` field `perf_event_attr` in the [`perf_event_open`][man] man
/// page.
///
/// [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
///
/// Not all possible values of this enum are currently included.
#[derive(Copy, Clone)]
#[repr(u64)]
pub enum PerfSampleType {
    /// Fill out the `ip` field of [`PerfRecordSample`] when sampling.
    ///
    /// [`PerfRecordSample`]: struct.PerfRecordSample.html
    IP = sys::bindings::perf_event_sample_format_PERF_SAMPLE_IP,

    /// Fill out the `pid` / `tid` fields of [`PerfRecordSample`] when sampling.
    ///
    /// [`PerfRecordSample`]: struct.PerfRecordSample.html
    TID = sys::bindings::perf_event_sample_format_PERF_SAMPLE_TID,

    /// Fill out the `timestamp` field of [`PerfRecordSample`] when sampling.
    ///
    /// [`PerfRecordSample`]: struct.PerfRecordSample.html
    TIME = sys::bindings::perf_event_sample_format_PERF_SAMPLE_TIME,

    /// Fill out the `addr` field of [`PerfRecordSample`] when sampling.
    ///
    /// [`PerfRecordSample`]: struct.PerfRecordSample.html
    ADDR = sys::bindings::perf_event_sample_format_PERF_SAMPLE_ADDR,

    /// Fill out the `callchain` field of [`PerfRecordSample`] when sampling.
    ///
    /// [`PerfRecordSample`]: struct.PerfRecordSample.html
    CALLCHAIN = sys::bindings::perf_event_sample_format_PERF_SAMPLE_CALLCHAIN,

    /// Fill out the `id` field of [`PerfRecordSample`] when sampling.
    ///
    /// [`PerfRecordSample`]: struct.PerfRecordSample.html
    ID = sys::bindings::perf_event_sample_format_PERF_SAMPLE_ID,

    /// Fill out the `cpu` field of [`PerfRecordSample`] when sampling.
    ///
    /// [`PerfRecordSample`]: struct.PerfRecordSample.html
    CPU = sys::bindings::perf_event_sample_format_PERF_SAMPLE_CPU,

    /// Fill out the `period` field of [`PerfRecordSample`] when sampling.
    ///
    /// [`PerfRecordSample`]: struct.PerfRecordSample.html
    PERIOD = sys::bindings::perf_event_sample_format_PERF_SAMPLE_PERIOD,

    /// Fill out the `stream_id` field of [`PerfRecordSample`] when sampling.
    ///
    /// [`PerfRecordSample`]: struct.PerfRecordSample.html
    STREAM_ID = sys::bindings::perf_event_sample_format_PERF_SAMPLE_STREAM_ID,

    /// Fill out the `raw_sample` field of [`PerfRecordSample`] when sampling.
    ///
    /// [`PerfRecordSample`]: struct.PerfRecordSample.html
    RAW = sys::bindings::perf_event_sample_format_PERF_SAMPLE_RAW,

    /// Fill out the `weight` field of [`PerfRecordSample`] when sampling.
    ///
    /// [`PerfRecordSample`]: struct.PerfRecordSample.html
    WEIGHT = sys::bindings::perf_event_sample_format_PERF_SAMPLE_WEIGHT,
}

/// A set of PerfSampleType that is implemented using bit math.
#[derive(Default)]
pub struct PerfSampleTypeSet(pub sys::bindings::perf_event_sample_format);

impl PerfSampleTypeSet {
    /// Add the given PerfSampleType to the set
    pub fn add(&mut self, sample_type: PerfSampleType) {
        self.0 |= sample_type as sys::bindings::perf_event_sample_format;
    }

    /// Returns true if the set contains the given PerfSampleType
    pub fn contains(&self, sample_type: PerfSampleType) -> bool {
        self.0 & sample_type as sys::bindings::perf_event_sample_format != 0
    }
}

/// This record indicates a throttle / unthrottle event.
#[derive(Debug)]
pub struct PerfRecordThrottle {
    /// Timestamp of when the record was created
    pub time: u64,
    /// A unique ID. If the event is a member of an event group, the group leader ID is
    /// returned.
    pub id: u64,
    /// A unique ID. Unlike the above field, this is always the actual ID and never the group
    /// leader ID.
    pub stream_id: u64,
}

impl PerfRecordThrottle {
    fn decode(_attrs: &sys::bindings::perf_event_attr, data: &[u8]) -> Result<Self> {
        let mut offset = 0;
        Ok(Self {
            time: data.read(&mut offset)?,
            id: data.read(&mut offset)?,
            stream_id: data.read(&mut offset)?,
        })
    }
}

#[test]
fn decode_perf_record_throttle() {
    let attrs = sys::bindings::perf_event_attr::default();
    let record = PerfRecordThrottle::decode(
        &attrs,
        &[
            1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0,
        ],
    )
    .unwrap();
    assert_eq!(record.time, 1);
    assert_eq!(record.id, 2);
    assert_eq!(record.stream_id, 3);
}

/// This record is a sample of the current state of some process.
///
/// Corresponds to the anonymous struct under `PERF_RECORD_SAMPLE` in [`perf_event_open`][man] man
/// page.
///
/// [man]: http://man7.org/linux/man-pages/man2/perf_event_open.2.html
#[derive(Debug, Default, PartialEq, Eq)]
pub struct PerfRecordSample {
    /// Instruction pointer. Included if [`PerfSampleType::IP`] is enabled.
    pub ip: Option<*const c_void>,

    /// Process id. Included if [`PerfSampleType::TID`] is enabled.
    pub pid: Option<pid_t>,

    /// Thread id. Included if [`PerfSampleType::TID`] is enabled.
    pub tid: Option<pid_t>,

    /// Timestamp of when the sample was taken. Obtained via local_clock() which is a hardware
    /// timestamp if available and the jiffies value if not.
    pub time: Option<u64>,

    /// Usually the address of a tracepoint, breakpoint, or software event; otherwise the value is
    /// 0. Included if [`PerfSampleType::ADDR`] is enabled.
    pub addr: Option<u64>,

    /// A unique ID. If the event is a member of an event group, the group leader ID is
    /// returned. Included if [`PerfSampleType::ID`] is enabled.
    pub id: Option<u64>,

    /// A unique ID. Unlike the above field, this is always the actual ID and never the group
    /// leader ID. Included if [`PerfSampleType::STREAM_ID`] is enabled.
    pub stream_id: Option<u64>,

    /// Value indicating which CPU was being used. Included if [`PerfSampleType::CPU`] is enabled.
    pub cpu: Option<u32>,

    /// Value indicating the current sampling period. Included if [`PerfSampleType::PERIOD`] is
    /// enabled.
    pub period: Option<u64>,

    // XXX placeholder; read format stuff not supported yet.
    _v: (),

    /// The current callchain. Included if [`PerfSampleType::CALLCHAIN`] is enabled.
    pub callchain: Option<Vec<*const c_void>>,

    /// This contains the raw record data. Included if [`PerfSampleType::RAW`] is enabled.
    ///
    /// This raw record data is opaque with respect to the ABI. The ABI doesn't make any promises
    /// with respect to the stability of its content, it may vary depending on the event, hardware,
    /// and kernel versions.
    pub raw_sample: Option<Vec<u8>>,

    // XXX placeholder; branch stack stuff not supported yet.
    _lbr: (),

    // XXX placeholder; user register stuff not supported yet.
    _user_regs: (),

    // XXX placeholder; user stack stuff not supported yet.
    _user_stack: (),

    /// Value provided by the hardware that indicates how costly the event was. Included if
    /// [`PerfSampleType::WEIGHT`] is enabled.
    ///
    /// This allows expensive events to stand out more clearly in profiles.
    pub weight: Option<u64>,

    // XXX placeholder; data_src stuff not supported yet.
    _data_src: (),

    // XXX placeholder; transaction stuff not supported yet.
    _transaction: (),

    // XXX placeholder; cpu register stuff not supported yet.
    _cpu_regs: (),
}

impl PerfRecordSample {
    fn decode(attrs: &sys::bindings::perf_event_attr, data: &[u8]) -> Result<Self> {
        let sample_type = PerfSampleTypeSet(attrs.sample_type);
        let mut offset = 0;

        let ip = if sample_type.contains(PerfSampleType::IP) {
            Some(data.read::<u64>(&mut offset)? as *const _)
        } else {
            None
        };

        let (pid, tid) = if sample_type.contains(PerfSampleType::TID) {
            (Some(data.read(&mut offset)?), Some(data.read(&mut offset)?))
        } else {
            (None, None)
        };

        let time = if sample_type.contains(PerfSampleType::TIME) {
            Some(data.read(&mut offset)?)
        } else {
            None
        };

        let addr = if sample_type.contains(PerfSampleType::ADDR) {
            Some(data.read(&mut offset)?)
        } else {
            None
        };

        let id = if sample_type.contains(PerfSampleType::ID) {
            Some(data.read(&mut offset)?)
        } else {
            None
        };

        let stream_id = if sample_type.contains(PerfSampleType::STREAM_ID) {
            Some(data.read(&mut offset)?)
        } else {
            None
        };

        let cpu = if sample_type.contains(PerfSampleType::CPU) {
            let value = data.read(&mut offset)?;
            let _res: u32 = data.read(&mut offset)?;
            Some(value)
        } else {
            None
        };

        let period = if sample_type.contains(PerfSampleType::PERIOD) {
            Some(data.read(&mut offset)?)
        } else {
            None
        };

        let callchain = if sample_type.contains(PerfSampleType::CALLCHAIN) {
            let len: u64 = data.read(&mut offset)?;
            let mut callchain = vec![];
            for _ in 0..len {
                callchain.push(data.read::<u64>(&mut offset)? as *const _);
            }
            Some(callchain)
        } else {
            None
        };

        let raw_sample = if sample_type.contains(PerfSampleType::RAW) {
            let len: u32 = data.read(&mut offset)?;
            let mut sample = vec![];
            for _ in 0..len {
                sample.push(data.read(&mut offset)?);
            }
            Some(sample)
        } else {
            None
        };

        let weight = if sample_type.contains(PerfSampleType::WEIGHT) {
            Some(data.read(&mut offset)?)
        } else {
            None
        };

        Ok(Self {
            ip,
            pid,
            tid,
            time,
            addr,
            id,
            stream_id,
            cpu,
            period,
            callchain,
            raw_sample,
            weight,
            ..Default::default()
        })
    }
}

#[test]
fn decode_perf_record_sample_empty() {
    let attrs = sys::bindings::perf_event_attr::default();
    let record = PerfRecordSample::decode(&attrs, &[]).unwrap();
    assert_eq!(record, Default::default());
}

#[cfg(test)]
fn make_test_record(sample_type: PerfSampleType, data: &[u8]) -> PerfRecordSample {
    let mut sample_type_set = PerfSampleTypeSet::default();
    sample_type_set.add(sample_type);
    let mut attrs = sys::bindings::perf_event_attr::default();
    attrs.sample_type = sample_type_set.0;
    PerfRecordSample::decode(&attrs, data).unwrap()
}

#[test]
fn decode_perf_record_sample_ip() {
    let record = make_test_record(PerfSampleType::IP, &[5, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(record.ip, Some(5 as *const _));
}

#[test]
fn decode_perf_record_sample_pid_tid() {
    let record = make_test_record(PerfSampleType::TID, &[5, 0, 0, 0, 6, 0, 0, 0]);
    assert_eq!(record.pid, Some(5));
    assert_eq!(record.tid, Some(6));
}

#[test]
fn decode_perf_record_sample_time() {
    let record = make_test_record(PerfSampleType::TIME, &[8, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(record.time, Some(8));
}

#[test]
fn decode_perf_record_sample_addr() {
    let record = make_test_record(PerfSampleType::ADDR, &[9, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(record.addr, Some(9));
}

#[test]
fn decode_perf_record_sample_id() {
    let record = make_test_record(PerfSampleType::ID, &[10, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(record.id, Some(10));
}

#[test]
fn decode_perf_record_sample_stream_id() {
    let record = make_test_record(PerfSampleType::STREAM_ID, &[11, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(record.stream_id, Some(11));
}

#[test]
fn decode_perf_record_sample_cpu() {
    let record = make_test_record(PerfSampleType::CPU, &[12, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(record.cpu, Some(12));
}

#[test]
fn decode_perf_record_sample_period() {
    let record = make_test_record(PerfSampleType::PERIOD, &[13, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(record.period, Some(13));
}

#[test]
fn decode_perf_record_sample_call_chain() {
    let record = make_test_record(
        PerfSampleType::CALLCHAIN,
        &[
            2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
        ],
    );
    assert_eq!(record.callchain, Some(vec![1 as *const _, 2 as *const _]));
}

#[test]
fn decode_perf_record_sample_raw_sample() {
    let record = make_test_record(PerfSampleType::RAW, &[4, 0, 0, 0, 3, 4, 5, 6]);
    assert_eq!(record.raw_sample, Some(vec![3, 4, 5, 6]));
}

#[test]
fn decode_perf_record_sample_weight() {
    let record = make_test_record(PerfSampleType::WEIGHT, &[5, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(record.weight, Some(5));
}

/// This is a single sample representing the instantaneous state or an event concerning the process
/// being profiled.
#[derive(Debug)]
pub enum PerfRecord {
    /// Throttle record
    Throttle(PerfRecordThrottle),

    /// Unthrottle record
    Unthrottle(PerfRecordThrottle),

    /// Sample record
    Sample(PerfRecordSample),
}

impl PerfRecord {
    /// Decode the `PerfRecord` from the information given to us from the kernel. The given
    /// `perf_event_attr` is the one passed to `perf_event_open`. The given type is that from
    /// `perf_event_header`, and the data is the record payload.
    ///
    /// Returns the decoded record or an error if the record was malformed for some reason.
    pub fn decode(attrs: &sys::bindings::perf_event_attr, type_: u32, data: &[u8]) -> Result<Self> {
        Ok(match type_ {
            sys::bindings::perf_event_type_PERF_RECORD_SAMPLE => {
                PerfRecord::Sample(PerfRecordSample::decode(attrs, data)?)
            }
            sys::bindings::perf_event_type_PERF_RECORD_THROTTLE => {
                PerfRecord::Throttle(PerfRecordThrottle::decode(attrs, data)?)
            }
            sys::bindings::perf_event_type_PERF_RECORD_UNTHROTTLE => {
                PerfRecord::Unthrottle(PerfRecordThrottle::decode(attrs, data)?)
            }
            t => panic!("Unknown perf_event_type {}", t),
        })
    }
}
