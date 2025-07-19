import type { MDXComponents } from "nextra/mdx-components";
import { useMDXComponents as getThemeComponents } from "nextra-theme-docs";

const themeComponents = getThemeComponents();

export function useMDXComponents(components?: MDXComponents) {
  return {
    ...themeComponents,
    ...components,
  };
}
