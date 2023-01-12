![status](https://github.com/paolobroglio/atium/actions/workflows/rust.yml/badge.svg)

# Atium

**atium** is a tool that enables video conversions and other useful tools.

## Installed features
### Video conversion
Simple conversion API that lets you convert a given input video to a given output video. 

Supported engines are:
* `ffmpeg`

Supported resolutions are:
* `SD`
* `HD`
* `Full HD`
* `2k`
* `Ultra HD`
* `8k`

Supported output containers are:
* `MP4`

Supported output codecs are:
* `H.264`

### Content analysis
A simple analysis API that lets you extract useful infos about a media content.

Supported engines are:
* `mediainfo`


## Wanted features
- Outputs:
  - [ ] MP4
  - [ ] HLS
- [ ] Configurable output resolutions
- [ ] Thumbnail extraction
- [ ] Subtitles burning