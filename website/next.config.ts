import nextra from "nextra";

const withNextra = nextra({});

export default withNextra({
  basePath: process.env.NODE_ENV === "production" ? "/nio" : "",
  output: "export",
  images: {
    unoptimized: true,
  },
});
