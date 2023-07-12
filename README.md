# KitX Installer

This project is part of KitX Project.
This project aims to provide installation programs for softwares in KitX Project.

## KitX Installer Egui

This is a rust program with egui as UI framework.
When it runs on Windows, it will display ui framework.
When it runs on Linux of MacOS, it will use terminal to install.
But you can always use `--run-gui` or `--run-cli` to force it to run in GUI or CLI mode.

### Build

You need prepare rust tool chain first.
Visit [rust-lang.org](https://www.rust-lang.org/) to get more information.

#### Clone

Run following commands in your terminal to get source code:

```shell
git clone git@github.com:Crequency/KitX-Installer.git
cd './KitX_Installer/KitX_Installer_Egui'
```

#### Run

Run following command in your terminal to run this program:

```shell
cargo run
```

#### Build

Run following command in your terminal to build this program in release mode:

```shell
cargo build --release
```

## KitX Installer Egui Patcher

This is a rust program aims to patch KitX Installer Egui in order to quickly generate different installers.

Now we patch two things:
- Version of KitX Dashboard to be install.
- Profile of KitX Dashboard to be install.

See details in [README.md](./KitX_Installer_Egui_Patcher/README.md) .


