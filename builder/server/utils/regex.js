export default {
    arrayU8(str, key, val) {
        const regex = new RegExp(`pub\\s+const\\s+${key}:\\s*&\\[.*?\\];`);
        return str.replace(regex, `pub const ${key}: &[u8] = &[${val}];`);
    },

    str(str, key, val) {
        const regex = new RegExp(`pub\\s+const\\s+${key}:\\s*&\\s*str\\s*=\\s*".*?";`);
        return str.replace(regex, `pub const ${key}: &str = "${val}";`);
    },

    bool(str, key, val) {
        const regex = new RegExp(`pub\\s+const\\s+${key}:\\s*bool\\s*=\\s*(true|false);`);
        return str.replace(regex, `pub const ${key}: bool = ${val};`);
    }
};