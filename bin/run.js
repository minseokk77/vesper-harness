#!/usr/bin/env node

const { spawnSync } = require('child_process');
const path = require('path');
const os = require('os');
const fs = require('fs');

const extension = os.platform() === 'win32' ? '.exe' : '';
const binPath = path.join(__dirname, `vesper${extension}`);

if (!fs.existsSync(binPath)) {
    console.error('[Vesper Error] Binary not found. Did the installation fail?');
    console.error(`Expected binary at: ${binPath}`);
    console.error('Try running: cargo build --release');
    process.exit(1);
}

// 스크립트 실행 인자를 Rust 바이너리로 모두 전달하고, TUI 화면(stdio)을 온전히 매핑합니다.
const result = spawnSync(binPath, process.argv.slice(2), {
    stdio: 'inherit'
});

process.exit(result.status || 0);
