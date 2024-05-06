"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.run = void 0;
const api_1 = require("@polkadot/api");
const consts_1 = require("./consts");
const common_1 = require("./common");
const coretime_utils_1 = require("coretime-utils");
const node_assert_1 = __importDefault(require("node:assert"));
async function run(_nodeName, networkInfo, _jsArgs) {
    const { wsUri: regionXUri } = networkInfo.nodesByName["regionx-collator01"];
    const { wsUri: coretimeUri } = networkInfo.nodesByName["coretime-collator01"];
    const { wsUri: rococoUri } = networkInfo.nodesByName["rococo-validator01"];
    const regionXApi = await api_1.ApiPromise.create({ provider: new api_1.WsProvider(regionXUri) });
    const rococoApi = await api_1.ApiPromise.create({ provider: new api_1.WsProvider(rococoUri) });
    const coretimeApi = await api_1.ApiPromise.create({ provider: new api_1.WsProvider(coretimeUri), types: { Id: coretime_utils_1.Id } });
    // account to submit tx
    const keyring = new api_1.Keyring({ type: "sr25519" });
    const alice = keyring.addFromUri("//Alice");
    const setCoretimeXcmVersion = coretimeApi.tx.polkadotXcm.forceDefaultXcmVersion([3]);
    await (0, common_1.submitExtrinsic)(alice, coretimeApi.tx.sudo.sudo(setCoretimeXcmVersion), {});
    await openHrmpChannel(alice, rococoApi, 1005, 2000);
    await openHrmpChannel(alice, rococoApi, 2000, 1005);
    await configureBroker(coretimeApi, alice);
    await startSales(coretimeApi, alice);
    const setBalanceCall = coretimeApi.tx.balances.forceSetBalance(alice.address, 1000 * consts_1.UNIT);
    await (0, common_1.submitExtrinsic)(alice, coretimeApi.tx.sudo.sudo(setBalanceCall), {});
    const regionId = await purchaseRegion(coretimeApi, alice);
    const encodedId = (0, coretime_utils_1.getEncodedRegionId)(regionId, coretimeApi).toString();
    const receiverKeypair = new api_1.Keyring();
    receiverKeypair.addFromAddress(alice.address);
    const feeAssetItem = 0;
    const weightLimit = "Unlimited";
    const reserveTransfer = coretimeApi.tx.polkadotXcm.limitedReserveTransferAssets({ V3: { parents: 1, interior: { X1: { Parachain: 2000 } } } }, //dest
    {
        V3: {
            parents: 0,
            interior: {
                X1: {
                    AccountId32: {
                        chain: "Any",
                        id: receiverKeypair.pairs[0].publicKey,
                    },
                },
            },
        },
    }, //beneficiary
    {
        V3: [
            {
                id: {
                    Concrete: {
                        parents: 0,
                        interior: { X1: { PalletInstance: 50 } },
                    },
                },
                fun: {
                    NonFungible: {
                        Index: encodedId
                    }
                },
            },
        ],
    }, //asset
    feeAssetItem, weightLimit);
    await (0, common_1.submitExtrinsic)(alice, reserveTransfer, {});
    await (0, common_1.sleep)(5000);
    const regions = (await regionXApi.query.regions.regions.entries());
    node_assert_1.default.equal(regions.length, 1);
    node_assert_1.default.deepStrictEqual(regions[0][0].toHuman(), [{ begin: '34', core: '0', mask: "0xffffffffffffffffffff" }]);
    node_assert_1.default.deepStrictEqual(regions[0][1].toHuman(), { owner: alice.address, record: 'Pending' });
}
exports.run = run;
async function openHrmpChannel(signer, relayApi, senderParaId, recipientParaId) {
    const newHrmpChannel = [
        senderParaId,
        recipientParaId,
        8, // Max capacity
        102400, // Max message size
    ];
    const openHrmp = relayApi.tx.parasSudoWrapper.sudoEstablishHrmpChannel(...newHrmpChannel);
    const sudoCall = relayApi.tx.sudo.sudo(openHrmp);
    const callTx = async (resolve) => {
        const unsub = await sudoCall.signAndSend(signer, (result) => {
            if (result.status.isInBlock) {
                unsub();
                resolve();
            }
        });
    };
    return new Promise(callTx);
}
async function configureBroker(coretimeApi, signer) {
    const configCall = coretimeApi.tx.broker.configure(consts_1.CONFIG);
    const sudo = coretimeApi.tx.sudo.sudo(configCall);
    return (0, common_1.submitExtrinsic)(signer, sudo, {});
}
async function startSales(coretimeApi, signer) {
    const startSaleCall = coretimeApi.tx.broker.startSales(consts_1.INITIAL_PRICE, consts_1.CORE_COUNT);
    const sudo = coretimeApi.tx.sudo.sudo(startSaleCall);
    return (0, common_1.submitExtrinsic)(signer, sudo, {});
}
async function purchaseRegion(coretimeApi, buyer) {
    const callTx = async (resolve) => {
        const purchase = coretimeApi.tx.broker.purchase(consts_1.INITIAL_PRICE * 2);
        const unsub = await purchase.signAndSend(buyer, async (result) => {
            if (result.status.isInBlock) {
                const regionId = await getRegionId(coretimeApi);
                unsub();
                resolve(regionId);
            }
        });
    };
    return new Promise(callTx);
}
async function getRegionId(coretimeApi) {
    const events = await coretimeApi.query.system.events();
    for (const record of events) {
        const { event } = record;
        if (event.section === "broker" && event.method === "Purchased") {
            const data = event.data[1].toHuman();
            return { begin: data.begin, core: data.core, mask: new coretime_utils_1.CoreMask(data.mask) };
        }
    }
    return { begin: 0, core: 0, mask: coretime_utils_1.CoreMask.voidMask() };
}
