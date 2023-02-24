<p align="center">
  <a href="https://jacobbenison.com/">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://user-images.githubusercontent.com/75739874/220756778-26348fde-148f-4383-859c-8624d3de955d.png">
      <source media="(prefers-color-scheme: light)" srcset="https://user-images.githubusercontent.com/75739874/220756757-7204f7eb-2d3d-4c8e-a449-b294ac851304.png">
      <img alt="Tailwind CSS" src="https://user-images.githubusercontent.com/75739874/220756757-7204f7eb-2d3d-4c8e-a449-b294ac851304.png" width="275" height="121" style="max-width: 100%;">
    </picture>
  </a>
</p>

# A Neocities client written in Rust

[![Neocities CLI Crate](https://img.shields.io/crates/v/neocities_cli)](https://crates.io/crates/neocities_cli)
[![Neocities CLI License](https://img.shields.io/github/license/ge3224/neocities)](https://github.com/ge3224/neocities/blob/main/LICENSE.md)
![Neocities CLI Crate Downloads](https://img.shields.io/crates/d/neocities_cli)

[Neocities](https://neocities.org/about) is a free web hosting service that allows users to create and publish their own websites. 

This client is a command-line interface application for managing websites hosted on Neocities. Users can easily upload and delete files between their local computer and their Neocities website, all from their terminals.

**Demo:**

https://user-images.githubusercontent.com/75739874/221270216-693d0626-59e0-4df3-a2cf-987cc0afff26.mp4

<!--## Installation-->

 <!--- [ ]  TODO-->

## Setup

Set two environment variables:

```bash
export NEOCITIES_USER=<user>
export NEOCITIES_PASS=<pass>
```

Alternatively, you can use the `NEOCITIES_KEY` variable.

## Usage

- Upload files to your website:

```bash
neocities upload foo.html bar.js folder/baz.jpg
```

- Delete files from your website:

```bash
neocities delete foo.html folder/baz.jpg
```

- Get a list of available commands:

```bash
neocities

# Output...
#
# usage: neocities <command> [<args>]
# 
# 
# Commands:
#    upload    Upload files to Neocities
#    delete    Delete files from Neocities
#    info      Info about Neocities websites
#    key       Neocities API key
#    list      List files on Neocities
#    version   Show neocities client version
# 
# Help for a specific command:
#    help [command]
```

## Donate

Neocities is funded by [donations](https://neocities.org/donate). If you’d like to contribute, you can help to pay for server costs using Bitcoin or PayPal.

## License (MIT)

Copyright (c) 2023 Jacob Benison https://jacobbenison.com/

> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the "Software"), to deal
> in the Software without restriction, including without limitation the rights
> to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
> copies of the Software, and to permit persons to whom the Software is
> furnished to do so, subject to the following conditions:

> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.

> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
> IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
> FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
> AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
> LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
> OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
> SOFTWARE.
