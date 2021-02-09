# Introduction
Iptvtk is a command line tool to handle IPTV playlists. It is intended to read an input channel list in a format, apply transformations on it, and write it to a possibly other output format. For now, it is at a early stage and only support reading a m3u file, filtering on a given group name, and writing to another m3u file.
It is written in Rust and compiles into a static native executable without any runtime or dependency.

# Build
You need a rust toolchain to build the program. Clone the repository, and run in its directory:

```bash
cargo build --release
```

 The executable can then be found in `target/release`.
# Usage
To get usage information, run:

```bash
iptvtk --help
```
You will see the following help:

```
iptvtk 0.1.0

USAGE:
    iptvtk -f <filter_on> -i <input_file> -o <output_file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f <filter_on>
    -i <input_file>         the m3u file
    -o <output_file>        the m3u output file
```

Example

```bash
iptvtk -f group-title="EU | FRANCE GENERAL" -i tv_channels.m3u -o output.m3u
```
# Features
Legend:

  not implemented (wish have)
  implemented

## Input format
  m3u
## Output formats
  m3u
  csv (with ou without header, and with custom separator)
  json (for Cumulus TV)
## Transformations
  Filter on IPTV property (name, url, tvg-group, etc.)
  Filter on EXTINF regex
  (Re)numerate
  Re-order
  Add property (like `tvg-guide` or `tvg-logo`)
 

# Supported or planned formats
## M3U
```m3u
#EXTM3U
#EXTINF:-1 tvg-id="1508" tvg-logo="http://server.com/logo/FRANCE/TF1HD.png" group-title="EUROPE | France FHD - OTT", FR - TF1 FHD                                           http://server.com/live/1508.ts                             #EXTINF:-1 tvg-id="1507" tvg-logo="http://server.com/logo/FRANCE/FRANCE2HD.png" group-title="EUROPE | France FHD - OTT,null", FR - FRANCE 2 FHD
http://server.com/live/1507.ts
```
## CSV
```csv
number;tvg-name;group-title;tvg-id;tvg-logo;name;url
1;FR - TF1 FHD;EUROPE | France FHD - OTT;1508;http://server.com/logo/FRANCE/TF1HD.png;TF1 FHD;http://server.com/live/1508.ts
2;FR - FRANCE 2 FHD;EUROPE | France FHD - OTT;1507;http://server.com/logo/FRANCE/FRANCE2HD.png;TF1 FHD;http://server.com/live/1507.ts
```
## JSON
This is the internal format of [Cumulus TV](https://cumulustv.herokuapp.com), an Android TV application to view internet streams as Live Channels. It can be installed on TVs supporting Live Channels, like Sony ones. With it, you can use your IPTV streams as ordinary channels. It supports importing m3u files, but this function is not fully working.
Instead, you can convert your playlist with iptvtk, put the resulting json file in you Google Drive account, and then import it in CumulusTV on your TV.
Example:

```json
{"channels":[,{"audioOnly":false,"epgUrl":"","genres":"TNT","logo":"http:\/\/server.com\/logo\/EUROPE\/FRANCE\/TF1.png","url":"http:\/\/server.com\/56988","name":"TF1 Full HD","number":"1","splashscreen":""},,{"audioOnly":false,"epgUrl":"","genres":"TNT","logo":"http:\/\/server.com\/logo\/EUROPE\/FRANCE\/FRANCE2FHD.png","url":"http:\/\/server.com\/\/57003","name":"FR - FRANCE 2 FHD","number":"2","splashscreen":""}
"name":"FR - I24 NEWS ],"modified": 1568151960000,"possibleGenres":["TNT"]
```
# Dependencies
This project relies on m3u crate from michmindtree, with modifications from me.
# Project status
The project is at early stage. Its current version is 0.1.0. I wrote it for my usage, but if someone is interested in more features, feel free to open Feature Requests or send Pull Requests.
# Licence
The project is release under GNU GPL version 3.

