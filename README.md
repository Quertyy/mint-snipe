[![status-badge](https://ci.enzofoucaud.fr/api/badges/the-diggers-dao/mint-snipe/status.svg?branch=<branch>)](https://ci.enzofoucaud.fr/the-diggers-dao/mint-snipe)

## Mint sniper
_Having issues? Contact Querty#0001 on Discord_

### Installation

##### Requirements
- [Rust and Cargo](https://win.rustup.rs/)
- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

Then, run the command bellow to install the sniper:

```bash
cargo install --git https://git.enzofoucaud.fr/the-diggers-dao/mint-sniper.git
cd mint-sniper && cp .env.example .env && cp src/abi/abi.example.json src/abi/abi.json
```

Replace the values in the `.env` file with your own.
- `ALCHEMY_URL` is your [Alchemy](https://www.alchemy.com/) API key
- `PRIVATE_KEY` is your wallet's private key
- `ADDRESS` is your wallet's address

Copy the contract's abi in the file `src/abi.json`.

### Usage
Example
```bash
cargo run -- --contract-address 0x9601482fD8EacE73745Dd7353e17c19F4Ce9142d --mint-method mint --price 1000000000000000 --amount 2 --timestamp 1676816130
```