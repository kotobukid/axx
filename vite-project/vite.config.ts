import {defineConfig} from 'vite'
import vue from '@vitejs/plugin-vue'

import dotenv from 'dotenv'
import process from 'node:process';

dotenv.config()

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [vue()],
    server: {
        host: process.env.VITE_API_HOST || 'localhost',
        port: 3001,

        proxy: {
            '/api': {
                target: `http://${process.env.VITE_API_HOST || 'localhost'}:3000`,
                changeOrigin: true,
            },
        }
    }
})