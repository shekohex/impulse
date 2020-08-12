<h1 align="center">Impulse 🔔</h1>
<div align="center">
  <strong>
    A CLI Tool to Send a Push Notifications when a command completes.
  </strong>
</div>

<br />

<div align="center">
  <a href="https://travis-ci.com/github/shekohex/impulse">
    <img src="https://travis-ci.com/shekohex/impulse.svg?branch=master"
      alt="Travis CI" />
  </a>
   <a href="https://ci.appveyor.com/project/shekohex/impulse">
    <img src="https://ci.appveyor.com/api/projects/status/kpaeadhhoscqbcdw/branch/master?svg=true"
      alt="AppVeyor" />
  </a>
  <a href="https://github.com/shekohex/impulse/releases">
    <img alt="GitHub All Releases" src="https://img.shields.io/github/downloads/shekohex/impulse/total">
  </a>
</div>

## About

Example:

```bash
$ impulse 'cargo build'
```

## Install

1. [Github Releases](https://github.com/shekohex/impulse/releases)

2. Or if you want to build it localy

```bash
$ git clone https://github.com/shekohex/impulse.git
$ cd impulse
$ cargo install
```

## Usage

1. Goto [Impulse Website](https://impulse-build.netlify.app/) and enable push notifications (the small red bell in the bottom right corner).
2. Copy your UserID and set `IMPULSE_USER_IDS` env to the UserID value.

```bash
Usage: impulse <cmd> [--uids <uids>] [-s <success-message>] [-e <error-message>]

Send push notification when your long build command finish

Options:
  --uids            override the UserIDs that we will send notification to.
                    normally this will be stored in `IMPULSE_USER_IDS` env. UIDs
                    is separated by `,`.
  -s, --success-message
                    set the success message. default to: Build exit successfully
  -e, --error-message
                    set the error message. default to: Build errored
  --help            display usage information
```

## FAQ

1. How do I support more than device (like my other laptop or mobile)?

- the `IMPULSE_USER_IDS` env accept more than one UID. just seprate them by a `,`.

2. How do I ..?

- Open an issue :)

## Contributing

Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[contributing]: https://github.com/shekohex/impulse/blob/master/.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/shekohex/impulse/labels/good%20first%20issue
[help-wanted]: https://github.com/shekohex/impulse/labels/help%20wanted

## Safety

This crate uses `#![deny(unsafe_code)]` to ensure everything is implemented in
100% Safe Rust.

## License

<sup>
Licensed under <a href="LICENSE">MIT license</a>.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the MIT license, without any additional terms or conditions.
</sub>
