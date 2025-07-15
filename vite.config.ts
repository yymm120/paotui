import path from "path"
import tailwindcss from "@tailwindcss/vite"
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vite.dev/config/
export default defineConfig({
  // plugins: [react()],
    plugins: [react(), tailwindcss()],
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./src"),
        },
    },
    server: {

        host: '0.0.0.0', // 监听所有网络接口

        // port: 5173,      // 确保端口一致

        strictPort: true,
        hmr: {
            protocol: 'ws',
            host: '127.0.0.1', // 替换为您的实际本地 IP
        }
    }
})
