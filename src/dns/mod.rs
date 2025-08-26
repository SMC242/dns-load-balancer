use std::collections::HashMap;
use std::net::IpAddr;

type UnixTimestamp = i32;

struct DNSConfig {
    ttl: u64,
}

struct RegularAuthority {
    host_name: String,
    ip_address: IpAddr,
    parent: Box<Authority>,
    public_key: String,
}

struct RootAuthority {
    host_name: String,
    ip_address: IpAddr,
    public_key: String,
}

enum Authority {
    RegularAuthority(RegularAuthority),
    RootAuthority(RootAuthority),
}

struct DNSRecord {
    authority: Authority,
    host_name: String,
    ip_address: IpAddr,
}

// In-memory for now. TODO: use Redis for persistence
struct DNSCache {
    size: usize,
    ttl: u64,
    table: HashMap<String, (UnixTimestamp, DNSRecord)>,
}

struct DNSServer {
    options: DNSConfig,
    table: DNSCache,
}
