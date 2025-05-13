function createInvoice(amount, currency) {
    return {
        amount,
        currency,
        timestamp: Date.now(),
        status: "Beklemede",
        iban: "TR68 0021 0000 0011 7743 1001 05"
    };
}

module.exports = { createInvoice };