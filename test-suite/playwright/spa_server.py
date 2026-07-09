import http.server
import socketserver
import os
import sys

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8091
DIRECTORY = "target/dx/vaelvet-ui/release/web/public"

class SPADirectoryHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

    def do_GET(self):
        # Determine the absolute path requested
        path = self.path.split('?')[0].split('#')[0]
        abs_path = os.path.join(DIRECTORY, path.lstrip('/'))
        
        # If the file does not exist, serve index.html (SPA fallback)
        if not os.path.isfile(abs_path):
            self.path = '/index.html'
            
        return super().do_GET()

# Ensure we're running from the workspace root where target/ exists
if not os.path.exists(DIRECTORY):
    print(f"Error: Directory {DIRECTORY} not found.", file=sys.stderr)
    print("Ensure you run this script from the workspace root after 'dx build'.", file=sys.stderr)
    sys.exit(1)

Handler = SPADirectoryHandler
with socketserver.TCPServer(("", PORT), Handler) as httpd:
    print(f"SPA server listening on http://localhost:{PORT} -> {DIRECTORY}")
    httpd.serve_forever()
