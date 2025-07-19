import type { LayoutProps } from ".next/types/app/layout";
import type { Metadata } from "next";
import { Footer, Layout, Navbar } from "nextra-theme-docs";
import "nextra-theme-docs/style.css";
import { Head } from "nextra/components";
import { getPageMap } from "nextra/page-map";
import "../styles/globals.css";

export const metadata: Metadata = {
  title: {
    default: "Nio",
    template: "%s \u2013 Nio",
  },
};

export default async function RootLayout({ children }: LayoutProps) {
  const navbar = (
    <Navbar
      logo={<span>Nio</span>}
      projectLink="https://github.com/n4o847/nio"
    />
  );
  const pageMap = await getPageMap();
  return (
    <html lang="en" dir="ltr" suppressHydrationWarning>
      <Head />
      <body>
        <Layout
          navbar={navbar}
          footer={<Footer>&copy; 2019 n4o847</Footer>}
          docsRepositoryBase="https://github.com/n4o847/nio/tree/main/website"
          sidebar={{ defaultMenuCollapseLevel: 1 }}
          pageMap={pageMap}
        >
          {children}
        </Layout>
      </body>
    </html>
  );
}
