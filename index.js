import { createRequire } from 'module';
const require = createRequire(import.meta.url);

const crypto = require('./index.node');

class Crypto {
    constructor(){

    }
    hello(){
        return crypto.hello();
    }
    md5(word){
        return crypto.md5(word);
    }
}
export default Crypto;