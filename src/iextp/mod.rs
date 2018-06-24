#![allow(unused_variables, dead_code)]
use byteorder::{LittleEndian, ReadBytesExt};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::io::Cursor;
use std::str;
use std::str::Utf8Error;

const SEGMENT_HEADER_SIZE: u16 = 40;
const UNIX_YEAR: i64 = 1_000_000_000;

pub trait Unmarshal {
    fn unmarshall(_buf: &[u8]) -> Self;

    fn parse_string(_buf: &[u8]) -> Result<&str, Utf8Error> {
        str::from_utf8(_buf).and_then(|val| Ok(val.trim_right()))
    }

    fn parse_price(_buf: &[u8; 8]) -> f64 {
        let n = Cursor::new(_buf).read_i64::<LittleEndian>().unwrap();
        (n as f64) / 10000.00
    }

    fn parse_timestamp(_buf: &[u8; 8]) -> DateTime<Utc> {
        let timestamp = Cursor::new(_buf).read_u64::<LittleEndian>().unwrap() as i64;

        let secs: i64 = timestamp / UNIX_YEAR;
        let n_secs = (timestamp % UNIX_YEAR) as u32;

        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(secs, n_secs), Utc)
    }

    // REVIEW(parse_event_time) -- implement test function and adapt logic accordingly if needed.
    fn parse_event_time(_buf: &[u8; 4]) -> DateTime<Utc> {
        let timestamp = Cursor::new(_buf).read_u32::<LittleEndian>().unwrap() as i64;
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    struct Mock;
    impl Unmarshal for Mock {
        fn unmarshall(_buf: &[u8]) -> Self {
            unimplemented!();
        }
    }

    #[test]
    fn test_parse_string() {
        // ZIEXT
        let symbol_literal = &[0x5a, 0x49, 0x45, 0x58, 0x54, 0x20, 0x20, 0x20];

        match Mock::parse_string(symbol_literal) {
            Ok(val) => assert_eq!("ZIEXT", val),
            Err(x) => panic!(x),
        };
    }

    #[test]
    fn test_parse_price() {
        // 99.05
        let price_literal = &[0x24, 0x1d, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(99.05, Mock::parse_price(price_literal));
    }

    #[test]
    fn test_parse_timestamp_secs() {
        // 2017-04-17 17:00:00
        let time_literal = &[0x00, 0xa0, 0x99, 0x97, 0xe9, 0x3d, 0xb6, 0x14];
        let date_expected = Utc.ymd(2017, 04, 17).and_hms(17, 0, 0);
        assert_eq!(date_expected, Mock::parse_timestamp(time_literal));
    }

    #[test]
    fn test_parse_timestamp_nsecs() {
        // 2016-08-23 15:30:32.572715948;
        let time_literal = &[0xac, 0x63, 0xc0, 0x20, 0x96, 0x86, 0x6d, 0x14];
        let date_expected = Utc.ymd(2016, 08, 23).and_hms_nano(19, 30, 32, 572_715_948);
        assert_eq!(date_expected, Mock::parse_timestamp(time_literal));
    }

    #[test]
    // TODO(test_parse_event_time)
    fn test_parse_event_time() {
        unimplemented!()
    }

}

type Message = [u8];

// #[derive(Debug, Default)]
// struct Segment {
//     Header: SegmentHeader,
//     Messages: Vec<Message>,
// }

struct SegmentHeader {
    // Version of the IEX-TP protocol.
    version: u8,
    // Reserved byte.
    // REVIEW: ^...?

    // A unique identifier for the higher-layer specification that describes
    // the messages contaiend within a segment. See the higher-layer protocol
    // specification for the protocol's message identification in IEX-TP.
    message_protocol_id: u16,

    // An identifier for a given stream of bytes/sequenced messages. Messages
    // received from multiple sources which use the same Channel ID are
    // guaranteed to be duplicates by sequence number and/or offset. See the
    // higher-layer protocol specification for the protocol's channel
    // identification on IEX-TP.
    channel_id: u32,

    // SessionID uniquely identifies a stream of messages produced by the
    // system. A given message is uniquely identified within a message
    // protocol by its Session ID and Sequence Number.
    session_id: u32,

    // PayloadLength is an unsigned binary count representing the number
    // of bytes contained in the segment's payload. Note that the Payload
    // Length field value does not include the length of the IEX-TP
    // header.
    payload_length: u16,

    // MessageCount is a count representing the number of Message Blocks
    // in the segment.
    message_count: u16,

    // StreamOffset is a counter representing the byte offset of the payload
    // in the data stream.
    stream_offset: i64,

    // FirstMessageSequenceNumber is a counter representing the sequence
    // number of the first message in the segment. If there is more than one
    // message in a segment, all subsequent messages are implicitly
    // numbered sequentially.
    first_message_sequence_number: i64,
    // The time the outbound segment was sent as set by the sender.
    send_time: DateTime<Utc>,
}

impl SegmentHeader {
    fn unmarshall(buf: &[u8; 40]) {}
}
