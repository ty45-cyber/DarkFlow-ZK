'use client';

import { useState } from 'react';
import { generateOrderProof, PrivateOrder } from '../lib/zkProver';

export default function DarkPoolTerminal() {
  const [status, setStatus] = useState<string>('Awaiting order formulation...');
  
  // Dummy EIP-712 / Wallet integration payload
  const TRADER_WALLET = "0xMidnightTraderAddress";

  const executeTrade = async () => {
    try {
      setStatus('Generating zero-knowledge proof locally (WASM)...');
      
      const order: PrivateOrder = {
        price: BigInt(45000), // e.g., BTC limit price
        quantity: BigInt(2),
        side: 'BUY',
        secretSalt: "0xRandomCryptographicSalt123" 
      };

      // 1. Generate the local ZK Proof using Midnight Compact
      const zkProof = await generateOrderProof(order, TRADER_WALLET);
      
      setStatus('Proof generated. Pushing to DarkFlow Gateway for telemetry...');

      // 2. Submit the shielded proof + depth estimation to Rust backend
      const res = await fetch('http://localhost:8080/api/v1/order', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          wallet_address: TRADER_WALLET,
          zk_proof_bytes: JSON.stringify(zkProof), // Serialize the WASM output
          order_depth_distribution: [0.1, 0.4, 0.3, 0.2] // Simulated local volume mapping
        })
      });

      const data = await res.json();
      setStatus(`Order executed. Market Entropy state: ${data.shannon_entropy.toFixed(4)} bits`);

    } catch (error: any) {
      console.error(error);
      setStatus(`Trade Failed: ${error.message}`);
    }
  };

  return (
    <main className="min-h-screen bg-black text-green-400 p-8 font-mono">
      <div className="max-w-3xl mx-auto border border-green-900 p-6 rounded-lg shadow-[0_0_15px_rgba(0,255,0,0.1)]">
        <h1 className="text-3xl font-bold mb-4 tracking-tighter">DarkFlow ZK Terminal</h1>
        <p className="text-gray-400 mb-8">Strictly Incomplete Information. Zero-Knowledge Execution Venue.</p>
        
        <div className="bg-gray-900 p-4 rounded mb-6 text-sm">
          <p>&gt; System Status: {status}</p>
        </div>

        <button 
          onClick={executeTrade}
          className="bg-green-900 hover:bg-green-700 text-black font-bold py-3 px-6 rounded transition-colors w-full"
        >
          EXECUTE SHIELDED ORDER
        </button>
      </div>
    </main>
  );
}