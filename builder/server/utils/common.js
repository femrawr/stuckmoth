export default {
    strToByte(str) {
        const encoder = new TextEncoder();
        const array = encoder.encode(str);

        const hex = Array.from(array).map(b => '0x' + b.toString(16)
            .padStart(2, 0)
            .toUpperCase()
        );

        return hex.join(', ');
    }
};