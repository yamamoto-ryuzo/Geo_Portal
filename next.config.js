/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  output: 'export', // 静的HTMLエクスポートを有効化（オフライン配布用）
};

module.exports = nextConfig;
