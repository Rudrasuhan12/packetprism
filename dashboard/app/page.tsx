import { PacketLogSchema } from "./validators/packet";

// Mock packets that follow the shared contract
const fakePackets = [
  {
    src_ip: "192.168.1.10",
    dst_ip: "8.8.8.8",
    protocol: "TCP",
    length: 90,
    timestamp: new Date().toISOString(),
  },
  {
    src_ip: "10.0.0.5",
    dst_ip: "1.1.1.1",
    protocol: "UDP",
    length: 120,
    timestamp: new Date().toISOString(),
  },
  // This one would be rejected if protocol was invalid
];

// Validate packets using Zod
const validatedPackets = fakePackets.filter((pkt) => {
  const result = PacketLogSchema.safeParse(pkt);
  return result.success;
});

export default function Home() {
  return (
    <main style={{ padding: "2rem" }}>
      <h2 style={{ fontSize: "20px", marginBottom: "1rem" }}>
        Captured Packets (Mock – Contract Validated)
      </h2>

      <table
        style={{
          width: "100%",
          borderCollapse: "collapse",
        }}
      >
        <thead>
          <tr>
            <th style={{ borderBottom: "1px solid #ccc" }}>Source IP</th>
            <th style={{ borderBottom: "1px solid #ccc" }}>Destination IP</th>
            <th style={{ borderBottom: "1px solid #ccc" }}>Protocol</th>
            <th style={{ borderBottom: "1px solid #ccc" }}>Length (bytes)</th>
            <th style={{ borderBottom: "1px solid #ccc" }}>Timestamp</th>
          </tr>
        </thead>
        <tbody>
          {validatedPackets.map((pkt, index) => (
            <tr key={index}>
              <td style={{ textAlign: "center" }}>{pkt.src_ip}</td>
              <td style={{ textAlign: "center" }}>{pkt.dst_ip}</td>
              <td style={{ textAlign: "center" }}>{pkt.protocol}</td>
              <td style={{ textAlign: "center" }}>{pkt.length}</td>
              <td style={{ textAlign: "center" }}>{pkt.timestamp}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </main>
  );
}
