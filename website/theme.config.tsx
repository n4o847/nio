import React from "react";
import { DocsThemeConfig } from "nextra-theme-docs";

const config: DocsThemeConfig = {
  logo: <span>Nio</span>,
  project: {
    link: "https://github.com/n4o847/nio",
  },
  // https://github.com/shuding/nextra/issues/1210
  toc: {
    extraContent: <></>,
  },
  footer: {
    text: <>&copy; 2019 n4o847</>,
  },
  i18n: [],
};

export default config;
