// Copyright 2022 Kali Prasad <kprasadvnsi@protonmail.com>
// SPDX-License-Identifier: MIT

extern crate packagekit;
use packagekit::PackageKitProxy;

fn main() -> zbus::Result<()> {
    futures::executor::block_on(async move {
        let connection = zbus::Connection::system().await?;

        let pk = PackageKitProxy::new(&connection).await?;

        println!(
            "Server version: {}.{}.{}",
            pk.version_major().await?,
            pk.version_minor().await?,
            pk.version_micro().await?
        );
        println!(
            "Backend: {} {}",
            pk.backend_description().await?,
            pk.backend_name().await?
        );
        println!("Supported MIME types:{:#?}", pk.mime_types().await?);

        Ok(())
    })
}
