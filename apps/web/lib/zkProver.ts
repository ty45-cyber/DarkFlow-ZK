import { compileCompactWitness } from '@midnightntwrk/compact-js';

export interface PrivateOrder {
  price: bigint;
  quantity: bigint;
  side: 'BUY' | 'SELL';
  secretSalt: string;
}

export async function generateOrderProof(order: PrivateOrder, walletAddress: string) {
  try {
    // Construct local private witness input
    const witnessData = {
      traderKey: walletAddress,
      orderType: order.side === 'BUY' ? 0 : 1,
      limitPrice: order.price,
      quantity: order.quantity,
      salt: order.secretSalt,
    };

    // Execute local proof generation in WASM
    const proof = await compileCompactWitness({
      circuitName: 'executePrivateOrder',
      witnessInputs: witnessData,
    });

    return proof;
  } catch (error) {
    console.error('Failed to generate zero-knowledge proof locally:', error);
    throw error;
  }
}