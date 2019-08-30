# opendata-download-radar

**I am not working on this anymore. Maybe in the future.**

Status: proof-of-concept, not even alpha

A drop-in proxy service for colorizing grayscale radar imagery from SMHI Öppna API Radar in `.tif` and converts it to `.png`.

You can run it in two modes, either serve the imagery in gray-scale
but as `.png` or you can supply a colorscheme that colorizes the
imagery for you, still serving you it as `.png`.

Downloads and colorizes and serves a 887x471 px image in < 100 ms on my laptop.

### What is this?
SMHI Open Data API does not support serving unbranded imagery unless
you use the `.tiff` format. This is problematic on platforms
such as Android because native image decoders does not support
the tiff-format.

This drop-in-replacement allows you to have the same behavior as SMHI's api,
but the only difference is that it will serve convert the tiff image into
`.png`-format and serve you it back.

## Usage
You can send a request like this to get the `.tif` image data as `.png`.
```
curl -O http://localhost:8000/api/version/latest/area/sweden/product/comp/2019/04/03/radar_1904030005
```

## Storage
The radar images are stored in `/data/{year}/{month}/{date}/{filename}.png`.

# Run
```sh
cargo run --release
```

## Build
Build is run in `rust:latest` image then binary is copied
to runtime image `opendata-download-radar` image.
```sh
$ docker build -t opendata-download-radar:latest .
```

### Configuration
Optional `colors.ini` can be supplied to serve colored images.
Otherwise the grayscale image will simply be returned to you
in png-format.

### Run in docker
You can run the instance via docker:
```sh
$ docker run -it --rm -v data:/data sphrak/opendata-download-radar:latest
```

### Versioning

### Libraries
* https://docs.rs/image/0.21.0/image/
* https://docs.rs/reqwest/0.9.13/reqwest/
* https://api.rocket.rs/v0.4/rocket/
* https://github.com/shssoichiro/oxipng
* https://github.com/BurntSushi/rust-csv

## License

	Copyright 2019 Niclas Kron

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

	   http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.
