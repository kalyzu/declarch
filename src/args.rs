use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dcl")]
#[command(version, about = "Declarative Arch Linux")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Update all groups and packages
    #[command(alias = "u")]
    Update,

    /// Rebuild the system from the config
    #[command(alias = "r")]
    Rebuild,

    /// Manage pacman packages
    #[command(alias = "p")]
    Pacman {
        #[command(subcommand)]
        action: PacmanAction,
    },

    /// Manage AUR packages
    #[command(alias = "a")]
    Aur {
        #[command(subcommand)]
        action: AurAction,
    },

    /// Manage flatpak packages
    #[command(alias = "f")]
    Flatpak {
        #[command(subcommand)]
        action: FlatpakAction,
    },

    /// Manage systemd services
    #[command(alias = "s")]
    Systemd {
        #[command(subcommand)]
        action: SystemdAction,
    },
}

#[derive(Subcommand)]
pub enum PacmanAction {
    /// Add packages
    Add {
        /// Package(s) to install
        packages: Vec<String>,
    },

    /// Remove packages
    Remove {
        /// Package(s) to remove
        packages: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum AurAction {
    /// Add packages
    Add {
        /// Package(s) to install
        packages: Vec<String>,
    },

    /// Remove packages
    Remove {
        /// Package(s) to remove
        packages: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum FlatpakAction {
    /// Add packages
    Add {
        /// Package(s) to install
        packages: Vec<String>,
    },

    /// Remove packages
    Remove {
        /// Package(s) to remove
        packages: Vec<String>,
    },

    /// Manage overrides
    Override {
        #[command(subcommand)]
        action: FlatpakOverride,
    },

    /// Manage remotes
    Remote {
        #[command(subcommand)]
        action: FlatpakRemote,
    },
}

#[derive(Subcommand)]
pub enum FlatpakOverride {
    /// Add Override
    Add {
        /// Target [possible values: global, any_package_id]
        target: String,
        /// A flatpak override flag
        flag: String,
    },
    /// Remove Override
    Rm {
        /// Target [possible values: global, any_package_id]
        target: String,
        /// A flatpak override flag
        flag: String,
    },
}

#[derive(Subcommand)]
pub enum FlatpakRemote {
    /// Add Override
    Add {
        /// Name of the remote
        name: String,
        /// Url of the remote
        url: String,
    },
    /// Remove Override
    Rm {
        /// Name of the remote
        name: String,
        /// Url of the remote
        url: String,
    },
}

#[derive(Subcommand)]
pub enum SystemdAction {
    /// Enable
    Enable {
        /// Owner of service
        #[arg(value_parser = ["user", "system"])]
        owner: String,
        /// Service (example: bluetooth.service)
        service: String,
    },
    /// Disable
    Disable {
        /// Owner of service
        #[arg(value_parser = ["user", "system"])]
        owner: String,
        /// Service (example: bluetooth.service)
        service: String,
    },
    /// Mask
    Mask {
        /// Owner of service
        #[arg(value_parser = ["user", "system"])]
        owner: String,
        /// Service (example: bluetooth.service)
        service: String,
    },
}
