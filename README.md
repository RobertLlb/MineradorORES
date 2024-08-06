
## Disclaimer 👷‍♂️

This code is experimental and provided "as is", without warranty of any kind. Use it at your own risk. It's under development, and its features may change. Always back up your data before use. Contributions and feedback are welcome.

The script automates the setup and running of the Ore CLI for $ORE mining operations on Solana Mainnet, ensuring you're always running the latest version and simplifying the mining setup process.

ORE program address `oreV2ZymfyeXgNgBdqMkumTqqAprVqgBWQfoYkrtKWQ`

$ORE (V2) token address `oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp`

$ORE (V1) token address `oreoN2tQbHXVaZsr3pf66A48miqcBXCDJozganhEJgz` - mining paused, migrate to v2 if you have

## Quick Start 🛠️

To get started with the Ore Miner Auto Script, clone this repository to your local machine using the following command:

```bash
git clone https://github.com/nodecattel/oreminer.git
cd oreminer
```

### Prerequisites ✅

Before running the script, ensure you have the following installed:

- Rust and Cargo (The script will attempt to install these if they're not present)
- Solana CLI (Also installed by the script if not present)
- (Recommended) Private RPC endpoints for fast & secure mining hash submission. You can use our referal link for [Quicknode here](https://www.quicknode.com/?via=nodecattel)

### Give permission for script become executable

```bash
chmod +x install.sh ore.sh
```

### Installation 💻

Navigate to the cloned repository directory and run the `install.sh` script to set up the necessary components:

```bash
./install.sh
```

Follow the on-screen instructions to complete the installation. The script will:

- Install Rust and Cargo
- Install the Solana CLI
- Install and update the Ore CLI to the latest version
- Set executable permissions for `ore.sh`
- Optionally run `ore.sh` for further setup

### Generating Keypair 🔑

To generate a new keypair for Solana, use the following command:

```bash
solana-keygen new --outfile ~/.config/solana/id.json
```

### Running Ore Miner ⛏️

(Devnet) Get SOL testnet by running this command:

```bash
solana config set --url d
solana airdrop 1
```
Try again if there is an error with rate limit.

After installation, fund your wallet and you can start the mining process by running:

```bash
./ore.sh mine
```

See all available commands:

```bash
./ore.sh
```

### Change default variable config manually

To change your settings, please set up in `ore.conf` located at `$HOME/.ore`

```bash
cd $HOME/.ore
nano ore.conf
```

### Export Private key to external wallet

```bash
cat $HOME/.config/solana/id.json
```

Then copy the output to your external wallet like Phantom, Backpack.

### Useful Links

Ore Cli's creates:

```
https://crates.io/crates/ore-cli
```

Official Ore-Cli repository:

```
https://github.com/regolith-labs/ore-cli
```

Dune $ORE PoW Mining on Solana Mainnet:

```
https://dune.com/rawrmaan/ore-mining-solana
```

`$ORE` Price chart:

```
https://birdeye.so/token/oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp?chain=solana
```

If you find value in the scripts here, please [Follow NodeCattel on X](https://twitter.com/nodecattel)
