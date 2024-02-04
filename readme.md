## Kibane

Meaning `feather` in japanese. Simplified implementation of Celestia Light Node in rust for PERSONAL LEARNING PURPOSE. Heavily Heavily Heavily inspired by [Lumina](https://github.com/eigerco/lumina)

### Background Knowledge

- libp2p : Because light client is syncing from other trusted peers, to understand this project higly recommend to learn about how does libp2p handles inter communication between peers. I made [tutorial](https://github.com/rkdud007/gossip-p2p-tutorial-rs).

### Quick Start

```bash
# install kibane
cargo install --path cli

# run kibane node
kibane --network mocha

# check out help for more configuration options
kibane --help
```
