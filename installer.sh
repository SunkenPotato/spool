#!/usr/bin/sh

exec_dir="$HOME/.local/bin"
asset_dir="$HOME/.local/share/spool"

platform=$(uname -s)

premature_exit() {
    rm "$exec_dir/spool"
    rm -rf "$asset_dir"
}

err_exit() {
    echo "Error: $1" >&2
    premature_exit
    exit 1
}

linux() {
    which spool > /dev/null
    if [ $? -eq 0 ]; then
        echo "Spool was detected on this system. Do you want to [u]ninstall, [r]einstall it, or [q]uit the installation?"
        read -p "[u/r/q]: " choice
        case "$choice" in
            u) uninstall_linux ;;
            r) ;;
            q) exit 0 ;;
            *) echo "Invalid choice. Installation aborted."; exit 1 ;;
        esac
    fi

    trap premature_exit INT TERM

    echo "Do you want to continue? [y/N]"
    read -p "[y/N]: " choice
    if [ "$choice" != "y" ]; then
        echo "Installation aborted."
        exit 0
    fi

    echo "Building binary..."
    cargo build --release --quiet --bin cli > /dev/null || err_exit "Failed to build spool"

    echo "Copying binary..."
    cp target/release/cli "$exec_dir/spool"
    cp -r assets "$asset_dir"

    echo "Cleaning up..."
    cargo clean
}

uninstall_linux() {
    echo "Uninstalling spool..."
    rm "$exec_dir/spool"
    rm -rf "$asset_dir"
    exit 0
}

macos() {
    echo "macOS is not supported yet. Do you want to try using the linux install script?"
    read -p "[y/N]" choice
    if [ "$choice" = "y" ]; then
        linux
    else
        echo "Installation aborted."
    fi
}

echo "Welcome to the spool installer."

if [ "$platform" = "Linux" ]; then
    linux
elif [ "$platform" = "Darwin" ]; then
    macos
else
    echo "Unsupported platform: $platform"
    exit 1
fi
