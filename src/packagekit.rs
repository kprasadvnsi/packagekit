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

#[dbus_proxy(interface = "org.freedesktop.PackageKit.Transaction")]
trait Transaction {
    /// AcceptEula method
    fn accept_eula(&self, eula_id: &str) -> zbus::Result<()>;

    /// Cancel method
    fn cancel(&self) -> zbus::Result<()>;

    /// DependsOn method
    fn depends_on(&self, filter: u64, package_ids: &[&str], recursive: bool) -> zbus::Result<()>;

    /// DownloadPackages method
    fn download_packages(&self, store_in_cache: bool, package_ids: &[&str]) -> zbus::Result<()>;

    /// GetCategories method
    fn get_categories(&self) -> zbus::Result<()>;

    /// GetDetails method
    fn get_details(&self, package_ids: &[&str]) -> zbus::Result<()>;

    /// GetDetailsLocal method
    fn get_details_local(&self, files: &[&str]) -> zbus::Result<()>;

    /// GetDistroUpgrades method
    fn get_distro_upgrades(&self) -> zbus::Result<()>;

    /// GetFiles method
    fn get_files(&self, package_ids: &[&str]) -> zbus::Result<()>;

    /// GetFilesLocal method
    fn get_files_local(&self, files: &[&str]) -> zbus::Result<()>;

    /// GetOldTransactions method
    fn get_old_transactions(&self, number: u32) -> zbus::Result<()>;

    /// GetPackages method
    fn get_packages(&self, filter: u64) -> zbus::Result<()>;

    /// GetRepoList method
    fn get_repo_list(&self, filter: u64) -> zbus::Result<()>;

    /// GetUpdateDetail method
    fn get_update_detail(&self, package_ids: &[&str]) -> zbus::Result<()>;

    /// GetUpdates method
    fn get_updates(&self, filter: u64) -> zbus::Result<()>;

    /// InstallFiles method
    fn install_files(&self, transaction_flags: u64, full_paths: &[&str]) -> zbus::Result<()>;

    /// InstallPackages method
    fn install_packages(&self, transaction_flags: u64, package_ids: &[&str]) -> zbus::Result<()>;

    /// InstallSignature method
    fn install_signature(&self, sig_type: u32, key_id: &str, package_id: &str) -> zbus::Result<()>;

    /// RefreshCache method
    fn refresh_cache(&self, force: bool) -> zbus::Result<()>;

    /// RemovePackages method
    fn remove_packages(
        &self,
        transaction_flags: u64,
        package_ids: &[&str],
        allow_deps: bool,
        autoremove: bool,
    ) -> zbus::Result<()>;

    /// RepairSystem method
    fn repair_system(&self, transaction_flags: u64) -> zbus::Result<()>;

    /// RepoEnable method
    fn repo_enable(&self, repo_id: &str, enabled: bool) -> zbus::Result<()>;

    /// RepoRemove method
    fn repo_remove(
        &self,
        transaction_flags: u64,
        repo_id: &str,
        autoremove: bool,
    ) -> zbus::Result<()>;

    /// RepoSetData method
    fn repo_set_data(&self, repo_id: &str, parameter: &str, value: &str) -> zbus::Result<()>;

    /// RequiredBy method
    fn required_by(&self, filter: u64, package_ids: &[&str], recursive: bool) -> zbus::Result<()>;

    /// Resolve method
    fn resolve(&self, filter: u64, packages: &[&str]) -> zbus::Result<()>;

    /// SearchDetails method
    fn search_details(&self, filter: u64, values: &[&str]) -> zbus::Result<()>;

    /// SearchFiles method
    fn search_files(&self, filter: u64, values: &[&str]) -> zbus::Result<()>;

    /// SearchGroups method
    fn search_groups(&self, filter: u64, values: &[&str]) -> zbus::Result<()>;

    /// SearchNames method
    fn search_names(&self, filter: u64, values: &[&str]) -> zbus::Result<()>;

    /// SetHints method
    fn set_hints(&self, hints: &[&str]) -> zbus::Result<()>;

    /// UpdatePackages method
    fn update_packages(&self, transaction_flags: u64, package_ids: &[&str]) -> zbus::Result<()>;

    /// UpgradeSystem method
    fn upgrade_system(
        &self,
        transaction_flags: u64,
        distro_id: &str,
        upgrade_kind: u32,
    ) -> zbus::Result<()>;

    /// WhatProvides method
    fn what_provides(&self, filter: u64, values: &[&str]) -> zbus::Result<()>;

    /// Category signal
    #[dbus_proxy(signal)]
    fn category(
        &self,
        parent_id: &str,
        cat_id: &str,
        name: &str,
        summary: &str,
        icon: &str,
    ) -> zbus::Result<()>;

    /// Destroy signal
    #[dbus_proxy(signal)]
    fn destroy(&self) -> zbus::Result<()>;

    /// Details signal
    #[dbus_proxy(signal)]
    fn details(
        &self,
        data: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// DistroUpgrade signal
    #[dbus_proxy(signal)]
    fn distro_upgrade(&self, type_: u32, name: &str, summary: &str) -> zbus::Result<()>;

    /// ErrorCode signal
    #[dbus_proxy(signal)]
    fn error_code(&self, code: u32, details: &str) -> zbus::Result<()>;

    /// EulaRequired signal
    #[dbus_proxy(signal)]
    fn eula_required(
        &self,
        eula_id: &str,
        package_id: &str,
        vendor_name: &str,
        license_agreement: &str,
    ) -> zbus::Result<()>;

    /// Files signal
    //#[dbus_proxy(signal)]
    //fn files(&self, package_id: &str, file_list: &[&str]) -> zbus::Result<()>;

    /// Finished signal
    #[dbus_proxy(signal)]
    fn finished(&self, exit: u32, runtime: u32) -> zbus::Result<()>;

    /// ItemProgress signal
    #[dbus_proxy(signal)]
    fn item_progress(&self, id: &str, status: u32, percentage: u32) -> zbus::Result<()>;

    /// MediaChangeRequired signal
    #[dbus_proxy(signal)]
    fn media_change_required(
        &self,
        media_type: u32,
        media_id: &str,
        media_text: &str,
    ) -> zbus::Result<()>;

    /// Package signal
    #[dbus_proxy(signal)]
    fn package(&self, info: u32, package_id: &str, summary: &str) -> zbus::Result<()>;

    /// RepoDetail signal
    #[dbus_proxy(signal)]
    fn repo_detail(&self, repo_id: &str, description: &str, enabled: bool) -> zbus::Result<()>;

    /// RepoSignatureRequired signal
    #[dbus_proxy(signal)]
    fn repo_signature_required(
        &self,
        package_id: &str,
        repository_name: &str,
        key_url: &str,
        key_userid: &str,
        key_id: &str,
        key_fingerprint: &str,
        key_timestamp: &str,
        type_: u32,
    ) -> zbus::Result<()>;

    /// RequireRestart signal
    #[dbus_proxy(signal)]
    fn require_restart(&self, type_: u32, package_id: &str) -> zbus::Result<()>;

    /// Transaction signal
    #[dbus_proxy(signal)]
    fn transaction(
        &self,
        object_path: zbus::zvariant::ObjectPath<'_>,
        timespec: &str,
        succeeded: bool,
        role: u32,
        duration: u32,
        data: &str,
        uid: u32,
        cmdline: &str,
    ) -> zbus::Result<()>;

    /// UpdateDetail signal
    // #[dbus_proxy(signal)]
    // fn update_detail(
    //     &self,
    //     package_id: &str,
    //     updates: &[&str],
    //     obsoletes: &[&str],
    //     vendor_urls: &[&str],
    //     bugzilla_urls: &[&str],
    //     cve_urls: &[&str],
    //     restart: u32,
    //     update_text: &str,
    //     changelog: &str,
    //     state: u32,
    //     issued: &str,
    //     updated: &str,
    // ) -> zbus::Result<()>;

    /// AllowCancel property
    #[dbus_proxy(property)]
    fn allow_cancel(&self) -> zbus::Result<bool>;

    /// CallerActive property
    #[dbus_proxy(property)]
    fn caller_active(&self) -> zbus::Result<bool>;

    /// DownloadSizeRemaining property
    #[dbus_proxy(property)]
    fn download_size_remaining(&self) -> zbus::Result<u64>;

    /// ElapsedTime property
    #[dbus_proxy(property)]
    fn elapsed_time(&self) -> zbus::Result<u32>;

    /// LastPackage property
    #[dbus_proxy(property)]
    fn last_package(&self) -> zbus::Result<String>;

    /// Percentage property
    #[dbus_proxy(property)]
    fn percentage(&self) -> zbus::Result<u32>;

    /// RemainingTime property
    #[dbus_proxy(property)]
    fn remaining_time(&self) -> zbus::Result<u32>;

    /// Role property
    #[dbus_proxy(property)]
    fn role(&self) -> zbus::Result<u32>;

    /// Speed property
    #[dbus_proxy(property)]
    fn speed(&self) -> zbus::Result<u32>;

    /// Status property
    #[dbus_proxy(property)]
    fn status(&self) -> zbus::Result<u32>;

    /// TransactionFlags property
    #[dbus_proxy(property)]
    fn transaction_flags(&self) -> zbus::Result<u64>;

    /// Uid property
    #[dbus_proxy(property)]
    fn uid(&self) -> zbus::Result<u32>;
}
