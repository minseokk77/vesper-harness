const https = require('https');
const fs = require('fs');
const path = require('path');
const os = require('os');
const { execSync } = require('child_process');
const { version } = require('../package.json');

const REPO = 'minseokk77/vesper-harness';
const VERSION = `v${version}`;

const platform = os.platform();
const arch = os.arch();

let binaryName = 'vesper-harness';
let extension = '';

if (platform === 'win32') {
    binaryName += '-windows';
    extension = '.exe';
} else if (platform === 'darwin') {
    binaryName += '-macos';
} else if (platform === 'linux') {
    binaryName += '-linux';
}

if (arch === 'x64') {
    binaryName += '-amd64';
} else if (arch === 'arm64') {
    binaryName += '-arm64';
} else {
    binaryName += '-amd64';
}

binaryName += extension;

const url = `https://github.com/${REPO}/releases/download/${VERSION}/${binaryName}`;
const binDir = path.join(__dirname, '..', 'bin');
const dest = path.join(binDir, `vesper${extension}`);

if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
}

console.log(`[Vesper] Fetching binary for ${platform} ${arch} from GitHub Releases...`);

function download(url, dest) {
    return new Promise((resolve, reject) => {
        https.get(url, (res) => {
            if (res.statusCode === 301 || res.statusCode === 302) {
                return download(res.headers.location, dest).then(resolve).catch(reject);
            }
            if (res.statusCode !== 200) {
                return reject(new Error(`HTTP Status ${res.statusCode}`));
            }
            const file = fs.createWriteStream(dest);
            res.pipe(file);
            file.on('finish', () => {
                file.close();
                resolve();
            });
        }).on('error', (err) => {
            if (fs.existsSync(dest)) fs.unlinkSync(dest);
            reject(err);
        });
    });
}

download(url, dest)
    .then(() => {
        fs.chmodSync(dest, 0o755);
        console.log('[Vesper] Successfully installed pre-built binary!');
    })
    .catch((err) => {
        console.warn(`\n[Vesper] GitHub Release download failed (${err.message}).`);
        console.log('[Vesper] Attempting to fallback to local cargo build...');
        
        try {
            // 설치 환경에 Rust(cargo)가 있다면 직접 소스를 빌드해서 가져옵니다.
            execSync('cargo build --release', { stdio: 'inherit', cwd: path.join(__dirname, '..') });
            const localBin = path.join(__dirname, '..', 'target', 'release', `vesper-harness${extension}`);
            
            if (fs.existsSync(localBin)) {
                fs.copyFileSync(localBin, dest);
                console.log('[Vesper] Local cargo build successful and binary copied.');
            } else {
                throw new Error('Local binary not found after build.');
            }
        } catch (buildErr) {
            console.error('[Vesper] Fallback build failed. Please ensure Rust is installed or download the binary manually.');
            console.error(buildErr);
            process.exit(1);
        }
    });
