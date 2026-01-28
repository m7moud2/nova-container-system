#!/usr/bin/env python3
"""
Simple Flask app to test Nova's Python runtime
Run with: nova run test_python_app.py
"""

from flask import Flask, jsonify
import datetime

app = Flask(__name__)

@app.route('/')
def home():
    return '''
    <html>
    <head>
        <title>Nova Python Test</title>
        <style>
            body {
                font-family: Arial, sans-serif;
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
            <p>This Flask app is running in Nova container</p>
            <p>Startup time: <strong>~50ms</strong></p>
        </div>
    </body>
    </html>
    '''

@app.route('/api/status')
def status():
    return jsonify({
        'status': 'running',
        'runtime': 'Python + Flask',
        'container': 'Nova',
        'time': datetime.datetime.now().isoformat()
    })

if __name__ == '__main__':
    print("🚀 Starting Flask app on Nova...")
    app.run(host='0.0.0.0', port=5000, debug=True)
