# IRON KEYS

A simple password generator

## Installation (Linux & macOS)
```
wget https://github.com/jsattari/iron-keys/releases/download/v1.0.0/iron-keys.tar.gz

tar -xzf iron-keys.tar.gz

sudo mv iron-keys /usr/local/bin/
```

## Usage
```
iron-keys -l 12
# returns a string of 12 random characters: v%o^xkfIiXY#
```

## Options
```
-l, --length <LENGTH>  [default: 8]
  -r, --repeats          
  -a, --avoid <AVOID>    [default: ]
  -h, --help             Print help
  -V, --version          Print version
```