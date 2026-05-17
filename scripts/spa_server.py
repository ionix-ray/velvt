#!/usr/bin/env python3
"""
SPA Static Server for Velvet PR Agency
Serves static files with fallback to index.html for client-side routing.
Usage: python3 scripts/spa_server.py <directory> [port]
"""

import http.server
import os
import sys
import urllib.parse

class SPAHandler(http.server.SimpleHTTPRequestHandler):
    """HTTP handler that falls back to index.html for SPA routing."""

    def __init__(self, *args, directory=None, **kwargs):
        self.spa_directory = directory
        super().__init__(*args, directory=directory, **kwargs)

    def do_GET(self):
        """Serve file if exists, otherwise serve index.html."""
        parsed_path = urllib.parse.urlparse(self.path)
        file_path = os.path.join(self.spa_directory, parsed_path.path.lstrip('/'))

        # If file exists, serve it
        if os.path.isfile(file_path):
            return super().do_GET()

        # If it's an API call or asset that should exist, return 404
        if parsed_path.path.startswith('/assets/'):
            return super().do_GET()

        # Otherwise, serve index.html for SPA routing
        index_path = os.path.join(self.spa_directory, 'index.html')
        if os.path.isfile(index_path):
            self.path = '/index.html'
            return super().do_GET()

        return super().do_GET()

    def end_headers(self):
        """Add security and caching headers."""
        # No cache for HTML
        if self.path.endswith('.html') or self.path == '/':
            self.send_header('Cache-Control', 'no-cache, no-store, must-revalidate')
            self.send_header('Pragma', 'no-cache')
            self.send_header('Expires', '0')
        # Long cache for assets
        elif any(self.path.endswith(ext) for ext in ['.wasm', '.js', '.css', '.woff2', '.png', '.jpg', '.svg']):
            self.send_header('Cache-Control', 'public, max-age=31536000, immutable')
        # Security headers
        self.send_header('X-Content-Type-Options', 'nosniff')
        self.send_header('X-Frame-Options', 'DENY')
        self.send_header('Referrer-Policy', 'strict-origin-when-cross-origin')
        super().end_headers()

    def log_message(self, format, *args):
        """Colorized log output."""
        status = args[1] if len(args) > 1 else ''
        if '404' in str(status):
            prefix = '\033[0;33m[WARN]\033[0m'
        elif '200' in str(status):
            prefix = '\033[0;32m[OK]\033[0m'
        else:
            prefix = '\033[0;34m[INFO]\033[0m'
        sys.stderr.write(f"{prefix} {self.address_string()} - {format % args}\n")
        sys.stderr.flush()


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 spa_server.py <directory> [port]")
        sys.exit(1)

    directory = sys.argv[1]
    port = int(sys.argv[2]) if len(sys.argv) > 2 else 8080

    if not os.path.isdir(directory):
        print(f"Error: Directory '{directory}' not found")
        sys.exit(1)

    handler = lambda *args, **kwargs: SPAHandler(*args, directory=directory, **kwargs)

    server = http.server.HTTPServer(('0.0.0.0', port), handler)
    print(f"\033[0;32m[OK]\033[0m SPA server running on http://localhost:{port}")
    print(f"\033[0;34m[INFO]\033[0m Serving from: {directory}")
    print(f"\033[0;34m[INFO]\033[0m Press Ctrl+C to stop\n")

    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\n\033[0;33m[INFO]\033[0m Server stopped")
        server.shutdown()


if __name__ == '__main__':
    main()
