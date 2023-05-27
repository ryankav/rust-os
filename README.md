# rust-os
Repo for working through Philipp Oppermann's `Writing an OS in Rust`

## Setup
You will need to have installed rust, preferably with rustup. As long as you
have done that then the toolchain.toml file should ensure that the nightly
channel is used and the required components are used.

The only other thing that will be needed is QEMU so that we can lauch the
binary being built. On Ubuntu this requires both the qemu package and the
qemu-kvm package.
