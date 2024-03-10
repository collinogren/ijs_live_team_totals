# IJS Live Team Totals
This software is an independent addon for U.S. Figure Skating and ISU accounting software. The purpose of this software is to provide a unified and flexible solution to calculating team points.

# Free and Open Source Software
This software is free and open source which not only allows you, the end user, to request changes from myself, but it also allows you to fix and adjust the program to suit your needs. I also encourage you to make pull requests so that other users may take advantage of your work.

# Building From Source
This software is written using the Rust programming language, a modern, fast, cross-platform, safe language.
The Rust compiler and associated software can be found here: https://www.rust-lang.org/tools/install
If you are unfamiliar with Rust and would like to learn more, check out the book *The Rust Programming Language*: https://doc.rust-lang.org/book/

Once the Rust compiler has been installed, download the source and run
`cargo build --release`
to build the release version and
`cargo build`
to build the debug version.

# TODO
This software is a work in progress, the current goals for new features include:
+ Implement support for partnered events for IJS and 6.0
+ Add an option to pull HTML data from the internet so as to avoid needing to do a full update in the backroom with an up-to-date database.

# Copyright Information
This free and open source software is published under the MIT License:

Copyright (c) 2023 Collin Ogren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
