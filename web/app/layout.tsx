import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "TokenLens — observability + filtering for AI coding agents",
  description:
    "TokenLens watches every prompt your AI coding agents send, strips noise, tracks spend, and enforces budgets across Claude, Codex, Cursor, ChatGPT, Perplexity, and Vercel AI SDK.",
  metadataBase: new URL("https://tokenlens.dev"),
  openGraph: {
    title: "TokenLens",
    description:
      "Universal Hook Protocol + token analytics for Claude Code, Codex, Cursor, and the Vercel AI SDK.",
    url: "https://tokenlens.dev",
    siteName: "TokenLens",
    type: "website",
  },
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
