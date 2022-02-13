// Copyright 2022 Kali Prasad <kprasadvnsi@protonmail.com>
// SPDX-License-Identifier: MIT

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.PackageKit")]
trait PackageKit {
    /// CanAuthorize method
    fn can_authorize(&self, action_id: &str) -> zbus::Result<u32>;

    /// CreateTransaction method
    fn create_transaction(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetDaemonState method
    fn get_daemon_state(&self) -> zbus::Result<String>;

    /// GetPackageHistory method
    fn get_package_history(
        &self,
        names: &[&str],
        count: u32,
    ) -> zbus::Result<
        std::collections::HashMap<
            String,
            Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>,
        >,
    >;

    /// GetTimeSinceAction method
    fn get_time_since_action(&self, role: u32) -> zbus::Result<u32>;

    /// GetTransactionList method
    fn get_transaction_list(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// SetProxy method
    fn set_proxy(
        &self,
        proxy_http: &str,
        proxy_https: &str,
        proxy_ftp: &str,
        proxy_socks: &str,
        no_proxy: &str,
        pac: &str,
    ) -> zbus::Result<()>;

    /// StateHasChanged method
    fn state_has_changed(&self, reason: &str) -> zbus::Result<()>;

    /// SuggestDaemonQuit method
    fn suggest_daemon_quit(&self) -> zbus::Result<()>;

    /// RepoListChanged signal
    #[dbus_proxy(signal)]
    fn repo_list_changed(&self) -> zbus::Result<()>;

    /// RestartSchedule signal
    #[dbus_proxy(signal)]
    fn restart_schedule(&self) -> zbus::Result<()>;

    /// TransactionListChanged signal
    // #[dbus_proxy(signal)]
    // fn transaction_list_changed(&self, transactions: &[&str]) -> zbus::Result<()>;

    /// UpdatesChanged signal
    #[dbus_proxy(signal)]
    fn updates_changed(&self) -> zbus::Result<()>;

    /// BackendAuthor property
    #[dbus_proxy(property)]
    fn backend_author(&self) -> zbus::Result<String>;

    /// BackendDescription property
    #[dbus_proxy(property)]
    fn backend_description(&self) -> zbus::Result<String>;

    /// BackendName property
    #[dbus_proxy(property)]
    fn backend_name(&self) -> zbus::Result<String>;

    /// DistroId property
    #[dbus_proxy(property)]
    fn distro_id(&self) -> zbus::Result<String>;

    /// Filters property
    #[dbus_proxy(property)]
    fn filters(&self) -> zbus::Result<u64>;

    /// Groups property
    #[dbus_proxy(property)]
    fn groups(&self) -> zbus::Result<u64>;

    /// Locked property
    #[dbus_proxy(property)]
    fn locked(&self) -> zbus::Result<bool>;

    /// MimeTypes property
    #[dbus_proxy(property)]
    fn mime_types(&self) -> zbus::Result<Vec<String>>;

    /// NetworkState property
    #[dbus_proxy(property)]
    fn network_state(&self) -> zbus::Result<u32>;

    /// Roles property
    #[dbus_proxy(property)]
    fn roles(&self) -> zbus::Result<u64>;

    /// VersionMajor property
    #[dbus_proxy(property)]
    fn version_major(&self) -> zbus::Result<u32>;

    /// VersionMicro property
    #[dbus_proxy(property)]
    fn version_micro(&self) -> zbus::Result<u32>;

    /// VersionMinor property
    #[dbus_proxy(property)]
    fn version_minor(&self) -> zbus::Result<u32>;
}

#[dbus_proxy(interface = "org.freedesktop.PackageKit.Offline")]
trait Offline {
    /// Cancel method
    fn cancel(&self) -> zbus::Result<()>;

    /// ClearResults method
    fn clear_results(&self) -> zbus::Result<()>;

    /// GetPrepared method
    fn get_prepared(&self) -> zbus::Result<Vec<String>>;

    /// Trigger method
    fn trigger(&self, action: &str) -> zbus::Result<()>;

    /// TriggerUpgrade method
    fn trigger_upgrade(&self, action: &str) -> zbus::Result<()>;

    /// PreparedUpgrade property
    #[dbus_proxy(property)]
    fn prepared_upgrade(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// TriggerAction property
    #[dbus_proxy(property)]
    fn trigger_action(&self) -> zbus::Result<String>;

    /// UpdatePrepared property
    #[dbus_proxy(property)]
    fn update_prepared(&self) -> zbus::Result<bool>;

    /// UpdateTriggered property
    #[dbus_proxy(property)]
    fn update_triggered(&self) -> zbus::Result<bool>;

    /// UpgradePrepared property
    #[dbus_proxy(property)]
    fn upgrade_prepared(&self) -> zbus::Result<bool>;

    /// UpgradeTriggered property
    #[dbus_proxy(property)]
    fn upgrade_triggered(&self) -> zbus::Result<bool>;
}
