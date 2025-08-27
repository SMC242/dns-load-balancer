#![allow(clippy::upper_case_acronyms)]

pub enum QRFlag {
    Query = 0,
    Reply = 1,
}
pub enum OpCode {
    /// Standard query
    Query = 0,
    /// Inverse query
    IQuery = 1,
    /// Server status request
    Status = 2,
}

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
    /// Whether the message is a query or a reply
    qr: QRFlag,
    // 4 bits
    /// The type of operation
    opcode: OpCode,
    /// Whether the answer is authoritative. Only used in responses
    aa: bool,
    /// Whether the message was truncated
    tc: bool,
    /// Whether the client wants a recursive query
    rd: bool,
    /// Zero. Reserved for future use. Always 0
    z: bool,
    /// If the replying DNS server verified the data. Only used in responses
    ad: bool,
    /// If non-verified data is allowed in responses. Only used in queries
    cd: bool,
    // 4 bits
    /// The response code
    rcode: ResponseCode,
}

pub struct DNSHeader {
    transaction_id: u16,
    flags: DNSHeaderFlags,
    n_questions: u16,
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

pub enum DNSHeaderParseError {}

pub fn parse_header(bitstream: std::io::Bytes<u8>) -> Result<DNSHeader, DNSHeaderParseError> {
    todo!("Implement parse_header")
}
