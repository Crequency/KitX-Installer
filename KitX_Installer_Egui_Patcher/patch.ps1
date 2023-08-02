
param($version, $profile)

cd '../KitX_Installer_Egui/'

rm './target/release/kitx_installer_egui.exe'
rm './target/release/topatch.exe'

cargo build --release

cp './target/release/kitx_installer_egui.exe' './target/release/topatch.exe'

cd '../KitX_Installer_Egui_Patcher/'

cargo run -- --patch '../KitX_Installer_Egui/target/release/topatch.exe' --from '$$_!_%Version%_@_$$                                        #' --to "$version"
cargo run -- --patch '../KitX_Installer_Egui/target/release/topatch.exe' --from '$$_!_%Profile%_@_$$                                        #' --to "$profile"

echo "Patched with version: $version and profile: $profile !"
