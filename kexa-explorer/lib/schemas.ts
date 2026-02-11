import { z } from 'zod';

export const HealthSchema = z.object({
  ok: z.boolean(),
}).passthrough();

export const TipSchema = z.object({
  height: z.number(),
  hash: z.string(),
}).passthrough();

export const BlockSummarySchema = z.object({
  height: z.number(),
  hash: z.string(),
  tx_count: z.number().optional().default(0),
  timestamp: z.union([z.string(), z.number()]).optional(),
}).passthrough();

export const BlockDetailSchema = z.object({
  height: z.number().optional(),
  hash: z.string(),
  tx_count: z.number().optional(),
  timestamp: z.union([z.string(), z.number()]).optional(),
}).passthrough();

export const PeersLiveSchema = z.array(z.string());

export const OptionalPeersSchema = z.union([
  z.object({ supported: z.literal(false), peers: z.array(z.string()).optional() }),
  z.object({ supported: z.literal(true), peers: z.array(z.string()) }),
]);

export type Health = z.infer<typeof HealthSchema>;
export type Tip = z.infer<typeof TipSchema>;
export type BlockSummary = z.infer<typeof BlockSummarySchema>;
export type BlockDetail = z.infer<typeof BlockDetailSchema>;
