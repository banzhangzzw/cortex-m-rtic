# Starting a new project

A recommendation when starting a RTIC project from scratch is to 
follow RTIC's [`defmt-app-template`].

If you are targeting ARMv6-M or ARMv8-M-base architecture, check out the section [Target  Architecture](../internals/targets.md) for more information on hardware limitations to be aware of.

[`defmt-app-template`]: https://github.com/rtic-rs/defmt-app-template

This will give you an RTIC application with support for RTT logging with [`defmt`] and stack overflow
protection using [`flip-link`]. There is also a multitude of examples provided by the community:

- [`rtic-examples`] - Multiple projects
- [https://github.com/kalkyl/f411-rtic](https://github.com/kalkyl/f411-rtic)
- ... More to come

[`defmt`]: https://github.com/knurling-rs/defmt/
[`flip-link`]: https://github.com/knurling-rs/flip-link/
[`rtic-examples`]: https://github.com/rtic-rs/rtic-examples
