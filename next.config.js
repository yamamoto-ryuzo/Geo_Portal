/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  output: process.env.VERCEL ? undefined : 'export', // Vercel以外（ローカル等）でのみ静的エクスポートを有効化
};

module.exports = nextConfig;
