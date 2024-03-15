# tauri-doublechar-issue

## Problem
When using latest tauri with multiwebview (adding webviews as childs)
And creating new post in twitter, the first letter which typed into the post creation input written twice, see video



https://github.com/thewh1teagle/tauri-doublechar-issue/assets/61390950/1e809966-ac66-406f-9d42-452836d53d29



## Repro

1. Clone and Run
```console
git clone https://github.com/thewh1teagle/tauri-doublechar-issue
cd tauri-doublechar-issue
npm i -g @tauri-apps/cli@2.0.0-beta.9
npx tauri dev
```

2. Log in to twitter
3. Create new post (from main page)

When typing first letter in the post, it will write double, duplicate character.

