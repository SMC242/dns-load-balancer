#![allow(clippy::upper_case_acronyms)]

use std::io::{Bytes, Error as IoError, ErrorKind, Read};

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    /// Standard query
    Query = 0,
    /// Inverse query
    IQuery = 1,
    /// Server status request
    Status = 2,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ResponseCode {
    /// OK
    NoError = 0,
    /// Formatting error
    FormErr = 1,
    /// The server failed
    ServFail = 2,
    /// The domain doesn't exist
    NXDomain = 3,
}

// This section is 16 bits
pub struct DNSHeaderFlags {
    // 1 bit
    /// Whether the message is a query or a reply
    is_query: bool,
    // 4 bits
    /// The type of operation
    opcode: OpCode,
    /// Whether the answer is authoritative. Only used in responses
    is_authoritative: bool,
    /// Whether the message was truncated
    truncated: bool,
    /// Whether the client wants a recursive query
    recursion_desired: bool,
    /// Whether the responding resolver supports recusion
    recursion_available: bool,
    /// Zero. Reserved for future use. Always 0
    zero: bool,
    /// If the replying DNS server verified the data. Only used in responses
    authentic_data: bool,
    /// If non-verified data is allowed in responses. Only used in queries
    checking_disabled: bool,
    // 4 bits
    /// The response code
    rcode: ResponseCode,
}

pub struct DNSHeaders {
    transaction_id: u16,
    flags: DNSHeaderFlags,
    n_questions: u16,
    n_answers: u16,
    n_authority_rrs: u16,
    n_additional_rrs: u16,
}

/// See https://en.wikipedia.org/wiki/List_of_DNS_record_types
pub enum ResourceRecordType {
    A = 1,
    NS = 2,
    Cname = 5,
    SOA = 6,
    PTR = 12,
    HINFO = 13,
    MX = 15,
    TXT = 16,
    RP = 17,
    AFSDB = 18,
    SIG = 24,
    KEY = 25,
    AAAA = 28,
    LOC = 29,
    SRV = 33,
    NAPTR = 35,
    KX = 36,
    CERT = 37,
    DNAME = 39,
    APL = 42,
    DS = 43,
    SSHFP = 44,
    IPSECKEY = 45,
    RRSIG = 46,
    NSEC = 47,
    DNSKEY = 48,
    DHCID = 49,
    NSEC3 = 50,
    NSEC3PARAM = 51,
    TLSA = 52,
    SMIMEA = 53,
    HIP = 55,
    CDS = 59,
    CDNSKEY = 60,
    OPENPGPKEY = 61,
    CSYNC = 62,
    ZONEMD = 63,
    SVCB = 64,
    HTTPS = 65,
    EUI48 = 108,
    EUI64 = 109,
    TKEY = 249,
    TSIG = 250,
    URI = 256,
    CAA = 257,
    TA = 32768,
    DLV = 32769,
}

/// De-facto just IN but here's the complete list http://www.faqs.org/rfcs/rfc2929.html
pub enum ClassCode {
    /// The class for the Internet. Most common
    IN = 1,
    /// Chaos
    CH = 3,
    /// Hesiod
    HS = 4,
}

/// One of multiple questions sent in a query
pub struct Question {
    // Variable length
    /// The name requested
    name: String,
    // 16 bits
    /// The type of resource record requested
    r#type: ResourceRecordType,
    /// The class code. Usually IN
    class: ClassCode,
}

pub struct ResourceRecord {
    // Variable length
    name: String,
    // 16 bits
    r#type: ResourceRecordType,
    // 16 bits
    class: ClassCode,
    /// The time to live for this record (I.E how long until it should be queried again)
    ttl: u32,
    /// The length of the RDATA field
    rd_length: u16,
    /// The data of size `rd_length`
    rdata: String,
}

pub struct DNSRequest {
    headers: DNSHeaders,
    records: Vec<ResourceRecord>,
}

pub enum DNSHeaderParseError {
    InvalidTransactionId,
    InvalidOpCode,
    InvalidResponseCode,
    IoError(IoError),
    InvalidNAdditionalRRs,
    InvalidNAuthorityRRs,
    InvalidNAnswers,
    InvalidNQuestions,
}

pub enum DNSBodyParseError {
    IoError(IoError),
}

/// Assumption: big-endian
fn to_bitflag(idx: u8, flag: bool) -> u8 {
    u8::from(flag) << idx
}

impl DNSHeaderFlags {
    pub fn to_bytes(&self) -> Vec<u8> {
        // QR = 0 when is_query = true
        let shifted_is_query = to_bitflag(7, !self.is_query);
        let shifted_opcode = (self.opcode.clone() as u8) << 6;
        let shifted_is_authoritative = to_bitflag(2, self.is_authoritative);
        let shifted_truncated = to_bitflag(1, self.truncated);
        let shifted_recursion_desired = to_bitflag(0, self.recursion_desired);
        let first_byte = shifted_is_query
            | shifted_opcode
            | shifted_is_authoritative
            | shifted_truncated
            | shifted_recursion_desired;

        let shifted_recursion_available = to_bitflag(7, self.recursion_available);
        let shifted_zero = to_bitflag(6, self.zero);
        let shifted_authentic_data = to_bitflag(5, self.authentic_data);
        let shifted_checking_disabled = to_bitflag(4, self.checking_disabled);
        let shifted_rcode = self.rcode.clone() as u8;
        let second_byte = shifted_recursion_available
            | shifted_zero
            | shifted_authentic_data
            | shifted_checking_disabled
            | shifted_rcode;

        vec![first_byte, second_byte]
    }
}

impl DNSHeaders {
    pub fn to_bytes(&self) -> impl Iterator<Item = u8> {
        let flag_bytes = self.flags.to_bytes();
        self.transaction_id
            .to_be_bytes()
            .into_iter()
            .chain(flag_bytes.into_iter())
            .chain(self.n_questions.to_be_bytes())
            .chain(self.n_answers.to_be_bytes())
            .chain(self.n_authority_rrs.to_be_bytes())
            .chain(self.n_additional_rrs.to_be_bytes())
    }
}

impl TryFrom<u8> for OpCode {
    type Error = DNSHeaderParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Query),
            1 => Ok(OpCode::IQuery),
            2 => Ok(OpCode::Status),
            _ => Err(DNSHeaderParseError::InvalidOpCode),
        }
    }
}

impl TryFrom<u8> for ResponseCode {
    type Error = DNSHeaderParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ResponseCode::NoError),
            1 => Ok(ResponseCode::FormErr),
            2 => Ok(ResponseCode::ServFail),
            3 => Ok(ResponseCode::NXDomain),
            _ => Err(DNSHeaderParseError::InvalidResponseCode),
        }
    }
}

fn read_u16<T: Read, E>(err: E, stream: &mut Bytes<T>) -> Result<u16, E> {
    stream
        .take(2)
        .collect::<Result<Vec<u8>, IoError>>()
        .and_then(|bytes| match bytes.try_into() {
            Ok(x) => Ok(x),
            Err(_) => Err(IoError::new(
                std::io::ErrorKind::InvalidInput,
                "Expected u16",
            )),
        })
        .map(u16::from_be_bytes)
        .or(Err(err))
}

// https://users.rust-lang.org/t/extracting-bits-from-bytes/77110/2
fn bit_at(idx: u8, byte: u8) -> bool {
    if idx > 7 {
        panic!("Byte index {idx} out of range")
    } else {
        match (byte >> idx) & 1 {
            0 => false,
            1 => true,
            x => panic!("Bit math was incorrect. Expected 0|1 but got {x}"),
        }
    }
}

fn read_opcode(byte: u8) -> u8 {
    let mask = 0b01111000;
    (byte & mask) >> 3
}

fn read_rcode(byte: u8) -> u8 {
    let mask = 0b00001111;
    byte & mask
}

fn parse_flags<R: Read>(stream: &mut Bytes<R>) -> Result<DNSHeaderFlags, DNSHeaderParseError> {
    let first_byte: u8 = match stream.next() {
        Some(Ok(x)) => x,
        Some(Err(err)) => return Err(DNSHeaderParseError::IoError(err)),
        None => {
            return Err(DNSHeaderParseError::IoError(IoError::new(
                ErrorKind::UnexpectedEof,
                "Stream terminated during first flags section",
            )));
        }
    };

    // Big-endian, so the leftmost bit is at index 7
    // QR = 0 when is_query = true
    let is_query = !bit_at(7, first_byte);
    let opcode: OpCode = read_opcode(first_byte).try_into()?;
    let is_authoritative = bit_at(2, first_byte);
    let truncated = bit_at(1, first_byte);
    let recursion_desired = bit_at(0, first_byte);

    let second_byte: u8 = match stream.next() {
        Some(Ok(x)) => x,
        Some(Err(err)) => return Err(DNSHeaderParseError::IoError(err)),
        None => {
            return Err(DNSHeaderParseError::IoError(IoError::new(
                ErrorKind::UnexpectedEof,
                "Stream terminated during second flags section",
            )));
        }
    };

    let recursion_available = bit_at(7, second_byte);
    let zero = bit_at(6, second_byte);
    let authentic_data = bit_at(5, second_byte);
    let checking_disabled = bit_at(4, second_byte);
    let rcode: ResponseCode = read_rcode(second_byte).try_into()?;

    Ok(DNSHeaderFlags {
        is_query,
        opcode,
        is_authoritative,
        truncated,
        recursion_desired,
        recursion_available,
        zero,
        authentic_data,
        checking_disabled,
        rcode,
    })
}

pub fn parse_header<T: std::io::Read>(
    bitstream: &mut Bytes<T>,
) -> Result<DNSHeaders, DNSHeaderParseError> {
    let transaction_id: u16 = read_u16(DNSHeaderParseError::InvalidTransactionId, bitstream)?;
    let flags = parse_flags(bitstream)?;

    let n_questions: u16 = read_u16(DNSHeaderParseError::InvalidNQuestions, bitstream)?;
    let n_answers: u16 = read_u16(DNSHeaderParseError::InvalidNAnswers, bitstream)?;
    let n_authority_rrs: u16 = read_u16(DNSHeaderParseError::InvalidNAuthorityRRs, bitstream)?;
    let n_additional_rrs: u16 = read_u16(DNSHeaderParseError::InvalidNAdditionalRRs, bitstream)?;

    Ok(DNSHeaders {
        transaction_id,
        flags,
        n_questions,
        n_answers,
        n_authority_rrs,
        n_additional_rrs,
    })
}

pub fn parse_body(
    headers: DNSHeaders,
    bitstream: &mut Bytes<T>,
) -> Result<DNSRequest, DNSBodyParseError> {
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use super::*;

    fn make_bitstream(bytes: &[u8]) -> Bytes<BufReader<Cursor<&[u8]>>> {
        BufReader::new(Cursor::new(bytes)).bytes()
    }

    fn show_binary(byte_array: &[u8]) -> String {
        byte_array
            .into_iter()
            .map(|x| format!("{x:b}"))
            .fold("".into(), |acc, x| format!("{acc} {x}"))
    }

    #[test]
    fn test_parse_header_normal() {
        let raw_header: [u8; 12] = [
            0x1a, 0x2b, // Transaction ID
            0x01, 0x00, // Flags (standard query, recursion desired)
            0x00, 0x01, // Questions: 1
            0x00, 0x00, // Answer RRs: 0
            0x00, 0x00, // Authority RRs: 0
            0x00, 0x00, // Additional RRs: 0
        ];

        let mut bitstream = make_bitstream(&raw_header);

        if let Ok(result) = parse_header(&mut bitstream) {
            assert_eq!(result.transaction_id, 0x1a2b);
            assert!(result.flags.is_query);
            assert_eq!(result.flags.opcode, OpCode::Query);
            assert!(!result.flags.is_authoritative);
            assert!(!result.flags.truncated);
            assert!(result.flags.recursion_desired);
            assert!(!result.flags.recursion_available);
            assert!(!result.flags.zero);
            assert!(!result.flags.authentic_data);
            assert!(!result.flags.checking_disabled);
            assert_eq!(result.flags.rcode, ResponseCode::NoError);

            assert_eq!(result.n_questions, 1);
            assert_eq!(result.n_answers, 0);
            assert_eq!(result.n_authority_rrs, 0);
            assert_eq!(result.n_additional_rrs, 0);
        } else {
            panic!("Parsing a normal header should succeed");
        }
    }

    #[test]
    fn test_header_to_bytes() {
        let expected_raw: [u8; 12] = [
            0xbe, 0xef, // Transaction ID
            0x85, 0x80, // Flags (response, AA=1, RD=1, RA=1, NoError)
            0x00, 0x01, // Questions: 1
            0x00, 0x01, // Answer RRs: 1
            0x00, 0x00, // Authority RRs: 0
            0x00, 0x00, // Additional RRs: 0
        ];
        let original = DNSHeaders {
            transaction_id: 0xbeef,
            flags: DNSHeaderFlags {
                is_query: false,
                opcode: OpCode::Query,
                is_authoritative: true,
                truncated: false,
                recursion_desired: true,
                recursion_available: true,
                zero: false,
                authentic_data: false,
                checking_disabled: false,
                rcode: ResponseCode::NoError,
            },
            n_questions: 1,
            n_answers: 1,
            n_authority_rrs: 0,
            n_additional_rrs: 0,
        };

        let output_raw = original.to_bytes().collect::<Vec<u8>>();
        assert_eq!(
            output_raw,
            expected_raw,
            "The correct binary representation should be generated.
                    Expected: {0}
                    Actual  : {1}",
            show_binary(&expected_raw),
            show_binary(&output_raw)
        )
    }
}
