# rustysecrets-node

[![Build Status](https://travis-ci.org/SpinResearch/rustysecrets-node.svg?branch=master)](https://travis-ci.org/SpinResearch/rustysecrets-node) [![npm](https://img.shields.io/npm/v/rusty-secrets.svg)](https://www.npmjs.com/package/rusty-secrets) [![License](https://img.shields.io/github/license/SpinResearch/rustysecrets-node.svg)]()

> Node.js bindings to [RustySecrets](https://github.com/SpinResearch/RustySecrets), a secret sharing scheme library written in Rust.

## Requirements

- Node.js v8.x LTS (Carbon) or Electron 1.7.x

## Installation

```shell
$ npm install --save rusty-secrets
```

## Usage

```javascript
var rustysecrets = require('rusty-secrets').wrapped;

var threshold   = 7;
var sharesCount = 10;
var secret      = "Hello, World";

var shares = rustysecrets.splitSecret(threshold, sharesCount, secret);
// > [ '7-1-CgyDwB3wLPHL4hinb1o',
// >   '7-2-CgzDMo5J6YvrIIHFahg',
// >   '7-3-CgwIl//VqlYAlfYQaSY',
// >   '7-4-CgxDy7Umfn+rua4BYJw',
// >   '7-5-CgyIbsS6PaJADNnUY6I',
// >   '7-6-CgzInFcD+NhgzkC2ZuA',
// >   '7-7-CgwDOSafuwWLezdjZd4',
// >   '7-8-CgxeJMP4TYorlvCUdIk',
// >   '7-9-CgyVgbJkDlfAI4dBd7c',
// >   '7-10-CgzVcyHdyy3g4R4jcvU' ]

var someShares = shares.slice(1, 8);
var recovered = rustysecrets.recoverSecret(someShares);
// > Hello, World!
```

## API

<a name="rustysecrets"></a>

### rustysecrets : <code>object</code>

* [rustysecrets](#rustysecrets) : <code>object</code>
    * [.sss](#rustysecrets.sss) : <code>object</code>
        * [.splitSecret(k, n, secret, signShares)](#rustysecrets.sss.splitSecret) ⇒ <code>Array.&lt;String&gt;</code>
        * [.recoverSecret(shares, verifySignatures)](#rustysecrets.sss.recoverSecret) ⇒ <code>String</code>
    * [.wrapped](#rustysecrets.wrapped) : <code>object</code>
        * [.splitSecret(k, n, secret, mimeType, signShares, cb)](#rustysecrets.wrapped.splitSecret)
        * [.recoverSecret(shares, verifySignatures, cb)](#rustysecrets.wrapped.recoverSecret)
    * [.generate_shares()](#rustysecrets.generate_shares)
    * [.recover_secret()](#rustysecrets.recover_secret)

<a name="rustysecrets.sss"></a>

### rustysecrets.sss : <code>object</code>
Provides an API to perform Shamir's secret sharing, with optional signatures

**Kind**: static namespace of [<code>rustysecrets</code>](#rustysecrets)

* [.sss](#rustysecrets.sss) : <code>object</code>
    * [.splitSecret(k, n, secret, signShares)](#rustysecrets.sss.splitSecret) ⇒ <code>Array.&lt;String&gt;</code>
    * [.recoverSecret(shares, verifySignatures)](#rustysecrets.sss.recoverSecret) ⇒ <code>String</code>

<a name="rustysecrets.sss.splitSecret"></a>

#### sss.splitSecret(k, n, secret, signShares) ⇒ <code>Array.&lt;String&gt;</code>
Performs k-out-of-n Shamir's secret sharing.

**Kind**: static method of [<code>sss</code>](#rustysecrets.sss)
**Returns**: <code>Array.&lt;String&gt;</code> - An array of shares
**Throws**:

- Will throw an error if the parameters are invalid.


| Param | Type | Description |
| --- | --- | --- |
| k | <code>Number</code> | Minimum number of shares to be provided to recover the secret (1 <= k <= 255). |
| n | <code>Number</code> | Number of shares to emit (2 <= n <= 255). |
| secret | <code>Buffer</code> | The secret to split. |
| signShares | <code>Boolean</code> | Sign the shares using Merkle signing. |

<a name="rustysecrets.sss.recoverSecret"></a>

#### sss.recoverSecret(shares, verifySignatures) ⇒ <code>String</code>
Recovers the secret from a k-out-of-n Shamir's secret sharing scheme.

At least `k` distinct shares need to be provided to recover the secret.

**Kind**: static method of [<code>sss</code>](#rustysecrets.sss)
**Returns**: <code>String</code> - The recovered secret
**Throws**:

- Will throw an error if there are not enough shares.
- Will throw an error if the shares are invalid.
- Will throw an error if the shares data is not well-formed.
- Will throw an error if `verifySignatures` is not set to the proper value.


| Param | Type | Description |
| --- | --- | --- |
| shares | <code>Array.&lt;String&gt;</code> | The shares to recover the secret from. |
| verifySignatures | <code>Boolean</code> | Verify the signatures.  Must be set to `true` if they are signed, `false` otherwise |

<a name="rustysecrets.wrapped"></a>

### rustysecrets.wrapped : <code>object</code>
Provides an API to perform Shamir's secret sharing, with MIME types

**Kind**: static namespace of [<code>rustysecrets</code>](#rustysecrets)

* [.wrapped](#rustysecrets.wrapped) : <code>object</code>
    * [.splitSecret(k, n, secret, mimeType, signShares, cb)](#rustysecrets.wrapped.splitSecret)
    * [.recoverSecret(shares, verifySignatures, cb)](#rustysecrets.wrapped.recoverSecret)

<a name="rustysecrets.wrapped.splitSecret"></a>

#### wrapped.splitSecret(k, n, secret, mimeType, signShares, cb)
Performs k-out-of-n Shamir's secret sharing.

**Kind**: static method of [<code>wrapped</code>](#rustysecrets.wrapped)

| Param | Type | Description |
| --- | --- | --- |
| k | <code>Number</code> | Minimum number of shares to be provided to recover the secret (1 <= k <= 255). |
| n | <code>Number</code> | Number of shares to emit (2 <= n <= 255). |
| secret | <code>Buffer</code> | The secret to split. |
| mimeType | <code>Buffer</code> | The MIME type of the secret (or null). |
| signShares | <code>Boolean</code> | Sign the shares using Merkle signing. |
| cb | <code>function</code> | The callback to call with the result. |

<a name="rustysecrets.wrapped.recoverSecret"></a>

#### wrapped.recoverSecret(shares, verifySignatures, cb)
Recovers the secret from a k-out-of-n Shamir's secret sharing scheme.

At least `k` distinct shares need to be provided to recover the secret.

**Kind**: static method of [<code>wrapped</code>](#rustysecrets.wrapped)

| Param | Type | Description |
| --- | --- | --- |
| shares | <code>Array.&lt;String&gt;</code> | The shares to recover the secret from. |
| verifySignatures | <code>Boolean</code> | Verify the signatures.  Must be set to `true` if they are signed, `false` otherwise |
| cb | <code>function</code> | The callback to call with the result. |

<a name="rustysecrets.generate_shares"></a>

### rustysecrets.generate_shares()
Legacy API: See [splitSecret](#rustysecrets.wrapped.splitSecret).

**Kind**: static method of [<code>rustysecrets</code>](#rustysecrets)
<a name="rustysecrets.recover_secret"></a>

### rustysecrets.recover_secret()
Legacy API: See [recoverSecret](#rustysecrets.wrapped.recoverSecret).

**Kind**: static method of [<code>rustysecrets</code>](#rustysecrets)

## Bug Reporting

Please report bugs either as pull requests or as issues in [the issue
tracker](https://github.com/SpinResearch/RustySecrets/issues). *rustysecrets-node* has a
**full disclosure** vulnerability policy. **Please do NOT attempt to report
any security vulnerability in this code privately to anybody.**

## License

See [LICENSE](LICENSE)

