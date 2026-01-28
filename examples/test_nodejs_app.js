#!/usr/bin/env node
/**
 * Simple Express app to test Nova's Node.js runtime
 * Run with: nova run test_nodejs_app.js
 */

const express = require('express');
const app = express();
const port = 3000;

app.get('/', (req, res) => {
    res.send(`
        <!DOCTYPE html>
        <html>
        <head>
            <title>Nova Node.js Test</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
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
                <h1>⚡ Node.js on Nova!</h1>
                <p>This Express app is running in Nova container</p>
                <p>Startup time: <strong>~50ms</strong></p>
            </div>
        </body>
        </html>
    `);
});

app.get('/api/status', (req, res) => {
    res.json({
        status: 'running',
        runtime: 'Node.js + Express',
        container: 'Nova',
        uptime: process.uptime(),
        time: new Date().toISOString()
    });
});

app.listen(port, () => {
    console.log(`🚀 Express app listening at http://localhost:${port}`);
});
