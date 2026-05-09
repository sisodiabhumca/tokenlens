export const metadata = { title: "TokenLens Cloud", description: "Team-wide token-savings dashboard" };

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body style={{ fontFamily: "system-ui, sans-serif", margin: 0, background: "#0b0d10", color: "#e6e8eb" }}>
        <header style={{ padding: "16px 24px", borderBottom: "1px solid #1f2328" }}>
          <strong>TokenLens</strong> Cloud
        </header>
        <main style={{ padding: 24, maxWidth: 1100, margin: "0 auto" }}>{children}</main>
      </body>
    </html>
  );
}
