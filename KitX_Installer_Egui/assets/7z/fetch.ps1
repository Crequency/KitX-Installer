# $template = "<base>7z(r|<version>)(-<platform>)?(-<profile>)?.(exe|tar.xz)";

# $base = "https://www.7-zip.org/a/";
# $version = "2301";

# $platform = New-Object -TypeName 'System.Collections.ArrayList';
# $platform.Add("linux")
# $platform.Add("mac")

# $profile = New-Object -TypeName 'System.Collections.ArrayList';
# $profile.Add("x86")
# $profile.Add("x64")
# $profile.Add("arm")
# $profile.Add("arm64")

$links = New-Object -TypeName 'System.Collections.ArrayList';
$links.Add("https://www.7-zip.org/a/7zr.exe")
$links.Add("https://www.7-zip.org/a/7z2301-linux-x86.tar.xz")
$links.Add("https://www.7-zip.org/a/7z2301-linux-x64.tar.xz")
$links.Add("https://www.7-zip.org/a/7z2301-linux-arm.tar.xz")
$links.Add("https://www.7-zip.org/a/7z2301-linux-arm64.tar.xz")
$links.Add("https://www.7-zip.org/a/7z2301-mac.tar.xz")

$targets = New-Object -TypeName 'System.Collections.ArrayList';

foreach ($link in $links) {
    $target = $link.Substring($link.LastIndexOf('/') + 1)
    $targets.Add($target)

    Invoke-WebRequest -Uri $link -OutFile $target
}

foreach ($target in $targets) {
    if ($IsWindows) {
        & .\7zr.exe e $target
    } else {
        7za e $target
    }
    if ($target.EndsWith(".tar.xz")) {
        $target_2 = $target.Substring(0, $target.LastIndexOf('.'))
        $exe = ""

        if ($target.Contains("mac")) {
            $exe = "7zz"
        } else {
            $exe = "7zzs"
        }

        tar -xvzf $target_2 $exe
        mv $exe $target_2.Replace(".tar", "").Replace("7z2301", $exe)
        Remove-Item $target_2
    }
}

foreach ($target in $targets) {
    if ($target.Contains(".exe")) {

    } else {
        Remove-Item $target
    }
}
