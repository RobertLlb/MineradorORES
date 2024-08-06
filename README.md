
# ORE CLI

A command line interface for ORE cryptocurrency mining.

## Prerequisites

- [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

## Installation

### Clone the Repository

To get started, clone the repository to your local machine:

```sh
git clone https://github.com/RobertLlb/MineradorORES.git
cd MineradorORES
```

### Build the Project

Build the project using Cargo:

```sh
cargo build --release
```

### Configuration

Ensure you have your Solana keypair JSON file ready. The default path used in the commands is `~/.config/solana/id.json`. You can adjust this path if your keypair file is located elsewhere.

## Running the Miner

To start mining, use the following command:

```sh
./target/release/ore mine --rpc-urls "https://ancient-broken-water.solana-mainnet.quiknode.pro/5669cad306938292d1864f0c088f745375080780/" "https://morning-dimensional-shard.solana-mainnet.quiknode.pro/7e7249d6717acec5588bf7e295120e86aaa59979/" --keypair "~/.config/solana/id.json" --priority-fee 1000 --threads 4 --buffer-time 5
```

### Command Line Options

You can use the `-h` flag on any command to pull up a help menu with documentation:

```sh
./target/release/ore -h
```

## VPS Setup

### Step 1: Access the VPS

Use SSH to access your VPS:

```sh
ssh user@your_vps_ip
```

### Step 2: Install Dependencies on the VPS

Update the packages:

```sh
sudo apt update
sudo apt upgrade -y
```

Install Git and other necessary dependencies:

```sh
sudo apt install git build-essential -y
```

Install Rust:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Step 3: Clone the Repository and Build the Project

Clone your repository:

```sh
git clone https://github.com/RobertLlb/MineradorORES.git
cd MineradorORES
```

Build the project:

```sh
cargo build --release
```

### Step 4: Run the Miner on the VPS

Run the miner with the desired configuration:

```sh
./target/release/ore mine --rpc-urls "https://ancient-broken-water.solana-mainnet.quiknode.pro/5669cad306938292d1864f0c088f745375080780/" "https://morning-dimensional-shard.solana-mainnet.quiknode.pro/7e7249d6717acec5588bf7e295120e86aaa59979/" --keypair "~/.config/solana/id.json" --priority-fee 1000 --threads 4 --buffer-time 5
```

### Optional: Automate the Execution with `systemd`

#### Create a Service File

Create a new service file:

```sh
sudo nano /etc/systemd/system/ore_miner.service
```

Add the following content (adjust as necessary):

```ini
[Unit]
Description=ORE Miner

[Service]
ExecStart=/caminho/completo/para/ore mine --rpc-urls "https://ancient-broken-water.solana-mainnet.quiknode.pro/5669cad306938292d1864f0c088f745375080780/" "https://morning-dimensional-shard.solana-mainnet.quiknode.pro/7e7249d6717acec5588bf7e295120e86aaa59979/" --keypair "~/.config/solana/id.json" --priority-fee 1000 --threads 4 --buffer-time 5
Restart=always
User=seu_usuario

[Install]
WantedBy=multi-user.target
```

#### Enable and Start the Service

Reload `systemd` to recognize the new service:

```sh
sudo systemctl daemon-reload
```

Enable the service to start automatically on boot:

```sh
sudo systemctl enable ore_miner
```

Start the service:

```sh
sudo systemctl start ore_miner
```

#### Check the Status of the Service

Verify the status of the service to ensure it is running:

```sh
sudo systemctl status ore_miner
```

## Troubleshooting

If you encounter any issues, please refer to the following steps:

1. Ensure all dependencies are installed correctly.
2. Verify your Solana keypair file is correctly configured.
3. Check the service logs using `sudo journalctl -u ore_miner` if using `systemd`.

For additional support, please open an issue on the [GitHub repository](https://github.com/RobertLlb/MineradorORES/issues).
