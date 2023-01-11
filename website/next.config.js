const withNextra = require("nextra")({
  theme: "nextra-theme-docs",
  themeConfig: "./theme.config.tsx",
});

module.exports = withNextra({
  basePath: process.env.NODE_ENV === "production" ? "/nio" : "",
  images: {
    unoptimized: true,
  },
});
