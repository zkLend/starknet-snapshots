<p align="center">
  <h1 align="center">starknet-snapshots</h1>
</p>

**Starknet full node snapshots maintained by the [zkLend](https://zklend.com) team**

## Downloading the snapshots

There are several ways to download the snapshots. You're recommended to use S3-compatible clients (e.g. `rclone`). It's also possible to just use a plain `curl` command, but it seems to be prone to failures based on reports from users.

### Rclone

Download and install [Rclone](https://rclone.org/). Then, add this section to your config file, typically located at `$HOME/.config/rclone/rclone.conf` (you may need to create the file on a fresh install):

```conf
[zklend-pathfinder-backup]
type = s3
provider = Cloudflare
region = auto
endpoint = https://pathfinder-backup.zklend.com/
```

Then, download a snapshot to the current directory with:

```console
rclone copy -P zklend-pathfinder-backup:mainnet/mainnet-v0.8.1-213169.tar.xz .
```

### `curl`

Downloading with `curl` is not recommended but possible:

```console
curl -OL "https://pathfinder-backup.zklend.com/mainnet/mainnet-v0.8.1-213169.tar.xz"
```

## Using the snapshots

Simply decompress and extract the file downloaded with `tar`:

```console
tar Jxvf ./mainnet-v0.8.1-213169.tar.xz
```

## `pathfinder`

### `mainnet`

| Network   | Version  | Block    | Download Link                                                             |
| --------- | -------- | -------- | ------------------------------------------------------------------------- |
| `mainnet` | `v0.8.1` | `213169` | https://pathfinder-backup.zklend.com/mainnet/mainnet-v0.8.1-213169.tar.xz |
| `mainnet` | `v0.7.0` | `141083` | https://pathfinder-backup.zklend.com/mainnet/mainnet-v0.7.0-141083.tar.xz |

### `goerli-1`

| Network    | Version  | Block    | Download Link                                                               |
| ---------- | -------- | -------- | --------------------------------------------------------------------------- |
| `goerli-1` | `v0.8.1` | `860674` | https://pathfinder-backup.zklend.com/goerli-1/goerli-1-v0.8.1-860674.tar.xz |
| `goerli-1` | `v0.7.0` | `841817` | https://pathfinder-backup.zklend.com/goerli-1/goerli-1-v0.7.0-841817.tar.xz |

### `goerli-2`

| Network    | Version  | Block    | Download Link                                                               |
| ---------- | -------- | -------- | --------------------------------------------------------------------------- |
| `goerli-2` | `v0.8.1` | `141801` | https://pathfinder-backup.zklend.com/goerli-2/goerli-2-v0.8.1-141801.tar.xz |
| `goerli-2` | `v0.7.0` | `136651` | https://pathfinder-backup.zklend.com/goerli-2/goerli-2-v0.7.0-136651.tar.xz |
