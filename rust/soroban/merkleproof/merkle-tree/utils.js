import { keccak256 } from 'ethereum-cryptography/keccak';

/**
 * Convert a hexstring in  a byte array
 * @param {string} hexString
 * @returns
 */
export function hexStringToByteArray(hexString) {
  if (hexString.length % 2 !== 0) {
    throw 'Must have an even number of hex digits to convert to bytes';
  } /* w w w.  jav  a2 s .  c o  m*/
  var numBytes = hexString.length / 2;
  var byteArray = new Uint8Array(numBytes);
  for (var i = 0; i < numBytes; i++) {
    byteArray[i] = parseInt(hexString.substr(i * 2, 2), 16);
  }
  return byteArray;
}

/**
 * Convert a byte array to hex string
 * @param {number[] | Uint8Array} byteArray
 * @returns
 */
export function byteArrayToHexString(byteArray) {
  if (byteArray instanceof Uint8Array) {
    return Buffer.from(byteArray).toString('hex');
  }

  const array = new Uint8Array(byteArray);
  return Buffer.from(array).toString('hex');
}

/**
 *
 * @param {number[]} array
 * @returns
 */
export function doubleHash(array) {
  return keccak256(keccak256(new Uint8Array(array)));
}

// const proof =
//   "4c8ce3926f0293b1d281eac81d8f773dc2a9333e951e7b199adeebc7d7a2ed66";
// const array = hexStringToByteArray(proof);
// console.log(array, array.length);

// const byteArray = [
//   26, 176, 198, 148, 138, 39, 83, 73, 174, 69, 160, 106, 173, 102, 168, 189, 101,
//   172, 24, 7, 70, 21, 213, 54, 118, 192, 155, 103, 128, 144, 153, 224,
// ];
// console.log(byteArrayToHexString(byteArray));

// const hash = doubleHash([
//   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//   0, 0, 0, 0, 0, 0, 1,
// ]) // uint256(1)
// console.log(
//   hash, byteArrayToHexString(hash),
// );
