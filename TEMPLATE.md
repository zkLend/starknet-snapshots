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
rclone copy -P zklend-pathfinder-backup:EXAMPLE_NETWORK/EXAMPLE_FILE_NAME .
```

### `curl`

Downloading with `curl` is not recommended but possible:

```console
curl -OL "https://pathfinder-backup.zklend.com/EXAMPLE_NETWORK/EXAMPLE_FILE_NAME"
```

## Using the snapshots

Simply decompress and extract the file downloaded with `tar`:

```console
tar Jxvf ./EXAMPLE_FILE_NAME
```

LIST_OF_SNAPSHOTS
