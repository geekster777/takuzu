packfolder src\ui target\assets.rc -binary

IF "%1" == "prod" (
  if not exist "bin\sciter.dll" (
    curl https://raw.githubusercontent.com/c-smile/sciter-js-sdk/main/bin/windows/x32/sciter.dll > bin\sciter.dll
  )

  cargo build --release
  copy target\release\takuzu.exe bin\takuzu.exe
  start bin\takuzu.exe
) ELSE (
  cargo build
  start cargo run
)
