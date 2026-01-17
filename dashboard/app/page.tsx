const fakePackets = [
  { id: 1, size: 74, protocol: "TCP" },
  { id: 2, size: 1500, protocol: "UDP" },
  { id: 3, size: 98, protocol: "ICMP" },
];

export default function Home() {
  return (
    <main style={{ padding: "2rem" }}>
      <h2 style={{ fontSize: "20px", marginBottom: "1rem" }}>
        Captured Packets (Mock)
      </h2>

      <table
        style={{
          width: "100%",
          borderCollapse: "collapse",
        }}
      >
        <thead>
          <tr>
            <th style={{ borderBottom: "1px solid #ccc" }}>ID</th>
            <th style={{ borderBottom: "1px solid #ccc" }}>Size (bytes)</th>
            <th style={{ borderBottom: "1px solid #ccc" }}>Protocol</th>
          </tr>
        </thead>
        <tbody>
          {fakePackets.map((pkt) => (
            <tr key={pkt.id}>
              <td style={{ textAlign: "center" }}>{pkt.id}</td>
              <td style={{ textAlign: "center" }}>{pkt.size}</td>
              <td style={{ textAlign: "center" }}>{pkt.protocol}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </main>
  );
}
