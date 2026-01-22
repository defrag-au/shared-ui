
export function detectWallets() {
    const wallets = [];
    if (typeof window !== 'undefined' && window.cardano) {
        const knownWallets = ['nami', 'eternl', 'lace', 'flint', 'typhon', 'vespr', 'nufi', 'gerowallet', 'yoroi'];
        for (const name of knownWallets) {
            if (window.cardano[name]) {
                wallets.push(name);
            }
        }
    }
    return wallets;
}

export async function enableWallet(name) {
    if (typeof window === 'undefined' || !window.cardano || !window.cardano[name]) {
        throw new Error(`Wallet ${name} not found`);
    }
    return await window.cardano[name].enable();
}

export async function getNetworkId(api) {
    return await api.getNetworkId();
}

export async function getUsedAddresses(api) {
    return await api.getUsedAddresses();
}

export async function getChangeAddress(api) {
    return await api.getChangeAddress();
}

export async function getBalance(api) {
    return await api.getBalance();
}

export async function signTx(api, txHex, partialSign) {
    return await api.signTx(txHex, partialSign);
}

export async function signData(api, address, payload) {
    return await api.signData(address, payload);
}

export async function submitTx(api, txHex) {
    return await api.submitTx(txHex);
}
