#!/usr/bin/env python3
"""
Simple HTTP server for Nova
This will work once Python runtime is integrated
"""

from http.server import HTTPServer, BaseHTTPRequestHandler

class NovaHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header('Content-type', 'text/html')
        self.end_headers()
        
        html = """
        <!DOCTYPE html>
        <html>
        <head>
            <title>Nova Python Server</title>
            <style>
                body {
                    font-family: 'Inter', sans-serif;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    color: white;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    margin: 0;
                }
                .container {
                    text-align: center;
                    background: rgba(255, 255, 255, 0.1);
                    padding: 3rem;
                    border-radius: 20px;
                    backdrop-filter: blur(10px);
                }
                h1 { font-size: 3rem; margin-bottom: 1rem; }
                p { font-size: 1.2rem; }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>🐍 Python on Nova!</h1>
                <p>This Python web server is running in a Nova container</p>
                <p>Startup time: <strong>~50ms</strong></p>
            </div>
        </body>
        </html>
        """
        
        self.wfile.write(html.encode())

def run(port=8080):
    server = HTTPServer(('0.0.0.0', port), NovaHandler)
    print(f'🚀 Python server running on http://localhost:{port}')
    server.serve_forever()

if __name__ == '__main__':
    run()
