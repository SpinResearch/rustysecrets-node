
var native = require('./native');

/** @namespace rustysecrets */
var rustysecrets = {

  /**
   * Provides an API to perform Shamir's secret sharing, with optional signatures
   * @namespace
   */
  sss: {

    /**
     * Performs k-out-of-n Shamir's secret sharing.
     *
     * @param k          {Number}  Minimum number of shares to be provided to recover the secret (1 <= k <= 255).
     * @param n          {Number}  Number of shares to emit (2 <= n <= 255).
     * @param secret     {Buffer}  The secret to split.
     * @param signShares {Boolean} Sign the shares using Merkle signing.
     *
     * @returns {String[]} An array of shares
     * @throws Will throw an error if the parameters are invalid.
     */
    splitSecret: function(k, n, secret, signShares, cb) {
      if (!(secret instanceof Buffer)) {
        secret = Buffer.from(secret);
      }

      try {
        native.sss_splitSecret(
          k,
          n,
          secret,
          signShares || false,
          cb
        );
      } catch (e) {
        cb(e, null);
      }
    },

    /**
     * Recovers the secret from a k-out-of-n Shamir's secret sharing scheme.
     *
     * At least `k` distinct shares need to be provided to recover the secret.
     *
     * @param shares           {String[]}  The shares to recover the secret from.
     * @param verifySignatures {Boolean}   Verify the signatures.  Must be set to `true` if they are signed, `false` otherwise
     *
     * @returns {String} The recovered secret
     * @throws Will throw an error if there are not enough shares.
     * @throws Will throw an error if the shares are invalid.
     * @throws Will throw an error if the shares data is not well-formed.
     * @throws Will throw an error if `verifySignatures` is not set to the proper value.
     */
    recoverSecret: function(shares, verifySignatures, cb) {
      try {
        native.sss_recoverSecret(
          shares,
          verifySignatures || false,
          cb
        );
      } catch (e) {
        cb(e, null);
      }
    }
  },

  /**
   * Provides an API to perform Shamir's secret sharing, with MIME types
   * @namespace
   */
  wrapped: {

    /**
     * Performs k-out-of-n Shamir's secret sharing.
     *
     * @param k          {Number}   Minimum number of shares to be provided to recover the secret (1 <= k <= 255).
     * @param n          {Number}   Number of shares to emit (2 <= n <= 255).
     * @param secret     {Buffer}   The secret to split.
     * @param mimeType   {Buffer}   The MIME type of the secret (or null).
     * @param signShares {Boolean}  Sign the shares using Merkle signing.
     * @param cb         {Function} The callback to call with the result.
     */
    splitSecret: function(k, n, secret, mimeType, signShares, cb) {
      if (!(secret instanceof Buffer)) {
        secret = Buffer.from(secret);
      }

      try {
        native.wrapped_splitSecret(
          k,
          n,
          secret,
          mimeType || null,
          signShares || false,
          cb
        );
      } catch (e) {
        cb(e, null);
      }
    },

    /**
     * Recovers the secret from a k-out-of-n Shamir's secret sharing scheme.
     *
     * At least `k` distinct shares need to be provided to recover the secret.
     *
     * @param shares           {String[]} The shares to recover the secret from.
     * @param verifySignatures {Boolean}  Verify the signatures.  Must be set to `true` if they are signed, `false` otherwise
     * @param cb               {Function} The callback to call with the result.
     */
    recoverSecret: function(shares, verifySignatures, cb) {
      try {
        native.wrapped_recoverSecret(
          shares,
          verifySignatures || false,
          cb
        );
      } catch (e) {
        cb(e, null);
      }
    }
  }
};

// Only export wrapped module and legacy API for now
module.exports = {
  sss: rustysecrets.sss,
  wrapped: rustysecrets.wrapped
};

