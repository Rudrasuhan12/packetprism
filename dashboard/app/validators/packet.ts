import { z } from "zod";

export const PacketLogSchema = z.object({
  src_ip: z.string(),
  dst_ip: z.string(),
  protocol: z.enum(["TCP", "UDP"]),
  length: z.number(),
  timestamp: z.string(),
});

export type PacketLogValidated = z.infer<typeof PacketLogSchema>;
