#!/bin/sh
set -euf

cd /out

JS_FILE="$(find . -name '*.js' -type f | head -1)"
if [ -z "$JS_FILE" ]; then
    echo "No JS file found, skipping inline"
    exit 0
fi

echo "Inlining $JS_FILE into index.html"

# Remove preload link for JS
grep -v 'rel="preload".*as="script"' index.html > .tmp.html
mv .tmp.html index.html

# Replace external script tag with inline JS content via awk
awk -v jsfile="$JS_FILE" '
/<script type="module"/ && /src=".*\.js"/ {
    printf("<script type=\"module\">\n")
    while ((getline line < jsfile) > 0) {
        print line
    }
    close(jsfile)
    printf("</script>\n")
    next
}
{ print }
' index.html > .inlined.html

mv .inlined.html index.html
rm "$JS_FILE"
echo "JS inlined: $(wc -c < index.html) bytes -> index.html"
