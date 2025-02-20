exec_dir="$HOME/.local/bin"
asset_dir="$HOME/.local/share/spool"

platform=$(uname -s)

premature_exit() {
    rm -f "$exec_dir/spool"
    rm -rf "$asset_dir"
}

err_exit() {
    echo "Error: $1" >&2
    premature_exit
    exit 1
}

# Linux functions
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
    mkdir -p "$exec_dir"
    cp target/release/cli "$exec_dir/spool"
    mkdir -p "$asset_dir"
    cp -r assets "$asset_dir"

    echo "Cleaning up..."
    cargo clean

    echo "Installation complete!"
    echo "Please add '$exec_dir' to your PATH environment variable."
    echo "You can do this by adding the following line to your ~/.bashrc or ~/.zshrc (depending on your shell) file:"
    echo "export PATH=\"\$PATH:$exec_dir\""
    echo "Then, either restart your terminal or source the appropriate file (e.g., source ~/.bashrc or source ~/.zshrc)."
}

uninstall_linux() {
    echo "Uninstalling spool..."
    rm -f "$exec_dir/spool"
    rm -rf "$asset_dir"
    exit 0
}


# macOS functions
macos() {
    which spool > /dev/null
    if [ $? -eq 0 ]; then
        echo "Spool was detected on this system. Do you want to [u]ninstall, [r]einstall it, or [q]uit the installation?"
        read -p "[u/r/q]: " choice
        case "$choice" in
            u) uninstall_macos ;;
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
    mkdir -p "$exec_dir"
    cp target/release/cli "$exec_dir/spool"
    mkdir -p "$asset_dir"
    cp -r assets "$asset_dir"

    echo "Cleaning up..."
    cargo clean

    echo "Installation complete!"
    echo "Please add '$exec_dir' to your PATH environment variable."
    echo "You can do this by adding the following line to your ~/.zshrc (if using zsh) or ~/.bash_profile (if using bash):"
    echo "export PATH=\"\$PATH:$exec_dir\""
    echo "Then, either restart your terminal or source the appropriate file (e.g., source ~/.zshrc or source ~/.bash_profile)."
}

uninstall_macos() {
    echo "Uninstalling spool..."
    rm -f "$exec_dir/spool"
    rm -rf "$asset_dir"
    exit 0
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
