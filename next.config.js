/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  output: 'export', // Vercelでもローカルでも常に静的エクスポート
};

module.exports = nextConfig;
