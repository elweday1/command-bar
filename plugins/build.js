#!/usr/bin/env node

import fs from 'fs';
import path from 'path';
import { spawn } from 'child_process';
import chokidar from 'chokidar';
import os from 'os';
import { glob } from 'glob';
import { fileURLToPath } from 'url';

const BUILD_DIR = path.join(os.homedir(), '.config', 'dossier', 'plugins');
const WATCH_MODE = process.argv.includes('--watch');
const SCRIPT_DIR = path.dirname(fileURLToPath(import.meta.url));
const PLUGINS_DIR = process.argv[2] || SCRIPT_DIR;

function ensureDir(dir) {
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
}

function getPluginDirs() {
    return fs.readdirSync(PLUGINS_DIR, { withFileTypes: true })
        .filter(dirent => dirent.isDirectory() && fs.existsSync(path.join(PLUGINS_DIR, dirent.name, 'Cargo.toml')))
        .map(dirent => path.join(PLUGINS_DIR, dirent.name));
}

function buildPlugin(pluginDir) {
    return new Promise((resolve) => {
        console.log(`[${pluginDir}] Building...`);
        
        const cargo = spawn('cargo', ['build', '--release'], {
            cwd: pluginDir,
            stdio: 'inherit'
        });

        cargo.on('close', (code) => {
            if (code === 0) {
                const targetDir = path.join(pluginDir, 'target', 'release');
                const extensions = process.platform === 'win32' ? ['dll'] : ['so', 'dylib'];
                
                let copied = false;
                for (const ext of extensions) {
                    const pattern = path.join(targetDir, `*.${ext}`);
                    const files = glob.sync(pattern);
                    
                    if (files.length > 0) {
                        files.forEach(file => {
                            const dest = path.join(BUILD_DIR, path.basename(file));
                            fs.copyFileSync(file, dest);
                        });
                        copied = true;
                        break;
                    }
                }
                
                if (copied) {
                    console.log(`[${pluginDir}] ✓ Built and copied to ${BUILD_DIR}`);
                } else {
                    console.log(`[${pluginDir}] ⚠ No library found after build`);
                }
            } else {
                console.log(`[${pluginDir}] ✗ Build failed`);
            }
            resolve();
        });
    });
}

async function buildAll() {
    console.log(`Building all plugins to ${BUILD_DIR}...`);
    console.log();
    
    ensureDir(BUILD_DIR);
    
    const plugins = getPluginDirs();
    for (const plugin of plugins) {
        await buildPlugin(plugin);
    }
    
    console.log();
    console.log('Build complete! Plugin libraries are in:');
    console.log(BUILD_DIR);
}

function watchFiles() {
    console.log('Watching for changes in plugin files...');
    console.log('Press Ctrl+C to stop watching');
    console.log();
    
    ensureDir(BUILD_DIR);
    
    const pluginDirs = getPluginDirs();
    const watchPaths = pluginDirs.map(dir => `${dir}/*.rs`);
    
    const watcher = chokidar.watch(watchPaths, {
        persistent: true
    });
    
    const debounce = new Map();
    
    watcher.on('change', (filePath) => {
        const pluginDir = path.dirname(filePath);
        
        if (!fs.existsSync(path.join(pluginDir, 'Cargo.toml'))) {
            return;
        }
        
        // Debounce builds
        if (debounce.has(pluginDir)) {
            clearTimeout(debounce.get(pluginDir));
        }
        
        debounce.set(pluginDir, setTimeout(() => {
            console.log(`[${path.basename(pluginDir)}] Change detected, rebuilding...`);
            buildPlugin(pluginDir);
            debounce.delete(pluginDir);
        }, 500));
    });
}

watchFiles();