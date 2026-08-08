#!/usr/bin/env node

const { spawnSync } = require('child_process');
const path = require('path');
const os = require('os');
const fs = require('fs');

function exitFrom(result, command) {
    if (result.error) {
        console.error(`[Vesper Error] Failed to launch ${command}: ${result.error.message}`);
        process.exit(1);
    }
    process.exit(result.status ?? 1);
}

const extension = os.platform() === 'win32' ? '.exe' : '';
const binPath = path.join(__dirname, `vesper${extension}`);

if (!fs.existsSync(binPath)) {
    console.warn('[Vesper Warn] Pre-built binary not found. Falling back to local cargo run...');
    const result = spawnSync('cargo', ['run', '--release', '--', ...process.argv.slice(2)], {
        stdio: 'inherit',
        cwd: path.join(__dirname, '..')
    });
    exitFrom(result, 'cargo');
}

// 스크립트 실행 인자를 Rust 바이너리로 모두 전달하고, TUI 화면(stdio)을 온전히 매핑합니다.
const result = spawnSync(binPath, process.argv.slice(2), {
    stdio: 'inherit'
});

exitFrom(result, binPath);
