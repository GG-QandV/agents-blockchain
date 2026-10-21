/**
 * Приклад клієнта для unix socket API μ-daemon (TypeScript/Node.js).
 *
 * Використання:
 *   npx ts-node mu-client.ts <socket_path> <recipient> <amount_minor> <agent_id>
 *
 * Залежності: Node.js 18+ з net module.
 */

import * as net from 'net';
import * as process from 'process';

function buildFrame(
  recipient: string,
  amount: bigint,
  chainId: number,
  agentId: string,
  nonce: number,
  ts: number,
  resource: string | null = null,
): Buffer {
  const payload = Buffer.alloc(4096);
  let off = 0;

  // version u16 BE
  payload.writeUInt16BE(2, off); off += 2;
  // recipient len u8 + data
  payload[off++] = recipient.length;
  payload.write(recipient, off, recipient.length, 'ascii'); off += recipient.length;
  // amount u128 BE (hi+lo)
  payload.writeBigUInt64BE(amount >> 64n, off); off += 8;
  payload.writeBigUInt64BE(amount & 0xFFFFFFFFFFFFFFFFn, off); off += 8;
  // chain_id u64 BE
  payload.writeBigUInt64BE(BigInt(chainId), off); off += 8;
  // agent_id len + data
  payload[off++] = agentId.length;
  payload.write(agentId, off, agentId.length, 'ascii'); off += agentId.length;
  // nonce u64 BE
  payload.writeBigUInt64BE(BigInt(nonce), off); off += 8;
  // ts u64 BE
  payload.writeBigUInt64BE(BigInt(ts), off); off += 8;
  // resource len u8 + data (v2; 0 = none)
  const res = Buffer.from(resource ?? '', 'ascii');
  payload[off++] = res.length;
  res.copy(payload, off); off += res.length;
  // signature (test/fake)
  off += 64;

  const body = payload.subarray(0, off);
  const header = Buffer.alloc(4);
  header.writeUInt32LE(body.length, 0);
  return Buffer.concat([header, body]);
}

function main() {
  const args = process.argv.slice(2);
  if (args.length < 4) {
    console.log(`Usage: ts-node mu-client.ts <socket> <recipient> <amount> <agent_id>`);
    process.exit(1);
  }

  const [sockPath, recipient, amountStr, agentId, resource] = args;
  const amount = BigInt(amountStr);

  const frame = buildFrame(recipient, amount, 8453, agentId, 1, 1000, resource ?? null);
  const sock = net.createConnection(sockPath);

  sock.on('connect', () => {
    sock.write(frame);
  });

  let buf = Buffer.alloc(0);
  sock.on('data', (data: Buffer) => {
    buf = Buffer.concat([buf, data]);
    const code = buf[0];
    if (code === 0x20 && buf.length >= 3) {
      const n = buf.readUInt16LE(1);
      if (buf.length < 3 + n) return; // wait for the rest
      console.log(`OK: settled; header: ${buf.subarray(3, 3 + n).toString('utf8')}`);
      sock.end();
      return;
    }
    const codes: Record<number, string> = {
      0x00: 'Parse error',
      0x01: 'Unknown agent',
      0x02: 'Bad signature',
      0x03: 'Stale timestamp',
      0x04: 'Replay nonce',
      0x05: 'Rate limited',
      0x06: 'Busy',
    };
    if (code === 0xFF) {
      console.log('OK: intent accepted');
    } else {
      console.log(`Denied: ${codes[code] ?? `unknown ${'0x' + code.toString(16)}`}`);
    }
    sock.end();
  });

  sock.on('error', (err) => {
    console.error('Socket error:', err.message);
    process.exit(1);
  });
}

main();
