# opendata-download-radar
Drop-in API replacement if you want to use SMHI radar imagery
but serve the `.tiff` format as `.png`'s instead since its
not officially supported.


Supports latest radar images served as `.png`.

What is this?
SMHI Open Data API does not support serving unbranded imagery unless
you use the `.tiff` format. This is problematic on platforms
such as Android because native image decoders does not support
the tiff-format.

This drop-in-replacement allows you to have the same behavior as SMHI`s api,
but the only difference is that it will serve convert the tiff image into
png format and serve you it back.

You still send the SMHI request like: 

```
GET /api/version/latest/area/sweden/product/comp/2019/04/03/radar_0810010000.tif
```

or http://localhost:8000/api/version/latest/area/sweden/product/comp/2019/04/03/radar_0810010000.tif

You only change your `BASE_URL` in whatever application you use.

But you will get back a radar image in .png format instead to overcome this.
OPtionall you can let the `opendata-download-radar`-api apply colors to it aswell.

The radar images are stored in `/latest/area/sweden/product/comp/{year}/{month}/{date}/`.

The API caches the radar image in memory for a certain time, otherwise it fetches it
from disk.

# Run
```sh
cargo run
```

# Test
```sh
GET http://localhost:8000/api/version/latest/area/sweden/product/comp/2019/04/03
Current path: /version/latest/area/sweden/product/comp/2019/04/03
```

# Models

```sh
GET https://opendata-download-radar.smhi.se/api/version/latest/area/sweden/product/comp/2019/04/03/radar_0810010000.png
GET https://opendata-download-radar.example.tld/api/version/latest/area/sweden/product/comp/{year}/{month}/{date}/{radar}_{date}.png
```

## Build

### Configuration
###### Optional coloring
Optional `colors.ini` can be supplied to serve colored images.
Otherwise the grayscale image will simply be returned to you
in a grayscale format.

### Supports


### Docker
You can run the instance via docker (only 7mb image!)

* HTTP 1.1
	GET /
