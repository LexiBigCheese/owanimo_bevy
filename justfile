check_no_dynamic_linking:
    #!/usr/bin/env bash
    if cargo tree -e features | grep "├── bevy feature \"dynamic_linking\"" > /dev/null; then
        echo "disable bevy dynamic_linking!"
        exit 1
    fi
    exit 0

create_dist_asset_folder:
    #!/usr/bin/env bash
    rm -r dist_assets 2>/dev/null
    mkdir dist_assets 2>/dev/null
    ASSET_FILES=()
    ASSET_FILES+=("bignuisancepuyo.glb")
    ASSET_FILES+=("redpuyo.glb")
    ASSET_FILES+=("greenpuyo.glb")
    ASSET_FILES+=("bluepuyo.glb")
    ASSET_FILES+=("yellowpuyo.glb")
    ASSET_FILES+=("purplepuyo.glb")
    ASSET_FILES+=("nuisancepuyo.glb")
    for ass in "${ASSET_FILES[@]}"; do
        cp "assets/$ass" "dist_assets/$ass"
    done


create_dist_folder: create_dist_asset_folder
    #!/usr/bin/env bash
    rm -r dist 2>/dev/null
    mkdir dist 2>/dev/null
    cp -r dist_assets dist/assets
    true

create_www_dist_folder: create_dist_asset_folder
    #!/usr/bin/env bash
    rm -r www_dist 2>/dev/null
    cp -r www www_dist
    cp -r dist_assets www_dist/assets
    true

release_win: check_no_dynamic_linking
    cargo xwin build --target x86_64-pc-windows-msvc --release

release_linux_glibc_2_36: check_no_dynamic_linking
    cargo zigbuild --target x86_64-unknown-linux-gnu.2.36 --release

release_wasm: check_no_dynamic_linking
    cargo build --profile wasm-release --target wasm32-unknown-unknown

dist_wasm: create_www_dist_folder release_wasm
    #!/usr/bin/env bash
    wasm-bindgen --out-dir ./www_dist/out --target web ./target/wasm32-unknown-unknown/wasm-release/owanimo_bevy.wasm
    for file in ./www_dist/out/*.wasm; do
        wasm-opt -Os -o wasm-opted.wasm ${file} && mv wasm-opted.wasm ${file}
    done
    rm owanimo_bevy_web.tar.xz 2>/dev/null
    tar -cJf owanimo_bevy_web.tar.xz www_dist/

publish_wasm: dist_wasm
    #!/usr/bin/env bash
    rm -rf /tmp/owanimo_bevy_web_dist 2>/dev/null
    ORIGINAL_ORIGIN=`git remote get-url origin`
    git clone $ORIGINAL_ORIGIN /tmp/owanimo_bevy_web_dist
    WWW_DIST_DIR=`pwd`/www_dist
    cd /tmp/owanimo_bevy_web_dist
    git checkout -b gh-pages
    git pull
    rm -rf *
    cp -r $WWW_DIST_DIR/* .
    git add -A
    git commit -m "web publish"
    git push -f --set-upstream origin gh-pages


dist: check_no_dynamic_linking create_dist_folder release_win release_linux_glibc_2_36
    #!/usr/bin/env bash
    cp target/x86_64-unknown-linux-gnu/release/owanimo_bevy dist/owanimo_bevy
    rm owanimo_bevy_linux_glibc_2_36.tar.xz 2>/dev/null
    tar -cJf owanimo_bevy_linux_glibc_2_36.tar.xz dist/
    rm dist/owanimo_bevy
    cp target/x86_64-pc-windows-msvc/release/owanimo_bevy.exe dist/owanimo_bevy.exe
    rm owanimo_bevy_windows.tar.xz 2>/dev/null
    tar -cJf owanimo_bevy_windows.tar.xz dist/
