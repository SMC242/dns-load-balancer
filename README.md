# Implementation plan

- [ ] Create resolver to learn about how the protocol works
  - [ ] Create parsers for all messages
  - [ ] Implement iterative resolver
  - [ ] Implement caching, enabling non-recursive queries
- [ ] Create name server
  - [ ] Implement cache
  - [ ] Expose an API for changing an entry for the next step
    - This can't be publicly accessible for obvious security reasons
- [ ] Create load-balancing logic
  - [ ] Health tracking
  - [ ] Distributing requests
- [ ] Use name server to opaquely change where requests will be routed to according to the load balancing logic

## Enhancements

- [ ] Use Redis for more robust caching
- [ ] Support IDN
- [ ] Support reverse lookup
