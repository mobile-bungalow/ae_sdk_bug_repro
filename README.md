## Description

### The Problem

Upon clicking the 'options...' button twice in the effect control panel the project will segfault.

Platform: After Effects 25.4 Beta, MacOs

### Steps To Reproduce

Build and install the plugin using

```bash
just build
```

launch and apply `Bugrepro` to a layer.

In the effect control panel click the `Options..` button at the top of the effect twice.


### Solution!

Uncomment the comment in `lib.rs` to fix the bug - it appears that using zero sized sequence data causes a segfault
when clicking the `options..` button.
