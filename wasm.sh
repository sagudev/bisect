#!/bin/sh
set -e
# Build
case "$1" in
    r*)
        wasm-pack build --target web --release -- --features wasm
    ;;
    *)
        wasm-pack build --target web --dev -- --features wasm
    ;;
esac
# Server
#python3 -m http.server
# post-bindgen
# allow dialog.hide as js_namespace
# TODO: no post-bindgen
#sed -i 's/typeof .* == '\''function'\'' ? .* : notDefined('\''.*'\'');/OK/g' test
# Back up
cp ./pkg/bisect.js ./pkg/bisect.js.bk
# Do post-bindgen
# tel namespace ni glih za dialog.show ampak če nardim sed k bo mal popravu boolr.js bo vse ok.
sed -i '/typeof .* == '\''function'\'' ? .* : notDefined('\''.*'\'');/{s! == '\''function'\'' ? ! == '\''function'\'' ? function(){ !g}' ./pkg/bisect.js
sed -i '/typeof .* == '\''function'\'' ? .* : notDefined('\''.*'\'');/{s! : notDefined('\''!(); } : notDefined('\''!g}' ./pkg/bisect.js
# What's changed  in postgen
diff ./pkg/bisect.js ./pkg/bisect.js.bk
rm ./pkg/package.json
rm ./pkg/.gitignore