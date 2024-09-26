# trivial-udp-mux
Muxes UDP packets from one socket to multiple destinations.

This functionality is trivial and implemented in many other applications.
Consider using another service which does this, or directly copying this code so you do not depend on this repo for such a trivial funcitonality.

# Getting started
```bash
cargo build --release
target/release/trivial-udp-mux --help
```

# Example usage
In this example, we mux packets from a [CMI](https://www.ta.co.at/x2-bedienung-schnittstellen/cmi) to two downstream services:
- [ta-asterisk-alarm](https://github.com/curatorsigma/ta-asterisk-alarm) to originate calls from asterisk on alarms from the CMI
- [churchtools-ta-sync](https://github.com/curatorsigma/churchtools-ta-sync#v0.2.3) to sync room bookings from [churchtools](https://church.tools) to the CMI

See the dockerfile for the command line options used. Note also that this will not actually work unless you have the config files set up - see the documentation for the two linked services for more information.
```bash
docker compose up
```

# License
This project is licensed under MIT-0 (MIT No Attribution).
By contributing to this repositry, you agree that your code will be licensed as MIT-0.

For my rationale for using MIT-0 instead of another more common license, please see
https://copy.church/objections/attribution/#why-not-require-attribution .

