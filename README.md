## Mint sniper
_Having issues? Contact Querty#0001 on Discord_

### Installation

##### Requirements
- [Rust and Cargo](https://win.rustup.rs/)
- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

Then, run the command bellow to install the sniper:

```bash
cargo install --git https://git.enzofoucaud.fr/the-diggers-dao/mint-sniper.git
cd mint-sniper && cp .env.example .env
```

Replace the values in the `.env` file with your own.
- `ALCHEMY_URL` is your [Alchemy](https://www.alchemy.com/) API key
- `PRIVATE_KEY` is your wallet's private key
- `ADDRESS` is your wallet's address

### Usage
```bash
mint-sniper --help
```
```