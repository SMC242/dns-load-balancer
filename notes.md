# Name server vs resolver

- A name-server holds the mappings between DNS zones and IP addresses
  - It will typically cache these (with a TTL)
- A resolver is the client - it asks each name server until it reaches the lowermost DNS zone

## Authoritative name servers

- Name servers that own the domain (I.E they don't query it from another name server)
- All DNS zones have an authoritative name server
  - Typically two for passive replication

# Resolver queries

3 types of queries can be issued by resolvers:

1. Iterative: the resolver traverses the chain of name servers until it reaches the bottommost DNS zone
2. Non-recursive: the resolver queries an authoritative name server or returns a partial result from its cache
3. Recursive: the resolver delegates to a DNS server which will query other DNS servers
   - The DNS server does all of the work of resolving

# Messages

- DNS is traditionally built on UDP but can use TCP

## Parsing

- Each segment of a resource record is preceeded by a single byte representing the length of the segment
  - Example for image.google.com: 5image6google3com

# TLDs

- The list of TLDs can be found [here](https://data.iana.org/TLD/tlds-alpha-by-domain.txt)
