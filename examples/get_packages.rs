// Copyright 2022 Kali Prasad <kprasadvnsi@protonmail.com>
// SPDX-License-Identifier: MIT

extern crate packagekit;
use futures::stream::StreamExt;
use packagekit::{PackageKitProxy, TransactionProxy};

fn main() -> zbus::Result<()> {
    futures::executor::block_on(async move {
        let connection = zbus::Connection::system().await?;

        let destination: &str = "org.freedesktop.PackageKit";

        let pk = PackageKitProxy::new(&connection).await?;
        let handle = pk.create_transaction().await?;
    
        let transcation = TransactionProxy::builder(&connection)
                                                        .destination(destination)?
                                                        .path(handle)?
                                                        .build()
                                                        .await?;
        
        let mut stream = transcation.receive_package().await?;
        
        while let Some(pkg) = stream.next().await {
            let args = pkg.args()?;

            println!("{:?}", args);
            
        }

        transcation.get_packages(0).await?;
        
        Ok(())
    })
}