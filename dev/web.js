import http from 'http';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { networkInterfaces } from 'os';


const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const PORT = 3000;
const PKG_DIR = path.join(__dirname, '../pkg/web');
const WEB_DIR = path.join(__dirname, 'web');

const MIME_TYPES = {
  '.html': 'text/html',
  '.js': 'application/javascript',
  '.json': 'application/json',
  '.wasm': 'application/wasm',
  '.css': 'text/css',
};

const server = http.createServer((req, res) => {
  console.log(`${req.method} ${req.url}`);

  // Serve index.html
  if (req.url === '/' || req.url === '/index.html') {
    const filePath = path.join(WEB_DIR, 'index.html');
    serveFile(filePath, res);
    return;
  }

  // Simulate CDN - serve files from pkg/web directory
  if (req.url.startsWith('/cdn/')) {
    const fileName = req.url.replace('/cdn/', '');
    const filePath = path.join(PKG_DIR, fileName);
    serveFile(filePath, res);
    return;
  }

  // 404
  res.writeHead(404, { 'Content-Type': 'text/plain' });
  res.end('Not Found');
});

function serveFile(filePath, res) {
  fs.readFile(filePath, (err, data) => {
    if (err) {
      console.error(`Error reading file: ${filePath}`, err);
      res.writeHead(404, { 'Content-Type': 'text/plain' });
      res.end('File Not Found');
      return;
    }

    const ext = path.extname(filePath);
    const contentType = MIME_TYPES[ext] || 'application/octet-stream';
    
    res.writeHead(200, { 
      'Content-Type': contentType,
      'Access-Control-Allow-Origin': '*'
    });
    res.end(data);
  });
}

server.listen(PORT, () => {
  const interfaces = networkInterfaces();
  console.log(`Dev server running at:`);
  console.log(Object.values(interfaces).flat().map(i => `- http://${i.address}:${PORT}/`).join("\n"));
});
