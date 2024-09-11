# Nonsense Encoder

Encoding any data using custom codec with zstd compression.

## Usage

### Encode

```
$ cat codec_1.txt
乐
急
典
孝
绷
批
赢
```

```
$ ./encoder --encode codec_1.txt
Rock and stone, brother!同舟共济，绝不放弃！🥺
codec len: 7
source bytes len: 59 bytes
compressed bytes len: 68 bytes (115.25%)
encoded len: 385
encoded bytes len: 1155 bytes (1957.63%)
急，典，典，急，急，乐，孝，绷，孝，孝，孝，赢，典，绷，典，乐，批，绷，赢，典，批，急，孝，绷，典，绷，孝，赢，急，绷，急，孝，批，急，绷，孝，赢，乐，绷，绷，绷，绷，典，急，孝，乐，典，孝，典，急，典，绷，急，急，孝，孝，孝，乐，典，典，绷，批，赢，赢，乐，孝，赢，急，绷，绷，绷，赢，典，批，孝，赢，绷，乐，绷，急，赢，孝，乐，典，赢，典，急，赢，典，乐，批，急，批，赢，急，急，赢，乐，孝，赢，急，批，批，赢，批，绷，典，绷，孝，典，急，赢，乐，赢，赢，急，赢，批，绷，绷，绷，典，绷，乐，乐，赢，孝，孝，绷，批，批，典，批，孝，急，绷，绷，乐，批，急，乐，赢，典，赢，乐，批，绷，乐，典，典，乐，孝，批，典，赢，赢，赢，孝，典，乐，急，急，赢，典，批，典，急，赢，批，批，孝，批，孝，赢，批，孝，典，赢，孝，绷，乐，急，乐，批，乐，批，乐，孝，孝，孝，批，孝，赢
```

```
$ head -n 20 rfc8878.txt




Internet Engineering Task Force (IETF)                         Y. Collet
Request for Comments: 8878                             M. Kucherawy, Ed.
Obsoletes: 8478                                                 Facebook
Category: Informational                                    February 2021
ISSN: 2070-1721


      Zstandard Compression and the 'application/zstd' Media Type

Abstract

   Zstandard, or "zstd" (pronounced "zee standard"), is a lossless data
   compression mechanism.  This document describes the mechanism and
   registers a media type, content encoding, and a structured syntax
   suffix to be used when transporting zstd-compressed content via MIME.

$ ./encoder --encode codec_1.txt rfc8878.txt rfc8878_1.txt
codec len: 7
source bytes len: 112425 bytes
compressed bytes len: 24449 bytes (21.75%)
encoded len: 139341
encoded bytes len: 418023 bytes (371.82%)
```

### Decode

```
$ ./encoder codec_1.txt
急，典，典，急，急，乐，孝，绷，孝，孝，孝，赢，典，绷，典，乐，批，绷，赢，典，批，急，孝，绷，典，绷，孝，赢，急，绷，急，孝，批，急，绷，孝，赢，乐，绷，绷，绷，绷，典，急，孝，乐，典，孝，典，急，典，绷，急，急，孝，孝，孝，乐，典，典，绷，批，赢，赢，乐，孝，赢，急，绷，绷，绷，赢，典，批，孝，赢，绷，乐，绷，急，赢，孝，乐，典，赢，典，急，赢，典，乐，批，急，批，赢，急，急，赢，乐，孝，赢，急，批，批，赢，批，绷，典，绷，孝，典，急，赢，乐，赢，赢，急，赢，批，绷，绷，绷，典，绷，乐，乐，赢，孝，孝，绷，批，批，典，批，孝，急，绷，绷，乐，批，急，乐，赢，典，赢，乐，批，绷，乐，典，典，乐，孝，批，典，赢，赢，赢，孝，典，乐，急，急，赢，典，批，典，急，赢，批，批，孝，批，孝，赢，批，孝，典，赢，孝，绷，乐，急，乐，批，乐，批，乐，孝，孝，孝，批，孝，赢
codec len: 7
source bytes len: 1155 bytes
decoded bytes len: 68 bytes
decompressed bytes len: 59 bytes (5.11%)
Rock and stone, brother!同舟共济，绝不放弃！🥺

```

## Build

This tool was written in Rust, so a Rust toolchain is required.

```
$ cargo build
```
