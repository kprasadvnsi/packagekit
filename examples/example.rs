// Copyright 2022 Kali Prasad <kprasadvnsi@protonmail.com>
// SPDX-License-Identifier: MIT

extern crate packagekit;

use packagekit::PackageKitProxy;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
}
