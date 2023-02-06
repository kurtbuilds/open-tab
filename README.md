# `open-tab`

Tiny command line tool to open a URL in Chrome on Mac OS. Opens the existing tab if the URL is already open,
otherwise opens a new tab with the URL.

# Usage

```bash
open-tab https://www.google.com  # opens Google in a new tab
open-tab https://www.google.com  # Second time, it re-opens the same tab
```

# Installation

Installation requires `just` and `checkexec`.

```bash
just install
```
