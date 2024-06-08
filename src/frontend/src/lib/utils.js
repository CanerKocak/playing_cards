export function formatBigDecimalToString2Digits(amount) {
	console.log("amount", amount);

	// Convert the number to a string
	let amountStr = amount.toString();

	// Pad the string with leading zeros to ensure it has at least 9 characters
	// This accounts for 8 decimal places plus at least one digit before the decimal point
	amountStr = amountStr.padStart(9, "0");

	// Insert the decimal point eight places from the end
	let formattedAmount = amountStr.slice(0, -8) + "." + amountStr.slice(-8);

	// Remove trailing zeros after the decimal point to clean up the display
	formattedAmount = parseFloat(formattedAmount).toString();

	return formattedAmount;
}

export function shortenCaller(caller) {
	if (!caller) return "";
	return caller.toString().slice(0, 8);
}