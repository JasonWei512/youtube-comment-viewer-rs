pub const ABOUT_API_KEY: &str = "About API key:
You need a valid YouTube API key to use this tool. Follow this tutorial to create one: 
https://developers.google.com/youtube/v3/getting-started#before-you-start
Then set the environment variable \"YOUTUBE_API_KEY\" to the key, or pass it as argument \"--api-key\".

PowerShell:
$env:YOUTUBE_API_KEY = \"VGFrZXVjaGkgTWFyaXlhIC0gUGxhc3RpYyBMb3Zl\"
./youtube_comment_viewer.exe https://www.youtube.com/watch?v=9Gj47G2e1Jc
or
./youtube_comment_viewer.exe 9Gj47G2e1Jc --api-key VGFrZXVjaGkgTWFyaXlhIC0gUGxhc3RpYyBMb3Zl

Bash:
export YOUTUBE_API_KEY=VGFrZXVjaGkgTWFyaXlhIC0gUGxhc3RpYyBMb3Zl
./youtube_comment_viewer https://www.youtube.com/watch?v=9Gj47G2e1Jc
or
./youtube_comment_viewer 9Gj47G2e1Jc --api-key VGFrZXVjaGkgTWFyaXlhIC0gUGxhc3RpYyBMb3Zl";
