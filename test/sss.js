var assert  = require('assert');
var format  = require('string-format');
var sss = require('..').sss;

format.extend(String.prototype, {
  ifNull: function(s) {
    return s !== null && s || 'null';
  }
});

function assertErr(test, regexp, done) {
  test(function(err, res) {
    assert.notEqual(err, null);
    assert.equal(res, null);
    assert(regexp.test(err.toString()), err.toString() + ' did not match ' + regexp);
    done();
  })
}

function doneAfter(n, cb) {
  var i = 0;
  return function() {
    i += 1;
    if (i >= n) {
      cb();
    }
  };
}

function splitRecoverWorks(k, n, secret, sign) {
  var title = 'split-recover({}, {}, {})'.format(k, n, sign);

  it(title, function(done) {
    sss.splitSecret(k, n, secret, sign, function(err, shares) {
      assert.ifError(err);
      if (!shares) assert.fail('Splitting failed (internal test error)');

      sss.recoverSecret(shares.slice(1, k + 1), sign, function(err, recovered) {
        assert.ifError(err);
        assert.equal(recovered, secret);
        done();
      });
    });
  });
}

function splitRecoverFailsMissingShares(k, n, secret, sign) {
  var title = 'split-recover({}, {}, {}) fails when shares missing'.format(k, n, sign);

  it(title, function(done) {
    sss.splitSecret(k, n, secret, sign, function(err, shares) {
      assert.ifError(err);
      if (!shares) assert.fail('Splitting failed (internal test error)');

      sss.recoverSecret(shares.slice(0, k - 1), sign, function(err, recovered) {
        assert.equal(recovered, null);
        assert.notEqual(err, null);
        done();
      });
    });
  });
}

function splitRecoverFailsIncompatibleSet(k, n, secret, sign) {
  var title = 'split-recover({}, {}, {}) fails on incompatible sets'.format(k, n, sign);

  it(title, function(done) {
    sss.splitSecret(k, n, secret, sign, function(err, shares1) {
      assert.ifError(err);
      if (!shares1) assert.fail('Splitting failed (internal test error)');

      sss.splitSecret(k - 1, n - 1, secret + " RANDOM", sign, function(err, shares2) {
        assert.ifError(err);
        if (!shares2) assert.fail('Splitting failed (internal test error)');

        var shares = shares1.slice(0, k / 2).concat(shares2.slice(k / 2, k + 1));
        sss.recoverSecret(shares, sign, function(err, recovered) {
          assert.equal(recovered, null);
          assert.notEqual(err, null);
          assert.notEqual(err.share_groups, null);
          done();
        });
      });
    });
  });
}


var secret = 'I do not want to live in a world where everything I do and say is recorded. That is not something I am willing to support or live under.';

describe('sss', function() {
  splitRecoverWorks(7, 10, secret, true);
  splitRecoverWorks(7, 10, secret, false);
  splitRecoverWorks(7, 10, secret, true);
  splitRecoverWorks(7, 10, secret, false);

  splitRecoverFailsMissingShares(7, 10, secret, true);
  splitRecoverFailsMissingShares(7, 10, secret, false);
  splitRecoverFailsMissingShares(7, 10, secret, true);
  splitRecoverFailsMissingShares(7, 10, secret, false);

  splitRecoverFailsIncompatibleSet(7, 10, secret, true);
  splitRecoverFailsIncompatibleSet(7, 10, secret, false);
  splitRecoverFailsIncompatibleSet(7, 10, secret, true);
  splitRecoverFailsIncompatibleSet(7, 10, secret, false);

  it('splitSecret errors on invalid threshold', function(done) {
    var finalDone = doneAfter(3, done);

    assertErr(function(cb) {
      sss.splitSecret(-10, 10, secret, false, cb);
    }, /Invalid threshold/, finalDone);

    assertErr(function(cb) {
      sss.splitSecret(1, 10, secret, false, cb);
    }, /Threshold is too small/, finalDone);

    assertErr(function(cb) {
      sss.splitSecret(1000, 10, secret, false, cb);
    }, /Invalid threshold/, finalDone);
  });

  it('splitSecret errors on invalid shares count', function(done) {
    var finalDone = doneAfter(3, done);

    assertErr(function(cb) {
      sss.splitSecret(7, -10, secret, false, cb);
    }, /Invalid shares count/, finalDone);

    assertErr(function(cb) {
      sss.splitSecret(7, 2, secret, false, cb);
    }, /Number of shares is too small/, finalDone);

    assertErr(function(cb) {
      sss.splitSecret(7, 1000, secret, false, cb);
    }, /Invalid shares count/, finalDone);
  });
});

