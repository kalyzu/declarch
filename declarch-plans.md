# declarch plans

*kalyzu 10-22-26*

---

declarch will use toml.

it'll have a bunch of different commands.

```shell
# add package(s)
dcl pacman add niri
dcl aur add niri
dcl flatpak add flathub com.niri.niri
# or the short versions
dcl p add niri
dcl a add niri
dcl f add flathub com.niri.niri
# replace add with rm to remove the package(s)
dcl pacman add niri
dcl aur rm niri
dcl flatpak rm com.niri.niri
# just list all the ones you want added or removed
dcl pacman add niri yazi micro
dcl pacman rm niri yazi micro

# Adds pacman package group
dcl pacman add gnome
# brings up this TUI
Select packages to add from group "gnome"
[x] gnome-shell
[x] gnome-weather

# updates groups and updates pacman, aur, and flatpak
dcl update

# systemd
dcl systemd user mask avahi-daemon.service
dcl systemd system disable bluetooth.service

# flatpak
dcl flatpak override add global "--filesystem=/home/arch-desktop"
dcl flatpak override add "app.drey.Haystack" "--filesystem=/home/arch-desktop"
# rm keyword works here too
dcl flatpak override rm global "--filesystem=/home/arch-desktop"
# add/rm remotes
dcl flatpak remote add flathub "https://flathub.org"
dcl flatpak remote rm flathub "https://flathub.org"

# If you update your config manually, run
dcl rebuild
```

when you add a package or remove one, it'll use the pacman flag to skip extra prompts because declarch will have already prompted you.

When you remove a pacman or aur package, pacman -Rns will be used. When removing a flatpak, declarch will remove the app folder in `~/.var/app/com.foo.bar`

the config will look like

```toml
# system.toml
[meta]
hostname = "arch-desktop"
```

```toml
# pacman.toml
[pacman]
packages = [
    "niri",
    "yazi",
    "micro",
]
[pacman.groups.gnome]
packages = [
    "gnome-shell",
    "gnome-weather",
]
```

```toml
# aur.toml
[aur]
helper = "paru" # options: paru/yay
packages = [
    "turbowarp-bin",
    "whatever-git",
]
[aur.groups.custom-fonts]
packages = [
    "ttf-ms-fonts",
    "nerd-fonts-inter"
]
```

```toml
# flatpak.toml
[flatpak]
global_overrides = [
    "--filesystem=xdg-config/gtk-3.0:ro",
    "--env=GTK_THEME=Adwaita-dark"
]

remotes = [
    { name = "flathub", url = "https://flathub.org" },
    { name = "flathub-beta", url = "https://flathub.org" }
]

# All packages
packages = [
    "app.drey.Haystack",
    "org.turbowarp.TurboWarp",
]

# Special overrides: permissions or different remotes
[[app_override]]
id = "app.drey.Haystack"
remote = "flathub" # defaults to flathub if omitted
overrides = [
    "--filesystem=/home/arch-desktop/Documents"
]

[[app_override]]
id = "org.turbowarp.TurboWarp"
remote = "flathub" # defaults to flathub if omitted
overrides = [
    "--filesystem=/home/arch-desktop/Downloads"
]
```

```toml
# systemd.toml
[system]
enabled = [
    "docker.service",
    "bluetooth.service",
]

disabled = [
    "avahi-daemon.service",
]

masked = [
    "avahi-daemon.service",
]

[user]
enabled = [
    "bar.service",
    "foobar.service",
]

disabled = [
    "foo.service",
]

masked = [
    "foo.service",
]
```
