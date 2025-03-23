# libnetxduo-sys

libnetxduo-sys provides Rust bindings for [NetX Duo][netx-duo], the ThreadX
network stack. The package can build NetX Duo for various ports. You can also
use this package with a pre-built NetX Duo library.

The package builds upon [libthreadx-sys], Rust bindings for ThreadX. Before
trying to build and use this package, you should know how to build and use
libthreadx-sys.

[netx-duo]: https://github.com/eclipse-threadx/netxduo
[libthreadx-sys]: https://github.com/mciantyre/libthreadx-sys

## Usage

Include the package in your project's `Cargo.toml`. This package isn't yet on
crates.io, so use a git dependency.

By default, this package builds NetX Duo from source. It selects your target
port by coordinating with libthreadx-sys. Note that, as of this writing, host
builds may not be supported.

Source builds have the same requirements as [libthreadx-sys]. If you can build
that package from source, then you should have no issues building this package.

Alternatively, you can [override this package's build
script][build-script-override] to reference a pre-built NetX Duo library. See
the [libthreadx-sys] documentation for an example.

[build-script-override]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#overriding-build-scripts

## Customizing NetX Duo

Many configurations only affect NetX internals without affecting the ABI. You
may change them by setting `CFLAGS` during the build. For example, you may use
this to specify minimum packet alignment, or to disable software checksumming
when hardware can accelerate the algorithm. Consult NetX Duo documentation to
understand the available configurations.

Other configurations change both the ABI and library internals. Like
libthreadx-sys, this package realizes ABI configurations with conditional
compilation options, not features. If you're building from source, setting the
Rust conditional compilation feature will enable the corresponding C option,
and it will change the Rust bindings. On the other hand, if you're referencing
a prebuilt library, then you must set the corresponding Rust configurations to
match your prebuilt library.

The rest of this section documents the available configurations.

`nx_enable_interface_capability` lets network drivers signal support for extra
features, including hardware checksumming. Note that this changes runtime
behavior; it does not exclude or include capabilities in the archive.

`nx_driver_deferred_processing` enables deferred network driver packet handling.
Received and transmit packet handling can occur on the IP thread instead of
inside an interrupt context.

`nx_dhcp_client_user_create_packet_pool` changes how the DCHP client allocates
packet pools. If enabled, the user must supply a packet pool. If disabled (the
default), the DHCP client allocates its own packet pool.

## Addons

NetX Duo provides addons for the network stack. In libnetxduo-sys, these are
exposed through Cargo features. Enabling the Cargo feature includes the addon
in the output archive. It _may_ expose Rust bindings for the addon.

`"dhcp-client"` includes the DCHP client feature in the archive. It also exposes
Rust bindings for DHCP client APIs. Use the bindings to integrate DHCP client
features into your Rust program.

`"web"` compiles the web HTTP addon into the archive. The addon does not support
HTTPS. Additionally, there are no Rust bindings exported by this package. This
feature is useful when separately compiling C code that expects the web addon.

If you're referencing a prebuilt NetX Duo library, then that library is expected
to contain the addons, as discussed in the upstream documentation. By enabling
the Cargo feature, the package will _only_ expose bindings, if implemented.

## Build script inputs

This package directly depends on libthreadx-sys. When the libthreadx-sys build
script executes, it sets various `DEP_THREADX_*` environment variables that are
expected by this package. For more information, see libthreadx-sys build script
output documentation.

If you are using build script overrides to reference a precompiled version of
libthreadx-sys, and if you are building NetX Duo from source, then you must set
the appropriate `DEP_THREADX_*` metadata in your override.

## Build script outputs

When this package's build script executes, it sets metadata that can be used
by immediate dependencies. Dependents access this metadata through environment
variables. See [the `links` manifest key][cargo-links] for more information.

[cargo-links]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key

`DEP_NETXDUO_COMMON_INCLUDE` is a path to a directory containing `nx_api.h` and
all other headers. If an addon feature is enabled, this directory also contains
theses addon headers. Consider using this as an include path if you're compiling
C code.

`DEP_NETXDUO_PORT_INCLUDE` is a path to a directory containing `nx_port.h`. The
contents of the header vary by port. Consider using this as an include path if
you're compiling C code.

Similar to libthreadx-sys, if you're overriding the build script, you're
encouraged to set this same metadata in your override configuration.

## License

libnetxduo-sys is MIT licensed. See [LICENSE](./LICENSE) for more information.

This package incorporates NetX Duo API documentation from the
[eclipse-threadx/rtos-docs] repository. libnetxduo-sys package vendors this
documentation in files matching `src/doc/*.md`. The associated MIT license is
at `src/doc/LICENSE`.

libnetxduo-sys includes NetX Duo source code from the `netxduo` directory. See
`netxduo/LICENSE.txt` for the source's MIT license.

[eclipse-threadx/rtos-docs]: https://github.com/eclipse-threadx/rtos-docs/tree/71a7c00f369b9cd648f73dfddb0d5ca6d5a1b855
