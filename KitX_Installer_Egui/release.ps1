cargo build --release
Copy-Item './target/release/kitx_installer_egui.exe' './target/release/kitx_installer_egui_lzma-compressed.exe'
upx --best --lzma './target/release/kitx_installer_egui_lzma-compressed.exe'
