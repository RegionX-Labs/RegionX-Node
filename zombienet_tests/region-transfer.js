"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try {
            step(generator.next(value));
        }
        catch (e) {
            reject(e);
        } }
        function rejected(value) { try {
            step(generator["throw"](value));
        }
        catch (e) {
            reject(e);
        } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function () { if (t[0] & 1)
            throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function () { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f)
            throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _)
            try {
                if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done)
                    return t;
                if (y = 0, t)
                    op = [op[0] & 2, t.value];
                switch (op[0]) {
                    case 0:
                    case 1:
                        t = op;
                        break;
                    case 4:
                        _.label++;
                        return { value: op[1], done: false };
                    case 5:
                        _.label++;
                        y = op[1];
                        op = [0];
                        continue;
                    case 7:
                        op = _.ops.pop();
                        _.trys.pop();
                        continue;
                    default:
                        if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) {
                            _ = 0;
                            continue;
                        }
                        if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) {
                            _.label = op[1];
                            break;
                        }
                        if (op[0] === 6 && _.label < t[1]) {
                            _.label = t[1];
                            t = op;
                            break;
                        }
                        if (t && _.label < t[2]) {
                            _.label = t[2];
                            _.ops.push(op);
                            break;
                        }
                        if (t[2])
                            _.ops.pop();
                        _.trys.pop();
                        continue;
                }
                op = body.call(thisArg, _);
            }
            catch (e) {
                op = [6, e];
                y = 0;
            }
            finally {
                f = t = 0;
            }
        if (op[0] & 5)
            throw op[1];
        return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.run = void 0;
var api_1 = require("@polkadot/api");
var consts_1 = require("./consts");
var common_1 = require("./common");
var coretime_utils_1 = require("coretime-utils");
var node_assert_1 = require("node:assert");
function run(_nodeName, networkInfo, _jsArgs) {
    return __awaiter(this, void 0, void 0, function () {
        var regionXUri, coretimeUri, rococoUri, regionXApi, rococoApi, coretimeApi, keyring, alice, setCoretimeXcmVersion, setBalanceCall, regionId, encodedId, receiverKeypair, feeAssetItem, weightLimit, reserveTransfer, regions;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    regionXUri = networkInfo.nodesByName["regionx-collator01"].wsUri;
                    coretimeUri = networkInfo.nodesByName["coretime-collator01"].wsUri;
                    rococoUri = networkInfo.nodesByName["rococo-validator01"].wsUri;
                    return [4 /*yield*/, api_1.ApiPromise.create({ provider: new api_1.WsProvider(regionXUri) })];
                case 1:
                    regionXApi = _a.sent();
                    return [4 /*yield*/, api_1.ApiPromise.create({ provider: new api_1.WsProvider(rococoUri) })];
                case 2:
                    rococoApi = _a.sent();
                    return [4 /*yield*/, api_1.ApiPromise.create({ provider: new api_1.WsProvider(coretimeUri), types: { Id: coretime_utils_1.Id } })];
                case 3:
                    coretimeApi = _a.sent();
                    keyring = new api_1.Keyring({ type: "sr25519" });
                    alice = keyring.addFromUri("//Alice");
                    setCoretimeXcmVersion = coretimeApi.tx.polkadotXcm.forceDefaultXcmVersion([3]);
                    return [4 /*yield*/, (0, common_1.submitExtrinsic)(alice, coretimeApi.tx.sudo.sudo(setCoretimeXcmVersion), {})];
                case 4:
                    _a.sent();
                    return [4 /*yield*/, openHrmpChannel(alice, rococoApi, 1005, 2000)];
                case 5:
                    _a.sent();
                    return [4 /*yield*/, openHrmpChannel(alice, rococoApi, 2000, 1005)];
                case 6:
                    _a.sent();
                    return [4 /*yield*/, configureBroker(coretimeApi, alice)];
                case 7:
                    _a.sent();
                    return [4 /*yield*/, startSales(coretimeApi, alice)];
                case 8:
                    _a.sent();
                    setBalanceCall = coretimeApi.tx.balances.forceSetBalance(alice.address, 1000 * consts_1.UNIT);
                    return [4 /*yield*/, (0, common_1.submitExtrinsic)(alice, coretimeApi.tx.sudo.sudo(setBalanceCall), {})];
                case 9:
                    _a.sent();
                    return [4 /*yield*/, purchaseRegion(coretimeApi, alice)];
                case 10:
                    regionId = _a.sent();
                    encodedId = (0, coretime_utils_1.getEncodedRegionId)(regionId, coretimeApi).toString();
                    receiverKeypair = new api_1.Keyring();
                    receiverKeypair.addFromAddress(alice.address);
                    feeAssetItem = 0;
                    weightLimit = "Unlimited";
                    reserveTransfer = coretimeApi.tx.polkadotXcm.limitedReserveTransferAssets({ V3: { parents: 1, interior: { X1: { Parachain: 2000 } } } }, //dest
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
                    return [4 /*yield*/, (0, common_1.submitExtrinsic)(alice, reserveTransfer, {})];
                case 11:
                    _a.sent();
                    return [4 /*yield*/, (0, common_1.sleep)(5000)];
                case 12:
                    _a.sent();
                    return [4 /*yield*/, regionXApi.query.regions.regions.entries()];
                case 13:
                    regions = (_a.sent());
                    node_assert_1.default.equal(regions.length, 1);
                    node_assert_1.default.deepStrictEqual(regions[0][0].toHuman(), [{ begin: '34', core: '0', mask: "0xffffffffffffffffffff" }]);
                    node_assert_1.default.deepStrictEqual(regions[0][1].toHuman(), { owner: alice.address, record: 'Pending' });
                    return [2 /*return*/];
            }
        });
    });
}
exports.run = run;
function openHrmpChannel(signer, relayApi, senderParaId, recipientParaId) {
    return __awaiter(this, void 0, void 0, function () {
        var newHrmpChannel, openHrmp, sudoCall, callTx;
        var _a;
        var _this = this;
        return __generator(this, function (_b) {
            newHrmpChannel = [
                senderParaId,
                recipientParaId,
                8, // Max capacity
                102400, // Max message size
            ];
            openHrmp = (_a = relayApi.tx.parasSudoWrapper).sudoEstablishHrmpChannel.apply(_a, newHrmpChannel);
            sudoCall = relayApi.tx.sudo.sudo(openHrmp);
            callTx = function (resolve) {
                return __awaiter(_this, void 0, void 0, function () {
                    var unsub;
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, sudoCall.signAndSend(signer, function (result) {
                                    if (result.status.isInBlock) {
                                        unsub();
                                        resolve();
                                    }
                                })];
                            case 1:
                                unsub = _a.sent();
                                return [2 /*return*/];
                        }
                    });
                });
            };
            return [2 /*return*/, new Promise(callTx)];
        });
    });
}
function configureBroker(coretimeApi, signer) {
    return __awaiter(this, void 0, void 0, function () {
        var configCall, sudo;
        return __generator(this, function (_a) {
            configCall = coretimeApi.tx.broker.configure(consts_1.CONFIG);
            sudo = coretimeApi.tx.sudo.sudo(configCall);
            return [2 /*return*/, (0, common_1.submitExtrinsic)(signer, sudo, {})];
        });
    });
}
function startSales(coretimeApi, signer) {
    return __awaiter(this, void 0, void 0, function () {
        var startSaleCall, sudo;
        return __generator(this, function (_a) {
            startSaleCall = coretimeApi.tx.broker.startSales(consts_1.INITIAL_PRICE, consts_1.CORE_COUNT);
            sudo = coretimeApi.tx.sudo.sudo(startSaleCall);
            return [2 /*return*/, (0, common_1.submitExtrinsic)(signer, sudo, {})];
        });
    });
}
function purchaseRegion(coretimeApi, buyer) {
    return __awaiter(this, void 0, void 0, function () {
        var callTx;
        var _this = this;
        return __generator(this, function (_a) {
            callTx = function (resolve) {
                return __awaiter(_this, void 0, void 0, function () {
                    var purchase, unsub;
                    var _this = this;
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0:
                                purchase = coretimeApi.tx.broker.purchase(consts_1.INITIAL_PRICE * 2);
                                return [4 /*yield*/, purchase.signAndSend(buyer, function (result) {
                                        return __awaiter(_this, void 0, void 0, function () {
                                            var regionId;
                                            return __generator(this, function (_a) {
                                                switch (_a.label) {
                                                    case 0:
                                                        if (!result.status.isInBlock)
                                                            return [3 /*break*/, 2];
                                                        return [4 /*yield*/, getRegionId(coretimeApi)];
                                                    case 1:
                                                        regionId = _a.sent();
                                                        unsub();
                                                        resolve(regionId);
                                                        _a.label = 2;
                                                    case 2: return [2 /*return*/];
                                                }
                                            });
                                        });
                                    })];
                            case 1:
                                unsub = _a.sent();
                                return [2 /*return*/];
                        }
                    });
                });
            };
            return [2 /*return*/, new Promise(callTx)];
        });
    });
}
function getRegionId(coretimeApi) {
    return __awaiter(this, void 0, void 0, function () {
        var events, _i, events_1, record, event_1, data;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, coretimeApi.query.system.events()];
                case 1:
                    events = _a.sent();
                    for (_i = 0, events_1 = events; _i < events_1.length; _i++) {
                        record = events_1[_i];
                        event_1 = record.event;
                        if (event_1.section === "broker" && event_1.method === "Purchased") {
                            data = event_1.data[1].toHuman();
                            return [2 /*return*/, { begin: data.begin, core: data.core, mask: new coretime_utils_1.CoreMask(data.mask) }];
                        }
                    }
                    return [2 /*return*/, { begin: 0, core: 0, mask: coretime_utils_1.CoreMask.voidMask() }];
            }
        });
    });
}
