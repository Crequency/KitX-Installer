# Usage

## Arguments

- `--patch`: Path of file to patch
- `--from`: Content to replace
- `--to`: Content to replace with

## Samples

This patch version id to `v3.23.04`:

```shell
cargo run -- --patch 'kitx_installer_egui.exe' --from '$$_!_%Version%_@_$$                                        #' --to "v3.23.04"
```

This patch build id to `win-x64-single`:

```shell
cargo run -- --patch 'kitx_installer_egui.exe' --from '$$_!_%Profile%_@_$$                                        #' --to "win-x64-single"
```



