import express from 'express';
import http from 'http';
import url from 'url';
import path from 'path';
import fs from 'fs';

const port = 8362;

const app = express();
const server = http.createServer(app);

const __filename = url.fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

app.use(express.json());
app.use(express.static(path.join(__dirname, '../public')));

server.listen(port, () => {
    console.log(`listening on http://localhost:${port}`);
});

const routes = path.join(__dirname, 'routes');
fs.readdirSync(routes).forEach(async (file) => {
    if (!file.endsWith('.js')) {
        return;
    }

    const route = await import('./routes/' + file);
    app.use(route.default);
});