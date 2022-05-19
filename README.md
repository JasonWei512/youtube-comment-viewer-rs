# YouTube Comment Viewer

A command line tool to display comments of a YouTube video.

[Download](https://github.com/JasonWei512/youtube-comment-viewer-rs/releases/latest)


## Before using

You need a valid YouTube API key to use this tool. Follow this tutorial to create one: 

https://developers.google.com/youtube/v3/getting-started#before-you-start

Then set the environment variable "YOUTUBE_API_KEY" to the key, or pass it as argument "--api-key".


## Example

### PowerShell:

```PowerShell
$env:YOUTUBE_API_KEY = "VGFrZXVjaGkgTWFyaXlhIC0gUGxhc3RpYyBMb3Zl"
./youtube_comment_viewer.exe https://www.youtube.com/watch?v=9Gj47G2e1Jc
```

```PowerShell
./youtube_comment_viewer.exe 9Gj47G2e1Jc --api-key VGFrZXVjaGkgTWFyaXlhIC0gUGxhc3RpYyBMb3Zl
```

### Bash

```Bash
export YOUTUBE_API_KEY=VGFrZXVjaGkgTWFyaXlhIC0gUGxhc3RpYyBMb3Zl
./youtube_comment_viewer https://www.youtube.com/watch?v=9Gj47G2e1Jc
```

```Bash
./youtube_comment_viewer 9Gj47G2e1Jc --api-key VGFrZXVjaGkgTWFyaXlhIC0gUGxhc3RpYyBMb3Zl
```


## Usage

``` 
youtube_comment_viewer.exe [OPTIONS] <VIDEO_ID_OR_URL>

ARGS:
    <VIDEO_ID_OR_URL>
            The YouTube video ID or url to display comments for

OPTIONS:
        --api-key <API_KEY>
            The YouTube API key to use instead of the default one

    -d, --default
            Display comments in YouTube's default order (default)

    -h, --help
            Print help information

    -l, --like
            Display the comment with the most likes first

    -n, --new
            Display the newest comment first

    -o, --old
            Display the oldest comment first

    -p, --peek
            Display the newest 10 comments only

    -V, --version
            Print version information
```


## Build

Install [Rust](https://rustup.rs/), then run:

```
cargo build --release
```