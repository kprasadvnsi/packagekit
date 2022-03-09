// Copyright 2022 Kali Prasad <kprasadvnsi@protonmail.com>
// SPDX-License-Identifier: MIT

extern crate packagekit;

use packagekit::{PackageKitProxy, TransactionProxy};
use futures::{
    future::FutureExt,
    stream::StreamExt,
    pin_mut,
};

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
        
        let mut package_stream = transcation.receive_package().await?;
        let mut progress_stream = transcation.receive_item_progress().await?;
        let mut finish_stream = transcation.receive_finished().await?;

        transcation.remove_packages(0, &vec!["blender;2.83.5+dfsg-5build1;amd64;ubuntu-hirsute-universe"], true, true).await?;

        loop {
            let package = package_stream.next().fuse();
            let progress = progress_stream.next().fuse();
            let finish = finish_stream.next().fuse();
            
            pin_mut!(package, progress, finish);

            futures_util::select! {
                pkg_s = package => {
                    if let Some(pkg) = pkg_s {
                        let args = pkg.args()?;
                        println!("Package: {:?}", args);
                    }
                },
                progress_s = progress => {
                    if let Some(prgs) = progress_s {
                        let args = prgs.args()?;
                        println!("Progress: {:?}", args);
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