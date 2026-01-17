export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <header
          style={{
            padding: "1rem",
            backgroundColor: "#0f172a",
            color: "white",
            fontWeight: "bold",
          }}
        >
          PacketPrism Enterprise
        </header>

        <main style={{ minHeight: "80vh" }}>{children}</main>

        <footer
          style={{
            padding: "1rem",
            backgroundColor: "#e5e7eb",
            textAlign: "center",
            fontSize: "14px",
          }}
        >
          © PacketPrism • Network Intelligence Platform
        </footer>
      </body>
    </html>
  );
}
