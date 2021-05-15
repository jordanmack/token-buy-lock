# Token Buy Lock

This is an example of a script which employs the "open transaction" design pattern, allowing CKBytes secured by the lock script to be exchanged for a specific number of SUDT token on-chain, and without the owner being present to sign the transaction. 

Note: This script is designed for demonstration purposes only, and should never be used to secure funds in a production environment.

Build contracts:

``` sh
capsule build
```

Run tests:

``` sh
capsule test
```
