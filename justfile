check_no_dynamic_linking:
    #!/usr/bin/env bash
    if cargo tree -e features | grep "├── bevy feature \"dynamic_linking\"" > /dev/null; then
        echo "disable bevy dynamic_linking!"
        exit 1
    fi
    exit 0

create_dist_folder:
    #!/usr/bin/env bash
    mkdir dist 2>/dev/null
    mkdir dist/assets 2>/dev/null
    ASSET_FILES=()
    ASSET_FILES+=("bignuisancepuyo.glb")
    ASSET_FILES+=("redpuyo.glb")
    ASSET_FILES+=("greenpuyo.glb")
    ASSET_FILES+=("bluepuyo.glb")
    ASSET_FILES+=("yellowpuyo.glb")
    ASSET_FILES+=("purplepuyo.glb")
    ASSET_FILES+=("nuisancepuyo.glb")
    for ass in "${ASSET_FILES[@]}"; do
        cp "assets/$ass" "dist/assets/$ass"
    done
    rm dist/owanimo_bevy 2>/dev/null
    rm dist/owanimo_bevy.exe 2>/dev/null
    true

release_win: check_no_dynamic_linking
    cargo xwin build --target x86_64-pc-windows-msvc --release

release_linux_glibc_2_36: check_no_dynamic_linking
    cargo zigbuild --target x86_64-unknown-linux-gnu.2.36 --release

dist: check_no_dynamic_linking create_dist_folder release_win release_linux_glibc_2_36
    #!/usr/bin/env bash
    cp target/x86_64-unknown-linux-gnu/release/owanimo_bevy dist/owanimo_bevy
    tar -cJf owanimo_bevy_linux_glibc_2_36.tar.xz dist/
    rm dist/owanimo_bevy
    cp target/x86_64-pc-windows-msvc/release/owanimo_bevy.exe dist/owanimo_bevy.exe
    tar -cJf owanimo_bevy_windows.tar.xz dist/
