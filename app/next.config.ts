import type { NextConfig } from "next";
const { version } = require('./package.json');

const nextConfig: NextConfig = {
  /* config options here */
  env:{
    version
  }
};

export default nextConfig;
