extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        println!("Running task from `winres` to set icon and manifest.");
        let mut res = winres::WindowsResource::new();
        res.set_icon("./assets/icon.ico");
        res.set_manifest(
            r#"
            <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
                <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
                    <security>
                        <requestedPrivileges>
                            <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
                        </requestedPrivileges>
                    </security>
                </trustInfo>
                <application xmlns="urn:schemas-microsoft-com:asm.v3">
                    <windowsSettings>
                        <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true</dpiAware>
                        <longPathAware xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">true</longPathAware>
                    </windowsSettings>
                </application>
            </assembly>
            "#,
        );
        res.compile().unwrap();

        println!("Running `static_vcruntime::metabuild()`.");
        static_vcruntime::metabuild();
    }
}
