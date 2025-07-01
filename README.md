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

Set `Property::AE_Effect_Global_OutFlags_2(OutFlags2::empty())` in the build.rs.
Removing the need for flattening the sequence data.
