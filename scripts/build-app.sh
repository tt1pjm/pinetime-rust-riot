#!/usr/bin/env bash
#  macOS and Linux Bash script to build Rust application hosted on RIOT OS

set -e  #  Exit when any command fails

set -x  #  Echo commands
build_app=pinetime
#  build_app=terminal_display
rust_build_target=thumbv7em-none-eabihf
launch_config=launch-nrf52.json
# TODO: For Raspberry Pi: launch_config=launch-nrf52-pi.json
set +x  #  Stop echo
echo ; echo "----- Building Rust app and RIOT OS for $rust_build_target / $build_app..." 

#  Rust build profile: debug or release
rust_build_profile=debug
#  rust_build_profile=release

#  Add toolchain to PATH.
#  export PATH="$PWD/xPacks/riscv-none-embed-gcc/8.2.0-3.1/bin:$PATH"

#  Location of the compiled ROM image.  We will remove this to force relinking the Rust app with RIOT OS.
app_build=$PWD/apps/$build_app/bin/$build_app/PineTime.elf

#  Location of the compiled Rust app and external libraries.  The Rust compiler generates a *.rlib archive for the Rust app and each external Rust library here.
rust_build_dir=$PWD/target/$rust_build_target/$rust_build_profile/deps

#  Location of the Rust Application Library to be linked with RIOT OS
rust_app_dir=$PWD/apps/$build_app/bin/$build_app
rust_app_dest=$rust_app_dir/rust_app.a

#  Location of the Rust Core Library to be linked with RIOT OS
rust_libcore_dir=$rust_app_dir
rust_libcore_dest=$rust_libcore_dir/rust_libcore.a

#  Rust build options
rust_build_options="--target $rust_build_target"
if [ "$rust_build_profile" == 'release' ]; then
    # Build for release
    rust_build_options="--release $rust_build_options"
#  else 
    # Build for debug: No change in options
fi

#  Set build commands for the architecture
if [ "$rust_build_target" == 'riscv32imac-unknown-none-elf' ]; then
    # RISC-V build commands
    ar_cmd=riscv-none-embed-ar
    readelf_cmd=riscv-none-embed-readelf
    objdump_cmd=riscv-none-embed-objdump
else 
    # Arm build commands
    ar_cmd=arm-none-eabi-ar
    readelf_cmd=arm-none-eabi-readelf
    objdump_cmd=arm-none-eabi-objdump
fi

#  Copy debugger launch config
cp .vscode/$launch_config .vscode/launch.json

function build_riot() {
    #  Build the RIOT OS application
    local build_app=$1     # RIOT OS app to be built e.g. apps/pinetime
    set -x  #  Echo commands
    pushd apps/$build_app
    make -s --jobs=10  # --trace
    popd
    set +x  #  Stop echo
}

#  If this is the very first build, do the RIOT OS build to generate the rust_app and rust_libcore stubs.  This build will not link successfully but it's OK.
# if [ ! -e $rust_app_dest ]; then
#     echo ; echo "----- Build RIOT OS stubs for Rust app and Rust libcore (ignore error)"
#     set +e
#     set -x
#     build_riot $build_app
#     set +x
#     set -e
# fi

#  Delete the compiled ROM image to force the RIOT OS build to relink the Rust app with RIOT OS.
if [ -e $app_build ]; then
    rm $app_build
fi

#  Delete the compiled Rust app to force the Rust build to relink the Rust app.  Sometimes there are multiple copies of the compiled app, this deletes all copies.
rust_app_build=$rust_build_dir/libapp*.rlib
for f in $rust_app_build
do
    if [ -e $f ]; then
        rm $f
    fi
done

#  Expand Rust macros for troubleshooting: logs/liblvgl-expanded.rs and libapp-expanded.rs
# set +e  # Ignore errors
# pushd rust/lvgl ; cargo rustc $rust_build_options -- -Z unstable-options --pretty expanded > ../../logs/liblvgl-expanded.rs ; popd
# pushd rust/app  ; cargo rustc $rust_build_options -- -Z unstable-options --pretty expanded > ../../logs/libapp-expanded.rs  ; popd
# set -e  # Stop on errors

#  Build the Rust app in "src" folder.
echo ; echo "----- Build Rust app" 
set -x
cargo build $rust_build_options
set +x

#  Export the metadata for the Rust build.
cargo metadata --format-version 1 >logs/libapp.json

#  Create rustlib, the library that contains the compiled Rust app and its dependencies (except libcore).  Create in temp folder named "tmprustlib"
echo ; echo "----- Consolidate Rust app and crates"
if [ -d tmprustlib ]; then
    rm -r tmprustlib
fi
if [ ! -d tmprustlib ]; then
    mkdir tmprustlib
fi
pushd tmprustlib >/dev/null

#  Extract the object (*.o) files in the compiled Rust output (*.rlib).
rust_build=$rust_build_dir/*.rlib
for f in $rust_build
do
    if [ -e $f ]; then
        #  echo "$ar_cmd x $f"
        $ar_cmd x $f >/dev/null 2>&1
    fi
done

#  Archive the object (*.o) files into rustlib.a.
#  echo "$ar_cmd r rustlib.a *.o"
$ar_cmd r rustlib.a *.o >/dev/null 2>&1

#  Copy rustlib.a to the RIOT OS build.
if [ ! -d $rust_app_dir ]; then
    mkdir -p $rust_app_dir
fi
set -x
cp $PWD/rustlib.a $rust_app_dest
set +x

#  Update the timestamp on libs_rust_app.a so that RIOT OS build won't overwrite the Rust app we have copied.
$ar_cmd s $rust_app_dest

#  Dump the ELF and disassembly for the compiled Rust application and libraries (except libcore)
#  $objdump_cmd -t -S            --line-numbers --wide rustlib.a >../logs/rustlib.S 2>&1
#  $objdump_cmd -t -S --demangle --line-numbers --wide rustlib.a >../logs/rustlib-demangle.S 2>&1

#  Return to the parent directory.
popd >/dev/null

#  Copy Rust Core Library to the RIOT OS build.
echo ; echo "----- Copy Rust libcore" 
#  Get the Rust compiler sysroot e.g. /Users/Luppy/.rustup/toolchains/nightly-2019-05-22-x86_64-apple-darwin
rust_sysroot=`rustc --print sysroot --target $rust_build_target`
#  Get the libcore file in the sysroot.
rust_libcore_src=$rust_sysroot/lib/rustlib/$rust_build_target/lib/libcore-*.rlib
#  Copy libcore to the RIOT OS build folder.
if [ ! -d $rust_libcore_dir ]; then
    mkdir -p $rust_libcore_dir
fi
if [ -e $rust_libcore_dest ]; then
    rm $rust_libcore_dest
fi
for f in $rust_libcore_src
do
    set -x
    cp $f $rust_libcore_dest
    set +x
done

#  Update the timestamp on libs_rust_libcore.a so that RIOT OS build won't overwrite the Rust libcore we have copied.
$ar_cmd s $rust_libcore_dest

#  Dump the ELF and disassembly for the compiled Rust application.
#  set +e
#  $readelf_cmd -a --wide target/$rust_build_target/$rust_build_profile/libapp.rlib >logs/libapp.elf 2>&1
#  $objdump_cmd -t -S            --line-numbers --wide target/$rust_build_target/$rust_build_profile/libapp.rlib >logs/libapp.S 2>&1
#  $objdump_cmd -t -S --demangle --line-numbers --wide target/$rust_build_target/$rust_build_profile/libapp.rlib >logs/libapp-demangle.S 2>&1
#  set -e

#  Run the RIOT OS build, which will link with the Rust app, Rust libraries and libcore.
echo ; echo "----- Build RIOT OS and link with Rust app" 
build_riot $build_app

#  TODO: Show the firmware size
set -x
set +e  # Ignore errors
# $HOME/PineTime/mynewt-newt/size_report/size_report \
#    apps/$build_app/bin/$build_app/*.elf \
#    apps/$build_app/bin/$build_app/*.map
set -e  # Stop on errors

#  Copy the disassembly and linker map to the logs folder.
$objdump_cmd -t -S --demangle --line-numbers --wide apps/$build_app/bin/$build_app/*.elf >logs/$build_app.S 2>&1
cp apps/$build_app/bin/$build_app/*.map logs

#  Flash the firmware
#  scripts/nrf52/flash-app.sh
