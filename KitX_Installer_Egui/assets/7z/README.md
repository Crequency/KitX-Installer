# 7z (console version 2301):

## Platforms with 7z

linux:
  - x86: 7zzs 3.74 MB
  - arm: 7zzs 1.84 MB
macos:
  - arm64 / x86-64: 7zz 5.38 MB
windows:
  - all: 7zr.exe (x86) 571 KB

file name:
    when target on linux or macos, 7z provides `7zz`, `7zzs` two executable files.
    - `7zz` is a dynamic executable file, ~~it requires `libgcc_s.so.1` and `libstdc++.so.6` to run~~.
    - `7zzs` is a static executable file, it does not require any other files to run.

download links:
  - template
    - `%base%`7z(r|`%version%`)#p1(windows)(-`%platform%`)?#not(windows)(-`%profile%`)?#and(not(windows),not(windows)).(exe|tar.xz)#p1(windows)
    - `%base%`: https://www.7-zip.org/a/
    - `%version%`: 2301
    - `%platform%`: [linux, mac]
    - `%profile%`: [x86, x64, arm, arm64]
  - windows
    - https://www.7-zip.org/a/7zr.exe
  - linux
    - https://www.7-zip.org/a/7z2301-linux-x86.tar.xz
    - https://www.7-zip.org/a/7z2301-linux-x64.tar.xz
    - https://www.7-zip.org/a/7z2301-linux-arm.tar.xz
    - https://www.7-zip.org/a/7z2301-linux-arm64.tar.xz
  - macos
    - https://www.7-zip.org/a/7z2301-mac.tar.xz

## Usage

Just run `./fetch.ps1` in this directory.
