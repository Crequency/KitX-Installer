#!/usr/bin/bash

# $1: Version
# $2: Profile

cd '../KitX_Installer_Egui/'

rm './target/release/kitx_installer_egui'
rm './target/release/topatch'

cargo build --release

cp './target/release/kitx_installer_egui' './target/release/topatch'

cd ../KitX_Installer_Egui_Patcher/

cargo run -- --patch '../KitX_Installer_Egui/target/release/topatch' --from '$$_!_%Version%_@_$$                                        #' --to "$1"
cargo run -- --patch '../KitX_Installer_Egui/target/release/topatch' --from '$$_!_%Profile%_@_$$                                        #' --to "$2"

echo "Patched with version: $1 and profile: $2 !"
