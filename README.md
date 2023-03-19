# Hexagonal rust
> Building hexagoanl arch with rust by following [this blog link](https://alexis-lozano.com/hexagonal-architecture-in-rust-1/)

Hexagonal architecture, onion architecture, clean architecture... are quite the same thing, so I'll refer to hexagonal architecture from now on.

- hexagoanl arch pros
    - you can change the domain without changing the dependencies
    - you can change the dependencies without changing the domain
    - you can easily test the domain
    - you can think about which technical dependencies to use when the need comes, and not at the very start of your implementation

## Business Logic

- Create a NFT
- Read all NFTs
- Read one NFT
- Delete a NFT


## setup

```bash
> cargo new nft
```