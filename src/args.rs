use crate::PkgMan;
use clap::{Parser, Subcommand};

pub fn action() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Update => declarch::update(),
        Commands::Rebuild => declarch::rebuild(),

        Commands::Pacman { action } => match action {
            PacmanAction::Add { packages } => declarch::add_packages(&packages, PkgMan::Pacman),
            PacmanAction::Remove { packages } => declarch::rm_packages(&packages, PkgMan::Pacman),
        },

        Commands::Aur { action } => match action {
            AurAction::Add { packages } => declarch::add_packages(&packages, PkgMan::Aur),
            AurAction::Remove { packages } => declarch::rm_packages(&packages, PkgMan::Aur),
        },

        Commands::Flatpak { action } => match action {
            FlatpakAction::Add { packages } => declarch::add_packages(&packages, PkgMan::Flatpak),
            FlatpakAction::Remove { packages } => declarch::rm_packages(&packages, PkgMan::Flatpak),

            FlatpakAction::Override { action } => match action {
                FlatpakOverride::Add { target, flag } => {
                    declarch::add_flatpak_override(&target, &flag)
                }
                FlatpakOverride::Remove { target, flag } => {
                    declarch::rm_flatpak_override(&target, &flag)
                }
            },

            FlatpakAction::Remote { action } => match action {
                FlatpakRemote::Add { name, url } => declarch::add_flatpak_remote(&name, &url),
                FlatpakRemote::Remove { name, url } => declarch::rm_flatpak_remote(&name, &url),
            },
        },

        Commands::Systemd { action } => match action {
            SystemdAction::Enable { owner, service } => {
                declarch::enable_systemd_service(&owner, &service)
            }
            SystemdAction::Disable { owner, service } => {
                declarch::disable_systemd_service(&owner, &service)
            }
            SystemdAction::Mask { owner, service } => {
                declarch::mask_systemd_service(&owner, &service)
            }
        },
    }
}

#[derive(Parser)]
#[command(name = "dcl")]
#[command(version, about = "Declarative Arch Linux")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
enum PacmanAction {
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
enum AurAction {
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
enum FlatpakAction {
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
enum FlatpakOverride {
    /// Add Override
    Add {
        /// Target [possible values: global, any_package_id]
        target: String,
        /// A flatpak override flag
        flag: String,
    },
    /// Remove Override
    Remove {
        /// Target [possible values: global, any_package_id]
        target: String,
        /// A flatpak override flag
        flag: String,
    },
}

#[derive(Subcommand)]
enum FlatpakRemote {
    /// Add Override
    Add {
        /// Name of the remote
        name: String,
        /// Url of the remote
        url: String,
    },
    /// Remove Override
    Remove {
        /// Name of the remote
        name: String,
        /// Url of the remote
        url: String,
    },
}

#[derive(Subcommand)]
enum SystemdAction {
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
