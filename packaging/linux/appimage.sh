#!/usr/bin/env bash
# Packages a built gallery binary as an AppImage with linuxdeploy, which
# also bundles the shared libraries it links (minus the ones every desktop
# has, like glibc and the GPU drivers).
#
#   packaging/linux/appimage.sh <binary> <app-id> <display-name> <out-dir>
#   packaging/linux/appimage.sh target/release/gallery-rsx gpuicn-rsx "gpuicn-rsx gallery" dist
#
# Set LINUXDEPLOY to a linuxdeploy AppImage to use it instead of downloading one.
set -euo pipefail

bin=$1 id=$2 name=$3 out=$4
here=$(cd "$(dirname "$0")" && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

tool="${LINUXDEPLOY:-$work/linuxdeploy-x86_64.AppImage}"
if [ ! -x "$tool" ]; then
    curl -fsSL -o "$tool" https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
    chmod +x "$tool"
fi

mkdir -p "$work/AppDir/usr/bin" "$out"
install -m755 "$bin" "$work/AppDir/usr/bin/$id"
cat > "$work/$id.desktop" <<DESKTOP
[Desktop Entry]
Type=Application
Name=$name
Comment=Every gpuicn component: shadcn/ui for gpui
Exec=$id
Icon=$id
Categories=Development;
Terminal=false
DESKTOP
cp "$here/$id.png" "$work/$id.png"

# CI runners and containers have no FUSE: run linuxdeploy extracted.
export APPIMAGE_EXTRACT_AND_RUN=1
(cd "$work" && OUTPUT="$id-x86_64.AppImage" "$tool" --appdir AppDir \
    --executable "AppDir/usr/bin/$id" --desktop-file "$id.desktop" --icon-file "$id.png" --output appimage)
mv "$work/$id-x86_64.AppImage" "$out/"
echo "$out/$id-x86_64.AppImage"
