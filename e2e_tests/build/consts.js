"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.CONFIG = exports.IDEAL_CORES_SOLD = exports.TIMESLICE_PERIOD = exports.CORE_COUNT = exports.INITIAL_PRICE = exports.UNIT = void 0;
const UNIT = 10 ** 12; // ROC has 12 decimals
exports.UNIT = UNIT;
const INITIAL_PRICE = 50 * UNIT;
exports.INITIAL_PRICE = INITIAL_PRICE;
const CORE_COUNT = 10;
exports.CORE_COUNT = CORE_COUNT;
const TIMESLICE_PERIOD = 80;
exports.TIMESLICE_PERIOD = TIMESLICE_PERIOD;
const IDEAL_CORES_SOLD = 5;
exports.IDEAL_CORES_SOLD = IDEAL_CORES_SOLD;
const CONFIG = {
    advance_notice: 20,
    interlude_length: 0,
    leadin_length: 10,
    ideal_bulk_proportion: 0,
    limit_cores_offered: 50,
    region_length: 30,
    renewal_bump: 10,
    contribution_timeout: 5,
};
exports.CONFIG = CONFIG;
