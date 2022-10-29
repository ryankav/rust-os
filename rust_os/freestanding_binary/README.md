# Free standing binary

This was simply an introduction binary into how to build a binary without the standard lib.

## takeaways:

- Have to set the `![no_std}` at top of file
- Need to remove all std dependencies
- Create a panic handler & add abort to toml file
- add the `![no_main]` attribute at the top of the file
- implement a start function for the booter to call
- If compiled on an existing OS this still fails as rustc assumes being built for current OS so have to specify bare metal target in order for this binary to compile
