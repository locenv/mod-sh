# A module for interfacing with Bourne shell

## Installation

```sh
locenv mod install github:locenv/mod-sh
```

## Usage

```lua
local sh = require 'sh'
local s = sh.newsession()

s:run('echo "Hello, world!"')
```

## License

MIT
