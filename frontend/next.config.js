/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  output: "standalone", // Enable standalone output for Docker
  env: {
    NEXT_PUBLIC_WS_URL:
      process.env.NEXT_PUBLIC_WS_URL || "ws://localhost:8080/ws",
  },
  // Next.js 15 uses App Router by default, but we're using Pages Router
  // No experimental flags needed for Pages Router in Next.js 15
};

module.exports = nextConfig;
