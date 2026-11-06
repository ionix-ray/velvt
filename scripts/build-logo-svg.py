#!/usr/bin/env python3
"""Vendor a transparent, animated, self-contained logo SVG from the master file.

Reads the Draw.io source (white editor canvas + bounds + embedded raster),
keeps only the emblem artwork and wordmark geometry, and emits a lightweight
transparent SVG: optimized emblem data-URI, vector VELVT text, internal
CSS float + sheen (reduced-motion aware).

Usage:
    scripts/build-logo-svg.py [source.svg] [out.svg]

Source of truth: /Users/samirparhi-dev/Desktop/velvt-logo.drawio.svg
"""
import base64
import sys
import xml.etree.ElementTree as ET
from io import BytesIO

from PIL import Image

SRC = sys.argv[1] if len(sys.argv) > 1 else "/Users/samirparhi-dev/Desktop/velvt-logo.drawio.svg"
OUT = sys.argv[2] if len(sys.argv) > 2 else "velvet-ui/assets/images/velvt-logo.svg"

SVGNS = "http://www.w3.org/2000/svg"
XLINK = "http://www.w3.org/1999/xlink"

root = ET.parse(SRC).getroot()
href = root.find(f".//{{{SVGNS}}}image").attrib[f"{{{XLINK}}}href"]
raw = base64.b64decode(href.split(",", 1)[1])
emblem = Image.open(BytesIO(raw)).convert("RGBA")
emblem = emblem.crop(emblem.getchannel("A").getbbox()).resize((384, 384), Image.LANCZOS)

buf = BytesIO()
emblem.save(buf, format="PNG", optimize=True)
data_uri = "data:image/png;base64," + base64.b64encode(buf.getvalue()).decode("ascii")

svg = f"""<?xml version="1.0" encoding="UTF-8"?>
<!-- Velvt logo — transparent production asset derived from {SRC}.
     Dropped: editor canvas, white backing, bounds, mxfile metadata.
     Emblem: alpha-cropped 384px PNG data-URI. Wordmark: vector text in
     crimson-light (#E63E3E) — passes large-text contrast on both Porcelain
     and Obsidian. (currentColor can't cross the <img> document boundary,
     so the fill is baked, not inherited.) -->
<svg xmlns="{SVGNS}" xmlns:xlink="{XLINK}" viewBox="0 0 874 344" role="img" aria-label="Velvt logo">
<title>Velvt logo</title>
<style>
.velvt-float {{ transform-box: fill-box; transform-origin: center; animation: velvt-float 7s ease-in-out infinite; }}
.velvt-sheen-rect {{ animation: velvt-sheen 6s ease-in-out infinite; }}
@keyframes velvt-float {{ 0%,100% {{ transform: translateY(0); }} 50% {{ transform: translateY(-8px); }} }}
@keyframes velvt-sheen {{ 0%,60% {{ transform: translateX(-220px) skewX(-14deg); opacity: 0; }} 75% {{ opacity: .5; }} 100% {{ transform: translateX(980px) skewX(-14deg); opacity: 0; }} }}
@media (prefers-reduced-motion: reduce) {{ .velvt-float, .velvt-sheen-rect {{ animation: none !important; }} }}
</style>
<g class="velvt-float">
<image x="10.94" y="30.95" width="282.11" height="282.11" preserveAspectRatio="xMidYMid meet" href="{data_uri}"/>
<text x="604" y="215" text-anchor="middle" font-family="'Cormorant Garamond', Georgia, serif" font-size="170" letter-spacing="8" fill="#E63E3E">VELVT</text>
</g>
<g clip-path="url(#velvt-clip)" pointer-events="none">
<clipPath id="velvt-clip"><rect x="0" y="20" width="874" height="304" rx="24"/></clipPath>
<rect class="velvt-sheen-rect" x="0" y="0" width="180" height="344" fill="#ffffff" opacity="0"/>
</g>
</svg>
"""
with open(OUT, "w", encoding="utf-8") as f:
    f.write(svg)
print(f"{OUT}: {len(svg)} bytes")
