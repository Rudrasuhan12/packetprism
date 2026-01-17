export interface PacketLog {
  src_ip: string;
  dst_ip: string;
  protocol: "TCP" | "UDP";
  length: number;
  timestamp: string;
}
