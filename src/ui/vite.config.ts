import { resolve } from 'node:path';
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

export default defineConfig({
  root: __dirname,
  plugins: [react()],
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    cssCodeSplit: false,
    codeSplitting: false,
    rollupOptions: {
      input: resolve(__dirname, 'management/main.tsx'),
      output: {
        entryFileNames: 'management.js',
        assetFileNames: 'management[extname]',
      },
    },
  },
});
