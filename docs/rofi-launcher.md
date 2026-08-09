# Launching from rofi

How to make productiviCLI show up when you search in rofi, and open in its own
kitty window.

Written for Arch + Hyprland + kitty + rofi.

## Setup (do this once)

### 1. Install the binary

```bash
SQLX_OFFLINE=true cargo install --path . --force
```

This builds the app and puts a copy at `~/.cargo/bin/productivicli`.

`SQLX_OFFLINE=true` tells sqlx to use the saved query info in `.sqlx/` instead
of connecting to the database while it builds.

### 2. Put your database settings where the launcher can see them

```bash
mkdir -p ~/.config/productivicli
ln -sf "$PWD/.env" ~/.config/productivicli/env
```

The app normally reads `.env` from the folder you run it in. rofi doesn't run
it from the project folder, so it looks here instead.

A symlink means you only ever edit `.env` and both stay in sync. If you'd
rather have a real copy, use `install -m 600 .env ~/.config/productivicli/env`.

### 3. Add the launcher entry

```bash
mkdir -p ~/.local/share/applications
sed "s|@BIN@|$HOME/.cargo/bin/productivicli|" dist/productivicli.desktop.in \
  > ~/.local/share/applications/productivicli.desktop
update-desktop-database ~/.local/share/applications
```

This is the file rofi reads to know the app exists.

### 4. Try it

Open rofi (`rofi -show drun`) and type `productivicli`.

## Keeping it up to date

The launcher runs a **copy** of the app, not the code in this folder. So after
you change any Rust code, run this again:

```bash
SQLX_OFFLINE=true cargo install --path . --force
```

That's the whole update process. Steps 2 and 3 don't need repeating.

If you forget, nothing breaks — you'll just be running the older version.

## Doing it all at once

`dist/install.sh` runs steps 1-3 for you.

## Optional: make the window float

Add to `~/.config/hypr/hyprland.conf`:

```
windowrulev2 = float, class:^(productivicli)$
windowrulev2 = size 900 600, class:^(productivicli)$
windowrulev2 = center, class:^(productivicli)$
```

## If something goes wrong

**It doesn't show up in rofi.** Check the file for typos:

```bash
desktop-file-validate ~/.local/share/applications/productivicli.desktop
```

**The window opens and instantly closes.** The app crashed. Run it by hand to
see the error:

```bash
kitty -e ~/.cargo/bin/productivicli
```

Usually this means it can't find your database URL — check step 2.
