#!/usr/bin/env bash
# Build the lightweight ambient hero-background loop from brand-owned source clips.
#
# Usage:
#   scripts/build-hero-media.sh [scene1.mp4 scene2.mp4 scene3.mp4] [outdir]
#
# Defaults use the user-supplied local sources (scene order: city billboard →
# live production → creative studio) and write into velvet-ui/assets/images/.
#
# Pipeline (FFmpeg media-processing pattern):
#   1. Per-scene normalize: 960x540, 24fps, yuv420p, no audio.
#   2. Brand grade: gentle contrast/saturation/temperature shift toward
#      crimson/obsidian; light soften (sigma 2) so foreground text stays king.
#   3. Stitch: 0.5s dissolves (xfade) between scenes for one seamless loop.
#   4. Watermark: Velvt mark, bottom-right, 20% opacity.
#   5. Emit: H.264 MP4 (faststart) + VP9 WebM + graded poster JPG.
#
# Only brand-owned inputs. No stock, no third-party marks.
set -euo pipefail

SCENE1="${1:-/Users/samirparhi-dev/tpppd3.mp4}"
SCENE2="${2:-/Users/samirparhi-dev/tpppd1.mp4}"
SCENE3="${3:-/Users/samirparhi-dev/tpppd2.mp4}"
OUTDIR="${4:-velvet-ui/assets/images}"
MARK="${MARK:-velvet-ui/assets/images/velvet-mark-sm.png}"
FADE=0.5

for f in "$SCENE1" "$SCENE2" "$SCENE3" "$MARK"; do
    [ -f "$f" ] || { echo "missing input: $f" >&2; exit 1; }
done
mkdir -p "$OUTDIR"

# Durations drive the xfade offsets: offset(i) = running_total - FADE.
D1=$(ffprobe -v error -show_entries format=duration -of csv=p=0 "$SCENE1")
D2=$(ffprobe -v error -show_entries format=duration -of csv=p=0 "$SCENE2")
O1=$(python3 -c "print($D1 - $FADE)")
O2=$(python3 -c "print($D1 + $D2 - 2*$FADE)")

GRADE="scale=960:540:force_original_aspect_ratio=increase,crop=960:540,fps=24,format=yuv420p,eq=contrast=1.05:saturation=0.78:brightness=-0.015,gblur=sigma=2,setsar=1"

ffmpeg -y -hide_banner -loglevel error \
  -i "$SCENE1" -i "$SCENE2" -i "$SCENE3" -i "$MARK" \
  -filter_complex "\
[0:v]$GRADE,format=rgba[s0];\
[1:v]$GRADE,format=rgba[s1];\
[2:v]$GRADE,format=rgba[s2];\
[s0][s1]xfade=transition=fade:duration=$FADE:offset=$O1[x1];\
[x1][s2]xfade=transition=fade:duration=$FADE:offset=$O2[x2];\
[3:v]scale=96:-1,format=rgba,colorchannelmixer=aa=0.2[wm];\
[x2][wm]overlay=W-w-24:H-h-24:format=yuv420[out]" \
  -map "[out]" -an \
  -c:v libx264 -preset medium -crf 27 -movflags +faststart \
  "$OUTDIR/hero-ambient.mp4"

ffmpeg -y -hide_banner -loglevel error -i "$OUTDIR/hero-ambient.mp4" \
  -an -c:v libvpx-vp9 -crf 34 -b:v 0 -deadline good -cpu-used 2 \
  "$OUTDIR/hero-ambient.webm"

ffmpeg -y -hide_banner -loglevel error -ss 0.6 -i "$OUTDIR/hero-ambient.mp4" \
  -frames:v 1 -q:v 2 "$OUTDIR/hero-ambient-poster.jpg"

ls -la "$OUTDIR"/hero-ambient.mp4 "$OUTDIR"/hero-ambient.webm "$OUTDIR"/hero-ambient-poster.jpg
