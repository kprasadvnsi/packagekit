// Copyright 2022 Kali Prasad <kprasadvnsi@protonmail.com>
// SPDX-License-Identifier: MIT

extern crate packagekit;
use futures::{future::FutureExt, pin_mut, stream::StreamExt};
use packagekit::{PackageKitProxy, TransactionProxy};

fn main() -> zbus::Result<()> {
    futures::executor::block_on(async move {
        let connection = zbus::Connection::system().await?;

        let destination: &str = "org.freedesktop.PackageKit";

        let pk = PackageKitProxy::new(&connection).await?;
        let handle = pk.create_transaction().await?;
        //let mut stream = TransactionProxy::receive_package().await?;
        let transcation = TransactionProxy::builder(&connection)
            .destination(destination)?
            .path(handle)?
            .build()
            .await?;

        let mut package_stream = transcation.receive_package().await?;
        let mut finish_stream = transcation.receive_finished().await?;

        transcation.get_packages(0).await?;
        loop {
            let package = package_stream.next().fuse();
            let finish = finish_stream.next().fuse();

            pin_mut!(package, finish);

            futures_util::select! {
                pkg_s = package => {
                    if let Some(pkg) = pkg_s {
                        let args = pkg.args()?;
                        println!("Package: {:?}", args);
                    }
                },
                finish_s = finish => {
                    if let Some(finished) = finish_s {
                        let args = finished.args()?;
                        println!("Finish: {:?}", args);
                        break;
                    }
                },
            };
        }

        Ok(())
    })
}
