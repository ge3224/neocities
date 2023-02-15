![image](https://user-images.githubusercontent.com/75739874/219054552-5a4de465-9498-4cb8-b3bb-42e9c4b4ac76.png)

# A Neocities client written in Rust

Upload files to your [Neocities](https://neocities.org/) website your terminal.

## Installation

// TODO

## Setup

Set two environment variables:

```
export NEOCITIES_USER=<user>
export NEOCITIES_PASS=<pass>
```

Alternatively, you can use the `NEOCITIES_KEY` variable.

## Usage

- Upload files to your website:

```
neocities upload foo.html bar.js folder/baz.jpg
```

- Delete files from your website:

```
neocities delete foo.html folder/baz.jpg
```

- Get a list of available commands:

```
$ neocities
usage: neocities <command> [<args>]

Commands:
   upload    Upload files to Neocities
   delete    Delete files from Neocities
   info      Info about Neocities websites
   key       Neocities API key
   list      List files on Neocities
   version   Show neocities client version

Help for a specific command:
   help [command]
```

## Donate

NeoCities is funded by donations. If you’d like to contribute, you can help to pay for server costs using Bitcoin or PayPal.

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
