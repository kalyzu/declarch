use clap::{Parser, Subcommand};
use declarch::ConfigFile;

pub fn action() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Update => declarch::update(),
        Commands::Rebuild => declarch::rebuild(),
        Commands::Init => declarch::init(),

        Commands::Pacman { action } => match action {
            PacmanAction::Add { packages } => {
                declarch::add_packages(&packages, ConfigFile::Pacman, None)
            }
            PacmanAction::Rm { packages } => declarch::rm_packages(&packages, ConfigFile::Pacman),
        },

        Commands::Aur { action } => match action {
            AurAction::Add { packages } => declarch::add_packages(&packages, ConfigFile::Aur, None),
            AurAction::Rm { packages } => declarch::rm_packages(&packages, ConfigFile::Aur),
        },

        Commands::Flatpak { action } => match action {
            FlatpakAction::Add { remote, packages } => {
                declarch::add_packages(&packages, ConfigFile::Flatpak, Some(&remote))
            }
            FlatpakAction::Rm { packages } => declarch::rm_packages(&packages, ConfigFile::Flatpak),

            FlatpakAction::Override { action } => match action {
                FlatpakOverride::Add { target, flag } => {
                    declarch::add_flatpak_override(&target, &flag)
                }
                FlatpakOverride::Rm { target, flag } => {
                    declarch::rm_flatpak_override(&target, &flag)
                }
            },

            FlatpakAction::Remote { action } => match action {
                FlatpakRemote::Add { name, url } => declarch::add_flatpak_remote(&name, &url),
                FlatpakRemote::Rm { name, url } => declarch::rm_flatpak_remote(&name, &url),
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
    /// Initialize with starter config files
    #[command(alias = "i")]
    Init,

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
    Rm {
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
    Rm {
        /// Package(s) to remove
        packages: Vec<String>,
    },
}

#[derive(Subcommand)]
enum FlatpakAction {
    /// Add packages
    Add {
        /// Remote to use
        remote: String,
        /// Package(s) to install
        packages: Vec<String>,
    },

    /// Remove packages
    Rm {
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
    Rm {
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
    Rm {
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
